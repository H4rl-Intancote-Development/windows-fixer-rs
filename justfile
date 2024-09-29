@default:
    just --list

@build:
    cargo build --release

@build-debug:
    cargo build

@run args="":
    cargo run --release -- {{args}}

@run-debug:
    cargo run

@clean:
    cargo clean