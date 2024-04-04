use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::view::PostProcessWrite};

use crate::FramesCount;

pub struct NotePlugin;

impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_note_frames))
            .add_systems(Update, (spawn_note, animate_note))
            .insert_resource(Beatmap {
                notes: SAMPLE_BEATMAP.into(),
            });
    }
}

#[derive(Component)]
pub struct NoteTag;

#[derive(Component, Clone)]
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

#[derive(Component, Clone, PartialEq, Eq, Copy)]
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
                if frames.count >= head.timing - 50 {
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

fn animate_note(time: Res<Time>, mut query: Query<(&mut Transform, Entity), With<NoteTag>>) {
    for (mut position, _entity) in query.iter_mut() {
        let translate = 500. * time.delta_seconds();

        if position.translation.y <= LANE_VERT_POS {
            position.translation.y = LANE_VERT_POS;
        } else {
            position.translation.y -= translate;
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

#[derive(Resource)]
struct Beatmap {
    notes: Vec<NoteId>,
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
