#!/bin/bash

echo "publish rust crates to crates.io"
cargo publish --token $1 -p file:://${PWD}/crates/fixedpointmath
BUILD_DISABLED=true cargo publish --token $1 -p file:://${PWD}/crates/hyperdrive-wrappers
cargo publish --token $1 -p file:://${PWD}/crates/hyperdrive-math