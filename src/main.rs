use chrono::{NaiveDate, NaiveTime};
use clap::{Arg, Parser, Subcommand, ValueEnum};

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

#[derive(Subcommand)]
enum MonoCommands {
    Add {
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
        #[clap(long)]
        time: Option<NaiveTime>,

        #[clap(long)]
        date: Option<NaiveDate>,

        #[clap(short, long)]
        paste: bool,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum NoteTypes {
    Idea,
    CheckIn,
    Random,
}

fn main() {
    println!("Hello, world!");
}
