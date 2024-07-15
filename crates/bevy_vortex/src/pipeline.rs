use bevy::{
    core_pipeline::core_3d::{Opaque3d, Opaque3dBinKey, CORE_3D_DEPTH_FORMAT},
    pbr::{
        DrawMesh, MeshPipeline, MeshPipelineKey, MeshPipelineViewLayoutKey, RenderMeshInstances,
        SetMeshBindGroup, SetMeshViewBindGroup,
    },
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        mesh::GpuMesh,
        render_asset::RenderAssets,
        render_phase::{
            AddRenderCommand, BinnedRenderPhaseType, DrawFunctions, SetItemPipeline,
            ViewBinnedRenderPhases,
        },
        render_resource::{
            BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState,
            DepthStencilState, Face, FragmentState, FrontFace, MultisampleState, PipelineCache,
            PolygonMode, PrimitiveState, RenderPipelineDescriptor, SpecializedRenderPipeline,
            SpecializedRenderPipelines, StencilState, TextureFormat, VertexBufferLayout,
            VertexFormat, VertexState, VertexStepMode,
        },
        texture::BevyDefault,
        view::{self, ExtractedView, ViewTarget, VisibilitySystems, VisibleEntities},
        Render, RenderApp, RenderSet,
    },
};

/// Component that associates a generated shader to a mesh.
#[derive(Component, Default, Clone, ExtractComponent)]
pub struct NodeShader3dHandle(pub Handle<Shader>);

/// A query filter that tells [`view::check_visibility`] about our custom
/// rendered entity.
type WithNodeShader3dHandle = With<NodeShader3dHandle>;

/// Custom pipeline for meshes with vertex colors
#[derive(Resource)]
pub struct NodeShaderMesh3dPipeline {
    /// this pipeline wraps the standard [`MeshPipeline`]
    mesh_pipeline: MeshPipeline,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct NodeShaderMesh3dPipelineKey {
    /// Handle to the generated shader.
    shader: Handle<Shader>,
    /// Key for the mesh pipeline
    mesh_key: MeshPipelineKey,
}

impl FromWorld for NodeShaderMesh3dPipeline {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh_pipeline: MeshPipeline::from_world(world),
        }
    }
}

// We implement `SpecializedPipeline` to customize the default rendering from `MeshPipeline`
impl SpecializedRenderPipeline for NodeShaderMesh3dPipeline {
    type Key = NodeShaderMesh3dPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        // Customize how to store the meshes' vertex attributes in the vertex buffer
        // Our meshes only have positions
        let formats = vec![VertexFormat::Float32x3];

