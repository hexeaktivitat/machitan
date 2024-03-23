use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use clap::Parser;

use player::PlayerPlugin;

mod player;

pub struct MachitanPlugin;

impl Plugin for MachitanPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, ConsolePlugin))
            .add_console_command::<HelpCommand, _>(help_command);
    }
}

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
