#!/bin/bash

echo start test
sh ready_test_files.sh
cargo run --package track2line_gui --bin track2line_gui 
