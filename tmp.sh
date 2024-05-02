#!/bin/bash

new_tests=$(diff main_tests.txt branch_tests.txt|awk -F'[>,]' '{gsub(/ /,"")} {print $2} '| sed '/^$/d')
if [ -n "$new_tests" ]; then
  echo "New tests found:"
  echo "$new_tests"
  while IFS= read -r test; do
    echo "Running test: $test"
    env HYPERDRIVE_FUZZ_RUNS=1000 HYPERDRIVE_FAST_FUZZ_RUNS=100000 cargo test $test --
  done <<< "$new_tests"
else
  echo "No new tests found."
fi