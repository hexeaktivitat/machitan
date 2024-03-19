use bevy::prelude::*;

use machitan::*;

fn main() {
    App::new()
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}
