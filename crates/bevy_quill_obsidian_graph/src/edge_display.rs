use std::ops::Mul;

use bevy::{prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::{
    StyleBuilder, StyleBuilderLayout, StyleBuilderPointerEvents, StyleBuilderVisibility,
};
use bevy_quill_core::prelude::*;
use bevy_quill_obsidian::cursor::StyleBuilderCursor;

use crate::{
    materials::{DrawPathMaterial, DrawablePath},
    relative_pos::RelativeWorldPositions,
    ConnectionAnchor, ConnectionTarget, DragAction, DragMode, Gesture, GestureState, GraphEvent,
};

fn style_edge(ss: &mut StyleBuilder) {
    ss.pointer_events(false);
}

fn style_edge_hitbox(ss: &mut StyleBuilder) {
    ss.position(ui::PositionType::Absolute)
        .width(24)
        .height(12)
        .pointer_events(true)
        // .border(2)
        // .border_color(colors::Y_GREEN)
        .cursor(CursorIcon::Grab);
}

/// Displays a stroked path between two nodes.
#[derive(Clone, PartialEq)]
pub struct EdgeDisplay {
    /// Entity id for the edge. If this is `None`, the edge is not pickable.
    pub edge_id: Option<Entity>,

    /// Pixel position of the source terminal.
    pub src_pos: IVec2,

    /// Color of the edge at the source terminal
    pub src_color: Srgba,

    /// Pixel position of the destination terminal.
    pub dst_pos: IVec2,

    /// Color of the edge at the destination terminal
    pub dst_color: Srgba,

    /// If true, the edge should not be displayed, but the display entities should still exist.
    pub hidden: bool,
}

impl ViewTemplate for EdgeDisplay {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let display_id = cx.create_entity();
        let material = cx.create_memo(
            |world, _| {
                let mut ui_materials = world
                    .get_resource_mut::<Assets<DrawPathMaterial>>()
                    .unwrap();
                ui_materials.add(DrawPathMaterial::default())
            },
            (),
        );
        let material_id = material.id();

        (
            Element::<MaterialNodeBundle<DrawPathMaterial>>::for_entity(display_id)
                .named("NodeGraph::Edge")
                .insert(material)
                .style(style_edge)
                .style_dyn(
                    |hidden, sb| {
                        sb.visible(!hidden);
                    },
                    self.hidden,
                )
                .effect(
                    move |cx, ent, (src, dst, src_color, dst_color)| {
                        let mut path = DrawablePath::new(1.7);
                        let dx = (dst.x - src.x).abs().mul(0.3).min(20.);
                        let src1 = src + Vec2::new(dx, 0.);
                        let dst1 = dst - Vec2::new(dx, 0.);
                        path.move_to(src);
                        // TODO: Marker
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
                        // TODO: Marker
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
                        material.update_path(&path);
                        material.update_color(
                            src_color,
                            src - bounds.min,
                            dst_color,
                            dst - bounds.min,
                        );
                    },
                    (
                        self.src_pos.as_vec2(),
                        self.dst_pos.as_vec2(),
                        self.src_color,
                        self.dst_color,
                    ),
                ),
            Cond::new(
                self.edge_id.is_some(),
                (
                    Element::<NodeBundle>::new()
                        .insert_dyn(edge_event_handlers, (self.edge_id, display_id, false))
                        .style(style_edge_hitbox)
                        .style_dyn(
                            |pos, sb| {
                                sb.left(pos.x).top(pos.y - 6.);
                            },
                            self.src_pos.as_vec2(),
                        ),
                    Element::<NodeBundle>::new()
                        .insert_dyn(edge_event_handlers, (self.edge_id, display_id, true))
                        .style(style_edge_hitbox)
                        .style_dyn(
                            |pos, sb| {
                                sb.left(pos.x - 24.).top(pos.y - 6.);
                            },
                            self.dst_pos.as_vec2(),
                        ),
                ),
                (),
            ),
        )
    }
}

#[allow(clippy::type_complexity)]
fn edge_event_handlers(
    args: (Option<Entity>, Entity, bool),
) -> (
    On<Pointer<DragStart>>,
    On<Pointer<Drag>>,
    On<Pointer<DragEnd>>,
) {
    let (edge_id, display_id, is_sink) = args;
    (
        On::<Pointer<DragStart>>::run(
            move |mut event: ListenerMut<Pointer<DragStart>>,
                  mut gesture_state: ResMut<GestureState>,
                  mut writer: EventWriter<GraphEvent>,
                  rel: RelativeWorldPositions| {
                event.stop_propagation();
                if gesture_state.mode != DragMode::Connect {
                    #[cfg(feature = "verbose")]
                    info!("Edge::DragStart: {}", event.target());
                    let edge_id = edge_id.unwrap();
                    let anchor = if is_sink {
                        ConnectionAnchor::EdgeSink(edge_id)
                    } else {
                        ConnectionAnchor::EdgeSource(edge_id)
                    };
                    gesture_state.target = ConnectionTarget::Location(rel.transform_relative(
                        display_id,
                        event.pointer_location.position,
                        4,
                    ));
                    gesture_state.mode = DragMode::Connect;
                    gesture_state.anchor = Some(anchor);
                    writer.send(GraphEvent {
                        target: display_id,
                        gesture: Gesture::Connect(anchor, gesture_state.target, DragAction::Start),
                    });
                }
            },
        ),
        On::<Pointer<Drag>>::run(
            move |mut event: ListenerMut<Pointer<Drag>>,
                  mut gesture_state: ResMut<GestureState>,
                  mut writer: EventWriter<GraphEvent>,
                  rel: RelativeWorldPositions| {
                event.stop_propagation();
                if gesture_state.mode == DragMode::Connect {
                    if let (Some(anchor), ConnectionTarget::Location(_)) =
                        (gesture_state.anchor, gesture_state.target)
                    {
                        gesture_state.target = ConnectionTarget::Location(rel.transform_relative(
                            display_id,
                            event.pointer_location.position,
                            4,
                        ));
                        writer.send(GraphEvent {
                            target: display_id,
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
                // if gesture_state.mode == DragMode::Connect {
                //     #[cfg(feature = "verbose")]
                //     info!("Edge::DragEnd: {}", event.target());
                //     gesture_state.mode = DragMode::None;
                //     writer.send(GraphEvent {
                //         target: display_id,
                //         gesture: Gesture::Cancel,
                //     });
                // }
                if gesture_state.mode == DragMode::Connect {
                    gesture_state.mode = DragMode::None;
                    match (gesture_state.anchor, gesture_state.target) {
                        (Some(_), ConnectionTarget::Location(_)) => {
                            #[cfg(feature = "verbose")]
                            info!("Edge::DragEnd [CANCEL]: {}", event.target());
                            writer.send(GraphEvent {
                                target: display_id,
                                gesture: Gesture::Cancel,
                            });
                        }
                        (Some(anchor), _) => {
                            #[cfg(feature = "verbose")]
                            info!("Edge::DragEnd: {}", event.target());
                            writer.send(GraphEvent {
                                target: display_id,
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
                    info!("Edge::DragEnd [IGNORED]: {}", event.target());
                }
            },
        ),
    )
}
