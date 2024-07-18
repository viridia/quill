use bevy::{app::Plugin, asset::embedded_asset};

mod bricks;
mod color;
mod geometry;
mod grayscale;
mod mix;
mod output;
mod simplex_noise;
mod wgsl;

use bricks::Bricks;
use color::ConstColor;
use geometry::Geometry;
use grayscale::Grayscale;
use mix::Mix;
use output::Output;
use simplex_noise::SimplexNoise;

pub struct OperatorsPlugin;

impl Plugin for OperatorsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        embedded_asset!(app, "wgsl/bricks.wgsl");
        embedded_asset!(app, "wgsl/smootherstep.wgsl");
        app.register_type::<Bricks>()
            .register_type::<ConstColor>()
            .register_type::<Geometry>()
            .register_type::<Grayscale>()
            .register_type::<Mix>()
            .register_type::<Output>()
            .register_type::<SimplexNoise>();
    }
}
