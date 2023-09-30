[private]
@help:
    just --list --unsorted

# run tests
@test:
    cargo test

# run tests with generated test coverage
@test-coverage:
    cargo tarpaulin --out Html --skip-clean --exclude-files src/main.rs

# build in release mode
@build:
    cargo build --release

# generate shell completions
@generate-completions: build
    mkdir -p completions
    docopt-compgen target/release/hs --namespace '' > completions/hs.bash
