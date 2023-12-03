use std::collections::HashMap;

use git2::{Branch, BranchType, Commit, Repository, Oid};

pub struct StackEntry<'repo> {
    commit: Commit<'repo>,
    local_branch: Option<Branch<'repo>>,
    remote_branch: Option<Branch<'repo>>,
}

pub fn current<'repo>(
    repo: &'repo Repository,
    base: Oid,
) -> anyhow::Result<Vec<StackEntry<'repo>>> {
    let branches = get_branch_lookup_table(&repo)?;

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut stack = Vec::new();

    for oid in revwalk {
        let oid = oid?;

        if oid == base {
            break;
        }

        let commit = repo.find_commit(oid)?;

        let (local_branch, remote_branch) = if let Some(commit_branches) = branches.get(&oid) {
            let local_branch = commit_branches
                .iter()
                .filter_map(|(branch, branch_ty)| if *branch_ty == BranchType::Local { Some(branch) } else { None })
                .map(|b| *b)
                .next();

            let remote_branch = local_branch.and_then(|local_branch| {
                commit_branches
                    .iter()
                    .filter_map(|(branch, branch_ty)| if *branch_ty == BranchType::Remote { Some(*branch) } else { None })
                    .next()
            });
            (local_branch, None)
        } else {
            (None, None)
        };

        stack.push(StackEntry {
            commit,
            local_branch,
            remote_branch,
        });
    }

    Ok(stack)
}

fn get_branch_lookup_table(
    repo: &Repository,
) -> anyhow::Result<HashMap<Oid, Vec<(Branch, BranchType)>>> {
    let mut branches = HashMap::<Oid, Vec<(Branch, BranchType)>>::new();

    for branch in repo.branches(None)? {
        let (branch, branch_type) = branch?;

        if let Some(target) = branch.get().target() {
            branches
                .entry(target)
                .or_default()
                .push((branch, branch_type));
        }
    }

    Ok(branches)
}
