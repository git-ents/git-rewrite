use clap::Parser;
use git_filter_tree::cli::FilterTreeArgs;

#[derive(Parser)]
#[command(name = "git-rewrite")]
#[command(author, version, about = "Rewrite Git repository history, trees, and blobs", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Filter Git tree entries by gitattributes-style patterns
    Tree(FilterTreeArgs),
}
