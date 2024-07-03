#!/bin/bash

echo "simlink assets into crates"
ln -s assets crates/fixedpointmath
ln -s assets crates/hyperdrive-wrappers
ln -s assets crates/hyperdrive-math

echo "publish rust crates to crates.io"
cargo publish --token $1 -p fixedpointmath
BUILD_DISABLED=true cargo publish --token $1 -p hyperdrive-wrappers
cargo publish --token $1 -p hyperdrive-math