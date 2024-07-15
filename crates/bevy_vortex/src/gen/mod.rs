mod expr;
mod output_chunk;
mod shader_assembly;

use crate::graph::{GraphNode, NodeModified};
use bevy::render::extract_component::ExtractComponent;
use bevy::tasks::futures_lite::future;
use bevy::{
    prelude::*,
    tasks::{block_on, AsyncComputeTaskPool, Task},
};
pub use expr::*;
pub use output_chunk::*;
pub use shader_assembly::ShaderAssembly;

/// Component used to indicate that a node is being observed. These nodes have higher priority
/// when it comes to rebuilding shaders.
#[derive(Component, Debug, Clone, Copy)]
pub(crate) struct NodeObserved(Entity);

/// Marker component that indicates that a graph node's shader is being rebuilt.
#[derive(Component)]
pub struct RebuildTask(Task<BuildShaderResult>);

pub struct BuildShaderResult(ShaderAssembly);

#[derive(Component)]
pub struct NodeOutput {
    /// Shader handle
    pub shader: Handle<Shader>,
}

#[derive(Clone, Component, ExtractComponent)]
struct NodeRenderedEntity;

pub(crate) fn finish_build_shaders(
    mut commands: Commands,
    q_output: Query<&NodeOutput>,
    mut q_rebuilding: Query<(Entity, &mut RebuildTask)>,
    mut shaders: ResMut<Assets<Shader>>,
) {
    // Collect results of shader rebuilds.
    for (node_id, mut rebuilding) in q_rebuilding.iter_mut() {
        let status = block_on(future::poll_once(&mut rebuilding.0));
        if let Some(result) = status {
            // if let Ok((_node, mut output)) = q_nodes.get_mut(node_id) {
            let mut entt = commands.entity(node_id);
            entt.remove::<RebuildTask>();
            let assembly = result.0;
            let source = assembly.gen_source().unwrap();
            // println!("Task complete:\n{}", assembly.gen_source().unwrap());
            let shader = Shader::from_wgsl(source, "".to_string());
            if let Ok(output) = q_output.get(node_id) {
                // Update shader asset in-place.
                shaders.insert(output.shader.id(), shader);
            } else {
                // Create shader asset and attach to node.
                commands.entity(node_id).insert(NodeOutput {
                    shader: shaders.add(shader),
                });
            }
        }
    }
}

pub(crate) fn begin_build_shaders(
    mut commands: Commands,
    q_nodes: Query<&GraphNode>,
    q_modified: Query<Entity, With<NodeModified>>,
) {
    // Spawn tasks for any nodes that are modified.
    // TODO: Limit
    let task_pool = AsyncComputeTaskPool::get();
    for modified in q_modified.iter() {
        if let Ok(node) = q_nodes.get(modified) {
            let mut entt = commands.entity(modified);
            entt.remove::<NodeModified>();
            // Need to walk the graph and build expression tree here.
            // Not sure that we need an async task since a lot of the effort is just querying
            // the graph, which is not accessible in a thread.
            if let Some(output) = node.outputs.first() {
                println!("Found first output: {:?}", output);
            }

            let task = task_pool.spawn(async move {
                // println!("Task spawned");
                // TODO: do whatever you want here!
                // generate_map_chunk(chunk_coord)
                let assembly = ShaderAssembly::new(modified);
                BuildShaderResult(assembly)
            });
            entt.insert(RebuildTask(task));
        }
    }
}
