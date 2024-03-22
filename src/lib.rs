use bevy::prelude::*;
use player::PlayerPlugin;

pub mod player;

pub struct MachitanPlugin;

impl Plugin for MachitanPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin);
    }
}
