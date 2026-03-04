//! Porcelain for rewriting Git repository history and trees.
//!
//! `git-rewrite` wraps the [`git-filter-tree`](https://docs.rs/git-filter-tree)
//! plumbing library in a user-friendly `git`-style CLI. It operates on the
//! working tree's index so results are immediately staged and ready to commit.
//!
//! # CLI
//!
//! ```text
//! git rewrite tree [--only <PATTERN>]... [<TREEISH>]
//! ```
//!
//! Keep only Rust source files from `HEAD`, staged into the index:
//!
//! ```text
//! git rewrite tree --only '**/*.rs'
//! ```
//!
//! Rewrite a specific commit, keeping two directories:
//!
//! ```text
//! git rewrite tree abc1234 --only 'src/' --only 'tests/'
//! ```
//!
//! By default the working tree must be clean. Pass `--allow-dirty` to skip
//! that check.
//!
//! # Library
//!
//! [`exe::tree`] is the programmatic entry point. It opens a repository from
//! the environment, filters the given tree-ish by glob patterns, and rewrites
//! the index:
//!
//! ```no_run
//! use git_rewrite::exe;
//! use git_rewrite::TreeArgs;
//!
//! let args = TreeArgs {
//!     treeish: "HEAD".into(),
//!     patterns: vec!["**/*.rs".into()],
//!     allow_dirty: false,
//! };
//! exe::tree(&args)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod exe;

pub use exe::TreeArgs;
