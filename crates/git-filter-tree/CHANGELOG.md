# Changelog

## [0.3.1](https://github.com/git-ents/git-rewrite/compare/git-filter-tree-v0.3.0...git-filter-tree-v0.3.1) (2026-03-08)


### Bug Fixes

* Use Git-compatible path separators ([c385840](https://github.com/git-ents/git-rewrite/commit/c3858401529c5fe7f66945abf7a6ad84bdac6366))

## [0.3.0](https://github.com/git-ents/git-rewrite/compare/git-filter-tree-v0.2.0...git-filter-tree-v0.3.0) (2026-03-06)


### Features

* Add new method `filter_by_predicate` ([8f80da2](https://github.com/git-ents/git-rewrite/commit/8f80da2e0c6ca119e726003a82680ae7b31fe1f5))


### Bug Fixes

* Remove OS-specific path separator from unit test ([7c39642](https://github.com/git-ents/git-rewrite/commit/7c39642b6ec0f24a7e5179d9d009f027b3704b38))

## [0.2.0](https://github.com/git-ents/git-rewrite/compare/git-filter-tree-v0.1.0...git-filter-tree-v0.2.0) (2026-03-04)


### Features

* Add new `filter_by_attributes` trait method ([fedd5fd](https://github.com/git-ents/git-rewrite/commit/fedd5fde2dc5cb498d8c8f5327268cce0debd061))
* Implement by-attribute filtering ([d761a56](https://github.com/git-ents/git-rewrite/commit/d761a56121ab26c753bf196bffd3708a4c20b022))


### Bug Fixes

* Add missing crate metadata ([2f252d4](https://github.com/git-ents/git-rewrite/commit/2f252d4a497198cef4ac83fade4d549de7566631))

## [0.1.0](https://github.com/git-ents/git-rewrite/compare/git-filter-tree-v0.0.6...git-filter-tree-v0.1.0) (2026-03-01)


### Features

* Add execution library for custom (future) CLI implementations ([7dc42c0](https://github.com/git-ents/git-rewrite/commit/7dc42c02942f9895d012d591f28030c8a5eb2940))
* Finalize `--only` API for porcelain commands ([7281313](https://github.com/git-ents/git-rewrite/commit/72813139db7e13a876cc99088916679e4d9c715c))
* Match CLI names to expected Git extension names ([5cf37da](https://github.com/git-ents/git-rewrite/commit/5cf37daa28e4ac9aad7198aeeed3fa9d73dd392c))

## [0.0.6](https://github.com/git-ents/git-rewrite/compare/git-filter-tree-v0.0.5...git-filter-tree-v0.0.6) (2026-03-01)


### Features

* Update crate description ([3d48542](https://github.com/git-ents/git-rewrite/commit/3d485420803e8a17b8754f920083a7e4565b297f))

## [0.0.5](https://github.com/git-ents/git-rewrite/compare/git-filter-tree-v0.0.4...git-filter-tree-v0.0.5) (2026-03-01)


### Reverts

* "chore(release): bump `git-rewrite` to v0.0.4 ([#24](https://github.com/git-ents/git-rewrite/issues/24))" ([8dc53a6](https://github.com/git-ents/git-rewrite/commit/8dc53a647349f0acd96cc74dfae28ff280725ce8))


### Miscellaneous Chores

* Pin next release ([b91f032](https://github.com/git-ents/git-rewrite/commit/b91f032cd75bcd64fd2a3b98c6bee474740b9073))
* Pin next release ([6d6e404](https://github.com/git-ents/git-rewrite/commit/6d6e404b7bcabc080929148fec27c125521e2b6f))

## [0.0.4](https://github.com/git-ents/git-rewrite/compare/git-filter-tree-v0.0.3...git-filter-tree-v0.0.4) (2026-03-01)


### Miscellaneous Chores

* Trigger next release ([671c58c](https://github.com/git-ents/git-rewrite/commit/671c58c7079460c3e8cc92775760d70b3b62df69))

## [0.0.3](https://github.com/git-ents/git-rewrite/compare/git-filter-tree-v0.0.2...git-filter-tree-v0.0.3) (2026-03-01)


### Features

* Update crate metadata ([b38c497](https://github.com/git-ents/git-rewrite/commit/b38c49726869f3db013633f8f8f34f0fd6dfe331))

## [0.0.2](https://github.com/git-ents/git-rewrite/compare/git-filter-tree-v0.0.1...git-filter-tree-v0.0.2) (2026-03-01)


### ⚠ BREAKING CHANGES

* rename top-level project to `git-rewrite`

### Features

* Add hidden `--generate-man` command to CLI ([ab897b8](https://github.com/git-ents/git-rewrite/commit/ab897b8fcad4f10e1962130bafe8d948c40903b7))


### Code Refactoring

* Rename top-level project to `git-rewrite` ([ab897b8](https://github.com/git-ents/git-rewrite/commit/ab897b8fcad4f10e1962130bafe8d948c40903b7))

## 0.0.1 (2026-03-01)


### Features

* Add `git-rewrite` CLI ([a49f564](https://github.com/git-ents/git-filter/commit/a49f564d7082083c1e1a22ea46e77112e6e58a02))
* Add CLI to `git-filter-tree` ([d2c4e9d](https://github.com/git-ents/git-filter/commit/d2c4e9d3cad71d1281fcade6373d2d9f3252fa2c))
* Add library crate: `git-filter-tree` ([21acf6d](https://github.com/git-ents/git-filter/commit/21acf6d206ca2af0b4726ba533fd7627d2a20a98))
