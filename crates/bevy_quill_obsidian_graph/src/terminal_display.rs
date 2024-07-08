use bevy::{prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill::{prelude::*, ViewChild};
use bevy_quill_obsidian::{colors, cursor::StyleBuilderCursor, hooks::UseIsHover};

use crate::{
    ConnectionAnchor, ConnectionTarget, DragAction, DragMode, Gesture, GestureState, GraphEvent,
};

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
        .pointer_events(true)
        .cursor(CursorIcon::Copy);
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

fn style_no_connector(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::Stretch)
        .min_height(20)
        .padding((8, 0));
}

/// Entry for a property that is neither an input or output terminal, but which can be edited.
#[derive(Clone, PartialEq)]
pub struct NoTerminalDisplay {
    /// Control rendered when the input is not connected.
    pub control: ViewChild,
}

impl ViewTemplate for NoTerminalDisplay {
    type View = impl View;

    fn create(&self, _cx: &mut Cx) -> Self::View {
        Element::<NodeBundle>::new()
            .named("NoTerminal")
            .style(style_no_connector)
            .children(self.control.clone())
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
                  mut writer: EventWriter<GraphEvent>,
                  rel: crate::relative_pos::RelativeWorldPositions| {
                event.stop_propagation();
                if gesture_state.mode != DragMode::Connect {
                    #[cfg(feature = "verbose")]
                    info!("Terminal::DragStart: {}", event.target());
                    let anchor = if is_output {
                        ConnectionAnchor::OutputTerminal(id)
                    } else {
                        ConnectionAnchor::InputTerminal(id)
                    };
                    gesture_state.target = ConnectionTarget::Location(rel.transform_relative(
                        id,
                        event.pointer_location.position,
                        4,
                    ));
                    gesture_state.mode = DragMode::Connect;
                    gesture_state.anchor = Some(anchor);
                    writer.send(GraphEvent {
                        target: id,
                        gesture: Gesture::Connect(anchor, gesture_state.target, DragAction::Start),
                    });
                } else {
                    #[cfg(feature = "verbose")]
                    info!("Terminal::DragStart [IGNORED]: {}", event.target());
                }
            },
        ),
        On::<Pointer<Drag>>::run(
            move |mut event: ListenerMut<Pointer<Drag>>,
                  mut gesture_state: ResMut<GestureState>,
                  mut writer: EventWriter<GraphEvent>,
                  rel: crate::relative_pos::RelativeWorldPositions| {
                event.stop_propagation();
                if gesture_state.mode == DragMode::Connect {
                    if let (Some(anchor), ConnectionTarget::Location(_)) =
                        (gesture_state.anchor, gesture_state.target)
                    {
                        gesture_state.target = ConnectionTarget::Location(rel.transform_relative(
                            id,
                            event.pointer_location.position,
                            4,
                        ));
                        writer.send(GraphEvent {
                            target: id,
                            gesture: Gesture::Connect(
                                anchor,
                                gesture_state.target,
                                DragAction::Update,
                            ),
                        });
                    }
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
                    match (gesture_state.anchor, gesture_state.target) {
                        (Some(_), ConnectionTarget::Location(_)) => {
                            #[cfg(feature = "verbose")]
                            info!("Terminal::DragEnd [CANCEL]: {}", event.target());
                            writer.send(GraphEvent {
                                target: id,
                                gesture: Gesture::Cancel,
                            });
                        }
                        (Some(anchor), _) => {
                            #[cfg(feature = "verbose")]
                            info!("Terminal::DragEnd: {}", event.target());
                            writer.send(GraphEvent {
                                target: id,
                                gesture: Gesture::Connect(
                                    anchor,
                                    gesture_state.target,
                                    DragAction::Finish,
                                ),
                            });
                        }
                        _ => {}
                    }
                    gesture_state.anchor = None;
                    gesture_state.target = ConnectionTarget::None;
                    gesture_state.mode = DragMode::None;
                } else {
                    #[cfg(feature = "verbose")]
                    info!("Terminal::DragEnd [IGNORED]: {}", event.target());
                }
            },
        ),
        On::<Pointer<DragEnter>>::run({
            move |mut event: ListenerMut<Pointer<DragEnter>>,
                  mut gesture_state: ResMut<GestureState>,
                  mut writer: EventWriter<GraphEvent>| {
                event.stop_propagation();
                if gesture_state.mode == DragMode::Connect {
                    #[cfg(feature = "verbose")]
                    info!("Terminal::DragEnter: {}", event.target());
                    if is_output {
                        gesture_state.target = ConnectionTarget::OutputTerminal(id);
                    } else {
                        gesture_state.target = ConnectionTarget::InputTerminal(id);
                    }
                    if let Some(anchor) = gesture_state.anchor {
                        writer.send(GraphEvent {
                            target: id,
                            gesture: Gesture::Connect(
                                anchor,
                                gesture_state.target,
                                DragAction::Update,
                            ),
                        });
                    }
                } else {
                    #[cfg(feature = "verbose")]
                    info!("Terminal::DragEnter [IGNORED]: {}", event.target());
                }
            }
        }),
        On::<Pointer<DragLeave>>::run({
            move |mut event: ListenerMut<Pointer<DragLeave>>,
                  mut gesture_state: ResMut<GestureState>,
                  mut writer: EventWriter<GraphEvent>,
                  rel: crate::relative_pos::RelativeWorldPositions| {
                event.stop_propagation();
                if gesture_state.mode == DragMode::Connect {
                    #[cfg(feature = "verbose")]
                    info!("Terminal::DragLeave: {}", event.target());
                    gesture_state.target = match gesture_state.target {
                        ConnectionTarget::OutputTerminal(target_id)
                        | ConnectionTarget::InputTerminal(target_id)
                            if id == target_id =>
                        {
                            ConnectionTarget::Location(rel.transform_relative(
                                id,
                                event.pointer_location.position,
                                4,
                            ))
                        }
                        _ if is_output => ConnectionTarget::OutputTerminal(id),
                        _ => ConnectionTarget::InputTerminal(id),
                    };
                    if let Some(anchor) = gesture_state.anchor {
                        writer.send(GraphEvent {
                            target: id,
                            gesture: Gesture::Connect(
                                anchor,
                                gesture_state.target,
                                DragAction::Update,
                            ),
                        });
                    }
                } else {
                    #[cfg(feature = "verbose")]
                    info!("Terminal::DragLeave [IGNORED]: {}", event.target());
                }
            }
        }),
        On::<Pointer<Drop>>::run(
            move |mut event: ListenerMut<Pointer<Drop>>,
                  mut gesture_state: ResMut<GestureState>,
                  mut writer: EventWriter<GraphEvent>| {
                event.stop_propagation();
                if gesture_state.mode == DragMode::Connect {
                    #[cfg(feature = "verbose")]
                    info!("Terminal::Drop: {}", event.target());
                    if let Some(anchor) = gesture_state.anchor {
                        writer.send(GraphEvent {
                            target: id,
                            gesture: Gesture::Connect(
                                anchor,
                                gesture_state.target,
                                DragAction::Finish,
                            ),
                        });
                    }
                    gesture_state.anchor = None;
                    gesture_state.target = ConnectionTarget::None;
                    gesture_state.mode = DragMode::None;
                } else {
                    #[cfg(feature = "verbose")]
                    info!("Terminal::Drop [IGNORED]: {}", event.target());
                }
            },
        ),
    )
}
