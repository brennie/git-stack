use anyhow::{anyhow, Context, Result};
use clap::Args;
use git2::{CredentialType, PushUpdate, Repository};

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

#[derive(Args)]
pub struct PushOptions {
    /// The base branch of the stack.
    #[arg(long, default_value = "main")]
    base: String,

    /// The remote to push to.
    #[arg()]
    remote: Option<String>,

    #[arg(long)]
    force: bool,
}

pub fn push_stack(repo: Repository, options: PushOptions) -> anyhow::Result<()> {
    let base_ref = repo.resolve_reference_from_short_name(&options.base)?;
    if !base_ref.is_branch() {
        return Err(anyhow!("`{}' is not a branch", options.base));
    }

    let base_commit = base_ref.peel_to_commit()?;
    let stack = stack::current(&repo, base_commit.id())?;

    let mut remote = match options.remote {
        Some(remote_name) => repo
            .find_remote(&remote_name)
            .with_context(|| format!("Could not find remote `{}'", remote_name))?,
        None => {
            let base_ref_name = base_ref.name().expect("reference is not UTF-8");
            let remote_name = repo.branch_upstream_remote(base_ref_name)?;
            let remote_name = remote_name.as_str().expect("remote is not UTF-8");
            repo.find_remote(remote_name)
                .with_context(|| format!("Could not find remote `{}'", remote_name))?
        }
    };

    let mut opts = git2::PushOptions::new();
    let mut cbs = git2::RemoteCallbacks::new();
    cbs.credentials(fetch_credentials)
       .push_negotiation(negotiate);

    opts.remote_callbacks(cbs);

    let force = if options.force { "+" } else { "" };

    let refspecs = stack
        .iter()
        .filter_map(|entry| entry.local_branch.as_ref())
        .map(|local_branch| {
            let local_branch_ref = repo.resolve_reference_from_short_name(local_branch)?;
            let local_branch_ref_name = local_branch_ref.name().expect("ref is not UTF-8");
            Ok(format!(
                "+{}{}:{}",
                force, local_branch_ref_name, local_branch_ref_name
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    remote.push(&refspecs, Some(&mut opts))?;

    Ok(())
}

fn fetch_credentials(
    url: &str,
    username_from_url: Option<&str>,
    allow_types: CredentialType,
) -> Result<git2::Cred, git2::Error> {
    if !allow_types.contains(CredentialType::SSH_KEY) {
        Err(git2::Error::from_str("Only can fetch SSH keys"))
    } else if let Some(username) = username_from_url {
        git2::Cred::ssh_key_from_agent(username)
    } else {
        Err(git2::Error::from_str("No username configured for remote"))
    }
}

fn negotiate(updates: &[PushUpdate<'_>]) -> Result<(), git2::Error> {
    for update in updates {
        println!(
            "{} ({}) -> {} ({})",
            update.src_refname().unwrap(),
            update.src(),
            update.dst_refname().unwrap(),
            update.dst()
        );
    }
    Ok(())
}
