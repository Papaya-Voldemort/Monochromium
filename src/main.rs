// #![allow(warnings)]
mod commands;
mod config;
mod database;
mod types;
mod utils;

use crate::commands::{add, delete, edit, export, init, list, reminder, search, view};
use crate::config::load_config;
use crate::database::Database;
use crate::types::{CommandOutput, EditMode, MonoError};
use crate::utils::copy;
use clap::{CommandFactory, Parser};
use types::{MonoCLI, MonoCommands};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), MonoError> {
    let db = Database::new();
    let config = load_config()?;

    let cli = MonoCLI::parse();

    let copy_output = cli.copy;

    let output = match cli.command {
        Some(cmd) => match cmd {
            MonoCommands::Add {
                text,
                note_type,
                time,
                date,
                paste,
            } => CommandOutput::Text(add(&db, text, note_type, time, date, paste)?),
            MonoCommands::Delete { note_id, approve } => {
                CommandOutput::Text(delete(&db, note_id, approve)?)
            }
            MonoCommands::Edit {
                note_id,
                headless,
                edit_mode,
                text,
            } => {
                let output = edit(&db, note_id, headless, edit_mode, text)?;

                let text = match output.update_type {
                    EditMode::Append => {
                        format!(
                            "Appended \"{}\" to the end of \"{}\"!",
                            output.new, output.old
                        )
                    }

                    EditMode::Overwrite => {
                        format!("Replaced \"{}\" with \"{}\"!", output.old, output.new)
                    }
                };

                CommandOutput::Text(text)
            }

            MonoCommands::List {
                limit,
                note_type,
                today,
                since,
                view,
            } => CommandOutput::Lines(list(&db, limit, note_type, today, since, view)?),
            MonoCommands::Search {
                text,
                limit,
                note_type,
                date,
            } => CommandOutput::Lines(search(&db, text, limit, note_type, date)?),
            MonoCommands::View { note_id, no_format } => {
                CommandOutput::Text(view(&db, note_id, no_format)?)
            }

            MonoCommands::Export {} => CommandOutput::Lines(export(&db)?),

            MonoCommands::Reminder {} => CommandOutput::Text(reminder(&db, config)?),
            MonoCommands::Init {} => {
                init();

                CommandOutput::Text("Initialized Monochromium configuration.".to_string())
            }
        },
        None => {
            let logo = r#"
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢰⣶⣦⠀⢀⣶⣶⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣶⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠶⠆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢸⣿⢻⡄⣸⢿⣿⠀⣴⡿⠻⣷⡄⢸⣷⠞⢿⣦⠀⣴⡿⠻⣷⡄⢠⣾⠛⢷⡆⢸⣿⠞⢿⣦⠀⣿⡶⠟⢠⣾⠟⢿⣦⠀⣿⡶⠻⣷⠞⢿⣦⠀⣿⡇⢸⣷⠀⢸⣿⠀⣿⡶⠻⣷⠞⢿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢸⣿⠸⣧⡿⢸⣿⠀⣿⡄⠀⣼⡇⢸⣿⠀⢸⣿⠀⣿⡄⠀⣼⡇⢸⣇⠀⣀⡀⢸⣿⠀⢸⣿⠀⣿⡇⠀⢸⣧⠀⢠⣿⠀⣿⡇⠀⣿⠀⢸⣿⠀⣿⡇⢸⣿⠀⢸⣿⠀⣿⡇⠀⣿⠀⢸⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠘⠛⠀⠛⠃⠘⠛⠀⠈⠛⠛⠋⠀⠘⠛⠀⠘⠛⠀⠈⠛⠛⠋⠀⠈⠛⠛⠛⠁⠘⠛⠀⠘⠛⠀⠛⠃⠀⠀⠙⠛⠛⠁⠀⠛⠃⠀⠛⠀⠘⠛⠀⠛⠃⠀⠛⠛⠙⠛⠀⠛⠃⠀⠛⠀⠘⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"#;

            let mut cmd = MonoCLI::command();
            let help_text = cmd.render_help().to_string();

            CommandOutput::Text(format!("{logo}\n{help_text}"))
        }
    };

    output.print();

    if copy_output && let Err(err) = copy(output.as_text()) {
        eprintln!("Warning: could not copy to clipboard: {err}");
    }

    Ok(())
}