        let vertex_layout =
            VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);

        let format = match key.mesh_key.contains(MeshPipelineKey::HDR) {
            true => ViewTarget::TEXTURE_FORMAT_HDR,
            false => TextureFormat::bevy_default(),
        };

        RenderPipelineDescriptor {
            vertex: VertexState {
                // Use our custom shader
                shader: key.shader.clone(),
                entry_point: "vertex".into(),
                shader_defs: vec![],
                // Use our custom vertex buffer
                buffers: vec![vertex_layout],
            },
            fragment: Some(FragmentState {
                // Use our custom shader
                shader: key.shader.clone(),
                shader_defs: vec![],
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            layout: vec![
                // Bind group 0 is the view uniform
                self.mesh_pipeline
                    .get_view_layout(MeshPipelineViewLayoutKey::from(key.mesh_key))
                    .clone(),
                // Bind group 1 is the mesh uniform
                self.mesh_pipeline.mesh_layouts.model_only.clone(),
            ],
            push_constant_ranges: vec![],
            primitive: PrimitiveState {
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
                topology: key.mesh_key.primitive_topology(),
                strip_index_format: None,
            },
            depth_stencil: Some(DepthStencilState {
                format: CORE_3D_DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Greater,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState {
                count: key.mesh_key.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("node_shader_mesh_pipeline".into()),
        }
    }
}

// This specifies how to render a node-shader mesh
type DrawNodeShaderMeshCommands = (
    // Set the pipeline
    SetItemPipeline,
    // Set the view uniform as bind group 0
    SetMeshViewBindGroup<0>,
    // Set the mesh uniform as bind group 1
    SetMeshBindGroup<1>,
    // Draw the mesh
    DrawMesh,
);

/// A render-world system that enqueues the entity with custom rendering into
/// the opaque render phases of each view.
#[allow(clippy::too_many_arguments)]
fn queue_node_shader_item(
    pipeline_cache: Res<PipelineCache>,
    custom_phase_pipeline: Res<NodeShaderMesh3dPipeline>,
    msaa: Res<Msaa>,
    mut opaque_render_phases: ResMut<ViewBinnedRenderPhases<Opaque3d>>,
    opaque_draw_functions: Res<DrawFunctions<Opaque3d>>,
    mut specialized_render_pipelines: ResMut<SpecializedRenderPipelines<NodeShaderMesh3dPipeline>>,
    views: Query<(Entity, &VisibleEntities, &ExtractedView), With<ExtractedView>>,
    render_meshes: Res<RenderAssets<GpuMesh>>,
    render_mesh_instances: Res<RenderMeshInstances>,
    shader_handle: Query<&NodeShader3dHandle>,
) {
    let draw_custom_phase_item = opaque_draw_functions
        .read()
        .id::<DrawNodeShaderMeshCommands>();

    // Render phases are per-view, so we need to iterate over all views so that
    // the entity appears in them. (In this example, we have only one view, but
    // it's good practice to loop over all views anyway.)
    for (view_entity, view_visible_entities, view) in views.iter() {
        let Some(opaque_phase) = opaque_render_phases.get_mut(&view_entity) else {
            continue;
        };

        let mesh_key = MeshPipelineKey::from_msaa_samples(msaa.samples())
            | MeshPipelineKey::from_hdr(view.hdr);

        // Find all the custom rendered entities that are visible from this
        // view.
        for &visible_entity in view_visible_entities.get::<WithNodeShader3dHandle>().iter() {
            let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(visible_entity)
            else {
                continue;
            };

            let mut mesh_key = mesh_key;
            if let Some(mesh) = render_meshes.get(mesh_instance.mesh_asset_id) {
                mesh_key |= MeshPipelineKey::from_primitive_topology(mesh.primitive_topology());
            }

            let shader = shader_handle.get(visible_entity).unwrap();
            let pipeline_id = specialized_render_pipelines.specialize(
                &pipeline_cache,
                &custom_phase_pipeline,
                NodeShaderMesh3dPipelineKey {
                    shader: shader.0.clone(),
                    mesh_key,
                },
            );

            // Add the custom render item. We use the
            // [`BinnedRenderPhaseType::NonMesh`] type to skip the special
            // handling that Bevy has for meshes (preprocessing, indirect
            // draws, etc.)
            //
            // The asset ID is arbitrary; we simply use [`AssetId::invalid`],
            // but you can use anything you like. Note that the asset ID need
            // not be the ID of a [`Mesh`].
            opaque_phase.add(
                Opaque3dBinKey {
                    draw_function: draw_custom_phase_item,
                    pipeline: pipeline_id,
                    asset_id: AssetId::<Mesh>::invalid().untyped(),
                    material_bind_group_id: None,
                    lightmap_image: None,
                },
                visible_entity,
                BinnedRenderPhaseType::UnbatchableMesh,
            );
        }
    }
}

/// Plugin that renders [`NodeShaderMesh3d`]s
pub struct NodeShaderMeshPlugin;

impl Plugin for NodeShaderMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<NodeShader3dHandle>::default())
            .add_systems(
                PostUpdate,
                // Make sure to tell Bevy to check our entity for visibility. Bevy won't
                // do this by default, for efficiency reasons.
                view::check_visibility::<WithNodeShader3dHandle>
                    .in_set(VisibilitySystems::CheckVisibility),
            );

        // We make sure to add these to the render app, not the main app.
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .init_resource::<SpecializedRenderPipelines<NodeShaderMesh3dPipeline>>()
            .add_render_command::<Opaque3d, DrawNodeShaderMeshCommands>()
            .add_systems(Render, queue_node_shader_item.in_set(RenderSet::Queue));
    }

    fn finish(&self, app: &mut App) {
        // Register our custom pipeline
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.init_resource::<NodeShaderMesh3dPipeline>();
    }
}
