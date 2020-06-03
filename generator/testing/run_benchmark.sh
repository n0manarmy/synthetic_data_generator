#!/bin/bash
PERF_COMMAND="perf record -o testing/perf.data -F99 --call-graph dwarf"
CARGO_BUILD_COMMAND="cargo build --release"
EXEC_SWITCHES="--output file --count 1000000 --output-buffer 10000 --output-name testing/output.json"

cleanup () {
        echo "Cleaning up previous run"
        echo "rm testing/output.json"
        rm testing/output.json
}

echo "Run from the root project directory!"
echo
echo
echo "Select build to test:"
echo "1. Standard Release"
echo "2. x86_64-unknown-linux-musl"

read CHOICE

if [ $CHOICE -eq 1 ]; then
        cleanup
        $CARGO_BUILD_COMMAND
        $PERF_COMMAND target/release/synthetic_data_generator $EXEC_SWITCHES
elif [ $CHOICE -eq 2 ]; then 
        cleanup
        $CARGO_BUILD_COMMAND --target x86_64-unknown-linux-musl
        $PERF_COMMAND target/x86_64-unknown-linux-musl/release/synthetic_data_generator $EXEC_SWITCHES
else
        echo "Not a valid selection!. Exiting"
        exit
fi

perf report -i testing/perf.data
