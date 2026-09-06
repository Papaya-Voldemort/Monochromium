#![allow(warnings)]
mod commands;
mod config;
mod database;
mod utils;

use crate::commands::{add, check_in, delete, edit, list, reminder, search, view};
use crate::database::make_db;
use clap::Parser;
use commands::definitions::{MonoCLI, MonoCommands};
use crate::config::setup_zshrc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let db = make_db().await;
    let conn = db.connect().unwrap();
    let prompt_setup = setup_zshrc().await;

    let cli = MonoCLI::parse();

    match cli.command {
        MonoCommands::Add {
            text,
            note_type,
            time,
            date,
            paste,
        } => {
            let output = add(conn.clone(), text, note_type, time, date, paste).await;
            match output {
                Err(err) => eprintln!("Database Error {:?}", err),
                Ok(String) => {
                    println!("{}", String)
                }
            }
        }
        MonoCommands::CheckIn {
            text,
            time,
            date,
            paste,
        } => {
            let output = check_in(conn.clone(), text, time, date, paste).await;
            println!("{}", output);
        }
        MonoCommands::Delete { note_id, approve } => {
            let output = delete(conn.clone(), note_id, approve).await;
            println!("{}", output.unwrap());
        }
        MonoCommands::Edit {
            note_id,
            headless,
            append,
            overwrite,
            text,
        } => {
            let output = edit(conn, note_id, headless, append, overwrite, text).await;
            match output {
                Err(err) => {}
                Ok(String) => {
                    if append {
                        println!(
                            "Appended \"{}\" to the end of \"{}\"!",
                            String.old, String.new
                        )
                    } else if overwrite {
                        println!("Replaced \"{}\" with \"{}\"!", String.new, String.old)
                    } else {
                        println!("Note updated successfully!")
                    }
                }
            }
        }
        MonoCommands::List {
            limit,
            note_type,
            today,
            since,
            view,
        } => {
            let output = list(conn, limit, note_type, today, since, view).await;
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
            let list = search(conn, text, limit, note_type, date).await;
            match list {
                Ok(list) => {
                    for item in list {
                        println!("{}", item)
                    }
                }
                Err(err) => eprintln!("Search error: {}", err),
            }
        }
        MonoCommands::View { note_id, no_format } => {
            let output = view(conn, note_id, no_format).await;
            println!("{}", output)
        }
        MonoCommands::Reminder {} => {
            reminder(conn).await
        }
    }
}
