[![Tests](https://github.com/delvtech/hyperdrive-rs/actions/workflows/rust_test.yml/badge.svg)](https://github.com/delvtech/hyperdrive-rs/actions/workflows/rust_test.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/delvtech/elf-contracts/blob/master/LICENSE)
[![Static Badge](https://img.shields.io/badge/DELV-Terms%20Of%20Service-orange)](https://elementfi.s3.us-east-2.amazonaws.com/element-finance-terms-of-service.pdf)

<img src="icons/hyperdrive_winter.webp" width="800" alt="hyperdrive"><br>

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

Python bindings for this libarary can be accessed via
[agent0](https://github.com/delvtech/agent0).

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

This repository makes use of [foundry](https://github.com/foundry-rs/foundry) to
build and test smart contracts against the Rust implementation.
Proceed through the following steps to set up the repository:

- [Install forge](https://github.com/foundry-rs/foundry#installatio://github.com/foundry-rs/foundry#installation)
- Copy the `.env.example` file to `.env` and adjust as needed to point to the
  correct version of the [Hyperdrive](https://github.com/delvtech/hyperdrive) repository.

## Build

```sh
make build
```

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

By accessing or using this code, you signify that you have read, understand and agree to be bound by and to comply with the [OSS License](http://www.apache.org/licenses/LICENSE-2.0) and [DELV's Terms of Service](https://elementfi.s3.us-east-2.amazonaws.com/element-finance-terms-of-service.pdf). If you do not agree to those terms, you are prohibited from accessing or using this code.

Unless required by applicable law or agreed to in writing, software distributed under the OSS License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the OSS License and the DELV Terms of Service for the specific language governing permissions and limitations.
