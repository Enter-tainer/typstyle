plugin_install_path := "contrib/typstyle-embedded/assets/typstyle.wasm"

default:
    @just --list

build-plugin:
    cargo build -p typstyle-typlugin --release --target wasm32-unknown-unknown
    cp target/wasm32-unknown-unknown/release/typstyle_typlugin.wasm {{plugin_install_path}}
