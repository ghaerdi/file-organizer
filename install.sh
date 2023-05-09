#!/bin/sh
cargo build --release

if [[ $OSTYPE == 'darwin'* ]]; then
    target_path="/usr/local/bin/organizer"
else
    target_path="/usr/bin/organizer"
fi

if [ "$(id -u)" != "0" ]; then
    sudo cp ./target/release/file-organizer "$target_path"
    sudo chmod +x "$target_path"
else
    cp ./target/release/file-organizer "$target_path"
    chmod +x "$target_path"
fi
