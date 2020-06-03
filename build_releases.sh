#!/bin/bash
echo "Cleaning up old releases"
rm releases/*
echo "Building default release and x86_64-unknown-linux-musl release"
CARGO_BUILD_COMMAND="cargo build --release"
echo $CARGO_BUILD_COMMAND
$CARGO_BUILD_COMMAND
#cp target/release/file_generator ./releases/file_generator_default
echo $CARGO_BUILD_COMMAND --target x86_64-unknown-linux-musl
$CARGO_BUILD_COMMAND --target x86_64-unknown-linux-musl
#cp target/x86_64-unknown-linux-musl/release/file_generator ./releases/file_generator_x86_64-unknown
