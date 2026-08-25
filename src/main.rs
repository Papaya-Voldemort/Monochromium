use clap::{Arg, Parser, Subcommand};

#[derive(Parser)]
#[clap( author, version, about)]
struct MonoCLI {
    #[command(subcommand)]
    command: MonoCommands,

    #[clap(short, long, global = true)]
    copy: bool,
}

#[derive(Subcommand)]
enum MonoCommands {

}

fn main() {
    println!("Hello, world!");
}
