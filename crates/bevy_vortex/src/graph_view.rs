use crate::{
    graph::{Connection, GraphNode, GraphResource, NodeSelected, Terminal},
    operator::{DisplayName, DisplayWidth, OperatorInput, OperatorInputOnly, OperatorOutput},
    propedit::GraphNodePropertyEdit,
};
use bevy::{color::Color, prelude::*, reflect::TypeInfo, ui};
use bevy_mod_stylebuilder::*;
use bevy_quill::{prelude::*, Dynamic, IntoViewChild};
use bevy_quill_obsidian::{colors, hooks::UseElementRect};
use bevy_quill_obsidian_graph::{
    ConnectionAnchor, ConnectionTarget, EdgeDisplay, GraphDisplay, InputTerminalDisplay,
    NoTerminalDisplay, NodeDisplay, OutputTerminalDisplay,
};

fn style_node_graph(ss: &mut StyleBuilder) {
    ss.flex_grow(1.)
        .border_left(1)
        .border_color(Color::BLACK)
        .min_width(100);
}

/// Component which stores the entity id of the graph view. Used for programmatic scrolling.
#[derive(Component)]
pub struct GraphViewId(pub(crate) Entity);

/// Component which stores the current dragging state.
#[derive(Component, Default)]
pub struct DragState {
    /// The terminal we are dragging from
    pub(crate) connect_from: Option<ConnectionAnchor>,
    /// The terminal we are dragging to.
    pub(crate) connect_to: Option<ConnectionTarget>,
    /// Whether the dragged connection is valid.
    pub(crate) valid_connection: bool,
    /// The rectangle to display when selecting by dragging.
    pub(crate) selection_rect: Option<Rect>,
}

/// View template for graph. Entity is the id for the graph view.
#[derive(Clone, PartialEq)]
pub struct GraphView;

impl ViewTemplate for GraphView {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let graph = cx.use_resource::<GraphResource>();
        let node_ids: Vec<_> = graph.0.iter_nodes().map(|(_, v)| *v).collect();
        let connection_ids: Vec<_> = graph.0.iter_connections().cloned().collect();
        let graph_view_id = cx.use_inherited_component::<GraphViewId>().unwrap().0;

        GraphDisplay::new()
            .entity(graph_view_id)
            .style(style_node_graph)
            .children((
                SelectionRectView,
                For::each(connection_ids, |conn| ConnectionView(*conn)),
                For::each(node_ids, |node| GraphNodeView(*node)),
                ConnectionProxyView,
            ))
    }
}

#[derive(Clone, PartialEq)]
pub struct GraphNodeView(Entity);

impl ViewTemplate for GraphNodeView {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let display_id = cx.create_entity();
        let node_id = self.0;
        let size = cx.use_element_size(display_id);
        let node = cx.use_component::<GraphNode>(node_id).unwrap();
        if node.size != size.as_ivec2() {
            // Save the node size
            let mut entt = cx.world_mut().entity_mut(node_id);
            let mut node = entt.get_mut::<GraphNode>().unwrap();
            node.size = size.as_ivec2();
        }
        let node = cx.use_component::<GraphNode>(node_id).unwrap();

        let relative_rect = Rect::from_center_size(node.position.as_vec2(), size);
        let drag_state = cx.use_inherited_component::<DragState>().unwrap();
        let is_selected = cx.use_component::<NodeSelected>(node_id).is_some()
            || drag_state.selection_rect.map_or_else(
                || false,
                |r| r.contains(relative_rect.min) && r.contains(relative_rect.max),
            );
        let reflect = node.operator_reflect();
        let info = reflect.get_represented_type_info().unwrap();
        let TypeInfo::Struct(st_info) = info else {
            panic!("Expected StructInfo");
        };
        let display_width = match st_info.custom_attributes().get::<DisplayWidth>() {
            Some(dwidth) => ui::Val::Px(dwidth.0 as f32),
            None => ui::Val::Auto,
        };

        let field_names = {
            let num_fields = st_info.field_len();
            let mut names = Vec::with_capacity(num_fields);
            // Filter out field names for fields with a value of `None`.
            for findex in 0..num_fields {
                names.push(st_info.field_at(findex).unwrap().name());
            }
            names
        };

