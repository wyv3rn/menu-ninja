#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: ./deploy.sh host"
    exit -1
fi

host=$1
target=x86_64-unknown-linux-musl
src=target/$target/release/menu-ninja

cargo build --target=$target --release &&
ssh $host systemctl --user stop menu-ninja || true
scp $src $host:.local/bin/menu-ninja
scp assets/menu-ninja.service $host:.config/systemd/user/menu-ninja.service
ssh $host systemctl --user daemon-reload || true
ssh $host systemctl --user restart menu-ninja
