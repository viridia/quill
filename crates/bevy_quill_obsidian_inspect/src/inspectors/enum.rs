use std::sync::Arc;

use bevy::{
    ecs::reflect::AppTypeRegistry,
    prelude::World,
    reflect::{
        std_traits::ReflectDefault, DynamicEnum, DynamicStruct, DynamicTuple, DynamicVariant,
        OffsetAccess, ReflectRef, TypeInfo, TypeRegistry, VariantInfo, VariantType,
    },
};
use bevy_quill::*;
use bevy_quill_obsidian::{
    controls::{MenuButton, MenuItem, MenuPopup},
    floating::{FloatAlign, FloatSide},
    size::Size,
};

use crate::{templates::field_label::FieldLabel, Inspectable, InspectorFactoryRegistry};

#[derive(Clone)]
pub struct EnumInspector(pub(crate) Arc<Inspectable>);

impl PartialEq for EnumInspector {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl ViewTemplate for EnumInspector {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        (
            FieldLabel {
                field: self.0.clone(),
            },
            VariantSelector {
                target: self.0.clone(),
            },
            EnumContentInspector(self.0.clone()),
        )
    }
}

#[derive(Clone)]
pub struct VariantSelector {
    pub target: Arc<Inspectable>,
}

impl PartialEq for VariantSelector {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.target, &other.target)
    }
}

impl ViewTemplate for VariantSelector {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let target = self.target.clone();
        let variant_name = match target.reflect(cx) {
            Some(reflect) => {
                if let ReflectRef::Enum(en) = reflect.reflect_ref() {
                    en.variant_name().to_string()
                } else {
                    "".to_string()
                }
            }
            None => "".to_string(),
        };

        let target = self.target.clone();
        Dynamic::new(
            match target
                .reflect(cx)
                .unwrap()
                .get_represented_type_info()
                .unwrap()
            {
                TypeInfo::Enum(en) => {
                    let num_variants = en.variant_len();
                    let mut items: Vec<ViewChild> = Vec::new();
                    let registry = cx.world().resource::<AppTypeRegistry>().0.clone();
                    let registry_lock = registry.read();
                    for findex in 0..num_variants {
                        let variant = en.variant_at(findex).unwrap();
                        let variant_default = variant_default_value(variant, &registry_lock);
                        if variant_default.is_none() {
                            continue;
                        }
                        items.push(
                            SetVariantItem {
                                field: target.clone(),
                                variant_name: variant.name().to_string(),
                                variant_index: findex,
                            }
                            .into_view_child(),
                        );
                    }

                    if !items.is_empty() {
                        let variant_name = variant_name.clone();
                        MenuButton::new()
                            .children(variant_name)
                            .popup(
                                MenuPopup::new()
                                    .side(FloatSide::Bottom)
                                    .align(FloatAlign::End)
                                    .children(items),
                            )
                            .size(Size::Sm)
                            .into_view_child()
                    } else {
                        ().into_view_child()
                    }
                }
                _ => {
                    println!(
                        "Fallback: {}",
                        target.reflect(cx).unwrap().reflect_type_path()
                    );
                    ().into_view_child()
                }
            },
        )
    }
}

#[derive(Clone)]
struct SetVariantItem {
    field: Arc<Inspectable>,
    variant_name: String,
    variant_index: usize,
}

impl PartialEq for SetVariantItem {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.field, &other.field)
            && self.variant_name == other.variant_name
            && self.variant_index == other.variant_index
    }
}

impl ViewTemplate for SetVariantItem {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let field = self.field.clone();
        let variant_index = self.variant_index;
        let callback = cx.create_callback(move |world: &mut World| {
            let Some(field_reflect) = field.reflect_untracked(world) else {
                return;
            };
            let Some(TypeInfo::Enum(enum_info)) = field_reflect.get_represented_type_info() else {
                panic!("Expected TypeInfo::Enum");
            };

            let registry = world.resource::<AppTypeRegistry>().0.clone();
            let variant = enum_info.variant_at(variant_index).unwrap();
            let registry_lock = registry.read();
            let variant_default = variant_default_value(variant, &registry_lock);
            if let Some(def) = variant_default {
                field.set_value(world, &def);
            } else {
                println!("Can't find ReflectDefault for: {:?}", variant.name());
            }
        });
        MenuItem::new()
            .label(self.variant_name.clone())
            .on_click(callback)
    }
}

