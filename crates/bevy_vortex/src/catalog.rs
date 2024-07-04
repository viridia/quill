use bevy::{
    prelude::*,
    reflect::TypeInfo,
    ui::{self, node_bundles::NodeBundle},
};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use bevy_quill_obsidian::{colors, controls::ListView, typography::text_strong};

use crate::{
    graph::{GraphNode, Selected},
    operator::{DisplayName, OperatorCategory, OperatorClass, ReflectOperator},
};

#[derive(Resource, Default)]
pub struct SelectedCatalogEntry(pub Option<&'static str>);

#[derive(Clone, PartialEq, Eq)]
pub struct CatalogEntry {
    category: OperatorCategory,
    display_name: &'static str,
    path: &'static str,
}

impl PartialOrd for CatalogEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CatalogEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.category
            .cmp(&other.category)
            .then_with(|| self.display_name.cmp(other.display_name))
            .then_with(|| self.path.cmp(other.path))
    }
}

#[derive(Resource, Default)]
pub struct OperatorCatalog(pub Vec<CatalogEntry>);

/// Displays the list of available operators, by category.
#[derive(Clone, PartialEq)]
pub struct CatalogView;

fn style_catalog(ss: &mut StyleBuilder) {
    ss.flex_grow(1.).min_height(100);
}

impl ViewTemplate for CatalogView {
    type View = impl View;

    fn create(&self, cx: &mut bevy_quill::Cx) -> Self::View {
        let catalog = cx.use_resource::<OperatorCatalog>();
        ListView::new()
            .style(style_catalog)
            .children(For::each(catalog.0.clone(), |entry| {
                CatalogRow(entry.clone())
            }))
    }
}

fn style_catalog_row(ss: &mut StyleBuilder) {
    ss.align_self(ui::AlignSelf::Stretch)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row);
}

fn style_catalog_operator_class(ss: &mut StyleBuilder) {
    ss.width(ui::Val::Percent(30.))
        .padding(2)
        .display(ui::Display::Flex)
        .justify_content(ui::JustifyContent::FlexEnd)
        .color(colors::DIM)
        .overflow(ui::OverflowAxis::Clip);
}

fn style_catalog_operator_name(ss: &mut StyleBuilder) {
    ss.flex_grow(1.)
        .padding(2)
        .color(colors::FOREGROUND)
        .overflow(ui::OverflowAxis::Clip);
}

#[derive(Clone, PartialEq)]
struct CatalogRow(CatalogEntry);

impl ViewTemplate for CatalogRow {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let selected = cx.use_resource::<SelectedCatalogEntry>();
        let is_selected = Some(self.0.path) == selected.0;
        let path = self.0.path;
        Element::<NodeBundle>::new()
            .style(style_catalog_row)
            .style_dyn(
                |selected, sb| {
                    sb.background_color(if selected {
                        colors::TEXT_SELECT
                    } else {
                        colors::TRANSPARENT
                    });
                },
                is_selected,
            )
            .insert_dyn(
                move |_| {
                    On::<Pointer<Click>>::run(
                        move |mut selected: ResMut<SelectedCatalogEntry>,
                        mut graph_nodes: Query<&mut Selected, With<GraphNode>>| {
                        // Clear node selection
                        for mut selected in graph_nodes.iter_mut() {
                            selected.0 = false;
                        }
                        selected.0 = Some(path);
                    })
                },
                (),
            )
            .children((
                Element::<NodeBundle>::new()
                    .style(style_catalog_operator_class)
                    .children(self.0.category.to_local_string()),
                Element::<NodeBundle>::new()
                    .style((text_strong, style_catalog_operator_name))
                    .children(self.0.display_name),
            ))
    }
}

pub fn build_operator_catalog(
    mut catalog: ResMut<OperatorCatalog>,
    registry: Res<AppTypeRegistry>,
) {
    if registry.is_changed() || catalog.0.is_empty() {
        let registry_lock = registry.read();
        let mut entries: Vec<CatalogEntry> = Vec::new();
        for rtype in registry_lock.iter() {
            if rtype.data::<ReflectOperator>().is_some() {
                let TypeInfo::Struct(st) = rtype.type_info() else {
                    panic!("Vortex operator must be a struct!")
                };
                let display_name = match st.get_attribute::<DisplayName>() {
                    Some(dname) => dname.0,
                    None => st.type_path_table().short_path(),
                };
                let category = match st.get_attribute::<OperatorClass>() {
                    Some(cls) => cls.0.clone(),
                    None => panic!("`OperatorClass` attribute is required on operators."),
                };
                entries.push(CatalogEntry {
                    category,
                    display_name,
                    path: st.type_path(),
                });
            }
        }
        entries.sort();
        catalog.0.clone_from(&entries);
    }
}
