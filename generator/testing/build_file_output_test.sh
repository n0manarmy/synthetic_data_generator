#!/bin/bash
rm testing/output_test.json
cargo run -- \
        --output-name testing/output_test.json \
        --count 10000 \
        --validate-json false \
        --output-dst file \
        --output-buffer-size 100

