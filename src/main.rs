// #![allow(warnings)]
mod commands;
mod config;
mod database;
mod types;
mod utils;

use crate::commands::{
    add, check_in, delete, edit, export, init, list, list_todos, reminder, search, view,
};
use crate::config::{create_config, load_config};
use crate::database::Database;
use crate::utils::copy;
use clap::{CommandFactory, Parser};
use types::{MonoCLI, MonoCommands};

fn main() {
    let db = Database::new();
    create_config();
    let config = load_config();

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
                let output = add(&db, text, note_type, time, date, paste);
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
                let output = check_in(&db, text, time, date, paste);
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
                let output = delete(&db, note_id, approve);
                match output {
                    Ok(msg) => println!("{}", msg),
                    Err(err) => eprintln!("{}", err),
                }
            }
            MonoCommands::Edit {
                note_id,
                headless,
                append,
                overwrite,
                text,
            } => {
                let output = edit(&db, note_id, headless, append, overwrite, text);
                match output {
                    Err(err) => {
                        println!("{}", err)
                    }
                    Ok(string) => {
                        if append {
                            println!(
                                "Appended \"{}\" to the end of \"{}\"!",
                                string.new, string.old
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
                let list = list(&db, limit, note_type, today, since, view);
                match list {
                    Ok(list) => {
                        for item in list {
                            println!("{}", item)
                        }
                    }
                    Err(err) => eprintln!("Search error: {}", err),
                }
            }
            MonoCommands::TodoList {
                limit,
                today,
                since,
                view,
            } => {
                let list = list_todos(&db, limit, today, since, view);
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
                let list = search(&db, text, limit, note_type, date);
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
                match view(&db, note_id, no_format) {
                    Ok(output) => {
                        println!("{}", output);
                        copy(output)
                    }
                    Err(err) => eprintln!("{}", err),
                }
            }
            MonoCommands::Export {} => {
                let output = export(&db);
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
                let result = reminder(&db, config);
                match result {
                    Ok(output) => print!("{}", output),
                    Err(err) => eprintln!("We hit a speed bump! Try again. Error: {}", err),
                }
            }
            MonoCommands::Init {} => {
                init();
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

            // if cli.animate {}

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
