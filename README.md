# ✏️ `git-rewrite`

*Porcelain for rewriting repository history, trees, and (eventually) blobs.*

> [!CAUTION]
> This project is being actively developed!

<!-- rumdl-disable MD013 -->
[![CI](https://github.com/git-ents/git-filter/actions/workflows/CI.yml/badge.svg)](https://github.com/git-ents/git-filter/actions/workflows/CI.yml)
[![CD](https://github.com/git-ents/git-filter/actions/workflows/CD.yml/badge.svg)](https://github.com/git-ents/git-filter/actions/workflows/CD.yml)
<!-- rumdl-enable MD013 -->

## Motivation

To support a more expansive usage of the Git object database, as is the goal for other projects within the [`git-ents`](https://github.com/git-ents) organization, new porecelain is needed.
This project provides porcelain that allows users to rewrite repository history as safely as possible.

## Installation

### CLI

The `git-rewrite` porecelain command can be installed with `cargo install`.

```shell
cargo install --locked git-rewrite
```

To install the latest development version, install from Git.

```shell
cargo install --git https://github.com/git-ents/git-rewrite
```

If `~/.cargo/bin` is on your `PATH`, you can invoke the command with `git`.

```shell
git rewrite -h
```
