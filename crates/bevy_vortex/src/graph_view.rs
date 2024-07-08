use crate::{
    graph::{Connection, GraphNode, GraphResource, Selected, Terminal},
    operator::{
        DisplayName, DisplayWidth, OpValuePrecision, OpValueRange, OpValueStep, OperatorInput,
        OperatorInputOnly, OperatorOutput,
    },
};
use bevy::{color::Color, prelude::*, reflect::TypeInfo, ui};
use bevy_mod_stylebuilder::*;
use bevy_quill::{prelude::*, Dynamic, IntoViewChild};
use bevy_quill_obsidian::{
    colors,
    controls::{
        ColorEdit, ColorEditState, ColorMode, MenuButton, MenuPopup, Slider, SpinBox, Swatch,
    },
    floating::{FloatAlign, FloatSide},
    size::Size,
};
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

const NODE_PROP_HEIGHT: f32 = 20.;

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
                // TODO: Selection rect.
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
        let entity = self.0;
        let is_selected = cx
            .use_component::<Selected>(entity)
            .map_or_else(|| false, |s| s.0);
        let node = cx.use_component::<GraphNode>(entity).unwrap();
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

        NodeDisplay::new(entity)
            .position(node.position)
            .width(display_width)
            .title(node.title())
            .selected(is_selected)
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
pub struct GraphNodePropertyEdit {
    node: Entity,
    display_name: &'static str,
    field: &'static str,
    editable: bool,
}

impl ViewTemplate for GraphNodePropertyEdit {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let node = cx.use_component::<GraphNode>(self.node).unwrap();
        let reflect = node.operator_reflect();
        let Some(TypeInfo::Struct(st_info)) = reflect.get_represented_type_info() else {
            panic!("Expected StructInfo");
        };
        let field = st_info.field(self.field).unwrap();

        Dynamic::new(match field.type_path() {
            "f32" | "bevy_color::linear_rgba::LinearRgba" if !self.editable => {
                Element::<NodeBundle>::new()
                    .style(|sb: &mut StyleBuilder| {
                        sb.min_width(128).height(NODE_PROP_HEIGHT);
                    })
                    .children(self.display_name)
                    .into_view_child()
            }
            "f32" => GraphNodePropertyEditF32 {
                node: self.node,
                display_name: self.display_name,
                field: self.field,
            }
            .into_view_child(),
            "bevy_color::linear_rgba::LinearRgba" => GraphNodePropertyEditLinearRgba {
                node: self.node,
                display_name: self.display_name,
                field: self.field,
            }
            .into_view_child(),

            _ => {
                warn!("Unsupported type: {}", field.type_path());
                self.display_name.into_view_child()
            }
        })
    }
}

#[derive(Clone, PartialEq)]
pub struct GraphNodePropertyEditF32 {
    node: Entity,
    display_name: &'static str,
    field: &'static str,
}

impl ViewTemplate for GraphNodePropertyEditF32 {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let id = self.node;
        let path = self.field;
        let node = cx.use_component::<GraphNode>(self.node).unwrap();
        let reflect = node.operator_reflect();
        let Some(TypeInfo::Struct(st_info)) = reflect.get_represented_type_info() else {
            panic!("Expected StructInfo");
        };
        let field = st_info.field(self.field).unwrap();
        let field_attrs = field.custom_attributes();
        let field_reflect = reflect.reflect_path(path).unwrap();

