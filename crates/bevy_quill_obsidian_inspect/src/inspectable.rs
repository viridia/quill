use core::panic;
use std::sync::Arc;

use bevy::{
    prelude::*,
    reflect::{
        attributes::CustomAttributes, DynamicEnum, DynamicVariant, ParsedPath, ReflectPathError,
    },
};
use bevy_quill::Cx;

/// Trait that represents an item that can be inspected
#[allow(unused_variables)]
pub trait InspectableRoot: Send + Sync {
    /// The name of the item being inspected
    fn name(&self, cx: &Cx) -> String;

    /// The reflect data for a path within the reflected item.
    fn reflect_path<'a>(&self, cx: &'a Cx, path: &ParsedPath) -> Option<&'a dyn Reflect>;

    /// The reflect data for a path within the reflected item.
    fn reflect_path_untracked<'a>(
        &self,
        world: &'a World,
        path: &ParsedPath,
    ) -> Option<&'a dyn Reflect>;

    /// Update a field within the item
    fn set_path(&self, world: &mut World, path: &ParsedPath, value: &dyn Reflect);

    /// Apply a closure to a field within the item
    fn update_path(&self, world: &mut World, path: &ParsedPath, f: &dyn Fn(&mut dyn Reflect));
}

/// A resource that can be inspected
pub struct InspectableResource<T: Resource + Reflect> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Resource + Reflect> Default for InspectableResource<T> {
    fn default() -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }
}

impl<T: Resource + Reflect> InspectableRoot for InspectableResource<T> {
    fn name(&self, cx: &Cx) -> String {
        let res = cx.use_resource::<T>();
        res.reflect_short_type_path().to_string()
    }

    fn reflect_path<'a>(&self, cx: &'a Cx, path: &ParsedPath) -> Option<&'a dyn Reflect> {
        let res = cx.use_resource::<T>();
        match res.reflect_path(path) {
            Ok(result) => Some(result),
            Err(ReflectPathError::InvalidAccess(_)) => None,
            Err(err) => panic!("{:?}", err),
        }
    }

    fn reflect_path_untracked<'a>(
        &self,
        world: &'a World,
        path: &ParsedPath,
    ) -> Option<&'a dyn Reflect> {
        let res = world.get_resource::<T>().unwrap();
        match res.reflect_path(path) {
            Ok(result) => Some(result),
            Err(ReflectPathError::InvalidAccess(_)) => None,
            Err(err) => panic!("{:?}", err),
        }
    }

    fn set_path(&self, world: &mut World, path: &ParsedPath, value: &dyn Reflect) {
        let mut res = world.get_resource_mut::<T>().unwrap();
        res.reflect_path_mut(path).unwrap().apply(value);
    }

    fn update_path(&self, world: &mut World, path: &ParsedPath, f: &dyn Fn(&mut dyn Reflect)) {
        let mut res = world.get_resource_mut::<T>().unwrap();
        f(res.reflect_path_mut(path).unwrap());
    }
}

/// An ECS component that can be inspected
pub struct InspectableComponent<T: Component + Reflect> {
    entity: Entity,
    marker: std::marker::PhantomData<T>,
}

impl<T: Component + Reflect> InspectableComponent<T> {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            marker: std::marker::PhantomData,
        }
    }
}

impl<T: Component + Reflect> InspectableRoot for InspectableComponent<T> {
    fn name(&self, cx: &Cx) -> String {
        let cmp = cx.use_component::<T>(self.entity).unwrap();
        cmp.reflect_short_type_path().to_string()
    }

