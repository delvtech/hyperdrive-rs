#!/bin/bash

echo "publish rust crates to crates.io"
cargo publish --token $1 -p fixedpointmath
BUILD_DISABLED=true cargo publish --token $1 -p hyperdrive-wrappers
cargo publish --token $1 -p hyperdrive-math