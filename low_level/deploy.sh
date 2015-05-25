#!/bin/bash

RP_IP="172.31.133.136"
CARGO_PATH="../../local_cargo/bin/cargo"

${CARGO_PATH} clean
${CARGO_PATH} build --target=arm-unknown-linux-gnueabihf
scp target/arm-unknown-linux-gnueabihf/debug/rust_blink root@${RP_IP}:/home
