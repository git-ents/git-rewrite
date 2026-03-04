use clap::Parser;
use git_rewrite::exe::TreeArgs;

#[derive(Parser)]
#[command(name = "git rewrite", bin_name = "git rewrite")]
#[command(
    author,
    version,
    about = "Rewrite Git repository history, trees, and blobs"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Rewrite a Git tree, keeping only matching entries
    Tree(TreeArgs),
}
