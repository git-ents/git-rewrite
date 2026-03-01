mod cli;

use clap::{CommandFactory, Parser};
use cli::{Cli, Command};
use git_filter_tree::FilterTree;
use git_filter_tree::cli::OutputFormat;
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
/// conflict with the required subcommand.
fn parse_generate_man_flag() -> Option<PathBuf> {
    let args: Vec<String> = std::env::args().collect();
    let pos = args.iter().position(|a| a == "--generate-man")?;
    let dir = args
        .get(pos + 1)
        .map(PathBuf::from)
        .unwrap_or_else(default_man_dir);
    Some(dir)
}

fn default_man_dir() -> PathBuf {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let home = std::env::var_os("HOME").expect("HOME is not set");
            PathBuf::from(home).join(".local/share")
        })
        .join("man")
}

fn generate_man_page(output_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let man1_dir = output_dir.join("man1");
    std::fs::create_dir_all(&man1_dir)?;

    let cmd = Cli::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer)?;

    let man_path = man1_dir.join("git-rewrite.1");
    std::fs::write(&man_path, buffer)?;

    let output_dir = output_dir.canonicalize()?;
    eprintln!("Wrote man page to {}", man_path.canonicalize()?.display());

    if !manpath_covers(&output_dir) {
        eprintln!();
        eprintln!("You may need to add this to your shell environment:");
        eprintln!();
        eprintln!("  export MANPATH=\"{}:$MANPATH\"", output_dir.display());
    }
    Ok(())
}

/// Returns `true` if `dir` is equal to, or a subdirectory of, any component
/// in the `MANPATH` environment variable.
fn manpath_covers(dir: &std::path::Path) -> bool {
    let Some(manpath) = std::env::var_os("MANPATH") else {
        return false;
    };
    for component in std::env::split_paths(&manpath) {
        let Ok(component) = component.canonicalize() else {
            continue;
        };
        if dir.starts_with(&component) {
            return true;
        }
    }
    false
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Tree(args) => filter_tree(&args.treeish, &args.patterns, args.format),
    }
}

fn filter_tree(
    treeish: &str,
    patterns: &[String],
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let repo = git::Repository::open_from_env()?;

    let obj = repo.revparse_single(treeish)?;
    let tree = obj.peel_to_tree()?;

    let patterns: Vec<&str> = patterns.iter().map(|s| s.as_str()).collect();

    let filtered_tree = repo.filter_by_patterns(&tree, &patterns)?;

    match format {
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
