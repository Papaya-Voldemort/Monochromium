use chrono::{NaiveDate, NaiveTime};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum NoteTypes {
    Idea,
    CheckIn,
    Todo,
    Other,
}

#[derive(Parser)]
#[clap(author, version, about)]
pub struct MonoCLI {
    #[command(subcommand)]
    pub command: Option<MonoCommands>,

    /// Copy output to clipboard
    #[clap(short, long, global = true)]
    pub copy: bool,

    #[clap(short, long)]
    pub animate: bool,
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
    },

    /// Create a note with check-in type
    CheckIn {
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
    },

    /// List out existing notes
    List {
        /// Max amount of notes to be listed
        #[clap(short, long)]
        limit: Option<u16>,

        /// Only allow certain types of notes (todo, check in, idea, other)
        #[clap(short, long)]
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
        #[arg()]
        text: String,

        #[clap(short, long)]
        limit: Option<u16>,

        #[clap(short, long)]
        note_type: Option<NoteTypes>,

        #[clap(long)]
        date: Option<NaiveDate>,
    },

    /// View a notes content
    View {
        #[arg()]
        note_id: u32,

        #[clap(short, long)]
        no_format: bool,
    },

    /// Edit a notes content (beta)
    Edit {
        #[arg()]
        note_id: u32,

        #[clap(long)]
        headless: bool,

        #[clap(short, long, conflicts_with = "overwrite")]
        append: bool,

        #[clap(short, long)]
        overwrite: bool,

        // For v2 do not require text
        #[clap()]
        text: String,
    },

    /// Delete a note
    Delete {
        #[arg()]
        note_id: u32,

        #[clap(short, long)]
        approve: bool,
    },

    /// Util command for displaying reminders
    Reminder {},

    /// Util command for initializing config
    Init {},
}
