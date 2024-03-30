use bevy::{prelude::*, time::Stopwatch};
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use clap::Parser;

use note::NotePlugin;
use player::PlayerPlugin;

mod note;
mod player;

pub struct MachitanPlugin;

impl Plugin for MachitanPlugin {
    fn build(&self, app: &mut App) {
        // state setup
        app.insert_state(ApplicationState::InGame)
            .init_state::<ModeState>()
            .init_state::<PauseState>();

        // resources
        app.insert_resource(FramesCount { count: 0 });

        // plugins
        app.add_plugins((PlayerPlugin, ConsolePlugin, NotePlugin));

        // systems
        app.add_systems(Startup, (start_timer));
        app.add_systems(Update, (update_framecount));

        // console comands
        app.add_console_command::<EchoCommand, _>(echo_command);
    }
}

// logical frame timer
#[derive(Component)]
struct FrameTime {
    timer: Stopwatch,
}

// global framecount resource
#[derive(Resource)]
pub struct FramesCount {
    count: usize,
}

fn start_timer(mut commands: Commands, time: Res<Time>) {
    let mut timer = Stopwatch::new();
    timer.tick(time.delta());
    commands.spawn(FrameTime { timer });
}

fn update_framecount(
    time: Res<Time>,
    mut query: Query<&mut FrameTime>,
    mut frame_count: ResMut<FramesCount>,
) {
    let mut frame_time = query.single_mut();

    frame_time.timer.tick(time.delta());

    if frame_time.timer.elapsed_secs_f64() >= 1. / 60. {
        frame_time.timer.reset();
        frame_count.count += 1;
    }
}

// console commands

#[derive(Parser, ConsoleCommand)]
#[command(name = "echo")]
struct EchoCommand {
    msg: String,
}

fn echo_command(mut log: ConsoleCommand<EchoCommand>) {
    if let Some(Ok(EchoCommand { msg })) = log.take() {
        log.reply(msg);
    }
}

// game states

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApplicationState {
    Loading,
    Menu,
    InGame,
}

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ModeState {
    NotInGame,
    #[default]
    Singleplayer,
}

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum PauseState {
    #[default]
    Unpaused,
    Paused,
}
