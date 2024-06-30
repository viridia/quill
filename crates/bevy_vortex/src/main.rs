//! Example of a comprehensive UI layout
#![feature(impl_trait_in_assoc_type)]

mod catalog;
mod operator;
mod ops;

use bevy_mod_picking::{
    backends::raycast::{RaycastBackendSettings, RaycastPickable},
    debug::DebugPickingMode,
    picking_core::Pickable,
    prelude::*,
    DefaultPickingPlugins,
};
use bevy_mod_stylebuilder::*;
use bevy_quill_obsidian_graph::{GraphDisplay, ObsidianGraphPlugin};
use catalog::CatalogView;
use ops::OperatorsPlugin;
use quill_obsidian::{
    colors,
    controls::{
        Button, ButtonVariant, Checkbox, Dialog, DialogBody, DialogFooter, DialogHeader, Slider,
        Splitter, SplitterDirection, ToolButton, ToolPalette,
    },
    focus::TabGroup,
    typography, viewport, ObsidianUiPlugin, RoundedCorners,
};

use std::f32::consts::PI;

use bevy::{
    color::palettes,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    ui,
};
use bevy_quill::*;

fn style_main(ss: &mut StyleBuilder) {
    ss.position(ui::PositionType::Absolute)
        .left(0)
        .top(0)
        .bottom(0)
        .right(0)
        .border(1)
        .border_color(colors::U2)
        .display(ui::Display::Flex)
        .pointer_events(false);
}

fn style_aside(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .background_color(colors::U2)
        .z_index(-1)
        .padding(8)
        .gap(8)
        .flex_direction(ui::FlexDirection::Column)
        .width(200)
        .pointer_events(true);
}

fn style_button_row(ss: &mut StyleBuilder) {
    ss.gap(8);
}

fn style_button_flex(ss: &mut StyleBuilder) {
    ss.flex_grow(1.);
}

fn style_slider(ss: &mut StyleBuilder) {
    ss.align_self(ui::AlignSelf::Stretch);
}

fn style_column_group(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .align_items(ui::AlignItems::FlexStart)
        .gap(8);
}

fn style_viewport(ss: &mut StyleBuilder) {
    ss.align_self(ui::AlignSelf::Stretch)
        .min_height(100)
        .min_width(100)
        .flex_grow(1.)
        .border(1)
        .border_color(Color::BLACK)
        .pointer_events(false)
        .aspect_ratio(1.);
}

#[derive(Resource)]
pub struct PanelWidth(f32);

#[derive(Resource)]
pub struct PanelHeight(f32);

#[derive(Resource, Default)]
pub struct SelectedShape(Option<Entity>);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum EditorState {
    #[default]
    Preview,
    Graph,
    Split,
}

#[derive(Resource)]
pub struct PreviewEntities {
    camera: Entity,
}

#[derive(Resource, Default)]
pub struct ClickLog(pub Vec<String>);

