
_default:
    @just --list

check:
    cargo clippy --locked -- -D warnings

fix:
    cargo clippy --fix --locked -- -D warnings

test:
    cargo test --locked
