# 🫥 `git-filter-tree`

*Filter Git tree objects in code, or on the command-line!*

## Motivation

When vendoring content from other Git repositories, you may want to filter the content you want with a `sparse-checkout`.
This is supported by Git submodules out-of-the-box, but submodules require extra steps for contributors.
Git subtrees are simpler for users —— they place content directly in the repository tree —— but they do not support `sparse-checkout`.
The `git-filter-tree` CLI (and accompanying `git_filter_tree` library) allows you to filter Git tree objects (tracked directories) by glob patterns.
This functionality is used as *plumbing* for the primary application of this [project](/), `git-filter`.

## Installation

### CLI

The `git-filter-tree` plumbing command can be installed with `cargo install`.

```shell
cargo install --git https://github.com/git-ents/git-filter git-filter-tree
```

If `~/.cargo/bin` is on your `PATH`, you can invoke the command with `git`.

```shell
git filter-tree -h
```

### Library

The `git_filter_tree` library can be added to your Rust project via `cargo add`.

```shell
cargo add --git https://github.com/git-ents/git-filter git-filter-tree
```
