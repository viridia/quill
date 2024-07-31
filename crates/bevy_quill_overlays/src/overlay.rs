use bevy::{
    color::LinearRgba,
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
};
use bevy_mod_picking::backends::raycast::RaycastPickable;
use bevy_quill_core::{effects::*, insert::*, prelude::*};

use crate::{
    overlay_material::{OverlayMaterial, UnderlayMaterial},
    ShapeBuilder,
};

use super::mesh_builder::MeshBuilder;

/// A transluent overlay that can be used to display diagnostic information in the 3d world.
#[allow(clippy::type_complexity)]
pub struct Overlay<C: View = (), E: EffectTuple = ()> {
    /// Debug name for this element.
    debug_name: String,

    /// The visible entity for this overlay.
    display: Option<Entity>,

    /// Children of this element.
    children: C,

    /// List of effects to be added to the element.
    effects: E,

    /// Occlusion opacity, 0.0 to 1.0. This represents the opacity of the overlay when it is
    /// occluded by other objects.
    underlay: f32,

    /// Whether the overlay is pickable.
    pickable: bool,

    /// Mesh topology
    topology: PrimitiveTopology,
    // - blend_mode
    // - sides
}

pub struct OverlayState<C: View, E: EffectTuple> {
    /// Entity for the overlay.
    entity: Entity,

    /// State for child views
    child_states: C::State,

    /// State for effects
    effect_states: <E as EffectTuple>::State,
}

#[derive(Component, Clone)]
struct OverlayMeshState {
    material: Handle<OverlayMaterial>,
    underlay_material: Handle<UnderlayMaterial>,
    mesh: Handle<Mesh>,

    underlay: f32,
}

impl Overlay<(), ()> {
    /// Construct a new `Overlay`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a new `Element` with a given entity id.
    pub fn for_entity(node: Entity) -> Self {
        Self {
            display: Some(node),
            ..default()
        }
    }
}

impl<C: View, E: EffectTuple> Overlay<C, E> {
    /// Set the debug name for this element.
    pub fn named(mut self, name: &str) -> Self {
        self.debug_name = name.to_string();
        self
    }

    /// Set the child entities for this element.
    pub fn children<C2: View>(self, children: C2) -> Overlay<C2, E> {
        Overlay {
            children,
            debug_name: self.debug_name,
            display: self.display,
            effects: self.effects,
            underlay: self.underlay,
            pickable: self.pickable,
            topology: self.topology,
        }
    }

    /// "Underlay" controls the opacity of the overlay when it is occluded by other objects.
    /// A value of 0 means that occluded portions of the overlay are completely invisible,
    /// while a value of 1 means that the overlay is completely visible even when occluded.
    ///
    /// The default value is 0.3.
    pub fn underlay(mut self, underlay: f32) -> Self {
        self.underlay = underlay.clamp(0.0, 1.0);
        self
    }

    /// Whether this overlay shape should be pickable with `bevy_mod_picking`.
    pub fn pickable(mut self, pickable: bool) -> Self {
        self.pickable = pickable;
        self
    }

    /// Set the color for this overlay.
    pub fn color(
        self,
        color: impl Into<LinearRgba>,
    ) -> Overlay<C, <E as AppendEffect<OverlayColorEffect>>::Result>
    where
        E: AppendEffect<OverlayColorEffect>,
    {
        self.add_effect(OverlayColorEffect {
            color: color.into(),
        })
    }

    /// Set the transform for this overlay.
    pub fn transform(
        self,
        transform: impl Into<Transform>,
    ) -> Overlay<C, <E as AppendEffect<OverlayTransformEffect>>::Result>
    where
        E: AppendEffect<OverlayTransformEffect>,
    {
        self.add_effect(OverlayTransformEffect {
            transform: transform.into(),
        })
    }

    /// Add an effect to this element.
    pub fn add_effect<E1: EntityEffect>(
        self,
        effect: E1,
    ) -> Overlay<C, <E as AppendEffect<E1>>::Result>
    where
        E: AppendEffect<E1>,
    {
        Overlay {
            children: self.children,
            debug_name: self.debug_name,
            display: self.display,
            effects: self.effects.append_effect(effect),
            underlay: self.underlay,
            pickable: self.pickable,
            topology: self.topology,
        }
    }

    /// Add a general-purpose effect which can mutate the display entity.
    pub fn effect<S: Fn(&mut Cx, Entity, D) + Send + Sync, D: PartialEq + Clone + Send + Sync>(
        self,
        effect_fn: S,
        deps: D,
    ) -> Overlay<C, <E as AppendEffect<CallbackEffect<S, D>>>::Result>
    where
        E: AppendEffect<CallbackEffect<S, D>>,
    {
        self.add_effect(CallbackEffect { effect_fn, deps })
    }

