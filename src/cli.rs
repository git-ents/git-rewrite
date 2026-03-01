use clap::Parser;

#[derive(Parser)]
#[command(name = "git rewrite", bin_name = "git rewrite")]
#[command(author, version, about = "Rewrite Git repository history, trees, and blobs", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Rewrite a Git tree, keeping only matching entries
    Tree(TreeArgs),
}

#[derive(clap::Args)]
pub struct TreeArgs {
    /// Tree-ish reference (commit, branch, tag, or tree SHA) [default: HEAD]
    #[arg(default_value = "HEAD")]
    pub treeish: String,

    /// Glob patterns for entries to keep in the tree (may be repeated)
    #[arg(long = "only", required = true)]
    pub patterns: Vec<String>,

    /// Allow running with uncommitted changes in the working tree
    #[arg(long)]
    pub allow_dirty: bool,
}
