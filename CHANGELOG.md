# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.7](https://github.com/jcape/rxegy/compare/rxegy-v0.0.6...rxegy-v0.0.7) - 2025-03-09

### Added

- [**breaking**] remove callbacks traits, add builders, cleanup callback invocation.
- [**breaking**] remove session callbacks trait

### Other

- *(ci)* fix rust-src component name
- *(ci)* actually install miri
- *(ci)* fix cargo deny issues
- add CONTRIBUTING.md with style guide
- [**breaking**] fix clippy warnings about enum names
- sort workspace entries, remove unused anyhow
- devcontainer on mac, add miri, add extra checks

## [0.0.6](https://github.com/jcape/rxegy/compare/rxegy-v0.0.5...rxegy-v0.0.6) - 2025-03-03

### Added

- start work on equity streaming.

### Fixed

- disable markdownlint pre-commit hook

## [0.0.5](https://github.com/jcape/rxegy/compare/rxegy-v0.0.4...rxegy-v0.0.5) - 2025-03-03

### Added

- initial catalog impl
- add display impls for key and symbol
- use arc for shared callbacks
- finish us equites mics, make xx enum a feed
- more work on keylist catalog

### Fixed

- post-attach install cargo tools to the path.
- clippy warnings
- don't lose track of session after the first callback
- functional session creation / connection

### Other

- [**breaking**] clean up bindgen usage, remove system types
- use custom dockerfile to install libclang
- use local cargo/pre-commit cache.
- move cargo installs to post-create command.

## [0.0.4](https://github.com/jcape/rxegy/compare/rxegy-v0.0.3...rxegy-v0.0.4) - 2025-02-28

### Added

- [**breaking**] support common event fields

## [0.0.3](https://github.com/jcape/rxegy/compare/rxegy-v0.0.2...rxegy-v0.0.3) - 2025-02-27

### Added

- implement some session api, fix send/sync issue
- wire up session and builder
- start work on session objects.

### Fixed

- semver-checks installed from source
- fix clippy warnings
- add more error codes, pin session

### Other

- release automation on pr merge
- refactor session handling into a good place
- add markdownlint vsc extension
- fix cargo install of binstall
- remove broken binstall feature

## [0.0.2](https://github.com/jcape/rxegy/compare/rxegy-v0.0.1...rxegy-v0.0.2) - 2025-02-24

### Other

- fix chiclets in readme

## [0.0.1](https://github.com/jcape/rxegy/releases/tag/rxegy-v0.0.1) - 2025-02-24

### Added

- Initial release
