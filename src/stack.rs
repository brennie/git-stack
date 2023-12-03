use std::fmt;

use git2::{Commit, Oid, Repository};

pub type Stack<'repo> = Vec<StackEntry<'repo>>;

pub struct StackEntry<'repo> {
    commit: Commit<'repo>,
}

pub fn current<'repo>(
    repo: &'repo Repository,
    base: Oid,
) -> anyhow::Result<Stack<'repo>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut stack = Vec::new();
    for oid in revwalk {
        let oid = oid?;

        if oid == base {
            break;
        }

        let commit = repo.find_commit(oid)?;

        stack.push(StackEntry { commit });
    }

    Ok(stack)
}

impl<'repo> fmt::Display for StackEntry<'repo> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let summary = self.commit.summary().expect("commit summary is not UTF-8");
        let id = self.commit.id();
        let short = &id.as_bytes()[..5];
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}{:x} {}",
            short[0],
            short[1],
            short[2],
            short[3],
            (short[4] & 0xF0) >> 4,
            summary,
        )
    }
}