    /// Insert a bundle into the target entity once and never update it.
    ///
    /// Arguments:
    /// - bundle: The bundle to insert.
    pub fn insert<B2: Bundle + Clone>(
        self,
        bundle: B2,
    ) -> Overlay<C, <E as AppendEffect<StaticInsertBundleEffect<B2>>>::Result>
    where
        E: AppendEffect<StaticInsertBundleEffect<B2>>,
    {
        self.add_effect(StaticInsertBundleEffect { bundle })
    }

    /// Insert a bundle into the target entity. This will be re-run whenever the dependencies
    /// change.
    ///
    /// Arguments:
    /// - bundle_gen: A function which computes the bundle based on the dependencies.
    /// - deps: The dependencies which trigger a recompute of the bundle.
    pub fn insert_dyn<
        B2: Bundle,
        S: Fn(D) -> B2 + Send + Sync,
        D: PartialEq + Clone + Send + Sync,
    >(
        self,
        bundle_gen: S,
        deps: D,
    ) -> Overlay<C, <E as AppendEffect<InsertBundleEffect<B2, S, D>>>::Result>
    where
        E: AppendEffect<InsertBundleEffect<B2, S, D>>,
    {
        self.add_effect(InsertBundleEffect {
            factory: bundle_gen,
            deps,
        })
    }

    /// Insert a component into the target entity if the condition is true. If the condition
    /// later becomes false, the component will be removed.
    ///
    /// Arguments:
    /// - condition: if true, the bundle will be inserted.
    /// - factory: A function which computes the component based on the dependencies.
    pub fn insert_if<C2: Component, S: Fn() -> C2 + Send + Sync>(
        self,
        condition: bool,
        factory: S,
    ) -> Overlay<C, <E as AppendEffect<ConditionalInsertComponentEffect<C2, S>>>::Result>
    where
        E: AppendEffect<ConditionalInsertComponentEffect<C2, S>>,
    {
        self.add_effect(ConditionalInsertComponentEffect { condition, factory })
    }

    /// Compute the mesh vertices of the overlay. This will be run once.
    pub fn shape<S: Fn(&mut ShapeBuilder) + Send + Sync>(
        self,
        shape_fn: S,
    ) -> Overlay<C, <E as AppendEffect<StaticMeshEffect<ShapeBuilder, S>>>::Result>
    where
        E: AppendEffect<StaticMeshEffect<ShapeBuilder, S>>,
    {
        Self {
            topology: ShapeBuilder::topology(),
            ..self
        }
        .add_effect(StaticMeshEffect {
            shape_fn,
            marker: std::marker::PhantomData,
        })
    }

    /// Compute the mesh vertices. This will be re-run whenever the
    /// dependencies change.
    ///
    /// Arguments:
    /// - shape_fn: A function which computes the mesh vertices.
    /// - deps: The dependencies which trigger a recompute of the styles.
    pub fn shape_dyn<
        S: Fn(D, &mut ShapeBuilder) + Send + Sync,
        D: PartialEq + Clone + Send + Sync,
    >(
        self,
        shape_fn: S,
        deps: D,
    ) -> Overlay<C, <E as AppendEffect<DynamicMeshEffect<ShapeBuilder, S, D>>>::Result>
    where
        E: AppendEffect<DynamicMeshEffect<ShapeBuilder, S, D>>,
    {
        Self {
            topology: ShapeBuilder::topology(),
            ..self
        }
        .add_effect(DynamicMeshEffect {
            shape_fn,
            deps,
            marker: std::marker::PhantomData,
        })
    }

    /// Compute the mesh vertices of the overlay. This will be run once.
    pub fn mesh<M: MeshBuilder + Default + Send + Sync + 'static, S: Fn(&mut M) + Send + Sync>(
        self,
        shape_fn: S,
    ) -> Overlay<C, <E as AppendEffect<StaticMeshEffect<M, S>>>::Result>
    where
        E: AppendEffect<StaticMeshEffect<M, S>>,
    {
        Self {
            topology: M::topology(),
            ..self
        }
        .add_effect(StaticMeshEffect {
            shape_fn,
            marker: std::marker::PhantomData,
        })
    }

    /// Compute the mesh vertices. This will be re-run whenever the
    /// dependencies change.
    ///
    /// Arguments:
    /// - shape_fn: A function which computes the mesh vertices.
    /// - deps: The dependencies which trigger a recompute of the styles.
    pub fn mesh_dyn<
        M: MeshBuilder + Default + Send + Sync + 'static,
        S: Fn(D, &mut M) + Send + Sync,
        D: PartialEq + Clone + Send + Sync,
    >(
        self,
        shape_fn: S,
        deps: D,
    ) -> Overlay<C, <E as AppendEffect<DynamicMeshEffect<M, S, D>>>::Result>
    where
        E: AppendEffect<DynamicMeshEffect<M, S, D>>,
    {
        Self {
            topology: M::topology(),
            ..self
        }
        .add_effect(DynamicMeshEffect {
            shape_fn,
            deps,
            marker: std::marker::PhantomData,
        })
    }
}

