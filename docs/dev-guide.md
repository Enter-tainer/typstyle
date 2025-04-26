# Development Guide

This guide walks you through setting up your environment, building the project, running tests, and generating documentation for **typstyle**.

## Prerequisites

- Rust (stable toolchain) with [cargo](https://doc.rust-lang.org/cargo/).
- Node.js and [yarn](https://yarnpkg.com/) (for web assets, if modifying `web/`)

## Initial Setup

To build Typstyle CLI:

```sh
git clone https://github.com/Enter-tainer/typstyle.git
cd typstyle
# Debug
cargo build
# Release
cargo build --release
```

To run tests, you need to install [cargo-nextest](https://nexte.st/) and [cargo-insta](https://insta.rs/).

## Workspace Layout

- `crates/typstyle/` — formatter CLI
- `crates/typstyle-core/` — core formatting logic
- `crates/typstyle-consistency/` — framework for consistency tests
- `tests/` — code and fixtures for testing core library features
- `web/` — frontend demo

## Running Tests

When you change the core library or fixtures, you need to update snapshots with insta.

When you change the CLI, you need to add or update tests in [typstyle/tests](./crates/typstyle/tests).
Specially, when you add a style arg, you need to write a testcase in [test_style_args.rs](crates/typstyle/tests/test_style_args.rs) to ensure it works.

- List all tests:

  ```sh
  cargo nextest list --workspace
  ```

- Run all tests and review snapshots:

  ```sh
  cargo nextest run --workspace --no-fail-fast
  cargo insta review
  ```

- Run snapshot tests only:

  ```sh
  cargo nextest run --workspace -E 'test([snapshot])' --no-fail-fast --no-default-features
  cargo insta review
  ```

- Run tests excluding end-to-end (e2e):

  ```sh
  cargo nextest run --workspace -E '!test([e2e])' --no-fail-fast
  ```

- Run tests for CLI:

  ```sh
  cargo nextest run -p typstyle --no-fail-fast
  ```

### Snapshot Review

When snapshot tests change, review and accept updates:

```sh
cargo insta review
# or simply
cargo insta accept
```

## Running Benchmarks

Benchmarks use [Criterion.rs](https://github.com/bheisler/criterion.rs).

- List benchmarks:

  ```sh
  cargo bench --workspace -- --list
  ```

- Run all benches:

  ```sh
  cargo bench --workspace
  ```

You can check `target/criterion/report` to see the reports.
