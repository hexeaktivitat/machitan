use bevy::{input::keyboard::KeyCode, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_setup, ganbaru_mun))
            .add_systems(Update, (player_input, play_mun, pause))
            .add_event::<MunIdEvent>()
            .add_event::<PauseEvent>();
    }
}

#[derive(Component)]
struct PlayerTag;

#[derive(Component)]
struct Pause {
    active: bool,
}

#[derive(Bundle)]
struct PlayerBundle {
    tag: PlayerTag,
    sprite: SpriteBundle,
    active: Pause,
}

// player specific systems

fn ganbaru_mun(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: server.load("ganbaru.ogg"),
        ..default()
    });
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
        active: Pause { active: false },
    };
    commands.spawn(player);
}

fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut ev_play_mun: EventWriter<MunIdEvent>,
    mut ev_pause: EventWriter<PauseEvent>,
    mut query: Query<(&mut Transform, Entity, &Pause), With<PlayerTag>>,
) {
    for (mut position, _entity, pause) in query.iter_mut() {
        let translate = 250. * time.delta_seconds();

        for key in keys.get_pressed() {
            if pause.active {
                break;
            }

            match key {
                KeyCode::ArrowDown => position.translation.y -= translate,
                KeyCode::ArrowLeft => position.translation.x -= translate,
                KeyCode::ArrowRight => position.translation.x += translate,
                KeyCode::ArrowUp => position.translation.y += translate,
                _ => {}
            }
        }

        for key in keys.get_just_pressed() {
            if pause.active {
                if key == &KeyCode::Backquote {
                    ev_pause.send(PauseEvent);
                }
                break;
            }
            if key == &KeyCode::KeyA {
                ev_play_mun.send(MunIdEvent(1));
            }
            if key == &KeyCode::KeyS {
                ev_play_mun.send(MunIdEvent(2));
            }
            if key == &KeyCode::KeyD {
                ev_play_mun.send(MunIdEvent(3));
            }
            if key == &KeyCode::KeyF {
                ev_play_mun.send(MunIdEvent(4));
            }
            if key == &KeyCode::KeyJ {
                ev_play_mun.send(MunIdEvent(5));
            }
            if key == &KeyCode::KeyK {
                ev_play_mun.send(MunIdEvent(6));
            }
            if key == &KeyCode::KeyL {
                ev_play_mun.send(MunIdEvent(7));
            }
            if key == &KeyCode::Semicolon {
                ev_play_mun.send(MunIdEvent(8));
            }
            if key == &KeyCode::Space {
                ev_play_mun.send(MunIdEvent(9));
            }
            if key == &KeyCode::Enter {
                ev_play_mun.send(MunIdEvent(0));
            }
            if key == &KeyCode::Backquote {
                ev_pause.send(PauseEvent);
            }
        }
    }
}

// player specific events

#[derive(Event)]
struct MunIdEvent(u8);

fn play_mun(
    mut ev_play_mun: EventReader<MunIdEvent>,
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    for ev in ev_play_mun.read() {
        let mun = server.load(match ev.0 {
            0 => "mun10.ogg",
            1 => "mun1.ogg",
            2 => "mun2.ogg",
            3 => "mun3.ogg",
            4 => "mun4.ogg",
            5 => "mun5.ogg",
            6 => "mun6.ogg",
            7 => "mun7.ogg",
            8 => "mun8.ogg",
            9 => "mun9.ogg",
            _ => return,
        });

        commands.spawn(AudioBundle {
            source: mun,
            ..default()
        });
    }
}

#[derive(Event)]
struct PauseEvent;

fn pause(mut ev_pause: EventReader<PauseEvent>, mut query: Query<&mut Pause>) {
    for _ev in ev_pause.read() {
        for mut pause in query.iter_mut() {
            // pauses and unpauses game
            pause.active = !pause.active;
        }
    }
}