impl Default for Overlay<(), ()> {
    fn default() -> Self {
        Self {
            debug_name: String::new(),
            display: None,
            children: (),
            effects: (),
            underlay: 0.3,
            pickable: false,
            topology: PrimitiveTopology::TriangleList,
        }
    }
}

impl<C: View, E: EffectTuple + 'static> View for Overlay<C, E> {
    type State = OverlayState<C, E>;

    fn nodes(&self, _world: &World, state: &Self::State, out: &mut Vec<Entity>) {
        out.push(state.entity);
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        let owner = cx.owner();
        if self.debug_name.is_empty() {
            cx.world_mut()
                .entity_mut(owner)
                .insert(Name::new("Overlay"));
        } else {
            cx.world_mut()
                .entity_mut(owner)
                .insert(Name::new(format!("Overlay::{}", self.debug_name)));
        }

        let mesh = Mesh::new(self.topology, RenderAssetUsages::default());
        let mut meshes = cx.world_mut().get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh_handle = meshes.add(mesh);

        let mut materials = cx
            .world_mut()
            .get_resource_mut::<Assets<OverlayMaterial>>()
            .unwrap();
        let material = materials.add(OverlayMaterial {
            ..Default::default()
        });

        // TODO: only insert an underlay material if the underlay is between 0 and 1 (exclusive).
        // If it's zero, the underly is invisible.
        // If it's one, then we can just disable the depth test on the primary material.
        // if self.underlay > 0.0 && self.underlay < 1.0 {}
        let mut underlay_materials = cx
            .world_mut()
            .get_resource_mut::<Assets<UnderlayMaterial>>()
            .unwrap();
        let underlay_material = underlay_materials.add(UnderlayMaterial::default());

        let mesh_state = OverlayMeshState {
            material: material.clone(),
            underlay_material: underlay_material.clone(),
            mesh: mesh_handle.clone(),
            underlay: self.underlay,
        };

        let bundle = (
            mesh_state,
            MaterialMeshBundle::<OverlayMaterial> {
                material: material.clone(),
                mesh: mesh_handle.clone(),
                ..default()
            },
            NotShadowCaster,
            NotShadowReceiver,
        );

        // Build display entity if it doesn't already exist.
        let display = match self.display {
            Some(display) => {
                cx.world_mut()
                    .entity_mut(display)
                    .insert((bundle, Name::new(self.debug_name.clone())));
                display
            }
            None => cx
                .world_mut()
                .spawn((bundle, Name::new(self.debug_name.clone())))
                .id(),
        };

        cx.world_mut()
            .entity_mut(display)
            .insert(underlay_material.clone());

        if self.pickable && self.topology == PrimitiveTopology::TriangleList {
            cx.world_mut().entity_mut(display).insert(RaycastPickable);
        }

        // Run attached effects.
        let eff_state = EffectTuple::apply(&self.effects, cx, display);

        // Build child nodes.
        let children = self.children.build(cx);
        let mut nodes: Vec<Entity> = Vec::new();
        self.children.nodes(cx.world(), &children, &mut nodes);

        cx.world_mut()
            .entity_mut(display)
            .replace_children(&nodes.to_vec());

        OverlayState {
            entity: display,
            child_states: children,
            effect_states: eff_state,
        }
    }

    fn rebuild(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        EffectTuple::reapply(&self.effects, cx, state.entity, &mut state.effect_states);
        if self.children.rebuild(cx, &mut state.child_states) {
            View::attach_children(self, cx.world_mut(), state);
        }
        // Note that we always return false, since the Overlay entity doesn't change.
        false
    }

    fn raze(&self, world: &mut bevy::ecs::world::DeferredWorld, state: &mut Self::State) {
        // Delete the display node.
        world.commands().entity(state.entity).remove_parent();
        if self.display.is_none() {
            // Only despawn the display entity if we created it. If we got it from the outside,
            // then it's the responsibility of the caller to clean it up.
            world.commands().entity(state.entity).despawn();
        } else {
            world
                .commands()
                .entity(state.entity)
                .remove::<MaterialMeshBundle<OverlayMaterial>>();
        }
        self.children.raze(world, &mut state.child_states);
    }

    fn attach_children(&self, world: &mut World, state: &mut Self::State) -> bool {
        assert!(world.get_entity(state.entity).is_some());
        self.children
            .attach_children(world, &mut state.child_states);
        let mut nodes: Vec<Entity> = Vec::new();
        self.children.nodes(world, &state.child_states, &mut nodes);
        world
            .entity_mut(state.entity)
            .replace_children(&nodes.to_vec());
        false
    }
}

