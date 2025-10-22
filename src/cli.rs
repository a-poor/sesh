//! App's CLI code.

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Init,
    Status,
    Up,
    Down,
    Attach,
    Restart,
}
