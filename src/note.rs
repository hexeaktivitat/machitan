use bevy::prelude::*;

use crate::FramesCount;

pub struct NotePlugin;

impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_note, animate_note))
            .insert_resource(Beatmap {
                notes: SAMPLE_BEATMAP.into(),
            });
    }
}

#[derive(Component)]
struct NoteTag;

#[derive(Component, Clone)]
struct NoteId {
    timing: usize,
    lane: NoteLane,
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

#[derive(Component, Clone)]
enum NoteLane {
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

fn spawn_note(
    mut commands: Commands,
    server: Res<AssetServer>,
    frames: Res<FramesCount>,
    mut beatmap: ResMut<Beatmap>,
) {
    if !beatmap.notes.is_empty() {
        let note_sprite: Handle<Image> = server.load("sq_note.png");
        let transform = lane_transforms(&beatmap.notes.first().unwrap().lane);
        match &mut *beatmap.notes {
            [head, tail @ ..] => {
                if frames.count >= head.timing {
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
        let translate = 250. * time.delta_seconds();

        if position.translation.y <= LANE_VERT_POS {
            position.translation.y = LANE_VERT_POS;
        } else {
            position.translation.y -= translate;
        }
    }
}

fn lane_transforms(lane: &NoteLane) -> Transform {
    match lane {
        NoteLane::LaneA => Transform::from_xyz(LANE_A_POS, -LANE_VERT_POS, 100.),
        NoteLane::LaneS => Transform::from_xyz(LANE_S_POS, -LANE_VERT_POS, 100.),
        NoteLane::LaneD => Transform::from_xyz(LANE_D_POS, -LANE_VERT_POS, 100.),
        NoteLane::LaneF => Transform::from_xyz(LANE_F_POS, -LANE_VERT_POS, 100.),
        NoteLane::LaneJ => Transform::from_xyz(LANE_J_POS, -LANE_VERT_POS, 100.),
        NoteLane::LaneK => Transform::from_xyz(LANE_K_POS, -LANE_VERT_POS, 100.),
        NoteLane::LaneL => Transform::from_xyz(LANE_L_POS, -LANE_VERT_POS, 100.),
        NoteLane::LaneSemicolon => Transform::from_xyz(LANE_SEMI_POS, -LANE_VERT_POS, 100.),
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
