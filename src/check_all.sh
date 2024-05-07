#!/bin/bash
echo "=========  fmt code  ========="
cargo fmt -- --check
echo "========= clippy all-targets ========="
cargo clippy --all-targets --all-features --tests --benches -- -D warnings
