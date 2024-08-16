pub use i_hyperdrive_factory::*;
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
pub mod i_hyperdrive_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_deployerCoordinator",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("_instance"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IHYPERDRIVEFACTORY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IHyperdriveFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IHyperdriveFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IHyperdriveFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IHyperdriveFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IHyperdriveFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IHyperdriveFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IHyperdriveFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IHYPERDRIVEFACTORY_ABI.clone(),
                    client,
                ),
            )
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
            deployer_coordinator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 192, 158, 89], deployer_coordinator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isInstance` (0x6b44e6be) function
        pub fn is_instance(
            &self,
            instance: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([107, 68, 230, 190], instance)
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
            IHyperdriveFactoryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IHyperdriveFactory<M> {
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
    pub enum IHyperdriveFactoryErrors {
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
        ReceiveLocked(ReceiveLocked),
        TransferFailed(TransferFailed),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IHyperdriveFactoryErrors {
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IHyperdriveFactoryErrors {
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
                Self::ReceiveLocked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IHyperdriveFactoryErrors {
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
                    == <ReceiveLocked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IHyperdriveFactoryErrors {
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
                Self::ReceiveLocked(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IHyperdriveFactoryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DeployerCoordinatorAlreadyAdded>
    for IHyperdriveFactoryErrors {
        fn from(value: DeployerCoordinatorAlreadyAdded) -> Self {
            Self::DeployerCoordinatorAlreadyAdded(value)
        }
    }
    impl ::core::convert::From<DeployerCoordinatorIndexMismatch>
    for IHyperdriveFactoryErrors {
        fn from(value: DeployerCoordinatorIndexMismatch) -> Self {
            Self::DeployerCoordinatorIndexMismatch(value)
        }
    }
    impl ::core::convert::From<DeployerCoordinatorNotAdded>
    for IHyperdriveFactoryErrors {
        fn from(value: DeployerCoordinatorNotAdded) -> Self {
            Self::DeployerCoordinatorNotAdded(value)
        }
    }
    impl ::core::convert::From<EndIndexTooLarge> for IHyperdriveFactoryErrors {
        fn from(value: EndIndexTooLarge) -> Self {
            Self::EndIndexTooLarge(value)
        }
    }
    impl ::core::convert::From<InvalidCheckpointDuration> for IHyperdriveFactoryErrors {
        fn from(value: InvalidCheckpointDuration) -> Self {
            Self::InvalidCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<InvalidCheckpointDurationResolution>
    for IHyperdriveFactoryErrors {
        fn from(value: InvalidCheckpointDurationResolution) -> Self {
            Self::InvalidCheckpointDurationResolution(value)
        }
    }
    impl ::core::convert::From<InvalidCircuitBreakerDelta> for IHyperdriveFactoryErrors {
        fn from(value: InvalidCircuitBreakerDelta) -> Self {
            Self::InvalidCircuitBreakerDelta(value)
        }
    }
    impl ::core::convert::From<InvalidDeployConfig> for IHyperdriveFactoryErrors {
        fn from(value: InvalidDeployConfig) -> Self {
            Self::InvalidDeployConfig(value)
        }
    }
    impl ::core::convert::From<InvalidDeployerCoordinator> for IHyperdriveFactoryErrors {
        fn from(value: InvalidDeployerCoordinator) -> Self {
            Self::InvalidDeployerCoordinator(value)
        }
    }
    impl ::core::convert::From<InvalidFees> for IHyperdriveFactoryErrors {
        fn from(value: InvalidFees) -> Self {
            Self::InvalidFees(value)
        }
    }
    impl ::core::convert::From<InvalidFixedAPR> for IHyperdriveFactoryErrors {
        fn from(value: InvalidFixedAPR) -> Self {
            Self::InvalidFixedAPR(value)
        }
    }
    impl ::core::convert::From<InvalidIndexes> for IHyperdriveFactoryErrors {
        fn from(value: InvalidIndexes) -> Self {
            Self::InvalidIndexes(value)
        }
    }
    impl ::core::convert::From<InvalidMaxCheckpointDuration>
    for IHyperdriveFactoryErrors {
        fn from(value: InvalidMaxCheckpointDuration) -> Self {
            Self::InvalidMaxCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<InvalidMaxCircuitBreakerDelta>
    for IHyperdriveFactoryErrors {
        fn from(value: InvalidMaxCircuitBreakerDelta) -> Self {
            Self::InvalidMaxCircuitBreakerDelta(value)
        }
    }
    impl ::core::convert::From<InvalidMaxFees> for IHyperdriveFactoryErrors {
        fn from(value: InvalidMaxFees) -> Self {
            Self::InvalidMaxFees(value)
        }
    }
    impl ::core::convert::From<InvalidMaxFixedAPR> for IHyperdriveFactoryErrors {
        fn from(value: InvalidMaxFixedAPR) -> Self {
            Self::InvalidMaxFixedAPR(value)
        }
    }
    impl ::core::convert::From<InvalidMaxPositionDuration> for IHyperdriveFactoryErrors {
        fn from(value: InvalidMaxPositionDuration) -> Self {
            Self::InvalidMaxPositionDuration(value)
        }
    }
    impl ::core::convert::From<InvalidMaxTimeStretchAPR> for IHyperdriveFactoryErrors {
        fn from(value: InvalidMaxTimeStretchAPR) -> Self {
            Self::InvalidMaxTimeStretchAPR(value)
        }
    }
    impl ::core::convert::From<InvalidMinCheckpointDuration>
    for IHyperdriveFactoryErrors {
        fn from(value: InvalidMinCheckpointDuration) -> Self {
            Self::InvalidMinCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<InvalidMinCircuitBreakerDelta>
    for IHyperdriveFactoryErrors {
        fn from(value: InvalidMinCircuitBreakerDelta) -> Self {
            Self::InvalidMinCircuitBreakerDelta(value)
        }
    }
    impl ::core::convert::From<InvalidMinFees> for IHyperdriveFactoryErrors {
        fn from(value: InvalidMinFees) -> Self {
            Self::InvalidMinFees(value)
        }
    }
    impl ::core::convert::From<InvalidMinFixedAPR> for IHyperdriveFactoryErrors {
        fn from(value: InvalidMinFixedAPR) -> Self {
            Self::InvalidMinFixedAPR(value)
        }
    }
    impl ::core::convert::From<InvalidMinPositionDuration> for IHyperdriveFactoryErrors {
        fn from(value: InvalidMinPositionDuration) -> Self {
            Self::InvalidMinPositionDuration(value)
        }
    }
    impl ::core::convert::From<InvalidMinTimeStretchAPR> for IHyperdriveFactoryErrors {
        fn from(value: InvalidMinTimeStretchAPR) -> Self {
            Self::InvalidMinTimeStretchAPR(value)
        }
    }
    impl ::core::convert::From<InvalidPositionDuration> for IHyperdriveFactoryErrors {
        fn from(value: InvalidPositionDuration) -> Self {
            Self::InvalidPositionDuration(value)
        }
    }
    impl ::core::convert::From<InvalidTimeStretchAPR> for IHyperdriveFactoryErrors {
        fn from(value: InvalidTimeStretchAPR) -> Self {
            Self::InvalidTimeStretchAPR(value)
        }
    }
    impl ::core::convert::From<ReceiveLocked> for IHyperdriveFactoryErrors {
        fn from(value: ReceiveLocked) -> Self {
            Self::ReceiveLocked(value)
        }
    }
    impl ::core::convert::From<TransferFailed> for IHyperdriveFactoryErrors {
        fn from(value: TransferFailed) -> Self {
            Self::TransferFailed(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for IHyperdriveFactoryErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
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
    pub enum IHyperdriveFactoryEvents {
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
    impl ::ethers::contract::EthLogDecode for IHyperdriveFactoryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CheckpointDurationResolutionUpdatedFilter::decode_log(
                log,
            ) {
                return Ok(
                    IHyperdriveFactoryEvents::CheckpointDurationResolutionUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = CheckpointRewarderUpdatedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::CheckpointRewarderUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = DefaultPausersUpdatedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::DefaultPausersUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = DeployedFilter::decode_log(log) {
                return Ok(IHyperdriveFactoryEvents::DeployedFilter(decoded));
            }
            if let Ok(decoded) = DeployerCoordinatorAddedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::DeployerCoordinatorAddedFilter(decoded),
                );
            }
            if let Ok(decoded) = DeployerCoordinatorManagerUpdatedFilter::decode_log(
                log,
            ) {
                return Ok(
                    IHyperdriveFactoryEvents::DeployerCoordinatorManagerUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = DeployerCoordinatorRemovedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::DeployerCoordinatorRemovedFilter(decoded),
                );
            }
            if let Ok(decoded) = FeeCollectorUpdatedFilter::decode_log(log) {
                return Ok(IHyperdriveFactoryEvents::FeeCollectorUpdatedFilter(decoded));
            }
            if let Ok(decoded) = GovernanceUpdatedFilter::decode_log(log) {
                return Ok(IHyperdriveFactoryEvents::GovernanceUpdatedFilter(decoded));
            }
            if let Ok(decoded) = HyperdriveGovernanceUpdatedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::HyperdriveGovernanceUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = LinkerCodeHashUpdatedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::LinkerCodeHashUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = LinkerFactoryUpdatedFilter::decode_log(log) {
                return Ok(IHyperdriveFactoryEvents::LinkerFactoryUpdatedFilter(decoded));
            }
            if let Ok(decoded) = MaxCheckpointDurationUpdatedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::MaxCheckpointDurationUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MaxCircuitBreakerDeltaUpdatedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::MaxCircuitBreakerDeltaUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = MaxFeesUpdatedFilter::decode_log(log) {
                return Ok(IHyperdriveFactoryEvents::MaxFeesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = MaxFixedAPRUpdatedFilter::decode_log(log) {
                return Ok(IHyperdriveFactoryEvents::MaxFixedAPRUpdatedFilter(decoded));
            }
            if let Ok(decoded) = MaxPositionDurationUpdatedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::MaxPositionDurationUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MaxTimeStretchAPRUpdatedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::MaxTimeStretchAPRUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MinCheckpointDurationUpdatedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::MinCheckpointDurationUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MinCircuitBreakerDeltaUpdatedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::MinCircuitBreakerDeltaUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = MinFeesUpdatedFilter::decode_log(log) {
                return Ok(IHyperdriveFactoryEvents::MinFeesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = MinFixedAPRUpdatedFilter::decode_log(log) {
                return Ok(IHyperdriveFactoryEvents::MinFixedAPRUpdatedFilter(decoded));
            }
            if let Ok(decoded) = MinPositionDurationUpdatedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::MinPositionDurationUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MinTimeStretchAPRUpdatedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::MinTimeStretchAPRUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = SweepCollectorUpdatedFilter::decode_log(log) {
                return Ok(
                    IHyperdriveFactoryEvents::SweepCollectorUpdatedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IHyperdriveFactoryEvents {
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
    for IHyperdriveFactoryEvents {
        fn from(value: CheckpointDurationResolutionUpdatedFilter) -> Self {
            Self::CheckpointDurationResolutionUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<CheckpointRewarderUpdatedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: CheckpointRewarderUpdatedFilter) -> Self {
            Self::CheckpointRewarderUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DefaultPausersUpdatedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: DefaultPausersUpdatedFilter) -> Self {
            Self::DefaultPausersUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DeployedFilter> for IHyperdriveFactoryEvents {
        fn from(value: DeployedFilter) -> Self {
            Self::DeployedFilter(value)
        }
    }
    impl ::core::convert::From<DeployerCoordinatorAddedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: DeployerCoordinatorAddedFilter) -> Self {
            Self::DeployerCoordinatorAddedFilter(value)
        }
    }
    impl ::core::convert::From<DeployerCoordinatorManagerUpdatedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: DeployerCoordinatorManagerUpdatedFilter) -> Self {
            Self::DeployerCoordinatorManagerUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DeployerCoordinatorRemovedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: DeployerCoordinatorRemovedFilter) -> Self {
            Self::DeployerCoordinatorRemovedFilter(value)
        }
    }
    impl ::core::convert::From<FeeCollectorUpdatedFilter> for IHyperdriveFactoryEvents {
        fn from(value: FeeCollectorUpdatedFilter) -> Self {
            Self::FeeCollectorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<GovernanceUpdatedFilter> for IHyperdriveFactoryEvents {
        fn from(value: GovernanceUpdatedFilter) -> Self {
            Self::GovernanceUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<HyperdriveGovernanceUpdatedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: HyperdriveGovernanceUpdatedFilter) -> Self {
            Self::HyperdriveGovernanceUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<LinkerCodeHashUpdatedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: LinkerCodeHashUpdatedFilter) -> Self {
            Self::LinkerCodeHashUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<LinkerFactoryUpdatedFilter> for IHyperdriveFactoryEvents {
        fn from(value: LinkerFactoryUpdatedFilter) -> Self {
            Self::LinkerFactoryUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MaxCheckpointDurationUpdatedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: MaxCheckpointDurationUpdatedFilter) -> Self {
            Self::MaxCheckpointDurationUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MaxCircuitBreakerDeltaUpdatedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: MaxCircuitBreakerDeltaUpdatedFilter) -> Self {
            Self::MaxCircuitBreakerDeltaUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MaxFeesUpdatedFilter> for IHyperdriveFactoryEvents {
        fn from(value: MaxFeesUpdatedFilter) -> Self {
            Self::MaxFeesUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MaxFixedAPRUpdatedFilter> for IHyperdriveFactoryEvents {
        fn from(value: MaxFixedAPRUpdatedFilter) -> Self {
            Self::MaxFixedAPRUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MaxPositionDurationUpdatedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: MaxPositionDurationUpdatedFilter) -> Self {
            Self::MaxPositionDurationUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MaxTimeStretchAPRUpdatedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: MaxTimeStretchAPRUpdatedFilter) -> Self {
            Self::MaxTimeStretchAPRUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MinCheckpointDurationUpdatedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: MinCheckpointDurationUpdatedFilter) -> Self {
            Self::MinCheckpointDurationUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MinCircuitBreakerDeltaUpdatedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: MinCircuitBreakerDeltaUpdatedFilter) -> Self {
            Self::MinCircuitBreakerDeltaUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MinFeesUpdatedFilter> for IHyperdriveFactoryEvents {
        fn from(value: MinFeesUpdatedFilter) -> Self {
            Self::MinFeesUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MinFixedAPRUpdatedFilter> for IHyperdriveFactoryEvents {
        fn from(value: MinFixedAPRUpdatedFilter) -> Self {
            Self::MinFixedAPRUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MinPositionDurationUpdatedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: MinPositionDurationUpdatedFilter) -> Self {
            Self::MinPositionDurationUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MinTimeStretchAPRUpdatedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: MinTimeStretchAPRUpdatedFilter) -> Self {
            Self::MinTimeStretchAPRUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<SweepCollectorUpdatedFilter>
    for IHyperdriveFactoryEvents {
        fn from(value: SweepCollectorUpdatedFilter) -> Self {
            Self::SweepCollectorUpdatedFilter(value)
        }
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
    pub struct IsDeployerCoordinatorCall {
        pub deployer_coordinator: ::ethers::core::types::Address,
    }
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
    pub struct IsInstanceCall {
        pub instance: ::ethers::core::types::Address,
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
    pub enum IHyperdriveFactoryCalls {
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
        MaxFees(MaxFeesCall),
        MaxFixedAPR(MaxFixedAPRCall),
        MaxPositionDuration(MaxPositionDurationCall),
        MaxTimeStretchAPR(MaxTimeStretchAPRCall),
        MinCheckpointDuration(MinCheckpointDurationCall),
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
        UpdateFeeCollector(UpdateFeeCollectorCall),
        UpdateGovernance(UpdateGovernanceCall),
        UpdateHyperdriveGovernance(UpdateHyperdriveGovernanceCall),
        UpdateLinkerCodeHash(UpdateLinkerCodeHashCall),
        UpdateLinkerFactory(UpdateLinkerFactoryCall),
        UpdateMaxCheckpointDuration(UpdateMaxCheckpointDurationCall),
        UpdateMaxFees(UpdateMaxFeesCall),
        UpdateMaxFixedAPR(UpdateMaxFixedAPRCall),
        UpdateMaxPositionDuration(UpdateMaxPositionDurationCall),
        UpdateMaxTimeStretchAPR(UpdateMaxTimeStretchAPRCall),
        UpdateMinCheckpointDuration(UpdateMinCheckpointDurationCall),
        UpdateMinFees(UpdateMinFeesCall),
        UpdateMinFixedAPR(UpdateMinFixedAPRCall),
        UpdateMinPositionDuration(UpdateMinPositionDurationCall),
        UpdateMinTimeStretchAPR(UpdateMinTimeStretchAPRCall),
        UpdateSweepCollector(UpdateSweepCollectorCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for IHyperdriveFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
    impl ::ethers::core::abi::AbiEncode for IHyperdriveFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
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
    impl ::core::fmt::Display for IHyperdriveFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
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
                Self::MaxFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxFixedAPR(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxPositionDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxTimeStretchAPR(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinCheckpointDuration(element) => {
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
    impl ::core::convert::From<AddDeployerCoordinatorCall> for IHyperdriveFactoryCalls {
        fn from(value: AddDeployerCoordinatorCall) -> Self {
            Self::AddDeployerCoordinator(value)
        }
    }
    impl ::core::convert::From<CheckpointDurationResolutionCall>
    for IHyperdriveFactoryCalls {
        fn from(value: CheckpointDurationResolutionCall) -> Self {
            Self::CheckpointDurationResolution(value)
        }
    }
    impl ::core::convert::From<CheckpointRewarderCall> for IHyperdriveFactoryCalls {
        fn from(value: CheckpointRewarderCall) -> Self {
            Self::CheckpointRewarder(value)
        }
    }
    impl ::core::convert::From<DefaultPausersCall> for IHyperdriveFactoryCalls {
        fn from(value: DefaultPausersCall) -> Self {
            Self::DefaultPausers(value)
        }
    }
    impl ::core::convert::From<DeployAndInitializeCall> for IHyperdriveFactoryCalls {
        fn from(value: DeployAndInitializeCall) -> Self {
            Self::DeployAndInitialize(value)
        }
    }
    impl ::core::convert::From<DeployTargetCall> for IHyperdriveFactoryCalls {
        fn from(value: DeployTargetCall) -> Self {
            Self::DeployTarget(value)
        }
    }
    impl ::core::convert::From<DeployerCoordinatorManagerCall>
    for IHyperdriveFactoryCalls {
        fn from(value: DeployerCoordinatorManagerCall) -> Self {
            Self::DeployerCoordinatorManager(value)
        }
    }
    impl ::core::convert::From<FeeCollectorCall> for IHyperdriveFactoryCalls {
        fn from(value: FeeCollectorCall) -> Self {
            Self::FeeCollector(value)
        }
    }
    impl ::core::convert::From<GetDeployerCoordinatorAtIndexCall>
    for IHyperdriveFactoryCalls {
        fn from(value: GetDeployerCoordinatorAtIndexCall) -> Self {
            Self::GetDeployerCoordinatorAtIndex(value)
        }
    }
    impl ::core::convert::From<GetDeployerCoordinatorByInstancesCall>
    for IHyperdriveFactoryCalls {
        fn from(value: GetDeployerCoordinatorByInstancesCall) -> Self {
            Self::GetDeployerCoordinatorByInstances(value)
        }
    }
    impl ::core::convert::From<GetDeployerCoordinatorsInRangeCall>
    for IHyperdriveFactoryCalls {
        fn from(value: GetDeployerCoordinatorsInRangeCall) -> Self {
            Self::GetDeployerCoordinatorsInRange(value)
        }
    }
    impl ::core::convert::From<GetInstanceAtIndexCall> for IHyperdriveFactoryCalls {
        fn from(value: GetInstanceAtIndexCall) -> Self {
            Self::GetInstanceAtIndex(value)
        }
    }
    impl ::core::convert::From<GetInstancesInRangeCall> for IHyperdriveFactoryCalls {
        fn from(value: GetInstancesInRangeCall) -> Self {
            Self::GetInstancesInRange(value)
        }
    }
    impl ::core::convert::From<GetNumberOfDeployerCoordinatorsCall>
    for IHyperdriveFactoryCalls {
        fn from(value: GetNumberOfDeployerCoordinatorsCall) -> Self {
            Self::GetNumberOfDeployerCoordinators(value)
        }
    }
    impl ::core::convert::From<GetNumberOfInstancesCall> for IHyperdriveFactoryCalls {
        fn from(value: GetNumberOfInstancesCall) -> Self {
            Self::GetNumberOfInstances(value)
        }
    }
    impl ::core::convert::From<GovernanceCall> for IHyperdriveFactoryCalls {
        fn from(value: GovernanceCall) -> Self {
            Self::Governance(value)
        }
    }
    impl ::core::convert::From<HyperdriveGovernanceCall> for IHyperdriveFactoryCalls {
        fn from(value: HyperdriveGovernanceCall) -> Self {
            Self::HyperdriveGovernance(value)
        }
    }
    impl ::core::convert::From<IsDeployerCoordinatorCall> for IHyperdriveFactoryCalls {
        fn from(value: IsDeployerCoordinatorCall) -> Self {
            Self::IsDeployerCoordinator(value)
        }
    }
    impl ::core::convert::From<IsInstanceCall> for IHyperdriveFactoryCalls {
        fn from(value: IsInstanceCall) -> Self {
            Self::IsInstance(value)
        }
    }
    impl ::core::convert::From<KindCall> for IHyperdriveFactoryCalls {
        fn from(value: KindCall) -> Self {
            Self::Kind(value)
        }
    }
    impl ::core::convert::From<LinkerCodeHashCall> for IHyperdriveFactoryCalls {
        fn from(value: LinkerCodeHashCall) -> Self {
            Self::LinkerCodeHash(value)
        }
    }
    impl ::core::convert::From<LinkerFactoryCall> for IHyperdriveFactoryCalls {
        fn from(value: LinkerFactoryCall) -> Self {
            Self::LinkerFactory(value)
        }
    }
    impl ::core::convert::From<MaxCheckpointDurationCall> for IHyperdriveFactoryCalls {
        fn from(value: MaxCheckpointDurationCall) -> Self {
            Self::MaxCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<MaxFeesCall> for IHyperdriveFactoryCalls {
        fn from(value: MaxFeesCall) -> Self {
            Self::MaxFees(value)
        }
    }
    impl ::core::convert::From<MaxFixedAPRCall> for IHyperdriveFactoryCalls {
        fn from(value: MaxFixedAPRCall) -> Self {
            Self::MaxFixedAPR(value)
        }
    }
    impl ::core::convert::From<MaxPositionDurationCall> for IHyperdriveFactoryCalls {
        fn from(value: MaxPositionDurationCall) -> Self {
            Self::MaxPositionDuration(value)
        }
    }
    impl ::core::convert::From<MaxTimeStretchAPRCall> for IHyperdriveFactoryCalls {
        fn from(value: MaxTimeStretchAPRCall) -> Self {
            Self::MaxTimeStretchAPR(value)
        }
    }
    impl ::core::convert::From<MinCheckpointDurationCall> for IHyperdriveFactoryCalls {
        fn from(value: MinCheckpointDurationCall) -> Self {
            Self::MinCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<MinFeesCall> for IHyperdriveFactoryCalls {
        fn from(value: MinFeesCall) -> Self {
            Self::MinFees(value)
        }
    }
    impl ::core::convert::From<MinFixedAPRCall> for IHyperdriveFactoryCalls {
        fn from(value: MinFixedAPRCall) -> Self {
            Self::MinFixedAPR(value)
        }
    }
    impl ::core::convert::From<MinPositionDurationCall> for IHyperdriveFactoryCalls {
        fn from(value: MinPositionDurationCall) -> Self {
            Self::MinPositionDuration(value)
        }
    }
    impl ::core::convert::From<MinTimeStretchAPRCall> for IHyperdriveFactoryCalls {
        fn from(value: MinTimeStretchAPRCall) -> Self {
            Self::MinTimeStretchAPR(value)
        }
    }
    impl ::core::convert::From<NameCall> for IHyperdriveFactoryCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<RemoveDeployerCoordinatorCall>
    for IHyperdriveFactoryCalls {
        fn from(value: RemoveDeployerCoordinatorCall) -> Self {
            Self::RemoveDeployerCoordinator(value)
        }
    }
    impl ::core::convert::From<SweepCollectorCall> for IHyperdriveFactoryCalls {
        fn from(value: SweepCollectorCall) -> Self {
            Self::SweepCollector(value)
        }
    }
    impl ::core::convert::From<UpdateCheckpointDurationResolutionCall>
    for IHyperdriveFactoryCalls {
        fn from(value: UpdateCheckpointDurationResolutionCall) -> Self {
            Self::UpdateCheckpointDurationResolution(value)
        }
    }
    impl ::core::convert::From<UpdateCheckpointRewarderCall>
    for IHyperdriveFactoryCalls {
        fn from(value: UpdateCheckpointRewarderCall) -> Self {
            Self::UpdateCheckpointRewarder(value)
        }
    }
    impl ::core::convert::From<UpdateDefaultPausersCall> for IHyperdriveFactoryCalls {
        fn from(value: UpdateDefaultPausersCall) -> Self {
            Self::UpdateDefaultPausers(value)
        }
    }
    impl ::core::convert::From<UpdateFeeCollectorCall> for IHyperdriveFactoryCalls {
        fn from(value: UpdateFeeCollectorCall) -> Self {
            Self::UpdateFeeCollector(value)
        }
    }
    impl ::core::convert::From<UpdateGovernanceCall> for IHyperdriveFactoryCalls {
        fn from(value: UpdateGovernanceCall) -> Self {
            Self::UpdateGovernance(value)
        }
    }
    impl ::core::convert::From<UpdateHyperdriveGovernanceCall>
    for IHyperdriveFactoryCalls {
        fn from(value: UpdateHyperdriveGovernanceCall) -> Self {
            Self::UpdateHyperdriveGovernance(value)
        }
    }
    impl ::core::convert::From<UpdateLinkerCodeHashCall> for IHyperdriveFactoryCalls {
        fn from(value: UpdateLinkerCodeHashCall) -> Self {
            Self::UpdateLinkerCodeHash(value)
        }
    }
    impl ::core::convert::From<UpdateLinkerFactoryCall> for IHyperdriveFactoryCalls {
        fn from(value: UpdateLinkerFactoryCall) -> Self {
            Self::UpdateLinkerFactory(value)
        }
    }
    impl ::core::convert::From<UpdateMaxCheckpointDurationCall>
    for IHyperdriveFactoryCalls {
        fn from(value: UpdateMaxCheckpointDurationCall) -> Self {
            Self::UpdateMaxCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<UpdateMaxFeesCall> for IHyperdriveFactoryCalls {
        fn from(value: UpdateMaxFeesCall) -> Self {
            Self::UpdateMaxFees(value)
        }
    }
    impl ::core::convert::From<UpdateMaxFixedAPRCall> for IHyperdriveFactoryCalls {
        fn from(value: UpdateMaxFixedAPRCall) -> Self {
            Self::UpdateMaxFixedAPR(value)
        }
    }
    impl ::core::convert::From<UpdateMaxPositionDurationCall>
    for IHyperdriveFactoryCalls {
        fn from(value: UpdateMaxPositionDurationCall) -> Self {
            Self::UpdateMaxPositionDuration(value)
        }
    }
    impl ::core::convert::From<UpdateMaxTimeStretchAPRCall> for IHyperdriveFactoryCalls {
        fn from(value: UpdateMaxTimeStretchAPRCall) -> Self {
            Self::UpdateMaxTimeStretchAPR(value)
        }
    }
    impl ::core::convert::From<UpdateMinCheckpointDurationCall>
    for IHyperdriveFactoryCalls {
        fn from(value: UpdateMinCheckpointDurationCall) -> Self {
            Self::UpdateMinCheckpointDuration(value)
        }
    }
    impl ::core::convert::From<UpdateMinFeesCall> for IHyperdriveFactoryCalls {
        fn from(value: UpdateMinFeesCall) -> Self {
            Self::UpdateMinFees(value)
        }
    }
    impl ::core::convert::From<UpdateMinFixedAPRCall> for IHyperdriveFactoryCalls {
        fn from(value: UpdateMinFixedAPRCall) -> Self {
            Self::UpdateMinFixedAPR(value)
        }
    }
    impl ::core::convert::From<UpdateMinPositionDurationCall>
    for IHyperdriveFactoryCalls {
        fn from(value: UpdateMinPositionDurationCall) -> Self {
            Self::UpdateMinPositionDuration(value)
        }
    }
    impl ::core::convert::From<UpdateMinTimeStretchAPRCall> for IHyperdriveFactoryCalls {
        fn from(value: UpdateMinTimeStretchAPRCall) -> Self {
            Self::UpdateMinTimeStretchAPR(value)
        }
    }
    impl ::core::convert::From<UpdateSweepCollectorCall> for IHyperdriveFactoryCalls {
        fn from(value: UpdateSweepCollectorCall) -> Self {
            Self::UpdateSweepCollector(value)
        }
    }
    impl ::core::convert::From<VersionCall> for IHyperdriveFactoryCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
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
