use std::sync::Arc;

use bevy::{
    color::Color,
    ecs::reflect::AppTypeRegistry,
    prelude::{In, World},
    reflect::{
        std_traits::ReflectDefault, DynamicEnum, DynamicTuple, OffsetAccess, ReflectKind,
        ReflectRef, TypeInfo, VariantInfo,
    },
    ui::{self, node_bundles::NodeBundle},
};
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use quill_obsidian::{
    colors,
    controls::{DisclosureToggle, Icon, MenuButton, MenuItem, MenuPopup},
    floating::FloatAlign,
    size::Size,
};

use crate::{templates::field_label::FieldLabelWide, Inspectable, InspectorFactoryRegistry};

#[derive(Clone)]
pub struct NestedStruct(pub(crate) Arc<Inspectable>);

impl PartialEq for NestedStruct {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

fn style_field_list(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Grid)
        .grid_auto_flow(ui::GridAutoFlow::Row)
        .grid_template_columns(vec![
            ui::RepeatedGridTrack::auto(1),
            ui::RepeatedGridTrack::flex(1, 1.),
        ])
        .column_gap(4)
        .row_gap(2)
        .align_items(ui::AlignItems::Stretch)
        .grid_column_span(2)
        .min_width(64)
        .color(colors::DIM)
        .margin_left(16)
        .margin_top(4)
        .margin_bottom(4);
}

impl ViewTemplate for NestedStruct {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let field = self.0.clone();
        let expanded = cx.create_mutable(false);

        (
            FieldLabelWide::new(field.clone())
                .name((
                    DisclosureToggle::new()
                        .size(Size::Xs)
                        .expanded(expanded.get(cx))
                        .on_change(cx.create_callback(
                            move |value: In<bool>, world: &mut World| {
                                expanded.set(world, *value);
                            },
                        )),
                    field.name.clone(),
                ))
                .buttons(StructInspectorHeaderControls {
                    target: self.0.clone(),
                    // expanded,
                }),
            Cond::new(
                expanded.get(cx),
                Element::<NodeBundle>::new()
                    .style(style_field_list)
                    .children(StructFieldList(self.0.clone())),
                (),
            ),
        )
    }
}

#[derive(Clone)]
pub struct StructFieldList(pub Arc<Inspectable>);

impl PartialEq for StructFieldList {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl ViewTemplate for StructFieldList {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let target = self.0.clone();
        let reflect = target.reflect(cx).unwrap();
        let info = reflect.get_represented_type_info().unwrap();

        // Get the memoized field names of the struct, minus missing optionals. This should
        // isolate the field editors from each other so that they don't constantly update.
        // We will still need to memoize the individual field values.
        // TODO: We had to un-memo this. Think of a way to memoize this again.
        let field_names = {
            let ReflectRef::Struct(st) = target.reflect(cx).unwrap().reflect_ref() else {
                panic!("Expected ReflectRef::Struct")
            };
            let num_fields = st.field_len();
            let mut names = Vec::with_capacity(num_fields);
            // Filter out field names for fields with a value of `None`.
            for findex in 0..num_fields {
                let field = st.field_at(findex).unwrap();
                // let info = st.get_represented_type_info().unwrap()
                if field.reflect_kind() == ReflectKind::Enum
                    && field
                        .reflect_type_path()
                        .starts_with("core::option::Option")
                {
                    let ReflectRef::Enum(enum_ref) = field.reflect_ref() else {
                        panic!("Expected ReflectRef::Enum");
                    };
                    if enum_ref.variant_name() != "None" {
                        names.push(st.name_at(findex).unwrap().to_string());
                    }
                } else {
                    names.push(st.name_at(findex).unwrap().to_string());
                }
            }
            names
        };

        let target = self.0.clone();
        For::each(field_names, move |name| {
            let mut path = target.field_path.clone();
            path.0.push(OffsetAccess {
                access: bevy::reflect::Access::Field(name.clone().into()),
                offset: None,
            });
            let TypeInfo::Struct(st_info) = info else {
                panic!("Expected StructInfo");
            };
            let field_info = st_info.field(name).unwrap();
            let attrs = field_info.custom_attributes();
            let field = Arc::new(Inspectable {
                root: target.root.clone(),
                name: name.to_string(),
                value_path: path.clone(),
                field_path: path,
                can_remove: false,
                attributes: Some(attrs),
            });
            NamedFieldInspector { field }
        })
    }
}

#[derive(Clone)]
struct NamedFieldInspector {
    field: Arc<Inspectable>,
}

impl PartialEq for NamedFieldInspector {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.field, &other.field)
    }
}

impl ViewTemplate for NamedFieldInspector {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let access = cx.create_memo(
            |world, field| match field.reflect_untracked(world) {
                Some(reflect) => {
                    if reflect.reflect_kind() == ReflectKind::Enum
                        && reflect
                            .reflect_type_path()
                            .starts_with("core::option::Option")
                    {
                        let ReflectRef::Enum(enum_ref) = reflect.reflect_ref() else {
                            panic!("Expected ReflectRef::Enum");
                        };
                        if enum_ref.variant_name() != "None" {
                            let mut path = field.value_path.clone();
                            path.0.push(OffsetAccess {
                                access: bevy::reflect::Access::TupleIndex(0),
                                offset: None,
                            });

                            Some(Arc::new(Inspectable {
                                root: field.root.clone(),
                                name: field.name.clone(),
                                value_path: path,
                                field_path: field.value_path.clone(),
                                can_remove: true,
                                attributes: field.attributes,
                            }))
                        } else {
                            None
                        }
                    } else {
                        Some(field.clone())
                    }
                }
                _ => None,
            },
            self.field.clone(),
        );

