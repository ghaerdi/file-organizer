#!/bin/sh
cargo build --release
cp ./target/release/file-organizer /usr/bin/organizer
