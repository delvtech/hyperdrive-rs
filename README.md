[![Tests](https://github.com/delvtech/hyperdrive-rs/actions/workflows/rust_test.yml/badge.svg)](https://github.com/delvtech/hyperdrive-rs/actions/workflows/rust_test.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/delvtech/elf-contracts/blob/master/LICENSE)
[![Static Badge](https://img.shields.io/badge/DELV-Terms%20Of%20Service-orange)](https://delv-public.s3.us-east-2.amazonaws.com/delv-terms-of-service.pdf)

<img src="https://raw.githubusercontent.com/delvtech/hyperdrive-rs/main/assets/icons/hyperdrive_winter.webp" width="800" alt="hyperdrive"><br>

# Hyperdrive

Hyperdrive is an automated market maker that enables fixed-rate markets to be
built on top of arbitrary yield sources. Hyperdrive provides several novel
features for fixed-rate AMMs including:

- Terms on Demand: Hyperdrive allows for minting to be a part of the
  AMM, where the AMM essentially underwrites a new term for the user
  whenever they open a position. The user is not constrained to purchasing,
  selling, or minting into preexisting terms that are partially matured.
- Continuous Liquidity: Hyperdrive pools never expire and underwrite a
  variety of fixed and variable rate terms with differing maturity dates. LPs
  can provide liquidity once without needing to roll their liquidity over to
  new terms.
- Single-Sided Liquidity: Hyperdrive liquidity providers are only required to
  provide base assets. The fact that LPs don't need to mint bonds to provide
  liquidity improves the capital efficiency and UX of providing liquidity to
  fixed-rate markets.

# Hyperdrive-rs

Hyperdrive-rs is a Rust library that mirrors the functionality of the
Hyperdrive Solidity smart contracts. Hyperdrive-rs includes differential testing
against smart contracts for implemented functions, which demonstrates parity
between the two implementations. This is a work-in-progress and not all
Hyperdrive features are present.

Python bindings for this library can be accessed via [hyperdrivepy](https://pypi.org/project/hyperdrivepy/)
or see [agent0](https://pypi.org/project/agent0/) for Python applications that utilize `hyperdrivepy`.

Typescript bindings for this library can be found [here](https://js.hyperdrive.box/).

# Resources

The [Hyperdrive docs](https://docs-delv.gitbook.io/hyperdrive) include documentation
on how to use Hyperdrive to source and provide liquidity, documentation for
developers seeking to use Hyperdrive programatically, and documentation for
developers that want to integrate Hyperdrive with a yield source.

The [Hyperdrive Whitepaper](https://github.com/delvtech/hyperdrive/blob/main/docs/Hyperdrive_Whitepaper.pdf)
describes the technical details underlying how Hyperdrive mints terms on demand, enables LPs to provide
everlasting liquidity, and explains how the AMM's pricing model works.

# Gettings Started

## Pre-requisites

### Install forge

This repository makes use of [foundry](https://github.com/foundry-rs/foundry) to
build and test smart contracts against the Rust implementation. If you haven't
already, you will need to [Install
forge](https://github.com/foundry-rs/foundry#installatio://github.com/foundry-rs/foundry#installation).

## Install hyperdrive-rs

`hyperdrive-rs` is composed of three published packages:

- [`fixedpointmath`](https://crates.io/crates/fixedpointmath)
- [`hyperdrive-wrappers`](https://crates.io/crates/hyperdrive-wrappers)
- [`hyperdrive-math`](https://crates.io/crates/hyperdrive-math)

You can install them for production runs via `cargo add [package]`, or add it to your `cargo.toml`.

If you'd like to set up a development environment, clone the repo and build with `make`:

```sh
make build
```

### Updating the Hyperdrive contracts

When the [hyperdrive-wrappers](./crates/hyperdrive-wrappers) crate is built,
it will clone the [Hyperdrive
repository](https://github.com/delvtech/hyperdrive) and generate type-safe Rust
bindings using [ethers-rs](https://github.com/gakonst/ethers-rs). You can modify
the version of the Hyperdrive contracts that are built against by updating the
git ref in the
[`hyperdrive.version`](./crates/hyperdrive-wrappers/hyperdrive.version) file.

To build against an existing local Hyperdrive repository instead, you can
symlink the local repository to the `hyperdrive-wrappers` directory:

```sh
ln -s <path-to-local-hyperdrive-clone> crates/hyperdrive-wrappers
```

And then in the .env file in hyperdrive-wrappers, add:

```sh
LOCAL_DEVELOPMENT=true
```

To prevent hyperdrive-wrappers from rebuilding automatically, add the following to the same .env:

```sh
BUILD_DISABLED=true
```

This is useful during development since running tests in hyperdrive-math will retrigger builds, which slows down development and testing considerably. Also, certain IDE's like vscode will over-eagerly rebuild the wrappers so turning off the build can be useful when there are no changes to the solidity files in hyperdrive.

## Test

```sh
make test
```

## Lint

```sh
make lint
```

# Disclaimer

The language used in this code and documentation is not intended to, and does not, have any particular financial, legal, or regulatory significance.

---

Copyright © 2024 DELV

Licensed under the Apache License, Version 2.0 (the "OSS License").

By accessing or using this code, you signify that you have read, understand and agree to be bound by and to comply with the [OSS License](http://www.apache.org/licenses/LICENSE-2.0) and [DELV's Terms of Service](https://delv-public.s3.us-east-2.amazonaws.com/delv-terms-of-service.pdf). If you do not agree to those terms, you are prohibited from accessing or using this code.

Unless required by applicable law or agreed to in writing, software distributed under the OSS License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the OSS License and the DELV Terms of Service for the specific language governing permissions and limitations.
