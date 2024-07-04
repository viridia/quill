use bevy::{prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use quill_obsidian::{colors, controls::ScrollView};

use crate::{materials::DotGridMaterial, Gesture, GraphEvent};

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
                                On::<Pointer<DragStart>>::run(move |mut event: ListenerMut<Pointer<DragStart>>,
                                    mut writer: EventWriter<GraphEvent>| {
                                        event.stop_propagation();
                                        writer.send(GraphEvent {
                                            target: event.target(),
                                            gesture: Gesture::SelectClear,
                                        });
                                }),
                                On::<Pointer<DragEnd>>::run(move |mut event: ListenerMut<Pointer<DragEnd>>,
                                    mut _writer: EventWriter<GraphEvent>| {
                                        event.stop_propagation();
                                    // drag_state.set(
                                    //     world,
                                    //     DragState {
                                    //         dragging: false,
                                    //         offset: position_capture.get(world),
                                    //     },
                                    // );
                                }),
                                On::<Pointer<Drag>>::run({
                                    move |mut event: ListenerMut<Pointer<Drag>>,
                                    mut writer: EventWriter<GraphEvent>
                                    | {
                                        // event.stop_propagation();
                                        // let gesture = Gesture::Move(event.distance);
                                        // writer.send(GraphEvent {
                                        //     target: id,
                                        //     gesture,
                                        //     action: crate::GestureAction::Move,
                                        // });
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
            .content_style(style_node_graph_content)
            .scroll_enable_x(true)
            .scroll_enable_y(true)
    }
}