fn main() {
    App::new()
        .init_resource::<SelectedShape>()
        .init_resource::<TrackingScopeTracing>()
        .init_resource::<ClickLog>()
        // .init_resource::<DemoGraphRoot>()
        // .insert_resource(TestStruct {
        //     unlit: Some(true),
        //     ..default()
        // })
        // .insert_resource(TestStruct2 {
        //     nested: TestStruct::default(),
        //     ..default()
        // })
        // .insert_resource(TestStruct3(true))
        .insert_resource(PanelWidth(300.))
        .insert_resource(PanelHeight(300.))
        .init_resource::<viewport::ViewportInset>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(DefaultPickingPlugins)
        .insert_state(EditorState::Preview)
        .insert_resource(DebugPickingMode::Disabled)
        .insert_resource(RaycastBackendSettings {
            require_markers: true,
            ..default()
        })
        // .add_plugins(InspectorPlugin)
        .add_plugins((
            QuillPlugin,
            ObsidianUiPlugin,
            ObsidianGraphPlugin,
            OperatorsPlugin,
        ))
        .add_systems(Startup, (setup, setup_ui.pipe(setup_view_root)))
        .add_systems(
            Update,
            (
                close_on_esc,
                rotate.run_if(in_state(EditorState::Preview)),
                rotate.run_if(in_state(EditorState::Split)),
                viewport::update_viewport_inset.run_if(in_state(EditorState::Preview)),
                viewport::update_viewport_inset.run_if(in_state(EditorState::Split)),
                viewport::update_camera_viewport.run_if(in_state(EditorState::Preview)),
                viewport::update_camera_viewport.run_if(in_state(EditorState::Split)),
            ),
        )
        .add_systems(OnEnter(EditorState::Preview), enter_preview_mode)
        .add_systems(OnExit(EditorState::Preview), exit_preview_mode)
        .add_systems(OnEnter(EditorState::Split), enter_preview_mode)
        .add_systems(OnExit(EditorState::Split), exit_preview_mode)
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.5;

fn setup_view_root(camera: In<Entity>, mut commands: Commands) {
    commands.spawn(DemoUi(*camera).to_root());
}

#[derive(Clone, PartialEq)]
struct DemoUi(Entity);

impl ViewTemplate for DemoUi {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let dialog_open = cx.create_mutable(false);
        let checked_1 = cx.create_mutable(false);
        let checked_2 = cx.create_mutable(true);
        let red = cx.create_mutable::<f32>(128.);
        // let name = cx.create_mutable("filename.txt".to_string());

        let panel_width = cx.use_resource::<PanelWidth>().0;
        let camera = self.0;

        Element::<NodeBundle>::new()
            .named("Main")
            .style((typography::text_default, style_main))
            .insert_dyn(move |_| (TabGroup::default(), TargetCamera(camera)), ())
            .children((
                Dialog::new()
                    .width(ui::Val::Px(400.))
                    .open(dialog_open.get(cx))
                    .on_close(cx.create_callback(move |world: &mut World| {
                        dialog_open.set(world, false);
                    }))
                    .children((
                        DialogHeader::new().children("Dialog Header"),
                        DialogBody::new().children("Example dialog body text."),
                        DialogFooter::new().children((
                            Button::new()
                                .children("Cancel")
                                .on_click(cx.create_callback(move |world: &mut World| {
                                    dialog_open.set(world, false);
                                })),
                            Button::new()
                                .children("Close")
                                .variant(ButtonVariant::Primary)
                                .autofocus(true)
                                .on_click(cx.create_callback(move |world: &mut World| {
                                    dialog_open.set(world, false);
                                })),
                        )),
                    )),
                Element::<NodeBundle>::new()
                    .named("ControlPalette")
                    .style(style_aside)
                    .style_dyn(
                        move |width, sb| {
                            sb.width(ui::Val::Px(width));
                        },
                        panel_width,
                    )
                    .children((
                        CatalogView,
                        ToolPalette::new().columns(3).children((
                            ToolButton::new()
                                .children("Preview")
                                .corners(RoundedCorners::Left)
                                .variant({
                                    let st = cx.use_resource::<State<EditorState>>();
                                    if *st.get() == EditorState::Preview {
                                        ButtonVariant::Selected
                                    } else {
                                        ButtonVariant::Default
                                    }
                                })
                                .on_click(cx.create_callback(
                                    |mut mode: ResMut<NextState<EditorState>>| {
                                        mode.set(EditorState::Preview);
                                    },
                                )),
                            ToolButton::new()
                                .children("Materials")
                                .corners(RoundedCorners::None)
                                .variant({
                                    let st = cx.use_resource::<State<EditorState>>();
                                    if *st.get() == EditorState::Graph {
                                        ButtonVariant::Selected
                                    } else {
                                        ButtonVariant::Default
                                    }
                                })
                                .on_click(cx.create_callback(
                                    |mut mode: ResMut<NextState<EditorState>>| {
                                        mode.set(EditorState::Graph);
                                    },
                                )),
                            ToolButton::new()
                                .children("Split")
                                .corners(RoundedCorners::Right)
                                .variant({
                                    let st = cx.use_resource::<State<EditorState>>();
                                    if *st.get() == EditorState::Split {
                                        ButtonVariant::Selected
                                    } else {
                                        ButtonVariant::Default
                                    }
                                })
                                .on_click(cx.create_callback(
                                    |mut mode: ResMut<NextState<EditorState>>| {
                                        mode.set(EditorState::Split);
                                    },
                                )),
                        )),
                        Element::<NodeBundle>::new()
                            .style(style_button_row)
                            .children((
                                Button::new()
                                    .children("Open…")
                                    .on_click(cx.create_callback(move |world: &mut World| {
                                        let mut log = world.get_resource_mut::<ClickLog>().unwrap();
                                        log.0.push("Open clicked".to_string());
                                        dialog_open.set(world, true);
                                    }))
                                    .style(style_button_flex),
                                Button::new()
                                    .children("Save")
                                    .on_click(cx.create_callback(
                                        move |mut log: ResMut<ClickLog>| {
                                            log.0.push("Save clicked".to_string());
                                        },
                                    ))
                                    .style(style_button_flex),
                            )),
                        Element::<NodeBundle>::new()
                            .style(style_column_group)
                            .children((
                                Checkbox::new()
                                    // .style(|ss: &mut StyleBuilder| {
                                    //     ss.cursor_image("demo://unlock.png", Vec2::new(8., 8.));
                                    // })
                                    .label("Include Author Name")
                                    .checked(checked_1.get(cx))
                                    .on_change(cx.create_callback(
                                        move |checked: In<bool>, world: &mut World| {
                                            let mut log =
                                                world.get_resource_mut::<ClickLog>().unwrap();
                                            log.0
                                                .push(format!("Include Author Name: {}", *checked));
                                            checked_1.set(world, *checked);
                                        },
                                    )),
                                Checkbox::new()
                                    .label("Include Metadata")
                                    .checked(checked_2.get(cx))
                                    .on_change(cx.create_callback(
                                        move |checked: In<bool>, world: &mut World| {
                                            let mut log =
                                                world.get_resource_mut::<ClickLog>().unwrap();
                                            log.0.push(format!("Include Metadata: {}", *checked));
                                            checked_2.set(world, *checked);
                                        },
                                    )),
                            )),
                        Element::<NodeBundle>::new()
                            .style(style_column_group)
                            .children(
                                Slider::new()
                                    .min(0.)
                                    .max(255.)
                                    .value(red.get(cx))
                                    .style(style_slider)
                                    .precision(1)
                                    .on_change(cx.create_callback(
                                        move |value: In<f32>, world: &mut World| {
                                            let mut log =
                                                world.get_resource_mut::<ClickLog>().unwrap();
                                            log.0.push(format!("Slider value: {}", *value));
                                            red.set(world, *value);
                                        },
                                    )),
                                // )
                                // TextInput::new(TextInputProps {
                                //     value: name.signal(),
                                //     on_change: Some(cx.create_callback(
                                //         move |cx: &mut Cx, value: String| {
                                //             name.set_clone(cx, value.clone());
                                //         },
                                //     )),
                                //     ..default()
                                // }
                            ),
                        // ResourcePropertyInspector::<TestStruct>::new(),
                        // ResourcePropertyInspector::<TestStruct2>::new(),
                        // ResourcePropertyInspector::<TestStruct3>::new(),
                        // ReactionsTable,
                        Element::<NodeBundle>::new()
                            .named("Preview")
                            .style(style_viewport)
                            .style_dyn(
                                move |width, sb: &mut StyleBuilder| {
                                    sb.width(width - 16.).max_width(width - 16.);
                                },
                                panel_width,
                            )
                            .insert((viewport::ViewportInsetElement, Pickable::IGNORE)),
                    )),
                Splitter::new()
                    .direction(SplitterDirection::Vertical)
                    .value(panel_width)
                    .on_change(cx.create_callback(|value: In<f32>, world: &mut World| {
                        let mut panel_width = world.get_resource_mut::<PanelWidth>().unwrap();
                        panel_width.0 = value.max(200.);
                    })),
                CenterPanel,
            ))
    }
}

