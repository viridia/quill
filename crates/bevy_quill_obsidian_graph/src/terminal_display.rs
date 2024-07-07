use bevy::{ecs::system::SystemParam, prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill::{prelude::*, ViewChild};
use bevy_quill_obsidian::{colors, hooks::UseIsHover};

use crate::{ConnectionTarget, DragMode, Gesture, GestureState, GraphEvent};

fn style_terminal_outline(ss: &mut StyleBuilder) {
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

fn style_terminal_hitbox(ss: &mut StyleBuilder) {
    ss.position(ui::PositionType::Absolute)
        .left(-3)
        .top(-3)
        .right(-3)
        .bottom(-3)
        .border_radius(8)
        .pointer_events(true);
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
                    .insert_dyn(terminal_event_handlers, (id, false))
                    .children((
                        Element::<NodeBundle>::new().style(style_terminal_hitbox),
                        Cond::new(
                            is_hover,
                            Element::<NodeBundle>::new().style(style_terminal_outline),
                            (),
                        ),
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
                    .insert_dyn(terminal_event_handlers, (id, true))
                    .children((
                        Element::<NodeBundle>::new().style(style_terminal_hitbox),
                        Cond::new(
                            is_hover,
                            Element::<NodeBundle>::new().style(style_terminal_outline),
                            (),
                        ),
                    )),
                self.label.clone(),
            ))
    }
}

#[allow(clippy::type_complexity)]
fn terminal_event_handlers(
    args: (Entity, bool),
) -> (
    On<Pointer<DragStart>>,
    On<Pointer<Drag>>,
    On<Pointer<DragEnd>>,
    On<Pointer<DragEnter>>,
    On<Pointer<DragLeave>>,
    On<Pointer<Drop>>,
) {
    let (id, is_output) = args;
    (
        On::<Pointer<DragStart>>::run(
            move |mut event: ListenerMut<Pointer<DragStart>>,
                  mut gesture_state: ResMut<GestureState>,
                  mut writer: EventWriter<GraphEvent>| {
                event.stop_propagation();
                if gesture_state.mode != DragMode::Connect {
                    gesture_state.mode = DragMode::Connect;
                    writer.send(GraphEvent {
                        target: id,
                        gesture: Gesture::Connect(if is_output {
                            crate::ConnectionAnchor::OutputTerminal(id)
                        } else {
                            crate::ConnectionAnchor::InputTerminal(id)
                        }),
                    });
                }
            },
        ),
        On::<Pointer<Drag>>::run(
            move |mut event: ListenerMut<Pointer<Drag>>,
                  gesture_state: ResMut<GestureState>,
                  mut writer: EventWriter<GraphEvent>,
                  rel: RelativeWorldPositions| {
                event.stop_propagation();
                if gesture_state.mode == DragMode::Connect {
                    // println!("position: {}", event.pointer_location.position);
                    writer.send(GraphEvent {
                        target: id,
                        gesture: Gesture::ConnectDrag(rel.transform_relative(
                            id,
                            event.pointer_location.position,
                            4,
                        )),
                    });
                }
            },
        ),
        On::<Pointer<DragEnd>>::run(
            move |mut event: ListenerMut<Pointer<DragEnd>>,
                  mut gesture_state: ResMut<GestureState>,
                  mut writer: EventWriter<GraphEvent>| {
                event.stop_propagation();
                if gesture_state.mode == DragMode::Connect {
                    gesture_state.mode = DragMode::None;
                    writer.send(GraphEvent {
                        target: id,
                        gesture: Gesture::ConnectFinish,
                    });
                }
            },
        ),
        On::<Pointer<DragEnter>>::run({
            move |mut event: ListenerMut<Pointer<DragEnter>>,
                  gesture_state: ResMut<GestureState>,
                  mut writer: EventWriter<GraphEvent>| {
                event.stop_propagation();
                if gesture_state.mode == DragMode::Connect {
                    writer.send(GraphEvent {
                        target: id,
                        gesture: Gesture::ConnectHover(if is_output {
                            ConnectionTarget::OutputTerminal(id)
                        } else {
                            ConnectionTarget::InputTerminal(id)
                        }),
                    });
                }
            }
        }),
        On::<Pointer<DragLeave>>::run({
            move |mut event: ListenerMut<Pointer<DragLeave>>,
                  gesture_state: ResMut<GestureState>,
                  mut writer: EventWriter<GraphEvent>| {
                event.stop_propagation();
                if gesture_state.mode == DragMode::Connect {
                    writer.send(GraphEvent {
                        target: id,
                        gesture: Gesture::ConnectHover(ConnectionTarget::None),
                    });
                }
            }
        }),
        On::<Pointer<Drop>>::run(
            move |mut event: ListenerMut<Pointer<Drop>>,
                  mut gesture_state: ResMut<GestureState>,
                  mut writer: EventWriter<GraphEvent>| {
                event.stop_propagation();
                if gesture_state.mode == DragMode::Connect {
                    gesture_state.mode = DragMode::None;
                    writer.send(GraphEvent {
                        target: id,
                        gesture: Gesture::ConnectFinish,
                    });
                }
            },
        ),
    )
}

#[derive(SystemParam)]
struct RelativeWorldPositions<'w, 's> {
    query: Query<'w, 's, (&'static Node, &'static GlobalTransform, &'static Parent)>,
}

impl<'w, 's> RelativeWorldPositions<'w, 's> {
    pub fn transform_relative(&self, id: Entity, pos: Vec2, levels: usize) -> Vec2 {
        let mut current = id;
        for _ in 0..levels {
            if let Ok((_, _, parent)) = self.query.get(current) {
                current = parent.get();
            } else {
                return pos;
            }
        }

        let Ok((node, transform, _)) = self.query.get(current) else {
            return pos;
        };

        let rect = node.logical_rect(transform);
        pos - rect.min
    }
}
