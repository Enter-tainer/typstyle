# typstyle-typlugin

WebAssembly plugin for embedding typstyle as a Typst plugin.

## Overview

This crate provides a WebAssembly interface for typstyle, allowing it to be used as an embedded plugin within Typst documents. It exposes the core formatting functionality through WASM exports.

## Building

```bash
cargo build -p typstyle-typlugin --release --target wasm32-unknown-unknown
```

Or use the justfile:

```bash
just build-plugin
```

## Usage

This crate is primarily used through the embedded Typst package in `contrib/typstyle-embedded/`. The compiled WASM binary is automatically copied to the package assets.
