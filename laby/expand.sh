#!/bin/sh
# Expands test macros on file change; useful for debugging. Requires cargo-watch.
cargo watch -x 'expand --test test'
