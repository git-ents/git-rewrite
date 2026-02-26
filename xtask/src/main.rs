use clap::CommandFactory;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(clap::Parser)]
#[command(name = "xtask")]
#[command(about = "Development tasks for git-filter")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Build all crates and generate man pages into the target profile directory
    Build {
        /// Build with the release profile
        #[arg(short, long)]
        release: bool,

        /// Build with a named profile (overrides --release)
        #[arg(long, value_name = "PROFILE")]
        profile: Option<String>,

        /// Target directory (default: workspace target/)
        #[arg(long, value_name = "DIR")]
        target_dir: Option<PathBuf>,
    },

    /// Generate man pages for all CLI tools
    GenMan {
        /// Output directory for man pages (a man1/ subdirectory will be created inside)
        #[arg(short, long, default_value = "target/debug/man")]
        output: PathBuf,
    },
}

fn main() {
    let cli = clap::Parser::parse();

    let result = match cli {
        Cli {
            command:
                Commands::Build {
                    release,
                    profile,
                    target_dir,
                },
        } => run_build(release, profile, target_dir),

        Cli {
            command: Commands::GenMan { output },
        } => generate_man_pages(&output),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

// ---------------------------------------------------------------------------
// Build
// ---------------------------------------------------------------------------

fn run_build(
    release: bool,
    profile: Option<String>,
    target_dir: Option<PathBuf>,
) -> std::io::Result<()> {
    // Resolve the effective profile name so we know where binaries land.
    let profile_name = profile.clone().unwrap_or_else(|| {
        if release {
            "release".into()
        } else {
            "debug".into()
        }
    });

    // Build the `cargo build` invocation, excluding xtask itself.
    let mut cmd = Command::new(cargo_bin());
    cmd.arg("build")
        .arg("--workspace")
        .arg("--exclude")
        .arg("xtask");

    if let Some(ref p) = profile {
        cmd.arg("--profile").arg(p);
    } else if release {
        cmd.arg("--release");
    }

    let resolved_target_dir = if let Some(ref dir) = target_dir {
        cmd.arg("--target-dir").arg(dir);
        dir.clone()
    } else {
        // Default: <workspace-root>/target
        workspace_root()?.join("target")
    };

    println!("Running: {:?}", cmd);

    let status = cmd.status()?;
    if !status.success() {
        return Err(std::io::Error::other(format!(
            "cargo build failed with status {}",
            status
        )));
    }

    // Generate man pages next to the built binaries:
    //   <target-dir>/<profile>/man/
    let man_dir = resolved_target_dir.join(&profile_name).join("man");
    generate_man_pages(&man_dir)?;

    println!("\nView with: MANPATH={} man git-filter", man_dir.display());
    Ok(())
}

/// Returns the path to the `cargo` executable, preferring `$CARGO` when set
/// (Cargo sets this env-var when invoking subprocesses so the same toolchain
/// is used).
fn cargo_bin() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".into())
}

/// Walk up from the xtask binary's location to find the workspace root, i.e.
/// the directory that contains the top-level `Cargo.toml`.  Falls back to the
/// current working directory when the manifest cannot be located.
fn workspace_root() -> std::io::Result<PathBuf> {
    // `cargo run -p xtask` sets CARGO_MANIFEST_DIR to the xtask package dir.
    // The workspace root is one level up from there.
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let xtask_dir = PathBuf::from(manifest_dir);
        if let Some(parent) = xtask_dir.parent() {
            return Ok(parent.to_path_buf());
        }
    }
    std::env::current_dir()
}

// ---------------------------------------------------------------------------
// Man-page generation
// ---------------------------------------------------------------------------

fn generate_man_pages(output_dir: &Path) -> std::io::Result<()> {
    let man1_dir = output_dir.join("man1");
    fs::create_dir_all(&man1_dir)?;

    println!("Generating man pages to: {}", man1_dir.display());

    generate_man::<git_filter_tree::cli::Cli>(&man1_dir, "git-filter-tree")?;
    generate_man::<git_filter_cli::cli::Cli>(&man1_dir, "git-filter")?;

    println!("✓ Man pages generated successfully!");
    Ok(())
}

fn generate_man<C: CommandFactory>(output_dir: &Path, name: &str) -> std::io::Result<()> {
    let cmd = C::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer)?;

    let filename = format!("{}.1", name);
    let man_path = output_dir.join(&filename);
    fs::write(&man_path, buffer)?;

    println!("  → {}", filename);
    Ok(())
}
