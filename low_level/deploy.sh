#!/bin/bash

RP_IP="172.31.133.136"

../../local_cargo/bin/cargo clean
../../local_cargo/bin/cargo build --target=arm-unknown-linux-gnueabihf
scp target/arm-unknown-linux-gnueabihf/debug/rust_blink root@${RP_IP}:/home
