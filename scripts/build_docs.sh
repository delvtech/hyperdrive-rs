#!/bin/bash

# Inject an html header into the docs build that allows us to render LaTeX
# formulae using KaTeX.

# Need the absolute path for this script.
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Location of the injection html file.
HEADER_PATH="$SCRIPT_DIR/../assets/katex_header.html"

# Allow for additional flags or arguments.
RUSTDOCFLAGS="--html-in-header $HEADER_PATH" cargo doc --no-deps --workspace $@