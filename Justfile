test:
    cargo test
    cargo test --features "autoreloader"

test coverage:
    cargo tarpaulin

dev install:
    cargo install cargo-tarpaulin
