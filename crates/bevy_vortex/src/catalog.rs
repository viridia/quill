use bevy::{prelude::AppTypeRegistry, reflect::TypeInfo};
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use quill_obsidian::controls::ListView;

use crate::operator::{DisplayName, OperatorCategory, OperatorClass, ReflectOperator};

#[derive(Clone, PartialEq)]
struct CatalogEntry {
    category: OperatorCategory,
    display_name: &'static str,
    path: &'static str,
}

impl PartialOrd for CatalogEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let category_cmp = self.category.partial_cmp(&other.category);
        if category_cmp.is_some() && category_cmp != Some(std::cmp::Ordering::Equal) {
            return category_cmp;
        }

        let display_name_cmp = self.display_name.partial_cmp(other.display_name);
        if display_name_cmp.is_some() && display_name_cmp != Some(std::cmp::Ordering::Equal) {
            return display_name_cmp;
        }

        self.path.partial_cmp(other.path)
    }
}

#[derive(Clone, PartialEq)]
pub struct CatalogView;

fn style_catalog(ss: &mut StyleBuilder) {
    ss.flex_grow(1.);
}

impl ViewTemplate for CatalogView {
    type View = impl View;

    fn create(&self, cx: &mut bevy_quill::Cx) -> Self::View {
        let registry = cx.use_resource::<AppTypeRegistry>();
        let registry_lock = registry.0.read();
        let mut entries: Vec<CatalogEntry> = Vec::new();
        for rtype in registry_lock.iter() {
            if let Some(oper) = rtype.data::<ReflectOperator>() {
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
                // println!("Found {}", );
            }
        }
        drop(registry_lock);
        ListView::new()
            .style(style_catalog)
            .children(For::each(entries, |entry| entry.display_name))
    }
}
