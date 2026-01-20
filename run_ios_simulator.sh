#/usr/bin/env bash

BUNDLE_ID="nl.couch-chilis.Sudoku-Pi"

xcrun simctl boot "iPhone 17"
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app
xcrun simctl install booted target/aarch64-apple-ios-sim/debug/bundle/ios/sudoku-pi.app
xcrun simctl launch --console booted "$BUNDLE_ID"
