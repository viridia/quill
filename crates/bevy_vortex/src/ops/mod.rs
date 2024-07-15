use bevy::{app::Plugin, asset::embedded_asset};
use bricks::Bricks;
use grayscale::Grayscale;
use mix::Mix;
use output::Output;

mod bricks;
mod grayscale;
mod mix;
mod output;

pub struct OperatorsPlugin;

impl Plugin for OperatorsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        embedded_asset!(app, "wgsl/bricks.wgsl");
        embedded_asset!(app, "wgsl/smootherstep.wgsl");
        app.register_type::<Bricks>()
            .register_type::<Grayscale>()
            .register_type::<Mix>()
            .register_type::<Output>();
    }
}
