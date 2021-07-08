@_help:
    just --list --unsorted

test:
    cargo test

test-coverage:
    cargo tarpaulin --out Html --skip-clean --exclude-files src/main.rs
