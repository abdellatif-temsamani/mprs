#!/bin/sh

cargo build --all-features --release
cp -v target/release/mprs ~/.dotfiles/utils/.local/bin
cp -vr mprs_config ~/.config
