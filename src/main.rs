// #![allow(warnings)]
mod commands;
mod config;
mod crypto;
mod database;
mod storage;
mod types;
mod utils;

use crate::commands::{
    add, config, delete, edit, export, get_stats, init, list, lock, reminder, search, unlock, view,
};
use crate::database::Database;
use crate::types::{CommandOutput, MonoError, NoteTypes};
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
    let cli = MonoCLI::parse();

    let db = Database::new()?;

    let copy_output = cli.copy;

    let output = match cli.command {
        Some(cmd) => match cmd {
            MonoCommands::Add {
                text,
                note_type,
                time,
                date,
                paste,
                value,
            } => CommandOutput::Text(add(&db, text, note_type, time, date, paste, value)?),
            MonoCommands::Checkin {
                text,
                time,
                date,
                paste,
                mood,
            } => CommandOutput::Text(add(
                &db,
                text,
                Some(NoteTypes::CheckIn),
                time,
                date,
                paste,
                mood,
            )?),
            MonoCommands::Todo {
                text,
                time,
                date,
                paste,
                priority,
            } => CommandOutput::Text(add(
                &db,
                text,
                Some(NoteTypes::Todo),
                time,
                date,
                paste,
                priority,
            )?),
            MonoCommands::Idea {
                text,
                time,
                date,
                paste,
                hype,
            } => CommandOutput::Text(add(
                &db,
                text,
                Some(NoteTypes::Idea),
                time,
                date,
                paste,
                hype,
            )?),
            MonoCommands::Delete { note_id, approve } => {
                CommandOutput::Text(delete(&db, note_id, approve)?)
            }
            MonoCommands::Edit {
                note_id,
                edit_mode,
                change: text,
            } => CommandOutput::Text(edit(&db, note_id, edit_mode, text)?.output),

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
            MonoCommands::View {
                note_id,
                no_format,
                verbose,
            } => CommandOutput::Text(view(&db, note_id, no_format, verbose)?),
            MonoCommands::Stats { short } => CommandOutput::Raw(get_stats(&db, short)?),

            MonoCommands::Export {} => CommandOutput::Lines(export(&db)?),

            MonoCommands::Reminder {} => CommandOutput::Raw(reminder(&db)?),
            MonoCommands::Init {} => CommandOutput::Text(init()?),
            MonoCommands::Lock { note_id } => CommandOutput::Text(lock(&db, note_id)?),
            MonoCommands::Unlock { note_id } => CommandOutput::Text(unlock(&db, note_id)?),
            MonoCommands::Config {} => CommandOutput::Text(config()?),
        },
        None => {
            let logo = r#"
 █▀▄▀█ █▀█ █▄ █ █▀█ █▀▀ █ █ █▀█ █▀█ █▀▄▀█ █ █ █ █▀▄▀█
 █ ▀ █ █▄█ █ ▀█ █▄█ █▄▄ █▀█ █▀▄ █▄█ █ ▀ █ █ █▄█ █ ▀ █
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
