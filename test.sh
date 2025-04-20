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
cargo +nightly miri test --package xz-rs --test tiny_stack test_tiny_stack -- --exact

#Valgrind
export VALGRINDFLAGS="--error-exitcode=1 --leak-check=full --show-leak-kinds=all"
cargo valgrind test --package xz-rs --test tiny_stack test_tiny_stack -- --exact

#Test some edge cases with usize/u32
export TEST_LARGE_SEED=true
cargo test --package xz-rs --test test_large_seeded --target x86_64-unknown-linux-gnu
cargo test --package xz-rs --test test_large_seeded --target i686-unknown-linux-gnu

# Clippy checks and checks if it compiles in some combinations
cargo clippy
cargo clippy --no-default-features
cargo clippy --no-default-features --features alloc
cargo clippy --no-default-features --features std
cargo clippy --no-default-features --features crc64
cargo clippy --no-default-features --features bcj
cargo clippy --no-default-features --features sha256