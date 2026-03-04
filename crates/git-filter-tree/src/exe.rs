use crate::FilterTree;
use git2 as git;

/// Arguments for filtering a Git tree by glob patterns.
///
/// Defined here so that the library exposes a self-contained type that
/// downstream code can construct directly without going through the CLI.
#[derive(clap::Args, Clone)]
pub struct FilterTreeArgs {
    /// Tree-ish reference (commit, branch, tag, or tree SHA).
    pub treeish: String,

    /// Glob patterns for entries to keep in the tree.
    #[arg(required = true)]
    pub patterns: Vec<String>,

    /// Output format.
    #[arg(short, long, value_enum, default_value = "tree-sha")]
    pub format: OutputFormat,
}

/// Output format for [`print_tree`].
#[derive(Clone, Copy, Default, clap::ValueEnum)]
pub enum OutputFormat {
    /// Print only the tree SHA (default).
    #[default]
    TreeSha,
    /// Print each entry's type and name.
    Entries,
    /// Print mode, type, OID, and name for every entry.
    Detailed,
}

/// Core filter-tree operation: resolve the treeish, filter by patterns, and
/// return the OID of the resulting tree.  This is the reusable building-block
/// that both the plumbing binary and the porcelain CLI can call.
pub fn filter_tree(
    repo: &git::Repository,
    treeish: &str,
    patterns: &[String],
) -> Result<git::Oid, Box<dyn std::error::Error>> {
    let obj = repo.revparse_single(treeish)?;
    let tree = obj.peel_to_tree()?;

    let pattern_refs: Vec<&str> = patterns.iter().map(|s| s.as_str()).collect();
    let filtered = repo.filter_by_patterns(&tree, &pattern_refs)?;
    Ok(filtered.id())
}

/// Print a tree according to the requested output format.
pub fn print_tree(
    repo: &git::Repository,
    tree_oid: git::Oid,
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let tree = repo.find_tree(tree_oid)?;

    match format {
        OutputFormat::TreeSha => {
            println!("{}", tree.id());
        }
        OutputFormat::Entries => {
            for entry in tree.iter() {
                let name = entry.name().unwrap_or("<invalid-utf8>");
                let kind = kind_label(entry.kind());
                println!("{}\t{}", kind, name);
            }
        }
        OutputFormat::Detailed => {
            println!("Tree: {}", tree.id());
            println!("Entries: {}", tree.len());
            println!();
            for entry in tree.iter() {
                let name = entry.name().unwrap_or("<invalid-utf8>");
                let kind = kind_label(entry.kind());
                let mode = entry.filemode();
                let id = entry.id();
                println!("{:06o} {} {}\t{}", mode, kind, id, name);
            }
        }
    }

    Ok(())
}

/// Run the full plumbing command: filter and print.
pub fn run(args: &FilterTreeArgs) -> Result<(), Box<dyn std::error::Error>> {
    let repo = git::Repository::open_from_env()?;
    let tree_oid = filter_tree(&repo, &args.treeish, &args.patterns)?;
    print_tree(&repo, tree_oid, args.format)?;
    Ok(())
}

fn kind_label(kind: Option<git::ObjectType>) -> &'static str {
    match kind {
        Some(git::ObjectType::Blob) => "blob",
        Some(git::ObjectType::Tree) => "tree",
        Some(git::ObjectType::Commit) => "commit",
        _ => "unknown",
    }
}
