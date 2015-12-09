#!/bin/sh
rustc -C opt-level=3 --emit obj main.rs
ld main.o -o hello
strip hello
$SSTRIP_PATH/sstrip hello
