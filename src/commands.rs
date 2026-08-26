use chrono::{NaiveDate, NaiveTime};
use clap::{Subcommand, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum NoteTypes {
    Idea,
    CheckIn,
    Random,
}

#[derive(Subcommand)]
pub enum MonoCommands {
    Add {
        #[arg()]
        text: String,

        #[clap(short = 't', long = "type")]
        note_type: Option<NoteTypes>,

        #[clap(long)]
        time: Option<NaiveTime>,

        #[clap(long)]
        date: Option<NaiveDate>,

        #[clap(short, long)]
        paste: bool,
    },

    CheckIn {
        #[arg()]
        text: String,

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

        #[clap(short, long)]
        headless: bool,

        #[clap(short, long)]
        append: bool,

        #[clap(short, long)]
        overwrite: bool,
    },

    Delete {
        #[arg()]
        note_id: u32,

        #[clap(short, long)]
        approve: bool,
    },
}