    fn reflect_path<'a>(&self, cx: &'a Cx, path: &ParsedPath) -> Option<&'a dyn Reflect> {
        let cmp = cx.use_component::<T>(self.entity).unwrap();
        match cmp.reflect_path(path) {
            Ok(result) => Some(result),
            Err(ReflectPathError::InvalidAccess(_)) => None,
            Err(err) => panic!("{:?}", err),
        }
    }

    fn reflect_path_untracked<'a>(
        &self,
        world: &'a World,
        path: &ParsedPath,
    ) -> Option<&'a dyn Reflect> {
        let cmp = world.entity(self.entity).get::<T>().unwrap();
        match cmp.reflect_path(path) {
            Ok(result) => Some(result),
            Err(ReflectPathError::InvalidAccess(_)) => None,
            Err(err) => panic!("{:?}", err),
        }
    }

    fn set_path(&self, world: &mut World, path: &ParsedPath, value: &dyn Reflect) {
        let mut entt = world.entity_mut(self.entity);
        let mut cmp = entt.get_mut::<T>().unwrap();
        cmp.reflect_path_mut(path).unwrap().apply(value);
    }

    fn update_path(&self, world: &mut World, path: &ParsedPath, f: &dyn Fn(&mut dyn Reflect)) {
        let mut entt = world.entity_mut(self.entity);
        let mut res = entt.get_mut::<T>().unwrap();
        f(res.reflect_path_mut(path).unwrap());
    }
}

/// A reference to a field within an `Inspectable`. This contains information needed to
/// get and set the field as well as query it's type.
#[derive(Clone)]
pub struct Inspectable {
    /// The top-level data structure being inspected, which contains this field.
    pub(crate) root: Arc<dyn InspectableRoot>,
    /// Name of the field.
    pub(crate) name: String,
    /// The path to the struct field or tuple field containing the value. This is used to
    /// add or remove the field from the parent.
    pub(crate) field_path: ParsedPath,
    /// The path to the actual value, which might be wrapped in an `Option` or `Vec`. This is
    /// used to edit the field value.
    pub(crate) value_path: ParsedPath,
    /// If true, then the field can be removed from it's parent.
    pub(crate) can_remove: bool,
    /// Custom attributes for the field
    pub(crate) attributes: Option<&'static CustomAttributes>,
}

impl Inspectable {
    /// Return the name of this field.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the reflected value of the field.
    pub fn reflect<'a>(&self, cx: &'a Cx) -> Option<&'a dyn Reflect> {
        self.root.reflect_path(cx, &self.value_path)
    }

    /// Get the reflected value of the field, but does not track the value.
    pub fn reflect_untracked<'a>(&self, world: &'a World) -> Option<&'a dyn Reflect> {
        self.root.reflect_path_untracked(world, &self.value_path)
    }

    /// Update the value of the field
    pub fn set_value(&self, world: &mut World, value: &dyn Reflect) {
        self.root.set_path(world, &self.value_path, value);
    }

    /// Whether the item can be removed (in other words, is it optional or an array element)
    pub fn can_remove(&self) -> bool {
        self.can_remove
    }

    /// Use a closure to modify the reflected field data.
    pub fn update(&self, world: &mut World, f: &dyn Fn(&mut dyn Reflect)) {
        self.root.update_path(world, &self.value_path, f);
    }

    /// Remove the value from the parent
    pub fn remove(&self, world: &mut World) {
        let Some(field) = self.root.reflect_path_untracked(world, &self.field_path) else {
            return;
        };
        match field.get_represented_type_info().unwrap() {
            bevy::reflect::TypeInfo::Struct(_) => todo!(),
            bevy::reflect::TypeInfo::TupleStruct(_) => todo!(),
            bevy::reflect::TypeInfo::Tuple(_) => todo!(),
            bevy::reflect::TypeInfo::List(_) => todo!(),
            bevy::reflect::TypeInfo::Array(_) => todo!(),
            bevy::reflect::TypeInfo::Map(_) => todo!(),
            bevy::reflect::TypeInfo::Enum(_enum_ref) => {
                if field
                    .reflect_type_path()
                    .starts_with("core::option::Option")
                {
                    let dynamic_enum = DynamicEnum::new("None", DynamicVariant::Unit);
                    self.root.set_path(world, &self.field_path, &dynamic_enum);
                } else {
                    panic!("Can't remove non-optional field");
                }
            }
            bevy::reflect::TypeInfo::Value(_) => todo!(),
        }
    }
}

impl PartialEq for Inspectable {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.root, &other.root)
            && self.name == other.name
            && self.value_path == other.value_path
            && self.field_path == other.field_path
            && self.can_remove == other.can_remove
            && match (self.attributes, other.attributes) {
                (Some(a), Some(b)) => std::ptr::eq(a, b),
                (None, None) => true,
                _ => false,
            }
    }
}
