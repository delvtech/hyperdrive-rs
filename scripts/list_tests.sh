#!/bin/bash

cargo test -- --list |
  sed -n '/: test$/p' |
  sed 's/^ *//;s/: test$//' |
  sed 's/\([^:]*\)::\([^:]*\)/\1::\2/' |
  sed 's/ /\\ /g' |
  grep -v -E '^crates/hyperdrive-wrappers/src/wrappers/'
