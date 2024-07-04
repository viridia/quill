use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_mod_stylebuilder::{StyleBuilder, StyleTuple};

use crate::{
    cx::Cx,
    effects::{self, AppendEffect, CallbackEffect, EffectTuple, EntityEffect},
    insert::{ConditionalInsertComponentEffect, InsertBundleEffect, StaticInsertBundleEffect},
    node_span::NodeSpan,
    style::{ApplyDynamicStylesEffect, ApplyStaticStylesEffect},
    view::View,
};

/// A view which generates an entity bundle.
#[derive(Default)]
pub struct Element<B: Bundle + Default = NodeBundle, C: View = (), E: EffectTuple = ()> {
    /// Debug name for this element.
    debug_name: String,

    /// The visible UI node for this element.
    display: Option<Entity>,

    /// Children of this element.
    children: C,

    /// List of effects to be added to the element.
    effects: E,

    marker: PhantomData<B>,
}

impl<B: Bundle + Default> Element<B, (), ()> {
    /// Construct a new `Element`.
    pub fn new() -> Self {
        Self {
            debug_name: String::new(),
            display: None,
            children: (),
            effects: (),
            marker: PhantomData,
        }
    }

    /// Construct a new `Element` with a given entity id.
    pub fn for_entity(node: Entity) -> Self {
        Self {
            debug_name: String::new(),
            display: Some(node),
            children: (),
            effects: (),
            marker: PhantomData,
        }
    }
}

impl<B: Bundle + Default, C: View, E: EffectTuple> Element<B, C, E> {
    /// Set the debug name for this element.
    pub fn named(mut self, name: &str) -> Self {
        self.debug_name = name.to_string();
        self
    }

    /// Set the child entities for this element.
    pub fn children<C2: View>(self, children: C2) -> Element<B, C2, E> {
        Element {
            children,
            debug_name: self.debug_name,
            display: self.display,
            effects: self.effects,
            marker: PhantomData,
        }
    }

    /// Add an effect to this element.
    pub fn add_effect<E1: EntityEffect>(
        self,
        effect: E1,
    ) -> Element<B, C, <E as AppendEffect<E1>>::Result>
    where
        E: AppendEffect<E1>,
    {
        Element {
            children: self.children,
            debug_name: self.debug_name,
            display: self.display,
            effects: self.effects.append_effect(effect),
            marker: PhantomData,
        }
    }

    /// Add a general-purpose effect which can mutate the display entity.
    pub fn effect<S: Fn(&mut Cx, Entity, D) + Send + Sync, D: PartialEq + Clone + Send + Sync>(
        self,
        effect_fn: S,
        deps: D,
    ) -> Element<B, C, <E as AppendEffect<CallbackEffect<S, D>>>::Result>
    where
        E: AppendEffect<CallbackEffect<S, D>>,
    {
        self.add_effect(CallbackEffect { effect_fn, deps })
    }

    /// Apply a set of styles to the element
    pub fn style<S: StyleTuple + 'static>(
        self,
        styles: S,
    ) -> Element<B, C, <E as AppendEffect<ApplyStaticStylesEffect<S>>>::Result>
    where
        E: AppendEffect<ApplyStaticStylesEffect<S>>,
    {
        self.add_effect(ApplyStaticStylesEffect { styles })
    }

    /// Apply a set of dynamic styles to the element. This will be re-run whenever the
    /// dependencies change.
    ///
    /// Arguments:
    /// - style_fn: A function which computes the styles based on the dependencies.
    /// - deps: The dependencies which trigger a recompute of the styles.
    pub fn style_dyn<
        S: Fn(D, &mut StyleBuilder) + Send + Sync,
        D: PartialEq + Clone + Send + Sync,
    >(
        self,
        style_fn: S,
        deps: D,
    ) -> Element<B, C, <E as AppendEffect<ApplyDynamicStylesEffect<S, D>>>::Result>
    where
        E: AppendEffect<ApplyDynamicStylesEffect<S, D>>,
    {
        self.add_effect(ApplyDynamicStylesEffect { style_fn, deps })
    }

    /// Insert a bundle into the target entity once and never update it.
    ///
    /// Arguments:
    /// - bundle: The bundle to insert.
    pub fn insert<B2: Bundle + Clone>(
        self,
        bundle: B2,
    ) -> Element<B, C, <E as AppendEffect<StaticInsertBundleEffect<B2>>>::Result>
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
    ) -> Element<B, C, <E as AppendEffect<InsertBundleEffect<B2, S, D>>>::Result>
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
    ) -> Element<B, C, <E as AppendEffect<ConditionalInsertComponentEffect<C2, S>>>::Result>
    where
        E: AppendEffect<ConditionalInsertComponentEffect<C2, S>>,
    {
        self.add_effect(ConditionalInsertComponentEffect { condition, factory })
    }
}

impl<B: Bundle + Default, C: View, E: EffectTuple + 'static> View for Element<B, C, E> {
    type State = (Entity, C::State, E::State);

    fn nodes(&self, _world: &World, state: &Self::State) -> NodeSpan {
        NodeSpan::Node(state.0)
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        let owner = cx.owner;
        if self.debug_name.is_empty() {
            cx.world_mut()
                .entity_mut(owner)
                .insert(Name::new("Element"));
        } else {
            cx.world_mut()
                .entity_mut(owner)
                .insert(Name::new(format!("Element::{}", self.debug_name)));
        }

        // Build display entity if it doesn't already exist.
        let display = match self.display {
            Some(display) => {
                cx.world_mut()
                    .entity_mut(display)
                    .insert((B::default(), Name::new(self.debug_name.clone())));
                display
            }
            None => cx
                .world_mut()
                .spawn((B::default(), Name::new(self.debug_name.clone())))
                .id(),
        };

        // Run attached effects.
        let eff_state = effects::EffectTuple::apply(&self.effects, cx, display);

        // Build child nodes.
        let children = self.children.build(cx);
        let nodes = self.children.nodes(cx.world(), &children);
        cx.world_mut()
            .entity_mut(display)
            .replace_children(&nodes.to_vec());
        (display, children, eff_state)
    }

    fn rebuild(&self, cx: &mut crate::cx::Cx, state: &mut Self::State) -> bool {
        effects::EffectTuple::reapply(&self.effects, cx, state.0, &mut state.2);
        if self.children.rebuild(cx, &mut state.1) {
            self.attach_children(cx.world_mut(), state);
        }
        // Note that we always return false, since the Element entity doesn't change.
        false
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        #[cfg(feature = "verbose")]
        info!("Razing element: {}", state.0);

        // Delete the display node.
        world.entity_mut(state.0).remove_parent();
        if self.display.is_none() {
            // Only despawn the display entity if we created it. If we got it from the outside,
            // then it's the responsibility of the caller to clean it up.
            world.entity_mut(state.0).despawn();
        } else {
            world.entity_mut(state.0).remove::<B>();
        }
        self.children.raze(world, &mut state.1);
    }

    fn attach_children(&self, world: &mut World, state: &mut Self::State) -> bool {
        assert!(world.get_entity(state.0).is_some());
        self.children.attach_children(world, &mut state.1);
        let nodes = self.children.nodes(world, &state.1);
        world.entity_mut(state.0).replace_children(&nodes.to_vec());
        false
    }
}
