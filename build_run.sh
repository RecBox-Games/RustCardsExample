#!/bin/bash

cd controlpad_server
./start.sh &
cd ..
export CARGO_NET_GIT_FETCH_WITH_CLI=true
$(
    cargo run
    cd controlpad_server
    ./start.sh -x
) &
