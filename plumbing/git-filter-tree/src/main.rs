mod cli;

use clap::{CommandFactory, Parser};
use cli::{Cli, OutputFormat};
use git_filter_tree::FilterTree;
use git2 as git;
use std::path::PathBuf;
use std::process;

fn main() {
    if let Some(dir) = parse_generate_man_flag() {
        if let Err(e) = generate_man_page(dir) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        return;
    }

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Check for `--generate-man <DIR>` before clap parses, so it doesn't
/// conflict with the required positional arguments.
fn parse_generate_man_flag() -> Option<PathBuf> {
    let args: Vec<String> = std::env::args().collect();
    let pos = args.iter().position(|a| a == "--generate-man")?;
    let dir = args
        .get(pos + 1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("man"));
    Some(dir)
}

fn generate_man_page(output_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let man1_dir = output_dir.join("man1");
    std::fs::create_dir_all(&man1_dir)?;

    let cmd = Cli::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer)?;
    std::fs::write(man1_dir.join("git-filter-tree.1"), buffer)?;

    eprintln!("  → git-filter-tree.1");
    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Open the repository in current directory
    let repo = git::Repository::open_from_env()?;

    // Resolve the tree-ish to a tree
    let obj = repo.revparse_single(&cli.args.treeish)?;
    let tree = obj.peel_to_tree()?;

    // Convert patterns to string slices
    let patterns: Vec<&str> = cli.args.patterns.iter().map(|s| s.as_str()).collect();

    // Filter the tree by patterns
    let filtered_tree = repo.filter_by_patterns(&tree, &patterns)?;

    // Output based on format
    match cli.args.format {
        OutputFormat::TreeSha => {
            println!("{}", filtered_tree.id());
        }
        OutputFormat::Entries => {
            for entry in filtered_tree.iter() {
                let name = entry.name().unwrap_or("<invalid-utf8>");
                let kind = match entry.kind() {
                    Some(git::ObjectType::Blob) => "blob",
                    Some(git::ObjectType::Tree) => "tree",
                    Some(git::ObjectType::Commit) => "commit",
                    _ => "unknown",
                };
                println!("{}\t{}", kind, name);
            }
        }
        OutputFormat::Detailed => {
            println!("Tree: {}", filtered_tree.id());
            println!("Entries: {}", filtered_tree.len());
            println!();
            for entry in filtered_tree.iter() {
                let name = entry.name().unwrap_or("<invalid-utf8>");
                let kind = match entry.kind() {
                    Some(git::ObjectType::Blob) => "blob",
                    Some(git::ObjectType::Tree) => "tree",
                    Some(git::ObjectType::Commit) => "commit",
                    _ => "unknown",
                };
                let mode = entry.filemode();
                let id = entry.id();
                println!("{:06o} {} {}\t{}", mode, kind, id, name);
            }
        }
    }

    Ok(())
}
