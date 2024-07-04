use bevy::prelude::*;
use bevy::{
    color::{Mix, Srgba},
    ecs::component::Component,
    math::{cubic_splines::CubicSegment, Vec2},
    ui::{self, BackgroundColor, BorderColor, Style},
};

/// Trait that represents a property that can be animated, such as background color,
/// transform, and so on.
pub trait AnimatableProperty {
    /// The data type of the animated property.
    type ValueType: Copy + Send + Sync + PartialEq + 'static;

    /// The type of component that contains the animated property.
    type ComponentType: Component;

    /// Get the current value of the animatable property.
    fn current(component: &Self::ComponentType) -> Self::ValueType;

    /// Update the value of the animatable property.
    fn update(
        component: &mut Self::ComponentType,
        value: f32,
        origin: Self::ValueType,
        target: Self::ValueType,
    );
}

/// Animated background color property.
pub struct AnimatedBackgroundColor;
impl AnimatableProperty for AnimatedBackgroundColor {
    type ValueType = Srgba;
    type ComponentType = BackgroundColor;

    fn current(component: &Self::ComponentType) -> Self::ValueType {
        component.0.into()
    }

    fn update(component: &mut Self::ComponentType, value: f32, origin: Srgba, target: Srgba) {
        component.0 = origin.mix(&target, value).into();
    }
}

/// Animated border color property.
pub struct AnimatedBorderColor;
impl AnimatableProperty for AnimatedBorderColor {
    type ValueType = Srgba;
    type ComponentType = BorderColor;

    fn current(component: &Self::ComponentType) -> Self::ValueType {
        component.0.into()
    }

    fn update(component: &mut Self::ComponentType, value: f32, origin: Srgba, target: Srgba) {
        component.0 = origin.mix(&target, value).into();
    }
}

/// Animated pixel width property.
pub struct AnimatedPxWidth;
impl AnimatableProperty for AnimatedPxWidth {
    type ValueType = f32;
    type ComponentType = Style;

    fn current(component: &Self::ComponentType) -> Self::ValueType {
        if let ui::Val::Px(value) = component.width {
            value
        } else {
            0.0
        }
    }

    fn update(component: &mut Self::ComponentType, value: f32, origin: f32, target: f32) {
        component.width = ui::Val::Px(origin.lerp(target, value));
    }
}

/// Animated pixel height property.
pub struct AnimatedPxHeight;
impl AnimatableProperty for AnimatedPxHeight {
    type ValueType = f32;
    type ComponentType = Style;

    fn current(component: &Self::ComponentType) -> Self::ValueType {
        if let ui::Val::Px(value) = component.height {
            value
        } else {
            0.0
        }
    }

    fn update(component: &mut Self::ComponentType, value: f32, origin: f32, target: f32) {
        component.height = ui::Val::Px(origin.lerp(target, value));
    }
}

/// Animated scale.
pub struct AnimatedScale;
impl AnimatableProperty for AnimatedScale {
    type ValueType = Vec3;
    type ComponentType = Transform;

    fn current(component: &Self::ComponentType) -> Self::ValueType {
        component.scale
    }

    fn update(trans: &mut Self::ComponentType, t: f32, origin: Vec3, target: Vec3) {
        trans.scale = origin.lerp(target, t);
    }
}

/// Animated scale.
pub struct AnimatedRotation;
impl AnimatableProperty for AnimatedRotation {
    type ValueType = Quat;
    type ComponentType = Transform;

    fn current(component: &Self::ComponentType) -> Self::ValueType {
        component.rotation
    }

    fn update(trans: &mut Self::ComponentType, t: f32, origin: Quat, target: Quat) {
        trans.rotation = origin.lerp(target, t);
    }
}

/// Animated translation.
pub struct AnimatedTranslation;
impl AnimatableProperty for AnimatedTranslation {
    type ValueType = Vec3;
    type ComponentType = Transform;

    fn current(component: &Self::ComponentType) -> Self::ValueType {
        component.translation
    }

    fn update(trans: &mut Self::ComponentType, t: f32, origin: Vec3, target: Vec3) {
        trans.translation = origin.lerp(target, t);
    }
}

/// ECS component that animates a visual property of a UI node.
#[derive(Component)]
pub struct AnimatedTransition<T>
where
    T: AnimatableProperty,
{
    timing: CubicSegment<Vec2>,
    origin: T::ValueType,
    target: T::ValueType,
    delay: f32,
    duration: f32,
    clock: f32,
}

impl<T> AnimatedTransition<T>
where
    T: AnimatableProperty + 'static,
{
    /// Create a new animated transition.
    pub fn new(origin: T::ValueType, target: T::ValueType, duration: f32, delay: f32) -> Self {
        Self {
            timing: CubicSegment::new_bezier(Vec2::new(0.25, 0.1), Vec2::new(0.25, 1.0)),
            origin,
            target,
            clock: 0.0,
            duration,
            delay,
        }
    }

    /// Start a new animated transition.
    /// If the entity already has an animated transition of the same type, the transition will be
    /// restarted with the new target value.
    pub fn start(entity: &mut EntityWorldMut, target: T::ValueType, duration: f32) {
        // If we're already animating to the same target, don't restart the animation.
        if let Some(anim) = entity.get_mut::<Self>() {
            if anim.target == target {
                return;
            }
        }
        if let Some(mut cmp) = entity.get_mut::<T::ComponentType>() {
            let origin = T::current(&cmp);
            let mut transition = Self::new(origin, target, duration, 0.0);
            transition.advance(&mut cmp, 0.0);
            entity.insert(transition);
        }
    }

    /// Set the initial delay of the effect.
    pub fn with_delay(&mut self, delay: f32) {
        self.delay = delay;
    }

    /// Set the easing curve of the effect.
    pub fn with_timing(&mut self, p1: Vec2, p2: Vec2) {
        self.timing = CubicSegment::new_bezier(p1, p2);
    }

    /// Restart the transition with a new target value.
    pub fn restart(&mut self, target: T::ValueType) {
        self.target = target;
        self.clock = 0.0;
    }

    /// Advance the transition by a given time step.
    pub fn advance(&mut self, component: &mut T::ComponentType, time: f32) {
        self.clock += time;
        if self.clock < self.delay {
            return;
        }
        let t = if self.duration > 0.0001 {
            ((self.clock - self.delay) / self.duration).min(1.0)
        } else {
            1.0
        };
        let t = self.timing.ease(t);
        T::update(component, t, self.origin, self.target);
    }

    pub(crate) fn run_animations(
        mut commands: Commands,
        mut query: Query<(Entity, &mut AnimatedTransition<T>, &mut T::ComponentType)>,
        time: Res<Time>,
    ) {
        for (entity, mut transition, mut cmp) in query.iter_mut() {
            transition.advance(&mut cmp, time.delta_seconds());
            if transition.clock >= transition.delay + transition.duration {
                commands.entity(entity).remove::<AnimatedTransition<T>>();
            }
        }
    }
}

/// Plugin to drive animated transitions.
pub struct AnimatedTransitionPlugin;

impl Plugin for AnimatedTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                AnimatedTransition::<AnimatedBackgroundColor>::run_animations,
                AnimatedTransition::<AnimatedBorderColor>::run_animations,
                AnimatedTransition::<AnimatedPxWidth>::run_animations,
                AnimatedTransition::<AnimatedPxHeight>::run_animations,
                AnimatedTransition::<AnimatedScale>::run_animations,
                AnimatedTransition::<AnimatedRotation>::run_animations,
                AnimatedTransition::<AnimatedTranslation>::run_animations,
            ),
        );
    }
}
