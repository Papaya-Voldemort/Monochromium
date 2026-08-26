mod commands;

use clap::{ Parser};
use commands::{ MonoCommands };

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

fn main() {
    println!("Hello, world!");
}
