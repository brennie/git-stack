use std::collections::HashMap;
use std::fmt;

use git2::{Branch, Commit, Oid, Repository, BranchType, PushUpdate};

pub type Stack<'repo> = Vec<StackEntry<'repo>>;

pub struct StackEntry<'repo> {
    pub commit: Commit<'repo>,
    pub local_branch: Option<String>,
}

pub fn current<'repo>(
    repo: &'repo Repository,
    base: Oid,
) -> anyhow::Result<Stack<'repo>> {
    let branches = branch_lookup_table(repo)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut stack = Vec::new();
    for oid in revwalk {
        let oid = oid?;

        if oid == base {
            break;
        }

        let commit = repo.find_commit(oid)?;

        let local_branch = branches
            .get(&oid)
            .iter()
            .flat_map(|b| b.iter())
            .filter_map(|(branch, branch_ty)| if *branch_ty == BranchType::Local { Some(branch) } else { None })
            .next()
            .and_then(|b: &Branch| b.name().expect("branch name is not UTF-8"))
            .map(Into::into);

        stack.push(StackEntry { commit, local_branch });
    }

    Ok(stack)
}

impl<'repo> fmt::Display for StackEntry<'repo> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let summary = self.commit.summary().expect("commit summary is not UTF-8");

        if let Some(local_branch) = &self.local_branch {
            writeln!(f, "[{}]", local_branch)?;
        }

        let id = self.commit.id();
        let short = &id.as_bytes()[..5];
        write!(
            f,
            "    {:02x}{:02x}{:02x}{:02x}{:x} {}",
            short[0],
            short[1],
            short[2],
            short[3],
            short[4] >> 4,
            summary,
        )
    }
}

fn branch_lookup_table(repo: &Repository) -> anyhow::Result<HashMap<Oid, Vec<(Branch, BranchType)>>> {
    let mut branches = HashMap::<_, Vec<_>>::new();

    for branch in repo.branches(None)? {
        let (branch, branch_type) = branch?;
        if let Some(target) = branch.get().target() {
            branches.entry(target).or_default().push((branch, branch_type));
        }
    }

    Ok(branches)
}

