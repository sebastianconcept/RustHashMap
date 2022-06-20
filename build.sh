#!/bin/bash
cargo test --features c-headers -- generate_headers --nocapture
cargo build
# gcc -Itarget/debug -Ltarget/debug  target/debug/main.c  -lrusthashmap -o ffiexercise