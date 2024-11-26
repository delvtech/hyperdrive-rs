pub use st_eth_target_2::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod st_eth_target_2 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_config"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                            ::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Tuple(
                                    ::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],
                                ),
                            ],
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "struct IHyperdrive.PoolConfig",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("__adminController"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IHyperdriveAdminController",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("openLong"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("openLong"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minOutput"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_minVaultSharePrice",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_options"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IHyperdrive.Options",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("openShort"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("openShort"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bondAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxDeposit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_minVaultSharePrice",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_options"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IHyperdrive.Options",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AddLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("provider"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lpAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asBase"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lpSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CloseLong"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CloseLong"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("trader"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("destination"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assetId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("maturityTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asBase"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("bondAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CloseShort"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CloseShort"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("trader"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("destination"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assetId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("maturityTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asBase"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("basePayment"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("bondAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CollectGovernanceFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CollectGovernanceFee",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("collector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asBase"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CreateCheckpoint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CreateCheckpoint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("checkpointTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "checkpointVaultSharePrice",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("maturedShorts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("maturedLongs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lpSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("provider"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lpAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asBase"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("apr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OpenLong"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OpenLong"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("trader"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assetId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("maturityTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asBase"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("bondAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OpenShort"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OpenShort"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("trader"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assetId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("maturityTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asBase"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("baseProceeds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("bondAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PauseStatusUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PauseStatusUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("isPaused"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RedeemWithdrawalShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RedeemWithdrawalShares",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("provider"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("destination"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawalShareAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asBase"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RemoveLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RemoveLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("provider"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("destination"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lpAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asBase"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawalShareAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lpSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Sweep"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Sweep"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("collector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferSingle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TransferSingle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DistributeExcessIdleFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DistributeExcessIdleFailed",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExpInvalidExponent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ExpInvalidExponent"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientLiquidity",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidTimestamp"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LnInvalidInput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("LnInvalidInput"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MinimumSharePrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("MinimumSharePrice"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MinimumTransactionAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MinimumTransactionAmount",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutputLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OutputLimit"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolIsPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PoolIsPaused"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReentrancyGuardReentrantCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ReentrancyGuardReentrantCall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RestrictedZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RestrictedZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TransferFailed"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnsafeCastToInt128"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnsafeCastToInt128"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnsafeCastToInt256"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnsafeCastToInt256"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnsafeCastToUint112"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UnsafeCastToUint112",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnsafeCastToUint128"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UnsafeCastToUint128",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STETHTARGET2_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct StETHTarget2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StETHTarget2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StETHTarget2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StETHTarget2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StETHTarget2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StETHTarget2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StETHTarget2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STETHTARGET2_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `openLong` (0xcba2e58d) function
        pub fn open_long(
            &self,
            amount: ::ethers::core::types::U256,
            min_output: ::ethers::core::types::U256,
            min_vault_share_price: ::ethers::core::types::U256,
            options: Options,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [203, 162, 229, 141],
                    (amount, min_output, min_vault_share_price, options),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `openShort` (0xdbbe8070) function
        pub fn open_short(
            &self,
            bond_amount: ::ethers::core::types::U256,
            max_deposit: ::ethers::core::types::U256,
            min_vault_share_price: ::ethers::core::types::U256,
            options: Options,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [219, 190, 128, 112],
                    (bond_amount, max_deposit, min_vault_share_price, options),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddLiquidity` event
        pub fn add_liquidity_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddLiquidityFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalForAllFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CloseLong` event
        pub fn close_long_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CloseLongFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CloseShort` event
        pub fn close_short_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CloseShortFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CollectGovernanceFee` event
        pub fn collect_governance_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CollectGovernanceFeeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CreateCheckpoint` event
        pub fn create_checkpoint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CreateCheckpointFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialize` event
        pub fn initialize_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OpenLong` event
        pub fn open_long_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OpenLongFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OpenShort` event
        pub fn open_short_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OpenShortFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PauseStatusUpdated` event
        pub fn pause_status_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PauseStatusUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RedeemWithdrawalShares` event
        pub fn redeem_withdrawal_shares_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RedeemWithdrawalSharesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RemoveLiquidity` event
        pub fn remove_liquidity_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemoveLiquidityFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Sweep` event
        pub fn sweep_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SweepFilter> {
            self.0.event()
        }
        ///Gets the contract's `TransferSingle` event
        pub fn transfer_single_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferSingleFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StETHTarget2Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for StETHTarget2<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DistributeExcessIdleFailed` with signature `DistributeExcessIdleFailed()` and selector `0x8bdf918d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "DistributeExcessIdleFailed",
        abi = "DistributeExcessIdleFailed()"
    )]
    pub struct DistributeExcessIdleFailed;
    ///Custom Error type `ExpInvalidExponent` with signature `ExpInvalidExponent()` and selector `0x73a2d6b1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ExpInvalidExponent", abi = "ExpInvalidExponent()")]
    pub struct ExpInvalidExponent;
    ///Custom Error type `InsufficientLiquidity` with signature `InsufficientLiquidity()` and selector `0xbb55fd27`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InsufficientLiquidity", abi = "InsufficientLiquidity()")]
    pub struct InsufficientLiquidity;
    ///Custom Error type `InvalidTimestamp` with signature `InvalidTimestamp()` and selector `0xb7d09497`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidTimestamp", abi = "InvalidTimestamp()")]
    pub struct InvalidTimestamp;
    ///Custom Error type `LnInvalidInput` with signature `LnInvalidInput()` and selector `0xe61b4975`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "LnInvalidInput", abi = "LnInvalidInput()")]
    pub struct LnInvalidInput;
    ///Custom Error type `MinimumSharePrice` with signature `MinimumSharePrice()` and selector `0x42af972b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "MinimumSharePrice", abi = "MinimumSharePrice()")]
    pub struct MinimumSharePrice;
    ///Custom Error type `MinimumTransactionAmount` with signature `MinimumTransactionAmount()` and selector `0x423bbb46`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "MinimumTransactionAmount", abi = "MinimumTransactionAmount()")]
    pub struct MinimumTransactionAmount;
    ///Custom Error type `OutputLimit` with signature `OutputLimit()` and selector `0xc9726517`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OutputLimit", abi = "OutputLimit()")]
    pub struct OutputLimit;
    ///Custom Error type `PoolIsPaused` with signature `PoolIsPaused()` and selector `0x21081abf`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "PoolIsPaused", abi = "PoolIsPaused()")]
    pub struct PoolIsPaused;
    ///Custom Error type `ReentrancyGuardReentrantCall` with signature `ReentrancyGuardReentrantCall()` and selector `0x3ee5aeb5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ReentrancyGuardReentrantCall",
        abi = "ReentrancyGuardReentrantCall()"
    )]
    pub struct ReentrancyGuardReentrantCall;
    ///Custom Error type `RestrictedZeroAddress` with signature `RestrictedZeroAddress()` and selector `0xf0dd15fd`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "RestrictedZeroAddress", abi = "RestrictedZeroAddress()")]
    pub struct RestrictedZeroAddress;
    ///Custom Error type `TransferFailed` with signature `TransferFailed()` and selector `0x90b8ec18`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "TransferFailed", abi = "TransferFailed()")]
    pub struct TransferFailed;
    ///Custom Error type `UnsafeCastToInt128` with signature `UnsafeCastToInt128()` and selector `0xa5353be5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UnsafeCastToInt128", abi = "UnsafeCastToInt128()")]
    pub struct UnsafeCastToInt128;
    ///Custom Error type `UnsafeCastToInt256` with signature `UnsafeCastToInt256()` and selector `0x72dd4e02`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UnsafeCastToInt256", abi = "UnsafeCastToInt256()")]
    pub struct UnsafeCastToInt256;
    ///Custom Error type `UnsafeCastToUint112` with signature `UnsafeCastToUint112()` and selector `0x10d62a2e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UnsafeCastToUint112", abi = "UnsafeCastToUint112()")]
    pub struct UnsafeCastToUint112;
    ///Custom Error type `UnsafeCastToUint128` with signature `UnsafeCastToUint128()` and selector `0x1e15f2a2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UnsafeCastToUint128", abi = "UnsafeCastToUint128()")]
    pub struct UnsafeCastToUint128;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum StETHTarget2Errors {
        DistributeExcessIdleFailed(DistributeExcessIdleFailed),
        ExpInvalidExponent(ExpInvalidExponent),
        InsufficientLiquidity(InsufficientLiquidity),
        InvalidTimestamp(InvalidTimestamp),
        LnInvalidInput(LnInvalidInput),
        MinimumSharePrice(MinimumSharePrice),
        MinimumTransactionAmount(MinimumTransactionAmount),
        OutputLimit(OutputLimit),
        PoolIsPaused(PoolIsPaused),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        RestrictedZeroAddress(RestrictedZeroAddress),
        TransferFailed(TransferFailed),
        UnsafeCastToInt128(UnsafeCastToInt128),
        UnsafeCastToInt256(UnsafeCastToInt256),
        UnsafeCastToUint112(UnsafeCastToUint112),
        UnsafeCastToUint128(UnsafeCastToUint128),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for StETHTarget2Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DistributeExcessIdleFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DistributeExcessIdleFailed(decoded));
            }
            if let Ok(decoded) = <ExpInvalidExponent as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpInvalidExponent(decoded));
            }
            if let Ok(decoded) = <InsufficientLiquidity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientLiquidity(decoded));
            }
            if let Ok(decoded) = <InvalidTimestamp as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidTimestamp(decoded));
            }
            if let Ok(decoded) = <LnInvalidInput as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LnInvalidInput(decoded));
            }
            if let Ok(decoded) = <MinimumSharePrice as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinimumSharePrice(decoded));
            }
            if let Ok(decoded) = <MinimumTransactionAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinimumTransactionAmount(decoded));
            }
            if let Ok(decoded) = <OutputLimit as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OutputLimit(decoded));
            }
            if let Ok(decoded) = <PoolIsPaused as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolIsPaused(decoded));
            }
            if let Ok(decoded) = <ReentrancyGuardReentrantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReentrancyGuardReentrantCall(decoded));
            }
            if let Ok(decoded) = <RestrictedZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RestrictedZeroAddress(decoded));
            }
            if let Ok(decoded) = <TransferFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFailed(decoded));
            }
            if let Ok(decoded) = <UnsafeCastToInt128 as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnsafeCastToInt128(decoded));
            }
            if let Ok(decoded) = <UnsafeCastToInt256 as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnsafeCastToInt256(decoded));
            }
            if let Ok(decoded) = <UnsafeCastToUint112 as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnsafeCastToUint112(decoded));
            }
            if let Ok(decoded) = <UnsafeCastToUint128 as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnsafeCastToUint128(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StETHTarget2Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DistributeExcessIdleFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpInvalidExponent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LnInvalidInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinimumSharePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinimumTransactionAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutputLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolIsPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RestrictedZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsafeCastToInt128(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsafeCastToInt256(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsafeCastToUint112(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsafeCastToUint128(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for StETHTarget2Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <DistributeExcessIdleFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ExpInvalidExponent as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientLiquidity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTimestamp as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <LnInvalidInput as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MinimumSharePrice as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MinimumTransactionAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutputLimit as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <PoolIsPaused as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ReentrancyGuardReentrantCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RestrictedZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnsafeCastToInt128 as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnsafeCastToInt256 as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnsafeCastToUint112 as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnsafeCastToUint128 as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for StETHTarget2Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DistributeExcessIdleFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExpInvalidExponent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::LnInvalidInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumSharePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumTransactionAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OutputLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolIsPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RestrictedZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsafeCastToInt128(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnsafeCastToInt256(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnsafeCastToUint112(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnsafeCastToUint128(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for StETHTarget2Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DistributeExcessIdleFailed> for StETHTarget2Errors {
        fn from(value: DistributeExcessIdleFailed) -> Self {
            Self::DistributeExcessIdleFailed(value)
        }
    }
    impl ::core::convert::From<ExpInvalidExponent> for StETHTarget2Errors {
        fn from(value: ExpInvalidExponent) -> Self {
            Self::ExpInvalidExponent(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidity> for StETHTarget2Errors {
        fn from(value: InsufficientLiquidity) -> Self {
            Self::InsufficientLiquidity(value)
        }
    }
    impl ::core::convert::From<InvalidTimestamp> for StETHTarget2Errors {
        fn from(value: InvalidTimestamp) -> Self {
            Self::InvalidTimestamp(value)
        }
    }
    impl ::core::convert::From<LnInvalidInput> for StETHTarget2Errors {
        fn from(value: LnInvalidInput) -> Self {
            Self::LnInvalidInput(value)
        }
    }
    impl ::core::convert::From<MinimumSharePrice> for StETHTarget2Errors {
        fn from(value: MinimumSharePrice) -> Self {
            Self::MinimumSharePrice(value)
        }
    }
    impl ::core::convert::From<MinimumTransactionAmount> for StETHTarget2Errors {
        fn from(value: MinimumTransactionAmount) -> Self {
            Self::MinimumTransactionAmount(value)
        }
    }
    impl ::core::convert::From<OutputLimit> for StETHTarget2Errors {
        fn from(value: OutputLimit) -> Self {
            Self::OutputLimit(value)
        }
    }
    impl ::core::convert::From<PoolIsPaused> for StETHTarget2Errors {
        fn from(value: PoolIsPaused) -> Self {
            Self::PoolIsPaused(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for StETHTarget2Errors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<RestrictedZeroAddress> for StETHTarget2Errors {
        fn from(value: RestrictedZeroAddress) -> Self {
            Self::RestrictedZeroAddress(value)
        }
    }
    impl ::core::convert::From<TransferFailed> for StETHTarget2Errors {
        fn from(value: TransferFailed) -> Self {
            Self::TransferFailed(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToInt128> for StETHTarget2Errors {
        fn from(value: UnsafeCastToInt128) -> Self {
            Self::UnsafeCastToInt128(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToInt256> for StETHTarget2Errors {
        fn from(value: UnsafeCastToInt256) -> Self {
            Self::UnsafeCastToInt256(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToUint112> for StETHTarget2Errors {
        fn from(value: UnsafeCastToUint112) -> Self {
            Self::UnsafeCastToUint112(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToUint128> for StETHTarget2Errors {
        fn from(value: UnsafeCastToUint128) -> Self {
            Self::UnsafeCastToUint128(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "AddLiquidity",
        abi = "AddLiquidity(address,uint256,uint256,uint256,bool,uint256,bytes)"
    )]
    pub struct AddLiquidityFilter {
        #[ethevent(indexed)]
        pub provider: ::ethers::core::types::Address,
        pub lp_amount: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub as_base: bool,
        pub lp_share_price: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "CloseLong",
        abi = "CloseLong(address,address,uint256,uint256,uint256,uint256,bool,uint256,bytes)"
    )]
    pub struct CloseLongFilter {
        #[ethevent(indexed)]
        pub trader: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub destination: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset_id: ::ethers::core::types::U256,
        pub maturity_time: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub as_base: bool,
        pub bond_amount: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "CloseShort",
        abi = "CloseShort(address,address,uint256,uint256,uint256,uint256,bool,uint256,uint256,bytes)"
    )]
    pub struct CloseShortFilter {
        #[ethevent(indexed)]
        pub trader: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub destination: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset_id: ::ethers::core::types::U256,
        pub maturity_time: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub as_base: bool,
        pub base_payment: ::ethers::core::types::U256,
        pub bond_amount: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "CollectGovernanceFee",
        abi = "CollectGovernanceFee(address,uint256,uint256,bool)"
    )]
    pub struct CollectGovernanceFeeFilter {
        #[ethevent(indexed)]
        pub collector: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub as_base: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "CreateCheckpoint",
        abi = "CreateCheckpoint(uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CreateCheckpointFilter {
        #[ethevent(indexed)]
        pub checkpoint_time: ::ethers::core::types::U256,
        pub checkpoint_vault_share_price: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub matured_shorts: ::ethers::core::types::U256,
        pub matured_longs: ::ethers::core::types::U256,
        pub lp_share_price: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Initialize",
        abi = "Initialize(address,uint256,uint256,uint256,bool,uint256,bytes)"
    )]
    pub struct InitializeFilter {
        #[ethevent(indexed)]
        pub provider: ::ethers::core::types::Address,
        pub lp_amount: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub as_base: bool,
        pub apr: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OpenLong",
        abi = "OpenLong(address,uint256,uint256,uint256,uint256,bool,uint256,bytes)"
    )]
    pub struct OpenLongFilter {
        #[ethevent(indexed)]
        pub trader: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset_id: ::ethers::core::types::U256,
        pub maturity_time: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub as_base: bool,
        pub bond_amount: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OpenShort",
        abi = "OpenShort(address,uint256,uint256,uint256,uint256,bool,uint256,uint256,bytes)"
    )]
    pub struct OpenShortFilter {
        #[ethevent(indexed)]
        pub trader: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset_id: ::ethers::core::types::U256,
        pub maturity_time: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub as_base: bool,
        pub base_proceeds: ::ethers::core::types::U256,
        pub bond_amount: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "PauseStatusUpdated", abi = "PauseStatusUpdated(bool)")]
    pub struct PauseStatusUpdatedFilter {
        pub is_paused: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RedeemWithdrawalShares",
        abi = "RedeemWithdrawalShares(address,address,uint256,uint256,uint256,bool,bytes)"
    )]
    pub struct RedeemWithdrawalSharesFilter {
        #[ethevent(indexed)]
        pub provider: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub destination: ::ethers::core::types::Address,
        pub withdrawal_share_amount: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub as_base: bool,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RemoveLiquidity",
        abi = "RemoveLiquidity(address,address,uint256,uint256,uint256,bool,uint256,uint256,bytes)"
    )]
    pub struct RemoveLiquidityFilter {
        #[ethevent(indexed)]
        pub provider: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub destination: ::ethers::core::types::Address,
        pub lp_amount: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub as_base: bool,
        pub withdrawal_share_amount: ::ethers::core::types::U256,
        pub lp_share_price: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Sweep", abi = "Sweep(address,address)")]
    pub struct SweepFilter {
        #[ethevent(indexed)]
        pub collector: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub target: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "TransferSingle",
        abi = "TransferSingle(address,address,address,uint256,uint256)"
    )]
    pub struct TransferSingleFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum StETHTarget2Events {
        AddLiquidityFilter(AddLiquidityFilter),
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        CloseLongFilter(CloseLongFilter),
        CloseShortFilter(CloseShortFilter),
        CollectGovernanceFeeFilter(CollectGovernanceFeeFilter),
        CreateCheckpointFilter(CreateCheckpointFilter),
        InitializeFilter(InitializeFilter),
        OpenLongFilter(OpenLongFilter),
        OpenShortFilter(OpenShortFilter),
        PauseStatusUpdatedFilter(PauseStatusUpdatedFilter),
        RedeemWithdrawalSharesFilter(RedeemWithdrawalSharesFilter),
        RemoveLiquidityFilter(RemoveLiquidityFilter),
        SweepFilter(SweepFilter),
        TransferSingleFilter(TransferSingleFilter),
    }
    impl ::ethers::contract::EthLogDecode for StETHTarget2Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddLiquidityFilter::decode_log(log) {
                return Ok(StETHTarget2Events::AddLiquidityFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(StETHTarget2Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(StETHTarget2Events::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = CloseLongFilter::decode_log(log) {
                return Ok(StETHTarget2Events::CloseLongFilter(decoded));
            }
            if let Ok(decoded) = CloseShortFilter::decode_log(log) {
                return Ok(StETHTarget2Events::CloseShortFilter(decoded));
            }
            if let Ok(decoded) = CollectGovernanceFeeFilter::decode_log(log) {
                return Ok(StETHTarget2Events::CollectGovernanceFeeFilter(decoded));
            }
            if let Ok(decoded) = CreateCheckpointFilter::decode_log(log) {
                return Ok(StETHTarget2Events::CreateCheckpointFilter(decoded));
            }
            if let Ok(decoded) = InitializeFilter::decode_log(log) {
                return Ok(StETHTarget2Events::InitializeFilter(decoded));
            }
            if let Ok(decoded) = OpenLongFilter::decode_log(log) {
                return Ok(StETHTarget2Events::OpenLongFilter(decoded));
            }
            if let Ok(decoded) = OpenShortFilter::decode_log(log) {
                return Ok(StETHTarget2Events::OpenShortFilter(decoded));
            }
            if let Ok(decoded) = PauseStatusUpdatedFilter::decode_log(log) {
                return Ok(StETHTarget2Events::PauseStatusUpdatedFilter(decoded));
            }
            if let Ok(decoded) = RedeemWithdrawalSharesFilter::decode_log(log) {
                return Ok(StETHTarget2Events::RedeemWithdrawalSharesFilter(decoded));
            }
            if let Ok(decoded) = RemoveLiquidityFilter::decode_log(log) {
                return Ok(StETHTarget2Events::RemoveLiquidityFilter(decoded));
            }
            if let Ok(decoded) = SweepFilter::decode_log(log) {
                return Ok(StETHTarget2Events::SweepFilter(decoded));
            }
            if let Ok(decoded) = TransferSingleFilter::decode_log(log) {
                return Ok(StETHTarget2Events::TransferSingleFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StETHTarget2Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddLiquidityFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CloseLongFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CloseShortFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CollectGovernanceFeeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateCheckpointFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OpenLongFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OpenShortFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseStatusUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RedeemWithdrawalSharesFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveLiquidityFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SweepFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferSingleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddLiquidityFilter> for StETHTarget2Events {
        fn from(value: AddLiquidityFilter) -> Self {
            Self::AddLiquidityFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalFilter> for StETHTarget2Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for StETHTarget2Events {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<CloseLongFilter> for StETHTarget2Events {
        fn from(value: CloseLongFilter) -> Self {
            Self::CloseLongFilter(value)
        }
    }
    impl ::core::convert::From<CloseShortFilter> for StETHTarget2Events {
        fn from(value: CloseShortFilter) -> Self {
            Self::CloseShortFilter(value)
        }
    }
    impl ::core::convert::From<CollectGovernanceFeeFilter> for StETHTarget2Events {
        fn from(value: CollectGovernanceFeeFilter) -> Self {
            Self::CollectGovernanceFeeFilter(value)
        }
    }
    impl ::core::convert::From<CreateCheckpointFilter> for StETHTarget2Events {
        fn from(value: CreateCheckpointFilter) -> Self {
            Self::CreateCheckpointFilter(value)
        }
    }
    impl ::core::convert::From<InitializeFilter> for StETHTarget2Events {
        fn from(value: InitializeFilter) -> Self {
            Self::InitializeFilter(value)
        }
    }
    impl ::core::convert::From<OpenLongFilter> for StETHTarget2Events {
        fn from(value: OpenLongFilter) -> Self {
            Self::OpenLongFilter(value)
        }
    }
    impl ::core::convert::From<OpenShortFilter> for StETHTarget2Events {
        fn from(value: OpenShortFilter) -> Self {
            Self::OpenShortFilter(value)
        }
    }
    impl ::core::convert::From<PauseStatusUpdatedFilter> for StETHTarget2Events {
        fn from(value: PauseStatusUpdatedFilter) -> Self {
            Self::PauseStatusUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<RedeemWithdrawalSharesFilter> for StETHTarget2Events {
        fn from(value: RedeemWithdrawalSharesFilter) -> Self {
            Self::RedeemWithdrawalSharesFilter(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityFilter> for StETHTarget2Events {
        fn from(value: RemoveLiquidityFilter) -> Self {
            Self::RemoveLiquidityFilter(value)
        }
    }
    impl ::core::convert::From<SweepFilter> for StETHTarget2Events {
        fn from(value: SweepFilter) -> Self {
            Self::SweepFilter(value)
        }
    }
    impl ::core::convert::From<TransferSingleFilter> for StETHTarget2Events {
        fn from(value: TransferSingleFilter) -> Self {
            Self::TransferSingleFilter(value)
        }
    }
    ///Container type for all input parameters for the `openLong` function with signature `openLong(uint256,uint256,uint256,(address,bool,bytes))` and selector `0xcba2e58d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "openLong",
        abi = "openLong(uint256,uint256,uint256,(address,bool,bytes))"
    )]
    pub struct OpenLongCall {
        pub amount: ::ethers::core::types::U256,
        pub min_output: ::ethers::core::types::U256,
        pub min_vault_share_price: ::ethers::core::types::U256,
        pub options: Options,
    }
    ///Container type for all input parameters for the `openShort` function with signature `openShort(uint256,uint256,uint256,(address,bool,bytes))` and selector `0xdbbe8070`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "openShort",
        abi = "openShort(uint256,uint256,uint256,(address,bool,bytes))"
    )]
    pub struct OpenShortCall {
        pub bond_amount: ::ethers::core::types::U256,
        pub max_deposit: ::ethers::core::types::U256,
        pub min_vault_share_price: ::ethers::core::types::U256,
        pub options: Options,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum StETHTarget2Calls {
        OpenLong(OpenLongCall),
        OpenShort(OpenShortCall),
    }
    impl ::ethers::core::abi::AbiDecode for StETHTarget2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <OpenLongCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OpenLong(decoded));
            }
            if let Ok(decoded) = <OpenShortCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OpenShort(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StETHTarget2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::OpenLong(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OpenShort(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StETHTarget2Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OpenLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::OpenShort(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OpenLongCall> for StETHTarget2Calls {
        fn from(value: OpenLongCall) -> Self {
            Self::OpenLong(value)
        }
    }
    impl ::core::convert::From<OpenShortCall> for StETHTarget2Calls {
        fn from(value: OpenShortCall) -> Self {
            Self::OpenShort(value)
        }
    }
    ///Container type for all return fields from the `openLong` function with signature `openLong(uint256,uint256,uint256,(address,bool,bytes))` and selector `0xcba2e58d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OpenLongReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `openShort` function with signature `openShort(uint256,uint256,uint256,(address,bool,bytes))` and selector `0xdbbe8070`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OpenShortReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///`Fees(uint256,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Fees {
        pub curve: ::ethers::core::types::U256,
        pub flat: ::ethers::core::types::U256,
        pub governance_lp: ::ethers::core::types::U256,
        pub governance_zombie: ::ethers::core::types::U256,
    }
    ///`Options(address,bool,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Options {
        pub destination: ::ethers::core::types::Address,
        pub as_base: bool,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///`PoolConfig(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256))`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PoolConfig {
        pub base_token: ::ethers::core::types::Address,
        pub vault_shares_token: ::ethers::core::types::Address,
        pub linker_factory: ::ethers::core::types::Address,
        pub linker_code_hash: [u8; 32],
        pub initial_vault_share_price: ::ethers::core::types::U256,
        pub minimum_share_reserves: ::ethers::core::types::U256,
        pub minimum_transaction_amount: ::ethers::core::types::U256,
        pub circuit_breaker_delta: ::ethers::core::types::U256,
        pub position_duration: ::ethers::core::types::U256,
        pub checkpoint_duration: ::ethers::core::types::U256,
        pub time_stretch: ::ethers::core::types::U256,
        pub governance: ::ethers::core::types::Address,
        pub fee_collector: ::ethers::core::types::Address,
        pub sweep_collector: ::ethers::core::types::Address,
        pub checkpoint_rewarder: ::ethers::core::types::Address,
        pub fees: Fees,
    }
}

pub struct StETHTarget2Libs {
    pub lp_math: ::ethers::types::Address,
}

impl<M: ::ethers::providers::Middleware> StETHTarget2<M> {
    pub fn link_and_deploy<T: ::ethers::core::abi::Tokenize>(
        client: ::std::sync::Arc<M>,
        constructor_args: T,
        libs: StETHTarget2Libs,
    ) -> ::core::result::Result<
        ::ethers::contract::builders::ContractDeployer<M, Self>,
        ::ethers::contract::ContractError<M>,
    > {
        let factory = crate::linked_factory::create(
            STETHTARGET2_ABI.clone(),
            "0x6102806040523480156200001257600080fd5b506040516200405b3803806200405b8339810160408190526200003591620001ae565b600160005581516001600160a01b039081166080908152602080850151831660a0908152918501516101a052908401516101c05260c0808501516101e090815260e080870151610200526101208088015190935261010080880151909152610140808801519091529086018051519092528151909201519091528051604090810151610160529051606090810151610180529084015182166102405290920151610260521661022052620002eb565b60405161020081016001600160401b03811182821017156200011657634e487b7160e01b600052604160045260246000fd5b60405290565b80516001600160a01b03811681146200013457600080fd5b919050565b6000608082840312156200014c57600080fd5b604051608081016001600160401b03811182821017156200017d57634e487b7160e01b600052604160045260246000fd5b8060405250809150825181526020830151602082015260408301516040820152606083015160608201525092915050565b600080828403610280811215620001c457600080fd5b61026080821215620001d557600080fd5b620001df620000e4565b9150620001ec856200011c565b8252620001fc602086016200011c565b60208301526200020f604086016200011c565b6040830152606085015160608301526080850151608083015260a085015160a083015260c085015160c083015260e085015160e08301526101008086015181840152506101208086015181840152506101408086015181840152506101606200027a8187016200011c565b908301526101806200028e8682016200011c565b908301526101a0620002a28682016200011c565b908301526101c0620002b68682016200011c565b908301526101e0620002cb8787830162000139565b8184015250819350620002e08186016200011c565b925050509250929050565b60805160a05160c05160e05161010051610120516101405161016051610180516101a0516101c0516101e05161020051610220516102405161026051613ba7620004b4600039600050506000505060008181610c2f01526117500152600050506000818160f8015281816103000152612e0801526000818161156701528181611e3c01528181611e9f01528181612651015281816129ed01528181612a2901528181612c210152612de20152600081816107dd01528181610d8f01528181610dd101528181610ec301528181611339015281816113980152612dbc01526000611a5b015260008181611b8101528181612746015281816127bf0152612f220152600081816109a101528181610e440152818161143301528181611b550152612793015260008181610e23015281816127120152612ede0152600081816107fe01528181610d6d01528181610df201528181610ee401528181611317015281816113b90152612e2e0152600081816101d601528181610398015281816108d901526133b2015260008181610697015281816107170152818161078d01528181610859015261089101526000818161149c015281816117200152611867015260005050613ba76000f3fe6080604052600436106100295760003560e01c8063cba2e58d1461002e578063dbbe80701461005a575b600080fd5b61004161003c366004613647565b61006d565b6040805192835260208301919091520160405180910390f35b610041610068366004613647565b61008a565b60008061007c86868686610099565b915091505b94509492505050565b60008061007c868686866102c1565b6000806100a4610539565b600654610100900460ff16156100cd576040516321081abf60e01b815260040160405180910390fd5b6100d683610563565b6000806100e3888661059c565b909250905060006100f48383610671565b90507f00000000000000000000000000000000000000000000000000000000000000008110156101375760405163211ddda360e11b815260040160405180910390fd5b86821015610158576040516342af972b60e01b815260040160405180910390fd5b600061016261068f565b90506101728184600460016106c0565b5060008060006101828787610d3e565b919a5091945090925090508b8810156101ae5760405163c972651760e01b815260040160405180910390fd5b81600960008282546101c091906136bc565b909155506101d19050844283610f20565b6101fb7f0000000000000000000000000000000000000000000000000000000000000000856136bc565b98506102098389888c61105e565b600061021660018b6111f8565b905061022f8161022960208e018e6136e4565b8b61122d565b8d8a8a898e8561024260208301836136e4565b6001600160a01b03167f7fc9757758f4c7f2eb9f011c4500beb349847d2f2acbdd5ffce3e2f01e79903a86888661027f604088016020890161370f565b8961028d60408a018a61372c565b6040516102a0979695949392919061379c565b60405180910390a350909b5050505050505050505050506100816001600055565b6000806102cc610539565b600654610100900460ff16156102f5576040516321081abf60e01b815260040160405180910390fd5b6102fe83610563565b7f000000000000000000000000000000000000000000000000000000000000000086101561033f5760405163211ddda360e11b815260040160405180910390fd5b60006103496112d5565b90508481101561036c576040516342af972b60e01b815260040160405180910390fd5b600061037661068f565b9050600061038f61038561068f565b84600460016106c0565b905060006103bd7f0000000000000000000000000000000000000000000000000000000000000000846136bc565b90506000806000806103d08e89886112e8565b60098054949850929650909450925083916000906103ef9084906136bc565b909155506104009050874283610f20565b61041261040d858a61146e565b611483565b61041d9060016136bc565b935050600061042d84898d61150f565b9050808d10156104505760405163c972651760e01b815260040160405180910390fd5b61045a818c61059c565b508e905061046a81858b89611541565b60006104776002886111f8565b90508c6104918261048b60208401846136e4565b8561122d565b858b86846104a260208601866136e4565b6001600160a01b03167ffa6dd2e3e152dbc3fe91196c0b8aa871c26fd7a1d07de126ec3159fd4ede2c758d8a866104df60408b0160208c0161370f565b6104f3896104ed8a8d6137dd565b90610671565b8d61050160408e018e61372c565b6040516105159897969594939291906137f0565b60405180910390a350989d50939b5050505050505050505050506100816001600055565b60026000540361055c57604051633ee5aeb560e01b815260040160405180910390fd5b6002600055565b600061057260208301836136e4565b6001600160a01b0316036105995760405163f0dd15fd60e01b815260040160405180910390fd5b50565b816000806105b0604085016020860161370f565b156105d5576105cb856105c6604087018761372c565b6116f5565b90935090506105ed565b50346105ed856105e8604087018761372c565b611845565b6105f56112d5565b9150801561066857604051600090339083908381818185875af1925050503d806000811461063f576040519150601f19603f3d011682016040523d82523d6000602084013e610644565b606091505b5050905080610666576040516312171d8360e31b815260040160405180910390fd5b505b505b9250929050565b60006106868383670de0b6b3a76400006118e2565b90505b92915050565b60006106bb427f0000000000000000000000000000000000000000000000000000000000000000611900565b905090565b600084815260086020526040812060018101546001600160801b03161515806106e857504286115b1561070157600101546001600160801b03169050610d36565b600080600061070e61068f565b9050600061073c7f00000000000000000000000000000000000000000000000000000000000000008b6136bc565b90505b818110156107b2576000818152600860205260409020600101546001600160801b0316801561078a576000828152600860205260409020549094506001600160801b031692506107b2565b507f00000000000000000000000000000000000000000000000000000000000000000161073f565b83600003610825578893506108226107c8611916565b600254600160801b90046001600160801b03167f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000611937565b92505b5061082f83611957565b6001850180546001600160801b0319166001600160801b039290921691909117905561088561087e7f00000000000000000000000000000000000000000000000000000000000000008b6137dd565b8a84610f20565b6108c3896108bd6108b67f0000000000000000000000000000000000000000000000000000000000000000836136bc565b4290611985565b84610f20565b6108cc8861199a565b50600090506008816108fe7f00000000000000000000000000000000000000000000000000000000000000008d6137dd565b815260208101919091526040016000908120600101546001600160801b0316915061092a60028c6111f8565b6000818152600b60205260408120549192508c8c8315610a70576001925060008061095986898d866000611b3e565b91509150806009600082825461096f91906136bc565b9091555061098b90508660008461098581611c0d565b88611c37565b61099581836136bc565b91506109c586838a8e877f0000000000000000000000000000000000000000000000000000000000000000611db1565b91506109d96109d48385610671565b611dfb565b600680546002906109fa9084906201000090046001600160701b0316613838565b92506101000a8154816001600160701b0302191690836001600160701b03160217905550610a2782611957565b60068054601090610a49908490600160801b90046001600160801b0316613858565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555050505b6000610a7d6001846111f8565b6000818152600b60205260409020549091508015610b8c5760019450600080610aaa838b8f886001611b3e565b915091508060096000828254610ac091906136bc565b90915550610adc905083600084610ad681611c0d565b8a611e25565b610ae681836137dd565b9150610af56109d48387610671565b60068054600290610b169084906201000090046001600160701b0316613838565b92506101000a8154816001600160701b0302191690836001600160701b03160217905550610b4382611957565b60068054601090610b65908490600160801b90046001600160801b0316613858565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555050505b8415610bc657610bb8610b9e87611c0d565b610ba783611c0d565b610bb19190613878565b6000612016565b8e610bc38482612055565b50505b6000610bd18461224f565b50604080518e815260208101879052908101899052606081018490526080810182905290915085907fff888cf98d2696e95c8c39aa98c9ad55a5378008f7a56614c9353b7137a57ab79060a00160405180910390a260008f905060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f25964586040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c8b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610caf9190613898565b90506001600160a01b03811615610d2457610d215a604051336024820152604481018a90528415156064820152600090819060840160408051601f198184030181529190526020810180516001600160e01b0316633488a6a760e11b1790526001600160a01b0386169392919061239b565b50505b509b9d50505050505050505050505050505b949350505050565b6000806000806000610d4e611916565b600254909150610db3908290600160801b90046001600160801b0316897f00000000000000000000000000000000000000000000000000000000000000008a7f0000000000000000000000000000000000000000000000000000000000000000612426565b600254909450610e16908290600160801b90046001600160801b03167f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000611937565b9150610e6d8785610e68857f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000612452565b6124b7565b15610e7a57610e7a6124f9565b610e8687858885612512565b91965094509250670de0b6b3a7640000610f08610ea387846136bc565b600254610ec1908890600160801b90046001600160801b03166137dd565b7f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000611937565b1115610f1657610f166124f9565b5092959194509250565b600083815260086020526040902054600160801b90046001600160801b0316808303610f4c5750505050565b6000848152600860205260408120546001600160801b031690819003610fa657610f7583611957565b600086815260086020526040902080546001600160801b0319166001600160801b0392909216919091179055611024565b610ff7610ff2670de0b6b3a7640000610fbf88866137dd565b610fc991906138b5565b85670de0b6b3a7640000610fdd878a6137dd565b610fe791906138b5565b85929190600161255e565b611957565b600086815260086020526040902080546001600160801b0319166001600160801b03929092169190911790555b61102d84611957565b60009586526008602052604090952080546001600160801b03968716600160801b0296169590951790945550505050565b600354600160801b90046001600160801b03166110a0610ff28261108a670de0b6b3a7640000866138b5565b6005546001600160801b0316919088600161255e565b600580546001600160801b0319166001600160801b03929092169190911790556110c985611957565b600280546000906110e49084906001600160801b0316613858565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555061111184611957565b60028054601090611133908490600160801b90046001600160801b03166138cc565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555061116084611957565b61116a9082613858565b600380546001600160801b03808416600160801b0291161790559050600061119183612606565b90506111af816111a087611c0d565b6111aa90846138ec565b612016565b6111b88461264a565b6111c4576111c46124f9565b60006111cf856126aa565b9050806111ef57604051638bdf918d60e01b815260040160405180910390fd5b50505050505050565b60006001600160f81b038211156112225760405163b7d0949760e01b815260040160405180910390fd5b5060f89190911b1790565b6000838152600a602090815260408083206001600160a01b03861684529091528120805483929061125f9084906136bc565b90915550506000838152600b6020526040812080548392906112829084906136bc565b909155505060408051848152602081018390526001600160a01b0384169160009133917fc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f62910160405180910390a4505050565b60006106bb670de0b6b3a7640000611483565b60008060008060006112f8611916565b60025490915061135d908290600160801b90046001600160801b03168a7f00000000000000000000000000000000000000000000000000000000000000008b7f00000000000000000000000000000000000000000000000000000000000000006126b7565b93508761136a85896126d8565b1115611378576113786124f9565b60025460009081906113dd908490600160801b90046001600160801b03167f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000611937565b93506113f38a670de0b6b3a7640000868c6126ed565b50919350909150611406905081836137dd565b61141090876137dd565b95508861145d816114578d611425868c6137dd565b8d61143087826127fb565b877f0000000000000000000000000000000000000000000000000000000000000000612811565b906126d8565b975090945050505093509350935093565b600061068683670de0b6b3a764000084612832565b604051630f451f7160e31b8152600481018290526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690637a28fb8890602401602060405180830381865afa1580156114eb573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106899190613914565b6000611521604083016020840161370f565b1561152d57508261153a565b6115378484612858565b90505b9392505050565b6002546001600160801b03168381101561155d5761155d6124f9565b60045490849003907f000000000000000000000000000000000000000000000000000000000000000090611595908390600f0b61286d565b10156115a3576115a36124f9565b6004546115ed90610ff290600160801b90046001600160801b03166115d0670de0b6b3a7640000866138b5565b600554600160801b90046001600160801b0316919089600161255e565b600580546001600160801b03928316600160801b02921691909117905561161381611957565b600280546001600160801b0319166001600160801b039290921691909117905561163c85611957565b6002805460109061165e908490600160801b90046001600160801b0316613858565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555061168b85611957565b600480546010906116ad908490600160801b90046001600160801b0316613858565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555060006116dc83612606565b90506111af816116eb88611c0d565b6111aa9084613878565b60008084341015611719576040516312171d8360e31b815260040160405180910390fd5b84340390507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663a1903eab867f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c415b95c6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156117ac573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906117d09190613898565b6040516001600160e01b031960e085901b1681526001600160a01b03909116600482015260240160206040518083038185885af1158015611815573d6000803e3d6000fd5b50505050506040513d601f19601f8201168201806040525081019061183a9190613914565b91505b935093915050565b604051636d78045960e01b8152336004820152306024820152604481018490527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690636d780459906064016020604051808303816000875af11580156118b8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118dc9190613914565b50505050565b60008260001904841183021582026118f957600080fd5b5091020490565b600061190c8284613943565b61068690846137dd565b6002546004546000916106bb916001600160801b0390911690600f0b61286d565b600061194e826119488588886118e2565b90612893565b95945050505050565b60006001600160801b0382111561198157604051630f0af95160e11b815260040160405180910390fd5b5090565b60008183116119945782610686565b50919050565b60065460009081906119bd908490600160801b90046001600160801b0316610671565b6006546201000090046001600160701b03169250905081811115611b395760006119e783836137dd565b90506119f6610ff2828661146e565b60068054601090611a18908490600160801b90046001600160801b03166138cc565b92506101000a8154816001600160801b0302191690836001600160801b031602179055506000611a51858361285890919063ffffffff16565b90506000611a7f827f0000000000000000000000000000000000000000000000000000000000000000610671565b90508060096000828254611a9391906136bc565b90915550611aa3905081836137dd565b9150611aae82611957565b60028054600090611ac99084906001600160801b0316613858565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550611af6826128fe565b60048054600090611b0b908490600f0b613957565b92506101000a8154816001600160801b030219169083600f0b6001600160801b031602179055508493505050505b915091565b600080611b4b8785612858565b91506000611b79837f0000000000000000000000000000000000000000000000000000000000000000610671565b9050611ba5817f0000000000000000000000000000000000000000000000000000000000000000610671565b91508315611bc857611bb782826137dd565b611bc190846137dd565b9250611bdf565b611bd282826137dd565b611bdc90846136bc565b92505b86861015611c0257611bf28387896118e2565b9250611bff8287896118e2565b91505b509550959350505050565b60006001600160ff1b038211156119815760405163396ea70160e11b815260040160405180910390fd5b600454600160801b90046001600160801b0316611c80610ff282611c63670de0b6b3a7640000866138b5565b600554600160801b90046001600160801b031691908a600061255e565b600580546001600160801b03928316600160801b029216919091179055611ca686611957565b611cb090826138cc565b600480546001600160801b03928316600160801b029216919091179055611cd684611957565b60028054600090611cf19084906001600160801b0316613858565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550611d1e83612928565b60048054600090611d33908490600f0b613957565b92506101000a8154816001600160801b030219169083600f0b6001600160801b03160217905550611d6385611957565b60028054601090611d85908490600160801b90046001600160801b03166138cc565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550505050505050565b600080611dc984611dc38a888a6118e2565b90612858565b9050611dd68884866118e2565b611de090826136bc565b905086811115611df05786810391505b509695505050505050565b60006001600160701b038211156119815760405163086b151760e11b815260040160405180910390fd5b6002546001600160801b031683811080611e6757507f0000000000000000000000000000000000000000000000000000000000000000611e6585836137dd565b105b15611e7457611e746124f9565b6004549084900390600f0b611e898482613878565b905083611e9586611c0d565b138015611eca57507f0000000000000000000000000000000000000000000000000000000000000000611ec8838361286d565b105b15611ed757611ed76124f9565b600354600160801b90046001600160801b0316611f19610ff282611f03670de0b6b3a7640000886138b5565b6005546001600160801b031691908c600061255e565b600580546001600160801b0319166001600160801b0392909216919091179055611f4388826137dd565b9050611f4e81611957565b600380546001600160801b03928316600160801b029216919091179055611f7483611957565b600280546001600160801b0319166001600160801b0392909216919091179055611f9d82612928565b600480546001600160801b0319166001600160801b0392909216919091179055611fc687611957565b60028054601090611fe8908490600160801b90046001600160801b0316613858565b92506101000a8154816001600160801b0302191690836001600160801b031602179055505050505050505050565b60035461203190610ff2906001600160801b03168484612964565b600380546001600160801b0319166001600160801b03929092169190911790555050565b600754600360f81b6000908152600b6020527f3ae204c42bf80d9df0ca83c69a5573417a7a5570428fcb513b3a0276db3e754354909182916120a0916001600160801b0316906137dd565b9050806000036120b4576001915050610689565b60006120bf856129cb565b9050806000036120d457600192505050610689565b6000806120e2838589612a67565b91509150806120f8576000945050505050610689565b60008073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__632c03ef68858a6040518363ffffffff1660e01b8152600401612134929190613a02565b6040805180830381865af4158015612150573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906121749190613a78565b9150915061219261218482611c0d565b61218d90613a9c565b612bdb565b9250826121a85760009650505050505050610689565b6121b182611957565b600780546000906121cc9084906001600160801b0316613858565b92506101000a8154816001600160801b0302191690836001600160801b031602179055506121f981611957565b6007805460109061221b908490600160801b90046001600160801b0316613858565b92506101000a8154816001600160801b0302191690836001600160801b031602179055506001965050505050505092915050565b60008060008073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__635a1b419e61227887612d73565b6040518263ffffffff1660e01b81526004016122949190613ab8565b6040805180830381865af41580156122b0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906122d49190613ac7565b91509150806122ea575060009485945092505050565b600754600b6020527f3ae204c42bf80d9df0ca83c69a5573417a7a5570428fcb513b3a0276db3e75435460008080527fdf7de25b7f1fd6d0b5205f0e18f1f35bd7b8d84cce336588d184533ce43a6f765490926001600160801b031691612350916136bc565b61235a91906137dd565b90508060000361237257506000958695509350505050565b600080871161238257600061238d565b61238d8488846118e2565b976001975095505050505050565b6000606060008060008661ffff1667ffffffffffffffff8111156123c1576123c1613af7565b6040519080825280601f01601f1916602001820160405280156123eb576020820181803683370190505b5090506000808751602089018b8e8ef191503d92508683111561240c578692505b828152826000602083013e90999098509650505050505050565b600061244787878761244088670de0b6b3a76400006137dd565b8787612ead565b979650505050505050565b600061153761249361246c84670de0b6b3a76400006137dd565b611457670de0b6b3a7640000612482818a61146e565b61248c91906137dd565b87906126d8565b6124a590670de0b6b3a76400006136bc565b611dc384670de0b6b3a76400006137dd565b6000806124ee856124c6611916565b6124d091906136bc565b600254610ec1908790600160801b90046001600160801b03166137dd565b909210949350505050565b60405163bb55fd2760e01b815260040160405180910390fd5b6000806000806000612525898789612ed0565b909250905061253482896137dd565b9750600061254382888a6118e2565b905061254f818b6137dd565b9a989950979650505050505050565b60008260000361256f57508461194e565b81156125c2576125a061258284876136bc565b61258c8587610671565b612596888a610671565b611dc391906136bc565b905060006125ae8588611985565b9050808210156125bc578091505b5061194e565b8285036125d15750600061194e565b6125fc6125de84876137dd565b6125e885876126d8565b6125f2888a610671565b611dc391906137dd565b9695505050505050565b600061262e600b600061261a6002866111f8565b815260200190815260200160002054611c0d565b612640600b600061261a6001876111f8565b6106899190613878565b60006126767f0000000000000000000000000000000000000000000000000000000000000000836126d8565b60035461268c91906001600160801b03166136bc565b6002546126a2906001600160801b031684610671565b101592915050565b6000610689826004612055565b60006124478787876126d188670de0b6b3a76400006137dd565b8787612f50565b60006106868383670de0b6b3a7640000612832565b600080808061273e87866127378b6114576127108c670de0b6b3a76400006137dd565b7f0000000000000000000000000000000000000000000000000000000000000000906126d8565b9190612832565b935061276a847f0000000000000000000000000000000000000000000000000000000000000000610671565b9150600061278b61278389670de0b6b3a76400006137dd565b8a9088612832565b90506127b7817f00000000000000000000000000000000000000000000000000000000000000006126d8565b93506127e3847f0000000000000000000000000000000000000000000000000000000000000000610671565b6127ed90846136bc565b915050945094509450949050565b600081831161280a5781610686565b5090919050565b600080612829846128238a888a612832565b9061146e565b9050611dd68884865b600082600019048411830215820261284957600080fd5b50910281810615159190040190565b600061068683670de0b6b3a7640000846118e2565b60008061287a8484612f61565b90925090508061288c5761288c6124f9565b5092915050565b6000816000036128ac5750670de0b6b3a7640000610689565b826000036128bc57506000610689565b60006128c783611c0d565b905060006128dc6128d786611c0d565b612f9d565b90508181026128f3670de0b6b3a764000082613b0d565b90506125fc816131c3565b600060016001607f1b038211156119815760405163a5353be560e01b815260040160405180910390fd5b600060016001607f1b0319821280612946575060016001607f1b0382135b156119815760405163a5353be560e01b815260040160405180910390fd5b600080612972846000613358565b61297d846000613358565b6129879190613878565b905060008113156129a35761299c81866136bc565b94506129c2565b60008112156129c2576129b581613a9c565b6129bf90866137dd565b94505b50929392505050565b60035460009081906129e6906001600160801b03168461146e565b9050612a127f0000000000000000000000000000000000000000000000000000000000000000826136bc565b6002546001600160801b03161115611994576002547f000000000000000000000000000000000000000000000000000000000000000090612a5d9083906001600160801b03166137dd565b61153a91906137dd565b612a6f613594565b600080612a7b84612d73565b9050600073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__635a1b419e836040518263ffffffff1660e01b8152600401612ab69190613ab8565b6040805180830381865af4158015612ad2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612af69190613ac7565b9350905082612b0b57506000915061183d9050565b6000612b32612b2d84610160015185610140015161067190919063ffffffff16565b611c0d565b612b52612b2d8561012001518661010001516126d890919063ffffffff16565b612b5c9190613878565b604080516101208101825285815260208082019590955260008052600b85527fdf7de25b7f1fd6d0b5205f0e18f1f35bd7b8d84cce336588d184533ce43a6f7654818301526060810199909952608089019990995260a088015250815160c087015281015160e086015290940151610100840152509092600192509050565b6002546004805460405163685a2be760e11b81526001600160801b03808516938201849052600f9290920b60248201819052600160801b909404909116604482018190527f000000000000000000000000000000000000000000000000000000000000000060648301526084820185905260009391849081908190819073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__9063d0b457ce9060a401608060405180830381865af4158015612c94573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612cb89190613b3b565b935093509350935080612cd45750600098975050505050505050565b868414612d0557612ce484611957565b600280546001600160801b0319166001600160801b03929092169190911790555b858314612d3657612d1583612928565b600480546001600160801b0319166001600160801b03929092169190911790555b848214612d6457612d4682611957565b600280546001600160801b03928316600160801b0292169190911790555b50600198975050505050505050565b612d7b6135e6565b60408051610180810182526002546001600160801b038082168352600454600f0b6020840152600160801b91829004811693830193909352606082018590527f000000000000000000000000000000000000000000000000000000000000000060808301527f000000000000000000000000000000000000000000000000000000000000000060a08301527f000000000000000000000000000000000000000000000000000000000000000060c08301527f000000000000000000000000000000000000000000000000000000000000000060e08301526003540482166101008201526005549091610120830191612e739116613367565b81526004546001600160801b03600160801b9182900481166020840152600554604090930192612ea592900416613367565b905292915050565b600080612ebe8888888888886133dd565b909250905080611df057611df06124f9565b600080612f1a8561145785817f0000000000000000000000000000000000000000000000000000000000000000670de0b6b3a7640000612f10818c61146e565b61145791906137dd565b9150612f46827f0000000000000000000000000000000000000000000000000000000000000000610671565b9050935093915050565b600080612ebe8888888888886134a5565b600080600083612f7086611c0d565b612f7a9190613878565b90506000811215612f9257600080925092505061066a565b946001945092505050565b6000808213612fbf5760405163e61b497560e01b815260040160405180910390fd5b506001600160801b03811160071b81811c67ffffffffffffffff1060061b1781811c63ffffffff1060051b1781811c61ffff1060041b1781811c60ff10600390811b90911782811c600f1060021b1782811c909110600190811b90911782811c90911017609f8181036060019290921b91605f198201906130429084901c611c0d565b6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d6c8c3f38e95a6b1ff2ab1c3b343619018302821d6d02384773bdf1ac5676facced60901901830290911d6cb9a025d814b29c212b8b1a07cd1901909102780a09507084cc699bb0e71ea869ffffffffffffffffffffffff190105711340daa0d5f769dba1915cef59f0815a5506027d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b391909102017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d92915050565b6000680248ce36a70cb26b3e1982136131de57506000919050565b680755bf798b4a1bf1e58212613207576040516373a2d6b160e01b815260040160405180910390fd5b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056001605f1b01901d6bb17217f7d1cf79abc9e3b3988102909303926c240c330e9fb2d9cbaf0fd5aafb1984018402606090811d6d0277594991cfc85f6e2461837cd9018502811d6d1a521255e34f6a5061b25ef1c9c319018502811d6db1bbb201f443cf962f1a1d3db4a5018502811d6e02c72388d9f74f51a9331fed693f1419018502811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765087018702831d9081019087016d01d3967ed30fc4f89c02bab570811901810290921d6e0587f503bb6ea29d25fcb7401964500186026d360d7aeea093263ecc6e0ecb291760621b0181810595509293509091906125fc74029d9dc38563c32e5c2f6dc192ee70ef65f9978af3860260c38690031c611c0d565b600081831361280a5781610686565b600080670de0b6b3a764000061337b61068f565b61338591906138b5565b905080831161339557600061339f565b61339f81846137dd565b915061153a6133d6670de0b6b3a76400007f00000000000000000000000000000000000000000000000000000000000000006138b5565b8390612858565b60008060006133ef8989888888613565565b9050613409866119486134028a8d6136bc565b8790610671565b9850613416858a866118e2565b98508881101561342d57600080925092505061349a565b888103670de0b6b3a764000081106134625761345b613454670de0b6b3a76400008961146e565b8290612893565b905061347a565b613477613454670de0b6b3a764000089612858565b90505b808910156134905760008093509350505061349a565b8803925060019150505b965096945050505050565b60008060006134b78989888888613565565b90506134c786611948898b6136bc565b9750878110156134de57600080925092505061349a565b8781036134ec818688612832565b9050670de0b6b3a7640000811061351957613512613454670de0b6b3a76400008961146e565b9050613531565b61352e613454670de0b6b3a764000089612858565b90505b61353b818661146e565b9050808a10156135535760008093509350505061349a565b90980398600198509650505050505050565b60006135718585612893565b61358a61358286611948868b6126d8565b859085612832565b6125fc91906136bc565b6040518061012001604052806135a86135e6565b815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b6040518061018001604052806000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b6000806000806080858703121561365d57600080fd5b843593506020850135925060408501359150606085013567ffffffffffffffff81111561368957600080fd5b85016060818803121561369b57600080fd5b939692955090935050565b634e487b7160e01b600052601160045260246000fd5b80820180821115610689576106896136a6565b6001600160a01b038116811461059957600080fd5b6000602082840312156136f657600080fd5b813561153a816136cf565b801515811461059957600080fd5b60006020828403121561372157600080fd5b813561153a81613701565b6000808335601e1984360301811261374357600080fd5b83018035915067ffffffffffffffff82111561375e57600080fd5b60200191503681900382131561066a57600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b878152866020820152856040820152841515606082015283608082015260c060a082015260006137d060c083018486613773565b9998505050505050505050565b81810381811115610689576106896136a6565b88815287602082015286604082015285151560608201528460808201528360a082015260e060c0820152600061382a60e083018486613773565b9a9950505050505050505050565b6001600160701b0381811683821601908082111561288c5761288c6136a6565b6001600160801b0381811683821601908082111561288c5761288c6136a6565b818103600083128015838313168383128216171561288c5761288c6136a6565b6000602082840312156138aa57600080fd5b815161153a816136cf565b8082028115828204841417610689576106896136a6565b6001600160801b0382811682821603908082111561288c5761288c6136a6565b808201828112600083128015821682158216171561390c5761390c6136a6565b505092915050565b60006020828403121561392657600080fd5b5051919050565b634e487b7160e01b600052601260045260246000fd5b6000826139525761395261392d565b500690565b600f81810b9083900b0160016001607f1b03811360016001607f1b031982121715610689576106896136a6565b805182526020810151602083015260408101516040830152606081015160608301526080810151608083015260a081015160a083015260c081015160c083015260e081015160e08301526101008082015181840152506101208082015181840152506101408082015181840152506101608082015181840152505050565b60006102a082019050613a16828551613984565b602084015161018083015260408401516101a083015260608401516101c083015260808401516101e083015260a084015161020083015260c084015161022083015260e084015161024083015261010090930151610260820152610280015290565b60008060408385031215613a8b57600080fd5b505080516020909101519092909150565b6000600160ff1b8201613ab157613ab16136a6565b5060000390565b61018081016106898284613984565b60008060408385031215613ada57600080fd5b825191506020830151613aec81613701565b809150509250929050565b634e487b7160e01b600052604160045260246000fd5b600082613b1c57613b1c61392d565b600160ff1b821460001984141615613b3657613b366136a6565b500590565b60008060008060808587031215613b5157600080fd5b845193506020850151925060408501519150606085015161369b8161370156fea26469706673582212206750ca5aa91eef20858e22237af1d1bbc7d3cd2df03871b220abc6469a95ed6664736f6c63430008160033",
            [
                (
                    "contracts/src/libraries/LPMath.sol:LPMath",
                    libs.lp_math,
                )
            ],
            client.clone(),
        ).unwrap();
        let deployer = factory.deploy(constructor_args)?;
        let deployer = ::ethers::contract::ContractDeployer::new(deployer);
        Ok(deployer)
    }
}

