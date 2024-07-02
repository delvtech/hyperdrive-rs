pub use erc4626_hyperdrive_deployer_coordinator::*;
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
pub mod erc4626_hyperdrive_deployer_coordinator {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_factory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_coreDeployer"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_target0Deployer"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_target1Deployer"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_target2Deployer"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_target3Deployer"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("coreDeployer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("coreDeployer"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deployHyperdrive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deployHyperdrive"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_deploymentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("__name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_deployConfig"),
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
                                            "struct IHyperdrive.PoolDeployConfig",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deployTarget"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deployTarget"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_deploymentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_deployConfig"),
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
                                            "struct IHyperdrive.PoolDeployConfig",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_targetIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deployments"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deployments"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_deploymentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct HyperdriveDeployerCoordinator.Deployment",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("factory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("factory"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNumberOfTargets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNumberOfTargets"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                                    name: ::std::borrow::ToOwned::to_owned("_deploymentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_lp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("lpShares"),
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
                    ::std::borrow::ToOwned::to_owned("kind"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("kind"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("target0Deployer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("target0Deployer"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("target1Deployer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("target1Deployer"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("target2Deployer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("target2Deployer"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("target3Deployer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("target3Deployer"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("version"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
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
                    ::std::borrow::ToOwned::to_owned("DeploymentAlreadyExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeploymentAlreadyExists",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DeploymentDoesNotExist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeploymentDoesNotExist",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("HyperdriveAlreadyDeployed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HyperdriveAlreadyDeployed",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HyperdriveIsNotDeployed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HyperdriveIsNotDeployed",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncompleteDeployment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "IncompleteDeployment",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientValue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InsufficientValue"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidBaseToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidBaseToken"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidCheckpointDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidCheckpointDuration",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFeeAmounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidFeeAmounts"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMinimumShareReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidMinimumShareReserves",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMinimumTransactionAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidMinimumTransactionAmount",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPositionDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidPositionDuration",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTargetIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidTargetIndex"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidVaultSharesToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidVaultSharesToken",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MismatchedConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("MismatchedConfig"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MismatchedExtraData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MismatchedExtraData",
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
                    ::std::borrow::ToOwned::to_owned("SenderIsNotFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SenderIsNotFactory"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TargetAlreadyDeployed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TargetAlreadyDeployed",
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ERC4626HYPERDRIVEDEPLOYERCOORDINATOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01@`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0'\xEE8\x03\x80b\0'\xEE\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\0\xB7V[\x86\x86\x86\x86\x86\x86\x86`\0b\0\0J\x88\x82b\0\x02\x8BV[P`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x80R\x93\x85\x16`\xA0R\x91\x84\x16`\xC0R\x83\x16`\xE0R\x82\x16a\x01\0R\x16a\x01 RPb\0\x03W\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xB2W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15b\0\0\xD3W`\0\x80\xFD[\x87Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\0\xEBW`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12b\0\x01\0W`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x01\x15Wb\0\x01\x15b\0\0\x84V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0\x01@Wb\0\x01@b\0\0\x84V[\x81`@R\x82\x81R` \x93P\x8D\x84\x84\x87\x01\x01\x11\x15b\0\x01]W`\0\x80\xFD[`\0\x91P[\x82\x82\x10\x15b\0\x01\x81W\x84\x82\x01\x84\x01Q\x81\x83\x01\x85\x01R\x90\x83\x01\x90b\0\x01bV[`\0\x84\x84\x83\x01\x01R\x80\x9BPPPPb\0\x01\x9C\x81\x8B\x01b\0\0\x9AV[\x97PPPb\0\x01\xAE`@\x89\x01b\0\0\x9AV[\x94Pb\0\x01\xBE``\x89\x01b\0\0\x9AV[\x93Pb\0\x01\xCE`\x80\x89\x01b\0\0\x9AV[\x92Pb\0\x01\xDE`\xA0\x89\x01b\0\0\x9AV[\x91Pb\0\x01\xEE`\xC0\x89\x01b\0\0\x9AV[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x02\x11W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x022WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x02\x86W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x02aWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x02\x82W\x82\x81U`\x01\x01b\0\x02mV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x02\xA7Wb\0\x02\xA7b\0\0\x84V[b\0\x02\xBF\x81b\0\x02\xB8\x84Tb\0\x01\xFCV[\x84b\0\x028V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x02\xF7W`\0\x84\x15b\0\x02\xDEWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x02\x82V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x03(W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x03\x07V[P\x85\x82\x10\x15b\0\x03GW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa$\x0Fb\0\x03\xDF`\09`\0\x81\x81a\x03\x1F\x01Ra\x0B\xD9\x01R`\0\x81\x81a\x03\x87\x01Ra\n\xBC\x01R`\0\x81\x81a\x02\xD3\x01Ra\t\x9F\x01R`\0\x81\x81a\x03S\x01Ra\x07\x95\x01R`\0\x81\x81a\x04\x0F\x01Ra\x0E\xD4\x01R`\0\x81\x81a\x03\xDB\x01R\x81\x81a\x05\x1C\x01R\x81\x81a\x06\xBA\x01Ra\x0C\xC8\x01Ra$\x0F`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xDDW`\x005`\xE0\x1C\x80c\xABq\x90_\x11a\0\x7FW\x80c\xC4Z\x01U\x11a\0YW\x80c\xC4Z\x01U\x14a\x03\xC9W\x80c\xC8>\x1FQ\x14a\x03\xFDW\x80c\xE9\x90\xEB\xA8\x14a\x041W\x80c\xE9\x9B\xE3\x96\x14a\x04QW`\0\x80\xFD[\x80c\xABq\x90_\x14a\x03AW\x80c\xB6\xCB\x11\x18\x14a\x03uW\x80c\xC1Q\x06\x18\x14a\x03\xA9W`\0\x80\xFD[\x80c7@@\x17\x11a\0\xBBW\x80c7@@\x17\x14a\x01CW\x80cT\xFDMP\x14a\x02\x91W\x80c\xA0\x85\xFA0\x14a\x02\xC1W\x80c\xAA\x8C\xD6\xC4\x14a\x03\rW`\0\x80\xFD[\x80c\x04\xBA\xA0\x0B\x14a\0\xE2W\x80c\x06\xFD\xDE\x03\x14a\x01\rW\x80c\x16\xAB\xFCp\x14a\x01\"W[`\0\x80\xFD[4\x80\x15a\0\xEEW`\0\x80\xFD[Pa\0\xF7a\x04eV[`@Qa\x01\x04\x91\x90a\x19\xDFV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x19W`\0\x80\xFD[Pa\0\xF7a\x04\x81V[a\x015a\x0106`\x04a\x1B\x1EV[a\x05\x0FV[`@Q\x90\x81R` \x01a\x01\x04V[4\x80\x15a\x01OW`\0\x80\xFD[Pa\x02 a\x01^6`\x04a\x1B\xE3V[`@\x80Qa\x01\0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91RP`\0\x90\x81R`\x01` \x81\x81R`@\x92\x83\x90 \x83Qa\x01\0\x81\x01\x85R\x81T\x81R\x92\x81\x01T\x91\x83\x01\x91\x90\x91R`\x02\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16``\x83\x01R`\x04\x83\x01T\x81\x16`\x80\x83\x01R`\x05\x83\x01T\x81\x16`\xA0\x83\x01R`\x06\x83\x01T\x81\x16`\xC0\x83\x01R`\x07\x90\x92\x01T\x90\x91\x16`\xE0\x82\x01R\x90V[`@Qa\x01\x04\x91\x90\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x83\x01\x91\x90\x91R`\x80\x80\x84\x01Q\x82\x16\x90\x83\x01R`\xA0\x80\x84\x01Q\x82\x16\x90\x83\x01R`\xC0\x80\x84\x01Q\x82\x16\x90\x83\x01R`\xE0\x92\x83\x01Q\x16\x91\x81\x01\x91\x90\x91Ra\x01\0\x01\x90V[4\x80\x15a\x02\x9DW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x07\x81Rf\x1D\x8CK\x8C\x0B\x8CM`\xCA\x1B` \x82\x01Ra\0\xF7V[4\x80\x15a\x02\xCDW`\0\x80\xFD[Pa\x02\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x04V[4\x80\x15a\x03\x19W`\0\x80\xFD[Pa\x02\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03MW`\0\x80\xFD[Pa\x02\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\x81W`\0\x80\xFD[Pa\x02\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\xB5W`\0\x80\xFD[Pa\x02\xF5a\x03\xC46`\x04a\x1DWV[a\x06\xADV[4\x80\x15a\x03\xD5W`\0\x80\xFD[Pa\x02\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04\tW`\0\x80\xFD[Pa\x02\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04=W`\0\x80\xFD[Pa\x02\xF5a\x04L6`\x04a\x1D\xC5V[a\x0C\xBBV[4\x80\x15a\x04]W`\0\x80\xFD[P`\x04a\x015V[`@Q\x80``\x01`@R\x80`$\x81R` \x01a#\xB6`$\x919\x81V[`\0\x80Ta\x04\x8E\x90a\x1EfV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xBA\x90a\x1EfV[\x80\x15a\x05\x07W\x80`\x1F\x10a\x04\xDCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x07V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xEAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05ZW`@Qc@\x845\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05ba\x0F\xEAV[`\0\x86\x81R`\x01` R`@\x90 `\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x05\x9BW`@Qc\x95+\x05\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x05\xA9\x82\x88\x88\x87a\x10\x0BV[\x90P\x81`\x01`\x01`\xA0\x1B\x03\x16cw\xD0_\xF4\x82\x88\x88\x88`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xDC\x93\x92\x91\x90a\x1E\xA0V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x05\xFAW=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x1F\x91\x90a\x1E\xF0V[\x92P`\0a\x06-\x824a\x1F\x1FV[\x90P\x80\x15a\x06\xA0W`@Q`\0\x903\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x06wW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06|V[``\x91P[PP\x90P\x80a\x06\x9EW`@Qc\x12\x17\x1D\x83`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[PPP[\x95\x94PPPPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xF8W`@Qc@\x845\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x86\x81R`\x01` R`@\x81 \x90\x84\x90\x03a\x08\x8DW\x80T\x15a\x07.W`@Qc;\xE1\xB3M`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x077\x86a\x11\x1BV[`\0a\x07C\x87\x87a\x13/V[\x90P`\0\x87`@Q` \x01a\x07X\x91\x90a\x1F2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x87\x80Q\x90` \x01 \x90P`\0a\x07\x87\x8Aa\x13\xAFV[\x90P\x83\x81`\x80\x01\x81\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ck27\x07\x82\x8B\x8E\x8B`@Q` \x01a\x07\xE0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\x14\x93\x92\x91\x90a!hV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x083W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08W\x91\x90a!\x9CV[\x92\x85UP`\x01\x84\x01U`\x02\x83\x01\x91\x90\x91U`\x04\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U\x90Pa\x06\xA4V[`\0\x87\x81R`\x01` R`@\x90 T\x80a\x08\xBAW`@Qc9\x8B\x1C\t`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x87`@Q` \x01a\x08\xCC\x91\x90a\x1F2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\t\0W`@Qc3.\xE1\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x82\x01T\x86Q` \x88\x01 \x14a\t*W`@Qc\x1A2r\xD1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t3\x87a\x11\x1BV[`\0a\t>\x88a\x13\xAFV[`\x02\x84\x01T`\x80\x82\x01R\x90P`\x01\x86\x90\x03a\nhW`\x05\x83\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\t~W`@Qb\xE8\x96\xAF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R\x90\x81\x01\x8A\x90R``\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ck27\x07\x90\x83\x90\x8A\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x01\x93\x92\x91\x90a!hV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nD\x91\x90a!\x9CV[`\x05\x84\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U\x93Pa\x06\xA0V[\x85`\x02\x03a\x0B\x85W`\x06\x83\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\n\x9BW`@Qb\xE8\x96\xAF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R\x90\x81\x01\x8A\x90R``\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ck27\x07\x90\x83\x90\x8A\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x1E\x93\x92\x91\x90a!hV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ba\x91\x90a!\x9CV[`\x06\x84\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U\x93Pa\x06\xA0V[\x85`\x03\x03a\x0C\xA2W`\x07\x83\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0B\xB8W`@Qb\xE8\x96\xAF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R\x90\x81\x01\x8A\x90R``\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ck27\x07\x90\x83\x90\x8A\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C;\x93\x92\x91\x90a!hV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0CZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C~\x91\x90a!\x9CV[`\x07\x84\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U\x93Pa\x06\xA0V[`@Qc\x1D\x9F\x81Y`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\r\x06W`@Qc@\x845\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x86\x81R`\x01` \x81\x81R`@\x92\x83\x90 \x83Qa\x01\0\x81\x01\x85R\x81T\x81R\x92\x81\x01T\x91\x83\x01\x91\x90\x91R`\x02\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16``\x83\x01\x81\x90R`\x04\x84\x01T\x82\x16`\x80\x84\x01R`\x05\x84\x01T\x82\x16`\xA0\x84\x01R`\x06\x84\x01T\x82\x16`\xC0\x84\x01R`\x07\x90\x93\x01T\x16`\xE0\x82\x01R\x90\x15a\r\xA2W`@Qc,\x95\xCA\xEB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qa\r\xC1W`@Qc9\x8B\x1C\t`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a\r\xE6WP`\xA0\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15[\x80a\r\xFCWP`\xC0\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15[\x80a\x0E\x12WP`\xE0\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15[\x15a\x0E0W`@Qc\xE9|\xC2\xBF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`@Qa\x0EC\x90\x87\x90` \x01a\x1F2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\x0EwW`@Qc3.\xE1\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80` \x01Q\x84\x80Q\x90` \x01 \x14a\x0E\xA2W`@Qc\x1A2r\xD1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\xAB\x85a\x11\x1BV[`\0a\x0E\xB6\x86a\x13\xAFV[\x90P\x81`@\x01Q\x81`\x80\x01\x81\x81RPP`\0\x88\x90P`\0\x85\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cjF%y\x8B\x86\x8B\x89`\x80\x01Q\x8A`\xA0\x01Q\x8B`\xC0\x01Q\x8C`\xE0\x01Q\x8B\x8B`@Q` \x01a\x0F4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x89c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Fm\x98\x97\x96\x95\x94\x93\x92\x91\x90a!\xB9V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0F\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xB0\x91\x90a!\x9CV[`\0\x9B\x8CR`\x01` R`@\x90\x9B `\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x8D\x16\x17\x90UP\x98\x99\x98PPPPPPPPPV[4\x15a\x10\tW`@Qc\x15t\xF9\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0\x80\x82` \x01Q\x15a\x10\x81W\x85`\x01`\x01`\xA0\x1B\x03\x16c\xC5]\xAEc`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10VW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10z\x91\x90a!\x9CV[\x90Pa\x10\xE6V[\x85`\x01`\x01`\xA0\x1B\x03\x16c\nN\x14\x93`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xE3\x91\x90a!\x9CV[\x90P[a\x10\xFB`\x01`\x01`\xA0\x1B\x03\x82\x16\x860\x87a\x14}V[a\x11\x0F`\x01`\x01`\xA0\x1B\x03\x82\x16\x87\x86a\x14\xEAV[P`\0\x95\x94PPPPPV[a\x11$\x81a\x15zV[` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x11OW`@Qc\x07?s\x9D`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c8\xD5.\x0F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xB5\x91\x90a!\x9CV[`\x01`\x01`\xA0\x1B\x03\x16\x81`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\xEAW`@Qc\x07\"\x15%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x81`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12R\x91\x90a\"+V[a\x12\\\x91\x90a\"NV[a\x12g\x90`\na#KV[\x81`\x80\x01Q\x10\x15a\x12\x8BW`@QcI\xDBD\xF5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x81`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xF3\x91\x90a\"+V[a\x12\xFD\x91\x90a\"NV[a\x13\x08\x90`\na#KV[\x81`\xA0\x01Q\x10\x15a\x13,W`@Qc\x18\xC9R#`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[` \x82\x01Q`@Qc\x03\xD1h\x9D`\xE1\x1B\x81Rg\r\xE0\xB6\xB3\xA7d\0\0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x07\xA2\xD1:\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xA6\x91\x90a\x1E\xF0V[\x90P[\x92\x91PPV[a\x13\xB7a\x18\xA8V[\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x84\x01Q\x82\x16\x90\x83\x01R`@\x80\x84\x01Q\x82\x16\x90\x83\x01R``\x80\x84\x01Q\x90\x83\x01R`\x80\x83\x01Q`\xA0\x80\x84\x01\x91\x90\x91R\x83\x01Q`\xC0\x80\x84\x01\x91\x90\x91R\x83\x01Q`\xE0\x80\x84\x01\x91\x90\x91R\x83\x01Qa\x01\0\x80\x84\x01\x91\x90\x91R\x83\x01Qa\x01 \x80\x84\x01\x91\x90\x91R\x83\x01Qa\x01@\x80\x84\x01\x91\x90\x91R\x83\x01Q\x81\x16a\x01`\x80\x84\x01\x91\x90\x91R\x83\x01Q\x81\x16a\x01\x80\x80\x84\x01\x91\x90\x91R\x83\x01Q\x81\x16a\x01\xA0\x80\x84\x01\x91\x90\x91R\x83\x01Q\x16a\x01\xC0\x80\x83\x01\x91\x90\x91R\x90\x91\x01Qa\x01\xE0\x82\x01R\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R\x83\x81\x16`D\x83\x01R`d\x82\x01\x83\x90Ra\x14\xE4\x91\x86\x91\x82\x16\x90c#\xB8r\xDD\x90`\x84\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPPa\x16\x8FV[PPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x15;\x84\x82a\x16\xFCV[a\x14\xE4W`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`\0`D\x83\x01Ra\x15p\x91\x86\x91\x82\x16\x90c\t^\xA7\xB3\x90`d\x01a\x14\xB2V[a\x14\xE4\x84\x82a\x16\x8FV[a\x03\xE8\x81`\x80\x01Q\x10\x15a\x15\xA1W`@QcI\xDBD\xF5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x01\0\x01Q`\0\x03a\x15\xC7W`@QcT(sM`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x01\0\x01Q\x81`\xE0\x01Q\x10\x80a\x15\xF1WP\x80a\x01\0\x01Q\x81`\xE0\x01Qa\x15\xEE\x91\x90a#ZV[\x15\x15[\x15a\x16\x0FW`@Qc%?\xFF\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\xC0\x81\x01QQg\r\xE0\xB6\xB3\xA7d\0\0\x10\x80a\x16;WPg\r\xE0\xB6\xB3\xA7d\0\0\x81a\x01\xC0\x01Q` \x01Q\x11[\x80a\x16VWPg\r\xE0\xB6\xB3\xA7d\0\0\x81a\x01\xC0\x01Q`@\x01Q\x11[\x80a\x16qWPg\r\xE0\xB6\xB3\xA7d\0\0\x81a\x01\xC0\x01Q``\x01Q\x11[\x15a\x13,W`@Qc\"\xF7,\xC3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x16\xA4`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x17\x9FV[\x90P\x80Q`\0\x14\x15\x80\x15a\x16\xC9WP\x80\x80` \x01\x90Q\x81\x01\x90a\x16\xC7\x91\x90a#|V[\x15[\x15a\x16\xF7W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[PPPV[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x17\x19\x91\x90a#\x99V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x17VW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x17[V[``\x91P[P\x91P\x91P\x81\x80\x15a\x17\x85WP\x80Q\x15\x80a\x17\x85WP\x80\x80` \x01\x90Q\x81\x01\x90a\x17\x85\x91\x90a#|V[\x80\x15a\x06\xA4WPPPPP`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[``a\x13\xA6\x83\x83`\0\x84`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x17\xC5\x91\x90a#\x99V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x18\x02W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x18\x07V[``\x91P[P\x91P\x91Pa\x18\x17\x86\x83\x83a\x18#V[\x92PPP[\x93\x92PPPV[``\x82a\x188Wa\x183\x82a\x18\x7FV[a\x18\x1CV[\x81Q\x15\x80\x15a\x18OWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x18xW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x16\xEEV[P\x80a\x18\x1CV[\x80Q\x15a\x18\x8FW\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80a\x02\0\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x80\x19\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x19\x8A`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x90R\x90V[`\0[\x83\x81\x10\x15a\x19\xAAW\x81\x81\x01Q\x83\x82\x01R` \x01a\x19\x92V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x19\xCB\x81` \x86\x01` \x86\x01a\x19\x8FV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x13\xA6` \x83\x01\x84a\x19\xB3V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13,W`\0\x80\xFD[\x805a\x1A\x12\x81a\x19\xF2V[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1APWa\x1APa\x1A\x17V[`@R\x90V[`@Qa\x01\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1APWa\x1APa\x1A\x17V[\x80\x15\x15\x81\x14a\x13,W`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x1A\xA3Wa\x1A\xA3a\x1A\x17V[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x1A\xCBWa\x1A\xCBa\x1A\x17V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x1A\xE4W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x1B\x0FW`\0\x80\xFD[a\x13\xA6\x83\x835` \x85\x01a\x1A\x88V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1B6W`\0\x80\xFD[\x855\x94P` \x86\x015a\x1BH\x81a\x19\xF2V[\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1BsW`\0\x80\xFD[\x90\x87\x01\x90``\x82\x8A\x03\x12\x15a\x1B\x87W`\0\x80\xFD[a\x1B\x8Fa\x1A-V[\x825a\x1B\x9A\x81a\x19\xF2V[\x81R` \x83\x015a\x1B\xAA\x81a\x1AzV[` \x82\x01R`@\x83\x015\x82\x81\x11\x15a\x1B\xC1W`\0\x80\xFD[a\x1B\xCD\x8B\x82\x86\x01a\x1A\xFEV[`@\x83\x01RP\x80\x93PPPP\x92\x95P\x92\x95\x90\x93PV[`\0` \x82\x84\x03\x12\x15a\x1B\xF5W`\0\x80\xFD[P5\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a\x1C\x0EW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1C1Wa\x1C1a\x1A\x17V[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[`\0a\x02@\x82\x84\x03\x12\x15a\x1CuW`\0\x80\xFD[a\x1C}a\x1AVV[\x90Pa\x1C\x88\x82a\x1A\x07V[\x81Ra\x1C\x96` \x83\x01a\x1A\x07V[` \x82\x01Ra\x1C\xA7`@\x83\x01a\x1A\x07V[`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015`\x80\x82\x01R`\xA0\x82\x015`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R`\xE0\x82\x015`\xE0\x82\x01Ra\x01\0\x80\x83\x015\x81\x83\x01RPa\x01 \x80\x83\x015\x81\x83\x01RPa\x01@a\x1D\x04\x81\x84\x01a\x1A\x07V[\x90\x82\x01Ra\x01`a\x1D\x16\x83\x82\x01a\x1A\x07V[\x90\x82\x01Ra\x01\x80a\x1D(\x83\x82\x01a\x1A\x07V[\x90\x82\x01Ra\x01\xA0a\x1D:\x83\x82\x01a\x1A\x07V[\x90\x82\x01Ra\x01\xC0a\x1DM\x84\x84\x83\x01a\x1B\xFCV[\x90\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0a\x02\xC0\x86\x88\x03\x12\x15a\x1DpW`\0\x80\xFD[\x855\x94Pa\x1D\x81\x87` \x88\x01a\x1CbV[\x93Pa\x02`\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\x9EW`\0\x80\xFD[a\x1D\xAA\x88\x82\x89\x01a\x1A\xFEV[\x95\x98\x94\x97P\x94\x95a\x02\x80\x81\x015\x95Pa\x02\xA0\x015\x93\x92PPPV[`\0\x80`\0\x80`\0a\x02\xC0\x86\x88\x03\x12\x15a\x1D\xDEW`\0\x80\xFD[\x855\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1D\xFDW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1E\x11W`\0\x80\xFD[a\x1E \x89\x835` \x85\x01a\x1A\x88V[\x95Pa\x1E/\x89`@\x8A\x01a\x1CbV[\x94Pa\x02\x80\x88\x015\x91P\x80\x82\x11\x15a\x1EFW`\0\x80\xFD[Pa\x1ES\x88\x82\x89\x01a\x1A\xFEV[\x95\x98\x94\x97P\x92\x95a\x02\xA0\x015\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1EzW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1E\x9AWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x83\x81R\x82` \x82\x01R```@\x82\x01R`\x01\x80`\xA0\x1B\x03\x82Q\x16``\x82\x01R` \x82\x01Q\x15\x15`\x80\x82\x01R`\0`@\x83\x01Q```\xA0\x84\x01Ra\x1E\xE6`\xC0\x84\x01\x82a\x19\xB3V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1F\x02W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x13\xA9Wa\x13\xA9a\x1F\tV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81Ra\x02@\x81\x01` \x83\x01Qa\x1F^` \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x83\x01Qa\x1Fy`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Q`\xE0\x83\x01Ra\x01\0\x80\x84\x01Q\x81\x84\x01RPa\x01 \x80\x84\x01Q\x81\x84\x01RPa\x01@\x80\x84\x01Qa\x1F\xDF\x82\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[PPa\x01`\x83\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x84\x01\x91\x90\x91Ra\x01\x80\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\xA0\x80\x85\x01Q\x90\x91\x16\x90\x83\x01Ra\x01\xC0\x92\x83\x01Q\x80Q\x93\x83\x01\x93\x90\x93R` \x83\x01Qa\x01\xE0\x83\x01R`@\x83\x01Qa\x02\0\x83\x01R``\x90\x92\x01Qa\x02 \x90\x91\x01R\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x01Qa r` \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x81\x01Qa \x8D`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x81\x01Q``\x83\x01R`\x80\x81\x01Q`\x80\x83\x01R`\xA0\x81\x01Q`\xA0\x83\x01R`\xC0\x81\x01Q`\xC0\x83\x01R`\xE0\x81\x01Q`\xE0\x83\x01Ra\x01\0\x80\x82\x01Q\x81\x84\x01RPa\x01 \x80\x82\x01Q\x81\x84\x01RPa\x01@\x80\x82\x01Q\x81\x84\x01RPa\x01`\x80\x82\x01Qa \xFF\x82\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[PPa\x01\x80\x81\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x84\x01\x91\x90\x91Ra\x01\xA0\x80\x83\x01Q\x82\x16\x90\x84\x01Ra\x01\xC0\x80\x83\x01Q\x90\x91\x16\x90\x83\x01Ra\x01\xE0\x80\x82\x01Q\x80Q\x82\x85\x01R` \x81\x01Qa\x02\0\x85\x01R`@\x81\x01Qa\x02 \x85\x01R``\x81\x01Qa\x02@\x85\x01Ra\x14\xE4V[`\0a\x02\xA0a!w\x83\x87a KV[\x80a\x02`\x84\x01Ra!\x8A\x81\x84\x01\x86a\x19\xB3V[\x91PP\x82a\x02\x80\x83\x01R\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a!\xAEW`\0\x80\xFD[\x81Qa\x18\x1C\x81a\x19\xF2V[`\0a\x03@\x80\x83Ra!\xCD\x81\x84\x01\x8Ca\x19\xB3V[\x90Pa!\xDC` \x84\x01\x8Ba KV[\x82\x81\x03a\x02\x80\x84\x01Ra!\xEF\x81\x8Aa\x19\xB3V[`\x01`\x01`\xA0\x1B\x03\x98\x89\x16a\x02\xA0\x85\x01R\x96\x88\x16a\x02\xC0\x84\x01RPP\x92\x85\x16a\x02\xE0\x84\x01R\x93\x16a\x03\0\x82\x01Ra\x03 \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\"=W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x18\x1CW`\0\x80\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x13\xA9Wa\x13\xA9a\x1F\tV[`\x01\x81\x81[\x80\x85\x11\x15a\"\xA2W\x81`\0\x19\x04\x82\x11\x15a\"\x88Wa\"\x88a\x1F\tV[\x80\x85\x16\x15a\"\x95W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\"lV[P\x92P\x92\x90PV[`\0\x82a\"\xB9WP`\x01a\x13\xA9V[\x81a\"\xC6WP`\0a\x13\xA9V[\x81`\x01\x81\x14a\"\xDCW`\x02\x81\x14a\"\xE6Wa#\x02V[`\x01\x91PPa\x13\xA9V[`\xFF\x84\x11\x15a\"\xF7Wa\"\xF7a\x1F\tV[PP`\x01\x82\x1Ba\x13\xA9V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a#%WP\x81\x81\na\x13\xA9V[a#/\x83\x83a\"gV[\x80`\0\x19\x04\x82\x11\x15a#CWa#Ca\x1F\tV[\x02\x93\x92PPPV[`\0a\x13\xA6`\xFF\x84\x16\x83a\"\xAAV[`\0\x82a#wWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0` \x82\x84\x03\x12\x15a#\x8EW`\0\x80\xFD[\x81Qa\x18\x1C\x81a\x1AzV[`\0\x82Qa#\xAB\x81\x84` \x87\x01a\x19\x8FV[\x91\x90\x91\x01\x92\x91PPV\xFEERC4626HyperdriveDeployerCoordinator\xA2dipfsX\"\x12 \xC4f\x0B\xB8|\xC12\xA5\xC4\x91\xA2\xD0\x85\x0Fn\xB9\x17r\xE3\xA0\x06\x130N\x1C\xD6{k_\xFF1\x88dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static ERC4626HYPERDRIVEDEPLOYERCOORDINATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xDDW`\x005`\xE0\x1C\x80c\xABq\x90_\x11a\0\x7FW\x80c\xC4Z\x01U\x11a\0YW\x80c\xC4Z\x01U\x14a\x03\xC9W\x80c\xC8>\x1FQ\x14a\x03\xFDW\x80c\xE9\x90\xEB\xA8\x14a\x041W\x80c\xE9\x9B\xE3\x96\x14a\x04QW`\0\x80\xFD[\x80c\xABq\x90_\x14a\x03AW\x80c\xB6\xCB\x11\x18\x14a\x03uW\x80c\xC1Q\x06\x18\x14a\x03\xA9W`\0\x80\xFD[\x80c7@@\x17\x11a\0\xBBW\x80c7@@\x17\x14a\x01CW\x80cT\xFDMP\x14a\x02\x91W\x80c\xA0\x85\xFA0\x14a\x02\xC1W\x80c\xAA\x8C\xD6\xC4\x14a\x03\rW`\0\x80\xFD[\x80c\x04\xBA\xA0\x0B\x14a\0\xE2W\x80c\x06\xFD\xDE\x03\x14a\x01\rW\x80c\x16\xAB\xFCp\x14a\x01\"W[`\0\x80\xFD[4\x80\x15a\0\xEEW`\0\x80\xFD[Pa\0\xF7a\x04eV[`@Qa\x01\x04\x91\x90a\x19\xDFV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x19W`\0\x80\xFD[Pa\0\xF7a\x04\x81V[a\x015a\x0106`\x04a\x1B\x1EV[a\x05\x0FV[`@Q\x90\x81R` \x01a\x01\x04V[4\x80\x15a\x01OW`\0\x80\xFD[Pa\x02 a\x01^6`\x04a\x1B\xE3V[`@\x80Qa\x01\0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91RP`\0\x90\x81R`\x01` \x81\x81R`@\x92\x83\x90 \x83Qa\x01\0\x81\x01\x85R\x81T\x81R\x92\x81\x01T\x91\x83\x01\x91\x90\x91R`\x02\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16``\x83\x01R`\x04\x83\x01T\x81\x16`\x80\x83\x01R`\x05\x83\x01T\x81\x16`\xA0\x83\x01R`\x06\x83\x01T\x81\x16`\xC0\x83\x01R`\x07\x90\x92\x01T\x90\x91\x16`\xE0\x82\x01R\x90V[`@Qa\x01\x04\x91\x90\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x83\x01\x91\x90\x91R`\x80\x80\x84\x01Q\x82\x16\x90\x83\x01R`\xA0\x80\x84\x01Q\x82\x16\x90\x83\x01R`\xC0\x80\x84\x01Q\x82\x16\x90\x83\x01R`\xE0\x92\x83\x01Q\x16\x91\x81\x01\x91\x90\x91Ra\x01\0\x01\x90V[4\x80\x15a\x02\x9DW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x07\x81Rf\x1D\x8CK\x8C\x0B\x8CM`\xCA\x1B` \x82\x01Ra\0\xF7V[4\x80\x15a\x02\xCDW`\0\x80\xFD[Pa\x02\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x04V[4\x80\x15a\x03\x19W`\0\x80\xFD[Pa\x02\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03MW`\0\x80\xFD[Pa\x02\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\x81W`\0\x80\xFD[Pa\x02\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\xB5W`\0\x80\xFD[Pa\x02\xF5a\x03\xC46`\x04a\x1DWV[a\x06\xADV[4\x80\x15a\x03\xD5W`\0\x80\xFD[Pa\x02\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04\tW`\0\x80\xFD[Pa\x02\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04=W`\0\x80\xFD[Pa\x02\xF5a\x04L6`\x04a\x1D\xC5V[a\x0C\xBBV[4\x80\x15a\x04]W`\0\x80\xFD[P`\x04a\x015V[`@Q\x80``\x01`@R\x80`$\x81R` \x01a#\xB6`$\x919\x81V[`\0\x80Ta\x04\x8E\x90a\x1EfV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xBA\x90a\x1EfV[\x80\x15a\x05\x07W\x80`\x1F\x10a\x04\xDCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x07V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xEAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05ZW`@Qc@\x845\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05ba\x0F\xEAV[`\0\x86\x81R`\x01` R`@\x90 `\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x05\x9BW`@Qc\x95+\x05\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x05\xA9\x82\x88\x88\x87a\x10\x0BV[\x90P\x81`\x01`\x01`\xA0\x1B\x03\x16cw\xD0_\xF4\x82\x88\x88\x88`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xDC\x93\x92\x91\x90a\x1E\xA0V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x05\xFAW=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x1F\x91\x90a\x1E\xF0V[\x92P`\0a\x06-\x824a\x1F\x1FV[\x90P\x80\x15a\x06\xA0W`@Q`\0\x903\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x06wW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06|V[``\x91P[PP\x90P\x80a\x06\x9EW`@Qc\x12\x17\x1D\x83`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[PPP[\x95\x94PPPPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xF8W`@Qc@\x845\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x86\x81R`\x01` R`@\x81 \x90\x84\x90\x03a\x08\x8DW\x80T\x15a\x07.W`@Qc;\xE1\xB3M`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x077\x86a\x11\x1BV[`\0a\x07C\x87\x87a\x13/V[\x90P`\0\x87`@Q` \x01a\x07X\x91\x90a\x1F2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x87\x80Q\x90` \x01 \x90P`\0a\x07\x87\x8Aa\x13\xAFV[\x90P\x83\x81`\x80\x01\x81\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ck27\x07\x82\x8B\x8E\x8B`@Q` \x01a\x07\xE0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\x14\x93\x92\x91\x90a!hV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x083W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08W\x91\x90a!\x9CV[\x92\x85UP`\x01\x84\x01U`\x02\x83\x01\x91\x90\x91U`\x04\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U\x90Pa\x06\xA4V[`\0\x87\x81R`\x01` R`@\x90 T\x80a\x08\xBAW`@Qc9\x8B\x1C\t`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x87`@Q` \x01a\x08\xCC\x91\x90a\x1F2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\t\0W`@Qc3.\xE1\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x82\x01T\x86Q` \x88\x01 \x14a\t*W`@Qc\x1A2r\xD1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t3\x87a\x11\x1BV[`\0a\t>\x88a\x13\xAFV[`\x02\x84\x01T`\x80\x82\x01R\x90P`\x01\x86\x90\x03a\nhW`\x05\x83\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\t~W`@Qb\xE8\x96\xAF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R\x90\x81\x01\x8A\x90R``\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ck27\x07\x90\x83\x90\x8A\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x01\x93\x92\x91\x90a!hV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nD\x91\x90a!\x9CV[`\x05\x84\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U\x93Pa\x06\xA0V[\x85`\x02\x03a\x0B\x85W`\x06\x83\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\n\x9BW`@Qb\xE8\x96\xAF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R\x90\x81\x01\x8A\x90R``\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ck27\x07\x90\x83\x90\x8A\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x1E\x93\x92\x91\x90a!hV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ba\x91\x90a!\x9CV[`\x06\x84\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U\x93Pa\x06\xA0V[\x85`\x03\x03a\x0C\xA2W`\x07\x83\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0B\xB8W`@Qb\xE8\x96\xAF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R\x90\x81\x01\x8A\x90R``\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ck27\x07\x90\x83\x90\x8A\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C;\x93\x92\x91\x90a!hV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0CZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C~\x91\x90a!\x9CV[`\x07\x84\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U\x93Pa\x06\xA0V[`@Qc\x1D\x9F\x81Y`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\r\x06W`@Qc@\x845\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x86\x81R`\x01` \x81\x81R`@\x92\x83\x90 \x83Qa\x01\0\x81\x01\x85R\x81T\x81R\x92\x81\x01T\x91\x83\x01\x91\x90\x91R`\x02\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16``\x83\x01\x81\x90R`\x04\x84\x01T\x82\x16`\x80\x84\x01R`\x05\x84\x01T\x82\x16`\xA0\x84\x01R`\x06\x84\x01T\x82\x16`\xC0\x84\x01R`\x07\x90\x93\x01T\x16`\xE0\x82\x01R\x90\x15a\r\xA2W`@Qc,\x95\xCA\xEB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qa\r\xC1W`@Qc9\x8B\x1C\t`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a\r\xE6WP`\xA0\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15[\x80a\r\xFCWP`\xC0\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15[\x80a\x0E\x12WP`\xE0\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15[\x15a\x0E0W`@Qc\xE9|\xC2\xBF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`@Qa\x0EC\x90\x87\x90` \x01a\x1F2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\x0EwW`@Qc3.\xE1\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80` \x01Q\x84\x80Q\x90` \x01 \x14a\x0E\xA2W`@Qc\x1A2r\xD1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\xAB\x85a\x11\x1BV[`\0a\x0E\xB6\x86a\x13\xAFV[\x90P\x81`@\x01Q\x81`\x80\x01\x81\x81RPP`\0\x88\x90P`\0\x85\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cjF%y\x8B\x86\x8B\x89`\x80\x01Q\x8A`\xA0\x01Q\x8B`\xC0\x01Q\x8C`\xE0\x01Q\x8B\x8B`@Q` \x01a\x0F4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x89c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Fm\x98\x97\x96\x95\x94\x93\x92\x91\x90a!\xB9V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0F\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xB0\x91\x90a!\x9CV[`\0\x9B\x8CR`\x01` R`@\x90\x9B `\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x8D\x16\x17\x90UP\x98\x99\x98PPPPPPPPPV[4\x15a\x10\tW`@Qc\x15t\xF9\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0\x80\x82` \x01Q\x15a\x10\x81W\x85`\x01`\x01`\xA0\x1B\x03\x16c\xC5]\xAEc`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10VW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10z\x91\x90a!\x9CV[\x90Pa\x10\xE6V[\x85`\x01`\x01`\xA0\x1B\x03\x16c\nN\x14\x93`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xE3\x91\x90a!\x9CV[\x90P[a\x10\xFB`\x01`\x01`\xA0\x1B\x03\x82\x16\x860\x87a\x14}V[a\x11\x0F`\x01`\x01`\xA0\x1B\x03\x82\x16\x87\x86a\x14\xEAV[P`\0\x95\x94PPPPPV[a\x11$\x81a\x15zV[` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x11OW`@Qc\x07?s\x9D`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c8\xD5.\x0F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xB5\x91\x90a!\x9CV[`\x01`\x01`\xA0\x1B\x03\x16\x81`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\xEAW`@Qc\x07\"\x15%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x81`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12R\x91\x90a\"+V[a\x12\\\x91\x90a\"NV[a\x12g\x90`\na#KV[\x81`\x80\x01Q\x10\x15a\x12\x8BW`@QcI\xDBD\xF5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x81`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xF3\x91\x90a\"+V[a\x12\xFD\x91\x90a\"NV[a\x13\x08\x90`\na#KV[\x81`\xA0\x01Q\x10\x15a\x13,W`@Qc\x18\xC9R#`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[` \x82\x01Q`@Qc\x03\xD1h\x9D`\xE1\x1B\x81Rg\r\xE0\xB6\xB3\xA7d\0\0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x07\xA2\xD1:\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xA6\x91\x90a\x1E\xF0V[\x90P[\x92\x91PPV[a\x13\xB7a\x18\xA8V[\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x84\x01Q\x82\x16\x90\x83\x01R`@\x80\x84\x01Q\x82\x16\x90\x83\x01R``\x80\x84\x01Q\x90\x83\x01R`\x80\x83\x01Q`\xA0\x80\x84\x01\x91\x90\x91R\x83\x01Q`\xC0\x80\x84\x01\x91\x90\x91R\x83\x01Q`\xE0\x80\x84\x01\x91\x90\x91R\x83\x01Qa\x01\0\x80\x84\x01\x91\x90\x91R\x83\x01Qa\x01 \x80\x84\x01\x91\x90\x91R\x83\x01Qa\x01@\x80\x84\x01\x91\x90\x91R\x83\x01Q\x81\x16a\x01`\x80\x84\x01\x91\x90\x91R\x83\x01Q\x81\x16a\x01\x80\x80\x84\x01\x91\x90\x91R\x83\x01Q\x81\x16a\x01\xA0\x80\x84\x01\x91\x90\x91R\x83\x01Q\x16a\x01\xC0\x80\x83\x01\x91\x90\x91R\x90\x91\x01Qa\x01\xE0\x82\x01R\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R\x83\x81\x16`D\x83\x01R`d\x82\x01\x83\x90Ra\x14\xE4\x91\x86\x91\x82\x16\x90c#\xB8r\xDD\x90`\x84\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPPa\x16\x8FV[PPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x15;\x84\x82a\x16\xFCV[a\x14\xE4W`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`\0`D\x83\x01Ra\x15p\x91\x86\x91\x82\x16\x90c\t^\xA7\xB3\x90`d\x01a\x14\xB2V[a\x14\xE4\x84\x82a\x16\x8FV[a\x03\xE8\x81`\x80\x01Q\x10\x15a\x15\xA1W`@QcI\xDBD\xF5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x01\0\x01Q`\0\x03a\x15\xC7W`@QcT(sM`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x01\0\x01Q\x81`\xE0\x01Q\x10\x80a\x15\xF1WP\x80a\x01\0\x01Q\x81`\xE0\x01Qa\x15\xEE\x91\x90a#ZV[\x15\x15[\x15a\x16\x0FW`@Qc%?\xFF\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\xC0\x81\x01QQg\r\xE0\xB6\xB3\xA7d\0\0\x10\x80a\x16;WPg\r\xE0\xB6\xB3\xA7d\0\0\x81a\x01\xC0\x01Q` \x01Q\x11[\x80a\x16VWPg\r\xE0\xB6\xB3\xA7d\0\0\x81a\x01\xC0\x01Q`@\x01Q\x11[\x80a\x16qWPg\r\xE0\xB6\xB3\xA7d\0\0\x81a\x01\xC0\x01Q``\x01Q\x11[\x15a\x13,W`@Qc\"\xF7,\xC3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x16\xA4`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x17\x9FV[\x90P\x80Q`\0\x14\x15\x80\x15a\x16\xC9WP\x80\x80` \x01\x90Q\x81\x01\x90a\x16\xC7\x91\x90a#|V[\x15[\x15a\x16\xF7W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[PPPV[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x17\x19\x91\x90a#\x99V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x17VW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x17[V[``\x91P[P\x91P\x91P\x81\x80\x15a\x17\x85WP\x80Q\x15\x80a\x17\x85WP\x80\x80` \x01\x90Q\x81\x01\x90a\x17\x85\x91\x90a#|V[\x80\x15a\x06\xA4WPPPPP`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[``a\x13\xA6\x83\x83`\0\x84`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x17\xC5\x91\x90a#\x99V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x18\x02W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x18\x07V[``\x91P[P\x91P\x91Pa\x18\x17\x86\x83\x83a\x18#V[\x92PPP[\x93\x92PPPV[``\x82a\x188Wa\x183\x82a\x18\x7FV[a\x18\x1CV[\x81Q\x15\x80\x15a\x18OWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x18xW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x16\xEEV[P\x80a\x18\x1CV[\x80Q\x15a\x18\x8FW\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80a\x02\0\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x80\x19\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x19\x8A`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x90R\x90V[`\0[\x83\x81\x10\x15a\x19\xAAW\x81\x81\x01Q\x83\x82\x01R` \x01a\x19\x92V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x19\xCB\x81` \x86\x01` \x86\x01a\x19\x8FV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x13\xA6` \x83\x01\x84a\x19\xB3V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13,W`\0\x80\xFD[\x805a\x1A\x12\x81a\x19\xF2V[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1APWa\x1APa\x1A\x17V[`@R\x90V[`@Qa\x01\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1APWa\x1APa\x1A\x17V[\x80\x15\x15\x81\x14a\x13,W`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x1A\xA3Wa\x1A\xA3a\x1A\x17V[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x1A\xCBWa\x1A\xCBa\x1A\x17V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x1A\xE4W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x1B\x0FW`\0\x80\xFD[a\x13\xA6\x83\x835` \x85\x01a\x1A\x88V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1B6W`\0\x80\xFD[\x855\x94P` \x86\x015a\x1BH\x81a\x19\xF2V[\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1BsW`\0\x80\xFD[\x90\x87\x01\x90``\x82\x8A\x03\x12\x15a\x1B\x87W`\0\x80\xFD[a\x1B\x8Fa\x1A-V[\x825a\x1B\x9A\x81a\x19\xF2V[\x81R` \x83\x015a\x1B\xAA\x81a\x1AzV[` \x82\x01R`@\x83\x015\x82\x81\x11\x15a\x1B\xC1W`\0\x80\xFD[a\x1B\xCD\x8B\x82\x86\x01a\x1A\xFEV[`@\x83\x01RP\x80\x93PPPP\x92\x95P\x92\x95\x90\x93PV[`\0` \x82\x84\x03\x12\x15a\x1B\xF5W`\0\x80\xFD[P5\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a\x1C\x0EW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1C1Wa\x1C1a\x1A\x17V[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[`\0a\x02@\x82\x84\x03\x12\x15a\x1CuW`\0\x80\xFD[a\x1C}a\x1AVV[\x90Pa\x1C\x88\x82a\x1A\x07V[\x81Ra\x1C\x96` \x83\x01a\x1A\x07V[` \x82\x01Ra\x1C\xA7`@\x83\x01a\x1A\x07V[`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015`\x80\x82\x01R`\xA0\x82\x015`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R`\xE0\x82\x015`\xE0\x82\x01Ra\x01\0\x80\x83\x015\x81\x83\x01RPa\x01 \x80\x83\x015\x81\x83\x01RPa\x01@a\x1D\x04\x81\x84\x01a\x1A\x07V[\x90\x82\x01Ra\x01`a\x1D\x16\x83\x82\x01a\x1A\x07V[\x90\x82\x01Ra\x01\x80a\x1D(\x83\x82\x01a\x1A\x07V[\x90\x82\x01Ra\x01\xA0a\x1D:\x83\x82\x01a\x1A\x07V[\x90\x82\x01Ra\x01\xC0a\x1DM\x84\x84\x83\x01a\x1B\xFCV[\x90\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0a\x02\xC0\x86\x88\x03\x12\x15a\x1DpW`\0\x80\xFD[\x855\x94Pa\x1D\x81\x87` \x88\x01a\x1CbV[\x93Pa\x02`\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\x9EW`\0\x80\xFD[a\x1D\xAA\x88\x82\x89\x01a\x1A\xFEV[\x95\x98\x94\x97P\x94\x95a\x02\x80\x81\x015\x95Pa\x02\xA0\x015\x93\x92PPPV[`\0\x80`\0\x80`\0a\x02\xC0\x86\x88\x03\x12\x15a\x1D\xDEW`\0\x80\xFD[\x855\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1D\xFDW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1E\x11W`\0\x80\xFD[a\x1E \x89\x835` \x85\x01a\x1A\x88V[\x95Pa\x1E/\x89`@\x8A\x01a\x1CbV[\x94Pa\x02\x80\x88\x015\x91P\x80\x82\x11\x15a\x1EFW`\0\x80\xFD[Pa\x1ES\x88\x82\x89\x01a\x1A\xFEV[\x95\x98\x94\x97P\x92\x95a\x02\xA0\x015\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1EzW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1E\x9AWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x83\x81R\x82` \x82\x01R```@\x82\x01R`\x01\x80`\xA0\x1B\x03\x82Q\x16``\x82\x01R` \x82\x01Q\x15\x15`\x80\x82\x01R`\0`@\x83\x01Q```\xA0\x84\x01Ra\x1E\xE6`\xC0\x84\x01\x82a\x19\xB3V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1F\x02W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x13\xA9Wa\x13\xA9a\x1F\tV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81Ra\x02@\x81\x01` \x83\x01Qa\x1F^` \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x83\x01Qa\x1Fy`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Q`\xE0\x83\x01Ra\x01\0\x80\x84\x01Q\x81\x84\x01RPa\x01 \x80\x84\x01Q\x81\x84\x01RPa\x01@\x80\x84\x01Qa\x1F\xDF\x82\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[PPa\x01`\x83\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x84\x01\x91\x90\x91Ra\x01\x80\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\xA0\x80\x85\x01Q\x90\x91\x16\x90\x83\x01Ra\x01\xC0\x92\x83\x01Q\x80Q\x93\x83\x01\x93\x90\x93R` \x83\x01Qa\x01\xE0\x83\x01R`@\x83\x01Qa\x02\0\x83\x01R``\x90\x92\x01Qa\x02 \x90\x91\x01R\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x01Qa r` \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x81\x01Qa \x8D`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x81\x01Q``\x83\x01R`\x80\x81\x01Q`\x80\x83\x01R`\xA0\x81\x01Q`\xA0\x83\x01R`\xC0\x81\x01Q`\xC0\x83\x01R`\xE0\x81\x01Q`\xE0\x83\x01Ra\x01\0\x80\x82\x01Q\x81\x84\x01RPa\x01 \x80\x82\x01Q\x81\x84\x01RPa\x01@\x80\x82\x01Q\x81\x84\x01RPa\x01`\x80\x82\x01Qa \xFF\x82\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[PPa\x01\x80\x81\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x84\x01\x91\x90\x91Ra\x01\xA0\x80\x83\x01Q\x82\x16\x90\x84\x01Ra\x01\xC0\x80\x83\x01Q\x90\x91\x16\x90\x83\x01Ra\x01\xE0\x80\x82\x01Q\x80Q\x82\x85\x01R` \x81\x01Qa\x02\0\x85\x01R`@\x81\x01Qa\x02 \x85\x01R``\x81\x01Qa\x02@\x85\x01Ra\x14\xE4V[`\0a\x02\xA0a!w\x83\x87a KV[\x80a\x02`\x84\x01Ra!\x8A\x81\x84\x01\x86a\x19\xB3V[\x91PP\x82a\x02\x80\x83\x01R\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a!\xAEW`\0\x80\xFD[\x81Qa\x18\x1C\x81a\x19\xF2V[`\0a\x03@\x80\x83Ra!\xCD\x81\x84\x01\x8Ca\x19\xB3V[\x90Pa!\xDC` \x84\x01\x8Ba KV[\x82\x81\x03a\x02\x80\x84\x01Ra!\xEF\x81\x8Aa\x19\xB3V[`\x01`\x01`\xA0\x1B\x03\x98\x89\x16a\x02\xA0\x85\x01R\x96\x88\x16a\x02\xC0\x84\x01RPP\x92\x85\x16a\x02\xE0\x84\x01R\x93\x16a\x03\0\x82\x01Ra\x03 \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\"=W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x18\x1CW`\0\x80\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x13\xA9Wa\x13\xA9a\x1F\tV[`\x01\x81\x81[\x80\x85\x11\x15a\"\xA2W\x81`\0\x19\x04\x82\x11\x15a\"\x88Wa\"\x88a\x1F\tV[\x80\x85\x16\x15a\"\x95W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\"lV[P\x92P\x92\x90PV[`\0\x82a\"\xB9WP`\x01a\x13\xA9V[\x81a\"\xC6WP`\0a\x13\xA9V[\x81`\x01\x81\x14a\"\xDCW`\x02\x81\x14a\"\xE6Wa#\x02V[`\x01\x91PPa\x13\xA9V[`\xFF\x84\x11\x15a\"\xF7Wa\"\xF7a\x1F\tV[PP`\x01\x82\x1Ba\x13\xA9V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a#%WP\x81\x81\na\x13\xA9V[a#/\x83\x83a\"gV[\x80`\0\x19\x04\x82\x11\x15a#CWa#Ca\x1F\tV[\x02\x93\x92PPPV[`\0a\x13\xA6`\xFF\x84\x16\x83a\"\xAAV[`\0\x82a#wWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0` \x82\x84\x03\x12\x15a#\x8EW`\0\x80\xFD[\x81Qa\x18\x1C\x81a\x1AzV[`\0\x82Qa#\xAB\x81\x84` \x87\x01a\x19\x8FV[\x91\x90\x91\x01\x92\x91PPV\xFEERC4626HyperdriveDeployerCoordinator\xA2dipfsX\"\x12 \xC4f\x0B\xB8|\xC12\xA5\xC4\x91\xA2\xD0\x85\x0Fn\xB9\x17r\xE3\xA0\x06\x130N\x1C\xD6{k_\xFF1\x88dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static ERC4626HYPERDRIVEDEPLOYERCOORDINATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ERC4626HyperdriveDeployerCoordinator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC4626HyperdriveDeployerCoordinator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC4626HyperdriveDeployerCoordinator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC4626HyperdriveDeployerCoordinator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC4626HyperdriveDeployerCoordinator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ERC4626HyperdriveDeployerCoordinator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC4626HyperdriveDeployerCoordinator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC4626HYPERDRIVEDEPLOYERCOORDINATOR_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                ERC4626HYPERDRIVEDEPLOYERCOORDINATOR_ABI.clone(),
                ERC4626HYPERDRIVEDEPLOYERCOORDINATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `coreDeployer` (0xc83e1f51) function
        pub fn core_deployer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([200, 62, 31, 81], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployHyperdrive` (0xe990eba8) function
        pub fn deploy_hyperdrive(
            &self,
            deployment_id: [u8; 32],
            name: ::std::string::String,
            deploy_config: PoolDeployConfig,
            extra_data: ::ethers::core::types::Bytes,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [233, 144, 235, 168],
                    (deployment_id, name, deploy_config, extra_data, salt),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployTarget` (0xc1510618) function
        pub fn deploy_target(
            &self,
            deployment_id: [u8; 32],
            deploy_config: PoolDeployConfig,
            extra_data: ::ethers::core::types::Bytes,
            target_index: ::ethers::core::types::U256,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [193, 81, 6, 24],
                    (deployment_id, deploy_config, extra_data, target_index, salt),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployments` (0x37404017) function
        pub fn deployments(
            &self,
            deployment_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, Deployment> {
            self.0
                .method_hash([55, 64, 64, 23], deployment_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNumberOfTargets` (0xe99be396) function
        pub fn get_number_of_targets(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([233, 155, 227, 150], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x16abfc70) function
        pub fn initialize(
            &self,
            deployment_id: [u8; 32],
            lp: ::ethers::core::types::Address,
            contribution: ::ethers::core::types::U256,
            apr: ::ethers::core::types::U256,
            options: Options,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [22, 171, 252, 112],
                    (deployment_id, lp, contribution, apr, options),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `kind` (0x04baa00b) function
        pub fn kind(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([4, 186, 160, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `target0Deployer` (0xab71905f) function
        pub fn target_0_deployer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([171, 113, 144, 95], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `target1Deployer` (0xa085fa30) function
        pub fn target_1_deployer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([160, 133, 250, 48], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `target2Deployer` (0xb6cb1118) function
        pub fn target_2_deployer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([182, 203, 17, 24], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `target3Deployer` (0xaa8cd6c4) function
        pub fn target_3_deployer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([170, 140, 214, 196], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC4626HyperdriveDeployerCoordinator<M> {
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
    ///Custom Error type `DeploymentAlreadyExists` with signature `DeploymentAlreadyExists()` and selector `0x77c3669a`
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
    #[etherror(name = "DeploymentAlreadyExists", abi = "DeploymentAlreadyExists()")]
    pub struct DeploymentAlreadyExists;
    ///Custom Error type `DeploymentDoesNotExist` with signature `DeploymentDoesNotExist()` and selector `0xe62c7024`
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
    #[etherror(name = "DeploymentDoesNotExist", abi = "DeploymentDoesNotExist()")]
    pub struct DeploymentDoesNotExist;
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
    ///Custom Error type `HyperdriveAlreadyDeployed` with signature `HyperdriveAlreadyDeployed()` and selector `0x2c95caeb`
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
    #[etherror(name = "HyperdriveAlreadyDeployed", abi = "HyperdriveAlreadyDeployed()")]
    pub struct HyperdriveAlreadyDeployed;
    ///Custom Error type `HyperdriveIsNotDeployed` with signature `HyperdriveIsNotDeployed()` and selector `0x952b05cb`
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
    #[etherror(name = "HyperdriveIsNotDeployed", abi = "HyperdriveIsNotDeployed()")]
    pub struct HyperdriveIsNotDeployed;
    ///Custom Error type `IncompleteDeployment` with signature `IncompleteDeployment()` and selector `0xe97cc2bf`
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
    #[etherror(name = "IncompleteDeployment", abi = "IncompleteDeployment()")]
    pub struct IncompleteDeployment;
    ///Custom Error type `InsufficientValue` with signature `InsufficientValue()` and selector `0x11011294`
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
    #[etherror(name = "InsufficientValue", abi = "InsufficientValue()")]
    pub struct InsufficientValue;
    ///Custom Error type `InvalidBaseToken` with signature `InvalidBaseToken()` and selector `0x0e442a4a`
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
    #[etherror(name = "InvalidBaseToken", abi = "InvalidBaseToken()")]
    pub struct InvalidBaseToken;
    ///Custom Error type `InvalidCheckpointDuration` with signature `InvalidCheckpointDuration()` and selector `0x5428734d`
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
    #[etherror(name = "InvalidCheckpointDuration", abi = "InvalidCheckpointDuration()")]
    pub struct InvalidCheckpointDuration;
    ///Custom Error type `InvalidFeeAmounts` with signature `InvalidFeeAmounts()` and selector `0x45ee5986`
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
    #[etherror(name = "InvalidFeeAmounts", abi = "InvalidFeeAmounts()")]
    pub struct InvalidFeeAmounts;
    ///Custom Error type `InvalidMinimumShareReserves` with signature `InvalidMinimumShareReserves()` and selector `0x49db44f5`
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
        name = "InvalidMinimumShareReserves",
        abi = "InvalidMinimumShareReserves()"
    )]
    pub struct InvalidMinimumShareReserves;
    ///Custom Error type `InvalidMinimumTransactionAmount` with signature `InvalidMinimumTransactionAmount()` and selector `0x3192a446`
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
        name = "InvalidMinimumTransactionAmount",
        abi = "InvalidMinimumTransactionAmount()"
    )]
    pub struct InvalidMinimumTransactionAmount;
    ///Custom Error type `InvalidPositionDuration` with signature `InvalidPositionDuration()` and selector `0x4a7fff9e`
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
    #[etherror(name = "InvalidPositionDuration", abi = "InvalidPositionDuration()")]
    pub struct InvalidPositionDuration;
    ///Custom Error type `InvalidTargetIndex` with signature `InvalidTargetIndex()` and selector `0x3b3f02b2`
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
    #[etherror(name = "InvalidTargetIndex", abi = "InvalidTargetIndex()")]
    pub struct InvalidTargetIndex;
    ///Custom Error type `InvalidVaultSharesToken` with signature `InvalidVaultSharesToken()` and selector `0xe7ee73a0`
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
    #[etherror(name = "InvalidVaultSharesToken", abi = "InvalidVaultSharesToken()")]
    pub struct InvalidVaultSharesToken;
    ///Custom Error type `MismatchedConfig` with signature `MismatchedConfig()` and selector `0x332ee11f`
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
    #[etherror(name = "MismatchedConfig", abi = "MismatchedConfig()")]
    pub struct MismatchedConfig;
    ///Custom Error type `MismatchedExtraData` with signature `MismatchedExtraData()` and selector `0xd1939688`
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
    #[etherror(name = "MismatchedExtraData", abi = "MismatchedExtraData()")]
    pub struct MismatchedExtraData;
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
    ///Custom Error type `SenderIsNotFactory` with signature `SenderIsNotFactory()` and selector `0x40843511`
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
    #[etherror(name = "SenderIsNotFactory", abi = "SenderIsNotFactory()")]
    pub struct SenderIsNotFactory;
    ///Custom Error type `TargetAlreadyDeployed` with signature `TargetAlreadyDeployed()` and selector `0x0744b578`
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
    #[etherror(name = "TargetAlreadyDeployed", abi = "TargetAlreadyDeployed()")]
    pub struct TargetAlreadyDeployed;
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
    pub enum ERC4626HyperdriveDeployerCoordinatorErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        DeploymentAlreadyExists(DeploymentAlreadyExists),
        DeploymentDoesNotExist(DeploymentDoesNotExist),
        FailedInnerCall(FailedInnerCall),
        HyperdriveAlreadyDeployed(HyperdriveAlreadyDeployed),
        HyperdriveIsNotDeployed(HyperdriveIsNotDeployed),
        IncompleteDeployment(IncompleteDeployment),
        InsufficientValue(InsufficientValue),
        InvalidBaseToken(InvalidBaseToken),
        InvalidCheckpointDuration(InvalidCheckpointDuration),
        InvalidFeeAmounts(InvalidFeeAmounts),
        InvalidMinimumShareReserves(InvalidMinimumShareReserves),
        InvalidMinimumTransactionAmount(InvalidMinimumTransactionAmount),
        InvalidPositionDuration(InvalidPositionDuration),
        InvalidTargetIndex(InvalidTargetIndex),
        InvalidVaultSharesToken(InvalidVaultSharesToken),
        MismatchedConfig(MismatchedConfig),
        MismatchedExtraData(MismatchedExtraData),
        NotPayable(NotPayable),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        SenderIsNotFactory(SenderIsNotFactory),
        TargetAlreadyDeployed(TargetAlreadyDeployed),
        TransferFailed(TransferFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ERC4626HyperdriveDeployerCoordinatorErrors {
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
            if let Ok(decoded) = <DeploymentAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeploymentAlreadyExists(decoded));
            }
            if let Ok(decoded) = <DeploymentDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeploymentDoesNotExist(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <HyperdriveAlreadyDeployed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HyperdriveAlreadyDeployed(decoded));
            }
            if let Ok(decoded) = <HyperdriveIsNotDeployed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HyperdriveIsNotDeployed(decoded));
            }
            if let Ok(decoded) = <IncompleteDeployment as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncompleteDeployment(decoded));
            }
            if let Ok(decoded) = <InsufficientValue as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientValue(decoded));
            }
            if let Ok(decoded) = <InvalidBaseToken as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidBaseToken(decoded));
            }
            if let Ok(decoded) = <InvalidCheckpointDuration as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidCheckpointDuration(decoded));
            }
            if let Ok(decoded) = <InvalidFeeAmounts as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidFeeAmounts(decoded));
            }
            if let Ok(decoded) = <InvalidMinimumShareReserves as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMinimumShareReserves(decoded));
            }
            if let Ok(decoded) = <InvalidMinimumTransactionAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMinimumTransactionAmount(decoded));
            }
            if let Ok(decoded) = <InvalidPositionDuration as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidPositionDuration(decoded));
            }
            if let Ok(decoded) = <InvalidTargetIndex as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidTargetIndex(decoded));
            }
            if let Ok(decoded) = <InvalidVaultSharesToken as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidVaultSharesToken(decoded));
            }
            if let Ok(decoded) = <MismatchedConfig as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MismatchedConfig(decoded));
            }
            if let Ok(decoded) = <MismatchedExtraData as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MismatchedExtraData(decoded));
            }
            if let Ok(decoded) = <NotPayable as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotPayable(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            if let Ok(decoded) = <SenderIsNotFactory as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SenderIsNotFactory(decoded));
            }
            if let Ok(decoded) = <TargetAlreadyDeployed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetAlreadyDeployed(decoded));
            }
            if let Ok(decoded) = <TransferFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFailed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressInsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeploymentAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeploymentDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HyperdriveAlreadyDeployed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HyperdriveIsNotDeployed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncompleteDeployment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidBaseToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidCheckpointDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFeeAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMinimumShareReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMinimumTransactionAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPositionDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTargetIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidVaultSharesToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MismatchedConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MismatchedExtraData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotPayable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SenderIsNotFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetAlreadyDeployed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert
    for ERC4626HyperdriveDeployerCoordinatorErrors {
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
                    == <DeploymentAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DeploymentDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <HyperdriveAlreadyDeployed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <HyperdriveIsNotDeployed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <IncompleteDeployment as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientValue as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidBaseToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidCheckpointDuration as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidFeeAmounts as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMinimumShareReserves as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMinimumTransactionAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidPositionDuration as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTargetIndex as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidVaultSharesToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MismatchedConfig as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MismatchedExtraData as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotPayable as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SenderIsNotFactory as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TargetAlreadyDeployed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeploymentAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeploymentDoesNotExist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::HyperdriveAlreadyDeployed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HyperdriveIsNotDeployed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncompleteDeployment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidBaseToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidCheckpointDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidFeeAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidMinimumShareReserves(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMinimumTransactionAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidPositionDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTargetIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidVaultSharesToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MismatchedConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::MismatchedExtraData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SenderIsNotFactory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TargetAlreadyDeployed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<DeploymentAlreadyExists>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: DeploymentAlreadyExists) -> Self {
            Self::DeploymentAlreadyExists(value)
        }
    }
    impl ::core::convert::From<DeploymentDoesNotExist>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: DeploymentDoesNotExist) -> Self {
            Self::DeploymentDoesNotExist(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<HyperdriveAlreadyDeployed>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: HyperdriveAlreadyDeployed) -> Self {
            Self::HyperdriveAlreadyDeployed(value)
        }
    }
    impl ::core::convert::From<HyperdriveIsNotDeployed>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: HyperdriveIsNotDeployed) -> Self {
            Self::HyperdriveIsNotDeployed(value)
        }
    }
    impl ::core::convert::From<IncompleteDeployment>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: IncompleteDeployment) -> Self {
            Self::IncompleteDeployment(value)
        }
    }
    impl ::core::convert::From<InsufficientValue>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: InsufficientValue) -> Self {
            Self::InsufficientValue(value)
        }
    }
    impl ::core::convert::From<InvalidBaseToken>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: InvalidBaseToken) -> Self {
            Self::InvalidBaseToken(value)
        }
    }
    impl ::core::convert::From<InvalidCheckpointDuration>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: InvalidCheckpointDuration) -> Self {
            Self::InvalidCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<InvalidFeeAmounts>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: InvalidFeeAmounts) -> Self {
            Self::InvalidFeeAmounts(value)
        }
    }
    impl ::core::convert::From<InvalidMinimumShareReserves>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: InvalidMinimumShareReserves) -> Self {
            Self::InvalidMinimumShareReserves(value)
        }
    }
    impl ::core::convert::From<InvalidMinimumTransactionAmount>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: InvalidMinimumTransactionAmount) -> Self {
            Self::InvalidMinimumTransactionAmount(value)
        }
    }
    impl ::core::convert::From<InvalidPositionDuration>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: InvalidPositionDuration) -> Self {
            Self::InvalidPositionDuration(value)
        }
    }
    impl ::core::convert::From<InvalidTargetIndex>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: InvalidTargetIndex) -> Self {
            Self::InvalidTargetIndex(value)
        }
    }
    impl ::core::convert::From<InvalidVaultSharesToken>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: InvalidVaultSharesToken) -> Self {
            Self::InvalidVaultSharesToken(value)
        }
    }
    impl ::core::convert::From<MismatchedConfig>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: MismatchedConfig) -> Self {
            Self::MismatchedConfig(value)
        }
    }
    impl ::core::convert::From<MismatchedExtraData>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: MismatchedExtraData) -> Self {
            Self::MismatchedExtraData(value)
        }
    }
    impl ::core::convert::From<NotPayable>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: NotPayable) -> Self {
            Self::NotPayable(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<SenderIsNotFactory>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: SenderIsNotFactory) -> Self {
            Self::SenderIsNotFactory(value)
        }
    }
    impl ::core::convert::From<TargetAlreadyDeployed>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: TargetAlreadyDeployed) -> Self {
            Self::TargetAlreadyDeployed(value)
        }
    }
    impl ::core::convert::From<TransferFailed>
    for ERC4626HyperdriveDeployerCoordinatorErrors {
        fn from(value: TransferFailed) -> Self {
            Self::TransferFailed(value)
        }
    }
    ///Container type for all input parameters for the `coreDeployer` function with signature `coreDeployer()` and selector `0xc83e1f51`
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
    #[ethcall(name = "coreDeployer", abi = "coreDeployer()")]
    pub struct CoreDeployerCall;
    ///Container type for all input parameters for the `deployHyperdrive` function with signature `deployHyperdrive(bytes32,string,(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256)),bytes,bytes32)` and selector `0xe990eba8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[ethcall(
        name = "deployHyperdrive",
        abi = "deployHyperdrive(bytes32,string,(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256)),bytes,bytes32)"
    )]
    pub struct DeployHyperdriveCall {
        pub deployment_id: [u8; 32],
        pub name: ::std::string::String,
        pub deploy_config: PoolDeployConfig,
        pub extra_data: ::ethers::core::types::Bytes,
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `deployTarget` function with signature `deployTarget(bytes32,(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256)),bytes,uint256,bytes32)` and selector `0xc1510618`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[ethcall(
        name = "deployTarget",
        abi = "deployTarget(bytes32,(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256)),bytes,uint256,bytes32)"
    )]
    pub struct DeployTargetCall {
        pub deployment_id: [u8; 32],
        pub deploy_config: PoolDeployConfig,
        pub extra_data: ::ethers::core::types::Bytes,
        pub target_index: ::ethers::core::types::U256,
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `deployments` function with signature `deployments(bytes32)` and selector `0x37404017`
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
    #[ethcall(name = "deployments", abi = "deployments(bytes32)")]
    pub struct DeploymentsCall {
        pub deployment_id: [u8; 32],
    }
    ///Container type for all input parameters for the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `getNumberOfTargets` function with signature `getNumberOfTargets()` and selector `0xe99be396`
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
    #[ethcall(name = "getNumberOfTargets", abi = "getNumberOfTargets()")]
    pub struct GetNumberOfTargetsCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(bytes32,address,uint256,uint256,(address,bool,bytes))` and selector `0x16abfc70`
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
        abi = "initialize(bytes32,address,uint256,uint256,(address,bool,bytes))"
    )]
    pub struct InitializeCall {
        pub deployment_id: [u8; 32],
        pub lp: ::ethers::core::types::Address,
        pub contribution: ::ethers::core::types::U256,
        pub apr: ::ethers::core::types::U256,
        pub options: Options,
    }
    ///Container type for all input parameters for the `kind` function with signature `kind()` and selector `0x04baa00b`
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
    #[ethcall(name = "kind", abi = "kind()")]
    pub struct KindCall;
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `target0Deployer` function with signature `target0Deployer()` and selector `0xab71905f`
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
    #[ethcall(name = "target0Deployer", abi = "target0Deployer()")]
    pub struct Target0DeployerCall;
    ///Container type for all input parameters for the `target1Deployer` function with signature `target1Deployer()` and selector `0xa085fa30`
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
    #[ethcall(name = "target1Deployer", abi = "target1Deployer()")]
    pub struct Target1DeployerCall;
    ///Container type for all input parameters for the `target2Deployer` function with signature `target2Deployer()` and selector `0xb6cb1118`
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
    #[ethcall(name = "target2Deployer", abi = "target2Deployer()")]
    pub struct Target2DeployerCall;
    ///Container type for all input parameters for the `target3Deployer` function with signature `target3Deployer()` and selector `0xaa8cd6c4`
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
    #[ethcall(name = "target3Deployer", abi = "target3Deployer()")]
    pub struct Target3DeployerCall;
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
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
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
    )]
    pub enum ERC4626HyperdriveDeployerCoordinatorCalls {
        CoreDeployer(CoreDeployerCall),
        DeployHyperdrive(DeployHyperdriveCall),
        DeployTarget(DeployTargetCall),
        Deployments(DeploymentsCall),
        Factory(FactoryCall),
        GetNumberOfTargets(GetNumberOfTargetsCall),
        Initialize(InitializeCall),
        Kind(KindCall),
        Name(NameCall),
        Target0Deployer(Target0DeployerCall),
        Target1Deployer(Target1DeployerCall),
        Target2Deployer(Target2DeployerCall),
        Target3Deployer(Target3DeployerCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CoreDeployerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CoreDeployer(decoded));
            }
            if let Ok(decoded) = <DeployHyperdriveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployHyperdrive(decoded));
            }
            if let Ok(decoded) = <DeployTargetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployTarget(decoded));
            }
            if let Ok(decoded) = <DeploymentsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deployments(decoded));
            }
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <GetNumberOfTargetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNumberOfTargets(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <KindCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Kind(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <Target0DeployerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Target0Deployer(decoded));
            }
            if let Ok(decoded) = <Target1DeployerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Target1Deployer(decoded));
            }
            if let Ok(decoded) = <Target2DeployerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Target2Deployer(decoded));
            }
            if let Ok(decoded) = <Target3DeployerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Target3Deployer(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CoreDeployer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployHyperdrive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployTarget(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deployments(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNumberOfTargets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Kind(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Target0Deployer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Target1Deployer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Target2Deployer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Target3Deployer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CoreDeployer(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeployHyperdrive(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeployTarget(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deployments(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNumberOfTargets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Kind(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Target0Deployer(element) => ::core::fmt::Display::fmt(element, f),
                Self::Target1Deployer(element) => ::core::fmt::Display::fmt(element, f),
                Self::Target2Deployer(element) => ::core::fmt::Display::fmt(element, f),
                Self::Target3Deployer(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CoreDeployerCall>
    for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn from(value: CoreDeployerCall) -> Self {
            Self::CoreDeployer(value)
        }
    }
    impl ::core::convert::From<DeployHyperdriveCall>
    for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn from(value: DeployHyperdriveCall) -> Self {
            Self::DeployHyperdrive(value)
        }
    }
    impl ::core::convert::From<DeployTargetCall>
    for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn from(value: DeployTargetCall) -> Self {
            Self::DeployTarget(value)
        }
    }
    impl ::core::convert::From<DeploymentsCall>
    for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn from(value: DeploymentsCall) -> Self {
            Self::Deployments(value)
        }
    }
    impl ::core::convert::From<FactoryCall>
    for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<GetNumberOfTargetsCall>
    for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn from(value: GetNumberOfTargetsCall) -> Self {
            Self::GetNumberOfTargets(value)
        }
    }
    impl ::core::convert::From<InitializeCall>
    for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<KindCall> for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn from(value: KindCall) -> Self {
            Self::Kind(value)
        }
    }
    impl ::core::convert::From<NameCall> for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<Target0DeployerCall>
    for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn from(value: Target0DeployerCall) -> Self {
            Self::Target0Deployer(value)
        }
    }
    impl ::core::convert::From<Target1DeployerCall>
    for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn from(value: Target1DeployerCall) -> Self {
            Self::Target1Deployer(value)
        }
    }
    impl ::core::convert::From<Target2DeployerCall>
    for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn from(value: Target2DeployerCall) -> Self {
            Self::Target2Deployer(value)
        }
    }
    impl ::core::convert::From<Target3DeployerCall>
    for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn from(value: Target3DeployerCall) -> Self {
            Self::Target3Deployer(value)
        }
    }
    impl ::core::convert::From<VersionCall>
    for ERC4626HyperdriveDeployerCoordinatorCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `coreDeployer` function with signature `coreDeployer()` and selector `0xc83e1f51`
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
    pub struct CoreDeployerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `deployHyperdrive` function with signature `deployHyperdrive(bytes32,string,(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256)),bytes,bytes32)` and selector `0xe990eba8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
    )]
    pub struct DeployHyperdriveReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `deployTarget` function with signature `deployTarget(bytes32,(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256)),bytes,uint256,bytes32)` and selector `0xc1510618`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
    )]
    pub struct DeployTargetReturn {
        pub target: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployments` function with signature `deployments(bytes32)` and selector `0x37404017`
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
    pub struct DeploymentsReturn(pub Deployment);
    ///Container type for all return fields from the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getNumberOfTargets` function with signature `getNumberOfTargets()` and selector `0xe99be396`
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
    pub struct GetNumberOfTargetsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `initialize` function with signature `initialize(bytes32,address,uint256,uint256,(address,bool,bytes))` and selector `0x16abfc70`
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
    pub struct InitializeReturn {
        pub lp_shares: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `kind` function with signature `kind()` and selector `0x04baa00b`
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
    pub struct KindReturn(pub ::std::string::String);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `target0Deployer` function with signature `target0Deployer()` and selector `0xab71905f`
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
    pub struct Target0DeployerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `target1Deployer` function with signature `target1Deployer()` and selector `0xa085fa30`
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
    pub struct Target1DeployerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `target2Deployer` function with signature `target2Deployer()` and selector `0xb6cb1118`
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
    pub struct Target2DeployerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `target3Deployer` function with signature `target3Deployer()` and selector `0xaa8cd6c4`
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
    pub struct Target3DeployerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
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
    pub struct VersionReturn(pub ::std::string::String);
    ///`Deployment(bytes32,bytes32,uint256,address,address,address,address,address)`
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
    pub struct Deployment {
        pub config_hash: [u8; 32],
        pub extra_data_hash: [u8; 32],
        pub initial_share_price: ::ethers::core::types::U256,
        pub hyperdrive: ::ethers::core::types::Address,
        pub target_0: ::ethers::core::types::Address,
        pub target_1: ::ethers::core::types::Address,
        pub target_2: ::ethers::core::types::Address,
        pub target_3: ::ethers::core::types::Address,
    }
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
    ///`PoolDeployConfig(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256))`
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
    pub struct PoolDeployConfig {
        pub base_token: ::ethers::core::types::Address,
        pub vault_shares_token: ::ethers::core::types::Address,
        pub linker_factory: ::ethers::core::types::Address,
        pub linker_code_hash: [u8; 32],
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
