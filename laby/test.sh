#!/bin/sh
# Continuously runs cargo tests on file change. Requires cargo-watch.
cargo watch -x 'test -- --nocapture'
