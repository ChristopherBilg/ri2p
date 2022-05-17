# ri2p

Rust implementation of the i2p client/server/router protocols

## Status

![ci-cd](https://github.com/ChristopherBilg/ri2p/actions/workflows/ci-cd.yml/badge.svg)
![mdBook](https://github.com/ChristopherBilg/ri2p/actions/workflows/gh-pages.yml/badge.svg)
![devskim](https://github.com/ChristopherBilg/ri2p/actions/workflows/devskim.yml/badge.svg)

## Common Commands

- `cargo build`: Builds the ri2p binary
- `cargo run`: Runs the ri2p binary
- `cargo test`: Runs all test functions, in the ./tests directory, that contain the #[test] attribute
- `cargo doc [--open]`: Builds the automatically generated ri2p Rust documentation, and optionally opens it in a web browser

- `cargo fmt`: Standard formatter for Rust codestyle
- `cargo clippy [--fix]`: Standard linter for Rust, and optionally will automatically fix linted issues

- `mdbook build`: Builds the ri2p mdbook locally
- `mdbook serve`: Serves the ri2p mdbook at http://localhost:3000, and rebuilds it on changes

## CI/CD Information

GitHub is the central CI/CD platform for the ri2p project. CI/CD will occur only when modifications are made (pull-requests) to the **master** branch.

## Additional Important Information

- [Code of Conduct Guidelines](./CODE_OF_CONDUCT.md)
- [Contributing Guidelines](./CONTRIBUTING.md)
- [Open Source License](./LICENSE.md)
- [Security (Vulnerability) Policy](./SECURITY.md)
- [Major Authors & Contributors](./AUTHORS.md)
- [Changelog (SemVer)](./CHANGELOG.md)

## Developers

If you would like to work on this project, please feel free to check the [issues](https://github.com/ChristopherBilg/ri2p/issues) for a list of the current items that we need to be worked on. If you would like to work on any of them, please contact ri2p in the [discussions](https://github.com/ChristopherBilg/ri2p/discussions) area and I will mark a task for you.

### Documentation

Developers can find documentation for ri2p in the [docs/](./docs/) directory, or hosted online at [chrisbilger.com/ri2p/](https://chrisbilger.com/ri2p/).

The `cargo doc` documentation that is automatically generated by Cargo can be viewed by running `cargo doc --open`, or hosted online at [chrisbilger.com/ri2p/doc/doc/ri2p](https://chrisbilger.com/ri2p/doc/doc/ri2p).

## Roadmap & Goals

### v1.0.0

Minimally viable i2p router.

### v2.0.0

Add in router UI. (This will probably be a web server that serves a static site and periodic data refreshes.)

### v3.0.0

Remove all unnecessary dependencies in order to reduce attack-surface. This also includes re-reading all source code in order to pinpoint potential security vulnerabilities.

### v4.0.0+

TBD. Versions at this level will be feature additions and possibly a "module" system that allows the addition of external modules for additional/enhanced features and functionality.
