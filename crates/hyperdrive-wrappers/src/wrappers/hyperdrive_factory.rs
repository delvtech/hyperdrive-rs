pub use hyperdrive_factory::*;
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
pub mod hyperdrive_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_factoryConfig"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                            ::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(
                                    ::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(
                                    ::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "struct HyperdriveFactory.FactoryConfig",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("_instancesToDeployerCoordinators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_instancesToDeployerCoordinators",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("instance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deployCoordinator"),
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
                    ::std::borrow::ToOwned::to_owned("addDeployerCoordinator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addDeployerCoordinator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_deployerCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkpointDurationResolution"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "checkpointDurationResolution",
                            ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkpointRewarder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkpointRewarder"),
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
                    ::std::borrow::ToOwned::to_owned("defaultPausers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("defaultPausers"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deployAndInitialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deployAndInitialize",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_deployerCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_contribution"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_fixedAPR"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_timeStretchAPR"),
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
                                        ::std::borrow::ToOwned::to_owned("contract IHyperdrive"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_deployerCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("_fixedAPR"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_timeStretchAPR"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("deployerCoordinatorManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deployerCoordinatorManager",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("feeCollector"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeCollector"),
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
                    ::std::borrow::ToOwned::to_owned("getDeployerCoordinatorAtIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDeployerCoordinatorAtIndex",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getDeployerCoordinatorByInstances",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDeployerCoordinatorByInstances",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("__instances"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("coordinators"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDeployerCoordinatorsInRange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDeployerCoordinatorsInRange",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_startIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_endIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("range"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInstanceAtIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInstanceAtIndex"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInstancesInRange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getInstancesInRange",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_startIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_endIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("range"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNumberOfDeployerCoordinators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getNumberOfDeployerCoordinators",
                            ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNumberOfInstances"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getNumberOfInstances",
                            ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("governance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("governance"),
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
                    ::std::borrow::ToOwned::to_owned("hyperdriveGovernance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "hyperdriveGovernance",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("isDeployerCoordinator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isDeployerCoordinator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isInstance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isInstance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("linkerCodeHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("linkerCodeHash"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("linkerFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("linkerFactory"),
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
                    ::std::borrow::ToOwned::to_owned("maxCheckpointDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "maxCheckpointDuration",
                            ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxCircuitBreakerDelta"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "maxCircuitBreakerDelta",
                            ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxFees"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IHyperdrive.Fees"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxFixedAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxFixedAPR"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxPositionDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "maxPositionDuration",
                            ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxTimeStretchAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxTimeStretchAPR"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minCheckpointDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "minCheckpointDuration",
                            ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minCircuitBreakerDelta"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "minCircuitBreakerDelta",
                            ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("minFees"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IHyperdrive.Fees"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minFixedAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("minFixedAPR"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minPositionDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "minPositionDuration",
                            ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minTimeStretchAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("minTimeStretchAPR"),
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
                    ::std::borrow::ToOwned::to_owned("removeDeployerCoordinator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeDeployerCoordinator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_deployerCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sweepCollector"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sweepCollector"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "updateCheckpointDurationResolution",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateCheckpointDurationResolution",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_checkpointDurationResolution",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateCheckpointRewarder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateCheckpointRewarder",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_checkpointRewarder",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateDefaultPausers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateDefaultPausers",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_defaultPausers_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateDeployerCoordinatorManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateDeployerCoordinatorManager",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_deployerCoordinatorManager",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateFeeCollector"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateFeeCollector"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_feeCollector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateGovernance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateGovernance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_governance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateHyperdriveGovernance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateHyperdriveGovernance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_hyperdriveGovernance",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateLinkerCodeHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateLinkerCodeHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_linkerCodeHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateLinkerFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateLinkerFactory",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_linkerFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMaxCheckpointDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateMaxCheckpointDuration",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_maxCheckpointDuration",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMaxCircuitBreakerDelta"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateMaxCircuitBreakerDelta",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_maxCircuitBreakerDelta",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMaxFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateMaxFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("__maxFees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IHyperdrive.Fees"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMaxFixedAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateMaxFixedAPR"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxFixedAPR"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMaxPositionDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateMaxPositionDuration",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_maxPositionDuration",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMaxTimeStretchAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateMaxTimeStretchAPR",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_maxTimeStretchAPR",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMinCheckpointDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateMinCheckpointDuration",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_minCheckpointDuration",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMinCircuitBreakerDelta"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateMinCircuitBreakerDelta",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_minCircuitBreakerDelta",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMinFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateMinFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("__minFees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IHyperdrive.Fees"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMinFixedAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateMinFixedAPR"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minFixedAPR"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMinPositionDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateMinPositionDuration",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_minPositionDuration",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMinTimeStretchAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateMinTimeStretchAPR",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_minTimeStretchAPR",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateSweepCollector"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateSweepCollector",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sweepCollector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CheckpointDurationResolutionUpdated",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CheckpointDurationResolutionUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newCheckpointDurationResolution",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("DefaultPausersUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DefaultPausersUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newDefaultPausers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Deployed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deployed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "deployerCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("hyperdrive"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("config"),
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
                    ::std::borrow::ToOwned::to_owned("DeployerCoordinatorAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeployerCoordinatorAdded",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "deployerCoordinator",
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
                    ::std::borrow::ToOwned::to_owned(
                        "DeployerCoordinatorManagerUpdated",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeployerCoordinatorManagerUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "deployerCoordinatorManager",
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
                    ::std::borrow::ToOwned::to_owned("DeployerCoordinatorRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeployerCoordinatorRemoved",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "deployerCoordinator",
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
                                    name: ::std::borrow::ToOwned::to_owned("governance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HyperdriveGovernanceUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HyperdriveGovernanceUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "hyperdriveGovernance",
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
                    ::std::borrow::ToOwned::to_owned("LinkerCodeHashUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LinkerCodeHashUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newLinkerCodeHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LinkerFactoryUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LinkerFactoryUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newLinkerFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MaxCheckpointDurationUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MaxCheckpointDurationUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMaxCheckpointDuration",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("MaxCircuitBreakerDeltaUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MaxCircuitBreakerDeltaUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMaxCircuitBreakerDelta",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("MaxFeesUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MaxFeesUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newMaxFees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MaxFixedAPRUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MaxFixedAPRUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newMaxFixedAPR"),
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
                    ::std::borrow::ToOwned::to_owned("MaxPositionDurationUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MaxPositionDurationUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMaxPositionDuration",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("MaxTimeStretchAPRUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MaxTimeStretchAPRUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMaxTimeStretchAPR",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("MinCheckpointDurationUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MinCheckpointDurationUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMinCheckpointDuration",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("MinCircuitBreakerDeltaUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MinCircuitBreakerDeltaUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMinCircuitBreakerDelta",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("MinFeesUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MinFeesUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newMinFees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MinFixedAPRUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MinFixedAPRUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newMinFixedAPR"),
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
                    ::std::borrow::ToOwned::to_owned("MinPositionDurationUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MinPositionDurationUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMinPositionDuration",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("MinTimeStretchAPRUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MinTimeStretchAPRUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMinTimeStretchAPR",
                                    ),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DeployerCoordinatorAlreadyAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeployerCoordinatorAlreadyAdded",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DeployerCoordinatorIndexMismatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeployerCoordinatorIndexMismatch",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DeployerCoordinatorNotAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeployerCoordinatorNotAdded",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EndIndexTooLarge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EndIndexTooLarge"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "InvalidCheckpointDurationResolution",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidCheckpointDurationResolution",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidCircuitBreakerDelta"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidCircuitBreakerDelta",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidDeployConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidDeployConfig",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidDeployerCoordinator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidDeployerCoordinator",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidFees"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFixedAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidFixedAPR"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidIndexes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidIndexes"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMaxCheckpointDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidMaxCheckpointDuration",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMaxCircuitBreakerDelta"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidMaxCircuitBreakerDelta",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMaxFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidMaxFees"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMaxFixedAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidMaxFixedAPR"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMaxPositionDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidMaxPositionDuration",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMaxTimeStretchAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidMaxTimeStretchAPR",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMinCheckpointDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidMinCheckpointDuration",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMinCircuitBreakerDelta"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidMinCircuitBreakerDelta",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMinFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidMinFees"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMinFixedAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidMinFixedAPR"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMinPositionDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidMinPositionDuration",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMinTimeStretchAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidMinTimeStretchAPR",
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
                    ::std::borrow::ToOwned::to_owned("InvalidTimeStretchAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidTimeStretchAPR",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("ReceiveLocked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ReceiveLocked"),
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
                    ::std::borrow::ToOwned::to_owned("Unauthorized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Unauthorized"),
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
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static HYPERDRIVEFACTORY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x02`\x01U4\x80\x15b\0\0\x16W`\0\x80\xFD[P`@Qb\0A,8\x03\x80b\0A,\x839\x81\x01`@\x81\x90Rb\0\09\x91b\0\x07\x08V[`\0b\0\0G\x82\x82b\0\tmV[P\x81`\xE0\x01Q\x82a\x01\0\x01Q\x10\x80b\0\0uWP\x81`\xE0\x01Q\x82a\x01\0\x01Qb\0\0r\x91\x90b\0\n9V[\x15\x15[\x15b\0\0\x94W`@Qc\x02\x19\xD6c`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\0\x82\x01Q`\x0B\x81\x90Ua\x01 \x83\x01Q\x10\x80b\0\0\xC7WP\x81`\xE0\x01Q\x82a\x01 \x01Qb\0\0\xC4\x91\x90b\0\n9V[\x15\x15[\x15b\0\0\xE6W`@Qc\xF9\xC0\x95\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01 \x82\x01Q`\x0C\x81\x90Ua\x01@\x83\x01Q\x10\x80b\0\x01\x19WP\x81`\xE0\x01Q\x82a\x01@\x01Qb\0\x01\x16\x91\x90b\0\n9V[\x15\x15[\x15b\0\x018W`@Qc0\x07\xAD\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01@\x82\x01Q`\r\x81\x90Ua\x01`\x83\x01Q\x10\x80b\0\x01kWP\x81`\xE0\x01Q\x82a\x01`\x01Qb\0\x01h\x91\x90b\0\n9V[\x15\x15[\x15b\0\x01\x8AW`@Qc\xCF\xB6\x99\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01`\x82\x01Q`\x0EUa\x01\xA0\x82\x01Qa\x01\x80\x83\x01Q\x11\x15b\0\x01\xBFW`@Qc\xEF\x9B\xC6_`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\x80\x82\x01Q`\x0FUa\x01\xA0\x82\x01Q`\x10Ua\x01\xE0\x82\x01Qa\x01\xC0\x83\x01Q\x11\x15b\0\x01\xFDW`@Qc0UM\xE1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\xC0\x82\x01Q`\x11Ua\x01\xE0\x82\x01Q`\x12Ua\x02 \x82\x01Qa\x02\0\x83\x01Q\x11\x15b\0\x02;W`@Qc\x83\xEB\xDF\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02\0\x82\x01Q`\x13Ua\x02 \x82\x01Q`\x14Ua\x02`\x82\x01QQg\r\xE0\xB6\xB3\xA7d\0\0\x10\x80b\0\x02zWPg\r\xE0\xB6\xB3\xA7d\0\0\x82a\x02`\x01Q` \x01Q\x11[\x80b\0\x02\x96WPg\r\xE0\xB6\xB3\xA7d\0\0\x82a\x02`\x01Q`@\x01Q\x11[\x80b\0\x02\xB2WPg\r\xE0\xB6\xB3\xA7d\0\0\x82a\x02`\x01Q``\x01Q\x11[\x15b\0\x02\xD1W`@Qc\x16\x10q\xFB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02`\x82\x01Q\x80Q`\x19\x81\x90U` \x82\x01Q`\x1AU`@\x82\x01Q`\x1BU``\x90\x91\x01Q`\x1CUa\x02@\x83\x01QQ\x11\x80b\0\x03\x1CWP\x81a\x02`\x01Q` \x01Q\x82a\x02@\x01Q` \x01Q\x11[\x80b\0\x039WP\x81a\x02`\x01Q`@\x01Q\x82a\x02@\x01Q`@\x01Q\x11[\x80b\0\x03VWP\x81a\x02`\x01Q``\x01Q\x82a\x02@\x01Q``\x01Q\x11[\x15b\0\x03uW`@Qc\x15\xB0Z\x8F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02@\x82\x01Q\x80Q`\x15U` \x80\x82\x01Q`\x16U`@\x80\x83\x01Q`\x17U``\x92\x83\x01Q`\x18U\x84Q`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U\x83\x87\x01Q`\x03\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x91\x86\x01Q`\x04\x80T\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`\x80\x86\x01Q`\x07\x80T\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`\xA0\x86\x01Q`\x08\x80T\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`\xC0\x86\x01Q`\t\x80T\x90\x93\x16\x91\x16\x17\x90U\x90\x83\x01Q\x80Qb\0\x041\x92`\x1D\x92\x01\x90b\0\x04nV[PPa\x02\x80\x81\x01Q`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90Ua\x02\xA0\x81\x01Q`\x06U`\xE0\x01Q`\nUb\0\n\\V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15b\0\x04\xC6W\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\x04\xC6W\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90b\0\x04\x8FV[Pb\0\x04\xD4\x92\x91Pb\0\x04\xD8V[P\x90V[[\x80\x82\x11\x15b\0\x04\xD4W`\0\x81U`\x01\x01b\0\x04\xD9V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x02\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x05+Wb\0\x05+b\0\x04\xEFV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x05\\Wb\0\x05\\b\0\x04\xEFV[`@R\x91\x90PV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x05|W`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12b\0\x05\x93W`\0\x80\xFD[\x81Q` `\x01`\x01`@\x1B\x03\x82\x11\x15b\0\x05\xB1Wb\0\x05\xB1b\0\x04\xEFV[\x81`\x05\x1Bb\0\x05\xC2\x82\x82\x01b\0\x051V[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15b\0\x05\xDDW`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15b\0\x06\x07Wb\0\x05\xF7\x83b\0\x05dV[\x82R\x91\x83\x01\x91\x90\x83\x01\x90b\0\x05\xE3V[\x97\x96PPPPPPPV[`\0`\x80\x82\x84\x03\x12\x15b\0\x06%W`\0\x80\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x06JWb\0\x06Jb\0\x04\xEFV[\x80`@RP\x80\x91P\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01RP\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12b\0\x06\x8DW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x06\xA9Wb\0\x06\xA9b\0\x04\xEFV[` b\0\x06\xBF`\x1F\x83\x01`\x1F\x19\x16\x82\x01b\0\x051V[\x82\x81R\x85\x82\x84\x87\x01\x01\x11\x15b\0\x06\xD4W`\0\x80\xFD[`\0[\x83\x81\x10\x15b\0\x06\xF4W\x85\x81\x01\x83\x01Q\x82\x82\x01\x84\x01R\x82\x01b\0\x06\xD7V[P`\0\x92\x81\x01\x90\x91\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x07\x1CW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x074W`\0\x80\xFD[\x90\x84\x01\x90a\x03\x80\x82\x87\x03\x12\x15b\0\x07JW`\0\x80\xFD[b\0\x07Tb\0\x05\x05V[b\0\x07_\x83b\0\x05dV[\x81Rb\0\x07o` \x84\x01b\0\x05dV[` \x82\x01Rb\0\x07\x82`@\x84\x01b\0\x05dV[`@\x82\x01R``\x83\x01Q\x82\x81\x11\x15b\0\x07\x9AW`\0\x80\xFD[b\0\x07\xA8\x88\x82\x86\x01b\0\x05\x81V[``\x83\x01RPb\0\x07\xBC`\x80\x84\x01b\0\x05dV[`\x80\x82\x01Rb\0\x07\xCF`\xA0\x84\x01b\0\x05dV[`\xA0\x82\x01Rb\0\x07\xE2`\xC0\x84\x01b\0\x05dV[`\xC0\x82\x01R`\xE0\x83\x81\x01Q\x90\x82\x01Ra\x01\0\x80\x84\x01Q\x90\x82\x01Ra\x01 \x80\x84\x01Q\x90\x82\x01Ra\x01@\x80\x84\x01Q\x90\x82\x01Ra\x01`\x80\x84\x01Q\x90\x82\x01Ra\x01\x80\x80\x84\x01Q\x90\x82\x01Ra\x01\xA0\x80\x84\x01Q\x90\x82\x01Ra\x01\xC0\x80\x84\x01Q\x90\x82\x01Ra\x01\xE0\x80\x84\x01Q\x90\x82\x01Ra\x02\0\x80\x84\x01Q\x90\x82\x01Ra\x02 \x80\x84\x01Q\x90\x82\x01Ra\x02@b\0\x08p\x88\x82\x86\x01b\0\x06\x12V[\x90\x82\x01Rb\0\x08\x84\x87a\x02\xC0\x85\x01b\0\x06\x12V[a\x02`\x82\x01Rb\0\x08\x99a\x03@\x84\x01b\0\x05dV[a\x02\x80\x82\x01Ra\x03`\x92\x90\x92\x01Qa\x02\xA0\x83\x01R` \x85\x01Q\x91\x93P\x80\x82\x11\x15b\0\x08\xC3W`\0\x80\xFD[Pb\0\x08\xD2\x85\x82\x86\x01b\0\x06{V[\x91PP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x08\xF1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\t\x12WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\thW`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\tCWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\tdW\x82\x81U`\x01\x01b\0\tOV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\t\x89Wb\0\t\x89b\0\x04\xEFV[b\0\t\xA1\x81b\0\t\x9A\x84Tb\0\x08\xDCV[\x84b\0\t\x18V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\t\xD9W`\0\x84\x15b\0\t\xC0WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\tdV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\n\nW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\t\xE9V[P\x85\x82\x10\x15b\0\n)W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x82b\0\nWWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[a6\xC0\x80b\0\nl`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x03\xA6W`\x005`\xE0\x1C\x80c\x8E\x12|\xF5\x11a\x01\xE7W\x80c\xD2\xC3\\\xE8\x11a\x01\rW\x80c\xE4\xE7\x14\x8F\x11a\0\xA0W\x80c\xEC\x89_\x11\x11a\0oW\x80c\xEC\x89_\x11\x14a\n\xE9W\x80c\xF2YdX\x14a\x0B\tW\x80c\xF8\xC0\x9EY\x14a\x0B)W\x80c\xFE=Z\xEB\x14a\x0BYW`\0\x80\xFD[\x80c\xE4\xE7\x14\x8F\x14a\ntW\x80c\xE7\x1F4\xB3\x14a\n\x94W\x80c\xE8>4\xB1\x14a\n\xB4W\x80c\xEBq\xF6l\x14a\n\xC9W`\0\x80\xFD[\x80c\xDD+\x8F\xBB\x11a\0\xDCW\x80c\xDD+\x8F\xBB\x14a\n\tW\x80c\xE0\xE2\xDA\xAA\x14a\n)W\x80c\xE1\xB3\x9C\x80\x14a\n?W\x80c\xE33\x15U\x14a\nTW`\0\x80\xFD[\x80c\xD2\xC3\\\xE8\x14a\t\x9DW\x80c\xD6\xF5\x01i\x14a\t\xBDW\x80c\xDA\xAC$\xDA\x14a\t\xD3W\x80c\xDA\xF0\x12\xE6\x14a\t\xF3W`\0\x80\xFD[\x80c\xB2V\x12c\x11a\x01\x85W\x80c\xC4\x15\xB9\\\x11a\x01TW\x80c\xC4\x15\xB9\\\x14a\t;W\x80c\xC9\x05\xA4\xB5\x14a\t[W\x80c\xD0\xF9k\x92\x14a\tqW\x80c\xD2=~\xA3\x14a\t\x87W`\0\x80\xFD[\x80c\xB2V\x12c\x14a\x08\xC3W\x80c\xBC0\xE7\xA1\x14a\x08\xE3W\x80c\xBF\x9B\xD5\xCD\x14a\t\x03W\x80c\xC1r%c\x14a\t\x19W`\0\x80\xFD[\x80c\x99b;\xB1\x11a\x01\xC1W\x80c\x99b;\xB1\x14a\x08NW\x80c\x9A\xF2Rb\x14a\x08nW\x80c\xA6L\x90\xBF\x14a\x08\x8EW\x80c\xA9\x8AF\xDB\x14a\x08\xA3W`\0\x80\xFD[\x80c\x8E\x12|\xF5\x14a\x07\xF8W\x80c\x8E\xFC\t\x86\x14a\x08\x18W\x80c\x97\xB0\xE8\xCE\x14a\x08.W`\0\x80\xFD[\x80cET\xF9\xA9\x11a\x02\xCCW\x80cb\x80'\xA3\x11a\x02jW\x80c\x83\xB3a\xE8\x11a\x029W\x80c\x83\xB3a\xE8\x14a\x07xW\x80c\x84\xC1\x9A\xAB\x14a\x07\x98W\x80c\x85\"\x97\x85\x14a\x07\xB8W\x80c\x86'\xA4\xF0\x14a\x07\xD8W`\0\x80\xFD[\x80cb\x80'\xA3\x14a\x06\xE3W\x80ckD\xE6\xBE\x14a\x07\x03W\x80cn\x95\xD6|\x14a\x07CW\x80com\\J\x14a\x07XW`\0\x80\xFD[\x80cO\xBF\xEEw\x11a\x02\xA6W\x80cO\xBF\xEEw\x14a\x06ZW\x80cT\xFDMP\x14a\x06zW\x80cW \xC9\xD5\x14a\x06\xADW\x80cZ\xA6\xE6u\x14a\x06\xC3W`\0\x80\xFD[\x80cET\xF9\xA9\x14a\x06\x0EW\x80cH\x80\x07`\x14a\x06$W\x80cI\xF1=\xE7\x14a\x06:W`\0\x80\xFD[\x80c\x1E\xCD\xA0\xFE\x11a\x03DW\x80c.|\xD9q\x11a\x03\x13W\x80c.|\xD9q\x14a\x05\x9BW\x80c>- \x14\x14a\x05\xAEW\x80cA\x1C05\x14a\x05\xCEW\x80cB\x1C\xAB\xA8\x14a\x05\xEEW`\0\x80\xFD[\x80c\x1E\xCD\xA0\xFE\x14a\x05\x02W\x80c(\x85\xE3\xAC\x14a\x05&W\x80c)\x07\xD3\xDD\x14a\x05FW\x80c+X\xF4\x18\x14a\x05fW`\0\x80\xFD[\x80c\x10\xD1\xDC>\x11a\x03\x80W\x80c\x10\xD1\xDC>\x14a\x04uW\x80c\x11\xE7{\xFE\x14a\x04\x95W\x80c\x19x\xEB\xCF\x14a\x04\xB5W\x80c\x1BY\xBE\x0C\x14a\x04\xD5W`\0\x80\xFD[\x80c\x04\xBA\xA0\x0B\x14a\x03\xD5W\x80c\x06\xFD\xDE\x03\x14a\x04(W\x80c\x10x\x0Fs\x14a\x04=W`\0\x80\xFD[6a\x03\xD0W`\x02`\x01T\x03a\x03\xCEW`@Qc\n\xACu\xB5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\x03\xE1W`\0\x80\xFD[Pa\x04\x12`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01pHyperdriveFactory`x\x1B\x81RP\x81V[`@Qa\x04\x1F\x91\x90a,\xECV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x044W`\0\x80\xFD[Pa\x04\x12a\x0ByV[4\x80\x15a\x04IW`\0\x80\xFD[P`\x08Ta\x04]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\x1FV[4\x80\x15a\x04\x81W`\0\x80\xFD[Pa\x03\xCEa\x04\x906`\x04a,\xFFV[a\x0C\x07V[4\x80\x15a\x04\xA1W`\0\x80\xFD[Pa\x03\xCEa\x04\xB06`\x04a-\x11V[a\r V[4\x80\x15a\x04\xC1W`\0\x80\xFD[Pa\x03\xCEa\x04\xD06`\x04a-\x11V[a\r\xF9V[4\x80\x15a\x04\xE1W`\0\x80\xFD[Pa\x04\xF5a\x04\xF06`\x04a-*V[a\x0E{V[`@Qa\x04\x1F\x91\x90a-\x9FV[4\x80\x15a\x05\x0EW`\0\x80\xFD[Pa\x05\x18`\x0FT\x81V[`@Q\x90\x81R` \x01a\x04\x1FV[4\x80\x15a\x052W`\0\x80\xFD[Pa\x03\xCEa\x05A6`\x04a,\xFFV[a\x0FVV[4\x80\x15a\x05RW`\0\x80\xFD[Pa\x03\xCEa\x05a6`\x04a-\x11V[a\x10\xBCV[4\x80\x15a\x05rW`\0\x80\xFD[Pa\x04]a\x05\x816`\x04a.\x14V[` \x80R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04]a\x05\xA96`\x04a0\xEEV[a\x11>V[4\x80\x15a\x05\xBAW`\0\x80\xFD[Pa\x03\xCEa\x05\xC96`\x04a.\x14V[a\x15\x80V[4\x80\x15a\x05\xDAW`\0\x80\xFD[Pa\x03\xCEa\x05\xE96`\x04a1\xCFV[a\x15\xF4V[4\x80\x15a\x05\xFAW`\0\x80\xFD[Pa\x03\xCEa\x06\t6`\x04a.\x14V[a\x17\xC5V[4\x80\x15a\x06\x1AW`\0\x80\xFD[Pa\x05\x18`\x10T\x81V[4\x80\x15a\x060W`\0\x80\xFD[Pa\x05\x18`\x14T\x81V[4\x80\x15a\x06FW`\0\x80\xFD[Pa\x04]a\x06U6`\x04a1\xFBV[a\x18\xCEV[4\x80\x15a\x06fW`\0\x80\xFD[Pa\x03\xCEa\x06u6`\x04a-\x11V[a\x19\xC0V[4\x80\x15a\x06\x86W`\0\x80\xFD[Pa\x04\x12`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fv1.0.17`\xC8\x1B\x81RP\x81V[4\x80\x15a\x06\xB9W`\0\x80\xFD[Pa\x05\x18`\x0BT\x81V[4\x80\x15a\x06\xCFW`\0\x80\xFD[P`\x02Ta\x04]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xEFW`\0\x80\xFD[Pa\x03\xCEa\x06\xFE6`\x04a-\x11V[a\x1A\x1DV[4\x80\x15a\x07\x0FW`\0\x80\xFD[Pa\x073a\x07\x1E6`\x04a.\x14V[`\"` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x04\x1FV[4\x80\x15a\x07OW`\0\x80\xFD[P`!Ta\x05\x18V[4\x80\x15a\x07dW`\0\x80\xFD[Pa\x03\xCEa\x07s6`\x04a-\x11V[a\x1A\x9FV[4\x80\x15a\x07\x84W`\0\x80\xFD[Pa\x03\xCEa\x07\x936`\x04a-\x11V[a\x1BCV[4\x80\x15a\x07\xA4W`\0\x80\xFD[Pa\x03\xCEa\x07\xB36`\x04a-\x11V[a\x1B\xC5V[4\x80\x15a\x07\xC4W`\0\x80\xFD[Pa\x03\xCEa\x07\xD36`\x04a.\x14V[a\x1CGV[4\x80\x15a\x07\xE4W`\0\x80\xFD[Pa\x03\xCEa\x07\xF36`\x04a.\x14V[a\x1C\xBBV[4\x80\x15a\x08\x04W`\0\x80\xFD[Pa\x03\xCEa\x08\x136`\x04a-\x11V[a\x1D/V[4\x80\x15a\x08$W`\0\x80\xFD[Pa\x05\x18`\x0ET\x81V[4\x80\x15a\x08:W`\0\x80\xFD[Pa\x03\xCEa\x08I6`\x04a-\x11V[a\x1D\xD3V[4\x80\x15a\x08ZW`\0\x80\xFD[P`\x05Ta\x04]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x08zW`\0\x80\xFD[Pa\x03\xCEa\x08\x896`\x04a-*V[a\x1EUV[4\x80\x15a\x08\x9AW`\0\x80\xFD[Pa\x04\xF5a\x1E\xBDV[4\x80\x15a\x08\xAFW`\0\x80\xFD[Pa\x03\xCEa\x08\xBE6`\x04a.\x14V[a\x1F\x1FV[4\x80\x15a\x08\xCFW`\0\x80\xFD[Pa\x03\xCEa\x08\xDE6`\x04a.\x14V[a\x1F\x93V[4\x80\x15a\x08\xEFW`\0\x80\xFD[Pa\x04\xF5a\x08\xFE6`\x04a2\x8FV[a \x07V[4\x80\x15a\t\x0FW`\0\x80\xFD[Pa\x05\x18`\x12T\x81V[4\x80\x15a\t%W`\0\x80\xFD[Pa\t.a!\tV[`@Qa\x04\x1F\x91\x90a2\xB1V[4\x80\x15a\tGW`\0\x80\xFD[P`\x07Ta\x04]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\tgW`\0\x80\xFD[Pa\x05\x18`\x06T\x81V[4\x80\x15a\t}W`\0\x80\xFD[Pa\x05\x18`\nT\x81V[4\x80\x15a\t\x93W`\0\x80\xFD[Pa\x05\x18`\x11T\x81V[4\x80\x15a\t\xA9W`\0\x80\xFD[Pa\x03\xCEa\t\xB86`\x04a.\x14V[a!aV[4\x80\x15a\t\xC9W`\0\x80\xFD[Pa\x05\x18`\x13T\x81V[4\x80\x15a\t\xDFW`\0\x80\xFD[Pa\x04]a\t\xEE6`\x04a-\x11V[a!\xD5V[4\x80\x15a\t\xFFW`\0\x80\xFD[Pa\x05\x18`\rT\x81V[4\x80\x15a\n\x15W`\0\x80\xFD[Pa\x03\xCEa\n$6`\x04a.\x14V[a\"\x05V[4\x80\x15a\n5W`\0\x80\xFD[Pa\x05\x18`\x0CT\x81V[4\x80\x15a\nKW`\0\x80\xFD[P`\x1ETa\x05\x18V[4\x80\x15a\n`W`\0\x80\xFD[P`\x04Ta\x04]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\n\x80W`\0\x80\xFD[P`\x03Ta\x04]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\n\xA0W`\0\x80\xFD[Pa\x03\xCEa\n\xAF6`\x04a-\x11V[a\"yV[4\x80\x15a\n\xC0W`\0\x80\xFD[Pa\t.a#\x1DV[4\x80\x15a\n\xD5W`\0\x80\xFD[Pa\x03\xCEa\n\xE46`\x04a-\x11V[a#uV[4\x80\x15a\n\xF5W`\0\x80\xFD[Pa\x04\xF5a\x0B\x046`\x04a2\x8FV[a$\rV[4\x80\x15a\x0B\x15W`\0\x80\xFD[P`\tTa\x04]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x0B5W`\0\x80\xFD[Pa\x073a\x0BD6`\x04a.\x14V[`\x1F` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x0BeW`\0\x80\xFD[Pa\x04]a\x0Bt6`\x04a-\x11V[a%\x0FV[`\0\x80Ta\x0B\x86\x90a2\xDCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xB2\x90a2\xDCV[\x80\x15a\x0B\xFFW\x80`\x1F\x10a\x0B\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xFFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xE2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C1W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x80\x81\x01\x82R`\x19T\x80\x82R`\x1AT` \x83\x01R`\x1BT\x92\x82\x01\x92\x90\x92R`\x1CT``\x82\x01R\x90\x825\x11\x80a\x0CqWP\x80` \x01Q\x82` \x015\x11[\x80a\x0C\x83WP\x80`@\x01Q\x82`@\x015\x11[\x80a\x0C\x95WP\x80``\x01Q\x82``\x015\x11[\x15a\x0C\xB3W`@Qc\x15\xB0Z\x8F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x815`\x15\x81\x90U` \x80\x84\x015`\x16\x81\x90U`@\x80\x86\x015`\x17\x81\x90U``\x80\x88\x015`\x18\x81\x90U\x83Q\x96\x87R\x94\x86\x01\x93\x90\x93R\x90\x84\x01R\x82\x01R\x7F\xE1\xC4_\x8A\xEBT?0\xB3|\xC2\xFC\xCF\xBA\xC0\xF3,\xC8\xF24(M\xF9!\xD7\x1C\xFF\x04\xE5\x1E\xF4!\x90`\x80\x01[`@Q\x80\x91\x03\x90\xA1PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\rJW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x0BTa\rX\x91\x90a3\x10V[\x15\x15\x80a\rqWP\x80`\x0CTa\rn\x91\x90a3\x10V[\x15\x15[\x80a\r\x88WP\x80`\rTa\r\x85\x91\x90a3\x10V[\x15\x15[\x80a\r\x9FWP\x80`\x0ETa\r\x9C\x91\x90a3\x10V[\x15\x15[\x15a\r\xBDW`@Qc\x11\xB7\\\x15`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n\x81\x90U`@Q\x81\x81R\x7F\x04\xED\x83[H\x8BO\xCF\n!*F\xEDg\xCB\xBF\xFC/\xC8\x1B\\\xB6\xA1,Ter\xCB\xF7\xB7\xE0j\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E#W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x12T\x81\x11\x15a\x0EFW`@Qc\x16p\xF7\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x11\x81\x90U`@Q\x81\x81R\x7F\xAA\xB0\xEE\x91\0b\x9C@ZMu3n\x16@\xCC\x81\xE0`\x8F\xB0\xD7\xF1s\x89\xC0n\xE8\xD4\xF0!\x91\x90` \x01a\r\xEEV[`!T``\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x99Wa\x0E\x99a.1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xC2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0FNW` `\0\x85\x85\x84\x81\x81\x10a\x0E\xE6Wa\x0E\xE6a32V[\x90P` \x02\x01` \x81\x01\x90a\x0E\xFB\x91\x90a.\x14V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x92\x90\x92R`@\x01`\0 T\x83Q\x91\x16\x90\x83\x90\x83\x90\x81\x10a\x0F.Wa\x0F.a32V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x0E\xC8V[P[\x92\x91PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\x80W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x80\x81\x01\x82R`\x15T\x81R`\x16T` \x82\x01R`\x17T\x91\x81\x01\x91\x90\x91R`\x18T``\x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0\x825\x11\x80a\x0F\xCBWPg\r\xE0\xB6\xB3\xA7d\0\0\x82` \x015\x11[\x80a\x0F\xE1WPg\r\xE0\xB6\xB3\xA7d\0\0\x82`@\x015\x11[\x80a\x0F\xF7WPg\r\xE0\xB6\xB3\xA7d\0\0\x82``\x015\x11[\x80a\x10\x03WP\x80Q\x825\x10[\x80a\x10\x15WP\x80` \x01Q\x82` \x015\x10[\x80a\x10'WP\x80`@\x01Q\x82`@\x015\x10[\x80a\x109WP\x80``\x01Q\x82``\x015\x10[\x15a\x10WW`@Qc\x16\x10q\xFB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x815`\x19\x81\x90U` \x80\x84\x015`\x1A\x81\x90U`@\x80\x86\x015`\x1B\x81\x90U``\x80\x88\x015`\x1C\x81\x90U\x83Q\x96\x87R\x94\x86\x01\x93\x90\x93R\x90\x84\x01R\x82\x01R\x7F\x8C`\x93\xC7\xE6]\xD8b\xE8\x81bw\x0CN\x15n\x8A\r\xA5}%\xD9a\xE0\xFBo(\xCF\xB7\xFF\x89\xA7\x90`\x80\x01a\r\x14V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\xE6W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10T\x81\x11\x15a\x11\tW`@Qc(\xA2\xD9\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0F\x81\x90U`@Q\x81\x81R\x7FA\xF7)\xB91\xAD\x8E3\xD6\x97\xFF\xF6\xBCg6\xA1\xACn\xE0\x9E\x82e\xEF\xAE'\x94\xAC\x165\xC2\x17\xA6\x90` \x01a\r\xEEV[`\x01`\x01`\xA0\x1B\x03\x89\x16`\0\x90\x81R`\x1F` R`@\x81 T`\xFF\x16a\x11wW`@Qcnb?\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11\x82\x88\x86\x86a%$V[`@\x80Q3` \x80\x83\x01\x91\x90\x91R\x81\x83\x01\x8E\x90R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x93\x84\x90R\x80Q\x91\x01 c\x1D2\x1Du`\xE3\x1B\x90\x92R`\0\x91`\x01`\x01`\xA0\x1B\x03\x8D\x16\x91c\xE9\x90\xEB\xA8\x91a\x11\xDF\x91\x8E\x90\x8E\x90\x8E\x90\x8A\x90`d\x01a4YV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\"\x91\x90a4\xA9V[\x90P\x8A` `\0\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\x04`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x89a\x01@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x8A`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB2[\x0F\x0F\x93 \x9B\xE0\x81R\x12/\x13!\xF6\xB0\xEFU\x9A\x93\xA6v\x95\xFF\xF5\xFE\xA3\xE5\xED#De\x82\x8C\x8C\x8C`@Qa\x12\xE1\x94\x93\x92\x91\x90a4\xC6V[`@Q\x80\x91\x03\x90\xA2`!\x80T`\x01\x80\x82\x01\x90\x92U\x7F:cW\x01,\x1A:\xE0\xA1}0L\x99 1\x03\x82\xD9h\xEB\xCCK\x17q\xF4\x1Ck0B\x05\xB5p\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x90\x92U`\0\x90\x81R`\"` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x17\x90U\x92\x80U\x80Q3\x93\x81\x01\x93\x90\x93R\x82\x01\x8E\x90R\x8C\x16\x90c\x16\xAB\xFCp\x904\x90``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 3\x8B\x8B\x8A`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xAA\x95\x94\x93\x92\x91\x90a5\x18V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x13\xC8W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xED\x91\x90a5{V[P`\x02`\x01U`\0[`\x1DT\x81\x10\x15a\x14\x9AW\x81`\x01`\x01`\xA0\x1B\x03\x16cq\x80\xC8\xCA`\x1D\x83\x81T\x81\x10a\x14\"Wa\x14\"a32V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`\x01`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14vW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\x8AW=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x13\xF6\x90PV[P`\x04\x80T`@Qc\xAB\x03>\xA9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x81\x01\x92\x90\x92R\x82\x16\x90c\xAB\x03>\xA9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\xE4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\xF8W=`\0\x80>=`\0\xFD[PG\x92PP\x81\x15\x90Pa\x15pW`@Q`\0\x903\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x15GW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x15LV[``\x91P[PP\x90P\x80a\x15nW`@Qc\x12\x17\x1D\x83`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[P\x9B\x9APPPPPPPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x15\xAAW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xAE\x06/\xB8,\x93,e<\xD4F\x174>\xCD\xA1\xD1>7^\ro \xD9i\xC9D\xFB\xDA\x19c\xD3\x90`\0\x90\xA2PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15\x90a\x16\x1AWP`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x167W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x1F` R`@\x90 T`\xFF\x16a\x16pW`@QcK\xF1!\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16`\x1E\x82\x81T\x81\x10a\x16\x8DWa\x16\x8Da32V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\xC0W`@Qc\x0F'\0\xCB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x1F` R`@\x90 \x80T`\xFF\x19\x16\x90U`\x1E\x80Ta\x16\xF0\x90`\x01\x90a5\xAAV[\x81T\x81\x10a\x17\0Wa\x17\0a32V[`\0\x91\x82R` \x90\x91 \x01T`\x1E\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x83\x90\x81\x10a\x17,Wa\x17,a32V[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\x1E\x80T\x80a\x17kWa\x17ka5\xBDV[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x90\x91\x01\x90\x91U`@Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x7Fp\x9BtP\xBF\xAF\xDA\x93\xEF\xD9\x1D)\x14\x98p\xA7\x94cz\xC9\xD6\x96\xCAab_\xD2\xF55H\xAF\xE0\x91\xA2PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15\x90a\x17\xEBWP`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x18\x08W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x1F` R`@\x90 T`\xFF\x16\x15a\x18BW`@Qc\xBD4cO`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x1F` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U`\x1E\x80T\x91\x82\x01\x81U\x83R\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90UQ\x7F\x16\xCE\x88(\\\xFDY\x82\x9AZ\xA0Cp\xA5\xEC\x80\x90\xA1\x8C\x14\xE7\xE7\xFB\x9DK\x12\xA4\"\x91\xC0\x98\xE3\x91\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x1F` R`@\x81 T`\xFF\x16a\x19\x07W`@Qcnb?\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x19\x12\x87\x86\x86a%$V[`@\x80Q3` \x80\x83\x01\x91\x90\x91R\x81\x83\x01\x8C\x90R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x93\x84\x90R\x80Q\x91\x01 c\x18* \xC3`\xE3\x1B\x90\x92R`\0\x91`\x01`\x01`\xA0\x1B\x03\x8B\x16\x91c\xC1Q\x06\x18\x91a\x19o\x91\x8C\x90\x8C\x90\x8A\x90\x8A\x90`d\x01a5\xD3V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x19\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xB2\x91\x90a4\xA9V[\x9A\x99PPPPPPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\xEAW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06\x81\x90U`@Q\x81\x90\x7F9Za%\x907)\x8D\x1CL\xD4\xBF\x17{d\xADY\x95\xD3\x8A\x93\x94W?\xCD\x90`\xD6I1J\xD0\x90`\0\x90\xA2PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1AGW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x13T\x81\x10\x15a\x1AjW`@Qc\n5S\x9D`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x14\x81\x90U`@Q\x81\x81R\x7F\xA9\xE7\x96\x1B\xB34'\x15\xDB\xEC\xC2\x08\x08zj\x9D\xF8\x98mRK:\n\x82\x9F\xD9\x0FZ/[\xA5>\x90` \x01a\r\xEEV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\xC9W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT\x81\x10\x80a\x1A\xE4WP`\nTa\x1A\xE1\x90\x82a3\x10V[\x15\x15[\x80a\x1A\xF0WP`\rT\x81\x11[\x15a\x1B\x0EW`@Qc\xF9\xC0\x95\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x81\x90U`@Q\x81\x81R\x7F1@}\xDD\x17\"\xF5\0\xB8\xAA,\x18\xE1\x129\x86&\xDD|(i\xA5\xF8\x071\xEC0\xB2D\xD9\xB5\xF2\x90` \x01a\r\xEEV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1BmW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x14T\x81\x11\x15a\x1B\x90W`@QcZ\x8FeW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x13\x81\x90U`@Q\x81\x81R\x7FI\x0E\xB2\xA9\x17F\xAA\x93<\x9F\xFE/y9\xAA\x06I\x8F2Y\x13\x9F\x88\x05\xF0\x08\xB1,\x8CizF\x90` \x01a\r\xEEV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1B\xEFW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0FT\x81\x10\x15a\x1C\x12W`@Qc~\xDC\x06\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10\x81\x90U`@Q\x81\x81R\x7F\x0EV\xD8?T\xE6\xF5\xB0\x87\x16\xA5K:\xBD\xB5\x9B0%\xBF\x12\xC1\x87\\\x87\xAB\x98\xAB\x08\x1Do\x83\x81\x90` \x01a\r\xEEV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1CqW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\x03\xAA[\x0F\xB6P\x14\xEE\xA8\x9F\xDA\x04\xA7\xBC\x11t \x14\x88\x1F<\x07\x8F,u\xB7\"l\xE1\r\x94\x18\x90`\0\x90\xA2PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C\xE5W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xC0I\x05\x8B\x1D\xF2\xDD\x89\x02s\x9C\xEBx\x99-\xF1/\xA86\x9C\x06\xC4P\xB3\xC6xq7\xB4R\xFD\xD2\x90`\0\x90\xA2PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1DYW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT\x81\x10\x80a\x1DtWP`\nTa\x1Dq\x90\x82a3\x10V[\x15\x15[\x80a\x1D\x80WP`\x0CT\x81\x11[\x15a\x1D\x9EW`@Qc\x02\x19\xD6c`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0B\x81\x90U`@Q\x81\x81R\x7Fo\x81u\xCD\xBA\xC1\xB4\xD28\xAB\xBA$\xA1}%T\xD7\xB9u\x0B\xBE\xDAd\x14\xE1\x91\xC4x8Kv1\x90` \x01a\r\xEEV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1D\xFDW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x11T\x81\x10\x15a\x1E W`@Qc\x01\x9C\xFB{`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x12\x81\x90U`@Q\x81\x81R\x7FO\xA3\\\x11\xCE\x9AE\xAE\x88,\x15N\xCBS\xAB\\\xACR\xA7J[\x9B\x03s\xBE6\xDE\xB3\x0Cx)x\x90` \x01a\r\xEEV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E\x7FW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\x8B`\x1D\x83\x83a,7V[P\x7F*\x85'l\xF6\x04\xA3\x82.\x19\xB2\x9A>\x97\xAE\xBF\xBCG\xA1\x90%\xC2\xE8\xF6\xE8\x0B:\xF7t\xDC\xBC8\x82\x82`@Qa\r\x14\x92\x91\x90a6\x12V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1F\x15W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x1E\xF7W[PPPPP\x90P\x90V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1FIW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7Foq\x7F\xB0\xABQ m\xEA@d\xA3\\\x94\xC2xO\x87\x14\xB0\x12\xFB\xDE\x82\x0E\r\xDE\xE3be\xEBj\x90`\0\x90\xA2PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1F\xBDW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\x9D>R.\x1EG\xA2\xF6\0\x9794+\x9C\xC7\xB2R\xA1\x88\x81T\xE8C\xABU\xEE\x1C\x81tW\x95\xAB\x90`\0\x90\xA2PV[``\x81\x83\x10a )W`@Qc;'5\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`!T\x82\x11\x15a LW`@Qc\xE0\xF7\xBE\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a V\x83\x83a5\xAAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a nWa na.1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \x97W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82[\x82\x81\x10\x15a\x0FNW`!\x81\x81T\x81\x10a \xB7Wa \xB7a32V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x85\x83\x03\x81Q\x81\x10a \xE9Wa \xE9a32V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a \x9CV[a!4`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P`@\x80Q`\x80\x81\x01\x82R`\x15T\x81R`\x16T` \x82\x01R`\x17T\x91\x81\x01\x91\x90\x91R`\x18T``\x82\x01R\x90V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a!\x8BW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xE5i9\x14\xD1\x9Cx\x9B\xDE\xE5\n6)\x98\xC0\xBC\x8D\x03Z\x83_\x98q\xDA]Q\x15/\x05\x82\xC3O\x90`\0\x90\xA2PV[`\0`!\x82\x81T\x81\x10a!\xEAWa!\xEAa32V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\"/W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xF3\xE0{K\xB49O/\xF3 \xBD\x1D\xD1QU\x1D\xFF0M^\x94\x8B@\x1D\x85X\xB2(H,\x97\xD8\x90`\0\x90\xA2PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\"\xA3W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0CT\x81\x10\x80a\"\xBEWP`\nTa\"\xBB\x90\x82a3\x10V[\x15\x15[\x80a\"\xCAWP`\x0ET\x81\x11[\x15a\"\xE8W`@Qc0\x07\xAD\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\r\x81\x90U`@Q\x81\x81R\x7F\xE9\xEF>\x93\xDF\xF7\x99\xD4\xDB\x8A\x12\xFFy\xE0\x91\x8AZx\xD7[\x10Rxd\xF4\xB1\xC9 \xF6\xF4\xF1x\x90` \x01a\r\xEEV[a#H`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P`@\x80Q`\x80\x81\x01\x82R`\x19T\x81R`\x1AT` \x82\x01R`\x1BT\x91\x81\x01\x91\x90\x91R`\x1CT``\x82\x01R\x90V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a#\x9FW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\rT\x81\x10\x80a#\xBAWP`\nTa#\xB7\x90\x82a3\x10V[\x15\x15[\x15a#\xD8W`@Qc\xCF\xB6\x99\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0E\x81\x90U`@Q\x81\x81R\x7F\x86o\xE9H_\x99\x83\xAF\xCE\xAA\x13\x850{n\xB0\xFD=\xF5\xA2P\xAE+\x0B\xF7m\xC9\xDD\xD3\x16\x92k\x90` \x01a\r\xEEV[``\x81\x83\x10a$/W`@Qc;'5\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x1ET\x82\x11\x15a$RW`@Qc\xE0\xF7\xBE\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$\\\x83\x83a5\xAAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$tWa$ta.1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$\x9DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82[\x82\x81\x10\x15a\x0FNW`\x1E\x81\x81T\x81\x10a$\xBDWa$\xBDa32V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x85\x83\x03\x81Q\x81\x10a$\xEFWa$\xEFa32V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a$\xA2V[`\0`\x1E\x82\x81T\x81\x10a!\xEAWa!\xEAa32V[`\x0BT\x83a\x01\0\x01Q\x10\x80a%?WP`\x0CT\x83a\x01\0\x01Q\x11[\x80a%[WP`\nT\x83a\x01\0\x01Qa%X\x91\x90a3\x10V[\x15\x15[\x15a%yW`@QcT(sM`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\rT\x83`\xE0\x01Q\x10\x80a%\x92WP`\x0ET\x83`\xE0\x01Q\x11[\x80a%\xB0WP\x82a\x01\0\x01Q\x83`\xE0\x01Qa%\xAD\x91\x90a3\x10V[\x15\x15[\x15a%\xCEW`@Qc%?\xFF\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0FT\x83`\xC0\x01Q\x10\x80a%\xE7WP`\x10T\x83`\xC0\x01Q\x11[\x15a&\x05W`@Qc\xEF\x9B\xC6_`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x19Ta\x01\xC0\x84\x01QQ\x11\x80a&9WP`\x1AT`\xE0\x84\x01Qa\x01\xC0\x85\x01Q` \x01Qa&7\x91c\x01\xE13\x80\x90a(\x95V[\x11[\x80a&NWP`\x1BTa\x01\xC0\x84\x01Q`@\x01Q\x11[\x80a&cWP`\x1CTa\x01\xC0\x84\x01Q``\x01Q\x11[\x80a&uWP`\x15Ta\x01\xC0\x84\x01QQ\x10[\x80a&\x9EWP`\x16T`\xE0\x84\x01Qa\x01\xC0\x85\x01Q` \x01Qa&\x9C\x91c\x01\xE13\x80\x90a(\xBBV[\x10[\x80a&\xB3WP`\x17Ta\x01\xC0\x84\x01Q`@\x01Q\x10[\x80a&\xC8WP`\x18Ta\x01\xC0\x84\x01Q``\x01Q\x10[\x15a&\xE6W`@Qc-\x87h\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x11T\x82\x10\x80a&\xF7WP`\x12T\x82\x11[\x15a'\x15W`@Qc0UM\xE1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a':f\x11\xC3y7\xE0\x80\0a'4\x85g\x1B\xC1mgN\xC8\0\0a(\xD9V[\x90a(\xF5V[`\x13T\x90\x91Pa'J\x90\x82a(\xF5V[\x82\x10\x80a'}WPa'za'qg\x1B\xC1mgN\xC8\0\0a'k\x86\x85a(\xF5V[\x90a)\x0BV[`\x14T\x90a) V[\x82\x11[\x15a'\x9BW`@Qc\x83\xEB\xDF\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a'\xAB\x83\x86`\xE0\x01Qa)5V[`\x05T`@\x87\x01Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x15\x80a'\xD6WP`\x06T\x85``\x01Q\x14\x15[\x80a'\xF5WP`\x07Ta\x01`\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x15[\x80a(\x14WP`\x08Ta\x01\x80\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x15[\x80a(3WP`\tTa\x01\xA0\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x15[\x80a(RWP`\x04Ta\x01@\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x15[\x80a(aWPa\x01 \x85\x01Q\x15\x15[\x15a(\x7FW`@Qc\xE8\xC0-\xD7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[0a\x01@\x86\x01Ra\x01 \x90\x94\x01\x93\x90\x93RPPPV[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a(\xACW`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a(\xD2W`\0\x80\xFD[P\x91\x02\x04\x90V[`\0a(\xEE\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a(\xBBV[\x93\x92PPPV[`\0\x81\x83\x11a)\x04W\x81a(\xEEV[P\x90\x91\x90PV[`\0a(\xEE\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a(\xBBV[`\0\x81\x83\x11a)/W\x82a(\xEEV[P\x91\x90PV[`\0\x80a)fa)Wa)I\x86`da6`V[f\xA5\xBB\xED\x86\xC5\xA0\0\x90a)\x0BV[gH\xCD@r(\x1E\0\0\x90a(\xD9V[\x90Pa)zg\r\xE0\xB6\xB3\xA7d\0\0\x82a(\xD9V[\x90Pa)\xD2\x81a'ka)\xA5a)\xA0a)\x9B\x89g\r\xE0\xB6\xB3\xA7d\0\0a6wV[a)\xDAV[a*\x08V[a)\xCCa)\xA0a)\xBA\x8A\x8Ac\x01\xE13\x80a(\xBBV[a)\x9B\x90g\r\xE0\xB6\xB3\xA7d\0\0a6wV[\x90a(\xD9V[\x94\x93PPPPV[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a*\x04W`@Qc9n\xA7\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0\x80\x82\x13a**W`@Qc\xE6\x1BIu`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x82\x81\x1C`\x0F\x10`\x02\x1B\x17\x82\x81\x1C\x90\x91\x10`\x01\x90\x81\x1B\x90\x91\x17\x82\x81\x1C\x90\x91\x10\x17`\x9F\x81\x81\x03``\x01\x92\x90\x92\x1B\x91`_\x19\x82\x01\x90a*\xB6\x90\x84\x90\x1Ca)\xDAV[lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x91\x90\x91\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a,\x8AW\x91` \x02\x82\x01[\x82\x81\x11\x15a,\x8AW\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x845\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a,WV[Pa*\x04\x92\x91P[\x80\x82\x11\x15a*\x04W`\0\x81U`\x01\x01a,\x92V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a,\xCCW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a,\xB0V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a(\xEE` \x83\x01\x84a,\xA6V[`\0`\x80\x82\x84\x03\x12\x15a)/W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a-#W`\0\x80\xFD[P5\x91\x90PV[`\0\x80` \x83\x85\x03\x12\x15a-=W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a-UW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a-iW`\0\x80\xFD[\x815\x81\x81\x11\x15a-xW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a-\x8DW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a-\xE0W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a-\xBBV[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a.\x01W`\0\x80\xFD[PV[\x805a.\x0F\x81a-\xECV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a.&W`\0\x80\xFD[\x815a(\xEE\x81a-\xECV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.kWa.ka.1V[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a.\x82W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a.\x9DWa.\x9Da.1V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a.\xC5Wa.\xC5a.1V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a.\xDEW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a/\x10W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a/3Wa/3a.1V[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[`\0a\x02@\x82\x84\x03\x12\x15a/wW`\0\x80\xFD[a/\x7Fa.GV[\x90Pa/\x8A\x82a.\x04V[\x81Ra/\x98` \x83\x01a.\x04V[` \x82\x01Ra/\xA9`@\x83\x01a.\x04V[`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015`\x80\x82\x01R`\xA0\x82\x015`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R`\xE0\x82\x015`\xE0\x82\x01Ra\x01\0\x80\x83\x015\x81\x83\x01RPa\x01 \x80\x83\x015\x81\x83\x01RPa\x01@a0\x06\x81\x84\x01a.\x04V[\x90\x82\x01Ra\x01`a0\x18\x83\x82\x01a.\x04V[\x90\x82\x01Ra\x01\x80a0*\x83\x82\x01a.\x04V[\x90\x82\x01Ra\x01\xA0a0<\x83\x82\x01a.\x04V[\x90\x82\x01Ra\x01\xC0a0O\x84\x84\x83\x01a.\xFEV[\x90\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a0kW`\0\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a0\x8FWa0\x8Fa.1V[\x81`@R\x82\x93P\x845\x91Pa0\xA3\x82a-\xECV[\x90\x82R` \x84\x015\x90\x81\x15\x15\x82\x14a0\xBAW`\0\x80\xFD[\x81` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15a0\xD4W`\0\x80\xFD[Pa0\xE1\x85\x82\x86\x01a.qV[`@\x83\x01RPP\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x03`\x8B\x8D\x03\x12\x15a1\x0EW`\0\x80\xFD[\x8A5\x99Pa1\x1E` \x8C\x01a.\x04V[\x98P`@\x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a1;W`\0\x80\xFD[a1G\x8E\x83\x8F\x01a.qV[\x99Pa1V\x8E``\x8F\x01a/dV[\x98Pa\x02\xA0\x8D\x015\x91P\x80\x82\x11\x15a1mW`\0\x80\xFD[a1y\x8E\x83\x8F\x01a.qV[\x97Pa\x02\xC0\x8D\x015\x96Pa\x02\xE0\x8D\x015\x95Pa\x03\0\x8D\x015\x94Pa\x03 \x8D\x015\x91P\x80\x82\x11\x15a1\xA8W`\0\x80\xFD[Pa1\xB5\x8D\x82\x8E\x01a0YV[\x92PPa\x03@\x8B\x015\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0\x80`@\x83\x85\x03\x12\x15a1\xE2W`\0\x80\xFD[\x825a1\xED\x81a-\xECV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x03 \x89\x8B\x03\x12\x15a2\x18W`\0\x80\xFD[\x885\x97P` \x89\x015a2*\x81a-\xECV[\x96Pa29\x8A`@\x8B\x01a/dV[\x95Pa\x02\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2VW`\0\x80\xFD[a2b\x8B\x82\x8C\x01a.qV[\x98\x9B\x97\x9AP\x95\x98a\x02\xA0\x81\x015\x97a\x02\xC0\x82\x015\x97Pa\x02\xE0\x82\x015\x96Pa\x03\0\x90\x91\x015\x94P\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a2\xA2W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x81\x01a\x0FPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a2\xF0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a)/WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0\x82a3-WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x01Qa3o` \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x81\x01Qa3\x8A`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x81\x01Q``\x83\x01R`\x80\x81\x01Q`\x80\x83\x01R`\xA0\x81\x01Q`\xA0\x83\x01R`\xC0\x81\x01Q`\xC0\x83\x01R`\xE0\x81\x01Q`\xE0\x83\x01Ra\x01\0\x80\x82\x01Q\x81\x84\x01RPa\x01 \x80\x82\x01Q\x81\x84\x01RPa\x01@\x80\x82\x01Qa3\xF0\x82\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[PPa\x01`\x81\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x84\x01\x91\x90\x91Ra\x01\x80\x80\x83\x01Q\x82\x16\x90\x84\x01Ra\x01\xA0\x80\x83\x01Q\x90\x91\x16\x90\x83\x01Ra\x01\xC0\x90\x81\x01Q\x80Q\x91\x83\x01\x91\x90\x91R` \x81\x01Qa\x01\xE0\x83\x01R`@\x81\x01Qa\x02\0\x83\x01R``\x01Qa\x02 \x90\x91\x01RV[`\0a\x02\xC0\x87\x83R\x80` \x84\x01Ra4s\x81\x84\x01\x88a,\xA6V[\x90Pa4\x82`@\x84\x01\x87a3HV[\x82\x81\x03a\x02\x80\x84\x01Ra4\x95\x81\x86a,\xA6V[\x91PP\x82a\x02\xA0\x83\x01R\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a4\xBBW`\0\x80\xFD[\x81Qa(\xEE\x81a-\xECV[`\x01`\x01`\xA0\x1B\x03\x85\x16\x81Ra\x02\xA0` \x82\x01\x81\x90R`\0\x90a4\xEB\x83\x82\x01\x87a,\xA6V[\x90Pa4\xFA`@\x84\x01\x86a3HV[\x82\x81\x03a\x02\x80\x84\x01Ra5\r\x81\x85a,\xA6V[\x97\x96PPPPPPPV[\x85\x81R`\0`\x01\x80`\xA0\x1B\x03\x80\x87\x16` \x84\x01R\x85`@\x84\x01R\x84``\x84\x01R`\xA0`\x80\x84\x01R\x80\x84Q\x16`\xA0\x84\x01RP` \x83\x01Q\x15\x15`\xC0\x83\x01R`@\x83\x01Q```\xE0\x84\x01Ra5oa\x01\0\x84\x01\x82a,\xA6V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a5\x8DW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0FPWa\x0FPa5\x94V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0a\x02\xC0\x87\x83Ra5\xE8` \x84\x01\x88a3HV[\x80a\x02`\x84\x01Ra5\xFB\x81\x84\x01\x87a,\xA6V[a\x02\x80\x84\x01\x95\x90\x95RPPa\x02\xA0\x01R\x93\x92PPPV[` \x80\x82R\x81\x81\x01\x83\x90R`\0\x90\x84`@\x84\x01\x83[\x86\x81\x10\x15a6UW\x825a6:\x81a-\xECV[`\x01`\x01`\xA0\x1B\x03\x16\x82R\x91\x83\x01\x91\x90\x83\x01\x90`\x01\x01a6'V[P\x96\x95PPPPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0FPWa\x0FPa5\x94V[\x80\x82\x01\x80\x82\x11\x15a\x0FPWa\x0FPa5\x94V\xFE\xA2dipfsX\"\x12 \x97\x13\x02\xAB,\xEC\x02\xD5^\xFBI\x8E=\xCE\x0C\x85\xFEwsc\xFE\xCF\xB7\x8BgF\xD2n\xE19\xC2\xD5dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static HYPERDRIVEFACTORY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x03\xA6W`\x005`\xE0\x1C\x80c\x8E\x12|\xF5\x11a\x01\xE7W\x80c\xD2\xC3\\\xE8\x11a\x01\rW\x80c\xE4\xE7\x14\x8F\x11a\0\xA0W\x80c\xEC\x89_\x11\x11a\0oW\x80c\xEC\x89_\x11\x14a\n\xE9W\x80c\xF2YdX\x14a\x0B\tW\x80c\xF8\xC0\x9EY\x14a\x0B)W\x80c\xFE=Z\xEB\x14a\x0BYW`\0\x80\xFD[\x80c\xE4\xE7\x14\x8F\x14a\ntW\x80c\xE7\x1F4\xB3\x14a\n\x94W\x80c\xE8>4\xB1\x14a\n\xB4W\x80c\xEBq\xF6l\x14a\n\xC9W`\0\x80\xFD[\x80c\xDD+\x8F\xBB\x11a\0\xDCW\x80c\xDD+\x8F\xBB\x14a\n\tW\x80c\xE0\xE2\xDA\xAA\x14a\n)W\x80c\xE1\xB3\x9C\x80\x14a\n?W\x80c\xE33\x15U\x14a\nTW`\0\x80\xFD[\x80c\xD2\xC3\\\xE8\x14a\t\x9DW\x80c\xD6\xF5\x01i\x14a\t\xBDW\x80c\xDA\xAC$\xDA\x14a\t\xD3W\x80c\xDA\xF0\x12\xE6\x14a\t\xF3W`\0\x80\xFD[\x80c\xB2V\x12c\x11a\x01\x85W\x80c\xC4\x15\xB9\\\x11a\x01TW\x80c\xC4\x15\xB9\\\x14a\t;W\x80c\xC9\x05\xA4\xB5\x14a\t[W\x80c\xD0\xF9k\x92\x14a\tqW\x80c\xD2=~\xA3\x14a\t\x87W`\0\x80\xFD[\x80c\xB2V\x12c\x14a\x08\xC3W\x80c\xBC0\xE7\xA1\x14a\x08\xE3W\x80c\xBF\x9B\xD5\xCD\x14a\t\x03W\x80c\xC1r%c\x14a\t\x19W`\0\x80\xFD[\x80c\x99b;\xB1\x11a\x01\xC1W\x80c\x99b;\xB1\x14a\x08NW\x80c\x9A\xF2Rb\x14a\x08nW\x80c\xA6L\x90\xBF\x14a\x08\x8EW\x80c\xA9\x8AF\xDB\x14a\x08\xA3W`\0\x80\xFD[\x80c\x8E\x12|\xF5\x14a\x07\xF8W\x80c\x8E\xFC\t\x86\x14a\x08\x18W\x80c\x97\xB0\xE8\xCE\x14a\x08.W`\0\x80\xFD[\x80cET\xF9\xA9\x11a\x02\xCCW\x80cb\x80'\xA3\x11a\x02jW\x80c\x83\xB3a\xE8\x11a\x029W\x80c\x83\xB3a\xE8\x14a\x07xW\x80c\x84\xC1\x9A\xAB\x14a\x07\x98W\x80c\x85\"\x97\x85\x14a\x07\xB8W\x80c\x86'\xA4\xF0\x14a\x07\xD8W`\0\x80\xFD[\x80cb\x80'\xA3\x14a\x06\xE3W\x80ckD\xE6\xBE\x14a\x07\x03W\x80cn\x95\xD6|\x14a\x07CW\x80com\\J\x14a\x07XW`\0\x80\xFD[\x80cO\xBF\xEEw\x11a\x02\xA6W\x80cO\xBF\xEEw\x14a\x06ZW\x80cT\xFDMP\x14a\x06zW\x80cW \xC9\xD5\x14a\x06\xADW\x80cZ\xA6\xE6u\x14a\x06\xC3W`\0\x80\xFD[\x80cET\xF9\xA9\x14a\x06\x0EW\x80cH\x80\x07`\x14a\x06$W\x80cI\xF1=\xE7\x14a\x06:W`\0\x80\xFD[\x80c\x1E\xCD\xA0\xFE\x11a\x03DW\x80c.|\xD9q\x11a\x03\x13W\x80c.|\xD9q\x14a\x05\x9BW\x80c>- \x14\x14a\x05\xAEW\x80cA\x1C05\x14a\x05\xCEW\x80cB\x1C\xAB\xA8\x14a\x05\xEEW`\0\x80\xFD[\x80c\x1E\xCD\xA0\xFE\x14a\x05\x02W\x80c(\x85\xE3\xAC\x14a\x05&W\x80c)\x07\xD3\xDD\x14a\x05FW\x80c+X\xF4\x18\x14a\x05fW`\0\x80\xFD[\x80c\x10\xD1\xDC>\x11a\x03\x80W\x80c\x10\xD1\xDC>\x14a\x04uW\x80c\x11\xE7{\xFE\x14a\x04\x95W\x80c\x19x\xEB\xCF\x14a\x04\xB5W\x80c\x1BY\xBE\x0C\x14a\x04\xD5W`\0\x80\xFD[\x80c\x04\xBA\xA0\x0B\x14a\x03\xD5W\x80c\x06\xFD\xDE\x03\x14a\x04(W\x80c\x10x\x0Fs\x14a\x04=W`\0\x80\xFD[6a\x03\xD0W`\x02`\x01T\x03a\x03\xCEW`@Qc\n\xACu\xB5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\x03\xE1W`\0\x80\xFD[Pa\x04\x12`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01pHyperdriveFactory`x\x1B\x81RP\x81V[`@Qa\x04\x1F\x91\x90a,\xECV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x044W`\0\x80\xFD[Pa\x04\x12a\x0ByV[4\x80\x15a\x04IW`\0\x80\xFD[P`\x08Ta\x04]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\x1FV[4\x80\x15a\x04\x81W`\0\x80\xFD[Pa\x03\xCEa\x04\x906`\x04a,\xFFV[a\x0C\x07V[4\x80\x15a\x04\xA1W`\0\x80\xFD[Pa\x03\xCEa\x04\xB06`\x04a-\x11V[a\r V[4\x80\x15a\x04\xC1W`\0\x80\xFD[Pa\x03\xCEa\x04\xD06`\x04a-\x11V[a\r\xF9V[4\x80\x15a\x04\xE1W`\0\x80\xFD[Pa\x04\xF5a\x04\xF06`\x04a-*V[a\x0E{V[`@Qa\x04\x1F\x91\x90a-\x9FV[4\x80\x15a\x05\x0EW`\0\x80\xFD[Pa\x05\x18`\x0FT\x81V[`@Q\x90\x81R` \x01a\x04\x1FV[4\x80\x15a\x052W`\0\x80\xFD[Pa\x03\xCEa\x05A6`\x04a,\xFFV[a\x0FVV[4\x80\x15a\x05RW`\0\x80\xFD[Pa\x03\xCEa\x05a6`\x04a-\x11V[a\x10\xBCV[4\x80\x15a\x05rW`\0\x80\xFD[Pa\x04]a\x05\x816`\x04a.\x14V[` \x80R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04]a\x05\xA96`\x04a0\xEEV[a\x11>V[4\x80\x15a\x05\xBAW`\0\x80\xFD[Pa\x03\xCEa\x05\xC96`\x04a.\x14V[a\x15\x80V[4\x80\x15a\x05\xDAW`\0\x80\xFD[Pa\x03\xCEa\x05\xE96`\x04a1\xCFV[a\x15\xF4V[4\x80\x15a\x05\xFAW`\0\x80\xFD[Pa\x03\xCEa\x06\t6`\x04a.\x14V[a\x17\xC5V[4\x80\x15a\x06\x1AW`\0\x80\xFD[Pa\x05\x18`\x10T\x81V[4\x80\x15a\x060W`\0\x80\xFD[Pa\x05\x18`\x14T\x81V[4\x80\x15a\x06FW`\0\x80\xFD[Pa\x04]a\x06U6`\x04a1\xFBV[a\x18\xCEV[4\x80\x15a\x06fW`\0\x80\xFD[Pa\x03\xCEa\x06u6`\x04a-\x11V[a\x19\xC0V[4\x80\x15a\x06\x86W`\0\x80\xFD[Pa\x04\x12`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fv1.0.17`\xC8\x1B\x81RP\x81V[4\x80\x15a\x06\xB9W`\0\x80\xFD[Pa\x05\x18`\x0BT\x81V[4\x80\x15a\x06\xCFW`\0\x80\xFD[P`\x02Ta\x04]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xEFW`\0\x80\xFD[Pa\x03\xCEa\x06\xFE6`\x04a-\x11V[a\x1A\x1DV[4\x80\x15a\x07\x0FW`\0\x80\xFD[Pa\x073a\x07\x1E6`\x04a.\x14V[`\"` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x04\x1FV[4\x80\x15a\x07OW`\0\x80\xFD[P`!Ta\x05\x18V[4\x80\x15a\x07dW`\0\x80\xFD[Pa\x03\xCEa\x07s6`\x04a-\x11V[a\x1A\x9FV[4\x80\x15a\x07\x84W`\0\x80\xFD[Pa\x03\xCEa\x07\x936`\x04a-\x11V[a\x1BCV[4\x80\x15a\x07\xA4W`\0\x80\xFD[Pa\x03\xCEa\x07\xB36`\x04a-\x11V[a\x1B\xC5V[4\x80\x15a\x07\xC4W`\0\x80\xFD[Pa\x03\xCEa\x07\xD36`\x04a.\x14V[a\x1CGV[4\x80\x15a\x07\xE4W`\0\x80\xFD[Pa\x03\xCEa\x07\xF36`\x04a.\x14V[a\x1C\xBBV[4\x80\x15a\x08\x04W`\0\x80\xFD[Pa\x03\xCEa\x08\x136`\x04a-\x11V[a\x1D/V[4\x80\x15a\x08$W`\0\x80\xFD[Pa\x05\x18`\x0ET\x81V[4\x80\x15a\x08:W`\0\x80\xFD[Pa\x03\xCEa\x08I6`\x04a-\x11V[a\x1D\xD3V[4\x80\x15a\x08ZW`\0\x80\xFD[P`\x05Ta\x04]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x08zW`\0\x80\xFD[Pa\x03\xCEa\x08\x896`\x04a-*V[a\x1EUV[4\x80\x15a\x08\x9AW`\0\x80\xFD[Pa\x04\xF5a\x1E\xBDV[4\x80\x15a\x08\xAFW`\0\x80\xFD[Pa\x03\xCEa\x08\xBE6`\x04a.\x14V[a\x1F\x1FV[4\x80\x15a\x08\xCFW`\0\x80\xFD[Pa\x03\xCEa\x08\xDE6`\x04a.\x14V[a\x1F\x93V[4\x80\x15a\x08\xEFW`\0\x80\xFD[Pa\x04\xF5a\x08\xFE6`\x04a2\x8FV[a \x07V[4\x80\x15a\t\x0FW`\0\x80\xFD[Pa\x05\x18`\x12T\x81V[4\x80\x15a\t%W`\0\x80\xFD[Pa\t.a!\tV[`@Qa\x04\x1F\x91\x90a2\xB1V[4\x80\x15a\tGW`\0\x80\xFD[P`\x07Ta\x04]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\tgW`\0\x80\xFD[Pa\x05\x18`\x06T\x81V[4\x80\x15a\t}W`\0\x80\xFD[Pa\x05\x18`\nT\x81V[4\x80\x15a\t\x93W`\0\x80\xFD[Pa\x05\x18`\x11T\x81V[4\x80\x15a\t\xA9W`\0\x80\xFD[Pa\x03\xCEa\t\xB86`\x04a.\x14V[a!aV[4\x80\x15a\t\xC9W`\0\x80\xFD[Pa\x05\x18`\x13T\x81V[4\x80\x15a\t\xDFW`\0\x80\xFD[Pa\x04]a\t\xEE6`\x04a-\x11V[a!\xD5V[4\x80\x15a\t\xFFW`\0\x80\xFD[Pa\x05\x18`\rT\x81V[4\x80\x15a\n\x15W`\0\x80\xFD[Pa\x03\xCEa\n$6`\x04a.\x14V[a\"\x05V[4\x80\x15a\n5W`\0\x80\xFD[Pa\x05\x18`\x0CT\x81V[4\x80\x15a\nKW`\0\x80\xFD[P`\x1ETa\x05\x18V[4\x80\x15a\n`W`\0\x80\xFD[P`\x04Ta\x04]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\n\x80W`\0\x80\xFD[P`\x03Ta\x04]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\n\xA0W`\0\x80\xFD[Pa\x03\xCEa\n\xAF6`\x04a-\x11V[a\"yV[4\x80\x15a\n\xC0W`\0\x80\xFD[Pa\t.a#\x1DV[4\x80\x15a\n\xD5W`\0\x80\xFD[Pa\x03\xCEa\n\xE46`\x04a-\x11V[a#uV[4\x80\x15a\n\xF5W`\0\x80\xFD[Pa\x04\xF5a\x0B\x046`\x04a2\x8FV[a$\rV[4\x80\x15a\x0B\x15W`\0\x80\xFD[P`\tTa\x04]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x0B5W`\0\x80\xFD[Pa\x073a\x0BD6`\x04a.\x14V[`\x1F` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x0BeW`\0\x80\xFD[Pa\x04]a\x0Bt6`\x04a-\x11V[a%\x0FV[`\0\x80Ta\x0B\x86\x90a2\xDCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xB2\x90a2\xDCV[\x80\x15a\x0B\xFFW\x80`\x1F\x10a\x0B\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xFFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xE2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C1W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x80\x81\x01\x82R`\x19T\x80\x82R`\x1AT` \x83\x01R`\x1BT\x92\x82\x01\x92\x90\x92R`\x1CT``\x82\x01R\x90\x825\x11\x80a\x0CqWP\x80` \x01Q\x82` \x015\x11[\x80a\x0C\x83WP\x80`@\x01Q\x82`@\x015\x11[\x80a\x0C\x95WP\x80``\x01Q\x82``\x015\x11[\x15a\x0C\xB3W`@Qc\x15\xB0Z\x8F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x815`\x15\x81\x90U` \x80\x84\x015`\x16\x81\x90U`@\x80\x86\x015`\x17\x81\x90U``\x80\x88\x015`\x18\x81\x90U\x83Q\x96\x87R\x94\x86\x01\x93\x90\x93R\x90\x84\x01R\x82\x01R\x7F\xE1\xC4_\x8A\xEBT?0\xB3|\xC2\xFC\xCF\xBA\xC0\xF3,\xC8\xF24(M\xF9!\xD7\x1C\xFF\x04\xE5\x1E\xF4!\x90`\x80\x01[`@Q\x80\x91\x03\x90\xA1PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\rJW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x0BTa\rX\x91\x90a3\x10V[\x15\x15\x80a\rqWP\x80`\x0CTa\rn\x91\x90a3\x10V[\x15\x15[\x80a\r\x88WP\x80`\rTa\r\x85\x91\x90a3\x10V[\x15\x15[\x80a\r\x9FWP\x80`\x0ETa\r\x9C\x91\x90a3\x10V[\x15\x15[\x15a\r\xBDW`@Qc\x11\xB7\\\x15`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n\x81\x90U`@Q\x81\x81R\x7F\x04\xED\x83[H\x8BO\xCF\n!*F\xEDg\xCB\xBF\xFC/\xC8\x1B\\\xB6\xA1,Ter\xCB\xF7\xB7\xE0j\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E#W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x12T\x81\x11\x15a\x0EFW`@Qc\x16p\xF7\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x11\x81\x90U`@Q\x81\x81R\x7F\xAA\xB0\xEE\x91\0b\x9C@ZMu3n\x16@\xCC\x81\xE0`\x8F\xB0\xD7\xF1s\x89\xC0n\xE8\xD4\xF0!\x91\x90` \x01a\r\xEEV[`!T``\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x99Wa\x0E\x99a.1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xC2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0FNW` `\0\x85\x85\x84\x81\x81\x10a\x0E\xE6Wa\x0E\xE6a32V[\x90P` \x02\x01` \x81\x01\x90a\x0E\xFB\x91\x90a.\x14V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x92\x90\x92R`@\x01`\0 T\x83Q\x91\x16\x90\x83\x90\x83\x90\x81\x10a\x0F.Wa\x0F.a32V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x0E\xC8V[P[\x92\x91PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\x80W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x80\x81\x01\x82R`\x15T\x81R`\x16T` \x82\x01R`\x17T\x91\x81\x01\x91\x90\x91R`\x18T``\x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0\x825\x11\x80a\x0F\xCBWPg\r\xE0\xB6\xB3\xA7d\0\0\x82` \x015\x11[\x80a\x0F\xE1WPg\r\xE0\xB6\xB3\xA7d\0\0\x82`@\x015\x11[\x80a\x0F\xF7WPg\r\xE0\xB6\xB3\xA7d\0\0\x82``\x015\x11[\x80a\x10\x03WP\x80Q\x825\x10[\x80a\x10\x15WP\x80` \x01Q\x82` \x015\x10[\x80a\x10'WP\x80`@\x01Q\x82`@\x015\x10[\x80a\x109WP\x80``\x01Q\x82``\x015\x10[\x15a\x10WW`@Qc\x16\x10q\xFB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x815`\x19\x81\x90U` \x80\x84\x015`\x1A\x81\x90U`@\x80\x86\x015`\x1B\x81\x90U``\x80\x88\x015`\x1C\x81\x90U\x83Q\x96\x87R\x94\x86\x01\x93\x90\x93R\x90\x84\x01R\x82\x01R\x7F\x8C`\x93\xC7\xE6]\xD8b\xE8\x81bw\x0CN\x15n\x8A\r\xA5}%\xD9a\xE0\xFBo(\xCF\xB7\xFF\x89\xA7\x90`\x80\x01a\r\x14V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\xE6W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10T\x81\x11\x15a\x11\tW`@Qc(\xA2\xD9\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0F\x81\x90U`@Q\x81\x81R\x7FA\xF7)\xB91\xAD\x8E3\xD6\x97\xFF\xF6\xBCg6\xA1\xACn\xE0\x9E\x82e\xEF\xAE'\x94\xAC\x165\xC2\x17\xA6\x90` \x01a\r\xEEV[`\x01`\x01`\xA0\x1B\x03\x89\x16`\0\x90\x81R`\x1F` R`@\x81 T`\xFF\x16a\x11wW`@Qcnb?\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11\x82\x88\x86\x86a%$V[`@\x80Q3` \x80\x83\x01\x91\x90\x91R\x81\x83\x01\x8E\x90R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x93\x84\x90R\x80Q\x91\x01 c\x1D2\x1Du`\xE3\x1B\x90\x92R`\0\x91`\x01`\x01`\xA0\x1B\x03\x8D\x16\x91c\xE9\x90\xEB\xA8\x91a\x11\xDF\x91\x8E\x90\x8E\x90\x8E\x90\x8A\x90`d\x01a4YV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\"\x91\x90a4\xA9V[\x90P\x8A` `\0\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\x04`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x89a\x01@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x8A`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB2[\x0F\x0F\x93 \x9B\xE0\x81R\x12/\x13!\xF6\xB0\xEFU\x9A\x93\xA6v\x95\xFF\xF5\xFE\xA3\xE5\xED#De\x82\x8C\x8C\x8C`@Qa\x12\xE1\x94\x93\x92\x91\x90a4\xC6V[`@Q\x80\x91\x03\x90\xA2`!\x80T`\x01\x80\x82\x01\x90\x92U\x7F:cW\x01,\x1A:\xE0\xA1}0L\x99 1\x03\x82\xD9h\xEB\xCCK\x17q\xF4\x1Ck0B\x05\xB5p\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x90\x92U`\0\x90\x81R`\"` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x17\x90U\x92\x80U\x80Q3\x93\x81\x01\x93\x90\x93R\x82\x01\x8E\x90R\x8C\x16\x90c\x16\xAB\xFCp\x904\x90``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 3\x8B\x8B\x8A`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xAA\x95\x94\x93\x92\x91\x90a5\x18V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x13\xC8W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xED\x91\x90a5{V[P`\x02`\x01U`\0[`\x1DT\x81\x10\x15a\x14\x9AW\x81`\x01`\x01`\xA0\x1B\x03\x16cq\x80\xC8\xCA`\x1D\x83\x81T\x81\x10a\x14\"Wa\x14\"a32V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`\x01`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14vW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\x8AW=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x13\xF6\x90PV[P`\x04\x80T`@Qc\xAB\x03>\xA9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x81\x01\x92\x90\x92R\x82\x16\x90c\xAB\x03>\xA9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\xE4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\xF8W=`\0\x80>=`\0\xFD[PG\x92PP\x81\x15\x90Pa\x15pW`@Q`\0\x903\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x15GW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x15LV[``\x91P[PP\x90P\x80a\x15nW`@Qc\x12\x17\x1D\x83`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[P\x9B\x9APPPPPPPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x15\xAAW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xAE\x06/\xB8,\x93,e<\xD4F\x174>\xCD\xA1\xD1>7^\ro \xD9i\xC9D\xFB\xDA\x19c\xD3\x90`\0\x90\xA2PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15\x90a\x16\x1AWP`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x167W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x1F` R`@\x90 T`\xFF\x16a\x16pW`@QcK\xF1!\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16`\x1E\x82\x81T\x81\x10a\x16\x8DWa\x16\x8Da32V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\xC0W`@Qc\x0F'\0\xCB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x1F` R`@\x90 \x80T`\xFF\x19\x16\x90U`\x1E\x80Ta\x16\xF0\x90`\x01\x90a5\xAAV[\x81T\x81\x10a\x17\0Wa\x17\0a32V[`\0\x91\x82R` \x90\x91 \x01T`\x1E\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x83\x90\x81\x10a\x17,Wa\x17,a32V[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\x1E\x80T\x80a\x17kWa\x17ka5\xBDV[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x90\x91\x01\x90\x91U`@Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x7Fp\x9BtP\xBF\xAF\xDA\x93\xEF\xD9\x1D)\x14\x98p\xA7\x94cz\xC9\xD6\x96\xCAab_\xD2\xF55H\xAF\xE0\x91\xA2PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15\x90a\x17\xEBWP`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x18\x08W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x1F` R`@\x90 T`\xFF\x16\x15a\x18BW`@Qc\xBD4cO`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x1F` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U`\x1E\x80T\x91\x82\x01\x81U\x83R\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90UQ\x7F\x16\xCE\x88(\\\xFDY\x82\x9AZ\xA0Cp\xA5\xEC\x80\x90\xA1\x8C\x14\xE7\xE7\xFB\x9DK\x12\xA4\"\x91\xC0\x98\xE3\x91\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x1F` R`@\x81 T`\xFF\x16a\x19\x07W`@Qcnb?\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x19\x12\x87\x86\x86a%$V[`@\x80Q3` \x80\x83\x01\x91\x90\x91R\x81\x83\x01\x8C\x90R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x93\x84\x90R\x80Q\x91\x01 c\x18* \xC3`\xE3\x1B\x90\x92R`\0\x91`\x01`\x01`\xA0\x1B\x03\x8B\x16\x91c\xC1Q\x06\x18\x91a\x19o\x91\x8C\x90\x8C\x90\x8A\x90\x8A\x90`d\x01a5\xD3V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x19\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xB2\x91\x90a4\xA9V[\x9A\x99PPPPPPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\xEAW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06\x81\x90U`@Q\x81\x90\x7F9Za%\x907)\x8D\x1CL\xD4\xBF\x17{d\xADY\x95\xD3\x8A\x93\x94W?\xCD\x90`\xD6I1J\xD0\x90`\0\x90\xA2PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1AGW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x13T\x81\x10\x15a\x1AjW`@Qc\n5S\x9D`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x14\x81\x90U`@Q\x81\x81R\x7F\xA9\xE7\x96\x1B\xB34'\x15\xDB\xEC\xC2\x08\x08zj\x9D\xF8\x98mRK:\n\x82\x9F\xD9\x0FZ/[\xA5>\x90` \x01a\r\xEEV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\xC9W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT\x81\x10\x80a\x1A\xE4WP`\nTa\x1A\xE1\x90\x82a3\x10V[\x15\x15[\x80a\x1A\xF0WP`\rT\x81\x11[\x15a\x1B\x0EW`@Qc\xF9\xC0\x95\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x81\x90U`@Q\x81\x81R\x7F1@}\xDD\x17\"\xF5\0\xB8\xAA,\x18\xE1\x129\x86&\xDD|(i\xA5\xF8\x071\xEC0\xB2D\xD9\xB5\xF2\x90` \x01a\r\xEEV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1BmW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x14T\x81\x11\x15a\x1B\x90W`@QcZ\x8FeW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x13\x81\x90U`@Q\x81\x81R\x7FI\x0E\xB2\xA9\x17F\xAA\x93<\x9F\xFE/y9\xAA\x06I\x8F2Y\x13\x9F\x88\x05\xF0\x08\xB1,\x8CizF\x90` \x01a\r\xEEV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1B\xEFW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0FT\x81\x10\x15a\x1C\x12W`@Qc~\xDC\x06\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10\x81\x90U`@Q\x81\x81R\x7F\x0EV\xD8?T\xE6\xF5\xB0\x87\x16\xA5K:\xBD\xB5\x9B0%\xBF\x12\xC1\x87\\\x87\xAB\x98\xAB\x08\x1Do\x83\x81\x90` \x01a\r\xEEV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1CqW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\x03\xAA[\x0F\xB6P\x14\xEE\xA8\x9F\xDA\x04\xA7\xBC\x11t \x14\x88\x1F<\x07\x8F,u\xB7\"l\xE1\r\x94\x18\x90`\0\x90\xA2PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C\xE5W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xC0I\x05\x8B\x1D\xF2\xDD\x89\x02s\x9C\xEBx\x99-\xF1/\xA86\x9C\x06\xC4P\xB3\xC6xq7\xB4R\xFD\xD2\x90`\0\x90\xA2PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1DYW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT\x81\x10\x80a\x1DtWP`\nTa\x1Dq\x90\x82a3\x10V[\x15\x15[\x80a\x1D\x80WP`\x0CT\x81\x11[\x15a\x1D\x9EW`@Qc\x02\x19\xD6c`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0B\x81\x90U`@Q\x81\x81R\x7Fo\x81u\xCD\xBA\xC1\xB4\xD28\xAB\xBA$\xA1}%T\xD7\xB9u\x0B\xBE\xDAd\x14\xE1\x91\xC4x8Kv1\x90` \x01a\r\xEEV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1D\xFDW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x11T\x81\x10\x15a\x1E W`@Qc\x01\x9C\xFB{`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x12\x81\x90U`@Q\x81\x81R\x7FO\xA3\\\x11\xCE\x9AE\xAE\x88,\x15N\xCBS\xAB\\\xACR\xA7J[\x9B\x03s\xBE6\xDE\xB3\x0Cx)x\x90` \x01a\r\xEEV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E\x7FW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\x8B`\x1D\x83\x83a,7V[P\x7F*\x85'l\xF6\x04\xA3\x82.\x19\xB2\x9A>\x97\xAE\xBF\xBCG\xA1\x90%\xC2\xE8\xF6\xE8\x0B:\xF7t\xDC\xBC8\x82\x82`@Qa\r\x14\x92\x91\x90a6\x12V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1F\x15W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x1E\xF7W[PPPPP\x90P\x90V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1FIW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7Foq\x7F\xB0\xABQ m\xEA@d\xA3\\\x94\xC2xO\x87\x14\xB0\x12\xFB\xDE\x82\x0E\r\xDE\xE3be\xEBj\x90`\0\x90\xA2PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1F\xBDW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\x9D>R.\x1EG\xA2\xF6\0\x9794+\x9C\xC7\xB2R\xA1\x88\x81T\xE8C\xABU\xEE\x1C\x81tW\x95\xAB\x90`\0\x90\xA2PV[``\x81\x83\x10a )W`@Qc;'5\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`!T\x82\x11\x15a LW`@Qc\xE0\xF7\xBE\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a V\x83\x83a5\xAAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a nWa na.1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \x97W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82[\x82\x81\x10\x15a\x0FNW`!\x81\x81T\x81\x10a \xB7Wa \xB7a32V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x85\x83\x03\x81Q\x81\x10a \xE9Wa \xE9a32V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a \x9CV[a!4`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P`@\x80Q`\x80\x81\x01\x82R`\x15T\x81R`\x16T` \x82\x01R`\x17T\x91\x81\x01\x91\x90\x91R`\x18T``\x82\x01R\x90V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a!\x8BW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xE5i9\x14\xD1\x9Cx\x9B\xDE\xE5\n6)\x98\xC0\xBC\x8D\x03Z\x83_\x98q\xDA]Q\x15/\x05\x82\xC3O\x90`\0\x90\xA2PV[`\0`!\x82\x81T\x81\x10a!\xEAWa!\xEAa32V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\"/W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xF3\xE0{K\xB49O/\xF3 \xBD\x1D\xD1QU\x1D\xFF0M^\x94\x8B@\x1D\x85X\xB2(H,\x97\xD8\x90`\0\x90\xA2PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\"\xA3W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0CT\x81\x10\x80a\"\xBEWP`\nTa\"\xBB\x90\x82a3\x10V[\x15\x15[\x80a\"\xCAWP`\x0ET\x81\x11[\x15a\"\xE8W`@Qc0\x07\xAD\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\r\x81\x90U`@Q\x81\x81R\x7F\xE9\xEF>\x93\xDF\xF7\x99\xD4\xDB\x8A\x12\xFFy\xE0\x91\x8AZx\xD7[\x10Rxd\xF4\xB1\xC9 \xF6\xF4\xF1x\x90` \x01a\r\xEEV[a#H`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P`@\x80Q`\x80\x81\x01\x82R`\x19T\x81R`\x1AT` \x82\x01R`\x1BT\x91\x81\x01\x91\x90\x91R`\x1CT``\x82\x01R\x90V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a#\x9FW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\rT\x81\x10\x80a#\xBAWP`\nTa#\xB7\x90\x82a3\x10V[\x15\x15[\x15a#\xD8W`@Qc\xCF\xB6\x99\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0E\x81\x90U`@Q\x81\x81R\x7F\x86o\xE9H_\x99\x83\xAF\xCE\xAA\x13\x850{n\xB0\xFD=\xF5\xA2P\xAE+\x0B\xF7m\xC9\xDD\xD3\x16\x92k\x90` \x01a\r\xEEV[``\x81\x83\x10a$/W`@Qc;'5\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x1ET\x82\x11\x15a$RW`@Qc\xE0\xF7\xBE\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$\\\x83\x83a5\xAAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$tWa$ta.1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$\x9DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82[\x82\x81\x10\x15a\x0FNW`\x1E\x81\x81T\x81\x10a$\xBDWa$\xBDa32V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x85\x83\x03\x81Q\x81\x10a$\xEFWa$\xEFa32V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a$\xA2V[`\0`\x1E\x82\x81T\x81\x10a!\xEAWa!\xEAa32V[`\x0BT\x83a\x01\0\x01Q\x10\x80a%?WP`\x0CT\x83a\x01\0\x01Q\x11[\x80a%[WP`\nT\x83a\x01\0\x01Qa%X\x91\x90a3\x10V[\x15\x15[\x15a%yW`@QcT(sM`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\rT\x83`\xE0\x01Q\x10\x80a%\x92WP`\x0ET\x83`\xE0\x01Q\x11[\x80a%\xB0WP\x82a\x01\0\x01Q\x83`\xE0\x01Qa%\xAD\x91\x90a3\x10V[\x15\x15[\x15a%\xCEW`@Qc%?\xFF\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0FT\x83`\xC0\x01Q\x10\x80a%\xE7WP`\x10T\x83`\xC0\x01Q\x11[\x15a&\x05W`@Qc\xEF\x9B\xC6_`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x19Ta\x01\xC0\x84\x01QQ\x11\x80a&9WP`\x1AT`\xE0\x84\x01Qa\x01\xC0\x85\x01Q` \x01Qa&7\x91c\x01\xE13\x80\x90a(\x95V[\x11[\x80a&NWP`\x1BTa\x01\xC0\x84\x01Q`@\x01Q\x11[\x80a&cWP`\x1CTa\x01\xC0\x84\x01Q``\x01Q\x11[\x80a&uWP`\x15Ta\x01\xC0\x84\x01QQ\x10[\x80a&\x9EWP`\x16T`\xE0\x84\x01Qa\x01\xC0\x85\x01Q` \x01Qa&\x9C\x91c\x01\xE13\x80\x90a(\xBBV[\x10[\x80a&\xB3WP`\x17Ta\x01\xC0\x84\x01Q`@\x01Q\x10[\x80a&\xC8WP`\x18Ta\x01\xC0\x84\x01Q``\x01Q\x10[\x15a&\xE6W`@Qc-\x87h\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x11T\x82\x10\x80a&\xF7WP`\x12T\x82\x11[\x15a'\x15W`@Qc0UM\xE1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a':f\x11\xC3y7\xE0\x80\0a'4\x85g\x1B\xC1mgN\xC8\0\0a(\xD9V[\x90a(\xF5V[`\x13T\x90\x91Pa'J\x90\x82a(\xF5V[\x82\x10\x80a'}WPa'za'qg\x1B\xC1mgN\xC8\0\0a'k\x86\x85a(\xF5V[\x90a)\x0BV[`\x14T\x90a) V[\x82\x11[\x15a'\x9BW`@Qc\x83\xEB\xDF\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a'\xAB\x83\x86`\xE0\x01Qa)5V[`\x05T`@\x87\x01Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x15\x80a'\xD6WP`\x06T\x85``\x01Q\x14\x15[\x80a'\xF5WP`\x07Ta\x01`\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x15[\x80a(\x14WP`\x08Ta\x01\x80\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x15[\x80a(3WP`\tTa\x01\xA0\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x15[\x80a(RWP`\x04Ta\x01@\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x15[\x80a(aWPa\x01 \x85\x01Q\x15\x15[\x15a(\x7FW`@Qc\xE8\xC0-\xD7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[0a\x01@\x86\x01Ra\x01 \x90\x94\x01\x93\x90\x93RPPPV[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a(\xACW`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a(\xD2W`\0\x80\xFD[P\x91\x02\x04\x90V[`\0a(\xEE\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a(\xBBV[\x93\x92PPPV[`\0\x81\x83\x11a)\x04W\x81a(\xEEV[P\x90\x91\x90PV[`\0a(\xEE\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a(\xBBV[`\0\x81\x83\x11a)/W\x82a(\xEEV[P\x91\x90PV[`\0\x80a)fa)Wa)I\x86`da6`V[f\xA5\xBB\xED\x86\xC5\xA0\0\x90a)\x0BV[gH\xCD@r(\x1E\0\0\x90a(\xD9V[\x90Pa)zg\r\xE0\xB6\xB3\xA7d\0\0\x82a(\xD9V[\x90Pa)\xD2\x81a'ka)\xA5a)\xA0a)\x9B\x89g\r\xE0\xB6\xB3\xA7d\0\0a6wV[a)\xDAV[a*\x08V[a)\xCCa)\xA0a)\xBA\x8A\x8Ac\x01\xE13\x80a(\xBBV[a)\x9B\x90g\r\xE0\xB6\xB3\xA7d\0\0a6wV[\x90a(\xD9V[\x94\x93PPPPV[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a*\x04W`@Qc9n\xA7\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0\x80\x82\x13a**W`@Qc\xE6\x1BIu`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x82\x81\x1C`\x0F\x10`\x02\x1B\x17\x82\x81\x1C\x90\x91\x10`\x01\x90\x81\x1B\x90\x91\x17\x82\x81\x1C\x90\x91\x10\x17`\x9F\x81\x81\x03``\x01\x92\x90\x92\x1B\x91`_\x19\x82\x01\x90a*\xB6\x90\x84\x90\x1Ca)\xDAV[lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x91\x90\x91\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a,\x8AW\x91` \x02\x82\x01[\x82\x81\x11\x15a,\x8AW\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x845\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a,WV[Pa*\x04\x92\x91P[\x80\x82\x11\x15a*\x04W`\0\x81U`\x01\x01a,\x92V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a,\xCCW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a,\xB0V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a(\xEE` \x83\x01\x84a,\xA6V[`\0`\x80\x82\x84\x03\x12\x15a)/W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a-#W`\0\x80\xFD[P5\x91\x90PV[`\0\x80` \x83\x85\x03\x12\x15a-=W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a-UW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a-iW`\0\x80\xFD[\x815\x81\x81\x11\x15a-xW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a-\x8DW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a-\xE0W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a-\xBBV[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a.\x01W`\0\x80\xFD[PV[\x805a.\x0F\x81a-\xECV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a.&W`\0\x80\xFD[\x815a(\xEE\x81a-\xECV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.kWa.ka.1V[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a.\x82W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a.\x9DWa.\x9Da.1V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a.\xC5Wa.\xC5a.1V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a.\xDEW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a/\x10W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a/3Wa/3a.1V[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[`\0a\x02@\x82\x84\x03\x12\x15a/wW`\0\x80\xFD[a/\x7Fa.GV[\x90Pa/\x8A\x82a.\x04V[\x81Ra/\x98` \x83\x01a.\x04V[` \x82\x01Ra/\xA9`@\x83\x01a.\x04V[`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015`\x80\x82\x01R`\xA0\x82\x015`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R`\xE0\x82\x015`\xE0\x82\x01Ra\x01\0\x80\x83\x015\x81\x83\x01RPa\x01 \x80\x83\x015\x81\x83\x01RPa\x01@a0\x06\x81\x84\x01a.\x04V[\x90\x82\x01Ra\x01`a0\x18\x83\x82\x01a.\x04V[\x90\x82\x01Ra\x01\x80a0*\x83\x82\x01a.\x04V[\x90\x82\x01Ra\x01\xA0a0<\x83\x82\x01a.\x04V[\x90\x82\x01Ra\x01\xC0a0O\x84\x84\x83\x01a.\xFEV[\x90\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a0kW`\0\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a0\x8FWa0\x8Fa.1V[\x81`@R\x82\x93P\x845\x91Pa0\xA3\x82a-\xECV[\x90\x82R` \x84\x015\x90\x81\x15\x15\x82\x14a0\xBAW`\0\x80\xFD[\x81` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15a0\xD4W`\0\x80\xFD[Pa0\xE1\x85\x82\x86\x01a.qV[`@\x83\x01RPP\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x03`\x8B\x8D\x03\x12\x15a1\x0EW`\0\x80\xFD[\x8A5\x99Pa1\x1E` \x8C\x01a.\x04V[\x98P`@\x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a1;W`\0\x80\xFD[a1G\x8E\x83\x8F\x01a.qV[\x99Pa1V\x8E``\x8F\x01a/dV[\x98Pa\x02\xA0\x8D\x015\x91P\x80\x82\x11\x15a1mW`\0\x80\xFD[a1y\x8E\x83\x8F\x01a.qV[\x97Pa\x02\xC0\x8D\x015\x96Pa\x02\xE0\x8D\x015\x95Pa\x03\0\x8D\x015\x94Pa\x03 \x8D\x015\x91P\x80\x82\x11\x15a1\xA8W`\0\x80\xFD[Pa1\xB5\x8D\x82\x8E\x01a0YV[\x92PPa\x03@\x8B\x015\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0\x80`@\x83\x85\x03\x12\x15a1\xE2W`\0\x80\xFD[\x825a1\xED\x81a-\xECV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x03 \x89\x8B\x03\x12\x15a2\x18W`\0\x80\xFD[\x885\x97P` \x89\x015a2*\x81a-\xECV[\x96Pa29\x8A`@\x8B\x01a/dV[\x95Pa\x02\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2VW`\0\x80\xFD[a2b\x8B\x82\x8C\x01a.qV[\x98\x9B\x97\x9AP\x95\x98a\x02\xA0\x81\x015\x97a\x02\xC0\x82\x015\x97Pa\x02\xE0\x82\x015\x96Pa\x03\0\x90\x91\x015\x94P\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a2\xA2W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x81\x01a\x0FPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a2\xF0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a)/WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0\x82a3-WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x01Qa3o` \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x81\x01Qa3\x8A`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x81\x01Q``\x83\x01R`\x80\x81\x01Q`\x80\x83\x01R`\xA0\x81\x01Q`\xA0\x83\x01R`\xC0\x81\x01Q`\xC0\x83\x01R`\xE0\x81\x01Q`\xE0\x83\x01Ra\x01\0\x80\x82\x01Q\x81\x84\x01RPa\x01 \x80\x82\x01Q\x81\x84\x01RPa\x01@\x80\x82\x01Qa3\xF0\x82\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[PPa\x01`\x81\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x84\x01\x91\x90\x91Ra\x01\x80\x80\x83\x01Q\x82\x16\x90\x84\x01Ra\x01\xA0\x80\x83\x01Q\x90\x91\x16\x90\x83\x01Ra\x01\xC0\x90\x81\x01Q\x80Q\x91\x83\x01\x91\x90\x91R` \x81\x01Qa\x01\xE0\x83\x01R`@\x81\x01Qa\x02\0\x83\x01R``\x01Qa\x02 \x90\x91\x01RV[`\0a\x02\xC0\x87\x83R\x80` \x84\x01Ra4s\x81\x84\x01\x88a,\xA6V[\x90Pa4\x82`@\x84\x01\x87a3HV[\x82\x81\x03a\x02\x80\x84\x01Ra4\x95\x81\x86a,\xA6V[\x91PP\x82a\x02\xA0\x83\x01R\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a4\xBBW`\0\x80\xFD[\x81Qa(\xEE\x81a-\xECV[`\x01`\x01`\xA0\x1B\x03\x85\x16\x81Ra\x02\xA0` \x82\x01\x81\x90R`\0\x90a4\xEB\x83\x82\x01\x87a,\xA6V[\x90Pa4\xFA`@\x84\x01\x86a3HV[\x82\x81\x03a\x02\x80\x84\x01Ra5\r\x81\x85a,\xA6V[\x97\x96PPPPPPPV[\x85\x81R`\0`\x01\x80`\xA0\x1B\x03\x80\x87\x16` \x84\x01R\x85`@\x84\x01R\x84``\x84\x01R`\xA0`\x80\x84\x01R\x80\x84Q\x16`\xA0\x84\x01RP` \x83\x01Q\x15\x15`\xC0\x83\x01R`@\x83\x01Q```\xE0\x84\x01Ra5oa\x01\0\x84\x01\x82a,\xA6V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a5\x8DW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0FPWa\x0FPa5\x94V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0a\x02\xC0\x87\x83Ra5\xE8` \x84\x01\x88a3HV[\x80a\x02`\x84\x01Ra5\xFB\x81\x84\x01\x87a,\xA6V[a\x02\x80\x84\x01\x95\x90\x95RPPa\x02\xA0\x01R\x93\x92PPPV[` \x80\x82R\x81\x81\x01\x83\x90R`\0\x90\x84`@\x84\x01\x83[\x86\x81\x10\x15a6UW\x825a6:\x81a-\xECV[`\x01`\x01`\xA0\x1B\x03\x16\x82R\x91\x83\x01\x91\x90\x83\x01\x90`\x01\x01a6'V[P\x96\x95PPPPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0FPWa\x0FPa5\x94V[\x80\x82\x01\x80\x82\x11\x15a\x0FPWa\x0FPa5\x94V\xFE\xA2dipfsX\"\x12 \x97\x13\x02\xAB,\xEC\x02\xD5^\xFBI\x8E=\xCE\x0C\x85\xFEwsc\xFE\xCF\xB7\x8BgF\xD2n\xE19\xC2\xD5dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static HYPERDRIVEFACTORY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct HyperdriveFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for HyperdriveFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for HyperdriveFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for HyperdriveFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for HyperdriveFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(HyperdriveFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> HyperdriveFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    HYPERDRIVEFACTORY_ABI.clone(),
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
                HYPERDRIVEFACTORY_ABI.clone(),
                HYPERDRIVEFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `_instancesToDeployerCoordinators` (0x2b58f418) function
        pub fn instances_to_deployer_coordinators(
            &self,
            instance: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([43, 88, 244, 24], instance)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addDeployerCoordinator` (0x421caba8) function
        pub fn add_deployer_coordinator(
            &self,
            deployer_coordinator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 28, 171, 168], deployer_coordinator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkpointDurationResolution` (0xd0f96b92) function
        pub fn checkpoint_duration_resolution(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([208, 249, 107, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkpointRewarder` (0xf2596458) function
        pub fn checkpoint_rewarder(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([242, 89, 100, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultPausers` (0xa64c90bf) function
        pub fn default_pausers(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([166, 76, 144, 191], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployAndInitialize` (0x2e7cd971) function
        pub fn deploy_and_initialize(
            &self,
            deployment_id: [u8; 32],
            deployer_coordinator: ::ethers::core::types::Address,
            name: ::std::string::String,
            config: PoolDeployConfig,
            extra_data: ::ethers::core::types::Bytes,
            contribution: ::ethers::core::types::U256,
            fixed_apr: ::ethers::core::types::U256,
            time_stretch_apr: ::ethers::core::types::U256,
            options: Options,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [46, 124, 217, 113],
                    (
                        deployment_id,
                        deployer_coordinator,
                        name,
                        config,
                        extra_data,
                        contribution,
                        fixed_apr,
                        time_stretch_apr,
                        options,
                        salt,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployTarget` (0x49f13de7) function
        pub fn deploy_target(
            &self,
            deployment_id: [u8; 32],
            deployer_coordinator: ::ethers::core::types::Address,
            config: PoolDeployConfig,
            extra_data: ::ethers::core::types::Bytes,
            fixed_apr: ::ethers::core::types::U256,
            time_stretch_apr: ::ethers::core::types::U256,
            target_index: ::ethers::core::types::U256,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [73, 241, 61, 231],
                    (
                        deployment_id,
                        deployer_coordinator,
                        config,
                        extra_data,
                        fixed_apr,
                        time_stretch_apr,
                        target_index,
                        salt,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployerCoordinatorManager` (0xe4e7148f) function
        pub fn deployer_coordinator_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([228, 231, 20, 143], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeCollector` (0xc415b95c) function
        pub fn fee_collector(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 21, 185, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDeployerCoordinatorAtIndex` (0xfe3d5aeb) function
        pub fn get_deployer_coordinator_at_index(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([254, 61, 90, 235], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDeployerCoordinatorByInstances` (0x1b59be0c) function
        pub fn get_deployer_coordinator_by_instances(
            &self,
            instances: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([27, 89, 190, 12], instances)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDeployerCoordinatorsInRange` (0xec895f11) function
        pub fn get_deployer_coordinators_in_range(
            &self,
            start_index: ::ethers::core::types::U256,
            end_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([236, 137, 95, 17], (start_index, end_index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInstanceAtIndex` (0xdaac24da) function
        pub fn get_instance_at_index(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([218, 172, 36, 218], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInstancesInRange` (0xbc30e7a1) function
        pub fn get_instances_in_range(
            &self,
            start_index: ::ethers::core::types::U256,
            end_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([188, 48, 231, 161], (start_index, end_index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNumberOfDeployerCoordinators` (0xe1b39c80) function
        pub fn get_number_of_deployer_coordinators(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([225, 179, 156, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNumberOfInstances` (0x6e95d67c) function
        pub fn get_number_of_instances(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([110, 149, 214, 124], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `governance` (0x5aa6e675) function
        pub fn governance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([90, 166, 230, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hyperdriveGovernance` (0xe3331555) function
        pub fn hyperdrive_governance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([227, 51, 21, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDeployerCoordinator` (0xf8c09e59) function
        pub fn is_deployer_coordinator(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 192, 158, 89], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isInstance` (0x6b44e6be) function
        pub fn is_instance(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([107, 68, 230, 190], p0)
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
        ///Calls the contract's `linkerCodeHash` (0xc905a4b5) function
        pub fn linker_code_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([201, 5, 164, 181], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `linkerFactory` (0x99623bb1) function
        pub fn linker_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([153, 98, 59, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxCheckpointDuration` (0xe0e2daaa) function
        pub fn max_checkpoint_duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([224, 226, 218, 170], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxCircuitBreakerDelta` (0x4554f9a9) function
        pub fn max_circuit_breaker_delta(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([69, 84, 249, 169], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxFees` (0xe83e34b1) function
        pub fn max_fees(&self) -> ::ethers::contract::builders::ContractCall<M, Fees> {
            self.0
                .method_hash([232, 62, 52, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxFixedAPR` (0xbf9bd5cd) function
        pub fn max_fixed_apr(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([191, 155, 213, 205], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxPositionDuration` (0x8efc0986) function
        pub fn max_position_duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([142, 252, 9, 134], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxTimeStretchAPR` (0x48800760) function
        pub fn max_time_stretch_apr(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([72, 128, 7, 96], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minCheckpointDuration` (0x5720c9d5) function
        pub fn min_checkpoint_duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([87, 32, 201, 213], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minCircuitBreakerDelta` (0x1ecda0fe) function
        pub fn min_circuit_breaker_delta(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([30, 205, 160, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minFees` (0xc1722563) function
        pub fn min_fees(&self) -> ::ethers::contract::builders::ContractCall<M, Fees> {
            self.0
                .method_hash([193, 114, 37, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minFixedAPR` (0xd23d7ea3) function
        pub fn min_fixed_apr(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([210, 61, 126, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minPositionDuration` (0xdaf012e6) function
        pub fn min_position_duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([218, 240, 18, 230], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minTimeStretchAPR` (0xd6f50169) function
        pub fn min_time_stretch_apr(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([214, 245, 1, 105], ())
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
        ///Calls the contract's `removeDeployerCoordinator` (0x411c3035) function
        pub fn remove_deployer_coordinator(
            &self,
            deployer_coordinator: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 28, 48, 53], (deployer_coordinator, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepCollector` (0x10780f73) function
        pub fn sweep_collector(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([16, 120, 15, 115], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateCheckpointDurationResolution` (0x11e77bfe) function
        pub fn update_checkpoint_duration_resolution(
            &self,
            checkpoint_duration_resolution: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([17, 231, 123, 254], checkpoint_duration_resolution)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateCheckpointRewarder` (0x3e2d2014) function
        pub fn update_checkpoint_rewarder(
            &self,
            checkpoint_rewarder: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 45, 32, 20], checkpoint_rewarder)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateDefaultPausers` (0x9af25262) function
        pub fn update_default_pausers(
            &self,
            default_pausers: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 242, 82, 98], default_pausers)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateDeployerCoordinatorManager` (0xa98a46db) function
        pub fn update_deployer_coordinator_manager(
            &self,
            deployer_coordinator_manager: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 138, 70, 219], deployer_coordinator_manager)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateFeeCollector` (0xd2c35ce8) function
        pub fn update_fee_collector(
            &self,
            fee_collector: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 195, 92, 232], fee_collector)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateGovernance` (0xb2561263) function
        pub fn update_governance(
            &self,
            governance: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 86, 18, 99], governance)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateHyperdriveGovernance` (0xdd2b8fbb) function
        pub fn update_hyperdrive_governance(
            &self,
            hyperdrive_governance: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 43, 143, 187], hyperdrive_governance)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateLinkerCodeHash` (0x4fbfee77) function
        pub fn update_linker_code_hash(
            &self,
            linker_code_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 191, 238, 119], linker_code_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateLinkerFactory` (0x85229785) function
        pub fn update_linker_factory(
            &self,
            linker_factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 34, 151, 133], linker_factory)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMaxCheckpointDuration` (0x6f6d5c4a) function
        pub fn update_max_checkpoint_duration(
            &self,
            max_checkpoint_duration: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 109, 92, 74], max_checkpoint_duration)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMaxCircuitBreakerDelta` (0x84c19aab) function
        pub fn update_max_circuit_breaker_delta(
            &self,
            max_circuit_breaker_delta: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 193, 154, 171], max_circuit_breaker_delta)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMaxFees` (0x2885e3ac) function
        pub fn update_max_fees(
            &self,
            max_fees: Fees,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 133, 227, 172], (max_fees,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMaxFixedAPR` (0x97b0e8ce) function
        pub fn update_max_fixed_apr(
            &self,
            max_fixed_apr: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 176, 232, 206], max_fixed_apr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMaxPositionDuration` (0xeb71f66c) function
        pub fn update_max_position_duration(
            &self,
            max_position_duration: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 113, 246, 108], max_position_duration)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMaxTimeStretchAPR` (0x628027a3) function
        pub fn update_max_time_stretch_apr(
            &self,
            max_time_stretch_apr: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 128, 39, 163], max_time_stretch_apr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMinCheckpointDuration` (0x8e127cf5) function
        pub fn update_min_checkpoint_duration(
            &self,
            min_checkpoint_duration: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 18, 124, 245], min_checkpoint_duration)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMinCircuitBreakerDelta` (0x2907d3dd) function
        pub fn update_min_circuit_breaker_delta(
            &self,
            min_circuit_breaker_delta: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([41, 7, 211, 221], min_circuit_breaker_delta)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMinFees` (0x10d1dc3e) function
        pub fn update_min_fees(
            &self,
            min_fees: Fees,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 209, 220, 62], (min_fees,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMinFixedAPR` (0x1978ebcf) function
        pub fn update_min_fixed_apr(
            &self,
            min_fixed_apr: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 120, 235, 207], min_fixed_apr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMinPositionDuration` (0xe71f34b3) function
        pub fn update_min_position_duration(
            &self,
            min_position_duration: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 31, 52, 179], min_position_duration)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMinTimeStretchAPR` (0x83b361e8) function
        pub fn update_min_time_stretch_apr(
            &self,
            min_time_stretch_apr: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 179, 97, 232], min_time_stretch_apr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateSweepCollector` (0x8627a4f0) function
        pub fn update_sweep_collector(
            &self,
            sweep_collector: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 39, 164, 240], sweep_collector)
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
        ///Gets the contract's `CheckpointDurationResolutionUpdated` event
        pub fn checkpoint_duration_resolution_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CheckpointDurationResolutionUpdatedFilter,
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
        ///Gets the contract's `DefaultPausersUpdated` event
        pub fn default_pausers_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultPausersUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deployed` event
        pub fn deployed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeployedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DeployerCoordinatorAdded` event
        pub fn deployer_coordinator_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeployerCoordinatorAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DeployerCoordinatorManagerUpdated` event
        pub fn deployer_coordinator_manager_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeployerCoordinatorManagerUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DeployerCoordinatorRemoved` event
        pub fn deployer_coordinator_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeployerCoordinatorRemovedFilter,
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
        ///Gets the contract's `HyperdriveGovernanceUpdated` event
        pub fn hyperdrive_governance_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            HyperdriveGovernanceUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LinkerCodeHashUpdated` event
        pub fn linker_code_hash_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LinkerCodeHashUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LinkerFactoryUpdated` event
        pub fn linker_factory_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LinkerFactoryUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MaxCheckpointDurationUpdated` event
        pub fn max_checkpoint_duration_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MaxCheckpointDurationUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MaxCircuitBreakerDeltaUpdated` event
        pub fn max_circuit_breaker_delta_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MaxCircuitBreakerDeltaUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MaxFeesUpdated` event
        pub fn max_fees_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MaxFeesUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MaxFixedAPRUpdated` event
        pub fn max_fixed_apr_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MaxFixedAPRUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MaxPositionDurationUpdated` event
        pub fn max_position_duration_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MaxPositionDurationUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MaxTimeStretchAPRUpdated` event
        pub fn max_time_stretch_apr_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MaxTimeStretchAPRUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MinCheckpointDurationUpdated` event
        pub fn min_checkpoint_duration_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinCheckpointDurationUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MinCircuitBreakerDeltaUpdated` event
        pub fn min_circuit_breaker_delta_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinCircuitBreakerDeltaUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MinFeesUpdated` event
        pub fn min_fees_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinFeesUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MinFixedAPRUpdated` event
        pub fn min_fixed_apr_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinFixedAPRUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MinPositionDurationUpdated` event
        pub fn min_position_duration_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinPositionDurationUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MinTimeStretchAPRUpdated` event
        pub fn min_time_stretch_apr_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinTimeStretchAPRUpdatedFilter,
        > {
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            HyperdriveFactoryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for HyperdriveFactory<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DeployerCoordinatorAlreadyAdded` with signature `DeployerCoordinatorAlreadyAdded()` and selector `0xbd34634f`
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
        name = "DeployerCoordinatorAlreadyAdded",
        abi = "DeployerCoordinatorAlreadyAdded()"
    )]
    pub struct DeployerCoordinatorAlreadyAdded;
    ///Custom Error type `DeployerCoordinatorIndexMismatch` with signature `DeployerCoordinatorIndexMismatch()` and selector `0x3c9c032c`
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
        name = "DeployerCoordinatorIndexMismatch",
        abi = "DeployerCoordinatorIndexMismatch()"
    )]
    pub struct DeployerCoordinatorIndexMismatch;
    ///Custom Error type `DeployerCoordinatorNotAdded` with signature `DeployerCoordinatorNotAdded()` and selector `0x4bf121ab`
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
        name = "DeployerCoordinatorNotAdded",
        abi = "DeployerCoordinatorNotAdded()"
    )]
    pub struct DeployerCoordinatorNotAdded;
    ///Custom Error type `EndIndexTooLarge` with signature `EndIndexTooLarge()` and selector `0xe0f7becb`
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
    #[etherror(name = "EndIndexTooLarge", abi = "EndIndexTooLarge()")]
    pub struct EndIndexTooLarge;
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
    ///Custom Error type `InvalidCheckpointDurationResolution` with signature `InvalidCheckpointDurationResolution()` and selector `0x8dbae0a8`
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
        name = "InvalidCheckpointDurationResolution",
        abi = "InvalidCheckpointDurationResolution()"
    )]
    pub struct InvalidCheckpointDurationResolution;
    ///Custom Error type `InvalidCircuitBreakerDelta` with signature `InvalidCircuitBreakerDelta()` and selector `0xef9bc65f`
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
        name = "InvalidCircuitBreakerDelta",
        abi = "InvalidCircuitBreakerDelta()"
    )]
    pub struct InvalidCircuitBreakerDelta;
    ///Custom Error type `InvalidDeployConfig` with signature `InvalidDeployConfig()` and selector `0xe8c02dd7`
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
    #[etherror(name = "InvalidDeployConfig", abi = "InvalidDeployConfig()")]
    pub struct InvalidDeployConfig;
    ///Custom Error type `InvalidDeployerCoordinator` with signature `InvalidDeployerCoordinator()` and selector `0x6e623f0f`
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
        name = "InvalidDeployerCoordinator",
        abi = "InvalidDeployerCoordinator()"
    )]
    pub struct InvalidDeployerCoordinator;
    ///Custom Error type `InvalidFees` with signature `InvalidFees()` and selector `0x2d8768f9`
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
    #[etherror(name = "InvalidFees", abi = "InvalidFees()")]
    pub struct InvalidFees;
    ///Custom Error type `InvalidFixedAPR` with signature `InvalidFixedAPR()` and selector `0x30554de1`
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
    #[etherror(name = "InvalidFixedAPR", abi = "InvalidFixedAPR()")]
    pub struct InvalidFixedAPR;
    ///Custom Error type `InvalidIndexes` with signature `InvalidIndexes()` and selector `0x764e6b56`
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
    #[etherror(name = "InvalidIndexes", abi = "InvalidIndexes()")]
    pub struct InvalidIndexes;
    ///Custom Error type `InvalidMaxCheckpointDuration` with signature `InvalidMaxCheckpointDuration()` and selector `0xf9c0959d`
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
        name = "InvalidMaxCheckpointDuration",
        abi = "InvalidMaxCheckpointDuration()"
    )]
    pub struct InvalidMaxCheckpointDuration;
    ///Custom Error type `InvalidMaxCircuitBreakerDelta` with signature `InvalidMaxCircuitBreakerDelta()` and selector `0xfdb80c26`
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
        name = "InvalidMaxCircuitBreakerDelta",
        abi = "InvalidMaxCircuitBreakerDelta()"
    )]
    pub struct InvalidMaxCircuitBreakerDelta;
    ///Custom Error type `InvalidMaxFees` with signature `InvalidMaxFees()` and selector `0x2c20e3f6`
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
    #[etherror(name = "InvalidMaxFees", abi = "InvalidMaxFees()")]
    pub struct InvalidMaxFees;
    ///Custom Error type `InvalidMaxFixedAPR` with signature `InvalidMaxFixedAPR()` and selector `0x673edec0`
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
    #[etherror(name = "InvalidMaxFixedAPR", abi = "InvalidMaxFixedAPR()")]
    pub struct InvalidMaxFixedAPR;
    ///Custom Error type `InvalidMaxPositionDuration` with signature `InvalidMaxPositionDuration()` and selector `0xcfb699cb`
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
        name = "InvalidMaxPositionDuration",
        abi = "InvalidMaxPositionDuration()"
    )]
    pub struct InvalidMaxPositionDuration;
    ///Custom Error type `InvalidMaxTimeStretchAPR` with signature `InvalidMaxTimeStretchAPR()` and selector `0xa35539d0`
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
    #[etherror(name = "InvalidMaxTimeStretchAPR", abi = "InvalidMaxTimeStretchAPR()")]
    pub struct InvalidMaxTimeStretchAPR;
    ///Custom Error type `InvalidMinCheckpointDuration` with signature `InvalidMinCheckpointDuration()` and selector `0x0433acc6`
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
        name = "InvalidMinCheckpointDuration",
        abi = "InvalidMinCheckpointDuration()"
    )]
    pub struct InvalidMinCheckpointDuration;
    ///Custom Error type `InvalidMinCircuitBreakerDelta` with signature `InvalidMinCircuitBreakerDelta()` and selector `0x28a2d9a9`
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
        name = "InvalidMinCircuitBreakerDelta",
        abi = "InvalidMinCircuitBreakerDelta()"
    )]
    pub struct InvalidMinCircuitBreakerDelta;
    ///Custom Error type `InvalidMinFees` with signature `InvalidMinFees()` and selector `0x15b05a8f`
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
    #[etherror(name = "InvalidMinFees", abi = "InvalidMinFees()")]
    pub struct InvalidMinFees;
    ///Custom Error type `InvalidMinFixedAPR` with signature `InvalidMinFixedAPR()` and selector `0x1670f797`
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
    #[etherror(name = "InvalidMinFixedAPR", abi = "InvalidMinFixedAPR()")]
    pub struct InvalidMinFixedAPR;
    ///Custom Error type `InvalidMinPositionDuration` with signature `InvalidMinPositionDuration()` and selector `0x600f5a02`
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
        name = "InvalidMinPositionDuration",
        abi = "InvalidMinPositionDuration()"
    )]
    pub struct InvalidMinPositionDuration;
    ///Custom Error type `InvalidMinTimeStretchAPR` with signature `InvalidMinTimeStretchAPR()` and selector `0x5a8f6557`
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
    #[etherror(name = "InvalidMinTimeStretchAPR", abi = "InvalidMinTimeStretchAPR()")]
    pub struct InvalidMinTimeStretchAPR;
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
    ///Custom Error type `InvalidTimeStretchAPR` with signature `InvalidTimeStretchAPR()` and selector `0x83ebdfb7`
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
    #[etherror(name = "InvalidTimeStretchAPR", abi = "InvalidTimeStretchAPR()")]
    pub struct InvalidTimeStretchAPR;
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
    ///Custom Error type `ReceiveLocked` with signature `ReceiveLocked()` and selector `0x5563ada8`
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
    #[etherror(name = "ReceiveLocked", abi = "ReceiveLocked()")]
    pub struct ReceiveLocked;
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
    ///Custom Error type `Unauthorized` with signature `Unauthorized()` and selector `0x82b42900`
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
    #[etherror(name = "Unauthorized", abi = "Unauthorized()")]
    pub struct Unauthorized;
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
    pub enum HyperdriveFactoryErrors {
        DeployerCoordinatorAlreadyAdded(DeployerCoordinatorAlreadyAdded),
        DeployerCoordinatorIndexMismatch(DeployerCoordinatorIndexMismatch),
        DeployerCoordinatorNotAdded(DeployerCoordinatorNotAdded),
        EndIndexTooLarge(EndIndexTooLarge),
        InvalidCheckpointDuration(InvalidCheckpointDuration),
        InvalidCheckpointDurationResolution(InvalidCheckpointDurationResolution),
        InvalidCircuitBreakerDelta(InvalidCircuitBreakerDelta),
        InvalidDeployConfig(InvalidDeployConfig),
        InvalidDeployerCoordinator(InvalidDeployerCoordinator),
        InvalidFees(InvalidFees),
        InvalidFixedAPR(InvalidFixedAPR),
        InvalidIndexes(InvalidIndexes),
        InvalidMaxCheckpointDuration(InvalidMaxCheckpointDuration),
        InvalidMaxCircuitBreakerDelta(InvalidMaxCircuitBreakerDelta),
        InvalidMaxFees(InvalidMaxFees),
        InvalidMaxFixedAPR(InvalidMaxFixedAPR),
        InvalidMaxPositionDuration(InvalidMaxPositionDuration),
        InvalidMaxTimeStretchAPR(InvalidMaxTimeStretchAPR),
        InvalidMinCheckpointDuration(InvalidMinCheckpointDuration),
        InvalidMinCircuitBreakerDelta(InvalidMinCircuitBreakerDelta),
        InvalidMinFees(InvalidMinFees),
        InvalidMinFixedAPR(InvalidMinFixedAPR),
        InvalidMinPositionDuration(InvalidMinPositionDuration),
        InvalidMinTimeStretchAPR(InvalidMinTimeStretchAPR),
        InvalidPositionDuration(InvalidPositionDuration),
        InvalidTimeStretchAPR(InvalidTimeStretchAPR),
        LnInvalidInput(LnInvalidInput),
        ReceiveLocked(ReceiveLocked),
        TransferFailed(TransferFailed),
        Unauthorized(Unauthorized),
        UnsafeCastToInt256(UnsafeCastToInt256),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for HyperdriveFactoryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DeployerCoordinatorAlreadyAdded as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployerCoordinatorAlreadyAdded(decoded));
            }
            if let Ok(decoded) = <DeployerCoordinatorIndexMismatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployerCoordinatorIndexMismatch(decoded));
            }
            if let Ok(decoded) = <DeployerCoordinatorNotAdded as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployerCoordinatorNotAdded(decoded));
            }
            if let Ok(decoded) = <EndIndexTooLarge as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EndIndexTooLarge(decoded));
            }
            if let Ok(decoded) = <InvalidCheckpointDuration as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidCheckpointDuration(decoded));
            }
            if let Ok(decoded) = <InvalidCheckpointDurationResolution as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidCheckpointDurationResolution(decoded));
            }
            if let Ok(decoded) = <InvalidCircuitBreakerDelta as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidCircuitBreakerDelta(decoded));
            }
            if let Ok(decoded) = <InvalidDeployConfig as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidDeployConfig(decoded));
            }
            if let Ok(decoded) = <InvalidDeployerCoordinator as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidDeployerCoordinator(decoded));
            }
            if let Ok(decoded) = <InvalidFees as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidFees(decoded));
            }
            if let Ok(decoded) = <InvalidFixedAPR as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidFixedAPR(decoded));
            }
            if let Ok(decoded) = <InvalidIndexes as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidIndexes(decoded));
            }
            if let Ok(decoded) = <InvalidMaxCheckpointDuration as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMaxCheckpointDuration(decoded));
            }
            if let Ok(decoded) = <InvalidMaxCircuitBreakerDelta as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMaxCircuitBreakerDelta(decoded));
            }
            if let Ok(decoded) = <InvalidMaxFees as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMaxFees(decoded));
            }
            if let Ok(decoded) = <InvalidMaxFixedAPR as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMaxFixedAPR(decoded));
            }
            if let Ok(decoded) = <InvalidMaxPositionDuration as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMaxPositionDuration(decoded));
            }
            if let Ok(decoded) = <InvalidMaxTimeStretchAPR as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMaxTimeStretchAPR(decoded));
            }
            if let Ok(decoded) = <InvalidMinCheckpointDuration as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMinCheckpointDuration(decoded));
            }
            if let Ok(decoded) = <InvalidMinCircuitBreakerDelta as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMinCircuitBreakerDelta(decoded));
            }
            if let Ok(decoded) = <InvalidMinFees as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMinFees(decoded));
            }
            if let Ok(decoded) = <InvalidMinFixedAPR as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMinFixedAPR(decoded));
            }
            if let Ok(decoded) = <InvalidMinPositionDuration as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMinPositionDuration(decoded));
            }
            if let Ok(decoded) = <InvalidMinTimeStretchAPR as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMinTimeStretchAPR(decoded));
            }
            if let Ok(decoded) = <InvalidPositionDuration as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidPositionDuration(decoded));
            }
            if let Ok(decoded) = <InvalidTimeStretchAPR as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidTimeStretchAPR(decoded));
            }
            if let Ok(decoded) = <LnInvalidInput as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LnInvalidInput(decoded));
            }
            if let Ok(decoded) = <ReceiveLocked as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReceiveLocked(decoded));
            }
            if let Ok(decoded) = <TransferFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFailed(decoded));
            }
            if let Ok(decoded) = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unauthorized(decoded));
            }
            if let Ok(decoded) = <UnsafeCastToInt256 as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnsafeCastToInt256(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for HyperdriveFactoryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DeployerCoordinatorAlreadyAdded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployerCoordinatorIndexMismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployerCoordinatorNotAdded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EndIndexTooLarge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidCheckpointDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidCheckpointDurationResolution(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidCircuitBreakerDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidDeployConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidDeployerCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFixedAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidIndexes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMaxCheckpointDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMaxCircuitBreakerDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMaxFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMaxFixedAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMaxPositionDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMaxTimeStretchAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMinCheckpointDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMinCircuitBreakerDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMinFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMinFixedAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMinPositionDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMinTimeStretchAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPositionDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTimeStretchAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LnInvalidInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReceiveLocked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsafeCastToInt256(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for HyperdriveFactoryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <DeployerCoordinatorAlreadyAdded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DeployerCoordinatorIndexMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DeployerCoordinatorNotAdded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EndIndexTooLarge as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidCheckpointDuration as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidCheckpointDurationResolution as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidCircuitBreakerDelta as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidDeployConfig as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidDeployerCoordinator as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidFees as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidFixedAPR as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidIndexes as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMaxCheckpointDuration as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMaxCircuitBreakerDelta as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMaxFees as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMaxFixedAPR as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMaxPositionDuration as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMaxTimeStretchAPR as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMinCheckpointDuration as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMinCircuitBreakerDelta as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMinFees as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMinFixedAPR as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMinPositionDuration as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMinTimeStretchAPR as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidPositionDuration as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTimeStretchAPR as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <LnInvalidInput as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ReceiveLocked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <UnsafeCastToInt256 as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for HyperdriveFactoryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeployerCoordinatorAlreadyAdded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployerCoordinatorIndexMismatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployerCoordinatorNotAdded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EndIndexTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidCheckpointDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidCheckpointDurationResolution(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidCircuitBreakerDelta(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidDeployConfig(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidDeployerCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidFixedAPR(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidIndexes(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidMaxCheckpointDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMaxCircuitBreakerDelta(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMaxFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidMaxFixedAPR(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMaxPositionDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMaxTimeStretchAPR(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMinCheckpointDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMinCircuitBreakerDelta(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMinFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidMinFixedAPR(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMinPositionDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMinTimeStretchAPR(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidPositionDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTimeStretchAPR(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LnInvalidInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReceiveLocked(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsafeCastToInt256(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for HyperdriveFactoryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DeployerCoordinatorAlreadyAdded>
    for HyperdriveFactoryErrors {
        fn from(value: DeployerCoordinatorAlreadyAdded) -> Self {
            Self::DeployerCoordinatorAlreadyAdded(value)
        }
    }
    impl ::core::convert::From<DeployerCoordinatorIndexMismatch>
    for HyperdriveFactoryErrors {
        fn from(value: DeployerCoordinatorIndexMismatch) -> Self {
            Self::DeployerCoordinatorIndexMismatch(value)
        }
    }
    impl ::core::convert::From<DeployerCoordinatorNotAdded> for HyperdriveFactoryErrors {
        fn from(value: DeployerCoordinatorNotAdded) -> Self {
            Self::DeployerCoordinatorNotAdded(value)
        }
    }
    impl ::core::convert::From<EndIndexTooLarge> for HyperdriveFactoryErrors {
        fn from(value: EndIndexTooLarge) -> Self {
            Self::EndIndexTooLarge(value)
        }
    }
    impl ::core::convert::From<InvalidCheckpointDuration> for HyperdriveFactoryErrors {
        fn from(value: InvalidCheckpointDuration) -> Self {
            Self::InvalidCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<InvalidCheckpointDurationResolution>
    for HyperdriveFactoryErrors {
        fn from(value: InvalidCheckpointDurationResolution) -> Self {
            Self::InvalidCheckpointDurationResolution(value)
        }
    }
    impl ::core::convert::From<InvalidCircuitBreakerDelta> for HyperdriveFactoryErrors {
        fn from(value: InvalidCircuitBreakerDelta) -> Self {
            Self::InvalidCircuitBreakerDelta(value)
        }
    }
    impl ::core::convert::From<InvalidDeployConfig> for HyperdriveFactoryErrors {
        fn from(value: InvalidDeployConfig) -> Self {
            Self::InvalidDeployConfig(value)
        }
    }
    impl ::core::convert::From<InvalidDeployerCoordinator> for HyperdriveFactoryErrors {
        fn from(value: InvalidDeployerCoordinator) -> Self {
            Self::InvalidDeployerCoordinator(value)
        }
    }
    impl ::core::convert::From<InvalidFees> for HyperdriveFactoryErrors {
        fn from(value: InvalidFees) -> Self {
            Self::InvalidFees(value)
        }
    }
    impl ::core::convert::From<InvalidFixedAPR> for HyperdriveFactoryErrors {
        fn from(value: InvalidFixedAPR) -> Self {
            Self::InvalidFixedAPR(value)
        }
    }
    impl ::core::convert::From<InvalidIndexes> for HyperdriveFactoryErrors {
        fn from(value: InvalidIndexes) -> Self {
            Self::InvalidIndexes(value)
        }
    }
    impl ::core::convert::From<InvalidMaxCheckpointDuration>
    for HyperdriveFactoryErrors {
        fn from(value: InvalidMaxCheckpointDuration) -> Self {
            Self::InvalidMaxCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<InvalidMaxCircuitBreakerDelta>
    for HyperdriveFactoryErrors {
        fn from(value: InvalidMaxCircuitBreakerDelta) -> Self {
            Self::InvalidMaxCircuitBreakerDelta(value)
        }
    }
    impl ::core::convert::From<InvalidMaxFees> for HyperdriveFactoryErrors {
        fn from(value: InvalidMaxFees) -> Self {
            Self::InvalidMaxFees(value)
        }
    }
    impl ::core::convert::From<InvalidMaxFixedAPR> for HyperdriveFactoryErrors {
        fn from(value: InvalidMaxFixedAPR) -> Self {
            Self::InvalidMaxFixedAPR(value)
        }
    }
    impl ::core::convert::From<InvalidMaxPositionDuration> for HyperdriveFactoryErrors {
        fn from(value: InvalidMaxPositionDuration) -> Self {
            Self::InvalidMaxPositionDuration(value)
        }
    }
    impl ::core::convert::From<InvalidMaxTimeStretchAPR> for HyperdriveFactoryErrors {
        fn from(value: InvalidMaxTimeStretchAPR) -> Self {
            Self::InvalidMaxTimeStretchAPR(value)
        }
    }
    impl ::core::convert::From<InvalidMinCheckpointDuration>
    for HyperdriveFactoryErrors {
        fn from(value: InvalidMinCheckpointDuration) -> Self {
            Self::InvalidMinCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<InvalidMinCircuitBreakerDelta>
    for HyperdriveFactoryErrors {
        fn from(value: InvalidMinCircuitBreakerDelta) -> Self {
            Self::InvalidMinCircuitBreakerDelta(value)
        }
    }
    impl ::core::convert::From<InvalidMinFees> for HyperdriveFactoryErrors {
        fn from(value: InvalidMinFees) -> Self {
            Self::InvalidMinFees(value)
        }
    }
    impl ::core::convert::From<InvalidMinFixedAPR> for HyperdriveFactoryErrors {
        fn from(value: InvalidMinFixedAPR) -> Self {
            Self::InvalidMinFixedAPR(value)
        }
    }
    impl ::core::convert::From<InvalidMinPositionDuration> for HyperdriveFactoryErrors {
        fn from(value: InvalidMinPositionDuration) -> Self {
            Self::InvalidMinPositionDuration(value)
        }
    }
    impl ::core::convert::From<InvalidMinTimeStretchAPR> for HyperdriveFactoryErrors {
        fn from(value: InvalidMinTimeStretchAPR) -> Self {
            Self::InvalidMinTimeStretchAPR(value)
        }
    }
    impl ::core::convert::From<InvalidPositionDuration> for HyperdriveFactoryErrors {
        fn from(value: InvalidPositionDuration) -> Self {
            Self::InvalidPositionDuration(value)
        }
    }
    impl ::core::convert::From<InvalidTimeStretchAPR> for HyperdriveFactoryErrors {
        fn from(value: InvalidTimeStretchAPR) -> Self {
            Self::InvalidTimeStretchAPR(value)
        }
    }
    impl ::core::convert::From<LnInvalidInput> for HyperdriveFactoryErrors {
        fn from(value: LnInvalidInput) -> Self {
            Self::LnInvalidInput(value)
        }
    }
    impl ::core::convert::From<ReceiveLocked> for HyperdriveFactoryErrors {
        fn from(value: ReceiveLocked) -> Self {
            Self::ReceiveLocked(value)
        }
    }
    impl ::core::convert::From<TransferFailed> for HyperdriveFactoryErrors {
        fn from(value: TransferFailed) -> Self {
            Self::TransferFailed(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for HyperdriveFactoryErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToInt256> for HyperdriveFactoryErrors {
        fn from(value: UnsafeCastToInt256) -> Self {
            Self::UnsafeCastToInt256(value)
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
        name = "CheckpointDurationResolutionUpdated",
        abi = "CheckpointDurationResolutionUpdated(uint256)"
    )]
    pub struct CheckpointDurationResolutionUpdatedFilter {
        pub new_checkpoint_duration_resolution: ::ethers::core::types::U256,
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
    #[ethevent(name = "DefaultPausersUpdated", abi = "DefaultPausersUpdated(address[])")]
    pub struct DefaultPausersUpdatedFilter {
        pub new_default_pausers: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[ethevent(
        name = "Deployed",
        abi = "Deployed(address,address,string,(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256)),bytes)"
    )]
    pub struct DeployedFilter {
        #[ethevent(indexed)]
        pub deployer_coordinator: ::ethers::core::types::Address,
        pub hyperdrive: ::ethers::core::types::Address,
        pub name: ::std::string::String,
        pub config: PoolDeployConfig,
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
        name = "DeployerCoordinatorAdded",
        abi = "DeployerCoordinatorAdded(address)"
    )]
    pub struct DeployerCoordinatorAddedFilter {
        #[ethevent(indexed)]
        pub deployer_coordinator: ::ethers::core::types::Address,
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
        name = "DeployerCoordinatorManagerUpdated",
        abi = "DeployerCoordinatorManagerUpdated(address)"
    )]
    pub struct DeployerCoordinatorManagerUpdatedFilter {
        #[ethevent(indexed)]
        pub deployer_coordinator_manager: ::ethers::core::types::Address,
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
        name = "DeployerCoordinatorRemoved",
        abi = "DeployerCoordinatorRemoved(address)"
    )]
    pub struct DeployerCoordinatorRemovedFilter {
        #[ethevent(indexed)]
        pub deployer_coordinator: ::ethers::core::types::Address,
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
        pub governance: ::ethers::core::types::Address,
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
        name = "HyperdriveGovernanceUpdated",
        abi = "HyperdriveGovernanceUpdated(address)"
    )]
    pub struct HyperdriveGovernanceUpdatedFilter {
        #[ethevent(indexed)]
        pub hyperdrive_governance: ::ethers::core::types::Address,
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
    #[ethevent(name = "LinkerCodeHashUpdated", abi = "LinkerCodeHashUpdated(bytes32)")]
    pub struct LinkerCodeHashUpdatedFilter {
        #[ethevent(indexed)]
        pub new_linker_code_hash: [u8; 32],
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
    #[ethevent(name = "LinkerFactoryUpdated", abi = "LinkerFactoryUpdated(address)")]
    pub struct LinkerFactoryUpdatedFilter {
        #[ethevent(indexed)]
        pub new_linker_factory: ::ethers::core::types::Address,
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
        name = "MaxCheckpointDurationUpdated",
        abi = "MaxCheckpointDurationUpdated(uint256)"
    )]
    pub struct MaxCheckpointDurationUpdatedFilter {
        pub new_max_checkpoint_duration: ::ethers::core::types::U256,
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
        name = "MaxCircuitBreakerDeltaUpdated",
        abi = "MaxCircuitBreakerDeltaUpdated(uint256)"
    )]
    pub struct MaxCircuitBreakerDeltaUpdatedFilter {
        pub new_max_circuit_breaker_delta: ::ethers::core::types::U256,
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
        name = "MaxFeesUpdated",
        abi = "MaxFeesUpdated((uint256,uint256,uint256,uint256))"
    )]
    pub struct MaxFeesUpdatedFilter {
        pub new_max_fees: Fees,
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
    #[ethevent(name = "MaxFixedAPRUpdated", abi = "MaxFixedAPRUpdated(uint256)")]
    pub struct MaxFixedAPRUpdatedFilter {
        pub new_max_fixed_apr: ::ethers::core::types::U256,
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
        name = "MaxPositionDurationUpdated",
        abi = "MaxPositionDurationUpdated(uint256)"
    )]
    pub struct MaxPositionDurationUpdatedFilter {
        pub new_max_position_duration: ::ethers::core::types::U256,
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
        name = "MaxTimeStretchAPRUpdated",
        abi = "MaxTimeStretchAPRUpdated(uint256)"
    )]
    pub struct MaxTimeStretchAPRUpdatedFilter {
        pub new_max_time_stretch_apr: ::ethers::core::types::U256,
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
        name = "MinCheckpointDurationUpdated",
        abi = "MinCheckpointDurationUpdated(uint256)"
    )]
    pub struct MinCheckpointDurationUpdatedFilter {
        pub new_min_checkpoint_duration: ::ethers::core::types::U256,
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
        name = "MinCircuitBreakerDeltaUpdated",
        abi = "MinCircuitBreakerDeltaUpdated(uint256)"
    )]
    pub struct MinCircuitBreakerDeltaUpdatedFilter {
        pub new_min_circuit_breaker_delta: ::ethers::core::types::U256,
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
        name = "MinFeesUpdated",
        abi = "MinFeesUpdated((uint256,uint256,uint256,uint256))"
    )]
    pub struct MinFeesUpdatedFilter {
        pub new_min_fees: Fees,
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
    #[ethevent(name = "MinFixedAPRUpdated", abi = "MinFixedAPRUpdated(uint256)")]
    pub struct MinFixedAPRUpdatedFilter {
        pub new_min_fixed_apr: ::ethers::core::types::U256,
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
        name = "MinPositionDurationUpdated",
        abi = "MinPositionDurationUpdated(uint256)"
    )]
    pub struct MinPositionDurationUpdatedFilter {
        pub new_min_position_duration: ::ethers::core::types::U256,
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
        name = "MinTimeStretchAPRUpdated",
        abi = "MinTimeStretchAPRUpdated(uint256)"
    )]
    pub struct MinTimeStretchAPRUpdatedFilter {
        pub new_min_time_stretch_apr: ::ethers::core::types::U256,
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
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
    )]
    pub enum HyperdriveFactoryEvents {
        CheckpointDurationResolutionUpdatedFilter(
            CheckpointDurationResolutionUpdatedFilter,
        ),
        CheckpointRewarderUpdatedFilter(CheckpointRewarderUpdatedFilter),
        DefaultPausersUpdatedFilter(DefaultPausersUpdatedFilter),
        DeployedFilter(DeployedFilter),
        DeployerCoordinatorAddedFilter(DeployerCoordinatorAddedFilter),
        DeployerCoordinatorManagerUpdatedFilter(DeployerCoordinatorManagerUpdatedFilter),
        DeployerCoordinatorRemovedFilter(DeployerCoordinatorRemovedFilter),
        FeeCollectorUpdatedFilter(FeeCollectorUpdatedFilter),
        GovernanceUpdatedFilter(GovernanceUpdatedFilter),
        HyperdriveGovernanceUpdatedFilter(HyperdriveGovernanceUpdatedFilter),
        LinkerCodeHashUpdatedFilter(LinkerCodeHashUpdatedFilter),
        LinkerFactoryUpdatedFilter(LinkerFactoryUpdatedFilter),
        MaxCheckpointDurationUpdatedFilter(MaxCheckpointDurationUpdatedFilter),
        MaxCircuitBreakerDeltaUpdatedFilter(MaxCircuitBreakerDeltaUpdatedFilter),
        MaxFeesUpdatedFilter(MaxFeesUpdatedFilter),
        MaxFixedAPRUpdatedFilter(MaxFixedAPRUpdatedFilter),
        MaxPositionDurationUpdatedFilter(MaxPositionDurationUpdatedFilter),
        MaxTimeStretchAPRUpdatedFilter(MaxTimeStretchAPRUpdatedFilter),
        MinCheckpointDurationUpdatedFilter(MinCheckpointDurationUpdatedFilter),
        MinCircuitBreakerDeltaUpdatedFilter(MinCircuitBreakerDeltaUpdatedFilter),
        MinFeesUpdatedFilter(MinFeesUpdatedFilter),
        MinFixedAPRUpdatedFilter(MinFixedAPRUpdatedFilter),
        MinPositionDurationUpdatedFilter(MinPositionDurationUpdatedFilter),
        MinTimeStretchAPRUpdatedFilter(MinTimeStretchAPRUpdatedFilter),
        SweepCollectorUpdatedFilter(SweepCollectorUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for HyperdriveFactoryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CheckpointDurationResolutionUpdatedFilter::decode_log(
                log,
            ) {
                return Ok(
                    HyperdriveFactoryEvents::CheckpointDurationResolutionUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = CheckpointRewarderUpdatedFilter::decode_log(log) {
                return Ok(
                    HyperdriveFactoryEvents::CheckpointRewarderUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = DefaultPausersUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveFactoryEvents::DefaultPausersUpdatedFilter(decoded));
            }
            if let Ok(decoded) = DeployedFilter::decode_log(log) {
                return Ok(HyperdriveFactoryEvents::DeployedFilter(decoded));
            }
            if let Ok(decoded) = DeployerCoordinatorAddedFilter::decode_log(log) {
                return Ok(
                    HyperdriveFactoryEvents::DeployerCoordinatorAddedFilter(decoded),
                );
            }
            if let Ok(decoded) = DeployerCoordinatorManagerUpdatedFilter::decode_log(
                log,
            ) {
                return Ok(
                    HyperdriveFactoryEvents::DeployerCoordinatorManagerUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = DeployerCoordinatorRemovedFilter::decode_log(log) {
                return Ok(
                    HyperdriveFactoryEvents::DeployerCoordinatorRemovedFilter(decoded),
                );
            }
            if let Ok(decoded) = FeeCollectorUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveFactoryEvents::FeeCollectorUpdatedFilter(decoded));
            }
            if let Ok(decoded) = GovernanceUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveFactoryEvents::GovernanceUpdatedFilter(decoded));
            }
            if let Ok(decoded) = HyperdriveGovernanceUpdatedFilter::decode_log(log) {
                return Ok(
                    HyperdriveFactoryEvents::HyperdriveGovernanceUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = LinkerCodeHashUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveFactoryEvents::LinkerCodeHashUpdatedFilter(decoded));
            }
            if let Ok(decoded) = LinkerFactoryUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveFactoryEvents::LinkerFactoryUpdatedFilter(decoded));
            }
            if let Ok(decoded) = MaxCheckpointDurationUpdatedFilter::decode_log(log) {
                return Ok(
                    HyperdriveFactoryEvents::MaxCheckpointDurationUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MaxCircuitBreakerDeltaUpdatedFilter::decode_log(log) {
                return Ok(
                    HyperdriveFactoryEvents::MaxCircuitBreakerDeltaUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MaxFeesUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveFactoryEvents::MaxFeesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = MaxFixedAPRUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveFactoryEvents::MaxFixedAPRUpdatedFilter(decoded));
            }
            if let Ok(decoded) = MaxPositionDurationUpdatedFilter::decode_log(log) {
                return Ok(
                    HyperdriveFactoryEvents::MaxPositionDurationUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MaxTimeStretchAPRUpdatedFilter::decode_log(log) {
                return Ok(
                    HyperdriveFactoryEvents::MaxTimeStretchAPRUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MinCheckpointDurationUpdatedFilter::decode_log(log) {
                return Ok(
                    HyperdriveFactoryEvents::MinCheckpointDurationUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MinCircuitBreakerDeltaUpdatedFilter::decode_log(log) {
                return Ok(
                    HyperdriveFactoryEvents::MinCircuitBreakerDeltaUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MinFeesUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveFactoryEvents::MinFeesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = MinFixedAPRUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveFactoryEvents::MinFixedAPRUpdatedFilter(decoded));
            }
            if let Ok(decoded) = MinPositionDurationUpdatedFilter::decode_log(log) {
                return Ok(
                    HyperdriveFactoryEvents::MinPositionDurationUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MinTimeStretchAPRUpdatedFilter::decode_log(log) {
                return Ok(
                    HyperdriveFactoryEvents::MinTimeStretchAPRUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = SweepCollectorUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveFactoryEvents::SweepCollectorUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for HyperdriveFactoryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckpointDurationResolutionUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckpointRewarderUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultPausersUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeployerCoordinatorAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployerCoordinatorManagerUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployerCoordinatorRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeCollectorUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GovernanceUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HyperdriveGovernanceUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LinkerCodeHashUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LinkerFactoryUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxCheckpointDurationUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxCircuitBreakerDeltaUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxFeesUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxFixedAPRUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxPositionDurationUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxTimeStretchAPRUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinCheckpointDurationUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinCircuitBreakerDeltaUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinFeesUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinFixedAPRUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinPositionDurationUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinTimeStretchAPRUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SweepCollectorUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CheckpointDurationResolutionUpdatedFilter>
    for HyperdriveFactoryEvents {
        fn from(value: CheckpointDurationResolutionUpdatedFilter) -> Self {
            Self::CheckpointDurationResolutionUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<CheckpointRewarderUpdatedFilter>
    for HyperdriveFactoryEvents {
        fn from(value: CheckpointRewarderUpdatedFilter) -> Self {
            Self::CheckpointRewarderUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DefaultPausersUpdatedFilter> for HyperdriveFactoryEvents {
        fn from(value: DefaultPausersUpdatedFilter) -> Self {
            Self::DefaultPausersUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DeployedFilter> for HyperdriveFactoryEvents {
        fn from(value: DeployedFilter) -> Self {
            Self::DeployedFilter(value)
        }
    }
    impl ::core::convert::From<DeployerCoordinatorAddedFilter>
    for HyperdriveFactoryEvents {
        fn from(value: DeployerCoordinatorAddedFilter) -> Self {
            Self::DeployerCoordinatorAddedFilter(value)
        }
    }
    impl ::core::convert::From<DeployerCoordinatorManagerUpdatedFilter>
    for HyperdriveFactoryEvents {
        fn from(value: DeployerCoordinatorManagerUpdatedFilter) -> Self {
            Self::DeployerCoordinatorManagerUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DeployerCoordinatorRemovedFilter>
    for HyperdriveFactoryEvents {
        fn from(value: DeployerCoordinatorRemovedFilter) -> Self {
            Self::DeployerCoordinatorRemovedFilter(value)
        }
    }
    impl ::core::convert::From<FeeCollectorUpdatedFilter> for HyperdriveFactoryEvents {
        fn from(value: FeeCollectorUpdatedFilter) -> Self {
            Self::FeeCollectorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<GovernanceUpdatedFilter> for HyperdriveFactoryEvents {
        fn from(value: GovernanceUpdatedFilter) -> Self {
            Self::GovernanceUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<HyperdriveGovernanceUpdatedFilter>
    for HyperdriveFactoryEvents {
        fn from(value: HyperdriveGovernanceUpdatedFilter) -> Self {
            Self::HyperdriveGovernanceUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<LinkerCodeHashUpdatedFilter> for HyperdriveFactoryEvents {
        fn from(value: LinkerCodeHashUpdatedFilter) -> Self {
            Self::LinkerCodeHashUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<LinkerFactoryUpdatedFilter> for HyperdriveFactoryEvents {
        fn from(value: LinkerFactoryUpdatedFilter) -> Self {
            Self::LinkerFactoryUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MaxCheckpointDurationUpdatedFilter>
    for HyperdriveFactoryEvents {
        fn from(value: MaxCheckpointDurationUpdatedFilter) -> Self {
            Self::MaxCheckpointDurationUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MaxCircuitBreakerDeltaUpdatedFilter>
    for HyperdriveFactoryEvents {
        fn from(value: MaxCircuitBreakerDeltaUpdatedFilter) -> Self {
            Self::MaxCircuitBreakerDeltaUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MaxFeesUpdatedFilter> for HyperdriveFactoryEvents {
        fn from(value: MaxFeesUpdatedFilter) -> Self {
            Self::MaxFeesUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MaxFixedAPRUpdatedFilter> for HyperdriveFactoryEvents {
        fn from(value: MaxFixedAPRUpdatedFilter) -> Self {
            Self::MaxFixedAPRUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MaxPositionDurationUpdatedFilter>
    for HyperdriveFactoryEvents {
        fn from(value: MaxPositionDurationUpdatedFilter) -> Self {
            Self::MaxPositionDurationUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MaxTimeStretchAPRUpdatedFilter>
    for HyperdriveFactoryEvents {
        fn from(value: MaxTimeStretchAPRUpdatedFilter) -> Self {
            Self::MaxTimeStretchAPRUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MinCheckpointDurationUpdatedFilter>
    for HyperdriveFactoryEvents {
        fn from(value: MinCheckpointDurationUpdatedFilter) -> Self {
            Self::MinCheckpointDurationUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MinCircuitBreakerDeltaUpdatedFilter>
    for HyperdriveFactoryEvents {
        fn from(value: MinCircuitBreakerDeltaUpdatedFilter) -> Self {
            Self::MinCircuitBreakerDeltaUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MinFeesUpdatedFilter> for HyperdriveFactoryEvents {
        fn from(value: MinFeesUpdatedFilter) -> Self {
            Self::MinFeesUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MinFixedAPRUpdatedFilter> for HyperdriveFactoryEvents {
        fn from(value: MinFixedAPRUpdatedFilter) -> Self {
            Self::MinFixedAPRUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MinPositionDurationUpdatedFilter>
    for HyperdriveFactoryEvents {
        fn from(value: MinPositionDurationUpdatedFilter) -> Self {
            Self::MinPositionDurationUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MinTimeStretchAPRUpdatedFilter>
    for HyperdriveFactoryEvents {
        fn from(value: MinTimeStretchAPRUpdatedFilter) -> Self {
            Self::MinTimeStretchAPRUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<SweepCollectorUpdatedFilter> for HyperdriveFactoryEvents {
        fn from(value: SweepCollectorUpdatedFilter) -> Self {
            Self::SweepCollectorUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `_instancesToDeployerCoordinators` function with signature `_instancesToDeployerCoordinators(address)` and selector `0x2b58f418`
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
        name = "_instancesToDeployerCoordinators",
        abi = "_instancesToDeployerCoordinators(address)"
    )]
    pub struct InstancesToDeployerCoordinatorsCall {
        pub instance: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addDeployerCoordinator` function with signature `addDeployerCoordinator(address)` and selector `0x421caba8`
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
    #[ethcall(name = "addDeployerCoordinator", abi = "addDeployerCoordinator(address)")]
    pub struct AddDeployerCoordinatorCall {
        pub deployer_coordinator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `checkpointDurationResolution` function with signature `checkpointDurationResolution()` and selector `0xd0f96b92`
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
        name = "checkpointDurationResolution",
        abi = "checkpointDurationResolution()"
    )]
    pub struct CheckpointDurationResolutionCall;
    ///Container type for all input parameters for the `checkpointRewarder` function with signature `checkpointRewarder()` and selector `0xf2596458`
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
    #[ethcall(name = "checkpointRewarder", abi = "checkpointRewarder()")]
    pub struct CheckpointRewarderCall;
    ///Container type for all input parameters for the `defaultPausers` function with signature `defaultPausers()` and selector `0xa64c90bf`
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
    #[ethcall(name = "defaultPausers", abi = "defaultPausers()")]
    pub struct DefaultPausersCall;
    ///Container type for all input parameters for the `deployAndInitialize` function with signature `deployAndInitialize(bytes32,address,string,(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256)),bytes,uint256,uint256,uint256,(address,bool,bytes),bytes32)` and selector `0x2e7cd971`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[ethcall(
        name = "deployAndInitialize",
        abi = "deployAndInitialize(bytes32,address,string,(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256)),bytes,uint256,uint256,uint256,(address,bool,bytes),bytes32)"
    )]
    pub struct DeployAndInitializeCall {
        pub deployment_id: [u8; 32],
        pub deployer_coordinator: ::ethers::core::types::Address,
        pub name: ::std::string::String,
        pub config: PoolDeployConfig,
        pub extra_data: ::ethers::core::types::Bytes,
        pub contribution: ::ethers::core::types::U256,
        pub fixed_apr: ::ethers::core::types::U256,
        pub time_stretch_apr: ::ethers::core::types::U256,
        pub options: Options,
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `deployTarget` function with signature `deployTarget(bytes32,address,(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256)),bytes,uint256,uint256,uint256,bytes32)` and selector `0x49f13de7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[ethcall(
        name = "deployTarget",
        abi = "deployTarget(bytes32,address,(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256)),bytes,uint256,uint256,uint256,bytes32)"
    )]
    pub struct DeployTargetCall {
        pub deployment_id: [u8; 32],
        pub deployer_coordinator: ::ethers::core::types::Address,
        pub config: PoolDeployConfig,
        pub extra_data: ::ethers::core::types::Bytes,
        pub fixed_apr: ::ethers::core::types::U256,
        pub time_stretch_apr: ::ethers::core::types::U256,
        pub target_index: ::ethers::core::types::U256,
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `deployerCoordinatorManager` function with signature `deployerCoordinatorManager()` and selector `0xe4e7148f`
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
    #[ethcall(name = "deployerCoordinatorManager", abi = "deployerCoordinatorManager()")]
    pub struct DeployerCoordinatorManagerCall;
    ///Container type for all input parameters for the `feeCollector` function with signature `feeCollector()` and selector `0xc415b95c`
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
    #[ethcall(name = "feeCollector", abi = "feeCollector()")]
    pub struct FeeCollectorCall;
    ///Container type for all input parameters for the `getDeployerCoordinatorAtIndex` function with signature `getDeployerCoordinatorAtIndex(uint256)` and selector `0xfe3d5aeb`
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
        name = "getDeployerCoordinatorAtIndex",
        abi = "getDeployerCoordinatorAtIndex(uint256)"
    )]
    pub struct GetDeployerCoordinatorAtIndexCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getDeployerCoordinatorByInstances` function with signature `getDeployerCoordinatorByInstances(address[])` and selector `0x1b59be0c`
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
        name = "getDeployerCoordinatorByInstances",
        abi = "getDeployerCoordinatorByInstances(address[])"
    )]
    pub struct GetDeployerCoordinatorByInstancesCall {
        pub instances: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getDeployerCoordinatorsInRange` function with signature `getDeployerCoordinatorsInRange(uint256,uint256)` and selector `0xec895f11`
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
        name = "getDeployerCoordinatorsInRange",
        abi = "getDeployerCoordinatorsInRange(uint256,uint256)"
    )]
    pub struct GetDeployerCoordinatorsInRangeCall {
        pub start_index: ::ethers::core::types::U256,
        pub end_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getInstanceAtIndex` function with signature `getInstanceAtIndex(uint256)` and selector `0xdaac24da`
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
    #[ethcall(name = "getInstanceAtIndex", abi = "getInstanceAtIndex(uint256)")]
    pub struct GetInstanceAtIndexCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getInstancesInRange` function with signature `getInstancesInRange(uint256,uint256)` and selector `0xbc30e7a1`
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
        name = "getInstancesInRange",
        abi = "getInstancesInRange(uint256,uint256)"
    )]
    pub struct GetInstancesInRangeCall {
        pub start_index: ::ethers::core::types::U256,
        pub end_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNumberOfDeployerCoordinators` function with signature `getNumberOfDeployerCoordinators()` and selector `0xe1b39c80`
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
        name = "getNumberOfDeployerCoordinators",
        abi = "getNumberOfDeployerCoordinators()"
    )]
    pub struct GetNumberOfDeployerCoordinatorsCall;
    ///Container type for all input parameters for the `getNumberOfInstances` function with signature `getNumberOfInstances()` and selector `0x6e95d67c`
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
    #[ethcall(name = "getNumberOfInstances", abi = "getNumberOfInstances()")]
    pub struct GetNumberOfInstancesCall;
    ///Container type for all input parameters for the `governance` function with signature `governance()` and selector `0x5aa6e675`
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
    #[ethcall(name = "governance", abi = "governance()")]
    pub struct GovernanceCall;
    ///Container type for all input parameters for the `hyperdriveGovernance` function with signature `hyperdriveGovernance()` and selector `0xe3331555`
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
    #[ethcall(name = "hyperdriveGovernance", abi = "hyperdriveGovernance()")]
    pub struct HyperdriveGovernanceCall;
    ///Container type for all input parameters for the `isDeployerCoordinator` function with signature `isDeployerCoordinator(address)` and selector `0xf8c09e59`
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
    #[ethcall(name = "isDeployerCoordinator", abi = "isDeployerCoordinator(address)")]
    pub struct IsDeployerCoordinatorCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `isInstance` function with signature `isInstance(address)` and selector `0x6b44e6be`
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
    #[ethcall(name = "isInstance", abi = "isInstance(address)")]
    pub struct IsInstanceCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `linkerCodeHash` function with signature `linkerCodeHash()` and selector `0xc905a4b5`
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
    #[ethcall(name = "linkerCodeHash", abi = "linkerCodeHash()")]
    pub struct LinkerCodeHashCall;
    ///Container type for all input parameters for the `linkerFactory` function with signature `linkerFactory()` and selector `0x99623bb1`
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
    #[ethcall(name = "linkerFactory", abi = "linkerFactory()")]
    pub struct LinkerFactoryCall;
    ///Container type for all input parameters for the `maxCheckpointDuration` function with signature `maxCheckpointDuration()` and selector `0xe0e2daaa`
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
    #[ethcall(name = "maxCheckpointDuration", abi = "maxCheckpointDuration()")]
    pub struct MaxCheckpointDurationCall;
    ///Container type for all input parameters for the `maxCircuitBreakerDelta` function with signature `maxCircuitBreakerDelta()` and selector `0x4554f9a9`
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
    #[ethcall(name = "maxCircuitBreakerDelta", abi = "maxCircuitBreakerDelta()")]
    pub struct MaxCircuitBreakerDeltaCall;
    ///Container type for all input parameters for the `maxFees` function with signature `maxFees()` and selector `0xe83e34b1`
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
    #[ethcall(name = "maxFees", abi = "maxFees()")]
    pub struct MaxFeesCall;
    ///Container type for all input parameters for the `maxFixedAPR` function with signature `maxFixedAPR()` and selector `0xbf9bd5cd`
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
    #[ethcall(name = "maxFixedAPR", abi = "maxFixedAPR()")]
    pub struct MaxFixedAPRCall;
    ///Container type for all input parameters for the `maxPositionDuration` function with signature `maxPositionDuration()` and selector `0x8efc0986`
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
    #[ethcall(name = "maxPositionDuration", abi = "maxPositionDuration()")]
    pub struct MaxPositionDurationCall;
    ///Container type for all input parameters for the `maxTimeStretchAPR` function with signature `maxTimeStretchAPR()` and selector `0x48800760`
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
    #[ethcall(name = "maxTimeStretchAPR", abi = "maxTimeStretchAPR()")]
    pub struct MaxTimeStretchAPRCall;
    ///Container type for all input parameters for the `minCheckpointDuration` function with signature `minCheckpointDuration()` and selector `0x5720c9d5`
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
    #[ethcall(name = "minCheckpointDuration", abi = "minCheckpointDuration()")]
    pub struct MinCheckpointDurationCall;
    ///Container type for all input parameters for the `minCircuitBreakerDelta` function with signature `minCircuitBreakerDelta()` and selector `0x1ecda0fe`
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
    #[ethcall(name = "minCircuitBreakerDelta", abi = "minCircuitBreakerDelta()")]
    pub struct MinCircuitBreakerDeltaCall;
    ///Container type for all input parameters for the `minFees` function with signature `minFees()` and selector `0xc1722563`
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
    #[ethcall(name = "minFees", abi = "minFees()")]
    pub struct MinFeesCall;
    ///Container type for all input parameters for the `minFixedAPR` function with signature `minFixedAPR()` and selector `0xd23d7ea3`
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
    #[ethcall(name = "minFixedAPR", abi = "minFixedAPR()")]
    pub struct MinFixedAPRCall;
    ///Container type for all input parameters for the `minPositionDuration` function with signature `minPositionDuration()` and selector `0xdaf012e6`
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
    #[ethcall(name = "minPositionDuration", abi = "minPositionDuration()")]
    pub struct MinPositionDurationCall;
    ///Container type for all input parameters for the `minTimeStretchAPR` function with signature `minTimeStretchAPR()` and selector `0xd6f50169`
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
    #[ethcall(name = "minTimeStretchAPR", abi = "minTimeStretchAPR()")]
    pub struct MinTimeStretchAPRCall;
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
    ///Container type for all input parameters for the `removeDeployerCoordinator` function with signature `removeDeployerCoordinator(address,uint256)` and selector `0x411c3035`
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
        name = "removeDeployerCoordinator",
        abi = "removeDeployerCoordinator(address,uint256)"
    )]
    pub struct RemoveDeployerCoordinatorCall {
        pub deployer_coordinator: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sweepCollector` function with signature `sweepCollector()` and selector `0x10780f73`
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
    #[ethcall(name = "sweepCollector", abi = "sweepCollector()")]
    pub struct SweepCollectorCall;
    ///Container type for all input parameters for the `updateCheckpointDurationResolution` function with signature `updateCheckpointDurationResolution(uint256)` and selector `0x11e77bfe`
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
        name = "updateCheckpointDurationResolution",
        abi = "updateCheckpointDurationResolution(uint256)"
    )]
    pub struct UpdateCheckpointDurationResolutionCall {
        pub checkpoint_duration_resolution: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateCheckpointRewarder` function with signature `updateCheckpointRewarder(address)` and selector `0x3e2d2014`
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
        name = "updateCheckpointRewarder",
        abi = "updateCheckpointRewarder(address)"
    )]
    pub struct UpdateCheckpointRewarderCall {
        pub checkpoint_rewarder: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateDefaultPausers` function with signature `updateDefaultPausers(address[])` and selector `0x9af25262`
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
    #[ethcall(name = "updateDefaultPausers", abi = "updateDefaultPausers(address[])")]
    pub struct UpdateDefaultPausersCall {
        pub default_pausers: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `updateDeployerCoordinatorManager` function with signature `updateDeployerCoordinatorManager(address)` and selector `0xa98a46db`
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
        name = "updateDeployerCoordinatorManager",
        abi = "updateDeployerCoordinatorManager(address)"
    )]
    pub struct UpdateDeployerCoordinatorManagerCall {
        pub deployer_coordinator_manager: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateFeeCollector` function with signature `updateFeeCollector(address)` and selector `0xd2c35ce8`
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
    #[ethcall(name = "updateFeeCollector", abi = "updateFeeCollector(address)")]
    pub struct UpdateFeeCollectorCall {
        pub fee_collector: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateGovernance` function with signature `updateGovernance(address)` and selector `0xb2561263`
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
    #[ethcall(name = "updateGovernance", abi = "updateGovernance(address)")]
    pub struct UpdateGovernanceCall {
        pub governance: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateHyperdriveGovernance` function with signature `updateHyperdriveGovernance(address)` and selector `0xdd2b8fbb`
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
        name = "updateHyperdriveGovernance",
        abi = "updateHyperdriveGovernance(address)"
    )]
    pub struct UpdateHyperdriveGovernanceCall {
        pub hyperdrive_governance: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateLinkerCodeHash` function with signature `updateLinkerCodeHash(bytes32)` and selector `0x4fbfee77`
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
    #[ethcall(name = "updateLinkerCodeHash", abi = "updateLinkerCodeHash(bytes32)")]
    pub struct UpdateLinkerCodeHashCall {
        pub linker_code_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `updateLinkerFactory` function with signature `updateLinkerFactory(address)` and selector `0x85229785`
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
    #[ethcall(name = "updateLinkerFactory", abi = "updateLinkerFactory(address)")]
    pub struct UpdateLinkerFactoryCall {
        pub linker_factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateMaxCheckpointDuration` function with signature `updateMaxCheckpointDuration(uint256)` and selector `0x6f6d5c4a`
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
        name = "updateMaxCheckpointDuration",
        abi = "updateMaxCheckpointDuration(uint256)"
    )]
    pub struct UpdateMaxCheckpointDurationCall {
        pub max_checkpoint_duration: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateMaxCircuitBreakerDelta` function with signature `updateMaxCircuitBreakerDelta(uint256)` and selector `0x84c19aab`
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
        name = "updateMaxCircuitBreakerDelta",
        abi = "updateMaxCircuitBreakerDelta(uint256)"
    )]
    pub struct UpdateMaxCircuitBreakerDeltaCall {
        pub max_circuit_breaker_delta: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateMaxFees` function with signature `updateMaxFees((uint256,uint256,uint256,uint256))` and selector `0x2885e3ac`
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
        name = "updateMaxFees",
        abi = "updateMaxFees((uint256,uint256,uint256,uint256))"
    )]
    pub struct UpdateMaxFeesCall {
        pub max_fees: Fees,
    }
    ///Container type for all input parameters for the `updateMaxFixedAPR` function with signature `updateMaxFixedAPR(uint256)` and selector `0x97b0e8ce`
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
    #[ethcall(name = "updateMaxFixedAPR", abi = "updateMaxFixedAPR(uint256)")]
    pub struct UpdateMaxFixedAPRCall {
        pub max_fixed_apr: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateMaxPositionDuration` function with signature `updateMaxPositionDuration(uint256)` and selector `0xeb71f66c`
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
        name = "updateMaxPositionDuration",
        abi = "updateMaxPositionDuration(uint256)"
    )]
    pub struct UpdateMaxPositionDurationCall {
        pub max_position_duration: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateMaxTimeStretchAPR` function with signature `updateMaxTimeStretchAPR(uint256)` and selector `0x628027a3`
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
        name = "updateMaxTimeStretchAPR",
        abi = "updateMaxTimeStretchAPR(uint256)"
    )]
    pub struct UpdateMaxTimeStretchAPRCall {
        pub max_time_stretch_apr: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateMinCheckpointDuration` function with signature `updateMinCheckpointDuration(uint256)` and selector `0x8e127cf5`
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
        name = "updateMinCheckpointDuration",
        abi = "updateMinCheckpointDuration(uint256)"
    )]
    pub struct UpdateMinCheckpointDurationCall {
        pub min_checkpoint_duration: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateMinCircuitBreakerDelta` function with signature `updateMinCircuitBreakerDelta(uint256)` and selector `0x2907d3dd`
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
        name = "updateMinCircuitBreakerDelta",
        abi = "updateMinCircuitBreakerDelta(uint256)"
    )]
    pub struct UpdateMinCircuitBreakerDeltaCall {
        pub min_circuit_breaker_delta: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateMinFees` function with signature `updateMinFees((uint256,uint256,uint256,uint256))` and selector `0x10d1dc3e`
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
        name = "updateMinFees",
        abi = "updateMinFees((uint256,uint256,uint256,uint256))"
    )]
    pub struct UpdateMinFeesCall {
        pub min_fees: Fees,
    }
    ///Container type for all input parameters for the `updateMinFixedAPR` function with signature `updateMinFixedAPR(uint256)` and selector `0x1978ebcf`
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
    #[ethcall(name = "updateMinFixedAPR", abi = "updateMinFixedAPR(uint256)")]
    pub struct UpdateMinFixedAPRCall {
        pub min_fixed_apr: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateMinPositionDuration` function with signature `updateMinPositionDuration(uint256)` and selector `0xe71f34b3`
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
        name = "updateMinPositionDuration",
        abi = "updateMinPositionDuration(uint256)"
    )]
    pub struct UpdateMinPositionDurationCall {
        pub min_position_duration: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateMinTimeStretchAPR` function with signature `updateMinTimeStretchAPR(uint256)` and selector `0x83b361e8`
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
        name = "updateMinTimeStretchAPR",
        abi = "updateMinTimeStretchAPR(uint256)"
    )]
    pub struct UpdateMinTimeStretchAPRCall {
        pub min_time_stretch_apr: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateSweepCollector` function with signature `updateSweepCollector(address)` and selector `0x8627a4f0`
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
    #[ethcall(name = "updateSweepCollector", abi = "updateSweepCollector(address)")]
    pub struct UpdateSweepCollectorCall {
        pub sweep_collector: ::ethers::core::types::Address,
    }
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
    pub enum HyperdriveFactoryCalls {
        InstancesToDeployerCoordinators(InstancesToDeployerCoordinatorsCall),
        AddDeployerCoordinator(AddDeployerCoordinatorCall),
        CheckpointDurationResolution(CheckpointDurationResolutionCall),
        CheckpointRewarder(CheckpointRewarderCall),
        DefaultPausers(DefaultPausersCall),
        DeployAndInitialize(DeployAndInitializeCall),
        DeployTarget(DeployTargetCall),
        DeployerCoordinatorManager(DeployerCoordinatorManagerCall),
        FeeCollector(FeeCollectorCall),
        GetDeployerCoordinatorAtIndex(GetDeployerCoordinatorAtIndexCall),
        GetDeployerCoordinatorByInstances(GetDeployerCoordinatorByInstancesCall),
        GetDeployerCoordinatorsInRange(GetDeployerCoordinatorsInRangeCall),
        GetInstanceAtIndex(GetInstanceAtIndexCall),
        GetInstancesInRange(GetInstancesInRangeCall),
        GetNumberOfDeployerCoordinators(GetNumberOfDeployerCoordinatorsCall),
        GetNumberOfInstances(GetNumberOfInstancesCall),
        Governance(GovernanceCall),
        HyperdriveGovernance(HyperdriveGovernanceCall),
        IsDeployerCoordinator(IsDeployerCoordinatorCall),
        IsInstance(IsInstanceCall),
        Kind(KindCall),
        LinkerCodeHash(LinkerCodeHashCall),
        LinkerFactory(LinkerFactoryCall),
        MaxCheckpointDuration(MaxCheckpointDurationCall),
        MaxCircuitBreakerDelta(MaxCircuitBreakerDeltaCall),
        MaxFees(MaxFeesCall),
        MaxFixedAPR(MaxFixedAPRCall),
        MaxPositionDuration(MaxPositionDurationCall),
        MaxTimeStretchAPR(MaxTimeStretchAPRCall),
        MinCheckpointDuration(MinCheckpointDurationCall),
        MinCircuitBreakerDelta(MinCircuitBreakerDeltaCall),
        MinFees(MinFeesCall),
        MinFixedAPR(MinFixedAPRCall),
        MinPositionDuration(MinPositionDurationCall),
        MinTimeStretchAPR(MinTimeStretchAPRCall),
        Name(NameCall),
        RemoveDeployerCoordinator(RemoveDeployerCoordinatorCall),
        SweepCollector(SweepCollectorCall),
        UpdateCheckpointDurationResolution(UpdateCheckpointDurationResolutionCall),
        UpdateCheckpointRewarder(UpdateCheckpointRewarderCall),
        UpdateDefaultPausers(UpdateDefaultPausersCall),
        UpdateDeployerCoordinatorManager(UpdateDeployerCoordinatorManagerCall),
        UpdateFeeCollector(UpdateFeeCollectorCall),
        UpdateGovernance(UpdateGovernanceCall),
        UpdateHyperdriveGovernance(UpdateHyperdriveGovernanceCall),
        UpdateLinkerCodeHash(UpdateLinkerCodeHashCall),
        UpdateLinkerFactory(UpdateLinkerFactoryCall),
        UpdateMaxCheckpointDuration(UpdateMaxCheckpointDurationCall),
        UpdateMaxCircuitBreakerDelta(UpdateMaxCircuitBreakerDeltaCall),
        UpdateMaxFees(UpdateMaxFeesCall),
        UpdateMaxFixedAPR(UpdateMaxFixedAPRCall),
        UpdateMaxPositionDuration(UpdateMaxPositionDurationCall),
        UpdateMaxTimeStretchAPR(UpdateMaxTimeStretchAPRCall),
        UpdateMinCheckpointDuration(UpdateMinCheckpointDurationCall),
        UpdateMinCircuitBreakerDelta(UpdateMinCircuitBreakerDeltaCall),
        UpdateMinFees(UpdateMinFeesCall),
        UpdateMinFixedAPR(UpdateMinFixedAPRCall),
        UpdateMinPositionDuration(UpdateMinPositionDurationCall),
        UpdateMinTimeStretchAPR(UpdateMinTimeStretchAPRCall),
        UpdateSweepCollector(UpdateSweepCollectorCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for HyperdriveFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <InstancesToDeployerCoordinatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InstancesToDeployerCoordinators(decoded));
            }
            if let Ok(decoded) = <AddDeployerCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddDeployerCoordinator(decoded));
            }
            if let Ok(decoded) = <CheckpointDurationResolutionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckpointDurationResolution(decoded));
            }
            if let Ok(decoded) = <CheckpointRewarderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckpointRewarder(decoded));
            }
            if let Ok(decoded) = <DefaultPausersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultPausers(decoded));
            }
            if let Ok(decoded) = <DeployAndInitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployAndInitialize(decoded));
            }
            if let Ok(decoded) = <DeployTargetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployTarget(decoded));
            }
            if let Ok(decoded) = <DeployerCoordinatorManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployerCoordinatorManager(decoded));
            }
            if let Ok(decoded) = <FeeCollectorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeCollector(decoded));
            }
            if let Ok(decoded) = <GetDeployerCoordinatorAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDeployerCoordinatorAtIndex(decoded));
            }
            if let Ok(decoded) = <GetDeployerCoordinatorByInstancesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDeployerCoordinatorByInstances(decoded));
            }
            if let Ok(decoded) = <GetDeployerCoordinatorsInRangeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDeployerCoordinatorsInRange(decoded));
            }
            if let Ok(decoded) = <GetInstanceAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInstanceAtIndex(decoded));
            }
            if let Ok(decoded) = <GetInstancesInRangeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInstancesInRange(decoded));
            }
            if let Ok(decoded) = <GetNumberOfDeployerCoordinatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNumberOfDeployerCoordinators(decoded));
            }
            if let Ok(decoded) = <GetNumberOfInstancesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNumberOfInstances(decoded));
            }
            if let Ok(decoded) = <GovernanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Governance(decoded));
            }
            if let Ok(decoded) = <HyperdriveGovernanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HyperdriveGovernance(decoded));
            }
            if let Ok(decoded) = <IsDeployerCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsDeployerCoordinator(decoded));
            }
            if let Ok(decoded) = <IsInstanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsInstance(decoded));
            }
            if let Ok(decoded) = <KindCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Kind(decoded));
            }
            if let Ok(decoded) = <LinkerCodeHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LinkerCodeHash(decoded));
            }
            if let Ok(decoded) = <LinkerFactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LinkerFactory(decoded));
            }
            if let Ok(decoded) = <MaxCheckpointDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxCheckpointDuration(decoded));
            }
            if let Ok(decoded) = <MaxCircuitBreakerDeltaCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxCircuitBreakerDelta(decoded));
            }
            if let Ok(decoded) = <MaxFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxFees(decoded));
            }
            if let Ok(decoded) = <MaxFixedAPRCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxFixedAPR(decoded));
            }
            if let Ok(decoded) = <MaxPositionDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxPositionDuration(decoded));
            }
            if let Ok(decoded) = <MaxTimeStretchAPRCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxTimeStretchAPR(decoded));
            }
            if let Ok(decoded) = <MinCheckpointDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinCheckpointDuration(decoded));
            }
            if let Ok(decoded) = <MinCircuitBreakerDeltaCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinCircuitBreakerDelta(decoded));
            }
            if let Ok(decoded) = <MinFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinFees(decoded));
            }
            if let Ok(decoded) = <MinFixedAPRCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinFixedAPR(decoded));
            }
            if let Ok(decoded) = <MinPositionDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinPositionDuration(decoded));
            }
            if let Ok(decoded) = <MinTimeStretchAPRCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinTimeStretchAPR(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <RemoveDeployerCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveDeployerCoordinator(decoded));
            }
            if let Ok(decoded) = <SweepCollectorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SweepCollector(decoded));
            }
            if let Ok(decoded) = <UpdateCheckpointDurationResolutionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateCheckpointDurationResolution(decoded));
            }
            if let Ok(decoded) = <UpdateCheckpointRewarderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateCheckpointRewarder(decoded));
            }
            if let Ok(decoded) = <UpdateDefaultPausersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateDefaultPausers(decoded));
            }
            if let Ok(decoded) = <UpdateDeployerCoordinatorManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateDeployerCoordinatorManager(decoded));
            }
            if let Ok(decoded) = <UpdateFeeCollectorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateFeeCollector(decoded));
            }
            if let Ok(decoded) = <UpdateGovernanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateGovernance(decoded));
            }
            if let Ok(decoded) = <UpdateHyperdriveGovernanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateHyperdriveGovernance(decoded));
            }
            if let Ok(decoded) = <UpdateLinkerCodeHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateLinkerCodeHash(decoded));
            }
            if let Ok(decoded) = <UpdateLinkerFactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateLinkerFactory(decoded));
            }
            if let Ok(decoded) = <UpdateMaxCheckpointDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateMaxCheckpointDuration(decoded));
            }
            if let Ok(decoded) = <UpdateMaxCircuitBreakerDeltaCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateMaxCircuitBreakerDelta(decoded));
            }
            if let Ok(decoded) = <UpdateMaxFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateMaxFees(decoded));
            }
            if let Ok(decoded) = <UpdateMaxFixedAPRCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateMaxFixedAPR(decoded));
            }
            if let Ok(decoded) = <UpdateMaxPositionDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateMaxPositionDuration(decoded));
            }
            if let Ok(decoded) = <UpdateMaxTimeStretchAPRCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateMaxTimeStretchAPR(decoded));
            }
            if let Ok(decoded) = <UpdateMinCheckpointDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateMinCheckpointDuration(decoded));
            }
            if let Ok(decoded) = <UpdateMinCircuitBreakerDeltaCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateMinCircuitBreakerDelta(decoded));
            }
            if let Ok(decoded) = <UpdateMinFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateMinFees(decoded));
            }
            if let Ok(decoded) = <UpdateMinFixedAPRCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateMinFixedAPR(decoded));
            }
            if let Ok(decoded) = <UpdateMinPositionDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateMinPositionDuration(decoded));
            }
            if let Ok(decoded) = <UpdateMinTimeStretchAPRCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateMinTimeStretchAPR(decoded));
            }
            if let Ok(decoded) = <UpdateSweepCollectorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateSweepCollector(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for HyperdriveFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::InstancesToDeployerCoordinators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddDeployerCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckpointDurationResolution(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckpointRewarder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultPausers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployAndInitialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployTarget(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployerCoordinatorManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeCollector(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDeployerCoordinatorAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDeployerCoordinatorByInstances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDeployerCoordinatorsInRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInstanceAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInstancesInRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNumberOfDeployerCoordinators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNumberOfInstances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Governance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HyperdriveGovernance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDeployerCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsInstance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Kind(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LinkerCodeHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LinkerFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxCheckpointDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxCircuitBreakerDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxFixedAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxPositionDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxTimeStretchAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinCheckpointDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinCircuitBreakerDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MinFixedAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinPositionDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinTimeStretchAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveDeployerCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SweepCollector(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateCheckpointDurationResolution(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateCheckpointRewarder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateDefaultPausers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateDeployerCoordinatorManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateFeeCollector(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateGovernance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateHyperdriveGovernance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateLinkerCodeHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateLinkerFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMaxCheckpointDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMaxCircuitBreakerDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMaxFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMaxFixedAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMaxPositionDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMaxTimeStretchAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMinCheckpointDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMinCircuitBreakerDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMinFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMinFixedAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMinPositionDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMinTimeStretchAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateSweepCollector(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for HyperdriveFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InstancesToDeployerCoordinators(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddDeployerCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckpointDurationResolution(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckpointRewarder(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultPausers(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeployAndInitialize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployTarget(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeployerCoordinatorManager(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeCollector(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDeployerCoordinatorAtIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetDeployerCoordinatorByInstances(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetDeployerCoordinatorsInRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetInstanceAtIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetInstancesInRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetNumberOfDeployerCoordinators(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetNumberOfInstances(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Governance(element) => ::core::fmt::Display::fmt(element, f),
                Self::HyperdriveGovernance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsDeployerCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsInstance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Kind(element) => ::core::fmt::Display::fmt(element, f),
                Self::LinkerCodeHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::LinkerFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxCheckpointDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxCircuitBreakerDelta(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxFixedAPR(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxPositionDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxTimeStretchAPR(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinCheckpointDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinCircuitBreakerDelta(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinFixedAPR(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinPositionDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinTimeStretchAPR(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveDeployerCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SweepCollector(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateCheckpointDurationResolution(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateCheckpointRewarder(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateDefaultPausers(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateDeployerCoordinatorManager(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateFeeCollector(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateGovernance(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateHyperdriveGovernance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateLinkerCodeHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateLinkerFactory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateMaxCheckpointDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateMaxCircuitBreakerDelta(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateMaxFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateMaxFixedAPR(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateMaxPositionDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateMaxTimeStretchAPR(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateMinCheckpointDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateMinCircuitBreakerDelta(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateMinFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateMinFixedAPR(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateMinPositionDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateMinTimeStretchAPR(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateSweepCollector(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InstancesToDeployerCoordinatorsCall>
    for HyperdriveFactoryCalls {
        fn from(value: InstancesToDeployerCoordinatorsCall) -> Self {
            Self::InstancesToDeployerCoordinators(value)
        }
    }
    impl ::core::convert::From<AddDeployerCoordinatorCall> for HyperdriveFactoryCalls {
        fn from(value: AddDeployerCoordinatorCall) -> Self {
            Self::AddDeployerCoordinator(value)
        }
    }
    impl ::core::convert::From<CheckpointDurationResolutionCall>
    for HyperdriveFactoryCalls {
        fn from(value: CheckpointDurationResolutionCall) -> Self {
            Self::CheckpointDurationResolution(value)
        }
    }
    impl ::core::convert::From<CheckpointRewarderCall> for HyperdriveFactoryCalls {
        fn from(value: CheckpointRewarderCall) -> Self {
            Self::CheckpointRewarder(value)
        }
    }
    impl ::core::convert::From<DefaultPausersCall> for HyperdriveFactoryCalls {
        fn from(value: DefaultPausersCall) -> Self {
            Self::DefaultPausers(value)
        }
    }
    impl ::core::convert::From<DeployAndInitializeCall> for HyperdriveFactoryCalls {
        fn from(value: DeployAndInitializeCall) -> Self {
            Self::DeployAndInitialize(value)
        }
    }
    impl ::core::convert::From<DeployTargetCall> for HyperdriveFactoryCalls {
        fn from(value: DeployTargetCall) -> Self {
            Self::DeployTarget(value)
        }
    }
    impl ::core::convert::From<DeployerCoordinatorManagerCall>
    for HyperdriveFactoryCalls {
        fn from(value: DeployerCoordinatorManagerCall) -> Self {
            Self::DeployerCoordinatorManager(value)
        }
    }
    impl ::core::convert::From<FeeCollectorCall> for HyperdriveFactoryCalls {
        fn from(value: FeeCollectorCall) -> Self {
            Self::FeeCollector(value)
        }
    }
    impl ::core::convert::From<GetDeployerCoordinatorAtIndexCall>
    for HyperdriveFactoryCalls {
        fn from(value: GetDeployerCoordinatorAtIndexCall) -> Self {
            Self::GetDeployerCoordinatorAtIndex(value)
        }
    }
    impl ::core::convert::From<GetDeployerCoordinatorByInstancesCall>
    for HyperdriveFactoryCalls {
        fn from(value: GetDeployerCoordinatorByInstancesCall) -> Self {
            Self::GetDeployerCoordinatorByInstances(value)
        }
    }
    impl ::core::convert::From<GetDeployerCoordinatorsInRangeCall>
    for HyperdriveFactoryCalls {
        fn from(value: GetDeployerCoordinatorsInRangeCall) -> Self {
            Self::GetDeployerCoordinatorsInRange(value)
        }
    }
    impl ::core::convert::From<GetInstanceAtIndexCall> for HyperdriveFactoryCalls {
        fn from(value: GetInstanceAtIndexCall) -> Self {
            Self::GetInstanceAtIndex(value)
        }
    }
    impl ::core::convert::From<GetInstancesInRangeCall> for HyperdriveFactoryCalls {
        fn from(value: GetInstancesInRangeCall) -> Self {
            Self::GetInstancesInRange(value)
        }
    }
    impl ::core::convert::From<GetNumberOfDeployerCoordinatorsCall>
    for HyperdriveFactoryCalls {
        fn from(value: GetNumberOfDeployerCoordinatorsCall) -> Self {
            Self::GetNumberOfDeployerCoordinators(value)
        }
    }
    impl ::core::convert::From<GetNumberOfInstancesCall> for HyperdriveFactoryCalls {
        fn from(value: GetNumberOfInstancesCall) -> Self {
            Self::GetNumberOfInstances(value)
        }
    }
    impl ::core::convert::From<GovernanceCall> for HyperdriveFactoryCalls {
        fn from(value: GovernanceCall) -> Self {
            Self::Governance(value)
        }
    }
    impl ::core::convert::From<HyperdriveGovernanceCall> for HyperdriveFactoryCalls {
        fn from(value: HyperdriveGovernanceCall) -> Self {
            Self::HyperdriveGovernance(value)
        }
    }
    impl ::core::convert::From<IsDeployerCoordinatorCall> for HyperdriveFactoryCalls {
        fn from(value: IsDeployerCoordinatorCall) -> Self {
            Self::IsDeployerCoordinator(value)
        }
    }
    impl ::core::convert::From<IsInstanceCall> for HyperdriveFactoryCalls {
        fn from(value: IsInstanceCall) -> Self {
            Self::IsInstance(value)
        }
    }
    impl ::core::convert::From<KindCall> for HyperdriveFactoryCalls {
        fn from(value: KindCall) -> Self {
            Self::Kind(value)
        }
    }
    impl ::core::convert::From<LinkerCodeHashCall> for HyperdriveFactoryCalls {
        fn from(value: LinkerCodeHashCall) -> Self {
            Self::LinkerCodeHash(value)
        }
    }
    impl ::core::convert::From<LinkerFactoryCall> for HyperdriveFactoryCalls {
        fn from(value: LinkerFactoryCall) -> Self {
            Self::LinkerFactory(value)
        }
    }
    impl ::core::convert::From<MaxCheckpointDurationCall> for HyperdriveFactoryCalls {
        fn from(value: MaxCheckpointDurationCall) -> Self {
            Self::MaxCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<MaxCircuitBreakerDeltaCall> for HyperdriveFactoryCalls {
        fn from(value: MaxCircuitBreakerDeltaCall) -> Self {
            Self::MaxCircuitBreakerDelta(value)
        }
    }
    impl ::core::convert::From<MaxFeesCall> for HyperdriveFactoryCalls {
        fn from(value: MaxFeesCall) -> Self {
            Self::MaxFees(value)
        }
    }
    impl ::core::convert::From<MaxFixedAPRCall> for HyperdriveFactoryCalls {
        fn from(value: MaxFixedAPRCall) -> Self {
            Self::MaxFixedAPR(value)
        }
    }
    impl ::core::convert::From<MaxPositionDurationCall> for HyperdriveFactoryCalls {
        fn from(value: MaxPositionDurationCall) -> Self {
            Self::MaxPositionDuration(value)
        }
    }
    impl ::core::convert::From<MaxTimeStretchAPRCall> for HyperdriveFactoryCalls {
        fn from(value: MaxTimeStretchAPRCall) -> Self {
            Self::MaxTimeStretchAPR(value)
        }
    }
    impl ::core::convert::From<MinCheckpointDurationCall> for HyperdriveFactoryCalls {
        fn from(value: MinCheckpointDurationCall) -> Self {
            Self::MinCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<MinCircuitBreakerDeltaCall> for HyperdriveFactoryCalls {
        fn from(value: MinCircuitBreakerDeltaCall) -> Self {
            Self::MinCircuitBreakerDelta(value)
        }
    }
    impl ::core::convert::From<MinFeesCall> for HyperdriveFactoryCalls {
        fn from(value: MinFeesCall) -> Self {
            Self::MinFees(value)
        }
    }
    impl ::core::convert::From<MinFixedAPRCall> for HyperdriveFactoryCalls {
        fn from(value: MinFixedAPRCall) -> Self {
            Self::MinFixedAPR(value)
        }
    }
    impl ::core::convert::From<MinPositionDurationCall> for HyperdriveFactoryCalls {
        fn from(value: MinPositionDurationCall) -> Self {
            Self::MinPositionDuration(value)
        }
    }
    impl ::core::convert::From<MinTimeStretchAPRCall> for HyperdriveFactoryCalls {
        fn from(value: MinTimeStretchAPRCall) -> Self {
            Self::MinTimeStretchAPR(value)
        }
    }
    impl ::core::convert::From<NameCall> for HyperdriveFactoryCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<RemoveDeployerCoordinatorCall>
    for HyperdriveFactoryCalls {
        fn from(value: RemoveDeployerCoordinatorCall) -> Self {
            Self::RemoveDeployerCoordinator(value)
        }
    }
    impl ::core::convert::From<SweepCollectorCall> for HyperdriveFactoryCalls {
        fn from(value: SweepCollectorCall) -> Self {
            Self::SweepCollector(value)
        }
    }
    impl ::core::convert::From<UpdateCheckpointDurationResolutionCall>
    for HyperdriveFactoryCalls {
        fn from(value: UpdateCheckpointDurationResolutionCall) -> Self {
            Self::UpdateCheckpointDurationResolution(value)
        }
    }
    impl ::core::convert::From<UpdateCheckpointRewarderCall> for HyperdriveFactoryCalls {
        fn from(value: UpdateCheckpointRewarderCall) -> Self {
            Self::UpdateCheckpointRewarder(value)
        }
    }
    impl ::core::convert::From<UpdateDefaultPausersCall> for HyperdriveFactoryCalls {
        fn from(value: UpdateDefaultPausersCall) -> Self {
            Self::UpdateDefaultPausers(value)
        }
    }
    impl ::core::convert::From<UpdateDeployerCoordinatorManagerCall>
    for HyperdriveFactoryCalls {
        fn from(value: UpdateDeployerCoordinatorManagerCall) -> Self {
            Self::UpdateDeployerCoordinatorManager(value)
        }
    }
    impl ::core::convert::From<UpdateFeeCollectorCall> for HyperdriveFactoryCalls {
        fn from(value: UpdateFeeCollectorCall) -> Self {
            Self::UpdateFeeCollector(value)
        }
    }
    impl ::core::convert::From<UpdateGovernanceCall> for HyperdriveFactoryCalls {
        fn from(value: UpdateGovernanceCall) -> Self {
            Self::UpdateGovernance(value)
        }
    }
    impl ::core::convert::From<UpdateHyperdriveGovernanceCall>
    for HyperdriveFactoryCalls {
        fn from(value: UpdateHyperdriveGovernanceCall) -> Self {
            Self::UpdateHyperdriveGovernance(value)
        }
    }
    impl ::core::convert::From<UpdateLinkerCodeHashCall> for HyperdriveFactoryCalls {
        fn from(value: UpdateLinkerCodeHashCall) -> Self {
            Self::UpdateLinkerCodeHash(value)
        }
    }
    impl ::core::convert::From<UpdateLinkerFactoryCall> for HyperdriveFactoryCalls {
        fn from(value: UpdateLinkerFactoryCall) -> Self {
            Self::UpdateLinkerFactory(value)
        }
    }
    impl ::core::convert::From<UpdateMaxCheckpointDurationCall>
    for HyperdriveFactoryCalls {
        fn from(value: UpdateMaxCheckpointDurationCall) -> Self {
            Self::UpdateMaxCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<UpdateMaxCircuitBreakerDeltaCall>
    for HyperdriveFactoryCalls {
        fn from(value: UpdateMaxCircuitBreakerDeltaCall) -> Self {
            Self::UpdateMaxCircuitBreakerDelta(value)
        }
    }
    impl ::core::convert::From<UpdateMaxFeesCall> for HyperdriveFactoryCalls {
        fn from(value: UpdateMaxFeesCall) -> Self {
            Self::UpdateMaxFees(value)
        }
    }
    impl ::core::convert::From<UpdateMaxFixedAPRCall> for HyperdriveFactoryCalls {
        fn from(value: UpdateMaxFixedAPRCall) -> Self {
            Self::UpdateMaxFixedAPR(value)
        }
    }
    impl ::core::convert::From<UpdateMaxPositionDurationCall>
    for HyperdriveFactoryCalls {
        fn from(value: UpdateMaxPositionDurationCall) -> Self {
            Self::UpdateMaxPositionDuration(value)
        }
    }
    impl ::core::convert::From<UpdateMaxTimeStretchAPRCall> for HyperdriveFactoryCalls {
        fn from(value: UpdateMaxTimeStretchAPRCall) -> Self {
            Self::UpdateMaxTimeStretchAPR(value)
        }
    }
    impl ::core::convert::From<UpdateMinCheckpointDurationCall>
    for HyperdriveFactoryCalls {
        fn from(value: UpdateMinCheckpointDurationCall) -> Self {
            Self::UpdateMinCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<UpdateMinCircuitBreakerDeltaCall>
    for HyperdriveFactoryCalls {
        fn from(value: UpdateMinCircuitBreakerDeltaCall) -> Self {
            Self::UpdateMinCircuitBreakerDelta(value)
        }
    }
    impl ::core::convert::From<UpdateMinFeesCall> for HyperdriveFactoryCalls {
        fn from(value: UpdateMinFeesCall) -> Self {
            Self::UpdateMinFees(value)
        }
    }
    impl ::core::convert::From<UpdateMinFixedAPRCall> for HyperdriveFactoryCalls {
        fn from(value: UpdateMinFixedAPRCall) -> Self {
            Self::UpdateMinFixedAPR(value)
        }
    }
    impl ::core::convert::From<UpdateMinPositionDurationCall>
    for HyperdriveFactoryCalls {
        fn from(value: UpdateMinPositionDurationCall) -> Self {
            Self::UpdateMinPositionDuration(value)
        }
    }
    impl ::core::convert::From<UpdateMinTimeStretchAPRCall> for HyperdriveFactoryCalls {
        fn from(value: UpdateMinTimeStretchAPRCall) -> Self {
            Self::UpdateMinTimeStretchAPR(value)
        }
    }
    impl ::core::convert::From<UpdateSweepCollectorCall> for HyperdriveFactoryCalls {
        fn from(value: UpdateSweepCollectorCall) -> Self {
            Self::UpdateSweepCollector(value)
        }
    }
    impl ::core::convert::From<VersionCall> for HyperdriveFactoryCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `_instancesToDeployerCoordinators` function with signature `_instancesToDeployerCoordinators(address)` and selector `0x2b58f418`
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
    pub struct InstancesToDeployerCoordinatorsReturn {
        pub deploy_coordinator: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `checkpointDurationResolution` function with signature `checkpointDurationResolution()` and selector `0xd0f96b92`
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
    pub struct CheckpointDurationResolutionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `checkpointRewarder` function with signature `checkpointRewarder()` and selector `0xf2596458`
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
    pub struct CheckpointRewarderReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `defaultPausers` function with signature `defaultPausers()` and selector `0xa64c90bf`
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
    pub struct DefaultPausersReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `deployAndInitialize` function with signature `deployAndInitialize(bytes32,address,string,(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256)),bytes,uint256,uint256,uint256,(address,bool,bytes),bytes32)` and selector `0x2e7cd971`
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
    pub struct DeployAndInitializeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `deployTarget` function with signature `deployTarget(bytes32,address,(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,address,(uint256,uint256,uint256,uint256)),bytes,uint256,uint256,uint256,bytes32)` and selector `0x49f13de7`
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
    pub struct DeployTargetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `deployerCoordinatorManager` function with signature `deployerCoordinatorManager()` and selector `0xe4e7148f`
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
    pub struct DeployerCoordinatorManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `feeCollector` function with signature `feeCollector()` and selector `0xc415b95c`
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
    pub struct FeeCollectorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDeployerCoordinatorAtIndex` function with signature `getDeployerCoordinatorAtIndex(uint256)` and selector `0xfe3d5aeb`
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
    pub struct GetDeployerCoordinatorAtIndexReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDeployerCoordinatorByInstances` function with signature `getDeployerCoordinatorByInstances(address[])` and selector `0x1b59be0c`
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
    pub struct GetDeployerCoordinatorByInstancesReturn {
        pub coordinators: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `getDeployerCoordinatorsInRange` function with signature `getDeployerCoordinatorsInRange(uint256,uint256)` and selector `0xec895f11`
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
    pub struct GetDeployerCoordinatorsInRangeReturn {
        pub range: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `getInstanceAtIndex` function with signature `getInstanceAtIndex(uint256)` and selector `0xdaac24da`
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
    pub struct GetInstanceAtIndexReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getInstancesInRange` function with signature `getInstancesInRange(uint256,uint256)` and selector `0xbc30e7a1`
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
    pub struct GetInstancesInRangeReturn {
        pub range: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `getNumberOfDeployerCoordinators` function with signature `getNumberOfDeployerCoordinators()` and selector `0xe1b39c80`
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
    pub struct GetNumberOfDeployerCoordinatorsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getNumberOfInstances` function with signature `getNumberOfInstances()` and selector `0x6e95d67c`
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
    pub struct GetNumberOfInstancesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `governance` function with signature `governance()` and selector `0x5aa6e675`
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
    pub struct GovernanceReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `hyperdriveGovernance` function with signature `hyperdriveGovernance()` and selector `0xe3331555`
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
    pub struct HyperdriveGovernanceReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isDeployerCoordinator` function with signature `isDeployerCoordinator(address)` and selector `0xf8c09e59`
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
    pub struct IsDeployerCoordinatorReturn(pub bool);
    ///Container type for all return fields from the `isInstance` function with signature `isInstance(address)` and selector `0x6b44e6be`
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
    pub struct IsInstanceReturn(pub bool);
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
    ///Container type for all return fields from the `linkerCodeHash` function with signature `linkerCodeHash()` and selector `0xc905a4b5`
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
    pub struct LinkerCodeHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `linkerFactory` function with signature `linkerFactory()` and selector `0x99623bb1`
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
    pub struct LinkerFactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `maxCheckpointDuration` function with signature `maxCheckpointDuration()` and selector `0xe0e2daaa`
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
    pub struct MaxCheckpointDurationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxCircuitBreakerDelta` function with signature `maxCircuitBreakerDelta()` and selector `0x4554f9a9`
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
    pub struct MaxCircuitBreakerDeltaReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxFees` function with signature `maxFees()` and selector `0xe83e34b1`
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
    pub struct MaxFeesReturn(pub Fees);
    ///Container type for all return fields from the `maxFixedAPR` function with signature `maxFixedAPR()` and selector `0xbf9bd5cd`
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
    pub struct MaxFixedAPRReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxPositionDuration` function with signature `maxPositionDuration()` and selector `0x8efc0986`
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
    pub struct MaxPositionDurationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxTimeStretchAPR` function with signature `maxTimeStretchAPR()` and selector `0x48800760`
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
    pub struct MaxTimeStretchAPRReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minCheckpointDuration` function with signature `minCheckpointDuration()` and selector `0x5720c9d5`
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
    pub struct MinCheckpointDurationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minCircuitBreakerDelta` function with signature `minCircuitBreakerDelta()` and selector `0x1ecda0fe`
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
    pub struct MinCircuitBreakerDeltaReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minFees` function with signature `minFees()` and selector `0xc1722563`
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
    pub struct MinFeesReturn(pub Fees);
    ///Container type for all return fields from the `minFixedAPR` function with signature `minFixedAPR()` and selector `0xd23d7ea3`
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
    pub struct MinFixedAPRReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minPositionDuration` function with signature `minPositionDuration()` and selector `0xdaf012e6`
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
    pub struct MinPositionDurationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minTimeStretchAPR` function with signature `minTimeStretchAPR()` and selector `0xd6f50169`
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
    pub struct MinTimeStretchAPRReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `sweepCollector` function with signature `sweepCollector()` and selector `0x10780f73`
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
    pub struct SweepCollectorReturn(pub ::ethers::core::types::Address);
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
    ///`FactoryConfig(address,address,address,address[],address,address,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,(uint256,uint256,uint256,uint256),(uint256,uint256,uint256,uint256),address,bytes32)`
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
    pub struct FactoryConfig {
        pub governance: ::ethers::core::types::Address,
        pub deployer_coordinator_manager: ::ethers::core::types::Address,
        pub hyperdrive_governance: ::ethers::core::types::Address,
        pub default_pausers: ::std::vec::Vec<::ethers::core::types::Address>,
        pub fee_collector: ::ethers::core::types::Address,
        pub sweep_collector: ::ethers::core::types::Address,
        pub checkpoint_rewarder: ::ethers::core::types::Address,
        pub checkpoint_duration_resolution: ::ethers::core::types::U256,
        pub min_checkpoint_duration: ::ethers::core::types::U256,
        pub max_checkpoint_duration: ::ethers::core::types::U256,
        pub min_position_duration: ::ethers::core::types::U256,
        pub max_position_duration: ::ethers::core::types::U256,
        pub min_circuit_breaker_delta: ::ethers::core::types::U256,
        pub max_circuit_breaker_delta: ::ethers::core::types::U256,
        pub min_fixed_apr: ::ethers::core::types::U256,
        pub max_fixed_apr: ::ethers::core::types::U256,
        pub min_time_stretch_apr: ::ethers::core::types::U256,
        pub max_time_stretch_apr: ::ethers::core::types::U256,
        pub min_fees: Fees,
        pub max_fees: Fees,
        pub linker_factory: ::ethers::core::types::Address,
        pub linker_code_hash: [u8; 32],
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
