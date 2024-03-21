use bevy::{input::keyboard::KeyCode, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup)
            .add_systems(Update, (player_input));
    }
}

#[derive(Component)]
struct PlayerTag;

#[derive(Bundle)]
struct PlayerBundle {
    tag: PlayerTag,
    sprite: SpriteBundle,
}

fn player_setup(mut commands: Commands, server: Res<AssetServer>) {
    let player_sprite: Handle<Image> = server.load("machitan.png");
    let player = PlayerBundle {
        tag: PlayerTag,
        sprite: SpriteBundle {
            texture: player_sprite,
            transform: Transform::from_xyz(0., 0., 100.),
            ..default()
        },
    };

    commands.spawn(player);
}

fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, Entity), With<PlayerTag>>,
) {
    for (mut position, _) in query.iter_mut() {
        let translate = 150. * time.delta_seconds();

        for key in keys.get_pressed() {
            match key {
                KeyCode::ArrowDown => position.translation.y -= translate,
                KeyCode::ArrowLeft => position.translation.x -= translate,
                KeyCode::ArrowRight => position.translation.x += translate,
                KeyCode::ArrowUp => position.translation.y += translate,
                _ => {}
            }
        }
    }
}
