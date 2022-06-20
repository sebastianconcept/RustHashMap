#!/bin/bash
cargo test --features c-headers -- generate_headers --nocapture
cargo build --release