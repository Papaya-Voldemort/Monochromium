use crate::types::notes::{EditMode, NoteTypes};
use chrono::{NaiveDate, NaiveTime};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct MonoCLI {
    #[command(subcommand)]
    pub command: Option<MonoCommands>,

    /// Copy output to clipboard
    #[clap(short, long, global = true)]
    pub copy: bool,
    // #[clap(short, long)]
    // pub animate: bool,
}

#[derive(Subcommand)]
pub enum MonoCommands {
    /// Create a new note
    Add {
        /// Note text content
        #[arg(value_name = "TEXT")]
        text: Option<String>,

        /// Note category [default 'other']
        #[clap(short = 't', long = "type")]
        note_type: Option<NoteTypes>,

        /// Logged time of note (HH:MM or HH:MM:SS) [default: current time]
        #[clap(long)]
        time: Option<NaiveTime>,

        /// Logged date of note (YYYY-MM-DD) [default: current date]
        #[clap(long)]
        date: Option<NaiveDate>,

        /// Use clipboard content as the note body
        #[clap(short, long)]
        paste: bool,

        /// Value assigned to each note [check-in mood, todo priority, idea hype] (-5..=5)
        #[clap(short, long, value_parser = clap::value_parser!(i64).range(-5..=5), allow_negative_numbers = true)]
        value: Option<i64>,
    },

    /// Quick mood check-in (sugar for: mono add -t checkin)
    Checkin {
        /// Note text content
        #[arg(value_name = "TEXT")]
        text: Option<String>,

        /// Logged time of note (HH:MM or HH:MM:SS) [default: current time]
        #[clap(long)]
        time: Option<NaiveTime>,

        /// Logged date of note (YYYY-MM-DD) [default: current date]
        #[clap(long)]
        date: Option<NaiveDate>,

        /// Use clipboard content as the note body
        #[clap(short, long)]
        paste: bool,

        /// Mood tracking (-5..=5)
        #[clap(short, long, value_parser = clap::value_parser!(i64).range(-5..=5), allow_negative_numbers = true)]
        mood: Option<i64>,
    },

    /// Quick todo (sugar for: mono add -t todo)
    Todo {
        /// Note text content
        #[arg(value_name = "TEXT")]
        text: Option<String>,

        /// Logged time of note (HH:MM or HH:MM:SS) [default: current time]
        #[clap(long)]
        time: Option<NaiveTime>,

        /// Logged date of note (YYYY-MM-DD) [default: current date]
        #[clap(long)]
        date: Option<NaiveDate>,

        /// Use clipboard content as the note body
        #[clap(short, long)]
        paste: bool,

        /// Task priority or importance level (-5..=5)
        #[clap(short = 'i', long, value_parser = clap::value_parser!(i64).range(-5..=5), allow_negative_numbers = true)]
        priority: Option<i64>,
    },

    /// Quick idea capture (sugar for: mono add -t idea)
    Idea {
        /// Note text content
        #[arg(value_name = "TEXT")]
        text: Option<String>,

        /// Logged time of note (HH:MM or HH:MM:SS) [default: current time]
        #[clap(long)]
        time: Option<NaiveTime>,

        /// Logged date of note (YYYY-MM-DD) [default: current date]
        #[clap(long)]
        date: Option<NaiveDate>,

        /// Use clipboard content as the note body
        #[clap(short, long)]
        paste: bool,

        /// How good of an idea it is (-5..=5)
        #[clap(short = 'y', long, value_parser = clap::value_parser!(i64).range(-5..=5), allow_negative_numbers = true)]
        hype: Option<i64>,
    },

    /// List out existing notes
    List {
        /// Max amount of notes to be listed
        #[clap(short, long)]
        limit: Option<u16>,

        /// Only allow certain types of notes (todo, checkin, idea, other)
        #[arg(short = 't', long = "type")]
        note_type: Option<NoteTypes>,

        /// Only show notes from today
        #[clap(long)]
        today: bool,

        /// Show all notes after a date
        #[clap(short, long)]
        since: Option<NaiveDate>,

        /// Show note content as well
        #[clap(short, long)]
        view: bool,
    },

    /// Search through notes
    Search {
        /// Search value
        #[arg()]
        text: String,

        /// Max amount of notes to be returned
        #[clap(short, long)]
        limit: Option<u16>,

        /// Only allow certain types of notes (todo, checkin, idea, other)
        #[clap(short, long)]
        note_type: Option<NoteTypes>,

        /// Filter by date (YYYY-MM-DD)
        #[clap(long)]
        date: Option<NaiveDate>,
    },

    /// View a notes content
    View {
        /// The id of the note you want to view
        #[arg()]
        note_id: u32,

        /// Output only the raw note content
        #[clap(short, long)]
        no_format: bool,
    },

    /// Edit a note's content or metadata
    Edit {
        /// The id of the note you want to edit
        #[arg()]
        note_id: u32,

        /// Update title, file path, type, or date instead of opening the editor
        #[arg(short = 'm', long = "mode", requires = "change")]
        edit_mode: Option<EditMode>,

        /// New value for the selected metadata field
        #[arg(requires = "edit_mode")]
        change: Option<String>,
    },

    /// Delete a note
    #[command(visible_alias = "rm")]
    Delete {
        /// The ID of the note you want to delete
        #[arg()]
        note_id: u32,

        /// Approval flag to actually delete the note
        #[clap(short, long)]
        approve: bool,
    },

    /// Display a basic stats overview (beta)
    Stats {},

    /// Output all the notes
    Export {},

    /// Util command for displaying reminders
    Reminder {},

    /// Util command for initializing config
    Init {},

    /// Open config file in default editor
    Config {},

    // Cryptography commands
    /// Lock a note
    Lock {
        /// The id of the note you want lock
        #[arg()]
        note_id: u32,
    },

    /// Unlock a note
    Unlock {
        /// The id of the note you want to unlock
        #[arg()]
        note_id: u32,
    },
}
