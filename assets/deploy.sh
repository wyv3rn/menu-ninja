#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: ./deploy.sh host"
    exit -1
fi

host=$1
src=target/aarch64-unknown-linux-musl/release/menu-ninja

cargo build --target=aarch64-unknown-linux-musl --release &&
ssh $host systemctl stop menu-ninja || true
scp $src $host:/usr/local/bin/menu-ninja
scp assets/menu-ninja.service $host:/lib/systemd/system/menu-ninja.service
ssh $host systemctl daemon-reload || true
ssh $host systemctl restart menu-ninja