        NodeDisplay::new(display_id, node_id)
            .position(node.position)
            .width(display_width)
            .title(node.title())
            .selected(is_selected)
            .children(For::each(field_names, move |field| GraphNodePropertyView {
                node: node_id,
                field,
            }))
    }
}

#[derive(Clone, PartialEq)]
pub struct GraphNodePropertyView {
    node: Entity,
    field: &'static str,
}

impl ViewTemplate for GraphNodePropertyView {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        Dynamic::new({
            if cx.world().get_entity(self.node).is_none() {
                ().into_view_child()
            } else {
                let node = cx.use_component::<GraphNode>(self.node).unwrap();
                let reflect = node.operator_reflect();
                let info = reflect.get_represented_type_info().unwrap();
                let TypeInfo::Struct(st_info) = info else {
                    panic!("Expected StructInfo");
                };
                let field = st_info.field(self.field).unwrap();
                let field_attrs = field.custom_attributes();
                let display_name = if let Some(dname) = field_attrs.get::<DisplayName>() {
                    dname.0
                } else {
                    self.field
                };

                if field_attrs.contains::<OperatorInput>() {
                    let id = node.get_input_terminal(self.field).unwrap();
                    let terminal = cx.use_component::<Terminal>(id).unwrap();
                    InputTerminalDisplay {
                        id,
                        color: get_terminal_color(cx, id),
                        control: GraphNodePropertyEdit {
                            node: self.node,
                            display_name,
                            field: self.field,
                            editable: !(terminal.is_connected()
                                || field_attrs.contains::<OperatorInputOnly>()),
                        }
                        .into_view_child(),
                    }
                    .into_view_child()
                } else if field_attrs.contains::<OperatorOutput>() {
                    let id = node.get_output_terminal(self.field).unwrap();
                    OutputTerminalDisplay {
                        id,
                        color: get_terminal_color(cx, id),
                        label: display_name.to_string(),
                    }
                    .into_view_child()
                } else {
                    NoTerminalDisplay {
                        control: GraphNodePropertyEdit {
                            node: self.node,
                            display_name,
                            field: self.field,
                            editable: true,
                        }
                        .into_view_child(),
                    }
                    .into_view_child()
                }
            }
        })
    }
}

#[derive(Clone, PartialEq)]
pub struct ConnectionView(Entity);

impl ViewTemplate for ConnectionView {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let Some(connection) = cx.use_component::<Connection>(self.0) else {
            return EdgeDisplay {
                edge_id: None,
                src_pos: IVec2::default(),
                dst_pos: IVec2::default(),
                src_color: colors::U3,
                dst_color: colors::U3,
                hidden: true,
            };
        };
        let Connection { output, input } = connection;
        let src_pos = get_terminal_position(cx, output.terminal_id);
        let dst_pos = get_terminal_position(cx, input.terminal_id);
        let src_color = get_terminal_edge_color(cx, output.terminal_id);
        let dst_color = get_terminal_edge_color(cx, input.terminal_id);

        let drag_state = cx.use_inherited_component::<DragState>().unwrap();
        let hidden = match drag_state.connect_from {
            Some(ConnectionAnchor::EdgeSink(edge)) | Some(ConnectionAnchor::EdgeSource(edge)) => {
                edge == self.0
            }
            _ => false,
        };

