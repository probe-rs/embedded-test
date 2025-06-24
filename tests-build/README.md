# Test that our proc macros work as expected

This directory contains UI tests for our proc macros.

Unfortunately, we cannot use the excellent `trybuild` crate, as it does not support compiling for a target other than
the host.

Test cases are defined in the cases directory. Inspired
by https://github.com/diondokter/device-driver/tree/kdl-input/tests