pub struct OverlayColorEffect {
    color: LinearRgba,
}

impl EntityEffect for OverlayColorEffect {
    type State = LinearRgba;
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        let target = cx.world_mut().entity_mut(target);
        let mesh_state = target.get::<OverlayMeshState>().unwrap().clone();
        let mut materials = cx
            .world_mut()
            .get_resource_mut::<Assets<OverlayMaterial>>()
            .unwrap();
        let material = materials.get_mut(mesh_state.material.id()).unwrap();
        material.color = self.color;

        let mut underlay_materials = cx
            .world_mut()
            .get_resource_mut::<Assets<UnderlayMaterial>>()
            .unwrap();
        if let Some(underlay_material) =
            underlay_materials.get_mut(mesh_state.underlay_material.id())
        {
            underlay_material.color = self
                .color
                .with_alpha(self.color.alpha * mesh_state.underlay);
        }

        self.color
    }

    fn reapply(&self, cx: &mut Cx, target: Entity, state: &mut Self::State) {
        if *state != self.color {
            *state = EntityEffect::apply(self, cx, target);
        }
    }
}

pub struct OverlayTransformEffect {
    transform: Transform,
}

impl EntityEffect for OverlayTransformEffect {
    type State = Transform;
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        let mut target = cx.world_mut().entity_mut(target);
        match target.get_mut::<Transform>() {
            Some(mut t) => {
                t.clone_from(&self.transform);
            }
            None => {
                target.insert(self.transform);
            }
        };
        self.transform
    }

    fn reapply(&self, cx: &mut Cx, target: Entity, state: &mut Self::State) {
        if *state != self.transform {
            *state = EntityEffect::apply(self, cx, target);
        }
    }
}

pub struct StaticMeshEffect<M, F: Fn(&mut M)> {
    shape_fn: F,
    marker: std::marker::PhantomData<M>,
}

impl<M: MeshBuilder + Default + Sync + Send, F: Fn(&mut M) + Send + Sync> EntityEffect
    for StaticMeshEffect<M, F>
{
    type State = ();
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        let target = cx.world_mut().entity_mut(target);
        let mut builder = M::default();
        (self.shape_fn)(&mut builder);
        let mesh_state = target.get::<OverlayMeshState>().unwrap().clone();
        let mut meshes = cx.world_mut().get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.get_mut(mesh_state.mesh.id()).unwrap();
        builder.build(mesh);
    }

    fn reapply(&self, _cx: &mut Cx, _target: Entity, _state: &mut Self::State) {}
}

pub struct DynamicMeshEffect<M, F: Fn(D, &mut M), D: PartialEq + Clone> {
    shape_fn: F,
    deps: D,
    marker: std::marker::PhantomData<M>,
}

impl<
        M: MeshBuilder + Default + Sync + Send,
        F: Fn(D, &mut M) + Send + Sync,
        D: PartialEq + Clone + Send + Sync,
    > EntityEffect for DynamicMeshEffect<M, F, D>
{
    type State = D;
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        let target_ent = cx.world_mut().entity_mut(target);
        let mut builder = M::default();
        (self.shape_fn)(self.deps.clone(), &mut builder);
        let mesh_state = target_ent.get::<OverlayMeshState>().unwrap().clone();
        let mut meshes = cx.world_mut().get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.get_mut(mesh_state.mesh.id()).unwrap();
        builder.build(mesh);
        if let Some(aabb) = mesh.compute_aabb() {
            let mut target_ent = cx.world_mut().entity_mut(target);
            target_ent.insert(aabb);
        }
        self.deps.clone()
    }

    fn reapply(&self, cx: &mut Cx, target: Entity, state: &mut Self::State) {
        if *state != self.deps {
            *state = EntityEffect::apply(self, cx, target);
        }
    }
}
