build +args="":
    cargo build {{args}}

_clippy target:
    @cd {{target}} && cargo check && cargo clippy

doc target +args="":
    @cd {{target}} && cargo doc {{args}}

lint:
    @just _clippy argot
    @just _clippy slang-cli
    @just _clippy vm
    @just _clippy instructor
    @just _clippy assembler

test:
    cargo test

argotc +args="":
    @cargo run --bin argotc -- {{args}}

run +args="":
    @cargo run --bin slang -- {{args}}

trace +args="":
    @RUST_LOG=trace just run {{args}}
