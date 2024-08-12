use bevy::{prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill_core::*;
use bevy_quill_obsidian::{colors, controls::ScrollView, scrolling::ScrollContent};

use crate::{materials::DotGridMaterial, DragAction, DragMode, Gesture, GestureState, GraphEvent};

fn style_node_graph(ss: &mut StyleBuilder) {
    ss.background_color(colors::U1);
}

fn style_node_graph_scroll(ss: &mut StyleBuilder) {
    ss.position(ui::PositionType::Absolute)
        .min_width(ui::Val::Px(2000.0))
        .min_height(ui::Val::Percent(100.));
}

/// An editable graph of nodes, connected by edges.
#[derive(Default, Clone, PartialEq)]
pub struct GraphDisplay {
    /// Nodes within the node graph.
    pub children: ViewChild,

    /// Additional styles to be applied to the graph element.
    pub style: StyleHandle,

    /// Optional entity id to use for the scrolling element. This is useful for querying the
    /// current scroll position.
    pub entity: Option<Entity>,
}

impl GraphDisplay {
    /// Create a new graph display.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the child views for this element.
    pub fn children(mut self, children: impl IntoViewChild) -> Self {
        self.children = children.into_view_child();
        self
    }

    /// Set the additional styles for the button.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set the entity id to use for the scrolling element.
    /// This is useful for querying the current scroll position.
    pub fn entity(mut self, entity: Entity) -> Self {
        self.entity = Some(entity);
        self
    }
}

impl ViewTemplate for GraphDisplay {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let material = cx.create_memo(
            |world, _| {
                let mut ui_materials = world.get_resource_mut::<Assets<DotGridMaterial>>().unwrap();
                ui_materials.add(DotGridMaterial {
                    color_bg: LinearRgba::from(colors::U1).to_vec4(),
                    color_fg: LinearRgba::from(colors::U3).to_vec4(),
                })
            },
            (),
        );

        ScrollView::new()
            .entity(self.entity)
            .children(
                Element::<MaterialNodeBundle<DotGridMaterial>>::new()
                    .named("NodeGraph::Scroll")
                    .insert_dyn(
                        move |_| {
                            (
                                ScrollContent,
                                On::<Pointer<Down>>::run(
                                    move |mut event: ListenerMut<Pointer<Down>>,
                                    mut writer: EventWriter<GraphEvent>| {
                                        event.stop_propagation();
                                        writer.send(GraphEvent {
                                            target: event.target(),
                                            gesture: Gesture::SelectClear,
                                        });
                                }),
                                On::<Pointer<DragStart>>::run(
                                    move |mut event: ListenerMut<Pointer<DragStart>>,
                                    mut gesture_state: ResMut<GestureState>,
                                    mut writer: EventWriter<GraphEvent>,
                                    rel: crate::relative_pos::RelativeWorldPositions| {
                                        event.stop_propagation();
                                        let pos = rel.transform_relative(event.listener(), event.pointer_location.position, 1);
                                        gesture_state.mode = DragMode::RectSelect(pos);
                                        writer.send(GraphEvent {
                                            target: event.target(),
                                            gesture: Gesture::SelectRect(Rect::from_corners(
                                                pos,
                                                pos), DragAction::Start),
                                        });
                                }),
                                On::<Pointer<DragEnd>>::run(
                                    move |mut event: ListenerMut<Pointer<DragEnd>>,
                                    mut gesture_state: ResMut<GestureState>,
                                    mut writer: EventWriter<GraphEvent>,
                                    rel: crate::relative_pos::RelativeWorldPositions| {
                                        event.stop_propagation();
                                        if let DragMode::RectSelect(pos) = gesture_state.mode {
                                            writer.send(GraphEvent {
                                                target: event.target(),
                                                gesture: Gesture::SelectRect(Rect::from_corners(
                                                    rel.transform_relative(event.listener(), event.pointer_location.position, 1),
                                                    pos), DragAction::Finish),
                                            });
                                            gesture_state.mode = DragMode::None;
                                        }
                                }),
                                On::<Pointer<Drag>>::run({
                                    move |mut event: ListenerMut<Pointer<Drag>>,
                                    gesture_state: ResMut<GestureState>,
                                    mut writer: EventWriter<GraphEvent>,
                                    rel: crate::relative_pos::RelativeWorldPositions
                                    | {
                                        event.stop_propagation();
                                        if let DragMode::RectSelect(pos) = gesture_state.mode {
                                            writer.send(GraphEvent {
                                            target: event.target(),
                                            gesture: Gesture::SelectRect(Rect::from_corners(
                                                rel.transform_relative(event.listener(), event.pointer_location.position, 1),
                                                pos), DragAction::Update),
                                        });
                                    }
                                    }
                                }),
                            )
                        },
                        (),
                    )
                    .insert(material.clone())
                    .style(style_node_graph_scroll)
                    .children(self.children.clone()),
            )
            .style((style_node_graph, self.style.clone()))
            .scroll_enable_x(true)
            .scroll_enable_y(true)
    }
}
