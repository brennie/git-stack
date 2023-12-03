use clap::Args;
use git2::Repository;

use crate::stack;

#[derive(Args)]
pub struct ShowOptions {
    /// The base branch of the stack.
    #[arg(long, default_value = "main")]
    base: String,
}


pub fn show_stack(repo: Repository, options: ShowOptions) -> anyhow::Result<()> {
    let base = repo.revparse_single(&options.base)?.peel_to_commit()?.id();
    let stack = stack::current(&repo, base)?;

    for entry in &stack {
        println!("{}", entry);
    }

    Ok(())
}
