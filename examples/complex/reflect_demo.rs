use std::sync::Arc;

use bevy::prelude::*;
use bevy_quill::*;
use bevy_quill_obsidian_inspect::{InspectableResource, Inspector, Precision, ValueRange};

#[derive(Debug, Reflect, Clone, Default)]
pub enum TestEnum {
    #[default]
    Unit,
    Float(f32),
    Color(Srgba),
    Struct {
        position: Vec3,
        color: Srgba,
    },
}

#[derive(Resource, Debug, Reflect, Clone, Default)]
pub struct TestStruct {
    pub selected: bool,

    #[reflect(@ValueRange::<f32>(0.0..1.0))]
    pub scale: f32,

    pub color: Srgba,
    pub position: Vec3,
    pub unlit: Option<bool>,

    #[reflect(@ValueRange::<f32>(0.0..10.0))]
    pub roughness: Option<f32>,

    #[reflect(@Precision(2))]
    pub metalness: Option<f32>,

    #[reflect(@ValueRange::<f32>(0.0..1000.0))]
    pub factors: Vec<f32>,
}

#[derive(Resource, Debug, Reflect, Clone, Default)]
pub struct TestStruct2 {
    pub nested: TestStruct,
    pub choice: TestEnum,
}

#[derive(Resource, Debug, Reflect, Clone, Default)]
pub struct TestStruct3(pub bool);

#[derive(Clone)]
pub struct ResourcePropertyInspector<T: Resource> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Resource> PartialEq for ResourcePropertyInspector<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T: Resource> ResourcePropertyInspector<T> {
    pub fn new() -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }
}

impl<T: Resource + Reflect> ViewTemplate for ResourcePropertyInspector<T> {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        Inspector::new(Arc::<InspectableResource<T>>::default())
    }
}
