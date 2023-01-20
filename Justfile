
_default:
    @just --list

check:
    cargo clippy --locked -- -D warnings

test:
    cargo test --locked
