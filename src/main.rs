mod adjectives;
mod app;
mod cli;
mod conf;
mod nouns;
mod words;

use clap::Parser;
use cli::{Cli, Command, WindowCommands};

fn main() {
    let c = Cli::parse();
    if let Err(err) = match c.command {
        Command::Init(ref args) => app::run_init(&c, args),
        Command::Status => app::run_status(&c),
        Command::Up => app::run_up(&c),
        Command::Down => app::run_down(&c),
        Command::Attach => app::run_attach(&c),
        Command::Restart => app::run_restart(&c),
        Command::Window(ref args) => match args.command {
            WindowCommands::Add(ref _args) => app::run_window_add(&c),
            WindowCommands::Remove(ref _args) => app::run_window_remove(&c),
        },
    } {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
