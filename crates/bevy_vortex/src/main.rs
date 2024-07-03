#![feature(impl_trait_in_assoc_type)]

mod add_node;
mod catalog;
mod graph;
mod graph_view;
mod operator;
mod ops;
mod preview;

use add_node::AddNodeButton;
use bevy_mod_picking::{
    debug::DebugPickingMode,
    picking_core::Pickable,
    prelude::{Listener, On},
    DefaultPickingPlugins,
};
use bevy_mod_stylebuilder::*;
use bevy_quill_obsidian_graph::{Gesture, GestureAction, GraphEvent, ObsidianGraphPlugin};
use catalog::{build_operator_catalog, CatalogView, OperatorCatalog, SelectedCatalogEntry};
use graph::{GraphNode, GraphResource, Selected};
use graph_view::{DragState, GraphView, GraphViewId};
use ops::OperatorsPlugin;
use preview::{
    enter_mode_cuboid, enter_mode_sphere, enter_mode_tetra, enter_mode_torus, enter_preview_3d,
    exit_mode_shape3d, exit_preview_3d, rotate_shapes, PreviewControls, PreviewMode, PreviewMode3d,
};
use quill_obsidian::{
    colors,
    controls::{Splitter, SplitterDirection},
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

fn main() {
    App::new()
        .init_resource::<OperatorCatalog>()
        .init_resource::<GraphResource>()
        .init_resource::<SelectedCatalogEntry>()
        .insert_resource(PanelWidth(300.))
        .init_resource::<viewport::ViewportInset>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(DefaultPickingPlugins)
        .insert_state(PreviewMode::Cuboid)
        .add_computed_state::<PreviewMode3d>()
        .insert_resource(DebugPickingMode::Disabled)
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

fn setup_view_root(camera: In<Entity>, mut commands: Commands) {
    commands.spawn(VortexUi(*camera).to_root());
}

#[derive(Clone, PartialEq)]
struct VortexUi(Entity);

impl ViewTemplate for VortexUi {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let graph_view_id = cx.create_entity();
        let panel_width = cx.use_resource::<PanelWidth>().0;
        let camera = self.0;

        // Insert the view id as a context variable.
        cx.insert(GraphViewId(graph_view_id));
        cx.insert(DragState::default());

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
                        AddNodeButton,
                        CatalogView,
                        PreviewControls,
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
                        panel_width.0 = value.clamp(200., 800.);
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
            .insert_dyn(
                |_| {
                    On::<GraphEvent>::run(
                        |event: Listener<GraphEvent>,
                         mut catalog_selection: ResMut<SelectedCatalogEntry>,
                         mut query_drag_state: Query<&mut DragState>,
                         mut query_graph_nodes: Query<(
                            Entity,
                            &mut GraphNode,
                            &mut Selected,
                        )>| {
                            match event.gesture {
                                // Move nodes by dragging.
                                Gesture::Move(position) => {
                                    let offset = position.as_ivec2();
                                    match event.action {
                                        GestureAction::Start | GestureAction::Move => {
                                            query_drag_state.single_mut().offset = offset;
                                        }
                                        GestureAction::End => {
                                            for (_, mut node, selected) in
                                                query_graph_nodes.iter_mut()
                                            {
                                                if selected.0 {
                                                    node.position += offset;
                                                }
                                            }
                                            query_drag_state.single_mut().offset = IVec2::default();
                                        }
                                        GestureAction::Cancel => {
                                            query_drag_state.single_mut().offset = IVec2::default();
                                        }
                                    }
                                }

                                // bevy_quill_obsidian_graph::Gesture::Create(_) => todo!(),
                                // bevy_quill_obsidian_graph::Gesture::ReconnectStart { edge, to } => todo!(),
                                // bevy_quill_obsidian_graph::Gesture::ReconnectEnd { edge, to } => todo!(),
                                // bevy_quill_obsidian_graph::Gesture::ConnectInput { terminal, to } => todo!(),
                                // bevy_quill_obsidian_graph::Gesture::ConnectEnd { terminal, to } => todo!(),
                                // bevy_quill_obsidian_graph::Gesture::ConnectCancel => todo!(),
                                // bevy_quill_obsidian_graph::Gesture::Scroll(_) => todo!(),
                                // bevy_quill_obsidian_graph::Gesture::SelectRect(_) => todo!(),
                                Gesture::SelectAdd(node) => {
                                    catalog_selection.0 = None;
                                    if let Ok((_, _, mut selected)) = query_graph_nodes.get_mut(node) {
                                        selected.0 = true;
                                    }
                                }
                                Gesture::SelectRemove(node) => {
                                    if let Ok((_, _, mut selected)) = query_graph_nodes.get_mut(node) {
                                        selected.0 = false;
                                    }
                                }
                                Gesture::SelectToggle(node) => {
                                    catalog_selection.0 = None;
                                    if let Ok((_, _, mut selected)) = query_graph_nodes.get_mut(node) {
                                        selected.0 = !selected.0;
                                    }
                                }
                                Gesture::SelectClear => {
                                    for (_, _, mut selected) in query_graph_nodes.iter_mut() {
                                        if selected.0 {
                                            selected.0 = false;
                                        }
                                    }
                                }
                                _ => {
                                    println!("Graph event received: {:?}", event.gesture)
                                }
                            }
                        },
                    )
                },
                (),
            )
            .children(GraphView)
            .style(wrapper_style)
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

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
