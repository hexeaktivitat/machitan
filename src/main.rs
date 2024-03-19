use bevy::prelude::*;

use machitan::*;

fn main() {
    App::new().add_systems(Update, hello_world).run();
}
