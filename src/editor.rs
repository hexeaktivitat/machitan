use std::{fs::File, io::Write};

use bevy::{input::keyboard::KeyCode, prelude::*};
use bevy_console::{AddConsoleCommand, ConsoleCommand};
use clap::Parser;
use serde::Serialize;

use crate::{
    note::{NoteId, NoteLane},
    FramesCount,
};

pub struct EditorPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EditorSet;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, record_key_presses.in_set(EditorSet))
            .insert_resource(BeatmapRecord { notes: vec![] })
            .add_console_command::<SaveRecordingCommand, _>(
                save_recording_command.in_set(EditorSet),
            )
            .add_console_command::<RecordCommand, _>(record_command);
    }
}

#[derive(Resource, Serialize)]
struct BeatmapRecord {
    notes: Vec<NoteId>,
}

fn record_key_presses(
    keys: Res<ButtonInput<KeyCode>>,
    frames: Res<FramesCount>,
    mut beatmap_record: ResMut<BeatmapRecord>,
) {
    for key in keys.get_just_pressed() {
        if key == &KeyCode::KeyA {
            beatmap_record.notes.push(NoteId {
                timing: frames.count,
                lane: NoteLane::LaneA,
            });
        }
        if key == &KeyCode::KeyS {
            beatmap_record.notes.push(NoteId {
                timing: frames.count,
                lane: NoteLane::LaneS,
            });
        }
        if key == &KeyCode::KeyD {
            beatmap_record.notes.push(NoteId {
                timing: frames.count,
                lane: NoteLane::LaneD,
            });
        }
        if key == &KeyCode::KeyF {
            beatmap_record.notes.push(NoteId {
                timing: frames.count,
                lane: NoteLane::LaneF,
            });
        }
        if key == &KeyCode::KeyJ {
            beatmap_record.notes.push(NoteId {
                timing: frames.count,
                lane: NoteLane::LaneJ,
            });
        }
        if key == &KeyCode::KeyK {
            beatmap_record.notes.push(NoteId {
                timing: frames.count,
                lane: NoteLane::LaneK,
            });
        }
        if key == &KeyCode::KeyL {
            beatmap_record.notes.push(NoteId {
                timing: frames.count,
                lane: NoteLane::LaneL,
            });
        }
        if key == &KeyCode::Semicolon {
            beatmap_record.notes.push(NoteId {
                timing: frames.count,
                lane: NoteLane::LaneSemicolon,
            });
        }
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "save-record")]
struct SaveRecordingCommand {
    dest: String,
}

fn save_recording_command(
    mut log: ConsoleCommand<SaveRecordingCommand>,
    mut beatmap_record: Res<BeatmapRecord>,
) {
    if let Some(Ok(SaveRecordingCommand { dest })) = log.take() {
        let mut file = File::create(&dest).unwrap();
        let toml_beatmap = toml::to_string(beatmap_record.as_ref()).unwrap();
        file.write_all(toml_beatmap.as_bytes()).unwrap();
        log.reply(format!("Saved current beatmap as {}", dest));
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "record")]
struct RecordCommand;

fn record_command(
    mut commands: Commands,
    mut log: ConsoleCommand<RecordCommand>,
    mut frames: ResMut<FramesCount>,
    server: Res<AssetServer>,
) {
    if let Some(Ok(RecordCommand)) = log.take() {
        commands.spawn(AudioBundle {
            source: server.load("make_debut_TV_size.ogg"),
            ..default()
        });

        // reinitialize framecount to 0
        // log.reply(format!("orig framecount: {}", frames.count));
        frames.count = 0;
    }
}