fn variant_default_value(variant: &VariantInfo, registry: &TypeRegistry) -> Option<DynamicEnum> {
    match variant {
        bevy::reflect::VariantInfo::Struct(st) => {
            let mut ds = DynamicStruct::default();
            for field in 0..st.field_len() {
                let f = st.field_at(field).unwrap();
                // let field_type = registry.get_type_info(f.type_id()).unwrap();
                let field_type_default = registry.get_type_data::<ReflectDefault>(f.type_id());
                if let Some(default) = field_type_default {
                    let default = default.default();
                    ds.insert_boxed(f.name(), default);
                } else {
                    return None;
                }
            }
            Some(DynamicEnum::new(variant.name(), ds))
        }
        bevy::reflect::VariantInfo::Tuple(tpl) => {
            let mut dt = DynamicTuple::default();
            for field in 0..tpl.field_len() {
                let f = tpl.field_at(field).unwrap();
                // let field_type = registry.get_type_info(f.type_id()).unwrap();
                let field_type_default = registry.get_type_data::<ReflectDefault>(f.type_id());
                if let Some(default) = field_type_default {
                    let default = default.default();
                    dt.insert_boxed(default);
                } else {
                    return None;
                }
            }
            Some(DynamicEnum::new(variant.name(), dt))
        }
        bevy::reflect::VariantInfo::Unit(_) => {
            Some(DynamicEnum::new(variant.name(), DynamicVariant::Unit))
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct EnumContentInspector(pub(crate) Arc<Inspectable>);

impl ViewTemplate for EnumContentInspector {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let field = self.0.clone();
        let factories = cx.use_resource::<InspectorFactoryRegistry>();
        Dynamic::new(if let Some(reflect) = field.reflect(cx) {
            if let ReflectRef::Enum(en) = reflect.reflect_ref() {
                let variant = en.variant_type();
                match variant {
                    VariantType::Struct => {
                        let mut fields: Vec<ViewChild> = Vec::new();
                        for findex in 0..en.field_len() {
                            let name = en.name_at(findex).unwrap().to_string();
                            let mut path = field.value_path.clone();
                            path.0.push(OffsetAccess {
                                access: bevy::reflect::Access::Field(name.clone().into()),
                                offset: None,
                            });

                            let access = Arc::new(Inspectable {
                                root: field.root.clone(),
                                name: name.clone(),
                                value_path: path,
                                field_path: field.value_path.clone(),
                                can_remove: false,
                                attributes: field.attributes,
                            });
                            if let Some(view_ref) = factories.create_inspector(cx, access) {
                                fields.push(Dynamic::new(view_ref).into_view_child());
                            }
                        }
                        fields.into_view_child()
                    }

                    VariantType::Tuple => {
                        let mut fields: Vec<ViewChild> = Vec::new();
                        for findex in 0..en.field_len() {
                            // let variant = en.field_at(findex).unwrap();
                            let mut path = field.value_path.clone();
                            path.0.push(OffsetAccess {
                                access: bevy::reflect::Access::TupleIndex(findex),
                                offset: None,
                            });

                            let access = Arc::new(Inspectable {
                                root: field.root.clone(),
                                name: if en.field_len() > 1 {
                                    format!("{}", findex)
                                } else {
                                    "".to_string()
                                },
                                value_path: path.clone(),
                                field_path: path,
                                can_remove: false,
                                attributes: field.attributes,
                            });
                            if let Some(view_ref) = factories.create_inspector(cx, access) {
                                fields.push(Dynamic::new(view_ref).into_view_child());
                            }
                        }
                        fields.into_view_child()
                    }

                    VariantType::Unit => ().into_view_child(),
                }
            } else {
                ().into_view_child()
            }
        } else {
            ().into_view_child()
        })
    }
}
