use std::sync::Arc;

use bevy::{
    prelude::*,
    reflect::{OffsetAccess, ReflectMut, ReflectRef, TypeInfo},
    ui,
};
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use quill_obsidian::{
    colors,
    controls::{DisclosureToggle, IconButton},
    size::Size,
};

use crate::{templates::field_label::FieldLabelWide, Inspectable, InspectorFactoryRegistry};

#[derive(Clone)]
pub struct ListInspector(pub(crate) Arc<Inspectable>);

impl PartialEq for ListInspector {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl ViewTemplate for ListInspector {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let field = self.0.clone();
        let expanded = cx.create_mutable(false);
        let length = if let Some(value) = field.reflect(cx) {
            if let ReflectRef::List(list) = value.reflect_ref() {
                list.len()
            } else {
                0
            }
        } else {
            0
        };

        let field = self.0.clone();
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
                    format!("{} ({})", field.name.clone(), length),
                ))
                .buttons(ListInspectorHeaderControls {
                    field: self.0.clone(),
                    length,
                    expanded,
                }),
            Cond::new(
                expanded.get(cx),
                ListElementsInspector {
                    field: self.0.clone(),
                    length,
                },
                (),
            ),
        )
    }
}

#[derive(Clone)]
struct ListInspectorHeaderControls {
    field: Arc<Inspectable>,
    length: usize,
    expanded: Mutable<bool>,
}

impl PartialEq for ListInspectorHeaderControls {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.field, &other.field)
            && self.length == other.length
            && self.expanded == other.expanded
    }
}

impl ViewTemplate for ListInspectorHeaderControls {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let length = self.length;
        let expanded = self.expanded;

        let pop_disabled = length == 0;

        let field = self.field.clone();
        let push = cx.create_callback(move |world: &mut World| {
            if let Some(list) = field.reflect_untracked(world) {
                if let TypeInfo::List(list_type) = list.get_represented_type_info().unwrap() {
                    let registry = world.resource::<AppTypeRegistry>().0.clone();
                    let registry_lock = registry.read();
                    let item_type =
                        registry_lock.get_type_data::<ReflectDefault>(list_type.item_type_id());
                    let default = item_type.unwrap().default();
                    field.update(world, &|reflect| {
                        if let ReflectMut::List(list) = reflect.reflect_mut() {
                            list.push(default.clone_value());
                        }
                    });
                    // Auto expand when pushing.
                    expanded.set(world, true);
                } else {
                    unreachable!("Expected List type ");
                }
            } else {
                unreachable!("Cannot push to non-list");
            }
        });

        let field = self.field.clone();
        let pop = cx.create_callback(move |world: &mut World| {
            field.update(world, &|reflect| {
                if let ReflectMut::List(list) = reflect.reflect_mut() {
                    if !list.is_empty() {
                        list.pop();
                    }
                } else {
                    unreachable!("Cannot pop from non-list")
                }
            })
        });

        (
            IconButton::new("obsidian_ui://icons/remove.png")
                .size(Size::Xs)
                .disabled(pop_disabled)
                .minimal(true)
                .on_click(pop),
            IconButton::new("obsidian_ui://icons/add.png")
                .size(Size::Xs)
                .minimal(true)
                .on_click(push),
        )
    }
}

#[derive(Clone)]
struct ListElementsInspector {
    field: Arc<Inspectable>,
    length: usize,
}

impl PartialEq for ListElementsInspector {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.field, &other.field) && self.length == other.length
    }
}

impl ViewTemplate for ListElementsInspector {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        let field = self.field.clone();
        let length = self.length;
        Element::<NodeBundle>::new()
            .style(style_list_items)
            .children(
                For::each(0..length, move |index| {
                    let mut path = field.value_path.clone();
                    path.0.push(OffsetAccess {
                        access: bevy::reflect::Access::ListIndex(*index),
                        offset: None,
                    });
                    let access = Arc::new(Inspectable {
                        root: field.root.clone(),
                        name: format!("{}", index),
                        value_path: path,
                        field_path: field.value_path.clone(),
                        can_remove: false,
                        attributes: field.attributes,
                    });
                    ListItemInspector { field: access }
                })
                .with_fallback(
                    Element::<NodeBundle>::new()
                        .style(style_empty_list)
                        .children("(empty list)"),
                ),
            )
    }
}

#[derive(Clone)]
struct ListItemInspector {
    field: Arc<Inspectable>,
}

impl PartialEq for ListItemInspector {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.field, &other.field)
    }
}

impl ViewTemplate for ListItemInspector {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let factories = cx.use_resource::<InspectorFactoryRegistry>();
        // Either create an inspector for the field, or return an empty view.
        Dynamic::new(
            factories
                .create_inspector(cx, self.field.clone())
                .into_view_child(),
        )
    }
}

fn style_list_items(ss: &mut StyleBuilder) {
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
        .margin_left(16);
}

fn style_empty_list(ss: &mut StyleBuilder) {
    ss.color(colors::DIM);
}
