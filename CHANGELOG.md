# Changelog

## [0.1.0](https://github.com/git-ents/git-rewrite/compare/v0.0.6...v0.1.0) (2026-03-01)


### Features

* Add execution library for custom (future) CLI implementations ([7dc42c0](https://github.com/git-ents/git-rewrite/commit/7dc42c02942f9895d012d591f28030c8a5eb2940))
* Match CLI names to expected Git extension names ([5cf37da](https://github.com/git-ents/git-rewrite/commit/5cf37daa28e4ac9aad7198aeeed3fa9d73dd392c))


### Miscellaneous Chores

* Remove pre-release Release Please specification ([3e56890](https://github.com/git-ents/git-rewrite/commit/3e568908753b8eb8fb9b83bb83d728612349dd91))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * git-filter-tree bumped from 0.0.6 to 0.0.7

## [0.0.6](https://github.com/git-ents/git-rewrite/compare/v0.0.5...v0.0.6) (2026-03-01)


### Features

* Update crate description ([3d48542](https://github.com/git-ents/git-rewrite/commit/3d485420803e8a17b8754f920083a7e4565b297f))


### Miscellaneous Chores

* Fix release step with proper tag filtering ([93bdb33](https://github.com/git-ents/git-rewrite/commit/93bdb331cc152197bc022c8da57952ef64abe7d5))
* Fix version syncing in Release Please ([cb7699c](https://github.com/git-ents/git-rewrite/commit/cb7699c7050223877bf056ba6b63e7c2b235063f))
* Remove temporary version pinning ([c612198](https://github.com/git-ents/git-rewrite/commit/c6121980708429c66d5012adf9f09c3d181ae338))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * git-filter-tree bumped from 0.0.5 to 0.0.6

## [0.0.5](https://github.com/git-ents/git-rewrite/compare/v0.0.4...v0.0.5) (2026-03-01)


### Reverts

* "chore(release): bump `git-rewrite` to v0.0.4 ([#24](https://github.com/git-ents/git-rewrite/issues/24))" ([8dc53a6](https://github.com/git-ents/git-rewrite/commit/8dc53a647349f0acd96cc74dfae28ff280725ce8))


### Miscellaneous Chores

* Move back to single release pull request ([3ee5715](https://github.com/git-ents/git-rewrite/commit/3ee5715e536eae8fb06eb21d3cd89d6ffadf9067))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * git-filter-tree bumped from 0.0.4 to 0.0.5

## [0.0.3](https://github.com/git-ents/git-rewrite/compare/v0.0.2...v0.0.3) (2026-03-01)


### Features

* Update crate metadata ([b38c497](https://github.com/git-ents/git-rewrite/commit/b38c49726869f3db013633f8f8f34f0fd6dfe331))


### Miscellaneous Chores

* Remove manual release pinning from configuration ([2dbf253](https://github.com/git-ents/git-rewrite/commit/2dbf253eb87ca0673446f851f0b66f1fd82173f8))

## [0.0.2](https://github.com/git-ents/git-rewrite/compare/v0.0.1...v0.0.2) (2026-03-01)


### ⚠ BREAKING CHANGES

* rename top-level project to `git-rewrite`

### Features

* Add hidden `--generate-man` command to CLI ([ab897b8](https://github.com/git-ents/git-rewrite/commit/ab897b8fcad4f10e1962130bafe8d948c40903b7))


### Code Refactoring

* Rename top-level project to `git-rewrite` ([ab897b8](https://github.com/git-ents/git-rewrite/commit/ab897b8fcad4f10e1962130bafe8d948c40903b7))

## 0.0.1 (2026-03-01)


### Features

* Add `git-rewrite` CLI ([a49f564](https://github.com/git-ents/git-filter/commit/a49f564d7082083c1e1a22ea46e77112e6e58a02))
* Add library crate: `git-filter-tree` ([21acf6d](https://github.com/git-ents/git-filter/commit/21acf6d206ca2af0b4726ba533fd7627d2a20a98))
* Add man-page generation via `clap_mangen` ([2a1a4b6](https://github.com/git-ents/git-filter/commit/2a1a4b6b6cf95bad79b4bf705d2a831490f3fab9))
* Enable publishing for initial release ([ed7a912](https://github.com/git-ents/git-filter/commit/ed7a912d30d45fdd1ef92a33f7cdfb35870ca626))


### Reverts

* "chore: add `main` to allowed commit scopes" ([957395e](https://github.com/git-ents/git-filter/commit/957395e61cd5ca12bc5102eb8db4b6e06ff23290))
