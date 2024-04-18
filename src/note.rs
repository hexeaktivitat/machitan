use std::fs::File;
use std::io::prelude::*;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::view::PostProcessWrite};
use bevy_console::{AddConsoleCommand, ConsoleCommand};
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::FramesCount;

pub struct NotePlugin;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NoteSet;

impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_note_frames).in_set(NoteSet))
            .add_systems(FixedUpdate, (spawn_note, animate_note).in_set(NoteSet))
            .insert_resource(Beatmap {
                notes: SAMPLE_BEATMAP.into(),
            })
            .add_console_command::<SaveCommand, _>(save_command)
            .add_console_command::<ReloadCommand, _>(reload_command)
            .add_console_command::<LoadCommand, _>(load_command);
    }
}

#[derive(Component)]
pub struct NoteTag;

#[derive(Component, Clone, Deserialize, Serialize, Debug)]
pub struct NoteId {
    pub timing: usize,
    pub lane: NoteLane,
}

// lane consts
// notes are 128 px wide + 16px boundary
// Y positions
const LANE_VERT_POS: f32 = -275.0; // Y position of lanes

// X positions
const LANE_A_POS: f32 = -525.0;
const LANE_S_POS: f32 = -381.0;
const LANE_D_POS: f32 = -237.0;
const LANE_F_POS: f32 = -93.0;
const LANE_J_POS: f32 = 93.0;
const LANE_K_POS: f32 = 237.0;
const LANE_L_POS: f32 = 381.0;
const LANE_SEMI_POS: f32 = 525.0;

#[derive(Component, Clone, PartialEq, Eq, Copy, Serialize, Deserialize, Debug)]
pub enum NoteLane {
    LaneA,
    LaneS,
    LaneD,
    LaneF,
    LaneJ,
    LaneK,
    LaneL,
    LaneSemicolon,
}

#[derive(Bundle)]
struct Note {
    tag: NoteTag,
    id: NoteId,
    sprite: SpriteBundle,
}

#[derive(Component)]
struct FrameTag;

#[derive(Bundle)]
struct Frame {
    tag: FrameTag,
    lane: NoteLane,
    sprite: SpriteBundle,
}

const LANES: [NoteLane; 8] = [
    NoteLane::LaneA,
    NoteLane::LaneS,
    NoteLane::LaneD,
    NoteLane::LaneF,
    NoteLane::LaneJ,
    NoteLane::LaneK,
    NoteLane::LaneL,
    NoteLane::LaneSemicolon,
];

const FRAMES_TO_TIMING: usize = 56;

fn spawn_note_frames(mut commands: Commands, server: Res<AssetServer>) {
    for lane in LANES.iter() {
        let frame_sprite: Handle<Image> = server.load("note_frame.png");

        let transform = lane_transforms(lane, Some(LANE_VERT_POS));
        let frame = Frame {
            tag: FrameTag,
            lane: lane.to_owned(),
            sprite: SpriteBundle {
                texture: frame_sprite,
                transform,
                ..default()
            },
        };

        commands.spawn(frame);
    }
}

fn spawn_note(
    mut commands: Commands,
    server: Res<AssetServer>,
    frames: Res<FramesCount>,
    mut beatmap: ResMut<Beatmap>,
) {
    if !beatmap.notes.is_empty() {
        let note_sprite: Handle<Image> = server.load("sq_note.png");
        let transform = lane_transforms(&beatmap.notes.first().unwrap().lane, None);
        match &mut *beatmap.notes {
            [head, tail @ ..] => {
                if frames.count >= head.timing - FRAMES_TO_TIMING {
                    let note = Note {
                        tag: NoteTag,
                        id: head.clone(),
                        sprite: SpriteBundle {
                            texture: note_sprite,
                            transform,
                            ..default()
                        },
                    };

                    commands.spawn(note);

                    beatmap.notes = tail.into();
                }
            }
            _ => unreachable!(),
        }
    }
}

fn animate_note(
    mut commands: Commands,
    time: Res<Time>,
    frames: Res<FramesCount>,
    mut query: Query<(&mut Transform, Entity, &NoteId), With<NoteTag>>,
) {
    for (mut position, entity, note_id) in query.iter_mut() {
        let translate = 600. * time.delta_seconds();

        if position.translation.y <= LANE_VERT_POS {
            position.translation.y = LANE_VERT_POS;
            println!("{}", frames.count);
            // position.translation.y -= translate;
        } else {
            position.translation.y -= translate;
        }

        if frames.count > note_id.timing + 10 {
            println!("despawn  at {} for {:?}", frames.count, &note_id);
            commands.entity(entity).despawn();
        }
    }
}

