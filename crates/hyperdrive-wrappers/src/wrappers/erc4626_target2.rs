pub use erc4626_target_2::*;
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
pub mod erc4626_target_2 {
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
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AddressInsufficientBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
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
                    ::std::borrow::ToOwned::to_owned("NotPayable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotPayable"),
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
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeERC20FailedOperation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
    pub static ERC4626TARGET2_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct ERC4626Target2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC4626Target2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC4626Target2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC4626Target2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC4626Target2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ERC4626Target2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC4626Target2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC4626TARGET2_ABI.clone(),
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
            ERC4626Target2Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC4626Target2<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
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
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `AddressInsufficientBalance` with signature `AddressInsufficientBalance(address)` and selector `0xcd786059`
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
        name = "AddressInsufficientBalance",
        abi = "AddressInsufficientBalance(address)"
    )]
    pub struct AddressInsufficientBalance {
        pub account: ::ethers::core::types::Address,
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
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
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
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
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
    ///Custom Error type `NotPayable` with signature `NotPayable()` and selector `0x1574f9f3`
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
    #[etherror(name = "NotPayable", abi = "NotPayable()")]
    pub struct NotPayable;
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
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
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
        name = "SafeERC20FailedOperation",
        abi = "SafeERC20FailedOperation(address)"
    )]
    pub struct SafeERC20FailedOperation {
        pub token: ::ethers::core::types::Address,
    }
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
    pub enum ERC4626Target2Errors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        DistributeExcessIdleFailed(DistributeExcessIdleFailed),
        ExpInvalidExponent(ExpInvalidExponent),
        FailedInnerCall(FailedInnerCall),
        InsufficientLiquidity(InsufficientLiquidity),
        InvalidTimestamp(InvalidTimestamp),
        LnInvalidInput(LnInvalidInput),
        MinimumSharePrice(MinimumSharePrice),
        MinimumTransactionAmount(MinimumTransactionAmount),
        NotPayable(NotPayable),
        OutputLimit(OutputLimit),
        PoolIsPaused(PoolIsPaused),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        RestrictedZeroAddress(RestrictedZeroAddress),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        TransferFailed(TransferFailed),
        UnsafeCastToInt128(UnsafeCastToInt128),
        UnsafeCastToInt256(UnsafeCastToInt256),
        UnsafeCastToUint112(UnsafeCastToUint112),
        UnsafeCastToUint128(UnsafeCastToUint128),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ERC4626Target2Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) = <AddressInsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressInsufficientBalance(decoded));
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
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedInnerCall(decoded));
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
            if let Ok(decoded) = <NotPayable as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotPayable(decoded));
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
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
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
    impl ::ethers::core::abi::AbiEncode for ERC4626Target2Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressInsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DistributeExcessIdleFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpInvalidExponent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => {
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
                Self::NotPayable(element) => {
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
                Self::SafeERC20FailedOperation(element) => {
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
    impl ::ethers::contract::ContractRevert for ERC4626Target2Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AddressInsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DistributeExcessIdleFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ExpInvalidExponent as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
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
                    == <NotPayable as ::ethers::contract::EthError>::selector() => true,
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
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for ERC4626Target2Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DistributeExcessIdleFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExpInvalidExponent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::LnInvalidInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumSharePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumTransactionAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutputLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolIsPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RestrictedZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeERC20FailedOperation(element) => {
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
    impl ::core::convert::From<::std::string::String> for ERC4626Target2Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for ERC4626Target2Errors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for ERC4626Target2Errors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<DistributeExcessIdleFailed> for ERC4626Target2Errors {
        fn from(value: DistributeExcessIdleFailed) -> Self {
            Self::DistributeExcessIdleFailed(value)
        }
    }
    impl ::core::convert::From<ExpInvalidExponent> for ERC4626Target2Errors {
        fn from(value: ExpInvalidExponent) -> Self {
            Self::ExpInvalidExponent(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for ERC4626Target2Errors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidity> for ERC4626Target2Errors {
        fn from(value: InsufficientLiquidity) -> Self {
            Self::InsufficientLiquidity(value)
        }
    }
    impl ::core::convert::From<InvalidTimestamp> for ERC4626Target2Errors {
        fn from(value: InvalidTimestamp) -> Self {
            Self::InvalidTimestamp(value)
        }
    }
    impl ::core::convert::From<LnInvalidInput> for ERC4626Target2Errors {
        fn from(value: LnInvalidInput) -> Self {
            Self::LnInvalidInput(value)
        }
    }
    impl ::core::convert::From<MinimumSharePrice> for ERC4626Target2Errors {
        fn from(value: MinimumSharePrice) -> Self {
            Self::MinimumSharePrice(value)
        }
    }
    impl ::core::convert::From<MinimumTransactionAmount> for ERC4626Target2Errors {
        fn from(value: MinimumTransactionAmount) -> Self {
            Self::MinimumTransactionAmount(value)
        }
    }
    impl ::core::convert::From<NotPayable> for ERC4626Target2Errors {
        fn from(value: NotPayable) -> Self {
            Self::NotPayable(value)
        }
    }
    impl ::core::convert::From<OutputLimit> for ERC4626Target2Errors {
        fn from(value: OutputLimit) -> Self {
            Self::OutputLimit(value)
        }
    }
    impl ::core::convert::From<PoolIsPaused> for ERC4626Target2Errors {
        fn from(value: PoolIsPaused) -> Self {
            Self::PoolIsPaused(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for ERC4626Target2Errors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<RestrictedZeroAddress> for ERC4626Target2Errors {
        fn from(value: RestrictedZeroAddress) -> Self {
            Self::RestrictedZeroAddress(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for ERC4626Target2Errors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<TransferFailed> for ERC4626Target2Errors {
        fn from(value: TransferFailed) -> Self {
            Self::TransferFailed(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToInt128> for ERC4626Target2Errors {
        fn from(value: UnsafeCastToInt128) -> Self {
            Self::UnsafeCastToInt128(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToInt256> for ERC4626Target2Errors {
        fn from(value: UnsafeCastToInt256) -> Self {
            Self::UnsafeCastToInt256(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToUint112> for ERC4626Target2Errors {
        fn from(value: UnsafeCastToUint112) -> Self {
            Self::UnsafeCastToUint112(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToUint128> for ERC4626Target2Errors {
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
    pub enum ERC4626Target2Events {
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
    impl ::ethers::contract::EthLogDecode for ERC4626Target2Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddLiquidityFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::AddLiquidityFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = CloseLongFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::CloseLongFilter(decoded));
            }
            if let Ok(decoded) = CloseShortFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::CloseShortFilter(decoded));
            }
            if let Ok(decoded) = CollectGovernanceFeeFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::CollectGovernanceFeeFilter(decoded));
            }
            if let Ok(decoded) = CreateCheckpointFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::CreateCheckpointFilter(decoded));
            }
            if let Ok(decoded) = InitializeFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::InitializeFilter(decoded));
            }
            if let Ok(decoded) = OpenLongFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::OpenLongFilter(decoded));
            }
            if let Ok(decoded) = OpenShortFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::OpenShortFilter(decoded));
            }
            if let Ok(decoded) = PauseStatusUpdatedFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::PauseStatusUpdatedFilter(decoded));
            }
            if let Ok(decoded) = RedeemWithdrawalSharesFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::RedeemWithdrawalSharesFilter(decoded));
            }
            if let Ok(decoded) = RemoveLiquidityFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::RemoveLiquidityFilter(decoded));
            }
            if let Ok(decoded) = SweepFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::SweepFilter(decoded));
            }
            if let Ok(decoded) = TransferSingleFilter::decode_log(log) {
                return Ok(ERC4626Target2Events::TransferSingleFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ERC4626Target2Events {
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
    impl ::core::convert::From<AddLiquidityFilter> for ERC4626Target2Events {
        fn from(value: AddLiquidityFilter) -> Self {
            Self::AddLiquidityFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalFilter> for ERC4626Target2Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for ERC4626Target2Events {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<CloseLongFilter> for ERC4626Target2Events {
        fn from(value: CloseLongFilter) -> Self {
            Self::CloseLongFilter(value)
        }
    }
    impl ::core::convert::From<CloseShortFilter> for ERC4626Target2Events {
        fn from(value: CloseShortFilter) -> Self {
            Self::CloseShortFilter(value)
        }
    }
    impl ::core::convert::From<CollectGovernanceFeeFilter> for ERC4626Target2Events {
        fn from(value: CollectGovernanceFeeFilter) -> Self {
            Self::CollectGovernanceFeeFilter(value)
        }
    }
    impl ::core::convert::From<CreateCheckpointFilter> for ERC4626Target2Events {
        fn from(value: CreateCheckpointFilter) -> Self {
            Self::CreateCheckpointFilter(value)
        }
    }
    impl ::core::convert::From<InitializeFilter> for ERC4626Target2Events {
        fn from(value: InitializeFilter) -> Self {
            Self::InitializeFilter(value)
        }
    }
    impl ::core::convert::From<OpenLongFilter> for ERC4626Target2Events {
        fn from(value: OpenLongFilter) -> Self {
            Self::OpenLongFilter(value)
        }
    }
    impl ::core::convert::From<OpenShortFilter> for ERC4626Target2Events {
        fn from(value: OpenShortFilter) -> Self {
            Self::OpenShortFilter(value)
        }
    }
    impl ::core::convert::From<PauseStatusUpdatedFilter> for ERC4626Target2Events {
        fn from(value: PauseStatusUpdatedFilter) -> Self {
            Self::PauseStatusUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<RedeemWithdrawalSharesFilter> for ERC4626Target2Events {
        fn from(value: RedeemWithdrawalSharesFilter) -> Self {
            Self::RedeemWithdrawalSharesFilter(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityFilter> for ERC4626Target2Events {
        fn from(value: RemoveLiquidityFilter) -> Self {
            Self::RemoveLiquidityFilter(value)
        }
    }
    impl ::core::convert::From<SweepFilter> for ERC4626Target2Events {
        fn from(value: SweepFilter) -> Self {
            Self::SweepFilter(value)
        }
    }
    impl ::core::convert::From<TransferSingleFilter> for ERC4626Target2Events {
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
    pub enum ERC4626Target2Calls {
        OpenLong(OpenLongCall),
        OpenShort(OpenShortCall),
    }
    impl ::ethers::core::abi::AbiDecode for ERC4626Target2Calls {
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
    impl ::ethers::core::abi::AbiEncode for ERC4626Target2Calls {
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
    impl ::core::fmt::Display for ERC4626Target2Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OpenLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::OpenShort(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OpenLongCall> for ERC4626Target2Calls {
        fn from(value: OpenLongCall) -> Self {
            Self::OpenLong(value)
        }
    }
    impl ::core::convert::From<OpenShortCall> for ERC4626Target2Calls {
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

pub struct ERC4626Target2Libs {
    pub lp_math: ::ethers::types::Address,
}

impl<M: ::ethers::providers::Middleware> ERC4626Target2<M> {
    pub fn link_and_deploy<T: ::ethers::core::abi::Tokenize>(
        client: ::std::sync::Arc<M>,
        constructor_args: T,
        libs: ERC4626Target2Libs,
    ) -> ::core::result::Result<
        ::ethers::contract::builders::ContractDeployer<M, Self>,
        ::ethers::contract::ContractError<M>,
    > {
        let factory = crate::linked_factory::create(
            ERC4626TARGET2_ABI.clone(),
            "0x6102806040523480156200001257600080fd5b50604051620043a1380380620043a18339810160408190526200003591620001ae565b600160005581516001600160a01b039081166080908152602080850151831660a0908152918501516101a052908401516101c05260c0808501516101e090815260e080870151610200526101208088015190935261010080880151909152610140808801519091529086018051519092528151909201519091528051604090810151610160529051606090810151610180529084015182166102405290920151610260521661022052620002eb565b60405161020081016001600160401b03811182821017156200011657634e487b7160e01b600052604160045260246000fd5b60405290565b80516001600160a01b03811681146200013457600080fd5b919050565b6000608082840312156200014c57600080fd5b604051608081016001600160401b03811182821017156200017d57634e487b7160e01b600052604160045260246000fd5b8060405250809150825181526020830151602082015260408301516040820152606083015160608201525092915050565b600080828403610280811215620001c457600080fd5b61026080821215620001d557600080fd5b620001df620000e4565b9150620001ec856200011c565b8252620001fc602086016200011c565b60208301526200020f604086016200011c565b6040830152606085015160608301526080850151608083015260a085015160a083015260c085015160c083015260e085015160e08301526101008086015181840152506101208086015181840152506101408086015181840152506101606200027a8187016200011c565b908301526101806200028e8682016200011c565b908301526101a0620002a28682016200011c565b908301526101c0620002b68682016200011c565b908301526101e0620002cb8787830162000139565b8184015250819350620002e08186016200011c565b925050509250929050565b60805160a05160c05160e05161010051610120516101405161016051610180516101a0516101c0516101e05161020051610220516102405161026051613ee2620004bf60003960005050600050506000610c6001526000505060008181610100015281816103100152612ecb01526000818161153801528181611d9601528181611df9015281816125ab01528181612ab001528181612aec01528181612ce40152612ea501526000818161080e01528181610dc001528181610e0201528181610ef40152818161136a015281816113c90152612e7f015260006119b5015260008181611adb015281816126a0015281816127190152612fe50152600081816109d201528181610e750152818161146401528181611aaf01526126ed015260008181610e540152818161266c0152612fa101526000818161082f01528181610d9e01528181610e2301528181610f1501528181611348015281816113ea0152612ef10152600081816101de015281816103a80152818161090a01526135800152600081816106c801528181610748015281816107be0152818161088a01526108c20152600081816114bb015281816117030152818161177d015261180f0152600081816116d601526117370152613ee26000f3fe6080604052600436106100295760003560e01c8063cba2e58d1461002e578063dbbe80701461005a575b600080fd5b61004161003c366004613936565b61006d565b6040805192835260208301919091520160405180910390f35b610041610068366004613936565b61008a565b60008061007c86868686610099565b915091505b94509492505050565b60008061007c868686866102c9565b6000806100a4610549565b600654610100900460ff16156100cd576040516321081abf60e01b815260040160405180910390fd5b6100d5610573565b6100de83610594565b6000806100eb88866105cd565b909250905060006100fc83836106a2565b90507f000000000000000000000000000000000000000000000000000000000000000081101561013f5760405163211ddda360e11b815260040160405180910390fd5b86821015610160576040516342af972b60e01b815260040160405180910390fd5b600061016a6106c0565b905061017a8184600460016106f1565b50600080600061018a8787610d6f565b919a5091945090925090508b8810156101b65760405163c972651760e01b815260040160405180910390fd5b81600960008282546101c891906139ab565b909155506101d99050844283610f51565b6102037f0000000000000000000000000000000000000000000000000000000000000000856139ab565b98506102118389888c61108f565b600061021e60018b611229565b90506102378161023160208e018e6139d3565b8b61125e565b8d8a8a898e8561024a60208301836139d3565b6001600160a01b03167f7fc9757758f4c7f2eb9f011c4500beb349847d2f2acbdd5ffce3e2f01e79903a86888661028760408801602089016139fe565b8961029560408a018a613a1b565b6040516102a89796959493929190613a8b565b60405180910390a350909b5050505050505050505050506100816001600055565b6000806102d4610549565b600654610100900460ff16156102fd576040516321081abf60e01b815260040160405180910390fd5b610305610573565b61030e83610594565b7f000000000000000000000000000000000000000000000000000000000000000086101561034f5760405163211ddda360e11b815260040160405180910390fd5b6000610359611306565b90508481101561037c576040516342af972b60e01b815260040160405180910390fd5b60006103866106c0565b9050600061039f6103956106c0565b84600460016106f1565b905060006103cd7f0000000000000000000000000000000000000000000000000000000000000000846139ab565b90506000806000806103e08e8988611319565b60098054949850929650909450925083916000906103ff9084906139ab565b909155506104109050874283610f51565b61042261041d858a61149f565b6114b4565b61042d9060016139ab565b935050600061043d84898d6114e0565b9050808d10156104605760405163c972651760e01b815260040160405180910390fd5b61046a818c6105cd565b508e905061047a81858b89611512565b6000610487600288611229565b90508c6104a18261049b60208401846139d3565b8561125e565b858b86846104b260208601866139d3565b6001600160a01b03167ffa6dd2e3e152dbc3fe91196c0b8aa871c26fd7a1d07de126ec3159fd4ede2c758d8a866104ef60408b0160208c016139fe565b610503896104fd8a8d613acc565b906106a2565b8d61051160408e018e613a1b565b604051610525989796959493929190613adf565b60405180910390a350989d50939b5050505050505050505050506100816001600055565b60026000540361056c57604051633ee5aeb560e01b815260040160405180910390fd5b6002600055565b341561059257604051631574f9f360e01b815260040160405180910390fd5b565b60006105a360208301836139d3565b6001600160a01b0316036105ca5760405163f0dd15fd60e01b815260040160405180910390fd5b50565b816000806105e160408501602086016139fe565b15610606576105fc856105f76040870187613a1b565b6116c6565b909350905061061e565b503461061e856106196040870187613a1b565b611802565b610626611306565b9150801561069957604051600090339083908381818185875af1925050503d8060008114610670576040519150601f19603f3d011682016040523d82523d6000602084013e610675565b606091505b5050905080610697576040516312171d8360e31b815260040160405180910390fd5b505b505b9250929050565b60006106b78383670de0b6b3a764000061183c565b90505b92915050565b60006106ec427f000000000000000000000000000000000000000000000000000000000000000061185a565b905090565b600084815260086020526040812060018101546001600160801b031615158061071957504286115b1561073257600101546001600160801b03169050610d67565b600080600061073f6106c0565b9050600061076d7f00000000000000000000000000000000000000000000000000000000000000008b6139ab565b90505b818110156107e3576000818152600860205260409020600101546001600160801b031680156107bb576000828152600860205260409020549094506001600160801b031692506107e3565b507f000000000000000000000000000000000000000000000000000000000000000001610770565b83600003610856578893506108536107f9611870565b600254600160801b90046001600160801b03167f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000611891565b92505b50610860836118b1565b6001850180546001600160801b0319166001600160801b03929092169190911790556108b66108af7f00000000000000000000000000000000000000000000000000000000000000008b613acc565b8a84610f51565b6108f4896108ee6108e77f0000000000000000000000000000000000000000000000000000000000000000836139ab565b42906118df565b84610f51565b6108fd886118f4565b506000905060088161092f7f00000000000000000000000000000000000000000000000000000000000000008d613acc565b815260208101919091526040016000908120600101546001600160801b0316915061095b60028c611229565b6000818152600b60205260408120549192508c8c8315610aa1576001925060008061098a86898d866000611a98565b9150915080600960008282546109a091906139ab565b909155506109bc9050866000846109b681611b67565b88611b91565b6109c681836139ab565b91506109f686838a8e877f0000000000000000000000000000000000000000000000000000000000000000611d0b565b9150610a0a610a0583856106a2565b611d55565b60068054600290610a2b9084906201000090046001600160701b0316613b27565b92506101000a8154816001600160701b0302191690836001600160701b03160217905550610a58826118b1565b60068054601090610a7a908490600160801b90046001600160801b0316613b47565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555050505b6000610aae600184611229565b6000818152600b60205260409020549091508015610bbd5760019450600080610adb838b8f886001611a98565b915091508060096000828254610af191906139ab565b90915550610b0d905083600084610b0781611b67565b8a611d7f565b610b178183613acc565b9150610b26610a0583876106a2565b60068054600290610b479084906201000090046001600160701b0316613b27565b92506101000a8154816001600160701b0302191690836001600160701b03160217905550610b74826118b1565b60068054601090610b96908490600160801b90046001600160801b0316613b47565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555050505b8415610bf757610be9610bcf87611b67565b610bd883611b67565b610be29190613b67565b6000611f70565b8e610bf48482611faf565b50505b6000610c02846121a9565b50604080518e815260208101879052908101899052606081018490526080810182905290915085907fff888cf98d2696e95c8c39aa98c9ad55a5378008f7a56614c9353b7137a57ab79060a00160405180910390a260008f905060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f25964586040518163ffffffff1660e01b8152600401602060405180830381865afa158015610cbc573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ce09190613b87565b90506001600160a01b03811615610d5557610d525a604051336024820152604481018a90528415156064820152600090819060840160408051601f198184030181529190526020810180516001600160e01b0316633488a6a760e11b1790526001600160a01b038616939291906122f5565b50505b509b9d50505050505050505050505050505b949350505050565b6000806000806000610d7f611870565b600254909150610de4908290600160801b90046001600160801b0316897f00000000000000000000000000000000000000000000000000000000000000008a7f0000000000000000000000000000000000000000000000000000000000000000612380565b600254909450610e47908290600160801b90046001600160801b03167f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000611891565b9150610e9e8785610e99857f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000006123ac565b612411565b15610eab57610eab612453565b610eb78785888561246c565b91965094509250670de0b6b3a7640000610f39610ed487846139ab565b600254610ef2908890600160801b90046001600160801b0316613acc565b7f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000611891565b1115610f4757610f47612453565b5092959194509250565b600083815260086020526040902054600160801b90046001600160801b0316808303610f7d5750505050565b6000848152600860205260408120546001600160801b031690819003610fd757610fa6836118b1565b600086815260086020526040902080546001600160801b0319166001600160801b0392909216919091179055611055565b611028611023670de0b6b3a7640000610ff08886613acc565b610ffa9190613ba4565b85670de0b6b3a764000061100e878a613acc565b6110189190613ba4565b8592919060016124b8565b6118b1565b600086815260086020526040902080546001600160801b0319166001600160801b03929092169190911790555b61105e846118b1565b60009586526008602052604090952080546001600160801b03968716600160801b0296169590951790945550505050565b600354600160801b90046001600160801b03166110d1611023826110bb670de0b6b3a764000086613ba4565b6005546001600160801b031691908860016124b8565b600580546001600160801b0319166001600160801b03929092169190911790556110fa856118b1565b600280546000906111159084906001600160801b0316613b47565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550611142846118b1565b60028054601090611164908490600160801b90046001600160801b0316613bbb565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550611191846118b1565b61119b9082613b47565b600380546001600160801b03808416600160801b029116179055905060006111c283612560565b90506111e0816111d187611b67565b6111db9084613bdb565b611f70565b6111e9846125a4565b6111f5576111f5612453565b600061120085612604565b90508061122057604051638bdf918d60e01b815260040160405180910390fd5b50505050505050565b60006001600160f81b038211156112535760405163b7d0949760e01b815260040160405180910390fd5b5060f89190911b1790565b6000838152600a602090815260408083206001600160a01b0386168452909152812080548392906112909084906139ab565b90915550506000838152600b6020526040812080548392906112b39084906139ab565b909155505060408051848152602081018390526001600160a01b0384169160009133917fc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f62910160405180910390a4505050565b60006106ec670de0b6b3a76400006114b4565b6000806000806000611329611870565b60025490915061138e908290600160801b90046001600160801b03168a7f00000000000000000000000000000000000000000000000000000000000000008b7f0000000000000000000000000000000000000000000000000000000000000000612611565b93508761139b8589612632565b11156113a9576113a9612453565b600254600090819061140e908490600160801b90046001600160801b03167f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000611891565b93506114248a670de0b6b3a7640000868c612647565b5091935090915061143790508183613acc565b6114419087613acc565b95508861148e816114888d611456868c613acc565b8d6114618782612755565b877f000000000000000000000000000000000000000000000000000000000000000061276b565b90612632565b975090945050505093509350935093565b60006106b783670de0b6b3a76400008461278c565b60006106ba7f0000000000000000000000000000000000000000000000000000000000000000836127b2565b60006114f260408301602084016139fe565b156114fe57508261150b565b611508848461281e565b90505b9392505050565b6002546001600160801b03168381101561152e5761152e612453565b60045490849003907f000000000000000000000000000000000000000000000000000000000000000090611566908390600f0b612833565b101561157457611574612453565b6004546115be9061102390600160801b90046001600160801b03166115a1670de0b6b3a764000086613ba4565b600554600160801b90046001600160801b031691908960016124b8565b600580546001600160801b03928316600160801b0292169190911790556115e4816118b1565b600280546001600160801b0319166001600160801b039290921691909117905561160d856118b1565b6002805460109061162f908490600160801b90046001600160801b0316613b47565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555061165c856118b1565b6004805460109061167e908490600160801b90046001600160801b0316613b47565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555060006116ad83612560565b90506111e0816116bc88611b67565b6111db9084613b67565b6000806116fe6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016333088612859565b61175e7f000000000000000000000000000000000000000000000000000000000000000061172d8760016139ab565b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001691906128c6565b604051636e553f6560e01b8152600481018690523060248201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690636e553f65906044016020604051808303816000875af11580156117ce573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906117f29190613c03565b925060009150505b935093915050565b6118376001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016333086612859565b505050565b600082600019048411830215820261185357600080fd5b5091020490565b60006118668284613c32565b6106b79084613acc565b6002546004546000916106ec916001600160801b0390911690600f0b612833565b60006118a8826118a285888861183c565b90612956565b95945050505050565b60006001600160801b038211156118db57604051630f0af95160e11b815260040160405180910390fd5b5090565b60008183116118ee57826106b7565b50919050565b6006546000908190611917908490600160801b90046001600160801b03166106a2565b6006546201000090046001600160701b03169250905081811115611a935760006119418383613acc565b9050611950611023828661149f565b60068054601090611972908490600160801b90046001600160801b0316613bbb565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555060006119ab858361281e90919063ffffffff16565b905060006119d9827f00000000000000000000000000000000000000000000000000000000000000006106a2565b905080600960008282546119ed91906139ab565b909155506119fd90508183613acc565b9150611a08826118b1565b60028054600090611a239084906001600160801b0316613b47565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550611a50826129c1565b60048054600090611a65908490600f0b613c46565b92506101000a8154816001600160801b030219169083600f0b6001600160801b031602179055508493505050505b915091565b600080611aa5878561281e565b91506000611ad3837f00000000000000000000000000000000000000000000000000000000000000006106a2565b9050611aff817f00000000000000000000000000000000000000000000000000000000000000006106a2565b91508315611b2257611b118282613acc565b611b1b9084613acc565b9250611b39565b611b2c8282613acc565b611b3690846139ab565b92505b86861015611b5c57611b4c83878961183c565b9250611b5982878961183c565b91505b509550959350505050565b60006001600160ff1b038211156118db5760405163396ea70160e11b815260040160405180910390fd5b600454600160801b90046001600160801b0316611bda61102382611bbd670de0b6b3a764000086613ba4565b600554600160801b90046001600160801b031691908a60006124b8565b600580546001600160801b03928316600160801b029216919091179055611c00866118b1565b611c0a9082613bbb565b600480546001600160801b03928316600160801b029216919091179055611c30846118b1565b60028054600090611c4b9084906001600160801b0316613b47565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550611c78836129eb565b60048054600090611c8d908490600f0b613c46565b92506101000a8154816001600160801b030219169083600f0b6001600160801b03160217905550611cbd856118b1565b60028054601090611cdf908490600160801b90046001600160801b0316613bbb565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550505050505050565b600080611d2384611d1d8a888a61183c565b9061281e565b9050611d3088848661183c565b611d3a90826139ab565b905086811115611d4a5786810391505b509695505050505050565b60006001600160701b038211156118db5760405163086b151760e11b815260040160405180910390fd5b6002546001600160801b031683811080611dc157507f0000000000000000000000000000000000000000000000000000000000000000611dbf8583613acc565b105b15611dce57611dce612453565b6004549084900390600f0b611de38482613b67565b905083611def86611b67565b138015611e2457507f0000000000000000000000000000000000000000000000000000000000000000611e228383612833565b105b15611e3157611e31612453565b600354600160801b90046001600160801b0316611e7361102382611e5d670de0b6b3a764000088613ba4565b6005546001600160801b031691908c60006124b8565b600580546001600160801b0319166001600160801b0392909216919091179055611e9d8882613acc565b9050611ea8816118b1565b600380546001600160801b03928316600160801b029216919091179055611ece836118b1565b600280546001600160801b0319166001600160801b0392909216919091179055611ef7826129eb565b600480546001600160801b0319166001600160801b0392909216919091179055611f20876118b1565b60028054601090611f42908490600160801b90046001600160801b0316613b47565b92506101000a8154816001600160801b0302191690836001600160801b031602179055505050505050505050565b600354611f8b90611023906001600160801b03168484612a27565b600380546001600160801b0319166001600160801b03929092169190911790555050565b600754600360f81b6000908152600b6020527f3ae204c42bf80d9df0ca83c69a5573417a7a5570428fcb513b3a0276db3e75435490918291611ffa916001600160801b031690613acc565b90508060000361200e5760019150506106ba565b600061201985612a8e565b90508060000361202e576001925050506106ba565b60008061203c838589612b2a565b91509150806120525760009450505050506106ba565b60008073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__632c03ef68858a6040518363ffffffff1660e01b815260040161208e929190613cf1565b6040805180830381865af41580156120aa573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906120ce9190613d67565b915091506120ec6120de82611b67565b6120e790613d8b565b612c9e565b92508261210257600096505050505050506106ba565b61210b826118b1565b600780546000906121269084906001600160801b0316613b47565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550612153816118b1565b60078054601090612175908490600160801b90046001600160801b0316613b47565b92506101000a8154816001600160801b0302191690836001600160801b031602179055506001965050505050505092915050565b60008060008073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__635a1b419e6121d287612e36565b6040518263ffffffff1660e01b81526004016121ee9190613da7565b6040805180830381865af415801561220a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061222e9190613db6565b9150915080612244575060009485945092505050565b600754600b6020527f3ae204c42bf80d9df0ca83c69a5573417a7a5570428fcb513b3a0276db3e75435460008080527fdf7de25b7f1fd6d0b5205f0e18f1f35bd7b8d84cce336588d184533ce43a6f765490926001600160801b0316916122aa916139ab565b6122b49190613acc565b9050806000036122cc57506000958695509350505050565b60008087116122dc5760006122e7565b6122e784888461183c565b976001975095505050505050565b6000606060008060008661ffff1667ffffffffffffffff81111561231b5761231b613de6565b6040519080825280601f01601f191660200182016040528015612345576020820181803683370190505b5090506000808751602089018b8e8ef191503d925086831115612366578692505b828152826000602083013e90999098509650505050505050565b60006123a187878761239a88670de0b6b3a7640000613acc565b8787612f70565b979650505050505050565b60006115086123ed6123c684670de0b6b3a7640000613acc565b611488670de0b6b3a76400006123dc818a61149f565b6123e69190613acc565b8790612632565b6123ff90670de0b6b3a76400006139ab565b611d1d84670de0b6b3a7640000613acc565b60008061244885612420611870565b61242a91906139ab565b600254610ef2908790600160801b90046001600160801b0316613acc565b909210949350505050565b60405163bb55fd2760e01b815260040160405180910390fd5b600080600080600061247f898789612f93565b909250905061248e8289613acc565b9750600061249d82888a61183c565b90506124a9818b613acc565b9a989950979650505050505050565b6000826000036124c95750846118a8565b811561251c576124fa6124dc84876139ab565b6124e685876106a2565b6124f0888a6106a2565b611d1d91906139ab565b9050600061250885886118df565b905080821015612516578091505b506118a8565b82850361252b575060006118a8565b6125566125388487613acc565b6125428587612632565b61254c888a6106a2565b611d1d9190613acc565b9695505050505050565b6000612588600b6000612574600286611229565b815260200190815260200160002054611b67565b61259a600b6000612574600187611229565b6106ba9190613b67565b60006125d07f000000000000000000000000000000000000000000000000000000000000000083612632565b6003546125e691906001600160801b03166139ab565b6002546125fc906001600160801b0316846106a2565b101592915050565b60006106ba826004611faf565b60006123a187878761262b88670de0b6b3a7640000613acc565b8787613013565b60006106b78383670de0b6b3a764000061278c565b600080808061269887866126918b61148861266a8c670de0b6b3a7640000613acc565b7f000000000000000000000000000000000000000000000000000000000000000090612632565b919061278c565b93506126c4847f00000000000000000000000000000000000000000000000000000000000000006106a2565b915060006126e56126dd89670de0b6b3a7640000613acc565b8a908861278c565b9050612711817f0000000000000000000000000000000000000000000000000000000000000000612632565b935061273d847f00000000000000000000000000000000000000000000000000000000000000006106a2565b61274790846139ab565b915050945094509450949050565b600081831161276457816106b7565b5090919050565b6000806127838461277d8a888a61278c565b9061149f565b9050611d308884865b60008260001904841183021582026127a357600080fd5b50910281810615159190040190565b6040516303d1689d60e11b8152600481018290526000906001600160a01b038416906307a2d13a90602401602060405180830381865afa1580156127fa573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106b79190613c03565b60006106b783670de0b6b3a76400008461183c565b6000806128408484613024565b90925090508061285257612852612453565b5092915050565b6040516001600160a01b0384811660248301528381166044830152606482018390526128c09186918216906323b872dd906084015b604051602081830303815290604052915060e01b6020820180516001600160e01b038381831617835250505050613060565b50505050565b604080516001600160a01b038416602482015260448082018490528251808303909101815260649091019091526020810180516001600160e01b031663095ea7b360e01b17905261291784826130c8565b6128c0576040516001600160a01b0384811660248301526000604483015261294c91869182169063095ea7b39060640161288e565b6128c08482613060565b60008160000361296f5750670de0b6b3a76400006106ba565b8260000361297f575060006106ba565b600061298a83611b67565b9050600061299f61299a86611b67565b61316b565b90508181026129b6670de0b6b3a764000082613dfc565b905061255681613391565b600060016001607f1b038211156118db5760405163a5353be560e01b815260040160405180910390fd5b600060016001607f1b0319821280612a09575060016001607f1b0382135b156118db5760405163a5353be560e01b815260040160405180910390fd5b600080612a35846000613526565b612a40846000613526565b612a4a9190613b67565b90506000811315612a6657612a5f81866139ab565b9450612a85565b6000811215612a8557612a7881613d8b565b612a829086613acc565b94505b50929392505050565b6003546000908190612aa9906001600160801b03168461149f565b9050612ad57f0000000000000000000000000000000000000000000000000000000000000000826139ab565b6002546001600160801b031611156118ee576002547f000000000000000000000000000000000000000000000000000000000000000090612b209083906001600160801b0316613acc565b61150b9190613acc565b612b32613883565b600080612b3e84612e36565b9050600073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__635a1b419e836040518263ffffffff1660e01b8152600401612b799190613da7565b6040805180830381865af4158015612b95573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612bb99190613db6565b9350905082612bce5750600091506117fa9050565b6000612bf5612bf08461016001518561014001516106a290919063ffffffff16565b611b67565b612c15612bf085610120015186610100015161263290919063ffffffff16565b612c1f9190613b67565b604080516101208101825285815260208082019590955260008052600b85527fdf7de25b7f1fd6d0b5205f0e18f1f35bd7b8d84cce336588d184533ce43a6f7654818301526060810199909952608089019990995260a088015250815160c087015281015160e086015290940151610100840152509092600192509050565b6002546004805460405163685a2be760e11b81526001600160801b03808516938201849052600f9290920b60248201819052600160801b909404909116604482018190527f000000000000000000000000000000000000000000000000000000000000000060648301526084820185905260009391849081908190819073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__9063d0b457ce9060a401608060405180830381865af4158015612d57573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612d7b9190613e2a565b935093509350935080612d975750600098975050505050505050565b868414612dc857612da7846118b1565b600280546001600160801b0319166001600160801b03929092169190911790555b858314612df957612dd8836129eb565b600480546001600160801b0319166001600160801b03929092169190911790555b848214612e2757612e09826118b1565b600280546001600160801b03928316600160801b0292169190911790555b50600198975050505050505050565b612e3e6138d5565b60408051610180810182526002546001600160801b038082168352600454600f0b6020840152600160801b91829004811693830193909352606082018590527f000000000000000000000000000000000000000000000000000000000000000060808301527f000000000000000000000000000000000000000000000000000000000000000060a08301527f000000000000000000000000000000000000000000000000000000000000000060c08301527f000000000000000000000000000000000000000000000000000000000000000060e08301526003540482166101008201526005549091610120830191612f369116613535565b81526004546001600160801b03600160801b9182900481166020840152600554604090930192612f6892900416613535565b905292915050565b600080612f818888888888886135ab565b909250905080611d4a57611d4a612453565b600080612fdd8561148885817f0000000000000000000000000000000000000000000000000000000000000000670de0b6b3a7640000612fd3818c61149f565b6114889190613acc565b9150613009827f00000000000000000000000000000000000000000000000000000000000000006106a2565b9050935093915050565b600080612f81888888888888613673565b60008060008361303386611b67565b61303d9190613b67565b9050600081121561305557600080925092505061069b565b946001945092505050565b60006130756001600160a01b03841683613733565b9050805160001415801561309a5750808060200190518101906130989190613e60565b155b1561183757604051635274afe760e01b81526001600160a01b03841660048201526024015b60405180910390fd5b6000806000846001600160a01b0316846040516130e59190613e7d565b6000604051808303816000865af19150503d8060008114613122576040519150601f19603f3d011682016040523d82523d6000602084013e613127565b606091505b50915091508180156131515750805115806131515750808060200190518101906131519190613e60565b80156118a85750505050506001600160a01b03163b151590565b600080821361318d5760405163e61b497560e01b815260040160405180910390fd5b506001600160801b03811160071b81811c67ffffffffffffffff1060061b1781811c63ffffffff1060051b1781811c61ffff1060041b1781811c60ff10600390811b90911782811c600f1060021b1782811c909110600190811b90911782811c90911017609f8181036060019290921b91605f198201906132109084901c611b67565b6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d6c8c3f38e95a6b1ff2ab1c3b343619018302821d6d02384773bdf1ac5676facced60901901830290911d6cb9a025d814b29c212b8b1a07cd1901909102780a09507084cc699bb0e71ea869ffffffffffffffffffffffff190105711340daa0d5f769dba1915cef59f0815a5506027d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b391909102017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d92915050565b6000680248ce36a70cb26b3e1982136133ac57506000919050565b680755bf798b4a1bf1e582126133d5576040516373a2d6b160e01b815260040160405180910390fd5b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056001605f1b01901d6bb17217f7d1cf79abc9e3b3988102909303926c240c330e9fb2d9cbaf0fd5aafb1984018402606090811d6d0277594991cfc85f6e2461837cd9018502811d6d1a521255e34f6a5061b25ef1c9c319018502811d6db1bbb201f443cf962f1a1d3db4a5018502811d6e02c72388d9f74f51a9331fed693f1419018502811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765087018702831d9081019087016d01d3967ed30fc4f89c02bab570811901810290921d6e0587f503bb6ea29d25fcb7401964500186026d360d7aeea093263ecc6e0ecb291760621b01818105955092935090919061255674029d9dc38563c32e5c2f6dc192ee70ef65f9978af3860260c38690031c611b67565b600081831361276457816106b7565b600080670de0b6b3a76400006135496106c0565b6135539190613ba4565b905080831161356357600061356d565b61356d8184613acc565b915061150b6135a4670de0b6b3a76400007f0000000000000000000000000000000000000000000000000000000000000000613ba4565b839061281e565b60008060006135bd8989888888613741565b90506135d7866118a26135d08a8d6139ab565b87906106a2565b98506135e4858a8661183c565b9850888110156135fb576000809250925050613668565b888103670de0b6b3a7640000811061363057613629613622670de0b6b3a76400008961149f565b8290612956565b9050613648565b613645613622670de0b6b3a76400008961281e565b90505b8089101561365e57600080935093505050613668565b8803925060019150505b965096945050505050565b60008060006136858989888888613741565b9050613695866118a2898b6139ab565b9750878110156136ac576000809250925050613668565b8781036136ba81868861278c565b9050670de0b6b3a764000081106136e7576136e0613622670de0b6b3a76400008961149f565b90506136ff565b6136fc613622670de0b6b3a76400008961281e565b90505b613709818661149f565b9050808a101561372157600080935093505050613668565b90980398600198509650505050505050565b60606106b783836000613770565b600061374d8585612956565b61376661375e866118a2868b612632565b85908561278c565b61255691906139ab565b6060814710156137955760405163cd78605960e01b81523060048201526024016130bf565b600080856001600160a01b031684866040516137b19190613e7d565b60006040518083038185875af1925050503d80600081146137ee576040519150601f19603f3d011682016040523d82523d6000602084013e6137f3565b606091505b50915091506125568683836060826138135761380e8261385a565b61150b565b815115801561382a57506001600160a01b0384163b155b1561385357604051639996b31560e01b81526001600160a01b03851660048201526024016130bf565b508061150b565b80511561386a5780518082602001fd5b604051630a12f52160e11b815260040160405180910390fd5b6040518061012001604052806138976138d5565b815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b6040518061018001604052806000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b6000806000806080858703121561394c57600080fd5b843593506020850135925060408501359150606085013567ffffffffffffffff81111561397857600080fd5b85016060818803121561398a57600080fd5b939692955090935050565b634e487b7160e01b600052601160045260246000fd5b808201808211156106ba576106ba613995565b6001600160a01b03811681146105ca57600080fd5b6000602082840312156139e557600080fd5b813561150b816139be565b80151581146105ca57600080fd5b600060208284031215613a1057600080fd5b813561150b816139f0565b6000808335601e19843603018112613a3257600080fd5b83018035915067ffffffffffffffff821115613a4d57600080fd5b60200191503681900382131561069b57600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b878152866020820152856040820152841515606082015283608082015260c060a08201526000613abf60c083018486613a62565b9998505050505050505050565b818103818111156106ba576106ba613995565b88815287602082015286604082015285151560608201528460808201528360a082015260e060c08201526000613b1960e083018486613a62565b9a9950505050505050505050565b6001600160701b0381811683821601908082111561285257612852613995565b6001600160801b0381811683821601908082111561285257612852613995565b818103600083128015838313168383128216171561285257612852613995565b600060208284031215613b9957600080fd5b815161150b816139be565b80820281158282048414176106ba576106ba613995565b6001600160801b0382811682821603908082111561285257612852613995565b8082018281126000831280158216821582161715613bfb57613bfb613995565b505092915050565b600060208284031215613c1557600080fd5b5051919050565b634e487b7160e01b600052601260045260246000fd5b600082613c4157613c41613c1c565b500690565b600f81810b9083900b0160016001607f1b03811360016001607f1b0319821217156106ba576106ba613995565b805182526020810151602083015260408101516040830152606081015160608301526080810151608083015260a081015160a083015260c081015160c083015260e081015160e08301526101008082015181840152506101208082015181840152506101408082015181840152506101608082015181840152505050565b60006102a082019050613d05828551613c73565b602084015161018083015260408401516101a083015260608401516101c083015260808401516101e083015260a084015161020083015260c084015161022083015260e084015161024083015261010090930151610260820152610280015290565b60008060408385031215613d7a57600080fd5b505080516020909101519092909150565b6000600160ff1b8201613da057613da0613995565b5060000390565b61018081016106ba8284613c73565b60008060408385031215613dc957600080fd5b825191506020830151613ddb816139f0565b809150509250929050565b634e487b7160e01b600052604160045260246000fd5b600082613e0b57613e0b613c1c565b600160ff1b821460001984141615613e2557613e25613995565b500590565b60008060008060808587031215613e4057600080fd5b845193506020850151925060408501519150606085015161398a816139f0565b600060208284031215613e7257600080fd5b815161150b816139f0565b6000825160005b81811015613e9e5760208186018101518583015201613e84565b50600092019182525091905056fea26469706673582212206f2bbfab1a773b68aeb201f3670d0267dc78fcc0d90068bf2cb15fbb3e4a861c64736f6c63430008160033",
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

