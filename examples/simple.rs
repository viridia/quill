//! Example of a simple UI layout

use std::f32::consts::PI;

use bevy::{
    color::palettes,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use bevy_mod_stylebuilder::*;
use quill::{Cond, Cx, Element, QuillPlugin, View, ViewFactory};

fn main() {
    App::new()
        .init_resource::<Counter>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            QuillPlugin,
        ))
        // .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))
        // .add_plugins(ReactorPlugin)
        .add_systems(Startup, (setup, setup_view_root))
        .add_systems(Update, (close_on_esc, rotate, update_counter))
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.5;

fn style_test(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Row)
        .border_color(palettes::css::LIME)
        .border(3)
        .padding(3);
}

fn setup_view_root(mut commands: Commands) {
    commands.spawn(
        Element::<NodeBundle>::new()
            .style(style_test)
            .children(("Hello, ", "world!", NestedView, EvenOdd))
            .to_root(),
    );
    //     commands.spawn(ViewRoot::new(
    //         Element::<NodeBundle>::new()
    //             .style(style_test)
    //             .insert(BorderColor(palettes::css::LIME.into()))
    //             .insert_computed(|cx| {
    //                 let counter = cx.use_resource::<Counter>();
    //                 BackgroundColor(if counter.count & 1 == 0 {
    //                     palettes::css::DARK_GRAY.into()
    //                 } else {
    //                     palettes::css::MAROON.into()
    //                 })
    //             })
    //             .create_effect(|cx, ent| {
    //                 let count = cx.use_resource::<Counter>().count;
    //                 let mut border = cx.world_mut().get_mut::<BorderColor>(ent).unwrap();
    //                 border.0 = if count & 1 == 0 {
    //                     palettes::css::LIME.into()
    //                 } else {
    //                     palettes::css::RED.into()
    //                 };
    //             })
    //             .children((
    //                 Element::<NodeBundle>::new(),
    //                 text("Count: "),
    //                 text_computed(|cx| {
    //                     let counter = cx.use_resource::<Counter>();
    //                     format!("{}", counter.count)
    //                 }),
    //                 ", ",
    //                 NestedView,
    //                 ": ",
    //                 Cond::new(
    //                     |cx: &Rcx| {
    //                         let counter = cx.use_resource::<Counter>();
    //                         counter.count & 1 == 0
    //                     },
    //                     || "[Even]",
    //                     || "[Odd]",
    //                 ),
    //                 DynamicKeyed::new(
    //                     |cx| cx.use_resource::<Counter>().count,
    //                     |count| format!(":{}:", count),
    //                 ),
    //                 For::each(
    //                     |cx| {
    //                         let counter = cx.use_resource::<Counter>();
    //                         [counter.count, counter.count + 1, counter.count + 2].into_iter()
    //                     },
    //                     |item| format!("item: {}", item),
    //                 ),
    //             )),
    //     ));
}

struct NestedView;

impl ViewFactory for NestedView {
    type View = String;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let counter = cx.use_resource::<Counter>();
        format!("{}", counter.count)
    }
}

struct EvenOdd;

impl ViewFactory for EvenOdd {
    type View = Cond<&'static str, &'static str>;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let counter = cx.use_resource::<Counter>();
        Cond::new(counter.count & 1 == 0, "[Even]", "[Odd]")
    }
}

#[derive(Resource, Default)]
pub struct Counter {
    pub count: u32,
    pub foo: usize,
}

fn update_counter(mut counter: ResMut<Counter>, key: Res<ButtonInput<KeyCode>>) {
    if key.pressed(KeyCode::Space) {
        counter.count += 1;
    }
}

// Setup 3d shapes
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    2.0,
                    0.0,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            // intensity: 9000.0,
            intensity: 10000000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::from(palettes::css::SILVER)),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
