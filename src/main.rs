use bevy::prelude::*;

use machitan::*;

fn main() {
    App::new().add_system(Update, hello_world).run();
}
