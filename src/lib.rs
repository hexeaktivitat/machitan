use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use clap::Parser;

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

        // plugins
        app.add_plugins((PlayerPlugin, ConsolePlugin));

        // console comands
        app.add_console_command::<HelpCommand, _>(help_command);
    }
}

// console commands

#[derive(Parser, ConsoleCommand)]
#[command(name = "echo")]
struct HelpCommand {
    msg: String,
}

fn help_command(mut log: ConsoleCommand<HelpCommand>) {
    if let Some(Ok(HelpCommand { msg })) = log.take() {
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
