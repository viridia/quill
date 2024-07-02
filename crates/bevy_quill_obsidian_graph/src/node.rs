use bevy::{prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill::{prelude::*, IntoViewChild, ViewChild};
use quill_obsidian::{colors, hooks::UseIsHover};

fn style_node_graph_node(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .align_items(ui::AlignItems::Stretch)
        .position(ui::PositionType::Absolute);
}

const NODE_BORDER_RADIUS: f32 = 5.;
const NODE_BORDER_WIDTH: f32 = 1.;

fn style_node_graph_node_title(ss: &mut StyleBuilder) {
    ss.border(1)
        .border_color(colors::U4)
        .border(ui::UiRect {
            left: ui::Val::Px(NODE_BORDER_WIDTH),
            right: ui::Val::Px(NODE_BORDER_WIDTH),
            top: ui::Val::Px(NODE_BORDER_WIDTH),
            bottom: ui::Val::Px(0.),
        })
        .border_radius(ui::BorderRadius {
            top_left: ui::Val::Px(NODE_BORDER_RADIUS),
            top_right: ui::Val::Px(NODE_BORDER_RADIUS),
            bottom_left: ui::Val::Px(0.),
            bottom_right: ui::Val::Px(0.),
        })
        .background_color(colors::Y_GREEN.darker(0.05))
        .padding((6, 2));
}

fn style_node_graph_node_content(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .align_items(ui::AlignItems::Stretch)
        .gap(2)
        .border(1)
        .border_color(colors::U4)
        .border(ui::UiRect {
            left: ui::Val::Px(NODE_BORDER_WIDTH),
            right: ui::Val::Px(NODE_BORDER_WIDTH),
            top: ui::Val::Px(0.),
            bottom: ui::Val::Px(NODE_BORDER_WIDTH),
        })
        .border_radius(ui::BorderRadius {
            top_left: ui::Val::Px(0.),
            top_right: ui::Val::Px(0.),
            bottom_left: ui::Val::Px(NODE_BORDER_RADIUS),
            bottom_right: ui::Val::Px(NODE_BORDER_RADIUS),
        })
        .background_color(colors::U2)
        .padding((0, 6));
}

fn style_node_graph_node_shadow(ss: &mut StyleBuilder) {
    ss.position(ui::PositionType::Absolute)
        .left(-3)
        .top(-3)
        .right(-3)
        .bottom(-3)
        .border_radius(NODE_BORDER_RADIUS + 3.)
        .background_color(Srgba::new(0., 0., 0., 0.7))
        .pointer_events(false);
}

fn style_node_graph_node_outline(ss: &mut StyleBuilder) {
    ss.position(ui::PositionType::Absolute)
        .left(-3)
        .top(-3)
        .right(-3)
        .bottom(-3)
        .border(2)
        .border_color(colors::FOCUS)
        .border_radius(NODE_BORDER_RADIUS + 3.)
        .pointer_events(false);
}

#[derive(Clone, PartialEq, Default, Copy)]
struct DragState {
    dragging: bool,
    offset: Vec2,
}

/// A node within a node graph.
#[derive(Default, Clone, PartialEq)]
pub struct NodeDisplay {
    /// The coordinates of the node's upper-left corner.
    pub position: IVec2,
    /// The title of the node.
    pub title: String,
    /// Whether the node is currently selected.
    pub selected: bool,
    /// The content of the node.
    pub children: ViewChild,
    /// Callback called when the title bar is dragged.
    pub on_drag: Option<Callback<Vec2>>,
}

impl NodeDisplay {
    /// Create a new node display.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the seletion state of the node.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set the position of the node.
    pub fn position(mut self, position: IVec2) -> Self {
        self.position = position;
        self
    }

    /// Set the title of the node.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the children of the node.
    pub fn children(mut self, children: impl IntoViewChild) -> Self {
        self.children = children.into_view_child();
        self
    }

    /// Set the callback called when the title bar is dragged.
    pub fn on_drag(mut self, on_drag: Callback<Vec2>) -> Self {
        self.on_drag = Some(on_drag);
        self
    }
}

impl ViewTemplate for NodeDisplay {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let position = self.position;
        let position_capture = cx.create_capture(position.as_vec2());
        let id = cx.create_entity();
        let hovering = cx.is_hovered(id);
        let drag_state = cx.create_mutable::<DragState>(DragState::default());
        let on_drag = self.on_drag;

        Element::<NodeBundle>::for_entity(id)
            .named("NodeGraph::Node")
            .style(style_node_graph_node)
            .effect(
                move |cx, ent, position| {
                    // Update node position.
                    let mut style = cx.world_mut().get_mut::<Style>(ent).unwrap();
                    style.left = ui::Val::Px(position.x as f32);
                    style.top = ui::Val::Px(position.y as f32);
                },
                position,
            )
            .children((
                Element::<NodeBundle>::new()
                    .named("NodeGraph::Node::Shadow")
                    .style(style_node_graph_node_shadow),
                Element::<NodeBundle>::new()
                    .named("NodeGraph::Node::Title")
                    .style(style_node_graph_node_title)
                    .insert_dyn(
                        move |_| {
                            (
                                On::<Pointer<DragStart>>::run(move |world: &mut World| {
                                    // Save initial value to use as drag offset.
                                    drag_state.set(
                                        world,
                                        DragState {
                                            dragging: true,
                                            offset: position_capture.get(world),
                                        },
                                    );
                                }),
                                On::<Pointer<DragEnd>>::run(move |world: &mut World| {
                                    drag_state.set(
                                        world,
                                        DragState {
                                            dragging: false,
                                            offset: position_capture.get(world),
                                        },
                                    );
                                }),
                                On::<Pointer<Drag>>::run({
                                    // let on_drag = on_drag.unwrap();
                                    move |world: &mut World| {
                                        let event = world
                                            .get_resource::<ListenerInput<Pointer<Drag>>>()
                                            .unwrap();
                                        let ev = event.distance;
                                        let ds = drag_state.get(world);
                                        if let Some(on_drag) = on_drag {
                                            if ds.dragging {
                                                world.run_callback(
                                                    on_drag,
                                                    Vec2::new(ev.x, ev.y) + ds.offset,
                                                );
                                            }
                                        }
                                    }
                                }),
                            )
                        },
                        (),
                    )
                    .children(self.title.clone()),
                Element::<NodeBundle>::new()
                    .style(style_node_graph_node_content)
                    .children(self.children.clone()),
                Cond::new(
                    hovering,
                    Element::<NodeBundle>::new()
                        .named("NodeGraph::Node::Outline")
                        .style(style_node_graph_node_outline),
                    (),
                ),
            ))
    }
}

