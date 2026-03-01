use clap::{Args, Parser};

/// Arguments for filtering a Git tree by gitattributes-style patterns.
/// Defined here so it can be re-used as a subcommand in the porcelain `git-rewrite` CLI.
#[derive(Args, Clone)]
pub struct FilterTreeArgs {
    /// Tree-ish reference (commit, branch, tag, or tree SHA)
    pub treeish: String,

    /// Gitattributes-style patterns to filter tree entries
    #[arg(required = true)]
    pub patterns: Vec<String>,

    /// Output format
    #[arg(short, long, value_enum, default_value = "tree-sha")]
    pub format: OutputFormat,
}

#[derive(Parser)]
#[command(name = "git filter-tree", bin_name = "git filter-tree")]
#[command(author, version, about = "Filter Git tree entries by gitattributes-style patterns", long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub args: FilterTreeArgs,
}

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    /// Output only the tree SHA
    TreeSha,
    /// Output tree entries (name and type)
    Entries,
    /// Output detailed tree information
    Detailed,
}
