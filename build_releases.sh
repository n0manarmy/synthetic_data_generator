#!/bin/bash
echo "Cleaning up old releases"
rm releases/*
echo "Building default release and x86_64-unknown-linux-musl release"
CARGO_BUILD_COMMAND="cargo build --release"

#Standard build release
echo $CARGO_BUILD_COMMAND
$CARGO_BUILD_COMMAND

#Centos distributions because of the older libc? versioning
echo $CARGO_BUILD_COMMAND --target x86_64-unknown-linux-musl
$CARGO_BUILD_COMMAND --target x86_64-unknown-linux-musl