#!/bin/bash
export RUSTFLAGS="-C target-cpu=native"
cargo run --release
