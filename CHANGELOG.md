# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]

[Unreleased]: https://github.com/althonos/embedded-picofont/compare/v0.2.1...HEAD

### Changed
- **(breaking)** Bumped `embedded-graphics` dependency to `v0.7.0`.
- **(breaking)** The `FontPico` struct is now a `const` called `PICO_FONT`.

## [0.2.1] - 2020-11-27

[0.2.1]: https://github.com/althonos/embedded-picofont/compare/v0.2.0...v0.2.1

### Fixed
- Broken link to the GitHub page in `Cargo.toml` manifest.

### Changed
- Bumped `lodepng` build dependency to `v3.2`.
- Bumped `bitvec` build dependency to `v0.19.4`.


## [0.2.0] - 2020-04-18

[0.2.0]: https://github.com/althonos/embedded-picofont/compare/v0.1.1...v0.2.0

### Changed
- Bumped `embedded-graphics` dependency to `v0.6.0`.
### Removed
- `text_pico` macro (not useful with `embedded-graphics` new API).


## [0.1.1] - 2019-10-30

[0.1.1]: https://github.com/althonos/embedded-picofont/compare/v0.1.0...v0.1.1

### Changed
- Bumped `bitvec` build dependency to `v0.15.0`.


## [0.1.0] - 2019-07-02

[0.1.0]: https://github.com/althonos/embedded-picofont/compare/2ae49c2...v0.1.0

Initial release.
