use clap::Parser;
use git_filter_tree::exe::FilterTreeArgs;

#[derive(Parser)]
#[command(name = "git filter-tree", bin_name = "git filter-tree")]
#[command(author, version, about = "Filter Git tree entries by glob patterns")]
pub struct Cli {
    #[command(flatten)]
    pub args: FilterTreeArgs,
}
