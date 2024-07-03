#!/bin/bash

echo "simlink assets into crates"
mkdir crates/fixedpointmath/assets && cp assets/katex_header.html crates/fixedpointmath/assets/
mkdir crates/hyperdrive-wrappers/assets && cp assets/katex_header.html crates/hyperdrive-wrappers/assets/
mkdir crates/hyperdrive-math/assets && cp assets/katex_header.html crates/hyperdrive-math/assets/

echo "publish rust crates to crates.io"
cargo publish --token $1 -p fixedpointmath
BUILD_DISABLED=true cargo publish --token $1 -p hyperdrive-wrappers
cargo publish --token $1 -p hyperdrive-math

echo "remove crate asset directories"
rm -rf crates/fixedpointmath/assets
rm -rf crates/hyperdrive-wrappers/assets
rm -rf crates/hyperdrive-math/assets