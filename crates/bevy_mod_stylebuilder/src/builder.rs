#![allow(missing_docs)]
//! Defines fluent builder for styles.

use bevy::{
    asset::AssetPath,
    color::{LinearRgba, Srgba},
    prelude::*,
    ui::{self, ZIndex},
};

/// An object that provides a fluent interface for defining styles for bevy_ui nodes.
/// Most components such as `BackgroundColor` are mutated immediately, however some component types
/// such as `Style` are cached in the builder and not applied until `finish` is called.
pub struct StyleBuilder<'a, 'w> {
    pub target: &'a mut EntityWorldMut<'w>,
    pub(crate) style: ui::Style,
    pub(crate) style_changed: bool,
}

impl<'a, 'w> StyleBuilder<'a, 'w> {
    /// Construct a new StyleBuilder instance.
    pub fn new(target: &'a mut EntityWorldMut<'w>, style: ui::Style) -> Self {
        Self {
            target,
            style,
            style_changed: false,
        }
    }

    /// Helper method for loading assets.
    pub fn load_asset<A: Asset>(&mut self, path: AssetPath<'_>) -> Handle<A> {
        self.target.world_scope(|world| {
            let server = world.get_resource::<AssetServer>().unwrap();
            server.load(path)
        })
    }

    /// Consumes the [`StyleBuilder`] and applies the style to the target entity.
    pub fn finish(self) {
        if self.style_changed {
            self.target.insert(self.style);
        }
    }
}

// LineBreak(BreakLineOn),

/// Trait that represents a CSS color
pub trait ColorParam {
    fn to_val(self) -> Option<Color>;
}

impl ColorParam for Option<Color> {
    fn to_val(self) -> Option<Color> {
        self
    }
}

impl ColorParam for Color {
    fn to_val(self) -> Option<Color> {
        Some(self)
    }
}

impl ColorParam for Srgba {
    fn to_val(self) -> Option<Color> {
        Some(Color::srgba(self.red, self.green, self.blue, self.alpha))
    }
}

impl ColorParam for Option<Srgba> {
    fn to_val(self) -> Option<Color> {
        self.map(|c| Color::srgba(c.red, c.green, c.blue, c.alpha))
    }
}

impl ColorParam for LinearRgba {
    fn to_val(self) -> Option<Color> {
        Some(self.into())
    }
}

impl ColorParam for Option<LinearRgba> {
    fn to_val(self) -> Option<Color> {
        self.map(|c| c.into())
    }
}

impl ColorParam for &str {
    fn to_val(self) -> Option<Color> {
        let c = Srgba::hex(self).unwrap();
        Some(Color::srgba(c.red, c.green, c.blue, c.alpha))
    }
}

/// Trait that represents a CSS "length"
pub trait LengthParam {
    fn to_val(self) -> ui::Val;
}

impl LengthParam for ui::Val {
    fn to_val(self) -> ui::Val {
        self
    }
}

impl LengthParam for f32 {
    fn to_val(self) -> ui::Val {
        ui::Val::Px(self)
    }
}

impl LengthParam for i32 {
    fn to_val(self) -> ui::Val {
        ui::Val::Px(self as f32)
    }
}

/// Trait that represents a CSS Z-index
pub trait ZIndexParam {
    fn to_val(self) -> ZIndex;
}

impl ZIndexParam for ZIndex {
    fn to_val(self) -> ZIndex {
        self
    }
}

impl ZIndexParam for i32 {
    fn to_val(self) -> ZIndex {
        ZIndex::Local(self)
    }
}

/// Trait that represents CSS edge widths (margin, padding, etc.)
pub trait UiRectParam {
    fn to_uirect(self) -> ui::UiRect;
}

impl UiRectParam for ui::UiRect {
    fn to_uirect(self) -> ui::UiRect {
        self
    }
}

impl UiRectParam for ui::Val {
    fn to_uirect(self) -> ui::UiRect {
        ui::UiRect::all(self)
    }
}

impl UiRectParam for f32 {
    fn to_uirect(self) -> ui::UiRect {
        ui::UiRect::all(ui::Val::Px(self))
    }
}

impl UiRectParam for i32 {
    fn to_uirect(self) -> ui::UiRect {
        ui::UiRect::all(ui::Val::Px(self as f32))
    }
}

impl<H: LengthParam, V: LengthParam> UiRectParam for (H, V) {
    fn to_uirect(self) -> ui::UiRect {
        ui::UiRect::axes(self.0.to_val(), self.1.to_val())
    }
}

/// Trait that represents border radius
pub trait BorderRadiusParam {
    fn to_border_radius(self) -> ui::BorderRadius;
}

impl BorderRadiusParam for ui::BorderRadius {
    fn to_border_radius(self) -> ui::BorderRadius {
        self
    }
}

impl BorderRadiusParam for ui::Val {
    fn to_border_radius(self) -> ui::BorderRadius {
        ui::BorderRadius::all(self)
    }
}

impl BorderRadiusParam for f32 {
    fn to_border_radius(self) -> ui::BorderRadius {
        ui::BorderRadius::all(ui::Val::Px(self))
    }
}

impl BorderRadiusParam for i32 {
    fn to_border_radius(self) -> ui::BorderRadius {
        ui::BorderRadius::all(ui::Val::Px(self as f32))
    }
}

/// Trait that represents an optional float
pub trait OptFloatParam {
    fn to_val(self) -> Option<f32>;
}

impl OptFloatParam for Option<f32> {
    fn to_val(self) -> Option<f32> {
        self
    }
}

impl OptFloatParam for f32 {
    fn to_val(self) -> Option<f32> {
        Some(self)
    }
}

impl OptFloatParam for i32 {
    fn to_val(self) -> Option<f32> {
        Some(self as f32)
    }
}

/// Trait that represents an optional float
pub trait AssetPathParam<'a> {
    fn to_path(self) -> Option<AssetPath<'a>>;
}

impl<'a> AssetPathParam<'a> for Option<AssetPath<'a>> {
    fn to_path(self) -> Option<AssetPath<'a>> {
        self
    }
}

impl<'a> AssetPathParam<'a> for AssetPath<'a> {
    fn to_path(self) -> Option<AssetPath<'a>> {
        Some(self)
    }
}

impl<'a> AssetPathParam<'a> for &'a str {
    fn to_path(self) -> Option<AssetPath<'a>> {
        Some(AssetPath::parse(self))
    }
}
