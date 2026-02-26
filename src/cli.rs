use clap::Parser;
use git_filter_tree::cli::FilterTreeArgs;

#[derive(Parser)]
#[command(name = "git-filter")]
#[command(author, version, about = "Filter Git repository history, trees, and blobs", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Filter Git tree entries by gitattributes-style patterns
    FilterTree(FilterTreeArgs),
}
