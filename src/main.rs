// #![allow(warnings)]
mod commands;
mod config;
mod database;
mod types;
mod utils;

use crate::commands::{
    add, check_in, delete, edit, export, init, list, list_todos, reminder, search, view,
};
use crate::config::load_config;
use crate::database::Database;
use crate::types::{EditMode, MonoError};
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

    match cli.command {
        Some(cmd) => match cmd {
            MonoCommands::Add {
                text,
                note_type,
                time,
                date,
                paste,
            } => {
                let output = add(&db, text, note_type, time, date, paste)?;
                println!("{}", output);

                if cli.copy {
                    copy(output)?;
                }

                Ok(())
            }
            MonoCommands::CheckIn {
                text,
                time,
                date,
                paste,
            } => {
                let output = check_in(&db, text, time, date, paste)?;
                println!("{}", output);

                if cli.copy {
                    copy(output)?;
                }

                Ok(())
            }
            MonoCommands::Delete { note_id, approve } => {
                let output = delete(&db, note_id, approve)?;
                println!("{}", output);

                if cli.copy {
                    copy(output)?;
                }

                Ok(())
            }
            MonoCommands::Edit {
                note_id,
                headless,
                edit_mode,
                text,
            } => {
                let output = edit(&db, note_id, headless, edit_mode, text)?;

                match output.update_type {
                    EditMode::Append => {
                        println!(
                            "Appended \"{}\" to the end of \"{}\"!",
                            output.new, output.old
                        );
                    }
                    EditMode::Overwrite => {
                        println!("Replaced \"{}\" with \"{}\"!", output.old, output.new);
                    }
                };

                Ok(())
            }
            MonoCommands::List {
                limit,
                note_type,
                today,
                since,
                view,
            } => {
                let list = list(&db, limit, note_type, today, since, view)?;
                for item in &list {
                    println!("{}", item);
                }

                if cli.copy {
                    copy(list.join("\n"))?;
                }

                Ok(())
            }
            MonoCommands::TodoList {
                limit,
                today,
                since,
                view,
            } => {
                let list = list_todos(&db, limit, today, since, view)?;
                for item in &list {
                    println!("{}", item);
                }

                if cli.copy {
                    copy(list.join("\n"))?;
                }

                Ok(())
            }
            MonoCommands::Search {
                text,
                limit,
                note_type,
                date,
            } => {
                let list = search(&db, text, limit, note_type, date)?;
                for item in &list {
                    println!("{}", item);
                }

                if cli.copy {
                    copy(list.join("\n"))?;
                }

                Ok(())
            }
            MonoCommands::View { note_id, no_format } => {
                let output = view(&db, note_id, no_format)?;

                println!("{}", output);

                if cli.copy {
                    copy(output)?;
                }

                Ok(())
            }

            MonoCommands::Export {} => {
                let list = export(&db)?;

                for item in &list {
                    println!("{}", item);
                }

                if cli.copy {
                    copy(list.join("\n"))?;
                }

                Ok(())
            }

            MonoCommands::Reminder {} => {
                let output = reminder(&db, config)?;
                print!("{}", output);

                Ok(())
            }
            MonoCommands::Init {} => {
                init();
                Ok(())
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
                copy(full_output)?;
            }

            Ok(())
        }
    }
}
