pub use erc4626_target_1::*;
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
pub mod erc4626_target_1 {
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
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("closeLong"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("closeLong"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maturityTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("_minOutput"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("closeShort"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("closeShort"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maturityTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("_minOutput"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("CheckpointRewarderUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CheckpointRewarderUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newCheckpointRewarder",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                    ::std::borrow::ToOwned::to_owned("FeeCollectorUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FeeCollectorUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newFeeCollector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GovernanceUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("GovernanceUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newGovernance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                    ::std::borrow::ToOwned::to_owned("PauserUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PauserUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPauser"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
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
                    ::std::borrow::ToOwned::to_owned("SweepCollectorUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SweepCollectorUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newSweepCollector"),
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
                    ::std::borrow::ToOwned::to_owned("InsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientBalance",
                            ),
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
    pub static ERC4626TARGET1_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct ERC4626Target1<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC4626Target1<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC4626Target1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC4626Target1<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC4626Target1<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ERC4626Target1))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC4626Target1<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC4626TARGET1_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `closeLong` (0xded06231) function
        pub fn close_long(
            &self,
            maturity_time: ::ethers::core::types::U256,
            bond_amount: ::ethers::core::types::U256,
            min_output: ::ethers::core::types::U256,
            options: Options,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [222, 208, 98, 49],
                    (maturity_time, bond_amount, min_output, options),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `closeShort` (0x29b23fc1) function
        pub fn close_short(
            &self,
            maturity_time: ::ethers::core::types::U256,
            bond_amount: ::ethers::core::types::U256,
            min_output: ::ethers::core::types::U256,
            options: Options,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [41, 178, 63, 193],
                    (maturity_time, bond_amount, min_output, options),
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
        ///Gets the contract's `CheckpointRewarderUpdated` event
        pub fn checkpoint_rewarder_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CheckpointRewarderUpdatedFilter,
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
        ///Gets the contract's `FeeCollectorUpdated` event
        pub fn fee_collector_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FeeCollectorUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `GovernanceUpdated` event
        pub fn governance_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GovernanceUpdatedFilter,
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
        ///Gets the contract's `PauserUpdated` event
        pub fn pauser_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PauserUpdatedFilter,
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
        ///Gets the contract's `SweepCollectorUpdated` event
        pub fn sweep_collector_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SweepCollectorUpdatedFilter,
        > {
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
            ERC4626Target1Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC4626Target1<M> {
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
    ///Custom Error type `InsufficientBalance` with signature `InsufficientBalance()` and selector `0xf4d678b8`
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
    #[etherror(name = "InsufficientBalance", abi = "InsufficientBalance()")]
    pub struct InsufficientBalance;
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
    pub enum ERC4626Target1Errors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        DistributeExcessIdleFailed(DistributeExcessIdleFailed),
        ExpInvalidExponent(ExpInvalidExponent),
        FailedInnerCall(FailedInnerCall),
        InsufficientBalance(InsufficientBalance),
        InsufficientLiquidity(InsufficientLiquidity),
        InvalidTimestamp(InvalidTimestamp),
        LnInvalidInput(LnInvalidInput),
        MinimumTransactionAmount(MinimumTransactionAmount),
        OutputLimit(OutputLimit),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        RestrictedZeroAddress(RestrictedZeroAddress),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        UnsafeCastToInt128(UnsafeCastToInt128),
        UnsafeCastToInt256(UnsafeCastToInt256),
        UnsafeCastToUint112(UnsafeCastToUint112),
        UnsafeCastToUint128(UnsafeCastToUint128),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ERC4626Target1Errors {
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
            if let Ok(decoded) = <InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientBalance(decoded));
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
    impl ::ethers::core::abi::AbiEncode for ERC4626Target1Errors {
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
                Self::InsufficientBalance(element) => {
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
                Self::MinimumTransactionAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutputLimit(element) => {
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
    impl ::ethers::contract::ContractRevert for ERC4626Target1Errors {
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
                    == <InsufficientBalance as ::ethers::contract::EthError>::selector() => {
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
                    == <MinimumTransactionAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutputLimit as ::ethers::contract::EthError>::selector() => true,
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
    impl ::core::fmt::Display for ERC4626Target1Errors {
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
                Self::InsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::LnInvalidInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumTransactionAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OutputLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RestrictedZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<::std::string::String> for ERC4626Target1Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for ERC4626Target1Errors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for ERC4626Target1Errors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<DistributeExcessIdleFailed> for ERC4626Target1Errors {
        fn from(value: DistributeExcessIdleFailed) -> Self {
            Self::DistributeExcessIdleFailed(value)
        }
    }
    impl ::core::convert::From<ExpInvalidExponent> for ERC4626Target1Errors {
        fn from(value: ExpInvalidExponent) -> Self {
            Self::ExpInvalidExponent(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for ERC4626Target1Errors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<InsufficientBalance> for ERC4626Target1Errors {
        fn from(value: InsufficientBalance) -> Self {
            Self::InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidity> for ERC4626Target1Errors {
        fn from(value: InsufficientLiquidity) -> Self {
            Self::InsufficientLiquidity(value)
        }
    }
    impl ::core::convert::From<InvalidTimestamp> for ERC4626Target1Errors {
        fn from(value: InvalidTimestamp) -> Self {
            Self::InvalidTimestamp(value)
        }
    }
    impl ::core::convert::From<LnInvalidInput> for ERC4626Target1Errors {
        fn from(value: LnInvalidInput) -> Self {
            Self::LnInvalidInput(value)
        }
    }
    impl ::core::convert::From<MinimumTransactionAmount> for ERC4626Target1Errors {
        fn from(value: MinimumTransactionAmount) -> Self {
            Self::MinimumTransactionAmount(value)
        }
    }
    impl ::core::convert::From<OutputLimit> for ERC4626Target1Errors {
        fn from(value: OutputLimit) -> Self {
            Self::OutputLimit(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for ERC4626Target1Errors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<RestrictedZeroAddress> for ERC4626Target1Errors {
        fn from(value: RestrictedZeroAddress) -> Self {
            Self::RestrictedZeroAddress(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for ERC4626Target1Errors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToInt128> for ERC4626Target1Errors {
        fn from(value: UnsafeCastToInt128) -> Self {
            Self::UnsafeCastToInt128(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToInt256> for ERC4626Target1Errors {
        fn from(value: UnsafeCastToInt256) -> Self {
            Self::UnsafeCastToInt256(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToUint112> for ERC4626Target1Errors {
        fn from(value: UnsafeCastToUint112) -> Self {
            Self::UnsafeCastToUint112(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToUint128> for ERC4626Target1Errors {
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
        name = "CheckpointRewarderUpdated",
        abi = "CheckpointRewarderUpdated(address)"
    )]
    pub struct CheckpointRewarderUpdatedFilter {
        #[ethevent(indexed)]
        pub new_checkpoint_rewarder: ::ethers::core::types::Address,
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
    #[ethevent(name = "FeeCollectorUpdated", abi = "FeeCollectorUpdated(address)")]
    pub struct FeeCollectorUpdatedFilter {
        #[ethevent(indexed)]
        pub new_fee_collector: ::ethers::core::types::Address,
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
    #[ethevent(name = "GovernanceUpdated", abi = "GovernanceUpdated(address)")]
    pub struct GovernanceUpdatedFilter {
        #[ethevent(indexed)]
        pub new_governance: ::ethers::core::types::Address,
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
    #[ethevent(name = "PauserUpdated", abi = "PauserUpdated(address,bool)")]
    pub struct PauserUpdatedFilter {
        #[ethevent(indexed)]
        pub new_pauser: ::ethers::core::types::Address,
        pub status: bool,
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
    #[ethevent(name = "SweepCollectorUpdated", abi = "SweepCollectorUpdated(address)")]
    pub struct SweepCollectorUpdatedFilter {
        #[ethevent(indexed)]
        pub new_sweep_collector: ::ethers::core::types::Address,
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
    pub enum ERC4626Target1Events {
        AddLiquidityFilter(AddLiquidityFilter),
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        CheckpointRewarderUpdatedFilter(CheckpointRewarderUpdatedFilter),
        CloseLongFilter(CloseLongFilter),
        CloseShortFilter(CloseShortFilter),
        CollectGovernanceFeeFilter(CollectGovernanceFeeFilter),
        CreateCheckpointFilter(CreateCheckpointFilter),
        FeeCollectorUpdatedFilter(FeeCollectorUpdatedFilter),
        GovernanceUpdatedFilter(GovernanceUpdatedFilter),
        InitializeFilter(InitializeFilter),
        OpenLongFilter(OpenLongFilter),
        OpenShortFilter(OpenShortFilter),
        PauseStatusUpdatedFilter(PauseStatusUpdatedFilter),
        PauserUpdatedFilter(PauserUpdatedFilter),
        RedeemWithdrawalSharesFilter(RedeemWithdrawalSharesFilter),
        RemoveLiquidityFilter(RemoveLiquidityFilter),
        SweepFilter(SweepFilter),
        SweepCollectorUpdatedFilter(SweepCollectorUpdatedFilter),
        TransferSingleFilter(TransferSingleFilter),
    }
    impl ::ethers::contract::EthLogDecode for ERC4626Target1Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddLiquidityFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::AddLiquidityFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = CheckpointRewarderUpdatedFilter::decode_log(log) {
                return Ok(
                    ERC4626Target1Events::CheckpointRewarderUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = CloseLongFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::CloseLongFilter(decoded));
            }
            if let Ok(decoded) = CloseShortFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::CloseShortFilter(decoded));
            }
            if let Ok(decoded) = CollectGovernanceFeeFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::CollectGovernanceFeeFilter(decoded));
            }
            if let Ok(decoded) = CreateCheckpointFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::CreateCheckpointFilter(decoded));
            }
            if let Ok(decoded) = FeeCollectorUpdatedFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::FeeCollectorUpdatedFilter(decoded));
            }
            if let Ok(decoded) = GovernanceUpdatedFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::GovernanceUpdatedFilter(decoded));
            }
            if let Ok(decoded) = InitializeFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::InitializeFilter(decoded));
            }
            if let Ok(decoded) = OpenLongFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::OpenLongFilter(decoded));
            }
            if let Ok(decoded) = OpenShortFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::OpenShortFilter(decoded));
            }
            if let Ok(decoded) = PauseStatusUpdatedFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::PauseStatusUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PauserUpdatedFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::PauserUpdatedFilter(decoded));
            }
            if let Ok(decoded) = RedeemWithdrawalSharesFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::RedeemWithdrawalSharesFilter(decoded));
            }
            if let Ok(decoded) = RemoveLiquidityFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::RemoveLiquidityFilter(decoded));
            }
            if let Ok(decoded) = SweepFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::SweepFilter(decoded));
            }
            if let Ok(decoded) = SweepCollectorUpdatedFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::SweepCollectorUpdatedFilter(decoded));
            }
            if let Ok(decoded) = TransferSingleFilter::decode_log(log) {
                return Ok(ERC4626Target1Events::TransferSingleFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ERC4626Target1Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddLiquidityFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckpointRewarderUpdatedFilter(element) => {
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
                Self::FeeCollectorUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GovernanceUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OpenLongFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OpenShortFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseStatusUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PauserUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RedeemWithdrawalSharesFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveLiquidityFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SweepFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepCollectorUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferSingleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddLiquidityFilter> for ERC4626Target1Events {
        fn from(value: AddLiquidityFilter) -> Self {
            Self::AddLiquidityFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalFilter> for ERC4626Target1Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for ERC4626Target1Events {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<CheckpointRewarderUpdatedFilter>
    for ERC4626Target1Events {
        fn from(value: CheckpointRewarderUpdatedFilter) -> Self {
            Self::CheckpointRewarderUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<CloseLongFilter> for ERC4626Target1Events {
        fn from(value: CloseLongFilter) -> Self {
            Self::CloseLongFilter(value)
        }
    }
    impl ::core::convert::From<CloseShortFilter> for ERC4626Target1Events {
        fn from(value: CloseShortFilter) -> Self {
            Self::CloseShortFilter(value)
        }
    }
    impl ::core::convert::From<CollectGovernanceFeeFilter> for ERC4626Target1Events {
        fn from(value: CollectGovernanceFeeFilter) -> Self {
            Self::CollectGovernanceFeeFilter(value)
        }
    }
    impl ::core::convert::From<CreateCheckpointFilter> for ERC4626Target1Events {
        fn from(value: CreateCheckpointFilter) -> Self {
            Self::CreateCheckpointFilter(value)
        }
    }
    impl ::core::convert::From<FeeCollectorUpdatedFilter> for ERC4626Target1Events {
        fn from(value: FeeCollectorUpdatedFilter) -> Self {
            Self::FeeCollectorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<GovernanceUpdatedFilter> for ERC4626Target1Events {
        fn from(value: GovernanceUpdatedFilter) -> Self {
            Self::GovernanceUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializeFilter> for ERC4626Target1Events {
        fn from(value: InitializeFilter) -> Self {
            Self::InitializeFilter(value)
        }
    }
    impl ::core::convert::From<OpenLongFilter> for ERC4626Target1Events {
        fn from(value: OpenLongFilter) -> Self {
            Self::OpenLongFilter(value)
        }
    }
    impl ::core::convert::From<OpenShortFilter> for ERC4626Target1Events {
        fn from(value: OpenShortFilter) -> Self {
            Self::OpenShortFilter(value)
        }
    }
    impl ::core::convert::From<PauseStatusUpdatedFilter> for ERC4626Target1Events {
        fn from(value: PauseStatusUpdatedFilter) -> Self {
            Self::PauseStatusUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PauserUpdatedFilter> for ERC4626Target1Events {
        fn from(value: PauserUpdatedFilter) -> Self {
            Self::PauserUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<RedeemWithdrawalSharesFilter> for ERC4626Target1Events {
        fn from(value: RedeemWithdrawalSharesFilter) -> Self {
            Self::RedeemWithdrawalSharesFilter(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityFilter> for ERC4626Target1Events {
        fn from(value: RemoveLiquidityFilter) -> Self {
            Self::RemoveLiquidityFilter(value)
        }
    }
    impl ::core::convert::From<SweepFilter> for ERC4626Target1Events {
        fn from(value: SweepFilter) -> Self {
            Self::SweepFilter(value)
        }
    }
    impl ::core::convert::From<SweepCollectorUpdatedFilter> for ERC4626Target1Events {
        fn from(value: SweepCollectorUpdatedFilter) -> Self {
            Self::SweepCollectorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TransferSingleFilter> for ERC4626Target1Events {
        fn from(value: TransferSingleFilter) -> Self {
            Self::TransferSingleFilter(value)
        }
    }
    ///Container type for all input parameters for the `closeLong` function with signature `closeLong(uint256,uint256,uint256,(address,bool,bytes))` and selector `0xded06231`
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
        name = "closeLong",
        abi = "closeLong(uint256,uint256,uint256,(address,bool,bytes))"
    )]
    pub struct CloseLongCall {
        pub maturity_time: ::ethers::core::types::U256,
        pub bond_amount: ::ethers::core::types::U256,
        pub min_output: ::ethers::core::types::U256,
        pub options: Options,
    }
    ///Container type for all input parameters for the `closeShort` function with signature `closeShort(uint256,uint256,uint256,(address,bool,bytes))` and selector `0x29b23fc1`
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
        name = "closeShort",
        abi = "closeShort(uint256,uint256,uint256,(address,bool,bytes))"
    )]
    pub struct CloseShortCall {
        pub maturity_time: ::ethers::core::types::U256,
        pub bond_amount: ::ethers::core::types::U256,
        pub min_output: ::ethers::core::types::U256,
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
    pub enum ERC4626Target1Calls {
        CloseLong(CloseLongCall),
        CloseShort(CloseShortCall),
    }
    impl ::ethers::core::abi::AbiDecode for ERC4626Target1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CloseLongCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CloseLong(decoded));
            }
            if let Ok(decoded) = <CloseShortCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CloseShort(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC4626Target1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CloseLong(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CloseShort(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ERC4626Target1Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CloseLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::CloseShort(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CloseLongCall> for ERC4626Target1Calls {
        fn from(value: CloseLongCall) -> Self {
            Self::CloseLong(value)
        }
    }
    impl ::core::convert::From<CloseShortCall> for ERC4626Target1Calls {
        fn from(value: CloseShortCall) -> Self {
            Self::CloseShort(value)
        }
    }
    ///Container type for all return fields from the `closeLong` function with signature `closeLong(uint256,uint256,uint256,(address,bool,bytes))` and selector `0xded06231`
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
    pub struct CloseLongReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `closeShort` function with signature `closeShort(uint256,uint256,uint256,(address,bool,bytes))` and selector `0x29b23fc1`
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
    pub struct CloseShortReturn(pub ::ethers::core::types::U256);
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

pub struct ERC4626Target1Libs {
    pub lp_math: ::ethers::types::Address,
}

impl<M: ::ethers::providers::Middleware> ERC4626Target1<M> {
    pub fn link_and_deploy<T: ::ethers::core::abi::Tokenize>(
        client: ::std::sync::Arc<M>,
        constructor_args: T,
        libs: ERC4626Target1Libs,
    ) -> ::core::result::Result<
        ::ethers::contract::builders::ContractDeployer<M, Self>,
        ::ethers::contract::ContractError<M>,
    > {
        let factory = crate::linked_factory::create(
            ERC4626TARGET1_ABI.clone(),
            "0x6102606040523480156200001257600080fd5b506040516200404638038062004046833981016040819052620000359162000202565b600160005580516001600160a01b039081166080908152602080840151831660a0908152918401516101a0908152918401516101c090815260c0808601516101e090815260e0808801516102005261012080890151909352610100808901519091526101408089015190915290870180515190925281519093015190925281516040908101516101609081529251606090810151610180908152918701518616610220528601516102405291850151600980546001600160a01b031990811692871692909217905591850151600a8054841691861691909117905591840151600b80548316918516919091179055920151600c8054909316911617905562000315565b60405161020081016001600160401b03811182821017156200016a57634e487b7160e01b600052604160045260246000fd5b60405290565b80516001600160a01b03811681146200018857600080fd5b919050565b600060808284031215620001a057600080fd5b604051608081016001600160401b0381118282101715620001d157634e487b7160e01b600052604160045260246000fd5b8060405250809150825181526020830151602082015260408301516040820152606083015160608201525092915050565b600061026082840312156200021657600080fd5b6200022062000138565b6200022b8362000170565b81526200023b6020840162000170565b60208201526200024e6040840162000170565b6040820152606083015160608201526080830151608082015260a083015160a082015260c083015160c082015260e083015160e0820152610100808401518183015250610120808401518183015250610140808401518183015250610160620002b981850162000170565b90820152610180620002cd84820162000170565b908201526101a0620002e184820162000170565b908201526101c0620002f584820162000170565b908201526101e06200030a858583016200018d565b908201529392505050565b60805160a05160c05160e05161010051610120516101405161016051610180516101a0516101c0516101e051610200516102205161024051613b88620004be6000396000505060005050600050506000818160af015281816103250152612c1301526000818161136401528181611776015281816117d9015281816127f10152818161282d01528181612a2c0152612bed01526000818161071301528181610d6101528181610da801528181610f6d015281816115db015281816116290152612bc701526000611aca015260008181611bf00152818161225301526122cc0152600081816108d701528181610f0501528181611bc401526122a0015260008181610dfa015261221901526000818161073401528181610d3f01528181610dc901528181610f8e015281816115b90152818161164a0152612c3901526000818161080f01528181610e84015281816116d2015281816120ee01526131f40152600081816105d20152818161064d015281816106c30152818161078f01526107c7015260008181611957015281816125220152818161257601526125f2015260005050613b886000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c806329b23fc11461003b578063ded0623114610060575b600080fd5b61004e610049366004613602565b610073565b60405190815260200160405180910390f35b61004e61006e366004613602565b61008c565b60006100818585858561009a565b90505b949350505050565b600061008185858585610310565b60006100a461054f565b6100ad82610579565b7f00000000000000000000000000000000000000000000000000000000000000008410156100ee5760405163211ddda360e11b815260040160405180910390fd5b60006100f86105b2565b90508542101561011d5761011761010d6105ca565b82600460016105f6565b5061012d565b61012b8682600460016105f6565b505b61014261013b600288610bec565b3387610c21565b6000806000806000806101568b888e610cfa565b95509550955095509550955060008c90508c4210156102165782600d60008282546101819190613677565b9091555061019990506101926105ca565b4284610fee565b6101a68c8887878561112c565b60006101b18e6112a6565b90506101cf6101bf8e6112f0565b6101c9908361368a565b8261131e565b6101d88961135d565b6101e4576101e46113bd565b60006101ef8a6113d6565b90508061020f57604051638bdf918d60e01b815260040160405180910390fd5b505061022d565b61022086896113e3565b955061022b886113d6565b505b600061023a878a8d6114c4565b90508b81101561025d5760405163c972651760e01b815260040160405180910390fd5b8c86858b8e61026d600288610bec565b61027a60208301836136aa565b6001600160a01b0316337ff87a3de08b9fe89d655d6731088496cf5f5da0abd455e9f7cdc5f0c717f209e58a8a876102b86040890160208a016136e1565b6102cc8a6102c68d8f613677565b9061155f565b8d6102da60408c018c6136fe565b6040516102ee98979695949392919061376e565b60405180910390a450939c505050505050505050505050506100846001600055565b600061031a61054f565b61032382610579565b7f00000000000000000000000000000000000000000000000000000000000000008410156103645760405163211ddda360e11b815260040160405180910390fd5b600061036e6105b2565b9050854210156103895761038361010d6105ca565b50610399565b6103978682600460016105f6565b505b6103a761013b600188610bec565b6000806000806000806103bb8b888e611574565b95509550955095509550955060008c90508c42101561046e5782600d60008282546103e69190613677565b909155506103f790506101926105ca565b6104048c8887878561175f565b600061040f826112a6565b905061042761041d8e6112f0565b6101c990836137b6565b6104308961135d565b61043c5761043c6113bd565b60006104478a6113d6565b90508061046757604051638bdf918d60e01b815260040160405180910390fd5b5050610485565b61047886896113e3565b9550610483886113d6565b505b6000610492878a8d6114c4565b90508b8110156104b55760405163c972651760e01b815260040160405180910390fd5b8c898c6104c3600186610bec565b6104d060208301836136aa565b6001600160a01b0316337f3b2c44173852b22d1ecf7784963c2bab6d4dd07e64ed560f818f144d72ee526788888761050e6040890160208a016136e1565b8a61051c60408b018b6136fe565b60405161052f97969594939291906137de565b60405180910390a450919a50505050505050505050506100846001600055565b60026000540361057257604051633ee5aeb560e01b815260040160405180910390fd5b6002600055565b600061058860208301836136aa565b6001600160a01b0316036105af5760405163f0dd15fd60e01b815260040160405180910390fd5b50565b60006105c5670de0b6b3a7640000611950565b905090565b60006105c5427f000000000000000000000000000000000000000000000000000000000000000061197c565b600084815260086020526040812060018101546001600160801b031615158061061e57504286115b1561063757600101546001600160801b03169050610084565b60008060006106446105ca565b905060006106727f00000000000000000000000000000000000000000000000000000000000000008b613677565b90505b818110156106e8576000818152600860205260409020600101546001600160801b031680156106c0576000828152600860205260409020549094506001600160801b031692506106e8565b507f000000000000000000000000000000000000000000000000000000000000000001610675565b8360000361075b578893506107586106fe611992565b600254600160801b90046001600160801b03167f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000006119b3565b92505b50610765836119ca565b6001850180546001600160801b0319166001600160801b03929092169190911790556107bb6107b47f00000000000000000000000000000000000000000000000000000000000000008b61381f565b8a84610fee565b6107f9896107f36107ec7f000000000000000000000000000000000000000000000000000000000000000083613677565b42906119f4565b84610fee565b61080288611a09565b50600090506008816108347f00000000000000000000000000000000000000000000000000000000000000008d61381f565b815260208101919091526040016000908120600101546001600160801b0316915061086060028c610bec565b6000818152601060205260408120549192508c8c83156109a6576001925060008061088f86898d866000611bad565b9150915080600d60008282546108a59190613677565b909155506108c19050866000846108bb816112f0565b8861112c565b6108cb8183613677565b91506108fb86838a8e877f0000000000000000000000000000000000000000000000000000000000000000611c7c565b915061090f61090a838561155f565b611cc6565b600680546002906109309084906201000090046001600160701b0316613832565b92506101000a8154816001600160701b0302191690836001600160701b0316021790555061095d826119ca565b6006805460109061097f908490600160801b90046001600160801b0316613852565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555050505b60006109b3600184610bec565b6000818152601060205260409020549091508015610ac257600194506000806109e0838b8f886001611bad565b9150915080600d60008282546109f69190613677565b90915550610a12905083600084610a0c816112f0565b8a61175f565b610a1c818361381f565b9150610a2b61090a838761155f565b60068054600290610a4c9084906201000090046001600160701b0316613832565b92506101000a8154816001600160701b0302191690836001600160701b03160217905550610a79826119ca565b60068054601090610a9b908490600160801b90046001600160801b0316613852565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555050505b8415610afc57610aee610ad4876112f0565b610add836112f0565b610ae7919061368a565b600061131e565b8e610af98482611cf0565b50505b6000610b0784611eea565b50604080518e815260208101879052908101899052606081018490526080810182905290915085907fff888cf98d2696e95c8c39aa98c9ad55a5378008f7a56614c9353b7137a57ab79060a00160405180910390a2600c546001600160a01b031615610bd5578e610bd15a604051336024820152604481018990528315156064820152600090819060840160408051601f198184030181529190526020810180516001600160e01b0316633488a6a760e11b179052600c546001600160a01b031693929190612036565b5050505b50999b505050505050505050505050949350505050565b60006001600160f81b03821115610c165760405163b7d0949760e01b815260040160405180910390fd5b5060f89190911b1790565b6000838152600f602090815260408083206001600160a01b0386168452909152902054811115610c6457604051631e9acf1760e31b815260040160405180910390fd5b6000838152600f602090815260408083206001600160a01b0386168452825280832080548590039055858352601090915281208054839290610ca790849061381f565b909155505060408051848152602081018390526000916001600160a01b0385169133917fc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f62910160405180910390a4505050565b600080600080600080600080610d0e611992565b90506000610d1b8a6120c1565b6002549091508c908c90610d85908590600160801b90046001600160801b031684867f0000000000000000000000000000000000000000000000000000000000000000867f0000000000000000000000000000000000000000000000000000000000000000612112565b600254919d509a50909550610ded908590600160801b90046001600160801b03167f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000006119b3565b9550610e23858c610e1e897f0000000000000000000000000000000000000000000000000000000000000000612182565b6121b2565b15610e3057610e306113bd565b6000806000610e4185878b876121f4565b9c5091945092509050610e54818461381f565b610e5e9089613677565b9750610e6a8284613677565b610e74908d613677565b9b505050505050506000600860007f00000000000000000000000000000000000000000000000000000000000000008c610eae919061381f565b815260208101919091526040016000908120600101546001600160801b03169150428b11610ef65760008b8152600860205260409020600101546001600160801b0316610ef8565b8b5b90508b610f298e8a8585857f0000000000000000000000000000000000000000000000000000000000000000611c7c565b9950610f35878a61381f565b9850670de0b6b3a7640000610fb2610f4d8787613677565b600254610f6b908f90600160801b90046001600160801b031661381f565b7f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000006119b3565b1115610fc057610fc06113bd565b610fd08a8a878a87876000612308565b939d50919b5090995090975050505050505093975093979195509350565b600083815260086020526040902054600160801b90046001600160801b031680830361101a5750505050565b6000848152600860205260408120546001600160801b03169081900361107457611043836119ca565b600086815260086020526040902080546001600160801b0319166001600160801b03929092169190911790556110f2565b6110c56110c0670de0b6b3a764000061108d888661381f565b6110979190613872565b85670de0b6b3a76400006110ab878a61381f565b6110b59190613872565b8592919060016123a7565b6119ca565b600086815260086020526040902080546001600160801b0319166001600160801b03929092169190911790555b6110fb846119ca565b60009586526008602052604090952080546001600160801b03968716600160801b0296169590951790945550505050565b600454600160801b90046001600160801b03166111756110c082611158670de0b6b3a764000086613872565b600554600160801b90046001600160801b031691908a60006123a7565b600580546001600160801b03928316600160801b02921691909117905561119b866119ca565b6111a59082613889565b600480546001600160801b03928316600160801b0292169190911790556111cb846119ca565b600280546000906111e69084906001600160801b0316613852565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555061121383612451565b60048054600090611228908490600f0b6138a9565b92506101000a8154816001600160801b030219169083600f0b6001600160801b03160217905550611258856119ca565b6002805460109061127a908490600160801b90046001600160801b0316613889565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550505050505050565b60006112ce601060006112ba600286610bec565b8152602001908152602001600020546112f0565b6112e0601060006112ba600187610bec565b6112ea919061368a565b92915050565b60006001600160ff1b0382111561131a5760405163396ea70160e11b815260040160405180910390fd5b5090565b600354611339906110c0906001600160801b0316848461248d565b600380546001600160801b0319166001600160801b03929092169190911790555050565b60006113897f0000000000000000000000000000000000000000000000000000000000000000836124e8565b60035461139f91906001600160801b0316613677565b6002546113b5906001600160801b03168461155f565b101592915050565b60405163bb55fd2760e01b815260040160405180910390fd5b60006112ea826004611cf0565b60008060006113f184611a09565b90925090506000611402868661155f565b90508183111561141a576114178683856124fd565b95505b8281101561142c578083039250611431565b600092505b61143a83611cc6565b600680546001600160701b039290921662010000026fffffffffffffffffffffffffffff00001990921691909117908190556001600160801b03600160801b909104168087101561148d57869003611491565b5060005b61149a816119ca565b600680546001600160801b03928316600160801b0292169190911790555085935050505092915050565b6000806114d1858561155f565b90506114dc8161251b565b9450846000036114f0576000915050611558565b61150060408401602085016136e1565b1561152f576115288561151660208601866136aa565b61152360408701876136fe565b612547565b9150611556565b6115528561154060208601866136aa565b61154d60408701876136fe565b6125e5565b8491505b505b9392505050565b60006115588383670de0b6b3a76400006124fd565b600080600080600080600080611588611992565b905060006115958a6120c1565b6002549091508b908d906115ff908590600160801b90046001600160801b031683867f0000000000000000000000000000000000000000000000000000000000000000877f000000000000000000000000000000000000000000000000000000000000000061261f565b600254919d509b509095506000908190819061166e908890600160801b90046001600160801b03167f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000006119b3565b985061167c84878b886121f4565b9c509194509092509050611690828461381f565b61169a908961381f565b97506116a68184613677565b6116b0908e61381f565b9c506116bc8a8e613677565b9b505050505050505061174986868386600860007f00000000000000000000000000000000000000000000000000000000000000008f6116fc919061381f565b81526020810191909152604001600020600101546001600160801b0316428e116117405760008e8152600860205260409020600101546001600160801b0316611742565b8e5b6001612308565b9a9e939d50919b50909950929650945050505050565b6002546001600160801b0316838110806117a157507f000000000000000000000000000000000000000000000000000000000000000061179f858361381f565b105b156117ae576117ae6113bd565b6004549084900390600f0b6117c3848261368a565b9050836117cf866112f0565b13801561180457507f00000000000000000000000000000000000000000000000000000000000000006118028383612673565b105b15611811576118116113bd565b600354600160801b90046001600160801b03166118536110c08261183d670de0b6b3a764000088613872565b6005546001600160801b031691908c60006123a7565b600580546001600160801b0319166001600160801b039290921691909117905561187d888261381f565b9050611888816119ca565b600380546001600160801b03928316600160801b0292169190911790556118ae836119ca565b600280546001600160801b0319166001600160801b03929092169190911790556118d782612451565b600480546001600160801b0319166001600160801b0392909216919091179055611900876119ca565b60028054601090611922908490600160801b90046001600160801b0316613852565b92506101000a8154816001600160801b0302191690836001600160801b031602179055505050505050505050565b60006112ea7f000000000000000000000000000000000000000000000000000000000000000083612699565b600061198882846138ec565b611558908461381f565b6002546004546000916105c5916001600160801b0390911690600f0b612673565b6000610081826119c48588886124fd565b90612706565b60006001600160801b0382111561131a57604051630f0af95160e11b815260040160405180910390fd5b6000818311611a035782611558565b50919050565b6006546000908190611a2c908490600160801b90046001600160801b031661155f565b6006546201000090046001600160701b03169250905081811115611ba8576000611a56838361381f565b9050611a656110c0828661277b565b60068054601090611a87908490600160801b90046001600160801b0316613889565b92506101000a8154816001600160801b0302191690836001600160801b031602179055506000611ac0858361279090919063ffffffff16565b90506000611aee827f000000000000000000000000000000000000000000000000000000000000000061155f565b905080600d6000828254611b029190613677565b90915550611b129050818361381f565b9150611b1d826119ca565b60028054600090611b389084906001600160801b0316613852565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550611b65826127a5565b60048054600090611b7a908490600f0b6138a9565b92506101000a8154816001600160801b030219169083600f0b6001600160801b031602179055508493505050505b915091565b600080611bba8785612790565b91506000611be8837f000000000000000000000000000000000000000000000000000000000000000061155f565b9050611c14817f000000000000000000000000000000000000000000000000000000000000000061155f565b91508315611c3757611c26828261381f565b611c30908461381f565b9250611c4e565b611c41828261381f565b611c4b9084613677565b92505b86861015611c7157611c618387896124fd565b9250611c6e8287896124fd565b91505b509550959350505050565b600080611c9484611c8e8a888a6124fd565b90612790565b9050611ca18884866124fd565b611cab9082613677565b905086811115611cbb5786810391505b509695505050505050565b60006001600160701b0382111561131a5760405163086b151760e11b815260040160405180910390fd5b600754600360f81b600090815260106020527fd6f7110f7a6485ce27c724322bdc9b60c9b2518194eace178d55653f1be730215490918291611d3b916001600160801b03169061381f565b905080600003611d4f5760019150506112ea565b6000611d5a856127cf565b905080600003611d6f576001925050506112ea565b600080611d7d83858961286b565b9150915080611d935760009450505050506112ea565b60008073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__632c03ef68858a6040518363ffffffff1660e01b8152600401611dcf92919061397e565b6040805180830381865af4158015611deb573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611e0f91906139f4565b91509150611e2d611e1f826112f0565b611e2890613a18565b6129e6565b925082611e4357600096505050505050506112ea565b611e4c826119ca565b60078054600090611e679084906001600160801b0316613852565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550611e94816119ca565b60078054601090611eb6908490600160801b90046001600160801b0316613852565b92506101000a8154816001600160801b0302191690836001600160801b031602179055506001965050505050505092915050565b60008060008073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__635a1b419e611f1387612b7e565b6040518263ffffffff1660e01b8152600401611f2f9190613a34565b6040805180830381865af4158015611f4b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611f6f9190613a43565b9150915080611f85575060009485945092505050565b60075460106020527fd6f7110f7a6485ce27c724322bdc9b60c9b2518194eace178d55653f1be730215460008080527f6e0956cda88cad152e89927e53611735b61a5c762d1428573c6931b0a5efcb015490926001600160801b031691611feb91613677565b611ff5919061381f565b90508060000361200d57506000958695509350505050565b600080871161201d576000612028565b6120288488846124fd565b976001975095505050505050565b6000606060008060008661ffff1667ffffffffffffffff81111561205c5761205c613a73565b6040519080825280601f01601f191660200182016040528015612086576020820181803683370190505b5090506000808751602089018b8e8ef191503d9250868311156120a7578692505b828152826000602083013e90999098509650505050505050565b6000806120cc6105ca565b90508083116120dc5760006120e6565b6120e6818461381f565b9150611558827f0000000000000000000000000000000000000000000000000000000000000000612790565b6000808061213361212b88670de0b6b3a764000061381f565b899087612cb8565b905086156121755761214588886124e8565b91506121668a8a8461215f8a670de0b6b3a764000061381f565b8989612cde565b92506121728382613677565b90505b9750975097945050505050565b60006121a061219984670de0b6b3a764000061381f565b83906124e8565b61155890670de0b6b3a764000061381f565b6000806121e9856121c1611992565b6121cb9190613677565b600254610f6b908790600160801b90046001600160801b031661381f565b909210949350505050565b600080808061224b87866122448b61223e6122178c670de0b6b3a764000061381f565b7f0000000000000000000000000000000000000000000000000000000000000000906124e8565b906124e8565b9190612cb8565b9350612277847f000000000000000000000000000000000000000000000000000000000000000061155f565b9150600061229861229089670de0b6b3a764000061381f565b8a9088612cb8565b90506122c4817f00000000000000000000000000000000000000000000000000000000000000006124e8565b93506122f0847f000000000000000000000000000000000000000000000000000000000000000061155f565b6122fa9084613677565b915050945094509450949050565b6000806000806000808888101561237757861561232d5761232a8d898b6124fd565b9c505b6123388c898b6124fd565b9b506123438b6112f0565b61234c8d6112f0565b612356919061368a565b90506123638b898b6124fd565b9a506123708a898b6124fd565b9950612396565b6123808b6112f0565b6123898d6112f0565b612393919061368a565b90505b9b9c9a9b999a975050505050505050565b6000826000036123b8575084612448565b811561240b576123e96123cb8487613677565b6123d5858761155f565b6123df888a61155f565b611c8e9190613677565b905060006123f785886119f4565b905080821015612405578091505b50612448565b82850361241a57506000612448565b612445612427848761381f565b61243185876124e8565b61243b888a61155f565b611c8e919061381f565b90505b95945050505050565b600060016001607f1b031982128061246f575060016001607f1b0382135b1561131a5760405163a5353be560e01b815260040160405180910390fd5b60008061249b846000612d01565b6124a6846000612d01565b6124b0919061368a565b905060008113156124cc576124c58186613677565b9450611552565b6000811215611552576124de81613a18565b612448908661381f565b60006115588383670de0b6b3a7640000612cb8565b600082600019048411830215820261251457600080fd5b5091020490565b60006112ea7f000000000000000000000000000000000000000000000000000000000000000083612d17565b604051635d043b2960e11b8152600481018590526001600160a01b0384811660248301523060448301526000917f00000000000000000000000000000000000000000000000000000000000000009091169063ba087652906064016020604051808303816000875af11580156125c1573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100819190613a89565b6126196001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000168486612d47565b50505050565b6000808061264061263888670de0b6b3a764000061381f565b8990876124fd565b9050861561217557612652888861155f565b91506121668a8a8461266c8a670de0b6b3a764000061381f565b8989612d9e565b6000806126808484612daf565b909250905080612692576126926113bd565b5092915050565b6040516303d1689d60e11b8152600481018290526000906001600160a01b038416906307a2d13a906024015b602060405180830381865afa1580156126e2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115589190613a89565b60008160000361271f5750670de0b6b3a76400006112ea565b8260000361272f575060006112ea565b600061273a836112f0565b9050600061274f61274a866112f0565b612dee565b9050818102612766670de0b6b3a764000082613aa2565b905061277181613014565b9695505050505050565b600061155883670de0b6b3a764000084612cb8565b600061155883670de0b6b3a7640000846124fd565b600060016001607f1b0382111561131a5760405163a5353be560e01b815260040160405180910390fd5b60035460009081906127ea906001600160801b03168461277b565b90506128167f000000000000000000000000000000000000000000000000000000000000000082613677565b6002546001600160801b03161115611a03576002547f0000000000000000000000000000000000000000000000000000000000000000906128619083906001600160801b031661381f565b611558919061381f565b61287361354f565b60008061287f84612b7e565b9050600073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__635a1b419e836040518263ffffffff1660e01b81526004016128ba9190613a34565b6040805180830381865af41580156128d6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906128fa9190613a43565b935090508261290f5750600091506129de9050565b600061293661293184610160015185610140015161155f90919063ffffffff16565b6112f0565b6129566129318561012001518661010001516124e890919063ffffffff16565b612960919061368a565b604080516101208101825285815260208082019590955260008052601085527f6e0956cda88cad152e89927e53611735b61a5c762d1428573c6931b0a5efcb015481830152606081018a9052608081018b905260a0810192909252845160c08301529284015160e08201529290910151610100830152509150600190505b935093915050565b6002546004805460405163685a2be760e11b81526001600160801b03808516938201849052600f9290920b60248201819052600160801b909404909116604482018190527f000000000000000000000000000000000000000000000000000000000000000060648301526084820185905260009391849081908190819073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__9063d0b457ce9060a401608060405180830381865af4158015612a9f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612ac39190613ad0565b935093509350935080612adf5750600098975050505050505050565b868414612b1057612aef846119ca565b600280546001600160801b0319166001600160801b03929092169190911790555b858314612b4157612b2083612451565b600480546001600160801b0319166001600160801b03929092169190911790555b848214612b6f57612b51826119ca565b600280546001600160801b03928316600160801b0292169190911790555b50600198975050505050505050565b612b866135a1565b60408051610180810182526002546001600160801b038082168352600454600f0b6020840152600160801b91829004811693830193909352606082018590527f000000000000000000000000000000000000000000000000000000000000000060808301527f000000000000000000000000000000000000000000000000000000000000000060a08301527f000000000000000000000000000000000000000000000000000000000000000060c08301527f000000000000000000000000000000000000000000000000000000000000000060e08301526003540482166101008201526005549091610120830191612c7e91166131a9565b81526004546001600160801b03600160801b9182900481166020840152600554604090930192612cb0929004166131a9565b905292915050565b6000826000190484118302158202612ccf57600080fd5b50910281810615159190040190565b600080612cef88888888888861321f565b909250905080611cbb57611cbb6113bd565b6000818313612d105781611558565b5090919050565b6040516363737ac960e11b8152600481018290526000906001600160a01b0384169063c6e6f592906024016126c5565b604080516001600160a01b038416602482015260448082018490528251808303909101815260649091019091526020810180516001600160e01b031663a9059cbb60e01b179052612d99908490613300565b505050565b600080612cef888888888888613368565b600080600083612dbe866112f0565b612dc8919061368a565b90506000811215612de0576000809250925050612de7565b9150600190505b9250929050565b6000808213612e105760405163e61b497560e01b815260040160405180910390fd5b506001600160801b03811160071b81811c67ffffffffffffffff1060061b1781811c63ffffffff1060051b1781811c61ffff1060041b1781811c60ff10600390811b90911782811c600f1060021b1782811c909110600190811b90911782811c90911017609f8181036060019290921b91605f19820190612e939084901c6112f0565b6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d6c8c3f38e95a6b1ff2ab1c3b343619018302821d6d02384773bdf1ac5676facced60901901830290911d6cb9a025d814b29c212b8b1a07cd1901909102780a09507084cc699bb0e71ea869ffffffffffffffffffffffff190105711340daa0d5f769dba1915cef59f0815a5506027d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b391909102017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d92915050565b6000680248ce36a70cb26b3e19821361302f57506000919050565b680755bf798b4a1bf1e58212613058576040516373a2d6b160e01b815260040160405180910390fd5b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056001605f1b01901d6bb17217f7d1cf79abc9e3b3988102909303926c240c330e9fb2d9cbaf0fd5aafb1984018402606090811d6d0277594991cfc85f6e2461837cd9018502811d6d1a521255e34f6a5061b25ef1c9c319018502811d6db1bbb201f443cf962f1a1d3db4a5018502811d6e02c72388d9f74f51a9331fed693f1419018502811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765087018702831d9081019087016d01d3967ed30fc4f89c02bab570811901810290921d6e0587f503bb6ea29d25fcb7401964500186026d360d7aeea093263ecc6e0ecb291760621b01818105955092935090919061277174029d9dc38563c32e5c2f6dc192ee70ef65f9978af3860260c38690031c6112f0565b600080670de0b6b3a76400006131bd6105ca565b6131c79190613872565b90508083116131d75760006131e1565b6131e1818461381f565b9150611558613218670de0b6b3a76400007f0000000000000000000000000000000000000000000000000000000000000000613872565b8390612790565b60008060006132318989888888613428565b9050868810156132485760008092509250506132f5565b96869003966132578887612706565b97508781101561326e5760008092509250506132f5565b87810361327c818688612cb8565b9050670de0b6b3a764000081106132b0576132a96132a2670de0b6b3a76400008961277b565b8290612706565b90506132c8565b6132c56132a2670de0b6b3a764000089612790565b90505b6132d2818661277b565b9050898110156132ea576000809350935050506132f5565b899003925060019150505b965096945050505050565b60006133156001600160a01b03841683613457565b9050805160001415801561333a5750808060200190518101906133389190613b06565b155b15612d9957604051635274afe760e01b81526001600160a01b03841660048201526024015b60405180910390fd5b600080600061337a8989888888613428565b905061338a866119c4898b613677565b9750878110156133a15760008092509250506132f5565b8781036133af818688612cb8565b9050670de0b6b3a764000081106133dc576133d56132a2670de0b6b3a76400008961277b565b90506133f4565b6133f16132a2670de0b6b3a764000089612790565b90505b6133fe818661277b565b9050808a1015613416576000809350935050506132f5565b90980398600198509650505050505050565b60006134348585612706565b61344d613445866119c4868b6124e8565b859085612cb8565b6124459190613677565b60606115588383600084600080856001600160a01b0316848660405161347d9190613b23565b60006040518083038185875af1925050503d80600081146134ba576040519150601f19603f3d011682016040523d82523d6000602084013e6134bf565b606091505b50915091506127718683836060826134df576134da82613526565b611558565b81511580156134f657506001600160a01b0384163b155b1561351f57604051639996b31560e01b81526001600160a01b038516600482015260240161335f565b5080611558565b8051156135365780518082602001fd5b604051630a12f52160e11b815260040160405180910390fd5b6040518061012001604052806135636135a1565b815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b6040518061018001604052806000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b6000806000806080858703121561361857600080fd5b843593506020850135925060408501359150606085013567ffffffffffffffff81111561364457600080fd5b85016060818803121561365657600080fd5b939692955090935050565b634e487b7160e01b600052601160045260246000fd5b808201808211156112ea576112ea613661565b818103600083128015838313168383128216171561269257612692613661565b6000602082840312156136bc57600080fd5b81356001600160a01b038116811461155857600080fd5b80151581146105af57600080fd5b6000602082840312156136f357600080fd5b8135611558816136d3565b6000808335601e1984360301811261371557600080fd5b83018035915067ffffffffffffffff82111561373057600080fd5b602001915036819003821315612de757600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b88815287602082015286604082015285151560608201528460808201528360a082015260e060c082015260006137a860e083018486613745565b9a9950505050505050505050565b80820182811260008312801582168215821617156137d6576137d6613661565b505092915050565b878152866020820152856040820152841515606082015283608082015260c060a0820152600061381260c083018486613745565b9998505050505050505050565b818103818111156112ea576112ea613661565b6001600160701b0381811683821601908082111561269257612692613661565b6001600160801b0381811683821601908082111561269257612692613661565b80820281158282048414176112ea576112ea613661565b6001600160801b0382811682821603908082111561269257612692613661565b600f81810b9083900b0160016001607f1b03811360016001607f1b0319821217156112ea576112ea613661565b634e487b7160e01b600052601260045260246000fd5b6000826138fb576138fb6138d6565b500690565b805182526020810151602083015260408101516040830152606081015160608301526080810151608083015260a081015160a083015260c081015160c083015260e081015160e08301526101008082015181840152506101208082015181840152506101408082015181840152506101608082015181840152505050565b60006102a082019050613992828551613900565b602084015161018083015260408401516101a083015260608401516101c083015260808401516101e083015260a084015161020083015260c084015161022083015260e084015161024083015261010090930151610260820152610280015290565b60008060408385031215613a0757600080fd5b505080516020909101519092909150565b6000600160ff1b8201613a2d57613a2d613661565b5060000390565b61018081016112ea8284613900565b60008060408385031215613a5657600080fd5b825191506020830151613a68816136d3565b809150509250929050565b634e487b7160e01b600052604160045260246000fd5b600060208284031215613a9b57600080fd5b5051919050565b600082613ab157613ab16138d6565b600160ff1b821460001984141615613acb57613acb613661565b500590565b60008060008060808587031215613ae657600080fd5b8451935060208501519250604085015191506060850151613656816136d3565b600060208284031215613b1857600080fd5b8151611558816136d3565b6000825160005b81811015613b445760208186018101518583015201613b2a565b50600092019182525091905056fea2646970667358221220cf9784601e605ce90c8002aeb55a113f468a3d5ab465d7f1f2d31626ed61c99064736f6c63430008140033",
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

