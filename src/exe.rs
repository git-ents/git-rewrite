use crate::cli::TreeArgs;
use git2 as git;

pub fn tree(args: &TreeArgs) -> Result<(), Box<dyn std::error::Error>> {
    let repo = git::Repository::open_from_env()?;

    if !args.allow_dirty {
        let statuses = repo.statuses(Some(
            git::StatusOptions::new()
                .include_untracked(false)
                .include_ignored(false),
        ))?;
        if !statuses.is_empty() {
            return Err(
                "working tree has uncommitted changes; use --allow-dirty to override".into(),
            );
        }
    }

    let tree_oid = git_filter_tree::exe::filter_tree(&repo, &args.treeish, &args.patterns)?;
    let filtered = repo.find_tree(tree_oid)?;

    let mut index = repo.index()?;
    index.read_tree(&filtered)?;
    index.write()?;

    let pattern_list: String = args
        .patterns
        .iter()
        .map(|p| format!("'{}'", p))
        .collect::<Vec<_>>()
        .join(", ");

    println!(
        "Rewrote {} keeping only {} and updated the index ({} entries).",
        args.treeish,
        pattern_list,
        filtered.len(),
    );

    Ok(())
}
