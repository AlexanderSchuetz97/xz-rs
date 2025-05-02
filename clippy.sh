#!/usr/bin/env bash
set -e

cargo clippy -- -D warnings
cargo clippy --no-default-features -- -D warnings
cargo clippy --no-default-features --features alloc -- -D warnings
cargo clippy --no-default-features --features std -- -D warnings
cargo clippy --no-default-features --features crc64 -- -D warnings
cargo clippy --no-default-features --features bcj -- -D warnings
cargo clippy --no-default-features --features sha256 -- -D warnings
cargo clippy --no-default-features --features delta -- -D warnings
