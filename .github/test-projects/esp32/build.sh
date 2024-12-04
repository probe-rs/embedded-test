#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <chip> <target>"
    exit 1
fi

set -x
set -e

# to show a possible error in the output
cargo test --locked --target "$2" --features "$1" --release --test example_test --no-run

# Copy artifact to output directory
exec=$(cargo test --locked --target "$2" --features "$1" --release --test example_test --no-run --message-format=json | jq -r 'select(.executable and .target.kind==["test"])|.executable')
mkdir -p output
cp "$exec" output/"$1"
cp expected_test_results.txt output/
cp expected_rtt_logs.txt output/