        EdgeDisplay {
            edge_id: Some(self.0),
            src_pos,
            dst_pos,
            src_color,
            dst_color,
            hidden,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ConnectionProxyView;

impl ViewTemplate for ConnectionProxyView {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let drag_state = cx.use_inherited_component::<DragState>().unwrap();
        let (src_pos, dst_pos, src_color, dst_color) = match drag_state.connect_from {
            Some(ConnectionAnchor::OutputTerminal(term)) => (
                get_terminal_position(cx, term),
                get_target_position(cx, drag_state.connect_to),
                get_terminal_edge_color(cx, term),
                get_target_color(cx, drag_state.connect_to),
            ),
            Some(ConnectionAnchor::InputTerminal(term)) => (
                get_target_position(cx, drag_state.connect_to),
                get_terminal_position(cx, term),
                get_target_color(cx, drag_state.connect_to),
                get_terminal_edge_color(cx, term),
            ),
            Some(ConnectionAnchor::EdgeSink(edge)) => {
                // If we're dragging the sink, then the source end (output terminal) is anchored.
                let output = cx
                    .use_component::<Connection>(edge)
                    .unwrap()
                    .output
                    .terminal_id;
                (
                    get_terminal_position(cx, output),
                    get_target_position(cx, drag_state.connect_to),
                    get_terminal_edge_color(cx, output),
                    get_target_color(cx, drag_state.connect_to),
                )
            }
            Some(ConnectionAnchor::EdgeSource(edge)) => {
                // If we're dragging the source, then the sink end (input terminal) is anchored.
                let input = cx
                    .use_component::<Connection>(edge)
                    .unwrap()
                    .input
                    .terminal_id;
                (
                    get_target_position(cx, drag_state.connect_to),
                    get_terminal_position(cx, input),
                    get_target_color(cx, drag_state.connect_to),
                    get_terminal_edge_color(cx, input),
                )
            }
            None => (IVec2::default(), IVec2::default(), colors::U3, colors::U3),
        };
        Cond::new(
            drag_state.connect_from.is_some(),
            EdgeDisplay {
                edge_id: None,
                src_pos,
                dst_pos,
                src_color,
                dst_color,
                hidden: false,
            },
            (),
        )
    }
}

fn get_terminal_position(cx: &Cx, terminal_id: Entity) -> IVec2 {
    let rect = get_relative_rect(cx, terminal_id, 4);
    rect.map_or(IVec2::default(), |f| f.center().as_ivec2())
}

fn get_terminal_color(cx: &Cx, terminal_id: Entity) -> Srgba {
    if let Some(terminal) = cx.use_component::<Terminal>(terminal_id) {
        match terminal.data_type {
            crate::graph::ConnectionDataType::Scalar => colors::U4,
            crate::graph::ConnectionDataType::Vector => colors::LIGHT,
            crate::graph::ConnectionDataType::Color => colors::RESOURCE,
        }
    } else {
        colors::U3
    }
}

fn get_terminal_edge_color(cx: &Cx, terminal_id: Entity) -> Srgba {
    get_terminal_color(cx, terminal_id).mix(&Srgba::BLACK, 0.3)
}

fn get_target_position(cx: &Cx, target: Option<ConnectionTarget>) -> IVec2 {
    match target {
        Some(ConnectionTarget::InputTerminal(term) | ConnectionTarget::OutputTerminal(term)) => {
            get_terminal_position(cx, term)
        }
        Some(ConnectionTarget::Location(loc)) => loc.as_ivec2(),
        _ => IVec2::default(),
    }
}

fn get_target_color(cx: &Cx, target: Option<ConnectionTarget>) -> Srgba {
    match target {
        Some(ConnectionTarget::InputTerminal(term) | ConnectionTarget::OutputTerminal(term)) => {
            get_terminal_edge_color(cx, term)
        }
        _ => colors::U3,
    }
}

fn get_relative_rect(cx: &Cx, id: Entity, levels: usize) -> Option<Rect> {
    cx.world().get_entity(id)?;
    let node = cx.use_component::<Node>(id)?;
    let transform = cx.use_component::<GlobalTransform>(id)?;
    let mut rect = node.logical_rect(transform);
    let mut current = id;
    for _ in 0..levels {
        if let Some(parent) = cx.use_component::<Parent>(current) {
            current = parent.get();
        } else {
            return None;
        }
    }
    let node = cx.use_component::<Node>(current)?;
    let transform = cx.use_component::<GlobalTransform>(current)?;
    let ancestor_rect = node.logical_rect(transform);
    rect.min -= ancestor_rect.min;
    rect.max -= ancestor_rect.min;
    Some(rect)
}

fn style_selection_rect(ss: &mut StyleBuilder) {
    ss.background_color(colors::TEXT_SELECT.with_alpha(0.02))
        .border_color(colors::TEXT_SELECT.with_alpha(0.1))
        .border(2);
}

#[derive(Clone, PartialEq)]
struct SelectionRectView;

impl ViewTemplate for SelectionRectView {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let drag_state = cx.use_inherited_component::<DragState>().unwrap();

        Dynamic::new(
            drag_state
                .selection_rect
                .map(|rect| {
                    Element::<NodeBundle>::new()
                        .style(style_selection_rect)
                        .style_dyn(
                            |rect, sb| {
                                sb.left(rect.min.x)
                                    .top(rect.min.y)
                                    .width(rect.width())
                                    .height(rect.height());
                            },
                            rect,
                        )
                })
                .into_view_child(),
        )
    }
}
