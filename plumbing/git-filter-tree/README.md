# 🫥 `git-filter-tree`

*Filter Git tree objects in code, or on the command-line!*

> [!CAUTION]
> This project is being actively developed!

## Motivation

When fetching content from upstream repositories, you may want to exclude specific patterns.
You may use `sparse-checkout` in certain contexts, but this alone is not sufficient to arbitrarily select content across a repository.
With the `git filter-tree` plumbing command, or with the `git_filter_tree` library, you can arbitrarily filter content in any Git tree.
Repository boundaries are a myth!
The `git-filter-tree` CLI (and accompanying `git_filter_tree` library) allows you to filter Git tree objects (tracked directories) by glob patterns.
This functionality is used as *plumbing* for the primary application of this [project](/), `git-rewrite`.

## Installation

### CLI

The `git-filter-tree` plumbing command can be installed with `cargo install`.

```shell
cargo install --locked git-filter-tree
```

To install the latest development version, install from Git.

```shell
cargo install --git https://github.com/git-ents/git-rewrite git-filter-tree
```

If `~/.cargo/bin` is on your `PATH`, you can invoke the command with `git`.

```shell
git filter-tree -h
```

To generate and install `man` entries for the CLI, use the `--generate-man` command option once.

```shell
git filter-tree --generate-man
```

### Library

The `git_filter_tree` library can be added to your Rust project via `cargo add`.

```shell
cargo add --git https://github.com/git-ents/git-rewrite git-filter-tree
```
