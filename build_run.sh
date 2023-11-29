#!/bin/bash

# don't continue if there's build errors
set -e
cargo check
set +e

# start the controlpad server and web server
cd controlpad_server
./start.sh &
cd ..

# build and run the game (getting ControlpadServer dependency from private repo)
export CARGO_NET_GIT_FETCH_WITH_CLI=true
$(
    cargo run 2>/dev/null
    cd controlpad_server
    ./start.sh -x
) &

# print out qr code to connect phones to web server at this computer's IP
ifconfig | grep 'inet ' | tail -1 | sed 's/.*inet \([^ ]*\).*/http:\/\/\1:3000/' | qrencode -t utf8 -m 2
