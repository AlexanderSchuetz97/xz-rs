#!/usr/bin/env bash
set -e


# 64 bit little endian
cross test --target x86_64-unknown-linux-gnu
cross test --target x86_64-unknown-linux-musl
cross test --release --target x86_64-unknown-linux-gnu
cross test --release --target x86_64-unknown-linux-musl

# 32 bit little endian
cross test --target i686-unknown-linux-gnu
cross test --target i686-unknown-linux-musl
cross test --release --target i686-unknown-linux-gnu
cross test --release --target i686-unknown-linux-musl

#64 bit big endian
cross test --target s390x-unknown-linux-gnu
cross test --release --target s390x-unknown-linux-gnu

#32 bit big endian
cross test --target powerpc-unknown-linux-gnu
cross test --release --target powerpc-unknown-linux-gnu

#Windows (this test requires mingw and wine and wine-binfmt to work)
cargo test --target x86_64-pc-windows-gnu
cargo test --target i686-pc-windows-gnu
cargo test --release --target x86_64-pc-windows-gnu
cargo test --release --target i686-pc-windows-gnu

#Miri
cargo +nightly miri test --package xz4rust --test tiny_stack test_tiny_stack -- --exact

#Valgrind
export VALGRINDFLAGS="--error-exitcode=1 --leak-check=full --show-leak-kinds=all"
cargo valgrind test --package xz4rust --test tiny_stack test_tiny_stack -- --exact

# Clippy checks and checks if it compiles in some combinations
cargo clippy -- -D warnings
cargo clippy --no-default-features -- -D warnings
cargo clippy --no-default-features --features alloc -- -D warnings
cargo clippy --no-default-features --features std -- -D warnings
cargo clippy --no-default-features --features crc64 -- -D warnings
cargo clippy --no-default-features --features bcj -- -D warnings
cargo clippy --no-default-features --features sha256 -- -D warnings

# Build examples
cargo build --examples

# Run bench just to make sure I didnt break it!
cargo +nightly bench

#Test some edge cases with usize/u32
export TEST_LARGE_SEED=true
cargo test --package xz4rust --test test_large_seeded --target x86_64-unknown-linux-gnu
cargo test --package xz4rust --test test_large_seeded --target i686-unknown-linux-gnu