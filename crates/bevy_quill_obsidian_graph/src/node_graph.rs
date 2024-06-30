use std::ops::Mul;

use bevy::{prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_reactor::*;
use bevy_reactor_signals::{Callback, Cx, IntoSignal, RunContextSetup, RunContextWrite, Signal};

use crate::{
    colors,
    materials::{DotGridMaterial, DrawPathMaterial, DrawablePath},
};

use super::ScrollView;

fn style_node_graph(ss: &mut StyleBuilder) {
    ss.background_color(colors::U1);
}

fn style_node_graph_content(ss: &mut StyleBuilder) {
    ss.border(0)
        // .border_color(colors::X_RED)
        .min_width(ui::Val::Percent(100.))
        .min_height(ui::Val::Percent(100.));
}

fn style_node_graph_scroll(ss: &mut StyleBuilder) {
    ss.min_width(ui::Val::Px(2000.0));
}

/// An editable graph of nodes, connected by edges.
#[derive(Default)]
pub struct GraphDisplay {
    /// Nodes within the node graph.
    pub children: ChildArray,

    /// Additional styles to be applied to the graph element.
    pub style: StyleHandle,
}

impl GraphDisplay {
    /// Create a new graph display.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the child views for this element.
    pub fn children<V: ChildViewTuple>(mut self, children: V) -> Self {
        self.children = children.to_child_array();
        self
    }

    /// Set the additional styles for the button.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }
}

impl ViewTemplate for GraphDisplay {
    fn create(&self, cx: &mut Cx) -> impl IntoView {
        let mut ui_materials = cx
            .world_mut()
            .get_resource_mut::<Assets<DotGridMaterial>>()
            .unwrap();
        let material = ui_materials.add(DotGridMaterial {
            color_bg: LinearRgba::from(colors::U1).to_vec4(),
            color_fg: LinearRgba::from(colors::U3).to_vec4(),
        });

        ScrollView::new()
            .children(
                Element::<MaterialNodeBundle<DotGridMaterial>>::new()
                    .named("NodeGraph::Scroll")
                    .insert(material.clone())
                    .style(style_node_graph_scroll)
                    .children(self.children.clone()),
            )
            .style((style_node_graph, self.style.clone()))
            .content_style(style_node_graph_content)
            .scroll_enable_x(true)
            .scroll_enable_y(true)
    }
}

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
#[derive(Default)]
pub struct NodeDisplay {
    /// The coordinates of the node's upper-left corner.
    pub position: Signal<Vec2>,
    /// The title of the node.
    pub title: Signal<String>,
    /// Whether the node is currently selected.
    pub selected: Signal<bool>,
    /// The content of the node.
    pub children: ChildArray,

    /// Callback called when the title bar is dragged.
    pub on_drag: Option<Callback<Vec2>>,
}

impl NodeDisplay {
    /// Create a new node display.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the seletion state of the node.
    pub fn selected(mut self, selected: impl IntoSignal<bool>) -> Self {
        self.selected = selected.into_signal();
        self
    }

    /// Set the position of the node.
    pub fn position(mut self, position: impl IntoSignal<Vec2>) -> Self {
        self.position = position.into_signal();
        self
    }

    /// Set the title of the node.
    pub fn title(mut self, title: impl IntoSignal<String>) -> Self {
        self.title = title.into_signal();
        self
    }

    /// Set the children of the node.
    pub fn children<V: ChildViewTuple>(mut self, children: V) -> Self {
        self.children = children.to_child_array();
        self
    }

    /// Set the callback called when the title bar is dragged.
    pub fn on_drag(mut self, on_drag: Callback<Vec2>) -> Self {
        self.on_drag = Some(on_drag);
        self
    }
}

impl ViewTemplate for NodeDisplay {
    fn create(&self, cx: &mut Cx) -> impl IntoView {
        let position = self.position;
        let id = cx.create_entity();
        let hovering = cx.create_hover_signal(id);
        let drag_state = cx.create_mutable::<DragState>(DragState::default());

        Element::<NodeBundle>::for_entity(id)
            .named("NodeGraph::Node")
            .style(style_node_graph_node)
            .create_effect(move |cx, ent| {
                // Update node position.
                let pos = position.get(cx);
                let mut style = cx.world_mut().get_mut::<Style>(ent).unwrap();
                style.left = ui::Val::Px(pos.x);
                style.top = ui::Val::Px(pos.y);
            })
            .children((
                Element::<NodeBundle>::new()
                    .named("NodeGraph::Node::Shadow")
                    .style(style_node_graph_node_shadow),
                Element::<NodeBundle>::new()
                    .named("NodeGraph::Node::Title")
                    .style(style_node_graph_node_title)
                    .insert((
                        On::<Pointer<DragStart>>::run(move |world: &mut World| {
                            // Save initial value to use as drag offset.
                            drag_state.set(
                                world,
                                DragState {
                                    dragging: true,
                                    offset: position.get(world),
                                },
                            );
                        }),
                        On::<Pointer<DragEnd>>::run(move |world: &mut World| {
                            drag_state.set(
                                world,
                                DragState {
                                    dragging: false,
                                    offset: position.get(world),
                                },
                            );
                        }),
                        On::<Pointer<Drag>>::run({
                            let on_drag = self.on_drag.unwrap();
                            move |world: &mut World| {
                                let event = world
                                    .get_resource::<ListenerInput<Pointer<Drag>>>()
                                    .unwrap();
                                let ev = event.distance;
                                let ds = drag_state.get(world);
                                if ds.dragging {
                                    world.run_callback(on_drag, Vec2::new(ev.x, ev.y) + ds.offset);
                                }
                            }
                        }),
                    ))
                    .children(self.title.clone()),
                Element::<NodeBundle>::new()
                    .style(style_node_graph_node_content)
                    .children(self.children.clone()),
                Cond::new(
                    hovering,
                    || {
                        Element::<NodeBundle>::new()
                            .named("NodeGraph::Node::Outline")
                            .style(style_node_graph_node_outline)
                    },
                    || (),
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
pub struct InputTerminalDisplay {
    /// Entity id for the terminal.
    pub id: Entity,
    /// Color of the connector terminal, which is typically used to indicate the data-type
    /// of the connector.
    pub color: Srgba,
    /// Control rendered when the input is not connected.
    pub control: ViewRef,
}

impl ViewTemplate for InputTerminalDisplay {
    fn create(&self, _cx: &mut Cx) -> impl IntoView {
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
    fn create(&self, _cx: &mut Cx) -> impl IntoView {
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

/// Displays a stroked path between two nodes.
pub struct EdgeDisplay {
    /// Pixel position of the source terminal.
    pub src_pos: Signal<Vec2>,

    /// Pixel position of the destination terminal.
    pub dst_pos: Signal<Vec2>,
}

impl ViewTemplate for EdgeDisplay {
    fn create(&self, cx: &mut Cx) -> impl IntoView {
        let mut ui_materials = cx
            .world_mut()
            .get_resource_mut::<Assets<DrawPathMaterial>>()
            .unwrap();
        let material = ui_materials.add(DrawPathMaterial::default());
        let material_id = material.id();
        let src_pos = self.src_pos;
        let dst_pos = self.dst_pos;

        Element::<MaterialNodeBundle<DrawPathMaterial>>::new()
            .named("NodeGraph::Edge")
            .insert(material)
            .create_effect(move |cx, ent| {
                let mut path = DrawablePath::new(colors::U4, 1.5);
                let src = src_pos.get(cx);
                let dst = dst_pos.get(cx);
                let dx = (dst.x - src.x).abs().mul(0.3).min(20.);
                let src1 = src + Vec2::new(dx, 0.);
                let dst1 = dst - Vec2::new(dx, 0.);
                path.move_to(src);
                let mlen = src1.distance(dst1);
                if mlen > 40. {
                    let src2 = src1.lerp(dst1, 20. / mlen);
                    let dst2 = src1.lerp(dst1, (mlen - 20.) / mlen);
                    path.quadratic_to(src1, src2);
                    path.line_to(dst2);
                    path.quadratic_to(dst1, dst);
                } else {
                    let mid = src1.lerp(dst1, 0.5);
                    path.quadratic_to(src1, mid);
                    path.quadratic_to(dst1, dst);
                }
                let bounds = path.bounds();

                let mut style = cx.world_mut().get_mut::<Style>(ent).unwrap();
                style.left = ui::Val::Px(bounds.min.x);
                style.top = ui::Val::Px(bounds.min.y);
                style.width = ui::Val::Px(bounds.width());
                style.height = ui::Val::Px(bounds.height());
                style.position_type = ui::PositionType::Absolute;

                let mut materials = cx
                    .world_mut()
                    .get_resource_mut::<Assets<DrawPathMaterial>>()
                    .unwrap();
                let material = materials.get_mut(material_id).unwrap();
                material.update(&path);
            })
    }
}
