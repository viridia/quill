use bevy::app::Plugin;
use grayscale::Grayscale;

mod grayscale;

pub struct OperatorsPlugin;

impl Plugin for OperatorsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Grayscale>();
    }
}
