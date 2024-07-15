#![feature(impl_trait_in_assoc_type)]

mod add_node;
mod catalog;
mod commands;
mod gen;
mod graph;
mod graph_view;
mod operator;
mod ops;
mod pipeline;
mod preview;
mod propedit;

use add_node::AddNodeButton;
use bevy_mod_picking::{
    debug::DebugPickingMode,
    picking_core::Pickable,
    prelude::{Listener, On},
    DefaultPickingPlugins,
};
use bevy_mod_stylebuilder::*;
use bevy_quill_obsidian::{
    colors,
    controls::{Splitter, SplitterDirection},
    focus::{DefaultKeyListener, KeyPressEvent, TabGroup},
    typography, viewport, ObsidianUiPlugin,
};
use bevy_quill_obsidian_graph::{
    ConnectionAnchor, ConnectionTarget, DragAction, Gesture, GraphEvent, ObsidianGraphPlugin,
};
use catalog::{build_operator_catalog, CatalogView, OperatorCatalog, SelectedCatalogEntry};
use commands::{AddConnectionCmd, DeleteSelectedCmd};
use gen::{begin_build_shaders, finish_build_shaders};
use graph::{
    sync_connection_refs, Connection, GraphNode, GraphResource, NodeBasePosition, NodeSelected,
    ValidateConnectionCmd,
};
use graph_view::{DragState, GraphView, GraphViewId};
use ops::OperatorsPlugin;
use pipeline::NodeShaderMeshPlugin;
use preview::{PreviewControls, PreviewPlugin};

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
        .add_plugins((DefaultPickingPlugins,))
        .insert_resource(DebugPickingMode::Disabled)
        .add_plugins((
            PreviewPlugin,
            QuillPlugin,
            ObsidianUiPlugin,
            ObsidianGraphPlugin,
            VortexPlugin,
            OperatorsPlugin,
            NodeShaderMeshPlugin,
        ))
        .add_systems(
            Startup,
            (sync_connection_refs, setup_ui.pipe(setup_view_root)),
        )
        .add_systems(
            Update,
            (
                close_on_esc,
                build_operator_catalog,
                viewport::update_viewport_inset,
                viewport::update_camera_viewport,
                begin_build_shaders,
                finish_build_shaders,
            ),
        )
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

        // Needed to ensure popup menus and dialogs render on the correct camera.
        cx.insert(TargetCamera(camera));

        // Insert the view id as a context variable.
        cx.insert(GraphViewId(graph_view_id));
        cx.insert(DragState::default());

        Element::<NodeBundle>::new()
            .named("Main")
            .style((typography::text_default, style_main))
            .insert_dyn(
                move |_| {
                    (
                        TabGroup::default(),
                        TargetCamera(camera),
                        DefaultKeyListener,
                        On::<KeyPressEvent>::run(
                            |event: Listener<KeyPressEvent>, mut commands: Commands| {
                                if event.key_code == KeyCode::Delete
                                    || event.key_code == KeyCode::Backspace
                                {
                                    commands.add(DeleteSelectedCmd);
                                }
                            },
                        ),
                    )
                },
                (),
            )
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
                         mut commands: Commands,
                         mut catalog_selection: ResMut<SelectedCatalogEntry>,
                         mut query_drag_state: Query<&mut DragState>,
                         mut query_graph_nodes: Query<(
                            Entity,
                            &mut GraphNode,
                            &mut NodeSelected,
                            Option<&NodeBasePosition>,
                        )>,
                         mut query_connections: Query<&mut Connection>| {
                            let mut drag_state = query_drag_state.single_mut();
                            match event.gesture {
                                // Move nodes by dragging.
                                Gesture::Move(position, action) => {
                                    let offset = position.as_ivec2();
                                    match action {
                                        DragAction::Start => {
                                            for (ent, node, selected, _base) in
                                                query_graph_nodes.iter_mut()
                                            {
                                                if selected.0 {
                                                    commands
                                                        .entity(ent)
                                                        .insert(NodeBasePosition(node.position));
                                                }
                                            }
                                        }

                                        DragAction::Update => {
                                            for (_, mut node, _, base) in
                                                query_graph_nodes.iter_mut()
                                            {
                                                if let Some(base) = base {
                                                    node.position = base.0 + offset;
                                                }
                                            }
                                        }

                                        DragAction::Finish => {
                                            for (ent, _, _, _) in query_graph_nodes.iter_mut() {
                                                commands.entity(ent).remove::<NodeBasePosition>();
                                            }
                                        }
                                    }
                                }

                                Gesture::Connect(anchor, target, action) => match action {
                                    DragAction::Start | DragAction::Update => {
                                        drag_state.connect_from = Some(anchor);
                                        drag_state.connect_to = Some(target);
                                        match (anchor, target) {
                                            (
                                                ConnectionAnchor::OutputTerminal(output),
                                                ConnectionTarget::InputTerminal(input),
                                            ) => {
                                                commands
                                                    .add(ValidateConnectionCmd { output, input });
                                            }

                                            (
                                                ConnectionAnchor::InputTerminal(output),
                                                ConnectionTarget::OutputTerminal(input),
                                            ) => {
                                                commands
                                                    .add(ValidateConnectionCmd { output, input });
                                            }

                                            (
                                                ConnectionAnchor::EdgeSink(edge),
                                                ConnectionTarget::OutputTerminal(output),
                                            ) => {
                                                let conn = query_connections.get_mut(edge).unwrap();
                                                let input = conn.input.terminal_id;
                                                commands
                                                    .add(ValidateConnectionCmd { output, input });
                                            }

                                            (
                                                ConnectionAnchor::EdgeSource(edge),
                                                ConnectionTarget::InputTerminal(input),
                                            ) => {
                                                let conn = query_connections.get_mut(edge).unwrap();
                                                let output = conn.output.terminal_id;
                                                commands
                                                    .add(ValidateConnectionCmd { output, input });
                                            }

                                            _ => {
                                                drag_state.valid_connection = false;
                                            }
                                        }
                                    }

                                    DragAction::Finish => {
                                        match (anchor, target) {
                                            (
                                                ConnectionAnchor::OutputTerminal(output),
                                                ConnectionTarget::InputTerminal(input),
                                            ) => {
                                                commands.add(AddConnectionCmd {
                                                    output,
                                                    input,
                                                    replace: None,
                                                });
                                            }

                                            (
                                                ConnectionAnchor::InputTerminal(output),
                                                ConnectionTarget::OutputTerminal(input),
                                            ) => {
                                                commands.add(AddConnectionCmd {
                                                    output,
                                                    input,
                                                    replace: None,
                                                });
                                            }

                                            (
                                                ConnectionAnchor::EdgeSource(edge),
                                                ConnectionTarget::OutputTerminal(output),
                                            ) => {
                                                let conn = query_connections.get_mut(edge).unwrap();
                                                let input = conn.input.terminal_id;
                                                commands.add(AddConnectionCmd {
                                                    output,
                                                    input,
                                                    replace: Some(edge),
                                                });
                                            }

                                            (
                                                ConnectionAnchor::EdgeSink(edge),
                                                ConnectionTarget::InputTerminal(input),
                                            ) => {
                                                let conn = query_connections.get_mut(edge).unwrap();
                                                let output = conn.output.terminal_id;
                                                commands.add(AddConnectionCmd {
                                                    output,
                                                    input,
                                                    replace: Some(edge),
                                                });
                                            }

                                            _ => {}
                                        }

                                        drag_state.connect_from = None;
                                        drag_state.connect_to = None;
                                        drag_state.valid_connection = false;
                                    }
                                },

                                // bevy_quill_obsidian_graph::Gesture::Scroll(_) => todo!(),
                                Gesture::SelectRect(rect, action) => {
                                    if action == DragAction::Finish {
                                        for (_, node, mut selected, _) in
                                            query_graph_nodes.iter_mut()
                                        {
                                            let node_rect = Rect::from_center_size(
                                                node.position.as_vec2(),
                                                node.size.as_vec2(),
                                            );
                                            if rect.contains(node_rect.min)
                                                && rect.contains(node_rect.max)
                                            {
                                                selected.0 = true;
                                            }
                                        }
                                        drag_state.selection_rect = None;
                                    } else {
                                        drag_state.selection_rect = Some(rect);
                                    }
                                }

                                Gesture::Select(node) => {
                                    catalog_selection.0 = None;
                                    let is_selected = query_graph_nodes
                                        .get_mut(node)
                                        .map_or(false, |(_, _, selected, _)| selected.0);
                                    if !is_selected {
                                        for (ent, _, mut selected, _) in
                                            query_graph_nodes.iter_mut()
                                        {
                                            let select = ent == node;
                                            if selected.0 != select {
                                                selected.0 = select;
                                            }
                                        }
                                    }
                                }

                                Gesture::SelectAdd(node) => {
                                    catalog_selection.0 = None;
                                    if let Ok((_, _, mut selected, _)) =
                                        query_graph_nodes.get_mut(node)
                                    {
                                        selected.0 = true;
                                    }
                                }

                                Gesture::SelectRemove(node) => {
                                    if let Ok((_, _, mut selected, _)) =
                                        query_graph_nodes.get_mut(node)
                                    {
                                        selected.0 = false;
                                    }
                                }

                                Gesture::SelectToggle(node) => {
                                    catalog_selection.0 = None;
                                    if let Ok((_, _, mut selected, _)) =
                                        query_graph_nodes.get_mut(node)
                                    {
                                        selected.0 = !selected.0;
                                    }
                                }

                                Gesture::SelectClear => {
                                    for (_, _, mut selected, _) in query_graph_nodes.iter_mut() {
                                        if selected.0 {
                                            selected.0 = false;
                                        }
                                    }
                                }

                                Gesture::Cancel => {
                                    drag_state.connect_from = None;
                                    drag_state.connect_to = None;
                                    drag_state.selection_rect = None;
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

pub fn handle_key_events() {}
