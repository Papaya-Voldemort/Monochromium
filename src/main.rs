// #![allow(warnings)]
mod commands;
mod config;
mod database;
mod utils;

use crate::commands::{add, check_in, delete, edit, export, init, list, reminder, search, view};
use crate::config::setup_zshrc;
use crate::database::make_db;
use crate::utils::copy;
use clap::{CommandFactory, Parser};
use commands::definitions::{MonoCLI, MonoCommands};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let db = make_db().await;
    let conn = db.connect().unwrap();
    let _prompt_setup = setup_zshrc().await;

    let cli = MonoCLI::parse();

    match cli.command {
        Some(cmd) => match cmd {
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
                    Ok(string) => {
                        println!("{}", string);
                        if cli.copy {
                            copy(string);
                        };
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
                match output {
                    Ok(output) => {
                        println!("{}", output);
                    }
                    Err(err) => {
                        println!("{}", err)
                    }
                }
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
                    Err(err) => {
                        println!("{}", err)
                    }
                    Ok(string) => {
                        if append {
                            println!(
                                "Appended \"{}\" to the end of \"{}\"!",
                                string.old, string.new
                            )
                        } else if overwrite {
                            println!("Replaced \"{}\" with \"{}\"!", string.new, string.old)
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
                let list = list(conn, limit, note_type, today, since, view).await;
                match list {
                    Ok(list) => {
                        for item in list {
                            println!("{}", item)
                        }
                    }
                    Err(err) => eprintln!("Search error: {}", err),
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
            MonoCommands::Export {} => {
                let output = export(conn).await;
                match output {
                    Ok(output) => {
                        let mut copied = String::new();
                        for item in output {
                            println!("{}", item);
                            copied.push_str(&item);
                            copied.push('\n');
                        }
                        if cli.copy {
                            copy(copied)
                        }
                    }
                    Err(err) => {
                        eprintln!("Search error: {}", err)
                    }
                }
            }
            MonoCommands::Reminder {} => {
                let result = reminder(conn).await;
                match result {
                    Ok(output) => println!("{}", output),
                    Err(err) => eprintln!("We hit a speed bump! Try again. Error: {}", err),
                }
            }
            MonoCommands::Init {} => {
                init().await;
            }
        },
        None => {
            let logo = r#"
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢰⣶⣦⠀⢀⣶⣶⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣶⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠶⠆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢸⣿⢻⡄⣸⢿⣿⠀⣴⡿⠻⣷⡄⢸⣷⠞⢿⣦⠀⣴⡿⠻⣷⡄⢠⣾⠛⢷⡆⢸⣿⠞⢿⣦⠀⣿⡶⠟⢠⣾⠟⢿⣦⠀⣿⡶⠻⣷⠞⢿⣦⠀⣿⡇⢸⣷⠀⢸⣿⠀⣿⡶⠻⣷⠞⢿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢸⣿⠸⣧⡿⢸⣿⠀⣿⡄⠀⣼⡇⢸⣿⠀⢸⣿⠀⣿⡄⠀⣼⡇⢸⣇⠀⣀⡀⢸⣿⠀⢸⣿⠀⣿⡇⠀⢸⣧⠀⢠⣿⠀⣿⡇⠀⣿⠀⢸⣿⠀⣿⡇⢸⣿⠀⢸⣿⠀⣿⡇⠀⣿⠀⢸⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠘⠛⠀⠛⠃⠘⠛⠀⠈⠛⠛⠋⠀⠘⠛⠀⠘⠛⠀⠈⠛⠛⠋⠀⠈⠛⠛⠛⠁⠘⠛⠀⠘⠛⠀⠛⠃⠀⠀⠙⠛⠛⠁⠀⠛⠃⠀⠛⠀⠘⠛⠀⠛⠃⠀⠛⠛⠙⠛⠀⠛⠃⠀⠛⠀⠘⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
            "#;

            if cli.animate {}

            println!("{}", logo);

            let mut cmd = MonoCLI::command();
            let help_text = cmd.render_help().to_string();

            println!("{}", help_text);

            if cli.copy {
                let full_output = format!("{}\n{}", logo, help_text);
                copy(full_output);
            }
        }
    }
}