        let factories = cx.use_resource::<InspectorFactoryRegistry>();
        // let field = self.field.clone();
        Dynamic::new(
            access
                .map(|a| factories.create_inspector(cx, a))
                .into_view_child(),
        )
    }
}

#[derive(Clone)]
pub struct StructInspectorHeaderControls {
    pub target: Arc<Inspectable>,
}

impl PartialEq for StructInspectorHeaderControls {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.target, &other.target)
    }
}

impl ViewTemplate for StructInspectorHeaderControls {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let target = self.target.clone();
        // Get the memoized field names of the struct, including only the missing optionals.
        let missing = missing_fields(target, cx, self.target.field_path.clone());

        Cond::new(
            !missing.is_empty(),
            MenuButton::new()
                .children(
                    Icon::new("obsidian_ui://icons/add_box.png")
                        .color(Color::from(colors::DIM))
                        .style(style_menu_icon),
                )
                .popup(MenuPopup::new().align(FloatAlign::End).children(For::each(
                    missing,
                    |item| AddStructFieldItem {
                        field: item.clone(),
                    },
                )))
                .size(Size::Xs)
                .minimal(true),
            (),
        )
    }
}

fn missing_fields(
    target: Arc<Inspectable>,
    cx: &mut Cx,
    base_path: bevy::reflect::ParsedPath,
) -> Vec<Arc<Inspectable>> {
    let st = target.reflect(cx).unwrap();
    let ReflectRef::Struct(st) = st.reflect_ref() else {
        return Vec::new();
    };
    let num_fields = st.field_len();
    let mut items = Vec::with_capacity(num_fields);
    let registry = cx.world().resource::<AppTypeRegistry>().0.clone();

    // Filter out field names for fields with a value of `None`.
    for findex in 0..num_fields {
        let field = st.field_at(findex).unwrap();
        // let info = st.get_represented_type_info().unwrap()
        if field.reflect_kind() == ReflectKind::Enum
            && field
                .reflect_type_path()
                .starts_with("core::option::Option")
        {
            let ReflectRef::Enum(enum_ref) = field.reflect_ref() else {
                panic!("Expected ReflectRef::Enum");
            };
            if enum_ref.variant_name() == "None" {
                let name = st.name_at(findex).unwrap();
                let Some(TypeInfo::Enum(enum_info)) = field.get_represented_type_info() else {
                    panic!("Expected TypeInfo::Enum");
                };
                let some_variant = enum_info.variant("Some").unwrap();
                let VariantInfo::Tuple(tuple_info) = some_variant else {
                    panic!()
                };
                let some_field = tuple_info.field_at(0).unwrap();
                let some_type_id = some_field.type_id();
                let registry_lock = registry.read();
                let some_default = registry_lock.get_type_data::<ReflectDefault>(some_type_id);
                if some_default.is_some() {
                    let mut path = base_path.clone();
                    path.0.push(OffsetAccess {
                        access: bevy::reflect::Access::Field(name.to_string().into()),
                        offset: None,
                    });
                    items.push(Arc::new(Inspectable {
                        root: target.root.clone(),
                        name: name.to_string(),
                        value_path: path.clone(),
                        field_path: path,
                        can_remove: false,
                        attributes: None,
                    }));
                }
            }
        }
    }
    items
}

#[derive(Clone)]
struct AddStructFieldItem {
    field: Arc<Inspectable>,
}

impl PartialEq for AddStructFieldItem {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.field, &other.field)
    }
}

impl ViewTemplate for AddStructFieldItem {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let field = self.field.clone();
        let callback = cx.create_callback(move |world: &mut World| {
            let Some(field_reflect) = field.reflect_untracked(world) else {
                return;
            };
            let Some(TypeInfo::Enum(enum_info)) = field_reflect.get_represented_type_info() else {
                panic!("Expected TypeInfo::Enum");
            };

            // let field = target.get_field(cx, &path);
            let registry = world.resource::<AppTypeRegistry>().0.clone();
            let some_variant = enum_info.variant("Some").unwrap();
            let VariantInfo::Tuple(tuple_info) = some_variant else {
                panic!("Expected VariantInfo::Tuple");
            };
            let some_field = tuple_info.field_at(0).unwrap();
            let some_type_id = some_field.type_id();
            let registry_lock = registry.read();
            let some_type = registry_lock.get_type_info(some_type_id).unwrap();
            if some_type.is::<bool>() {
                // For Option<bool> we assume that the user wants a default of 'true', because
                // that's the most common use case. This is because for most fields, `Some(false)`
                // is the same as `None`.
                let mut data = DynamicTuple::default();
                data.insert_boxed(Box::new(true));
                let dynamic_enum = DynamicEnum::new("Some", data);
                field.set_value(world, &dynamic_enum);
            } else {
                let some_default = registry_lock.get_type_data::<ReflectDefault>(some_type_id);
                if some_default.is_some() {
                    // The value that needs to get wrapped in `Some`.
                    let default = some_default.unwrap().default();
                    let mut data = DynamicTuple::default();
                    data.insert_boxed(default);
                    let dynamic_enum = DynamicEnum::new("Some", data);
                    field.set_value(world, &dynamic_enum);
                } else {
                    println!("Can't find ReflectDefault for: {:?}", some_type.type_path());
                }
            }
        });
        MenuItem::new()
            .label(self.field.name.clone())
            .on_click(callback)
    }
}

fn style_menu_icon(ss: &mut StyleBuilder) {
    ss.margin((4, 0));
}
