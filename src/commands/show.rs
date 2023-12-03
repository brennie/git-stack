use std::collections::HashMap;

use clap::Args;
use git2::{Branch, BranchType, Oid, Repository};
use owo_colors::colors::*;
use owo_colors::*;

#[derive(Args, Debug)]
pub(crate) struct ShowOptions {
    #[arg(long, default_value = "main")]
    base: String,
}

pub(crate) fn show_stack(opts: ShowOptions) -> anyhow::Result<()> {
    let repo = Repository::discover(".")?;

    let base_commit_id = repo.revparse_single(&opts.base)?.peel_to_commit()?.id();

    let mut walk = repo.revwalk()?;
    walk.push_head()?;

    let branches = get_branch_lookup_table(&repo)?;

    for oid in walk {
        let oid = oid?;

        if oid == base_commit_id {
            break;
        }

        let commit = repo.find_commit(oid)?;
        if let Some(bs) = branches.get(&oid) {
            for (branch, branch_type) in bs {
                if *branch_type == BranchType::Local {
                    let branch_name = branch.name()?.expect("branch name is not UTF-8");
                    println!("[{}]", branch_name.fg::<Blue>());
                }
            }
        }

        let summary = commit.summary().expect("commit summary is not UTF-8");
        let short = &oid.as_bytes()[..4];
        println!(
            "    {:02x}{:02x}{:02x}{:02x} {}",
            short[0].fg::<Yellow>(),
            short[1].fg::<Yellow>(),
            short[2].fg::<Yellow>(),
            short[3].fg::<Yellow>(),
            summary
        );
    }

    Ok(())
}

