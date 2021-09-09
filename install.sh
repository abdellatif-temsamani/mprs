#!/bin/sh

cargo build --all-features --release
cp target/release/mprs ~/.dotfiles/utils/.local/bin
