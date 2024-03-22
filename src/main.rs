use bevy::{prelude::*, window::close_on_esc};
use bevy_console::ConsolePlugin;

use machitan::MachitanPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // AssetLoadPlugin,
            ConsolePlugin,
            MachitanPlugin,
            ViewportPlugin,
        ))
        .add_systems(Update, (close_on_esc))
        .run();
}

struct ViewportPlugin;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
    }
}

fn camera_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}

fn field_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {}