fn lane_transforms(lane: &NoteLane, y_pos: Option<f32>) -> Transform {
    let y = if let Some(pos) = y_pos {
        pos
    } else {
        -LANE_VERT_POS
    };

    match lane {
        NoteLane::LaneA => Transform::from_xyz(LANE_A_POS, y, 100.),
        NoteLane::LaneS => Transform::from_xyz(LANE_S_POS, y, 100.),
        NoteLane::LaneD => Transform::from_xyz(LANE_D_POS, y, 100.),
        NoteLane::LaneF => Transform::from_xyz(LANE_F_POS, y, 100.),
        NoteLane::LaneJ => Transform::from_xyz(LANE_J_POS, y, 100.),
        NoteLane::LaneK => Transform::from_xyz(LANE_K_POS, y, 100.),
        NoteLane::LaneL => Transform::from_xyz(LANE_L_POS, y, 100.),
        NoteLane::LaneSemicolon => Transform::from_xyz(LANE_SEMI_POS, y, 100.),
    }
}

#[derive(ConsoleCommand, Parser)]
#[command(name = "save")]
struct SaveCommand {
    dest: String,
}

fn save_command(mut log: ConsoleCommand<SaveCommand>, beatmap: Res<Beatmap>) {
    if let Some(Ok(SaveCommand { dest })) = log.take() {
        let mut file = File::create(&dest).unwrap();
        // let toml_beatmap = toml::to_string(beatmap.as_ref()).unwrap();
        let toml_beatmap = toml::to_string(&Beatmap {
            notes: SAMPLE_BEATMAP.into(),
        })
        .unwrap();
        file.write_all(toml_beatmap.as_bytes()).unwrap();
        log.reply(format!("Saved current beatmap as {}", dest));
    }
}

#[derive(ConsoleCommand, Parser)]
#[command(name = "reload")]
struct ReloadCommand;

fn reload_command(
    mut log: ConsoleCommand<ReloadCommand>,
    mut beatmap: ResMut<Beatmap>,
    mut frames: ResMut<FramesCount>,
) {
    if let Some(Ok(ReloadCommand)) = log.take() {
        beatmap.notes = SAMPLE_BEATMAP.into();
        frames.count = 0;
        log.reply("Reloaded!");
    }
}

#[derive(ConsoleCommand, Parser)]
#[command(name = "load")]
struct LoadCommand {
    file: String,
    song: String,
}

fn load_command(
    mut commands: Commands,
    mut log: ConsoleCommand<LoadCommand>,
    mut beatmap: ResMut<Beatmap>,
    mut frames: ResMut<FramesCount>,
    server: Res<AssetServer>,
) {
    if let Some(Ok(LoadCommand { file, song })) = log.take() {
        let mut load_file = File::open(file).unwrap();
        let mut load_beatmap = String::new();
        load_file.read_to_string(&mut load_beatmap).unwrap();
        let new_beatmap: Beatmap = toml::from_str(&load_beatmap).unwrap();

        let new_song: Handle<AudioSource> = server.load(song);

        commands.spawn(AudioBundle {
            source: new_song,
            ..default()
        });
        beatmap.notes = new_beatmap.notes;
        frames.count = 0;
    }
}

#[derive(Resource, Serialize, Deserialize)]
pub struct Beatmap {
    // filename: String,
    pub notes: Vec<NoteId>,
}

const SAMPLE_BEATMAP: [NoteId; 8] = [
    NoteId {
        timing: 200,
        lane: NoteLane::LaneA,
    },
    NoteId {
        timing: 250,
        lane: NoteLane::LaneS,
    },
    NoteId {
        timing: 300,
        lane: NoteLane::LaneD,
    },
    NoteId {
        timing: 350,
        lane: NoteLane::LaneF,
    },
    NoteId {
        timing: 400,
        lane: NoteLane::LaneJ,
    },
    NoteId {
        timing: 450,
        lane: NoteLane::LaneK,
    },
    NoteId {
        timing: 500,
        lane: NoteLane::LaneL,
    },
    NoteId {
        timing: 550,
        lane: NoteLane::LaneSemicolon,
    },
];