fn wrapper_style(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_grow(1.)
        .align_self(ui::AlignSelf::Stretch)
        .flex_direction(FlexDirection::Column);
}

#[derive(Clone, PartialEq)]
struct CenterPanel;

impl ViewTemplate for CenterPanel {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        Element::<NodeBundle>::new()
            .children(NodeGraphDemo {})
            .style(wrapper_style)
    }
}

fn style_node_graph(ss: &mut StyleBuilder) {
    ss.flex_grow(1.).border_left(1).border_color(Color::BLACK);
}

#[derive(Clone, PartialEq)]
struct NodeGraphDemo;

impl ViewTemplate for NodeGraphDemo {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        GraphDisplay::new().style(style_node_graph)
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

    let shapes_parent = commands
        .spawn((
            SpatialBundle { ..default() },
            // BackdropPickable,
            On::<Pointer<Down>>::run(
                |mut event: ListenerMut<Pointer<Down>>,
                 shapes: Query<&Shape>,
                 mut selection: ResMut<SelectedShape>| {
                    if shapes.get(event.target).is_ok() {
                        selection.0 = Some(event.target);
                        // println!("Pointer down on shape {:?}", event.target);
                    } else {
                        selection.0 = None;
                        // println!("Pointer down on backdrop {:?}", event.target);
                    }
                    event.stop_propagation();
                },
            ),
        ))
        .id();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands
            .spawn((
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
                // PickableBundle::default(),
                RaycastPickable,
            ))
            .set_parent(shapes_parent);
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
}

fn setup_ui(mut commands: Commands) -> Entity {
    commands
        .spawn((Camera2dBundle {
            camera: Camera {
                order: -1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            camera_2d: Camera2d {},
            ..default()
        },))
        .id()
}

fn enter_preview_mode(mut commands: Commands) {
    let camera = commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 6., 12.0)
                    .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
                ..default()
            },
            viewport::ViewportCamera,
            RaycastPickable,
            // BackdropPickable,
        ))
        .id();

    // let overlay = commands.spawn(TransformOverlayDemo.to_root()).id();
    commands.insert_resource(PreviewEntities { camera });
}

fn exit_preview_mode(mut commands: Commands, preview: Res<PreviewEntities>) {
    commands.entity(preview.camera).despawn();
    // commands.add(DespawnViewRoot::new(preview.overlay));
    commands.remove_resource::<PreviewEntities>()
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
