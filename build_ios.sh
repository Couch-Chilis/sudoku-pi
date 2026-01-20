#/usr/bin/env bash

cargo build --bin sudoku_pi --release --target aarch64-apple-ios

mkdir -p target/aarch64-apple-ios/release/bundle/ios/
lipo target/aarch64-apple-ios/release/sudoku_pi -create -output target/aarch64-apple-ios/release/bundle/ios/sudoku-pi.app
