#!/bin/sh

cargo build --all-features --release
cp target/release/mprs ~/.dotfiles/utils/.local/bin
cp -r mprs_config ~/.config
