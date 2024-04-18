use bevy::{input::keyboard::KeyCode, prelude::*};

use crate::{
    note::{NoteId, NoteLane, NoteTag},
    FramesCount, PauseState,
};

pub struct PlayerPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_setup, ganbaru_mun))
            .add_systems(Update, (player_input, play_mun, lane_tap, pause))
            .add_event::<MunIdEvent>()
            .add_event::<LaneTapEvent>()
            .add_event::<PauseEvent>();
    }
}

#[derive(Component)]
struct PlayerTag;

#[derive(Component)]
pub struct Pause {
    pub active: bool,
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
    println!("player setup");
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
    // mut ev_play_mun: EventWriter<MunIdEvent>,
    mut ev_lane_tap: EventWriter<LaneTapEvent>,
    mut ev_pause: EventWriter<PauseEvent>,
    mut query: Query<(&mut Transform, Entity, &Pause), With<PlayerTag>>,
) {
    for (mut position, _entity, pause) in query.iter_mut() {
        let translate = 250. * time.delta_seconds();

        for key in keys.get_pressed() {
            match key {
                KeyCode::ArrowDown => position.translation.y -= translate,
                KeyCode::ArrowLeft => position.translation.x -= translate,
                KeyCode::ArrowRight => position.translation.x += translate,
                KeyCode::ArrowUp => position.translation.y += translate,
                _ => {}
            }
        }

        for key in keys.get_just_pressed() {
            if key == &KeyCode::KeyA {
                ev_lane_tap.send(LaneTapEvent(NoteLane::LaneA));
            }
            if key == &KeyCode::KeyS {
                ev_lane_tap.send(LaneTapEvent(NoteLane::LaneS));
            }
            if key == &KeyCode::KeyD {
                ev_lane_tap.send(LaneTapEvent(NoteLane::LaneD));
            }
            if key == &KeyCode::KeyF {
                ev_lane_tap.send(LaneTapEvent(NoteLane::LaneF));
            }
            if key == &KeyCode::KeyJ {
                ev_lane_tap.send(LaneTapEvent(NoteLane::LaneJ));
            }
            if key == &KeyCode::KeyK {
                ev_lane_tap.send(LaneTapEvent(NoteLane::LaneK));
            }
            if key == &KeyCode::KeyL {
                ev_lane_tap.send(LaneTapEvent(NoteLane::LaneL));
            }
            if key == &KeyCode::Semicolon {
                ev_lane_tap.send(LaneTapEvent(NoteLane::LaneSemicolon));
            }
            if key == &KeyCode::Space {
                ev_pause.send(PauseEvent);
            }
            if key == &KeyCode::Enter {
                //              ev_lane_tap.send(LaneTapEvent(0));
            }
            if key == &KeyCode::Backquote {
                ev_pause.send(PauseEvent);
            }
        }
    }
}

// player specific events

#[derive(Event)]
struct MunIdEvent(NoteLane);

fn play_mun(
    mut ev_play_mun: EventReader<MunIdEvent>,
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    for ev in ev_play_mun.read() {
        let mun = server.load(match ev.0 {
            NoteLane::LaneA => "mun1.ogg",
            NoteLane::LaneS => "mun2.ogg",
            NoteLane::LaneD => "mun3.ogg",
            NoteLane::LaneF => "mun4.ogg",
            NoteLane::LaneJ => "mun5.ogg",
            NoteLane::LaneK => "mun6.ogg",
            NoteLane::LaneL => "mun7.ogg",
            NoteLane::LaneSemicolon => "mun8.ogg",
        });

        commands.spawn(AudioBundle {
            source: mun,
            ..default()
        });
    }
}

#[derive(Event)]
struct LaneTapEvent(NoteLane);

fn lane_tap(
    mut ev_lane_tap: EventReader<LaneTapEvent>,
    mut commands: Commands,
    frames: Res<FramesCount>,
    mut note_query: Query<(Entity, &NoteId), With<NoteTag>>,
    mut ev_play_mun: EventWriter<MunIdEvent>,
) {
    for ev in ev_lane_tap.read() {
        for (entity, note_id) in note_query.iter_mut() {
            if frames.count >= note_id.timing - 10
                && frames.count <= note_id.timing + 10
                && ev.0 == note_id.lane
            {
                commands.entity(entity).despawn();
                ev_play_mun.send(MunIdEvent(ev.0));
            }
        }
    }
}

#[derive(Event)]
struct PauseEvent;

fn pause(
    mut ev_pause: EventReader<PauseEvent>,
    state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
) {
    for _ev in ev_pause.read() {
        match state.get() {
            PauseState::Unpaused => next_state.set(PauseState::Paused),
            PauseState::Paused => next_state.set(PauseState::Unpaused),
        }
    }
}
