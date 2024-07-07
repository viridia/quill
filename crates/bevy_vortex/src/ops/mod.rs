use bevy::app::Plugin;
use grayscale::Grayscale;
use mix::Mix;
use output::Output;

mod grayscale;
mod mix;
mod output;

pub struct OperatorsPlugin;

impl Plugin for OperatorsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Grayscale>()
            .register_type::<Mix>()
            .register_type::<Output>();
    }
}
