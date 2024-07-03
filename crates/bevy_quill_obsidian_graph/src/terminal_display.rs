use bevy::{prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill::{prelude::*, ViewChild};
use quill_obsidian::{colors, hooks::UseIsHover};

use crate::{DropLocation, Gesture, GestureAction, GestureState, GraphEvent};

fn style_node_graph_terminal_outline(ss: &mut StyleBuilder) {
    ss.position(ui::PositionType::Absolute)
        .left(-3)
        .top(-3)
        .right(-3)
        .bottom(-3)
        .border(2)
        .border_color(colors::FOCUS)
        .border_radius(8)
        .pointer_events(false);
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
        .border_radius(5);
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

    fn create(&self, cx: &mut Cx) -> Self::View {
        let id = self.id;
        let is_hover = cx.is_hovered(id);
        let color = self.color;
        Element::<NodeBundle>::new()
            .named("InputTerminal")
            .style(style_input_connector)
            .children((
                Element::<NodeBundle>::for_entity(self.id)
                    .style((style_input_terminal, move |sb: &mut StyleBuilder| {
                        sb.background_color(color);
                    }))
                    .children(Cond::new(
                        is_hover,
                        Element::<NodeBundle>::new().style(style_node_graph_terminal_outline),
                        (),
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
        .border_radius(5);
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

    fn create(&self, cx: &mut Cx) -> Self::View {
        let id = self.id;
        let is_hover = cx.is_hovered(id);
        let color = self.color;
        Element::<NodeBundle>::new()
            .named("OutputTerminal")
            .style(style_output_connector)
            .children((
                Element::<NodeBundle>::for_entity(self.id)
                    .style((style_output_terminal, move |sb: &mut StyleBuilder| {
                        sb.background_color(color);
                    }))
                    .insert_dyn(
                        move |_| {
                            (
                                On::<Pointer<DragStart>>::run(
                                    move |
                                    event: Listener<Pointer<DragStart>>,
                                    mut gesture_state: ResMut<GestureState>,
                                    mut writer: EventWriter<GraphEvent>| {
                                        let gesture = Gesture::ConnectInput {
                                            terminal: id, to: DropLocation::Position(event.pointer_location.position) };
                                        gesture_state.gesture = Some(gesture.clone());
                                        writer.send(GraphEvent {
                                            target: id,
                                            gesture,
                                            action: GestureAction::Start
                                        });
                                    },
                                ),
                                On::<Pointer<DragEnd>>::run(move |world: &mut World| {
                                    println!("Drag end");
                                    // drag_state.set(
                                    //     world,
                                    //     DragState {
                                    //         dragging: false,
                                    //         offset: position_capture.get(world),
                                    //     },
                                    // );
                                }),
                                On::<Pointer<Drag>>::run({
                                    move |world: &mut World| {
                                        // println!("Drag");
                                        //     let event = world
                                        //         .get_resource::<ListenerInput<Pointer<Drag>>>()
                                        //         .unwrap();
                                        //     let ev = event.distance;
                                        //     let ds = drag_state.get(world);
                                        //     if let Some(on_drag) = on_drag {
                                        //         if ds.dragging {
                                        //             world.run_callback(
                                        //                 on_drag,
                                        //                 Vec2::new(ev.x, ev.y) + ds.offset,
                                        //             );
                                        //         }
                                        //     }
                                    }
                                }),
                                On::<Pointer<DragEnter>>::run({
                                    move |world: &mut World| {
                                        println!("Drag Enter");
                                        //     let event = world
                                        //         .get_resource::<ListenerInput<Pointer<Drag>>>()
                                        //         .unwrap();
                                        //     let ev = event.distance;
                                        //     let ds = drag_state.get(world);
                                        //     if let Some(on_drag) = on_drag {
                                        //         if ds.dragging {
                                        //             world.run_callback(
                                        //                 on_drag,
                                        //                 Vec2::new(ev.x, ev.y) + ds.offset,
                                        //             );
                                        //         }
                                        //     }
                                    }
                                }),
                                On::<Pointer<DragLeave>>::run({
                                    move |world: &mut World| {
                                        println!("Drag Leave");
                                        //     let event = world
                                        //         .get_resource::<ListenerInput<Pointer<Drag>>>()
                                        //         .unwrap();
                                        //     let ev = event.distance;
                                        //     let ds = drag_state.get(world);
                                        //     if let Some(on_drag) = on_drag {
                                        //         if ds.dragging {
                                        //             world.run_callback(
                                        //                 on_drag,
                                        //                 Vec2::new(ev.x, ev.y) + ds.offset,
                                        //             );
                                        //         }
                                        //     }
                                    }
                                }),
                            )
                        },
                        (),
                    )
                    .children(Cond::new(
                        is_hover,
                        Element::<NodeBundle>::new().style(style_node_graph_terminal_outline),
                        (),
                    )),
                self.label.clone(),
            ))
    }
}
