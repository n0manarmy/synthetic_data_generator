#!/bin/bash
cargo run -- \
        --output port \
        --count 100 \
        --port-rate variable \
        --output-format json
