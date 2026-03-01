use clap::Parser;

#[derive(Parser)]
#[command(name = "git-rewrite")]
#[command(author, version, about = "Rewrite Git repository history, trees, and blobs", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Filter a Git tree by glob patterns
    Tree(TreeArgs),
}

#[derive(clap::Args)]
pub struct TreeArgs {
    /// Tree-ish reference (commit, branch, tag, or tree SHA) [default: HEAD]
    #[arg(default_value = "HEAD")]
    pub treeish: String,

    /// Glob patterns to filter tree entries (may be repeated)
    #[arg(long = "glob", required = true)]
    pub patterns: Vec<String>,

    /// Allow running with uncommitted changes in the working tree
    #[arg(long)]
    pub allow_dirty: bool,
}
