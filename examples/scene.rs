//! Example of a simple UI layout
#![feature(impl_trait_in_assoc_type)]

#[path = "./common/lib.rs"]
mod common;

use bevy::{math::primitives, prelude::*};

use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::{StyleBuilder, StyleBuilderFont, StyleBuilderLayout};
use bevy_quill_core::{Cond, Cx, Element, For, QuillPlugin, View, ViewTemplate};
use common::*;

fn main() {
    App::new()
        .init_resource::<Counter>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            DefaultPickingPlugins,
            QuillPlugin,
        ))
        // .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))
        .add_systems(Startup, (setup, setup_view_root))
        .add_systems(Update, (close_on_esc, rotate, update_counter))
        .run();
}

fn setup_view_root(mut commands: Commands) {
    commands.spawn(SceneRoot.to_root());
    commands.spawn(InstructionRoot.to_root());
}

// Counter containing global state.
#[derive(Resource, Default, PartialEq)]
pub struct Counter {
    pub count: usize,
}

fn update_counter(mut counter: ResMut<Counter>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::Space) {
        counter.count += 1;
    }
}

#[derive(Clone, PartialEq)]
struct SceneRoot;

impl ViewTemplate for SceneRoot {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let counter = cx.use_resource::<Counter>();
        let count = counter.count;

        const STEP_WIDTH: f32 = 1.5;
        let x_offset = (STEP_WIDTH * count as f32) / 2.;

        Element::<SpatialBundle>::new().children((
            // Always show meshcube widget
            MeshCube {
                translation: Vec3::new(-1.5, 3., 2.),
            },
            // Conditionally show MeshCube widget.
            Cond::new(
                count % 2 == 0,
                MeshCube {
                    translation: Vec3::new(1.5, 3., 2.),
                },
                (),
            ),
            // Keeps children centered
            Element::<SpatialBundle>::new()
                .insert_dyn(
                    move |x_offset| Transform {
                        translation: Vec3::new(-x_offset, 0., 2.),
                        ..Default::default()
                    },
                    x_offset,
                )
                .children(
                    // Loop over data, (in this case a simple range) and show
                    // a MeshCube for each element.
                    For::each(0..count, move |i| MeshCube {
                        translation: Vec3::new(STEP_WIDTH * *i as f32, 1., 0.),
                    }),
                ),
        ))
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MeshCube {
    translation: Vec3,
}

impl ViewTemplate for MeshCube {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let is_hovered = cx.create_mutable(false);

        // Note: It's better to re-use the same material by sharing the handle in a resource or
        // through props.
        let (mesh_handle, material_handle) = cx.create_memo(
            move |world, _| {
                let mut meshes = world.resource_mut::<Assets<Mesh>>();
                let mesh_handle = meshes.add(primitives::Cuboid::new(1., 1., 1.));
                let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
                let material_handle = materials.add(StandardMaterial::from_color(
                    Color::linear_rgba(1., 0., 1., 1.),
                ));
                (mesh_handle, material_handle)
            },
            (),
        );

        let counter = cx.use_resource::<Counter>();
        let count = counter.count;

        Element::<SpatialBundle>::new()
            .insert((mesh_handle, material_handle))
            .named(format!("MeshCube {}", count).as_str())
            .insert_dyn(
                move |(is_hovered, translation)| {
                    let scale = if is_hovered {
                        Vec3::new(1.2, 1.2, 1.2)
                    } else {
                        Vec3::new(1., 1., 1.)
                    };
                    Transform {
                        translation,
                        scale,
                        ..Default::default()
                    }
                },
                (is_hovered.get(cx), self.translation),
            )
            .insert(Pickable::default())
            .insert(PickingInteraction::default())
            .insert_dyn(
                move |_| {
                    (
                        On::<Pointer<Over>>::run(move |world: &mut World| {
                            is_hovered.set(world, true);
                        }),
                        On::<Pointer<Out>>::run(move |world: &mut World| {
                            is_hovered.set(world, false);
                        }),
                    )
                },
                (),
            )
    }
}

#[derive(Clone, PartialEq)]
struct InstructionRoot;

impl InstructionRoot {
    fn style(ss: &mut StyleBuilder) {
        ss.display(Display::Flex)
            .flex_direction(FlexDirection::Column)
            .border(3)
            .padding(3)
            .font_size(16.);
    }
}

impl ViewTemplate for InstructionRoot {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let counter = cx.use_resource::<Counter>();
        let count = counter.count;

        Element::<NodeBundle>::new().style(Self::style).children((
            "Scene (non-ui) example\n",
            format!("This UI reacts to the `Counter` resource which has a count of '{count}'.\n"),
            "When then count is even the cube in the top right is visible.\n",
            "We also loop over the range of `0..count` to spawn `count` number of cubes.",
        ))
    }
}
