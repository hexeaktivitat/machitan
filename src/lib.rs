use bevy::{prelude::*, time::Stopwatch};
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use clap::Parser;

use editor::{EditorPlugin, EditorSet};
use note::{NotePlugin, NoteSet};
use player::{Pause, PlayerPlugin, PlayerSet};
use ui::{UiPlugin, UiSet};

mod editor;
mod note;
mod player;
mod ui;

pub struct MachitanPlugin;

impl Plugin for MachitanPlugin {
    fn build(&self, app: &mut App) {
        // state setup
        app.insert_state(ApplicationState::Menu)
            .init_state::<ModeState>()
            .init_state::<PauseState>();

        app.configure_sets(
            Startup,
            (
                PlayerSet
                    .run_if(in_state(ApplicationState::Menu))
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(ApplicationState::Editor)),
                UiSet.run_if(in_state(ApplicationState::Menu)),
            ),
        );
        app.configure_sets(
            Update,
            PlayerSet
                .run_if(in_state(ApplicationState::Menu))
                .run_if(in_state(ApplicationState::InGame))
                .run_if(in_state(ApplicationState::Editor)),
        );
        app.configure_sets(
            FixedUpdate,
            (
                NoteSet
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(PauseState::Unpaused)),
                EditorSet.run_if(in_state(ApplicationState::Editor)),
                UiSet.run_if(in_state(ApplicationState::Menu)),
            ),
        );

        // resources
        app.insert_resource(FramesCount { count: 0 })
            .insert_resource(Time::<Fixed>::from_hz(60.0));

        // plugins
        app.add_plugins((
            PlayerPlugin,
            ConsolePlugin,
            NotePlugin,
            EditorPlugin,
            UiPlugin,
        ));

        // systems
        app.add_systems(
            FixedPreUpdate,
            (update_framecount).run_if(in_state(ApplicationState::InGame)),
        );

        // console comands
        app.add_console_command::<EchoCommand, _>(echo_command);
    }
}

// global framecount resource
#[derive(Resource)]
pub struct FramesCount {
    pub count: usize,
}

fn update_framecount(time: Res<Time>, mut frame_count: ResMut<FramesCount>) {
    frame_count.count += 1;
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
    Editor,
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
