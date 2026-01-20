#/usr/bin/env bash

cargo rustc --crate-type staticlib --lib --target aarch64-apple-ios-sim

cargo bundle --target aarch64-apple-ios-sim
