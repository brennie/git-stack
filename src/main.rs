mod commands;
mod stack;

use clap::{Parser, Subcommand};
use git2::Repository;

use crate::commands::{show_stack, ShowOptions};


#[derive(Parser)]
#[command(about, version)]
#[command(disable_help_flag(true))] // git cmd --help becomes git help cmd.
struct Options {
    #[command(subcommand)]
    command: Command,

}

#[derive(Subcommand)]
enum Command    {
    Show(ShowOptions),
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();
    let repo = Repository::discover(".")?;
    match options.command {
        Command::Show(command_options) => show_stack(repo, command_options)?,
    }

    Ok(())
}
