# ✏️ `git-rewrite`

*Porcelain for rewriting repository history, trees, and (eventually) blobs.*

<!-- rumdl-disable MD013 -->
[![CI](https://github.com/git-ents/git-filter/actions/workflows/CI.yml/badge.svg)](https://github.com/git-ents/git-filter/actions/workflows/CI.yml)
[![CD](https://github.com/git-ents/git-filter/actions/workflows/CD.yml/badge.svg)](https://github.com/git-ents/git-filter/actions/workflows/CD.yml)
<!-- rumdl-enable MD013 -->

> [!CAUTION]
> This project is being actively developed!
> Despite this, semantic versioning rules will be respected.
> Expect frequent updates.

## About

To support a more expansive usage of the Git object database — as is the goal for other projects within the [`git-ents`](https://github.com/git-ents) organization — new porcelain is needed.
This project provides porcelain that allows users to rewrite repository history as safely as possible.

The terms *porcelain* and *plumbing* are [borrowed from Git itself](https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain).
The user-facing `git-rewrite` command is the porcelain: a friendly interface that wraps lower-level functionality.
That lower-level functionality lives in plumbing crates, which are independently versioned and usable as libraries in their own right.

## Crates

| Crate | Kind | Description |
|---|---|---|
| [`git-rewrite`](porcelain/git-rewrite/) | porcelain | Rewrite repository history and trees via a `git`-style CLI |
| [`git-filter-tree`](plumbing/git-filter-tree/) | plumbing | Filter Git tree objects by glob pattern, as a library or CLI |
