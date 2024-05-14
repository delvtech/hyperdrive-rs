# hyperdrivepy

Hyperdrivepy is the Python bindings for the Rust [hyperdrive](https://github.com/delvtech/hyperdrive) implementation.

## Hyperdrivepy install

Hyperdrivepy can be installed via pip: `python -m pip install hyperdrivepy`

## Build Types

PoolInfo and PoolConfig are passed into many of the functions.
These are built from the Hyperdrive abi json with pypechain.
After compiling in the `hyperdrive` repo, run the following
from the hyperdrive-rs project root:

```shell
pip install --upgrade -r requirements-dev.txt
pypechain --line-length 120 --output-dir bindings/hyperdrivepy/python/hyperdrivepy/pypechain_types path/to/hyperdrive/out/IHyperdrive.sol/
```


## Disclaimer

The language used in this codebase is for coding convenience only, and is not
intended to, and does not, have any particular legal or regulatory significance.
