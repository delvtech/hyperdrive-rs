pub use erc4626_target_3::*;
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
pub mod erc4626_target_3 {
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
                    ::std::borrow::ToOwned::to_owned("addLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_contribution"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minLpSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minApr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxApr"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_contribution"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_apr"),
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
                    ::std::borrow::ToOwned::to_owned("BelowMinimumContribution"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BelowMinimumContribution",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CircuitBreakerTriggered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CircuitBreakerTriggered",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "DecreasedPresentValueWhenAddingLiquidity",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DecreasedPresentValueWhenAddingLiquidity",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("InvalidApr"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidApr"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidEffectiveShareReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidEffectiveShareReserves",
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
                    ::std::borrow::ToOwned::to_owned("PoolAlreadyInitialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PoolAlreadyInitialized",
                            ),
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
                (
                    ::std::borrow::ToOwned::to_owned("UpdateLiquidityFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UpdateLiquidityFailed",
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
    pub static ERC4626TARGET3_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct ERC4626Target3<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC4626Target3<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC4626Target3<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC4626Target3<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC4626Target3<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ERC4626Target3))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC4626Target3<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC4626TARGET3_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `addLiquidity` (0x4c2ac1d9) function
        pub fn add_liquidity(
            &self,
            contribution: ::ethers::core::types::U256,
            min_lp_share_price: ::ethers::core::types::U256,
            min_apr: ::ethers::core::types::U256,
            max_apr: ::ethers::core::types::U256,
            options: Options,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [76, 42, 193, 217],
                    (contribution, min_lp_share_price, min_apr, max_apr, options),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x77d05ff4) function
        pub fn initialize(
            &self,
            contribution: ::ethers::core::types::U256,
            apr: ::ethers::core::types::U256,
            options: Options,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([119, 208, 95, 244], (contribution, apr, options))
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
            ERC4626Target3Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC4626Target3<M> {
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
    ///Custom Error type `BelowMinimumContribution` with signature `BelowMinimumContribution()` and selector `0xabed41c4`
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
    #[etherror(name = "BelowMinimumContribution", abi = "BelowMinimumContribution()")]
    pub struct BelowMinimumContribution;
    ///Custom Error type `CircuitBreakerTriggered` with signature `CircuitBreakerTriggered()` and selector `0x2a958098`
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
    #[etherror(name = "CircuitBreakerTriggered", abi = "CircuitBreakerTriggered()")]
    pub struct CircuitBreakerTriggered;
    ///Custom Error type `DecreasedPresentValueWhenAddingLiquidity` with signature `DecreasedPresentValueWhenAddingLiquidity()` and selector `0x309b2a42`
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
        name = "DecreasedPresentValueWhenAddingLiquidity",
        abi = "DecreasedPresentValueWhenAddingLiquidity()"
    )]
    pub struct DecreasedPresentValueWhenAddingLiquidity;
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
    ///Custom Error type `InvalidApr` with signature `InvalidApr()` and selector `0x76c22a22`
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
    #[etherror(name = "InvalidApr", abi = "InvalidApr()")]
    pub struct InvalidApr;
    ///Custom Error type `InvalidEffectiveShareReserves` with signature `InvalidEffectiveShareReserves()` and selector `0x85bd2ac4`
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
        name = "InvalidEffectiveShareReserves",
        abi = "InvalidEffectiveShareReserves()"
    )]
    pub struct InvalidEffectiveShareReserves;
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
    ///Custom Error type `PoolAlreadyInitialized` with signature `PoolAlreadyInitialized()` and selector `0x7983c051`
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
    #[etherror(name = "PoolAlreadyInitialized", abi = "PoolAlreadyInitialized()")]
    pub struct PoolAlreadyInitialized;
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
    ///Custom Error type `UpdateLiquidityFailed` with signature `UpdateLiquidityFailed()` and selector `0x5044b7f5`
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
    #[etherror(name = "UpdateLiquidityFailed", abi = "UpdateLiquidityFailed()")]
    pub struct UpdateLiquidityFailed;
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
    pub enum ERC4626Target3Errors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        BelowMinimumContribution(BelowMinimumContribution),
        CircuitBreakerTriggered(CircuitBreakerTriggered),
        DecreasedPresentValueWhenAddingLiquidity(
            DecreasedPresentValueWhenAddingLiquidity,
        ),
        DistributeExcessIdleFailed(DistributeExcessIdleFailed),
        ExpInvalidExponent(ExpInvalidExponent),
        FailedInnerCall(FailedInnerCall),
        InsufficientLiquidity(InsufficientLiquidity),
        InvalidApr(InvalidApr),
        InvalidEffectiveShareReserves(InvalidEffectiveShareReserves),
        InvalidTimestamp(InvalidTimestamp),
        LnInvalidInput(LnInvalidInput),
        MinimumTransactionAmount(MinimumTransactionAmount),
        NotPayable(NotPayable),
        OutputLimit(OutputLimit),
        PoolAlreadyInitialized(PoolAlreadyInitialized),
        PoolIsPaused(PoolIsPaused),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        RestrictedZeroAddress(RestrictedZeroAddress),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        TransferFailed(TransferFailed),
        UnsafeCastToInt128(UnsafeCastToInt128),
        UnsafeCastToInt256(UnsafeCastToInt256),
        UnsafeCastToUint112(UnsafeCastToUint112),
        UnsafeCastToUint128(UnsafeCastToUint128),
        UpdateLiquidityFailed(UpdateLiquidityFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ERC4626Target3Errors {
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
            if let Ok(decoded) = <BelowMinimumContribution as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BelowMinimumContribution(decoded));
            }
            if let Ok(decoded) = <CircuitBreakerTriggered as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CircuitBreakerTriggered(decoded));
            }
            if let Ok(decoded) = <DecreasedPresentValueWhenAddingLiquidity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DecreasedPresentValueWhenAddingLiquidity(decoded));
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
            if let Ok(decoded) = <InvalidApr as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidApr(decoded));
            }
            if let Ok(decoded) = <InvalidEffectiveShareReserves as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidEffectiveShareReserves(decoded));
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
            if let Ok(decoded) = <PoolAlreadyInitialized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolAlreadyInitialized(decoded));
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
            if let Ok(decoded) = <UpdateLiquidityFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateLiquidityFailed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC4626Target3Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressInsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BelowMinimumContribution(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CircuitBreakerTriggered(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreasedPresentValueWhenAddingLiquidity(element) => {
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
                Self::InvalidApr(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEffectiveShareReserves(element) => {
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
                Self::NotPayable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutputLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolAlreadyInitialized(element) => {
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
                Self::UpdateLiquidityFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ERC4626Target3Errors {
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
                    == <BelowMinimumContribution as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CircuitBreakerTriggered as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DecreasedPresentValueWhenAddingLiquidity as ::ethers::contract::EthError>::selector() => {
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
                    == <InvalidApr as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidEffectiveShareReserves as ::ethers::contract::EthError>::selector() => {
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
                    == <NotPayable as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <OutputLimit as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <PoolAlreadyInitialized as ::ethers::contract::EthError>::selector() => {
                    true
                }
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
                _ if selector
                    == <UpdateLiquidityFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ERC4626Target3Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BelowMinimumContribution(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CircuitBreakerTriggered(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DecreasedPresentValueWhenAddingLiquidity(element) => {
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
                Self::InvalidApr(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidEffectiveShareReserves(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::LnInvalidInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumTransactionAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutputLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolAlreadyInitialized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::UpdateLiquidityFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ERC4626Target3Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for ERC4626Target3Errors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for ERC4626Target3Errors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<BelowMinimumContribution> for ERC4626Target3Errors {
        fn from(value: BelowMinimumContribution) -> Self {
            Self::BelowMinimumContribution(value)
        }
    }
    impl ::core::convert::From<CircuitBreakerTriggered> for ERC4626Target3Errors {
        fn from(value: CircuitBreakerTriggered) -> Self {
            Self::CircuitBreakerTriggered(value)
        }
    }
    impl ::core::convert::From<DecreasedPresentValueWhenAddingLiquidity>
    for ERC4626Target3Errors {
        fn from(value: DecreasedPresentValueWhenAddingLiquidity) -> Self {
            Self::DecreasedPresentValueWhenAddingLiquidity(value)
        }
    }
    impl ::core::convert::From<DistributeExcessIdleFailed> for ERC4626Target3Errors {
        fn from(value: DistributeExcessIdleFailed) -> Self {
            Self::DistributeExcessIdleFailed(value)
        }
    }
    impl ::core::convert::From<ExpInvalidExponent> for ERC4626Target3Errors {
        fn from(value: ExpInvalidExponent) -> Self {
            Self::ExpInvalidExponent(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for ERC4626Target3Errors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidity> for ERC4626Target3Errors {
        fn from(value: InsufficientLiquidity) -> Self {
            Self::InsufficientLiquidity(value)
        }
    }
    impl ::core::convert::From<InvalidApr> for ERC4626Target3Errors {
        fn from(value: InvalidApr) -> Self {
            Self::InvalidApr(value)
        }
    }
    impl ::core::convert::From<InvalidEffectiveShareReserves> for ERC4626Target3Errors {
        fn from(value: InvalidEffectiveShareReserves) -> Self {
            Self::InvalidEffectiveShareReserves(value)
        }
    }
    impl ::core::convert::From<InvalidTimestamp> for ERC4626Target3Errors {
        fn from(value: InvalidTimestamp) -> Self {
            Self::InvalidTimestamp(value)
        }
    }
    impl ::core::convert::From<LnInvalidInput> for ERC4626Target3Errors {
        fn from(value: LnInvalidInput) -> Self {
            Self::LnInvalidInput(value)
        }
    }
    impl ::core::convert::From<MinimumTransactionAmount> for ERC4626Target3Errors {
        fn from(value: MinimumTransactionAmount) -> Self {
            Self::MinimumTransactionAmount(value)
        }
    }
    impl ::core::convert::From<NotPayable> for ERC4626Target3Errors {
        fn from(value: NotPayable) -> Self {
            Self::NotPayable(value)
        }
    }
    impl ::core::convert::From<OutputLimit> for ERC4626Target3Errors {
        fn from(value: OutputLimit) -> Self {
            Self::OutputLimit(value)
        }
    }
    impl ::core::convert::From<PoolAlreadyInitialized> for ERC4626Target3Errors {
        fn from(value: PoolAlreadyInitialized) -> Self {
            Self::PoolAlreadyInitialized(value)
        }
    }
    impl ::core::convert::From<PoolIsPaused> for ERC4626Target3Errors {
        fn from(value: PoolIsPaused) -> Self {
            Self::PoolIsPaused(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for ERC4626Target3Errors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<RestrictedZeroAddress> for ERC4626Target3Errors {
        fn from(value: RestrictedZeroAddress) -> Self {
            Self::RestrictedZeroAddress(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for ERC4626Target3Errors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<TransferFailed> for ERC4626Target3Errors {
        fn from(value: TransferFailed) -> Self {
            Self::TransferFailed(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToInt128> for ERC4626Target3Errors {
        fn from(value: UnsafeCastToInt128) -> Self {
            Self::UnsafeCastToInt128(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToInt256> for ERC4626Target3Errors {
        fn from(value: UnsafeCastToInt256) -> Self {
            Self::UnsafeCastToInt256(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToUint112> for ERC4626Target3Errors {
        fn from(value: UnsafeCastToUint112) -> Self {
            Self::UnsafeCastToUint112(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToUint128> for ERC4626Target3Errors {
        fn from(value: UnsafeCastToUint128) -> Self {
            Self::UnsafeCastToUint128(value)
        }
    }
    impl ::core::convert::From<UpdateLiquidityFailed> for ERC4626Target3Errors {
        fn from(value: UpdateLiquidityFailed) -> Self {
            Self::UpdateLiquidityFailed(value)
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
    pub enum ERC4626Target3Events {
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
    impl ::ethers::contract::EthLogDecode for ERC4626Target3Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddLiquidityFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::AddLiquidityFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = CheckpointRewarderUpdatedFilter::decode_log(log) {
                return Ok(
                    ERC4626Target3Events::CheckpointRewarderUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = CloseLongFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::CloseLongFilter(decoded));
            }
            if let Ok(decoded) = CloseShortFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::CloseShortFilter(decoded));
            }
            if let Ok(decoded) = CollectGovernanceFeeFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::CollectGovernanceFeeFilter(decoded));
            }
            if let Ok(decoded) = CreateCheckpointFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::CreateCheckpointFilter(decoded));
            }
            if let Ok(decoded) = FeeCollectorUpdatedFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::FeeCollectorUpdatedFilter(decoded));
            }
            if let Ok(decoded) = GovernanceUpdatedFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::GovernanceUpdatedFilter(decoded));
            }
            if let Ok(decoded) = InitializeFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::InitializeFilter(decoded));
            }
            if let Ok(decoded) = OpenLongFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::OpenLongFilter(decoded));
            }
            if let Ok(decoded) = OpenShortFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::OpenShortFilter(decoded));
            }
            if let Ok(decoded) = PauseStatusUpdatedFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::PauseStatusUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PauserUpdatedFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::PauserUpdatedFilter(decoded));
            }
            if let Ok(decoded) = RedeemWithdrawalSharesFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::RedeemWithdrawalSharesFilter(decoded));
            }
            if let Ok(decoded) = RemoveLiquidityFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::RemoveLiquidityFilter(decoded));
            }
            if let Ok(decoded) = SweepFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::SweepFilter(decoded));
            }
            if let Ok(decoded) = SweepCollectorUpdatedFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::SweepCollectorUpdatedFilter(decoded));
            }
            if let Ok(decoded) = TransferSingleFilter::decode_log(log) {
                return Ok(ERC4626Target3Events::TransferSingleFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ERC4626Target3Events {
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
    impl ::core::convert::From<AddLiquidityFilter> for ERC4626Target3Events {
        fn from(value: AddLiquidityFilter) -> Self {
            Self::AddLiquidityFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalFilter> for ERC4626Target3Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for ERC4626Target3Events {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<CheckpointRewarderUpdatedFilter>
    for ERC4626Target3Events {
        fn from(value: CheckpointRewarderUpdatedFilter) -> Self {
            Self::CheckpointRewarderUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<CloseLongFilter> for ERC4626Target3Events {
        fn from(value: CloseLongFilter) -> Self {
            Self::CloseLongFilter(value)
        }
    }
    impl ::core::convert::From<CloseShortFilter> for ERC4626Target3Events {
        fn from(value: CloseShortFilter) -> Self {
            Self::CloseShortFilter(value)
        }
    }
    impl ::core::convert::From<CollectGovernanceFeeFilter> for ERC4626Target3Events {
        fn from(value: CollectGovernanceFeeFilter) -> Self {
            Self::CollectGovernanceFeeFilter(value)
        }
    }
    impl ::core::convert::From<CreateCheckpointFilter> for ERC4626Target3Events {
        fn from(value: CreateCheckpointFilter) -> Self {
            Self::CreateCheckpointFilter(value)
        }
    }
    impl ::core::convert::From<FeeCollectorUpdatedFilter> for ERC4626Target3Events {
        fn from(value: FeeCollectorUpdatedFilter) -> Self {
            Self::FeeCollectorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<GovernanceUpdatedFilter> for ERC4626Target3Events {
        fn from(value: GovernanceUpdatedFilter) -> Self {
            Self::GovernanceUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializeFilter> for ERC4626Target3Events {
        fn from(value: InitializeFilter) -> Self {
            Self::InitializeFilter(value)
        }
    }
    impl ::core::convert::From<OpenLongFilter> for ERC4626Target3Events {
        fn from(value: OpenLongFilter) -> Self {
            Self::OpenLongFilter(value)
        }
    }
    impl ::core::convert::From<OpenShortFilter> for ERC4626Target3Events {
        fn from(value: OpenShortFilter) -> Self {
            Self::OpenShortFilter(value)
        }
    }
    impl ::core::convert::From<PauseStatusUpdatedFilter> for ERC4626Target3Events {
        fn from(value: PauseStatusUpdatedFilter) -> Self {
            Self::PauseStatusUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PauserUpdatedFilter> for ERC4626Target3Events {
        fn from(value: PauserUpdatedFilter) -> Self {
            Self::PauserUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<RedeemWithdrawalSharesFilter> for ERC4626Target3Events {
        fn from(value: RedeemWithdrawalSharesFilter) -> Self {
            Self::RedeemWithdrawalSharesFilter(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityFilter> for ERC4626Target3Events {
        fn from(value: RemoveLiquidityFilter) -> Self {
            Self::RemoveLiquidityFilter(value)
        }
    }
    impl ::core::convert::From<SweepFilter> for ERC4626Target3Events {
        fn from(value: SweepFilter) -> Self {
            Self::SweepFilter(value)
        }
    }
    impl ::core::convert::From<SweepCollectorUpdatedFilter> for ERC4626Target3Events {
        fn from(value: SweepCollectorUpdatedFilter) -> Self {
            Self::SweepCollectorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TransferSingleFilter> for ERC4626Target3Events {
        fn from(value: TransferSingleFilter) -> Self {
            Self::TransferSingleFilter(value)
        }
    }
    ///Container type for all input parameters for the `addLiquidity` function with signature `addLiquidity(uint256,uint256,uint256,uint256,(address,bool,bytes))` and selector `0x4c2ac1d9`
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
        name = "addLiquidity",
        abi = "addLiquidity(uint256,uint256,uint256,uint256,(address,bool,bytes))"
    )]
    pub struct AddLiquidityCall {
        pub contribution: ::ethers::core::types::U256,
        pub min_lp_share_price: ::ethers::core::types::U256,
        pub min_apr: ::ethers::core::types::U256,
        pub max_apr: ::ethers::core::types::U256,
        pub options: Options,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint256,uint256,(address,bool,bytes))` and selector `0x77d05ff4`
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
        name = "initialize",
        abi = "initialize(uint256,uint256,(address,bool,bytes))"
    )]
    pub struct InitializeCall {
        pub contribution: ::ethers::core::types::U256,
        pub apr: ::ethers::core::types::U256,
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
    pub enum ERC4626Target3Calls {
        AddLiquidity(AddLiquidityCall),
        Initialize(InitializeCall),
    }
    impl ::ethers::core::abi::AbiDecode for ERC4626Target3Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddLiquidity(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC4626Target3Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ERC4626Target3Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddLiquidityCall> for ERC4626Target3Calls {
        fn from(value: AddLiquidityCall) -> Self {
            Self::AddLiquidity(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for ERC4626Target3Calls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    ///Container type for all return fields from the `addLiquidity` function with signature `addLiquidity(uint256,uint256,uint256,uint256,(address,bool,bytes))` and selector `0x4c2ac1d9`
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
    pub struct AddLiquidityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `initialize` function with signature `initialize(uint256,uint256,(address,bool,bytes))` and selector `0x77d05ff4`
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
    pub struct InitializeReturn(pub ::ethers::core::types::U256);
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

pub struct ERC4626Target3Libs {
    pub lp_math: ::ethers::types::Address,
}

impl<M: ::ethers::providers::Middleware> ERC4626Target3<M> {
    pub fn link_and_deploy<T: ::ethers::core::abi::Tokenize>(
        client: ::std::sync::Arc<M>,
        constructor_args: T,
        libs: ERC4626Target3Libs,
    ) -> ::core::result::Result<
        ::ethers::contract::builders::ContractDeployer<M, Self>,
        ::ethers::contract::ContractError<M>,
    > {
        let factory = crate::linked_factory::create(
            ERC4626TARGET3_ABI.clone(),
            "0x6102606040523480156200001257600080fd5b506040516200429038038062004290833981016040819052620000359162000202565b600160005580516001600160a01b039081166080908152602080840151831660a0908152918401516101a0908152918401516101c090815260c0808601516101e090815260e0808801516102005261012080890151909352610100808901519091526101408089015190915290870180515190925281519093015190925281516040908101516101609081529251606090810151610180908152918701518616610220528601516102405291850151600980546001600160a01b031990811692871692909217905591850151600a8054841691861691909117905591840151600b80548316918516919091179055920151600c8054909316911617905562000315565b60405161020081016001600160401b03811182821017156200016a57634e487b7160e01b600052604160045260246000fd5b60405290565b80516001600160a01b03811681146200018857600080fd5b919050565b600060808284031215620001a057600080fd5b604051608081016001600160401b0381118282101715620001d157634e487b7160e01b600052604160045260246000fd5b8060405250809150825181526020830151602082015260408301516040820152606083015160608201525092915050565b600061026082840312156200021657600080fd5b6200022062000138565b6200022b8362000170565b81526200023b6020840162000170565b60208201526200024e6040840162000170565b6040820152606083015160608201526080830151608082015260a083015160a082015260c083015160c082015260e083015160e0820152610100808401518183015250610120808401518183015250610140808401518183015250610160620002b981850162000170565b90820152610180620002cd84820162000170565b908201526101a0620002e184820162000170565b908201526101c0620002f584820162000170565b908201526101e06200030a858583016200018d565b908201529392505050565b60805160a05160c05160e05161010051610120516101405161016051610180516101a0516101c0516101e051610200516102205161024051613da2620004ee60003960005050600050506000818161030301528181610332015261035f01526000818160de015281816105a9015261169901526000818161085601528181610941015281816109d401528181610add0152818161156a01528181611673015281816120da0152818161213d01528181612a1c01528181612e240152612e6001526000818161013f015281816108c601528181610ea70152818161142e01528181611492015281816114d9015261164d01526000611cdf015260008181611e3a01526130b901526000818161106b0152611e0e0152600061306f0152600081816101810152818161091901528181610ec8015281816113fe01528181611462015281816114fa01526116bf01526000818161016001528181610238015281816102d8015281816106f3015281816108f301528181610fa301526129b201526000818161029701528181610d6601528181610de101528181610e5701528181610f230152610f5b01526000818161196f015281816119e901528181611a7b0152612d0001526000818161194201526119a30152613da26000f3fe6080604052600436106100295760003560e01c80634c2ac1d91461002e57806377d05ff414610053575b600080fd5b61004161003c3660046137ec565b610066565b60405190815260200160405180910390f35b610041610061366004613850565b610081565b60006100758686868686610098565b90505b95945050505050565b600061008e848484610800565b90505b9392505050565b60006100a2610bb9565b600654610100900460ff16156100cb576040516321081abf60e01b815260040160405180910390fd5b6100d3610be3565b6100dc82610c04565b7f000000000000000000000000000000000000000000000000000000000000000086101561011d5760405163211ddda360e11b815260040160405180910390fd5b60006101a561012a610c3d565b600254600160801b90046001600160801b03167f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000610c63565b9050848110806101b457508381115b156101d257604051633b61151160e11b815260040160405180910390fd5b6000806101df8986610c89565b9150915060006101ed610d5e565b90506101fd818360046001610d8a565b506002546004546003546000928392610267926001600160801b0380841693600f9390930b92600160801b900481169189911661026261025d7f00000000000000000000000000000000000000000000000000000000000000008b6138b6565b611381565b6113cb565b915091508061028957604051630552b01360e31b815260040160405180910390fd5b8b60006102fc6008826102bc7f0000000000000000000000000000000000000000000000000000000000000000896138c9565b81526020810191909152604001600020546001600160801b03167f00000000000000000000000000000000000000000000000000000000000000006115d8565b90506103287f0000000000000000000000000000000000000000000000000000000000000000826138b6565b88118061038757507f00000000000000000000000000000000000000000000000000000000000000008111801561038757506103847f0000000000000000000000000000000000000000000000000000000000000000826138c9565b88105b156103a557604051630552b01360e31b815260040160405180910390fd5b50600754600360f81b600090815260106020527fd6f7110f7a6485ce27c724322bdc9b60c9b2518194eace178d55653f1be730215490916103f1916001600160801b03909116906138c9565b600080805260106020527f6e0956cda88cad152e89927e53611735b61a5c762d1428573c6931b0a5efcb01549192509061042c9083906138b6565b9050600080600061043c8a611604565b604051635f9d50ab60e11b815290915073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__9063bf3aa1569061047690849060040161395a565b602060405180830381865af4158015610493573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104b79190613969565b91506104ca6104c58c61173e565b61176c565b6002546001600160801b03808216835260048054600f0b6020850152600160801b9092041660408084019190915251635f9d50ab60e11b815273__$2b4fa6f02a36eedfe41c65e8dd342257d3$__9163bf3aa1569161052b9185910161395a565b602060405180830381865af4158015610548573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061056c9190613969565b92508183101561058f5760405163184d952160e11b815260040160405180910390fd5b6105a5848361059e81876138c9565b9190611792565b9c507f00000000000000000000000000000000000000000000000000000000000000008d10156105e85760405163211ddda360e11b815260040160405180910390fd5b508f6105f4868e6117b0565b10156106135760405163c972651760e01b815260040160405180910390fd5b61063160008e600001602081019061062b9190613982565b8e6117c5565b61063a8961186d565b95508561065a57604051638bdf918d60e01b815260040160405180910390fd5b600088905060008d9050368f905060008c9050600061071d600260000160009054906101000a90046001600160801b03166001600160801b03166002800160009054906101000a9004600f0b600f0b600260000160109054906101000a90046001600160801b03166001600160801b031685600260010160009054906101000a90046001600160801b03166001600160801b03166102627f00000000000000000000000000000000000000000000000000000000000000008c61025d91906138b6565b9b5090508a158061073757506107348c600061187a565b81125b1561075557604051630552b01360e31b815260040160405180910390fd5b6000881561076d5761076887848b611792565b610770565b60005b905061077f6020850185613982565b6001600160a01b03167fdcc4a01cea4510bd52201cebc8cd2d47d60429b35d68329abc591a70aa2efabf868d866107bc60408a0160208b016139b9565b866107ca60408c018c6139d6565b6040516107dd9796959493929190613a1d565b60405180910390a250505050505050505050505050505050506100786001600055565b600061080a610bb9565b610812610be3565b61081b82610c04565b60065460ff161561083f57604051637983c05160e01b815260040160405180910390fd5b60008061084c8685610c89565b909250905061087c7f00000000000000000000000000000000000000000000000000000000000000006002613a71565b82101561089c57604051632afb507160e21b815260040160405180910390fd5b6006805460ff1916600117905560405163594f0e8f60e11b815260048101839052602481018290527f00000000000000000000000000000000000000000000000000000000000000006044820152606481018690527f000000000000000000000000000000000000000000000000000000000000000060848201527f000000000000000000000000000000000000000000000000000000000000000060a482015260027f000000000000000000000000000000000000000000000000000000000000000002830393506000908190819073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__9063b29e1d1e9060c401606060405180830381865af41580156109a8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109cc9190613a88565b9250925092507f00000000000000000000000000000000000000000000000000000000000000006109fd848461188f565b1015610a1c5760405163216f4ab160e21b815260040160405180910390fd5b600080610a2e858585896000806113cb565b91509150801580610a3f5750600082125b15610a5d57604051630552b01360e31b815260040160405180910390fd5b610a66856118b5565b600280546001600160801b0319166001600160801b0392909216919091179055610a8f846118df565b600480546001600160801b0319166001600160801b0392909216919091179055610ab8836118b5565b600280546001600160801b03928316600160801b029216919091179055610b016000807f00000000000000000000000000000000000000000000000000000000000000006117c5565b610b196000610b1360208c018c613982565b8a6117c5565b610b2e610b24610d5e565b8760046001610d8a565b508a8a8a610b3f6020820182613982565b6001600160a01b03167f4931b9953a65531203c17d9abe77870a3e49d8b13af522ec3321c18b5abb8af38c858c610b7c60408701602088016139b9565b87610b8a60408901896139d6565b604051610b9d9796959493929190613a1d565b60405180910390a2505050505050505050506100916001600055565b600260005403610bdc57604051633ee5aeb560e01b815260040160405180910390fd5b6002600055565b3415610c0257604051631574f9f360e01b815260040160405180910390fd5b565b6000610c136020830183613982565b6001600160a01b031603610c3a5760405163f0dd15fd60e01b815260040160405180910390fd5b50565b600254600454600091610c5e916001600160801b0390911690600f0b61188f565b905090565b600080610c728787878661191b565b9050610c7e81856115d8565b979650505050505050565b81600080610c9d60408501602086016139b9565b15610cc257610cb885610cb360408701876139d6565b611932565b9093509050610cda565b5034610cda85610cd560408701876139d6565b611a6e565b610ce2611aa8565b91508015610d5557604051600090339083908381818185875af1925050503d8060008114610d2c576040519150601f19603f3d011682016040523d82523d6000602084013e610d31565b606091505b5050905080610d53576040516312171d8360e31b815260040160405180910390fd5b505b505b9250929050565b6000610c5e427f0000000000000000000000000000000000000000000000000000000000000000611abb565b600084815260086020526040812060018101546001600160801b0316151580610db257504286115b15610dcb57600101546001600160801b03169050611379565b6000806000610dd8610d5e565b90506000610e067f00000000000000000000000000000000000000000000000000000000000000008b6138b6565b90505b81811015610e7c576000818152600860205260409020600101546001600160801b03168015610e54576000828152600860205260409020549094506001600160801b03169250610e7c565b507f000000000000000000000000000000000000000000000000000000000000000001610e09565b83600003610eef57889350610eec610e92610c3d565b600254600160801b90046001600160801b03167f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000061191b565b92505b50610ef9836118b5565b6001850180546001600160801b0319166001600160801b0392909216919091179055610f4f610f487f00000000000000000000000000000000000000000000000000000000000000008b6138c9565b8a84611ad1565b610f8d89610f87610f807f0000000000000000000000000000000000000000000000000000000000000000836138b6565b4290611c0f565b84611ad1565b610f9688611c1e565b5060009050600881610fc87f00000000000000000000000000000000000000000000000000000000000000008d6138c9565b815260208101919091526040016000908120600101546001600160801b03169150610ff460028c611dc2565b6000818152601060205260408120549192508c8c831561113a576001925060008061102386898d866000611df7565b9150915080600d600082825461103991906138b6565b9091555061105590508660008461104f8161173e565b88611ec6565b61105f81836138b6565b915061108f86838a8e877f0000000000000000000000000000000000000000000000000000000000000000612040565b91506110a361109e8385612084565b612099565b600680546002906110c49084906201000090046001600160701b0316613ab6565b92506101000a8154816001600160701b0302191690836001600160701b031602179055506110f1826118b5565b60068054601090611113908490600160801b90046001600160801b0316613ad6565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555050505b6000611147600184611dc2565b60008181526010602052604090205490915080156112565760019450600080611174838b8f886001611df7565b9150915080600d600082825461118a91906138b6565b909155506111a69050836000846111a08161173e565b8a6120c3565b6111b081836138c9565b91506111bf61109e8387612084565b600680546002906111e09084906201000090046001600160701b0316613ab6565b92506101000a8154816001600160701b0302191690836001600160701b0316021790555061120d826118b5565b6006805460109061122f908490600160801b90046001600160801b0316613ad6565b92506101000a8154816001600160801b0302191690836001600160801b0316021790555050505b8415611290576112826112688761173e565b6112718361173e565b61127b9190613af6565b60006122b4565b8e61128d84826122f3565b50505b600061129b846124ed565b50604080518e815260208101879052908101899052606081018490526080810182905290915085907fff888cf98d2696e95c8c39aa98c9ad55a5378008f7a56614c9353b7137a57ab79060a00160405180910390a2600c546001600160a01b031615611369578e6113655a604051336024820152604481018990528315156064820152600090819060840160408051601f198184030181529190526020810180516001600160e01b0316633488a6a760e11b179052600c546001600160a01b031693929190612639565b5050505b50999b5050505050505050505050505b949350505050565b60006113a960106000611395600286611dc2565b81526020019081526020016000205461173e565b6113bb60106000611395600187611dc2565b6113c59190613af6565b92915050565b6000806000806113db8a8a6126c6565b91509150806113f2576000809350935050506115cd565b6000611452838a61142b7f0000000000000000000000000000000000000000000000000000000000000000670de0b6b3a76400006138c9565b8b7f0000000000000000000000000000000000000000000000000000000000000000612702565b50905060006114b6848b8461148f7f0000000000000000000000000000000000000000000000000000000000000000670de0b6b3a76400006138c9565b8d7f00000000000000000000000000000000000000000000000000000000000000006127bf565b5090508115806114c4575080155b156114d0575060009050805b600061151e858c7f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000061191b565b905061152c83838c8461287d565b509093506000905061153e848f6138b6565b9050600061155f8b8b6115508761173e565b61155a908e613b16565b6128c9565b90508b61159461158f7f000000000000000000000000000000000000000000000000000000000000000083612930565b61173e565b61159d8361173e565b6115aa61158f8685612084565b6115b49190613af6565b6115be9190613af6565b60019950995050505050505050505b965096945050505050565b60006100916115ec84846301e13380612941565b6115fe85670de0b6b3a76400006138c9565b906117b0565b61160c613727565b60408051610180810182526002546001600160801b038082168352600454600f0b6020840152600160801b91829004811693830193909352606082018590527f000000000000000000000000000000000000000000000000000000000000000060808301527f000000000000000000000000000000000000000000000000000000000000000060a08301527f000000000000000000000000000000000000000000000000000000000000000060c08301527f000000000000000000000000000000000000000000000000000000000000000060e083015260035404821661010082015260055490916101208301916117049116612967565b81526004546001600160801b03600160801b918290048116602084015260055460409093019261173692900416612967565b905292915050565b60006001600160ff1b038211156117685760405163396ea70160e11b815260040160405180910390fd5b5090565b611775816129d6565b610c3a57604051635044b7f560e01b815260040160405180910390fd5b60008260001904841183021582026117a957600080fd5b5091020490565b600061009183670de0b6b3a764000084611792565b6000838152600f602090815260408083206001600160a01b0386168452909152812080548392906117f79084906138b6565b90915550506000838152601060205260408120805483929061181a9084906138b6565b909155505060408051848152602081018390526001600160a01b0384169160009133917fc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f62910160405180910390a4505050565b60006113c58260046122f3565b60008183136118895782610091565b50919050565b60008061189c84846126c6565b9092509050806118ae576118ae612b6e565b5092915050565b60006001600160801b0382111561176857604051630f0af95160e11b815260040160405180910390fd5b600060016001607f1b03198212806118fd575060016001607f1b0382135b156117685760405163a5353be560e01b815260040160405180910390fd5b60006100788261192c858888611792565b90612b87565b60008061196a6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016333088612bfc565b6119ca7f00000000000000000000000000000000000000000000000000000000000000006119998760016138b6565b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169190612c69565b604051636e553f6560e01b8152600481018690523060248201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690636e553f65906044016020604051808303816000875af1158015611a3a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a5e9190613969565b925060009150505b935093915050565b611aa36001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016333086612bfc565b505050565b6000610c5e670de0b6b3a7640000612cf9565b6000611ac78284613b54565b61009190846138c9565b600083815260086020526040902054600160801b90046001600160801b0316808303611afd5750505050565b6000848152600860205260408120546001600160801b031690819003611b5757611b26836118b5565b600086815260086020526040902080546001600160801b0319166001600160801b0392909216919091179055611bd5565b611ba8611ba3670de0b6b3a7640000611b7088866138c9565b611b7a9190613a71565b85670de0b6b3a7640000611b8e878a6138c9565b611b989190613a71565b859291906001612d25565b6118b5565b600086815260086020526040902080546001600160801b0319166001600160801b03929092169190911790555b611bde846118b5565b60009586526008602052604090952080546001600160801b03968716600160801b0296169590951790945550505050565b60008183116118895782610091565b6006546000908190611c41908490600160801b90046001600160801b0316612084565b6006546201000090046001600160701b03169250905081811115611dbd576000611c6b83836138c9565b9050611c7a611ba38286612dc3565b60068054601090611c9c908490600160801b90046001600160801b0316613b68565b92506101000a8154816001600160801b0302191690836001600160801b031602179055506000611cd585836117b090919063ffffffff16565b90506000611d03827f0000000000000000000000000000000000000000000000000000000000000000612084565b905080600d6000828254611d1791906138b6565b90915550611d27905081836138c9565b9150611d32826118b5565b60028054600090611d4d9084906001600160801b0316613ad6565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550611d7a82612dd8565b60048054600090611d8f908490600f0b613b88565b92506101000a8154816001600160801b030219169083600f0b6001600160801b031602179055508493505050505b915091565b60006001600160f81b03821115611dec5760405163b7d0949760e01b815260040160405180910390fd5b5060f89190911b1790565b600080611e0487856117b0565b91506000611e32837f0000000000000000000000000000000000000000000000000000000000000000612084565b9050611e5e817f0000000000000000000000000000000000000000000000000000000000000000612084565b91508315611e8157611e7082826138c9565b611e7a90846138c9565b9250611e98565b611e8b82826138c9565b611e9590846138b6565b92505b86861015611ebb57611eab838789611792565b9250611eb8828789611792565b91505b509550959350505050565b600454600160801b90046001600160801b0316611f0f611ba382611ef2670de0b6b3a764000086613a71565b600554600160801b90046001600160801b031691908a6000612d25565b600580546001600160801b03928316600160801b029216919091179055611f35866118b5565b611f3f9082613b68565b600480546001600160801b03928316600160801b029216919091179055611f65846118b5565b60028054600090611f809084906001600160801b0316613ad6565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550611fad836118df565b60048054600090611fc2908490600f0b613b88565b92506101000a8154816001600160801b030219169083600f0b6001600160801b03160217905550611ff2856118b5565b60028054601090612014908490600160801b90046001600160801b0316613b68565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550505050505050565b600080612052846115fe8a888a611792565b905061205f888486611792565b61206990826138b6565b9050868111156120795786810391505b509695505050505050565b60006100918383670de0b6b3a7640000611792565b60006001600160701b038211156117685760405163086b151760e11b815260040160405180910390fd5b6002546001600160801b03168381108061210557507f000000000000000000000000000000000000000000000000000000000000000061210385836138c9565b105b1561211257612112612b6e565b6004549084900390600f0b6121278482613af6565b9050836121338661173e565b13801561216857507f0000000000000000000000000000000000000000000000000000000000000000612166838361188f565b105b1561217557612175612b6e565b600354600160801b90046001600160801b03166121b7611ba3826121a1670de0b6b3a764000088613a71565b6005546001600160801b031691908c6000612d25565b600580546001600160801b0319166001600160801b03929092169190911790556121e188826138c9565b90506121ec816118b5565b600380546001600160801b03928316600160801b029216919091179055612212836118b5565b600280546001600160801b0319166001600160801b039290921691909117905561223b826118df565b600480546001600160801b0319166001600160801b0392909216919091179055612264876118b5565b60028054601090612286908490600160801b90046001600160801b0316613ad6565b92506101000a8154816001600160801b0302191690836001600160801b031602179055505050505050505050565b6003546122cf90611ba3906001600160801b031684846128c9565b600380546001600160801b0319166001600160801b03929092169190911790555050565b600754600360f81b600090815260106020527fd6f7110f7a6485ce27c724322bdc9b60c9b2518194eace178d55653f1be73021549091829161233e916001600160801b0316906138c9565b9050806000036123525760019150506113c5565b600061235d85612e02565b905080600003612372576001925050506113c5565b600080612380838589612e9e565b91509150806123965760009450505050506113c5565b60008073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__632c03ef68858a6040518363ffffffff1660e01b81526004016123d2929190613bb5565b6040805180830381865af41580156123ee573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906124129190613c2b565b915091506124306124228261173e565b61242b90613c4f565b6129d6565b92508261244657600096505050505050506113c5565b61244f826118b5565b6007805460009061246a9084906001600160801b0316613ad6565b92506101000a8154816001600160801b0302191690836001600160801b03160217905550612497816118b5565b600780546010906124b9908490600160801b90046001600160801b0316613ad6565b92506101000a8154816001600160801b0302191690836001600160801b031602179055506001965050505050505092915050565b60008060008073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__635a1b419e61251687611604565b6040518263ffffffff1660e01b8152600401612532919061395a565b6040805180830381865af415801561254e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125729190613c6b565b9150915080612588575060009485945092505050565b60075460106020527fd6f7110f7a6485ce27c724322bdc9b60c9b2518194eace178d55653f1be730215460008080527f6e0956cda88cad152e89927e53611735b61a5c762d1428573c6931b0a5efcb015490926001600160801b0316916125ee916138b6565b6125f891906138c9565b90508060000361261057506000958695509350505050565b600080871161262057600061262b565b61262b848884611792565b976001975095505050505050565b6000606060008060008661ffff1667ffffffffffffffff81111561265f5761265f613c9b565b6040519080825280601f01601f191660200182016040528015612689576020820181803683370190505b5090506000808751602089018b8e8ef191503d9250868311156126aa578692505b828152826000602083013e9093509150505b9550959350505050565b6000806000836126d58661173e565b6126df9190613af6565b905060008112156126f7576000809250925050610d57565b946001945092505050565b6000806000612714888888888861300d565b9050600061273f670de0b6b3a764000061272e8888612dc3565b61273891906138b6565b83906117b0565b9050670de0b6b3a764000081106127735761276c612765670de0b6b3a7640000896117b0565b8290612b87565b905061278b565b612788612765670de0b6b3a764000089612dc3565b90505b61279581866117b0565b9050888110156127ad576000809350935050506126bc565b97909703976001975095505050505050565b60008060006127d1898988888861303c565b90506127eb8661192c6127e48a8d6138b6565b8790612084565b98506127f8858a86611792565b98508881101561280f5760008092509250506115cd565b888103670de0b6b3a7640000811061283d57612836612765670de0b6b3a764000089612dc3565b9050612855565b612852612765670de0b6b3a7640000896117b0565b90505b8089101561286b576000809350935050506115cd565b90970398600198509650505050505050565b6000806000806000612890898789613061565b909250905061289f82896138c9565b975060006128ae82888a611792565b90506128ba818b6138c9565b9a989950979650505050505050565b6000806128d78460006130e7565b6128e28460006130e7565b6128ec9190613af6565b905060008113156129085761290181866138b6565b9450612927565b60008112156129275761291a81613c4f565b61292490866138c9565b94505b50929392505050565b60006100918383670de0b6b3a76400005b600082600019048411830215820261295857600080fd5b50910281810615159190040190565b600080670de0b6b3a764000061297b610d5e565b6129859190613a71565b905080831161299557600061299f565b61299f81846138c9565b9150610091612738670de0b6b3a76400007f0000000000000000000000000000000000000000000000000000000000000000613a71565b6002546004805460405163685a2be760e11b81526001600160801b03808516938201849052600f9290920b60248201819052600160801b909404909116604482018190527f000000000000000000000000000000000000000000000000000000000000000060648301526084820185905260009391849081908190819073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__9063d0b457ce9060a401608060405180830381865af4158015612a8f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612ab39190613cb1565b935093509350935080612acf5750600098975050505050505050565b868414612b0057612adf846118b5565b600280546001600160801b0319166001600160801b03929092169190911790555b858314612b3157612b10836118df565b600480546001600160801b0319166001600160801b03929092169190911790555b848214612b5f57612b41826118b5565b600280546001600160801b03928316600160801b0292169190911790555b50600198975050505050505050565b60405163bb55fd2760e01b815260040160405180910390fd5b600081600003612ba05750670de0b6b3a76400006113c5565b82600003612bb0575060006113c5565b6000612bbb8361173e565b90506000612bd0612bcb8661173e565b6130fd565b9050818102612be7670de0b6b3a764000082613cf2565b9050612bf281613323565b9695505050505050565b6040516001600160a01b038481166024830152838116604483015260648201839052612c639186918216906323b872dd906084015b604051602081830303815290604052915060e01b6020820180516001600160e01b0383818316178352505050506134b8565b50505050565b604080516001600160a01b038416602482015260448082018490528251808303909101815260649091019091526020810180516001600160e01b031663095ea7b360e01b179052612cba8482613520565b612c63576040516001600160a01b03848116602483015260006044830152612cef91869182169063095ea7b390606401612c31565b612c6384826134b8565b60006113c57f0000000000000000000000000000000000000000000000000000000000000000836135c3565b600082600003612d36575084610078565b8115612d8957612d67612d4984876138b6565b612d538587612084565b612d5d888a612084565b6115fe91906138b6565b90506000612d758588611c0f565b905080821015612d83578091505b50610078565b828503612d9857506000610078565b610075612da584876138c9565b612daf8587612930565b612db9888a612084565b6115fe91906138c9565b600061009183670de0b6b3a764000084612941565b600060016001607f1b038211156117685760405163a5353be560e01b815260040160405180910390fd5b6003546000908190612e1d906001600160801b031684612dc3565b9050612e497f0000000000000000000000000000000000000000000000000000000000000000826138b6565b6002546001600160801b03161115611889576002547f000000000000000000000000000000000000000000000000000000000000000090612e949083906001600160801b03166138c9565b61009191906138c9565b612ea6613788565b600080612eb284611604565b9050600073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__635a1b419e836040518263ffffffff1660e01b8152600401612eed919061395a565b6040805180830381865af4158015612f09573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612f2d9190613c6b565b9350905082612f42575060009150611a669050565b6000612f6461158f84610160015185610140015161208490919063ffffffff16565b612f8461158f85610120015186610100015161293090919063ffffffff16565b612f8e9190613af6565b604080516101208101825285815260208082019590955260008052601085527f6e0956cda88cad152e89927e53611735b61a5c762d1428573c6931b0a5efcb0154818301526060810199909952608089019990995260a088015250815160c087015281015160e086015290940151610100840152509092600192509050565b60006130198585612b87565b61303261302a8661192c868b612084565b859085611792565b61007591906138b6565b60006130488585612b87565b6130326130598661192c868b612930565b859085612941565b6000806130b1856130ab85817f0000000000000000000000000000000000000000000000000000000000000000670de0b6b3a76400006130a1818c612dc3565b6130ab91906138c9565b90612930565b91506130dd827f0000000000000000000000000000000000000000000000000000000000000000612084565b9050935093915050565b60008183136130f65781610091565b5090919050565b600080821361311f5760405163e61b497560e01b815260040160405180910390fd5b506001600160801b03811160071b81811c67ffffffffffffffff1060061b1781811c63ffffffff1060051b1781811c61ffff1060041b1781811c60ff10600390811b90911782811c600f1060021b1782811c909110600190811b90911782811c90911017609f8181036060019290921b91605f198201906131a29084901c61173e565b6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d6c8c3f38e95a6b1ff2ab1c3b343619018302821d6d02384773bdf1ac5676facced60901901830290911d6cb9a025d814b29c212b8b1a07cd1901909102780a09507084cc699bb0e71ea869ffffffffffffffffffffffff190105711340daa0d5f769dba1915cef59f0815a5506027d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b391909102017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d92915050565b6000680248ce36a70cb26b3e19821361333e57506000919050565b680755bf798b4a1bf1e58212613367576040516373a2d6b160e01b815260040160405180910390fd5b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056001605f1b01901d6bb17217f7d1cf79abc9e3b3988102909303926c240c330e9fb2d9cbaf0fd5aafb1984018402606090811d6d0277594991cfc85f6e2461837cd9018502811d6d1a521255e34f6a5061b25ef1c9c319018502811d6db1bbb201f443cf962f1a1d3db4a5018502811d6e02c72388d9f74f51a9331fed693f1419018502811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765087018702831d9081019087016d01d3967ed30fc4f89c02bab570811901810290921d6e0587f503bb6ea29d25fcb7401964500186026d360d7aeea093263ecc6e0ecb291760621b018181059550929350909190612bf274029d9dc38563c32e5c2f6dc192ee70ef65f9978af3860260c38690031c61173e565b60006134cd6001600160a01b0384168361362f565b905080516000141580156134f25750808060200190518101906134f09190613d20565b155b15611aa357604051635274afe760e01b81526001600160a01b03841660048201526024015b60405180910390fd5b6000806000846001600160a01b03168460405161353d9190613d3d565b6000604051808303816000865af19150503d806000811461357a576040519150601f19603f3d011682016040523d82523d6000602084013e61357f565b606091505b50915091508180156135a95750805115806135a95750808060200190518101906135a99190613d20565b80156100785750505050506001600160a01b03163b151590565b6040516303d1689d60e11b8152600481018290526000906001600160a01b038416906307a2d13a90602401602060405180830381865afa15801561360b573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100919190613969565b60606100918383600084600080856001600160a01b031684866040516136559190613d3d565b60006040518083038185875af1925050503d8060008114613692576040519150601f19603f3d011682016040523d82523d6000602084013e613697565b606091505b5091509150612bf28683836060826136b7576136b2826136fe565b610091565b81511580156136ce57506001600160a01b0384163b155b156136f757604051639996b31560e01b81526001600160a01b0385166004820152602401613517565b5080610091565b80511561370e5780518082602001fd5b604051630a12f52160e11b815260040160405180910390fd5b6040518061018001604052806000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b60405180610120016040528061379c613727565b815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b60006060828403121561188957600080fd5b600080600080600060a0868803121561380457600080fd5b85359450602086013593506040860135925060608601359150608086013567ffffffffffffffff81111561383757600080fd5b613843888289016137da565b9150509295509295909350565b60008060006060848603121561386557600080fd5b8335925060208401359150604084013567ffffffffffffffff81111561388a57600080fd5b613896868287016137da565b9150509250925092565b634e487b7160e01b600052601160045260246000fd5b808201808211156113c5576113c56138a0565b818103818111156113c5576113c56138a0565b805182526020810151602083015260408101516040830152606081015160608301526080810151608083015260a081015160a083015260c081015160c083015260e081015160e08301526101008082015181840152506101208082015181840152506101408082015181840152506101608082015181840152505050565b61018081016113c582846138dc565b60006020828403121561397b57600080fd5b5051919050565b60006020828403121561399457600080fd5b81356001600160a01b038116811461009157600080fd5b8015158114610c3a57600080fd5b6000602082840312156139cb57600080fd5b8135610091816139ab565b6000808335601e198436030181126139ed57600080fd5b83018035915067ffffffffffffffff821115613a0857600080fd5b602001915036819003821315610d5757600080fd5b878152866020820152856040820152841515606082015283608082015260c060a08201528160c0820152818360e0830137600081830160e090810191909152601f909201601f191601019695505050505050565b80820281158282048414176113c5576113c56138a0565b600080600060608486031215613a9d57600080fd5b8351925060208401519150604084015190509250925092565b6001600160701b038181168382160190808211156118ae576118ae6138a0565b6001600160801b038181168382160190808211156118ae576118ae6138a0565b81810360008312801583831316838312821617156118ae576118ae6138a0565b8082018281126000831280158216821582161715613b3657613b366138a0565b505092915050565b634e487b7160e01b600052601260045260246000fd5b600082613b6357613b63613b3e565b500690565b6001600160801b038281168282160390808211156118ae576118ae6138a0565b600f81810b9083900b0160016001607f1b03811360016001607f1b0319821217156113c5576113c56138a0565b60006102a082019050613bc98285516138dc565b602084015161018083015260408401516101a083015260608401516101c083015260808401516101e083015260a084015161020083015260c084015161022083015260e084015161024083015261010090930151610260820152610280015290565b60008060408385031215613c3e57600080fd5b505080516020909101519092909150565b6000600160ff1b8201613c6457613c646138a0565b5060000390565b60008060408385031215613c7e57600080fd5b825191506020830151613c90816139ab565b809150509250929050565b634e487b7160e01b600052604160045260246000fd5b60008060008060808587031215613cc757600080fd5b8451935060208501519250604085015191506060850151613ce7816139ab565b939692955090935050565b600082613d0157613d01613b3e565b600160ff1b821460001984141615613d1b57613d1b6138a0565b500590565b600060208284031215613d3257600080fd5b8151610091816139ab565b6000825160005b81811015613d5e5760208186018101518583015201613d44565b50600092019182525091905056fea264697066735822122072f206e16961ae8572eca0c46ef2a5c232a3f188ca8deea51275068ce42d061564736f6c63430008160033",
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

