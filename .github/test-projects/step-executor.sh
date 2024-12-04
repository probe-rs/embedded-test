#!/bin/sh
set -e
set -x

/home/probe-rs-runner/probe-rs run "$TARGET" $TARGET_CONFIG

exit 1