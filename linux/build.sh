#!/bin/bash
cd syscall.rs
cargo build --release
cd ..

rustc --extern syscall=syscall.rs/target/release/libsyscall.rlib --emit obj \
    -C opt-level=3 -o hello.o src/main.rs
ld hello.o -o hello
strip hello
if [ -n "$SSTRIP_PATH" ]; then
    $SSTRIP_PATH hello;
fi
