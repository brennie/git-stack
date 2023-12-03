mod stack;
mod commands;

use clap::{Parser, Subcommand};

use crate::commands::diff::{diff_stack, DiffOptions};
use crate::commands::show::{show_stack, ShowOptions};

#[derive(Debug, Parser)]
#[command(about, version)]
#[command(disable_help_flag(true))] // `git cmd --help` becomes `git help cmd`. We'll just use a `help` subcommand.
struct Options {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Diff the stack with a remote.
    Diff(DiffOptions),
    /// Show the current stack.
    Show(ShowOptions),
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    match options.command {
        Command::Show(opts) => show_stack(opts)?,
        Command::Diff(opts) => diff_stack(opts)?,
    }

    Ok(())
}
