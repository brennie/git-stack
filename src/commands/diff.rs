
use clap::Args;
use git2::Repository;

#[derive(Args, Debug)]
pub(crate) struct DiffOptions {
    #[arg()]
    /// The remote to compare with. Will default to the first remote listed.
    remote: Option<String>,
}

pub(crate) fn diff_stack(opts: DiffOptions) -> anyhow::Result<()> {
    let repo = Repository::discover(".")?;

    let remote_name = match opts.remote {
        Some(remote_name) => remote_name,
        None => {
            let remotes = repo.remotes()?;

            if remotes.is_empty() {
                anyhow::bail!("Could not find a remote");
            }

            remotes.get(0).expect("remotes should not be empty").into()
        }
    };

    let remote = repo.find_remote(&remote_name)?;

    let head = repo.head()?;
    let branch = head.name().expect("branch is not utf-8");

    // let mut matching_refspec = None;

    println!("branch: {}", branch);

    println!("remote: {remote_name}");
    for refspec in remote.refspecs() {
        if refspec.direction() == git2::Direction::Fetch {
            if let (Some(src), Some(dst)) = (refspec.src(), refspec.dst()) {
                if src.ends_with("/*") && dst.ends_with("/*")  {
                    let src = &src[..src.len() - 1];
                    let dst = &dst[..dst.len() - 1];
                    if let Some(rest) = branch.strip_prefix(src) {
                        println!("{}{} -> {}{}", src, rest, dst, rest);

                        let upstream_ref_name = format!("{}{}", dst, rest);

                        let upstream_ref = repo.find_reference(&upstream_ref_name)?;
                        println!("is branch: {:?}", upstream_ref.target());

                    }
                }

            }
        }
    }

    Ok(())
}
