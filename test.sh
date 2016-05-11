#!/bin/bash -ex

cargo build --verbose
cargo test --verbose

if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
    cargo build --verbose --features quickcheck
    cargo test --verbose --features quickcheck
fi