fn style_input_connector(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .padding((8, 0));
}

fn style_input_terminal(ss: &mut StyleBuilder) {
    ss.position(ui::PositionType::Absolute)
        .left(-4)
        .top(6)
        .width(8)
        .height(8)
        .border_radius(5)
        .background_color(colors::U4);
}

/// Depicts an input connector on a node.
#[derive(Clone, PartialEq)]
pub struct InputTerminalDisplay {
    /// Entity id for the terminal.
    pub id: Entity,
    /// Color of the connector terminal, which is typically used to indicate the data-type
    /// of the connector.
    pub color: Srgba,
    /// Control rendered when the input is not connected.
    pub control: ViewChild,
}

impl ViewTemplate for InputTerminalDisplay {
    type View = impl View;

    fn create(&self, _cx: &mut Cx) -> Self::View {
        let color = self.color;
        Element::<NodeBundle>::for_entity(self.id)
            .named("InputConnector")
            .style(style_input_connector)
            .children((
                Element::<NodeBundle>::new().style((
                    style_input_terminal,
                    move |sb: &mut StyleBuilder| {
                        sb.background_color(color);
                    },
                )),
                self.control.clone(),
            ))
    }
}

fn style_output_connector(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::FlexEnd)
        .min_height(20)
        .padding((8, 0));
}

fn style_output_terminal(ss: &mut StyleBuilder) {
    ss.position(ui::PositionType::Absolute)
        .right(-4)
        .top(6)
        .width(8)
        .height(8)
        .border_radius(5)
        .background_color(colors::U4);
}

/// Depicts an output connector on a node.
#[derive(Clone, PartialEq)]
pub struct OutputTerminalDisplay {
    /// Entity id for the terminal.
    pub id: Entity,
    /// Color of the connector terminal, which is typically used to indicate the data-type
    /// of the connector.
    pub color: Srgba,
    /// The name of the output.
    pub label: String,
}

impl ViewTemplate for OutputTerminalDisplay {
    type View = impl View;

    fn create(&self, _cx: &mut Cx) -> Self::View {
        let color = self.color;
        Element::<NodeBundle>::for_entity(self.id)
            .named("OutputConnector")
            .style(style_output_connector)
            .children((
                Element::<NodeBundle>::new().style((
                    style_output_terminal,
                    move |sb: &mut StyleBuilder| {
                        sb.background_color(color);
                    },
                )),
                self.label.clone(),
            ))
    }
}
