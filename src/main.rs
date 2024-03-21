use bevy::{prelude::*, window::close_on_esc};

use machitan::{player::PlayerPlugin, GreetTimer};

fn main() {
    App::new()
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins((
            DefaultPlugins,
            // AssetLoadPlugin,
            PlayerPlugin,
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
