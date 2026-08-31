mod commands;
mod config;
mod database;
mod utils;

use crate::commands::{add, check_in, delete, edit, list, search, view};
use crate::database::make_db;
use clap::Parser;
use commands::definitions::MonoCommands;

#[derive(Parser)]
#[clap(author, version, about)]
struct MonoCLI {
    #[command(subcommand)]
    command: MonoCommands,

    #[clap(short, long, global = true)]
    copy: bool,

    #[clap(short, long)]
    animate: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let db = make_db().await;

    let cli = MonoCLI::parse();

    match cli.command {
        MonoCommands::Add {
            text,
            note_type,
            time,
            date,
            paste,
        } => {
            let output = add(db, text, note_type, time, date, paste).await;
            println!("{}", output)
        }
        MonoCommands::CheckIn {
            text,
            time,
            date,
            paste,
        } => {
            let output = check_in(db, text, time, date, paste).await;
            println!("{}", output)
        }
        MonoCommands::Delete { note_id, approve } => {
            delete(note_id, approve);
        }
        MonoCommands::Edit {
            note_id,
            headless,
            append,
            overwrite,
        } => {
            edit(note_id, headless, append, overwrite);
        }
        MonoCommands::List {
            limit,
            note_type,
            today,
            since,
            view,
        } => {
            let output = list(db, limit, note_type, today, since, view).await;
            match output {
                Err(err) => eprintln!("Database Error {:?}", err),
                Ok(vector_data) => {
                    for note in &vector_data {
                        println!("Note: {}", note);
                    }
                }
            }
        }
        MonoCommands::Search {
            text,
            limit,
            note_type,
            date,
        } => {
            search(text, limit, note_type, date);
        }
        MonoCommands::View { note_id, no_format } => {
            view(note_id, no_format);
        }
    }
}
