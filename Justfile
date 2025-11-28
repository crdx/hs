set quiet := true
set shell := ["bash", "-cu", "-o", "pipefail"]

[private]
help:
    just --list --unsorted

fmt:
    echo 'Nothing to format'

test:
    cargo test

cov:
    cargo tarpaulin --out Html --skip-clean --exclude-files src/main.rs

build:
    cargo build --release

generate-completions: build
    mkdir -p completions
    docopt-compgen target/release/hs --namespace '' > completions/hs.bash
