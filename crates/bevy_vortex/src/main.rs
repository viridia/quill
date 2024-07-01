//! Example of a comprehensive UI layout
#![feature(impl_trait_in_assoc_type)]

mod catalog;
mod operator;
mod ops;
mod preview;

use bevy_mod_picking::{debug::DebugPickingMode, picking_core::Pickable, DefaultPickingPlugins};
use bevy_mod_stylebuilder::*;
use bevy_quill_obsidian_graph::{GraphDisplay, ObsidianGraphPlugin};
use catalog::{build_operator_catalog, CatalogView, OperatorCatalog, SelectedCatalogEntry};
use ops::OperatorsPlugin;
use preview::{
    enter_mode_cuboid, enter_mode_sphere, enter_mode_tetra, enter_mode_torus, enter_preview_3d,
    exit_mode_shape3d, exit_preview_3d, rotate_shapes, PreviewControls, PreviewMode, PreviewMode3d,
};
use quill_obsidian::{
    colors,
    controls::{Slider, Splitter, SplitterDirection},
    focus::TabGroup,
    typography, viewport, ObsidianUiPlugin,
};

use bevy::{asset::embedded_asset, prelude::*, ui};
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

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum EditorState {
    #[default]
    Preview,
    Graph,
    Split,
}

#[derive(Resource, Default)]
pub struct ClickLog(pub Vec<String>);

fn main() {
    App::new()
        .init_resource::<ClickLog>()
        .init_resource::<OperatorCatalog>()
        .insert_resource(SelectedCatalogEntry(None))
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
        .init_resource::<viewport::ViewportInset>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(DefaultPickingPlugins)
        .insert_state(EditorState::Preview)
        .insert_state(PreviewMode::Square)
        .add_computed_state::<PreviewMode3d>()
        .insert_resource(DebugPickingMode::Disabled)
        // .add_plugins(InspectorPlugin)
        .add_plugins((
            QuillPlugin,
            ObsidianUiPlugin,
            ObsidianGraphPlugin,
            VortexPlugin,
            OperatorsPlugin,
        ))
        .add_systems(Startup, setup_ui.pipe(setup_view_root))
        .add_systems(
            Update,
            (
                close_on_esc,
                build_operator_catalog,
                rotate_shapes,
                viewport::update_viewport_inset,
                viewport::update_camera_viewport,
            ),
        )
        .add_systems(OnEnter(PreviewMode3d), enter_preview_3d)
        .add_systems(OnExit(PreviewMode3d), exit_preview_3d)
        .add_systems(OnEnter(PreviewMode::Cuboid), enter_mode_cuboid)
        .add_systems(OnEnter(PreviewMode::Sphere), enter_mode_sphere)
        .add_systems(OnEnter(PreviewMode::Tetra), enter_mode_tetra)
        .add_systems(OnEnter(PreviewMode::Torus), enter_mode_torus)
        .add_systems(OnExit(PreviewMode::Sphere), exit_mode_shape3d)
        .add_systems(OnExit(PreviewMode::Cuboid), exit_mode_shape3d)
        .add_systems(OnExit(PreviewMode::Tetra), exit_mode_shape3d)
        .add_systems(OnExit(PreviewMode::Torus), exit_mode_shape3d)
        .run();
}

pub struct VortexPlugin;

impl Plugin for VortexPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "assets/icons/square.png");
        embedded_asset!(app, "assets/icons/square2x2.png");
        embedded_asset!(app, "assets/icons/square3x3.png");
        embedded_asset!(app, "assets/icons/cuboid.png");
        embedded_asset!(app, "assets/icons/sphere.png");
        embedded_asset!(app, "assets/icons/tetra.png");
        embedded_asset!(app, "assets/icons/torus.png");
    }
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

fn setup_view_root(camera: In<Entity>, mut commands: Commands) {
    commands.spawn(DemoUi(*camera).to_root());
}

#[derive(Clone, PartialEq)]
struct DemoUi(Entity);

impl ViewTemplate for DemoUi {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let red = cx.create_mutable::<f32>(128.);

        let panel_width = cx.use_resource::<PanelWidth>().0;
        let camera = self.0;

        Element::<NodeBundle>::new()
            .named("Main")
            .style((typography::text_default, style_main))
            .insert_dyn(move |_| (TabGroup::default(), TargetCamera(camera)), ())
            .children((
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
                        PreviewControls,
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
                            ),
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

// /// Creates a colorful test pattern
// fn uv_debug_texture() -> Image {
//     const TEXTURE_SIZE: usize = 8;

//     let mut palette: [u8; 32] = [
//         255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
//         198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
//     ];

//     let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
//     for y in 0..TEXTURE_SIZE {
//         let offset = TEXTURE_SIZE * y * 4;
//         texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
//         palette.rotate_right(4);
//     }

//     Image::new_fill(
//         Extent3d {
//             width: TEXTURE_SIZE as u32,
//             height: TEXTURE_SIZE as u32,
//             depth_or_array_layers: 1,
//         },
//         TextureDimension::D2,
//         &texture_data,
//         TextureFormat::Rgba8UnormSrgb,
//         RenderAssetUsages::default(),
//     )
// }

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
