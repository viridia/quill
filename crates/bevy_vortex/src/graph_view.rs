use crate::{
    graph::{GraphNode, GraphResource, Selected},
    operator::{DisplayName, OperatorInput, OperatorOutput},
};
use bevy::{color::Color, prelude::*, reflect::TypeInfo};
use bevy_mod_stylebuilder::*;
use bevy_quill::{prelude::*, IntoViewChild};
use bevy_quill_obsidian::colors;
use bevy_quill_obsidian_graph::{
    ConnectionAnchor, ConnectionTarget, EdgeDisplay, GraphDisplay, InputTerminalDisplay,
    NodeDisplay, OutputTerminalDisplay,
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
    /// Offset while dragging nodes
    pub(crate) offset: IVec2,
    pub(crate) connect_from: Option<ConnectionAnchor>,
    pub(crate) connect_to: Option<ConnectionTarget>,
}

/// View template for graph. Entity is the id for the graph view.
#[derive(Clone, PartialEq)]
pub struct GraphView;

impl ViewTemplate for GraphView {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let graph = cx.use_resource::<GraphResource>();
        let node_ids: Vec<_> = graph.0.iter_nodes().map(|(_, v)| *v).collect();
        let graph_view_id = cx.use_inherited_component::<GraphViewId>().unwrap().0;

        GraphDisplay::new()
            .entity(graph_view_id)
            .style(style_node_graph)
            .children((
                For::each(node_ids, |node| GraphNodeView(*node)),
                ConnectionProxyView,
                EdgeDisplay {
                    src_pos: IVec2::new(50, 50),
                    dst_pos: IVec2::new(400, 50),
                },
                EdgeDisplay {
                    src_pos: IVec2::new(50, 60),
                    dst_pos: IVec2::new(400, 70),
                },
                EdgeDisplay {
                    src_pos: IVec2::new(50, 70),
                    dst_pos: IVec2::new(400, 170),
                },
                EdgeDisplay {
                    src_pos: IVec2::new(50, 170),
                    dst_pos: IVec2::new(400, 70),
                },
            ))
    }
}

#[derive(Clone, PartialEq)]
pub struct GraphNodeView(Entity);

impl ViewTemplate for GraphNodeView {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let entity = self.0;
        // TODO: Using selection this way means re-rendering every node every time the selection
        // changes.
        let is_selected = cx
            .use_component::<Selected>(entity)
            .map_or_else(|| false, |s| s.0);
        let node = cx.use_component::<GraphNode>(entity).unwrap();
        let reflect = node.operator_reflect();
        let info = reflect.get_represented_type_info().unwrap();
        let TypeInfo::Struct(st_info) = info else {
            panic!("Expected StructInfo");
        };
        let drag_state = cx.use_inherited_component::<DragState>().unwrap();

        let field_names = {
            let num_fields = st_info.field_len();
            let mut names = Vec::with_capacity(num_fields);
            // Filter out field names for fields with a value of `None`.
            for findex in 0..num_fields {
                names.push(st_info.field_at(findex).unwrap().name());
            }
            names
        };

        // Offset position while dragging.
        let position = if is_selected {
            node.position + drag_state.offset
        } else {
            node.position
        };

        NodeDisplay::new(entity)
            .position(position)
            .title(node.title())
            .selected(is_selected)
            // .on_drag(
            //     cx.create_callback(move |new_pos: In<Vec2>, world: &mut World| {
            //         let mut entt = world.entity_mut(entity);
            //         let mut pos = entt.get_mut::<GraphNode>().unwrap();
            //         pos.position = new_pos.as_ivec2();
            //     }),
            // )
            .children(For::each(field_names, move |field| GraphNodePropertyView {
                node: entity,
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
            InputTerminalDisplay {
                color: colors::RESOURCE,
                control: display_name.to_owned().into_view_child(),
                id: node.get_input_terminal(self.field).unwrap().id(),
            }
            .into_view_child()
        } else if field_attrs.contains::<OperatorOutput>() {
            OutputTerminalDisplay {
                id: node.get_output_terminal(self.field).unwrap().id(),
                label: display_name.to_string(),
                color: colors::LIGHT,
            }
            .into_view_child()
        } else {
            display_name.into_view_child()
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ConnectionProxyView;

impl ViewTemplate for ConnectionProxyView {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let drag_state = cx.use_inherited_component::<DragState>().unwrap();
        let (src_pos, dst_pos) = match drag_state.connect_from {
            Some(ConnectionAnchor::OutputTerminal(term)) => (
                get_terminal_position(cx, term),
                get_target_position(cx, drag_state.connect_to),
            ),
            Some(ConnectionAnchor::InputTerminal(term)) => (
                get_target_position(cx, drag_state.connect_to),
                get_terminal_position(cx, term),
            ),
            Some(ConnectionAnchor::EdgeSource(edge)) => todo!(),
            Some(ConnectionAnchor::EdgeSink(edge)) => todo!(),
            None => (IVec2::default(), IVec2::default()),
        };
        // println!("src_pos: {src_pos}, dst_pos: {dst_pos}");
        Cond::new(
            drag_state.connect_from.is_some() && drag_state.connect_to.is_some(),
            EdgeDisplay { src_pos, dst_pos },
            (),
        )
    }
}

fn get_terminal_position(cx: &Cx, terminal_id: Entity) -> IVec2 {
    let rect = get_relative_rect(cx, terminal_id, 4);
    rect.map_or(IVec2::default(), |f| f.center().as_ivec2())
}

fn get_target_position(cx: &Cx, target: Option<ConnectionTarget>) -> IVec2 {
    match target {
        Some(ConnectionTarget::InputTerminal(term)) => get_terminal_position(cx, term),
        Some(ConnectionTarget::OutputTerminal(term)) => get_terminal_position(cx, term),
        Some(ConnectionTarget::Position(pos)) => pos.as_ivec2(),
        None => IVec2::default(),
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
