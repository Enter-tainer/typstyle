#import "./book.typ": *
#import "../deps.typ": *

#show: book-page.with(title: "Developer Guide")

= Developer Guide

This guide covers environment setup, building, testing, and documentation for typstyle.

== Prerequisites

- Rust stable toolchain with #link("https://doc.rust-lang.org/cargo/")[cargo]
- Node.js and #link("https://yarnpkg.com/")[yarn] (for web assets)
- #link("https://nexte.st/")[cargo-nextest] and #link("https://insta.rs/")[cargo-insta] (for testing)
- #link("https://github.com/Myriad-Dreamin/shiroa")[shiroa] (for documentation)

== Initial Setup

Clone and build the project:

```bash
git clone https://github.com/Enter-tainer/typstyle.git
cd typstyle
cargo build              # Debug build
cargo build --release    # Release build
```

Install required tools:
```bash
# For testing
cargo binstall cargo-nextest cargo-insta
# For building wasm
cargo binstall wasm-pack
# For documentation
cargo binstall shiroa
```

== Workspace Layout

- `crates/typstyle/` — formatter CLI
- `crates/typstyle-core/` — core formatting logic
- `crates/typstyle-consistency/` — consistency test framework
- `crates/typstyle-typlugin/` — typst plugin for embedded usage
- `crates/typstyle-wasm/` — wasm bindings
- `tests/` — integration tests and fixtures
- `docs/` — documentation source (based on shiroa and written in typst)
- `contrib/typstyle-embedded/` — typstyle as typst package

== Building Components

=== CLI and Core

```bash
cargo build                     # Debug build
cargo build --release           # Release build
cargo build -p typstyle         # CLI only
cargo build -p typstyle-core    # Core only
```

=== WebAssembly Plugin

For the embedded Typst plugin:
```bash
just build-plugin
# or manually:
cargo build -p typstyle-typlugin --release --target wasm32-unknown-unknown
```

== Running Tests

#callout.important[
  Update snapshots with `cargo insta` when changing core library or fixtures.

  Add CLI tests to `crates/typstyle/tests/` when modifying CLI behavior.
  For style arguments, add tests to `test_style_args.rs`.
]

=== Test Commands

List all tests:
```bash
cargo nextest list --workspace
```

Run all tests and review snapshots:
```bash
cargo nextest run --workspace --no-fail-fast
cargo insta review
```

Snapshot tests only:
```bash
cargo nextest run --workspace -E 'test([snapshot])' \
  --no-fail-fast --no-default-features
cargo insta review
```

Exclude end-to-end tests:
```bash
cargo nextest run --workspace -E '!test([e2e])' --no-fail-fast
```

CLI tests only:
```bash
cargo nextest run -p typstyle --no-fail-fast
```

Integration tests:
```bash
cargo nextest run -p tests --no-fail-fast
```

=== Snapshot Management

Review and accept snapshot changes:
```bash
cargo insta review    # Interactive review
cargo insta accept    # Accept all changes
```

=== Benchmarks

Using #link("https://github.com/bheisler/criterion.rs")[Criterion.rs]:

```bash
cargo bench --workspace -- --list    # List benchmarks
cargo bench --workspace              # Run all benchmarks
```

View HTML reports at `target/criterion/report/index.html`.

== Documentation

=== Building Documentation

The documentation uses #link("https://github.com/Myriad-Dreamin/shiroa")[shiroa] to build from Typst source files.

Development server with auto-reload:
```bash
just dev-docs
# or manually:
shiroa serve docs/pages -w . --mode static-html
```

Build static documentation:
```bash
shiroa build docs/pages
```

Generate CLI help text:
```bash
just generate-cli-help
# or manually:
cargo run -p typstyle -- --help > docs/assets/generated/cli-help.txt
```

=== Documentation Structure

The documentation is organized as follows:
- `docs/pages/book.typ` — main book configuration and metadata
- `docs/pages/` — individual documentation pages
- `docs/packages/` — dependent third party packages not in universe
- `docs/templates/` — page templates and components
- `docs/assets/` — static assets and generated content

==== Using Callout Components

#import callout: *

The documentation supports various callout types for highlighting important information:

#note[This is a standard note callout for general information.]

#important[This is an important callout for critical information that users must be aware of.]

#warning[This is a warning callout for potential issues or dangerous operations.]

#tip[This is a tip callout for helpful suggestions and best practices.]

#caution[This is a caution callout for operations that require careful consideration.]

You can also use custom titles:

#callout(type: "important", title: "Custom Title")[
  This callout has a custom title instead of the default "Important".
]

== Development Workflow
