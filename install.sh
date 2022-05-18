#!/bin/sh
cargo build --release
# if the user is not root, then we need to run the script as root to install and copy the binary to /usr/bin
# cp ./target/release/file-organizer /usr/bin/organizer
# chmod +x /usr/bin/organizer
if [ "$(id -u)" != "0" ]; then
    sudo cp ./target/release/file-organizer /usr/bin/organizer
    sudo chmod +x /usr/bin/organizer
else
    cp ./target/release/file-organizer /usr/bin/organizer
    chmod +x /usr/bin/organizer
fi
