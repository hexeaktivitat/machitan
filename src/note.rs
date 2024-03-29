use bevy::prelude::*;

pub struct NotePlugin;

impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, build_note);
    }
}

#[derive(Component)]
struct NoteTag;

#[derive(Component)]
struct NoteId {
    num: usize,
    timing: NoteTiming,
    lane: NoteLane,
}

// lane consts
// notes are 128 px wide + 16px boundary
// Y positions
const LANE_VERT_POS: f32 = -275.0; // Y position of lanes

// X positions
const LANE_A_POS: f32 = -525.0;
const LANE_S_POS: f32 = -381.0;

#[derive(Component)]
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

#[derive(Component)]
struct NoteTiming {
    frame: usize,
}

#[derive(Bundle)]
struct Note {
    tag: NoteTag,
    id: NoteId,
    sprite: SpriteBundle,
}

fn build_note(mut commands: Commands, server: Res<AssetServer>) {
    let note_sprite: Handle<Image> = server.load("sq_note.png");
    let note = Note {
        tag: NoteTag,
        id: NoteId {
            num: 1,
            timing: NoteTiming { frame: 20 },
            lane: NoteLane::LaneA,
        },
        sprite: SpriteBundle {
            texture: note_sprite.clone(),
            transform: Transform::from_xyz(LANE_A_POS, LANE_VERT_POS, 100.),
            ..default()
        },
    };

    commands.spawn(note);

    let note_2 = Note {
        tag: NoteTag,
        id: NoteId {
            num: 2,
            timing: NoteTiming { frame: 25 },
            lane: NoteLane::LaneS,
        },
        sprite: SpriteBundle {
            texture: note_sprite,
            transform: Transform::from_xyz(LANE_S_POS, LANE_VERT_POS, 100.),
            ..default()
        },
    };

    commands.spawn(note_2);
}
