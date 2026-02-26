# 🚽 Plumbing Commands

*...and libraries!*

> [!NOTE]
> The term *plumbing* is [borrowed](https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain) from Git itself!

## Motivation

This project's top-level crate, [`git-filter`](/), wraps several library targets and plumbing commands (binaries) into a single user interface.
Most of the underlying functionality is implemented as workspace crates.
This allows for more precisely scoped versioning and testing.
