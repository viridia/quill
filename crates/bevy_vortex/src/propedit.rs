use crate::{
    graph::{GraphNode, NodeModified},
    operator::{OpValuePrecision, OpValueRange, OpValueStep},
};
use bevy::{prelude::*, reflect::TypeInfo, ui};
use bevy_mod_stylebuilder::*;
use bevy_quill::{prelude::*, Dynamic, IntoViewChild};
use bevy_quill_obsidian::{
    controls::{
        ColorEdit, ColorEditState, ColorMode, MenuButton, MenuPopup, Slider, SpinBox, Swatch,
    },
    floating::{FloatAlign, FloatSide},
    size::Size,
};

const NODE_PROP_HEIGHT: f32 = 20.;

#[derive(Clone, PartialEq)]
pub struct GraphNodePropertyEdit {
    pub node: Entity,
    pub display_name: &'static str,
    pub field: &'static str,
    pub editable: bool,
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
            "f32" | "i32" | "glam::Vec2" | "glam::Vec3" | "bevy_color::linear_rgba::LinearRgba"
                if !self.editable =>
            {
                Element::<NodeBundle>::new()
                    .style(|sb: &mut StyleBuilder| {
                        sb.min_width(128).height(NODE_PROP_HEIGHT);
                    })
                    .children(self.display_name)
                    .into_view_child()
            }
            "i32" => GraphNodePropertyEditI32 {
                node: self.node,
                display_name: self.display_name,
                field: self.field,
            }
            .into_view_child(),
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
pub struct GraphNodePropertyEditI32 {
    node: Entity,
    display_name: &'static str,
    field: &'static str,
}

impl ViewTemplate for GraphNodePropertyEditI32 {
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

        if let Some(range) = field_attrs.get::<OpValueRange<i32>>() {
            let mut slider = Slider::new()
                .range((*range.0.start() as f32)..=(*range.0.end() as f32))
                .value(*field_reflect.downcast_ref::<i32>().unwrap() as f32)
                .label(self.display_name)
                .style(|sb: &mut StyleBuilder| {
                    sb.flex_grow(1.).min_width(128);
                })
                .on_change(cx.create_callback(
                    move |value: In<f32>,
                          mut nodes: Query<&mut GraphNode>,
                          mut commands: Commands| {
                        let mut node = nodes.get_mut(id).unwrap();
                        let reflect = node.operator_reflect_mut();
                        let field_reflect = reflect.reflect_path_mut(path).unwrap();
                        field_reflect.apply((*value as i32).as_reflect());
                        commands.entity(id).insert(NodeModified);
                    },
                ));

            if let Some(precision) = field_attrs.get::<OpValuePrecision>() {
                slider = slider.precision(precision.0);
            }

            if let Some(step) = field_attrs.get::<OpValueStep<i32>>() {
                slider = slider.step(step.0 as f32);
            }

            slider.into_view_child()
        } else {
            // TODO: Need label
            let mut spinbox = SpinBox::new()
                .value(*field_reflect.downcast_ref::<i32>().unwrap() as f32)
                .style(|sb: &mut StyleBuilder| {
                    sb.flex_grow(1.);
                })
                .on_change(cx.create_callback(
                    move |value: In<f32>,
                          mut nodes: Query<&mut GraphNode>,
                          mut commands: Commands| {
                        let mut node = nodes.get_mut(id).unwrap();
                        let reflect = node.operator_reflect_mut();
                        let field_reflect = reflect.reflect_path_mut(path).unwrap();
                        field_reflect.apply((*value as i32).as_reflect());
                        commands.entity(id).insert(NodeModified);
                    },
                ));

            if let Some(precision) = field_attrs.get::<OpValuePrecision>() {
                spinbox = spinbox.precision(precision.0);
            }

            if let Some(step) = field_attrs.get::<OpValueStep<i32>>() {
                spinbox = spinbox.step(step.0 as f32);
            }

            spinbox.into_view_child()
        }
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
                    move |value: In<f32>,
                          mut nodes: Query<&mut GraphNode>,
                          mut commands: Commands| {
                        let mut node = nodes.get_mut(id).unwrap();
                        let reflect = node.operator_reflect_mut();
                        let field_reflect = reflect.reflect_path_mut(path).unwrap();
                        field_reflect.apply((*value).as_reflect());
                        commands.entity(id).insert(NodeModified);
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
                    move |value: In<f32>,
                          mut nodes: Query<&mut GraphNode>,
                          mut commands: Commands| {
                        let mut node = nodes.get_mut(id).unwrap();
                        let reflect = node.operator_reflect_mut();
                        let field_reflect = reflect.reflect_path_mut(path).unwrap();
                        field_reflect.apply((*value).as_reflect());
                        commands.entity(id).insert(NodeModified);
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
                                        world.entity_mut(node_id).insert(NodeModified);
                                    },
                                ),
                            )),
                    ),
            ))
    }
}
