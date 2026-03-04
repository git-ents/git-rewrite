# ✏️ `git-rewrite`

*Porcelain for rewriting repository history, trees, and (eventually) blobs.*

> [!CAUTION]
> This project is being actively developed!
> Despite this, semantic versioning rules will be respected.
> Expect frequent updates.

## Motivation

`git-rewrite` wraps the [`git-filter-tree`](https://crates.io/crates/git-filter-tree) plumbing library in a user-friendly `git`-style CLI.
It operates on the working tree's index so results are immediately staged and ready to commit.

## Installation

### CLI

Install the latest published release with `cargo install`.

```shell
cargo install --locked git-rewrite
```

To install the latest development version directly from the repository:

```shell
cargo install --git https://github.com/git-ents/git-rewrite
```

Once installed, if `~/.cargo/bin` is on your `PATH`, the command integrates naturally with `git`:

```shell
git rewrite -h
```

To generate and install a `man` page:

```shell
git rewrite --generate-man
```

## Usage

### `git rewrite tree`

Filter a Git tree, keeping only entries that match one or more glob patterns.
The result is written directly to the index, ready to commit.

Keep only Rust source files from `HEAD`:

```shell
git rewrite tree --only '**/*.rs'
```

Rewrite a specific commit, keeping two directories:

```shell
git rewrite tree abc1234 --only 'src/' --only 'tests/'
```

By default the working tree must be clean.
Pass `--allow-dirty` to skip that check.
