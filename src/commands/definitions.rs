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
    pub command: MonoCommands,

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

    CheckIn {
        #[arg()]
        text: Option<String>,

        #[clap(long)]
        time: Option<NaiveTime>,

        #[clap(long)]
        date: Option<NaiveDate>,

        #[clap(short, long)]
        paste: bool,
    },

    List {
        #[clap(short, long)]
        limit: Option<u16>,

        #[clap(short, long)]
        note_type: Option<NoteTypes>,

        #[clap(long)]
        today: bool,

        #[clap(short, long)]
        since: Option<NaiveDate>,

        #[clap(short, long)]
        view: bool,
    },

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

    View {
        #[arg()]
        note_id: u32,

        #[clap(short, long)]
        no_format: bool,
    },

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

    Delete {
        #[arg()]
        note_id: u32,

        #[clap(short, long)]
        approve: bool,
    },

    Reminder {}
}