        if let Some(range) = field_attrs.get::<OpValueRange<f32>>() {
            let mut slider = Slider::new()
                .range(range.0.clone())
                .value(*field_reflect.downcast_ref::<f32>().unwrap())
                .label(self.display_name)
                .style(|sb: &mut StyleBuilder| {
                    sb.flex_grow(1.).min_width(128);
                })
                .on_change(cx.create_callback(
                    move |value: In<f32>, mut nodes: Query<&mut GraphNode>| {
                        let mut node = nodes.get_mut(id).unwrap();
                        let reflect = node.operator_reflect_mut();
                        let field_reflect = reflect.reflect_path_mut(path).unwrap();
                        field_reflect.apply((*value).as_reflect());
                    },
                ));

            if let Some(precision) = field_attrs.get::<OpValuePrecision>() {
                slider = slider.precision(precision.0);
            }

            if let Some(step) = field_attrs.get::<OpValueStep<f32>>() {
                slider = slider.step(step.0);
            }

            slider.into_view_child()
        } else {
            // TODO: Need label
            let mut spinbox = SpinBox::new()
                .value(*field_reflect.downcast_ref::<f32>().unwrap())
                .style(|sb: &mut StyleBuilder| {
                    sb.flex_grow(1.);
                })
                .on_change(cx.create_callback(
                    move |value: In<f32>, mut nodes: Query<&mut GraphNode>| {
                        let mut node = nodes.get_mut(id).unwrap();
                        let reflect = node.operator_reflect_mut();
                        let field_reflect = reflect.reflect_path_mut(path).unwrap();
                        field_reflect.apply((*value).as_reflect());
                    },
                ));

            if let Some(precision) = field_attrs.get::<OpValuePrecision>() {
                spinbox = spinbox.precision(precision.0);
            }

            if let Some(step) = field_attrs.get::<OpValueStep<f32>>() {
                spinbox = spinbox.step(step.0);
            }

            spinbox.into_view_child()
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct GraphNodePropertyEditLinearRgba {
    node: Entity,
    display_name: &'static str,
    field: &'static str,
}

impl ViewTemplate for GraphNodePropertyEditLinearRgba {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let node_id = self.node;
        let node = cx.use_component::<GraphNode>(node_id).unwrap();
        let reflect = node.operator_reflect();
        let field_name = self.field;
        let field_reflect = reflect.reflect_path(self.field).unwrap();
        let color = *field_reflect.downcast_ref::<LinearRgba>().unwrap();

        let state = cx.create_mutable(ColorEditState {
            mode: ColorMode::Rgb,
            rgb: Srgba::default(),
            hsl: Hsla::default(),
        });

        Element::<NodeBundle>::new()
            .style(|sb: &mut StyleBuilder| {
                sb.gap(4).justify_items(ui::JustifyItems::End);
            })
            .children((
                Element::<NodeBundle>::new()
                    .style(|sb: &mut StyleBuilder| {
                        sb.flex_grow(1.0).flex_basis(0);
                    })
                    .children(self.display_name),
                MenuButton::new()
                    .minimal(true)
                    .no_caret(true)
                    .size(Size::Xxs)
                    .style(|sb: &mut StyleBuilder| {
                        sb.padding(0).min_width(64).height(NODE_PROP_HEIGHT);
                    })
                    .children(Swatch::new(color).style(|sb: &mut StyleBuilder| {
                        sb.align_self(ui::AlignSelf::Stretch)
                            .justify_self(ui::JustifySelf::Stretch)
                            .flex_grow(1.0);
                    }))
                    .popup(
                        MenuPopup::new()
                            .side(FloatSide::Right)
                            .align(FloatAlign::Start)
                            .children(ColorEdit::new(
                                state.get(cx),
                                cx.create_callback(
                                    move |st: In<ColorEditState>, world: &mut World| {
                                        state.set(world, *st);

                                        let mut node_entt = world.entity_mut(node_id);
                                        let mut node = node_entt.get_mut::<GraphNode>().unwrap();
                                        let reflect = node.operator_reflect_mut();
                                        let field_reflect =
                                            reflect.reflect_path_mut(field_name).unwrap();
                                        field_reflect.apply(LinearRgba::from(st.rgb).as_reflect());
                                    },
                                ),
                            )),
                    ),
            ))
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
            };
        };
        let Connection { output, input } = connection;
        let src_pos = get_terminal_position(cx, output.terminal_id);
        let dst_pos = get_terminal_position(cx, input.terminal_id);
        let src_color = get_terminal_edge_color(cx, output.terminal_id);
        let dst_color = get_terminal_edge_color(cx, input.terminal_id);

        EdgeDisplay {
            edge_id: Some(self.0),
            src_pos,
            dst_pos,
            src_color,
            dst_color,
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
            Some(ConnectionAnchor::EdgeSource(_edge)) => todo!(),
            Some(ConnectionAnchor::EdgeSink(_edge)) => todo!(),
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
