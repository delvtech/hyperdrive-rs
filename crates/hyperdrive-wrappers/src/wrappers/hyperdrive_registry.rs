pub use hyperdrive_registry::*;
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
pub mod hyperdrive_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("admin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("getFactoriesInRange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getFactoriesInRange",
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
                                    name: ::std::borrow::ToOwned::to_owned("factories"),
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
                    ::std::borrow::ToOwned::to_owned("getFactoryAtIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFactoryAtIndex"),
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
                    ::std::borrow::ToOwned::to_owned("getFactoryInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFactoryInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_factory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("info"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IHyperdriveRegistry.FactoryInfo",
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
                    ::std::borrow::ToOwned::to_owned("getFactoryInfoWithMetadata"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getFactoryInfoWithMetadata",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_factory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("info"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IHyperdriveRegistry.FactoryInfoWithMetadata",
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
                    ::std::borrow::ToOwned::to_owned("getFactoryInfos"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFactoryInfos"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("__factories"),
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
                                    name: ::std::borrow::ToOwned::to_owned("info"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IHyperdriveRegistry.FactoryInfo[]",
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
                    ::std::borrow::ToOwned::to_owned("getFactoryInfosWithMetadata"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getFactoryInfosWithMetadata",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("__factories"),
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
                                    name: ::std::borrow::ToOwned::to_owned("info"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IHyperdriveRegistry.FactoryInfoWithMetadata[]",
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
                    ::std::borrow::ToOwned::to_owned("getInstanceInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInstanceInfo"),
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
                                    name: ::std::borrow::ToOwned::to_owned("info"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IHyperdriveRegistry.InstanceInfo",
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
                    ::std::borrow::ToOwned::to_owned("getInstanceInfoWithMetadata"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getInstanceInfoWithMetadata",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("info"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IHyperdriveRegistry.InstanceInfoWithMetadata",
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
                    ::std::borrow::ToOwned::to_owned("getInstanceInfos"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInstanceInfos"),
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
                                    name: ::std::borrow::ToOwned::to_owned("info"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IHyperdriveRegistry.InstanceInfo[]",
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
                    ::std::borrow::ToOwned::to_owned("getInstanceInfosWithMetadata"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getInstanceInfosWithMetadata",
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
                                    name: ::std::borrow::ToOwned::to_owned("info"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IHyperdriveRegistry.InstanceInfoWithMetadata[]",
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
                                    name: ::std::borrow::ToOwned::to_owned("instances"),
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
                    ::std::borrow::ToOwned::to_owned("getNumberOfFactories"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getNumberOfFactories",
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_admin"),
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
                    ::std::borrow::ToOwned::to_owned("isInitialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isInitialized"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("setFactoryInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFactoryInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("__factories"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128[]"),
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
                    ::std::borrow::ToOwned::to_owned("setInstanceInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setInstanceInfo"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("__factories"),
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
                    ::std::borrow::ToOwned::to_owned("updateAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_admin"),
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
                    ::std::borrow::ToOwned::to_owned("updateName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateName"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("AdminUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdminUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FactoryInfoUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FactoryInfoUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("factory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InstanceInfoUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InstanceInfoUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("instance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("factory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NameUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NameUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
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
                    ::std::borrow::ToOwned::to_owned("EndIndexTooLarge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EndIndexTooLarge"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InputLengthMismatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InputLengthMismatch",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidFactory"),
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
                    ::std::borrow::ToOwned::to_owned("RegistryAlreadyInitialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RegistryAlreadyInitialized",
                            ),
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
    pub static HYPERDRIVEREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa)C\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01XW`\x005`\xE0\x1C\x80c\x9BrJ\xD4\x11a\0\xC3W\x80c\xE2\xF2s\xBD\x11a\0|W\x80c\xE2\xF2s\xBD\x14a\x03\xF8W\x80c\xE9g\xE3\x88\x14a\x04\x0BW\x80c\xEA5\x03!\x14a\x04\x1EW\x80c\xF3,\x9E4\x14a\x04>W\x80c\xF5\x9D\0\xB9\x14a\x04^W\x80c\xF8Q\xA4@\x14a\x04fW`\0\x80\xFD[\x80c\x9BrJ\xD4\x14a\x03\x10W\x80c\xA5\x87\xBB\xE1\x14a\x03gW\x80c\xB7>?\xAB\x14a\x03\x92W\x80c\xBC0\xE7\xA1\x14a\x03\xB2W\x80c\xD2\xF7-R\x14a\x03\xC5W\x80c\xDA\xAC$\xDA\x14a\x03\xE5W`\0\x80\xFD[\x80cM\xB6\xC0\xE0\x11a\x01\x15W\x80cM\xB6\xC0\xE0\x14a\x02rW\x80cT\xFDMP\x14a\x02\x92W\x80cn\x95\xD6|\x14a\x02\xB8W\x80cqk\xA5\xF6\x14a\x02\xCAW\x80cz\xB43\x9D\x14a\x02\xEAW\x80c\x84\xDA\x92\xA7\x14a\x02\xFDW`\0\x80\xFD[\x80c\x04\xBA\xA0\x0B\x14a\x01]W\x80c\x06\xFD\xDE\x03\x14a\x01\xA4W\x80c\x18\xBB;T\x14a\x01\xACW\x80c\x1F\xF3\n\xD2\x14a\x02 W\x80c*\xD1\x9D\xE8\x14a\x025W\x80c9.S\xCD\x14a\x02UW[`\0\x80\xFD[a\x01\x8E`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qHyperdriveRegistry`p\x1B\x81RP\x81V[`@Qa\x01\x9B\x91\x90a\x1F\xA5V[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Ea\x04yV[a\x02\x13a\x01\xBA6`\x04a\x1F\xDBV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x81\x81R`\x06` \x81\x81R\x94\x82 \x80T`\x01`\x01`\x80\x1B\x03\x16\x85R\x92\x90\x91R\x83R`\x01\x01T\x90\x92\x16\x90\x82\x01R\x90V[`@Qa\x01\x9B\x91\x90a\x1F\xF6V[a\x023a\x02.6`\x04a aV[a\x05\x07V[\0[a\x02Ha\x02C6`\x04a\x1F\xDBV[a\x08\xD2V[`@Qa\x01\x9B\x91\x90a!gV[`\0Ta\x02b\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\x9BV[a\x02\x85a\x02\x806`\x04a!zV[a\naV[`@Qa\x01\x9B\x91\x90a!\xBBV[a\x01\x8E`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f\x07c\x12\xE3\x02\xE3#`\xCC\x1B\x81RP\x81V[`\x05T[`@Q\x90\x81R` \x01a\x01\x9BV[a\x02\xDDa\x02\xD86`\x04a\"\x1BV[a\x0B\xAAV[`@Qa\x01\x9B\x91\x90a\"=V[a\x023a\x02\xF86`\x04a\"\x8AV[a\x0C\xA9V[a\x023a\x03\x0B6`\x04a#wV[a\rLV[a\x03Xa\x03\x1E6`\x04a\x1F\xDBV[`@\x80Q` \x80\x82\x01\x83R`\0\x91\x82\x90R\x82Q\x80\x82\x01\x84R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x82R`\x04\x90R T`\x01`\x01`\x80\x1B\x03\x16\x81R\x90V[`@Q\x90Q\x81R` \x01a\x01\x9BV[a\x03za\x03u6`\x04a#\xF6V[a\r\xC4V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x9BV[a\x03\xA5a\x03\xA06`\x04a!zV[a\r\xF4V[`@Qa\x01\x9B\x91\x90a$aV[a\x02\xDDa\x03\xC06`\x04a\"\x1BV[a\x10\x83V[a\x03\xD8a\x03\xD36`\x04a!zV[a\x11\x82V[`@Qa\x01\x9B\x91\x90a$\xC5V[a\x03za\x03\xF36`\x04a#\xF6V[a\x14CV[a\x023a\x04\x066`\x04a\x1F\xDBV[a\x14XV[a\x023a\x04\x196`\x04a%\x1CV[a\x14\xCCV[a\x041a\x04,6`\x04a!zV[a\x18xV[`@Qa\x01\x9B\x91\x90a%\x87V[a\x04Qa\x04L6`\x04a\x1F\xDBV[a\x19bV[`@Qa\x01\x9B\x91\x90a%\xC0V[`\x03Ta\x02\xBCV[`\x02Ta\x03z\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01\x80Ta\x04\x86\x90a%\xD3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xB2\x90a%\xD3V[\x80\x15a\x04\xFFW\x80`\x1F\x10a\x04\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xFFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xE2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x051W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x83\x14\x15\x80a\x05@WP\x84\x81\x14\x15[\x15a\x05^W`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x85\x81\x10\x15a\x08\xC9W`\0`\x06`\0\x89\x89\x85\x81\x81\x10a\x05\x81Wa\x05\x81a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x05\x96\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\x80\x1B\x03\x16\x90P\x85\x85\x83\x81\x81\x10a\x05\xCDWa\x05\xCDa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x05\xE2\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15a\x05\xF7WP\x80\x15\x15[\x15a\x06\x80W`\0\x84\x84\x84\x81\x81\x10a\x06\x10Wa\x06\x10a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x06%\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06LW`@QczD\xDB\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06{\x88\x88\x84\x81\x81\x10a\x06aWa\x06aa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x06v\x91\x90a\x1F\xDBV[a\x19\xF7V[a\x08\x03V[\x85\x85\x83\x81\x81\x10a\x06\x92Wa\x06\x92a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x06\xA7\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x06\xBDWP\x80\x15\x15[\x15a\x07?Wa\x06{\x88\x88\x84\x81\x81\x10a\x06\xD7Wa\x06\xD7a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x06\xEC\x91\x90a\x1F\xDBV[\x87\x87\x85\x81\x81\x10a\x06\xFEWa\x06\xFEa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x07\x13\x91\x90a&#V[\x86\x86\x86\x81\x81\x10a\x07%Wa\x07%a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x07:\x91\x90a\x1F\xDBV[a\x1BMV[\x85\x85\x83\x81\x81\x10a\x07QWa\x07Qa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x07f\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x07{WP\x80\x15[\x15a\x07\xFDWa\x06{\x88\x88\x84\x81\x81\x10a\x07\x95Wa\x07\x95a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x07\xAA\x91\x90a\x1F\xDBV[\x87\x87\x85\x81\x81\x10a\x07\xBCWa\x07\xBCa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x07\xD1\x91\x90a&#V[\x86\x86\x86\x81\x81\x10a\x07\xE3Wa\x07\xE3a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x07\xF8\x91\x90a\x1F\xDBV[a\x1C\x94V[Pa\x08\xC1V[\x83\x83\x83\x81\x81\x10a\x08\x15Wa\x08\x15a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x08*\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x84\x81\x81\x10a\x08EWa\x08Ea&\rV[\x90P` \x02\x01` \x81\x01\x90a\x08Z\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x89\x89\x85\x81\x81\x10a\x08uWa\x08ua&\rV[\x90P` \x02\x01` \x81\x01\x90a\x08\x8A\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDC\xDD\xA2\xB8&{\x8F\xE0\xEB\xFE\xB2\xCC\x8F&h\x07\xB4\x12\xBE\xC0\x96\xD1l\xBB\xE5v\xD4m\x12%S\xE0`@Q`@Q\x80\x91\x03\x90\xA4P[`\x01\x01a\x05aV[PPPPPPPV[a\x08\xDAa\x1F\x1DV[`@\x80Q`\xA0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x06` \x81\x81R\x86\x83 \x80T`\x01`\x01`\x80\x1B\x03\x16\x87R\x84\x84R\x91\x81R`\x01\x90\x91\x01T\x90\x93\x16\x92\x84\x01\x92\x90\x92R\x83Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x84Q\x87\x95\x85\x01\x93c\x06\xFD\xDE\x03\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\tZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\x82\x91\x90\x81\x01\x90a&LV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x04\xBA\xA0\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\xED\x91\x90\x81\x01\x90a&LV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16cT\xFDMP`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\nX\x91\x90\x81\x01\x90a&LV[\x90R\x93\x92PPPV[``\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\n{Wa\n{a#\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xC0W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\n\x99W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B\xA2W`@Q\x80`@\x01`@R\x80`\x06`\0\x87\x87\x86\x81\x81\x10a\n\xEFWa\n\xEFa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x0B\x04\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 T`\x01`\x01`\x80\x1B\x03\x16\x83R\x91\x01\x90`\x06\x90\x87\x87\x86\x81\x81\x10a\x0BDWa\x0BDa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x0BY\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x92\x90\x92R`@\x01`\0 `\x01\x01T\x16\x90R\x82Q\x83\x90\x83\x90\x81\x10a\x0B\x8FWa\x0B\x8Fa&\rV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\n\xC6V[P[\x92\x91PPV[``\x81\x83\x10a\x0B\xCCW`@Qc;'5\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T\x82\x11\x15a\x0B\xEFW`@Qc\xE0\xF7\xBE\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xF9\x83\x83a&\xB9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\x10Wa\x0C\x10a#\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82[\x82\x81\x10\x15a\x0B\xA2W`\x03\x81\x81T\x81\x10a\x0CYWa\x0CYa&\rV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x82a\x0Cy\x86\x84a&\xB9V[\x81Q\x81\x10a\x0C\x89Wa\x0C\x89a&\rV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x0C>V[`\0T`\xFF\x16\x15a\x0C\xCDW`@Qcr,9[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91Ua\x0C\xE8\x83\x85\x83a'+V[P`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Qa\r\x17\x90\x85\x90\x85\x90a'\xEAV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xE5t9\xD87qU\x89\xA7Q\xE3\x91\xF7:\x95\x92\x9E\xCC\x96\xAEj\xA0\x9A\x16\xBE\x8A\xE3\x12\xD6\x14qW\x90`\0\x90\xA3PPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\rvW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01a\r\x82\x82\x82a'\xFAV[P\x80`@Qa\r\x91\x91\x90a(\xB9V[`@Q\x90\x81\x90\x03\x81 \x90\x7F\x9Fv\x88\xA9\x7F\x1A\xC5\x1F\xE0;\xAC\x18\xAF\x18\xD6\x81\x0F\x9F\x11\xF0\xDB\x08\xC5\x9B\x198\xA9\xAC\x82^\xF7D\x90`\0\x90\xA2PV[`\0`\x03\x82\x81T\x81\x10a\r\xD9Wa\r\xD9a&\rV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[``\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\x0EWa\x0E\x0Ea#\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0EjW\x81` \x01[a\x0EW`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0E,W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B\xA2W`\0\x84\x84\x83\x81\x81\x10a\x0E\x8CWa\x0E\x8Ca&\rV[\x90P` \x02\x01` \x81\x01\x90a\x0E\xA1\x91\x90a\x1F\xDBV[\x90P`@Q\x80`\x80\x01`@R\x80`\x04`\0\x88\x88\x87\x81\x81\x10a\x0E\xC4Wa\x0E\xC4a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x0E\xD9\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x06\xFD\xDE\x03`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F\x84\x91\x90\x81\x01\x90a&LV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x04\xBA\xA0\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F\xEF\x91\x90\x81\x01\x90a&LV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16cT\xFDMP`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x102W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x10Z\x91\x90\x81\x01\x90a&LV[\x81RP\x83\x83\x81Q\x81\x10a\x10oWa\x10oa&\rV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0EpV[``\x81\x83\x10a\x10\xA5W`@Qc;'5\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T\x82\x11\x15a\x10\xC8W`@Qc\xE0\xF7\xBE\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xD2\x83\x83a&\xB9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\xE9Wa\x10\xE9a#\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\x12W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82[\x82\x81\x10\x15a\x0B\xA2W`\x05\x81\x81T\x81\x10a\x112Wa\x112a&\rV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x82a\x11R\x86\x84a&\xB9V[\x81Q\x81\x10a\x11bWa\x11ba&\rV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x11\x17V[``\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x9CWa\x11\x9Ca#\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xD5W\x81` \x01[a\x11\xC2a\x1F\x1DV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x11\xBAW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B\xA2W`\0\x84\x84\x83\x81\x81\x10a\x11\xF7Wa\x11\xF7a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x12\x0C\x91\x90a\x1F\xDBV[\x90P`@Q\x80`\xA0\x01`@R\x80`\x06`\0\x88\x88\x87\x81\x81\x10a\x12/Wa\x12/a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x12D\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 T`\x01`\x01`\x80\x1B\x03\x16\x83R\x91\x01\x90`\x06\x90\x88\x88\x87\x81\x81\x10a\x12\x84Wa\x12\x84a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x12\x99\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x06\xFD\xDE\x03`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13D\x91\x90\x81\x01\x90a&LV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x04\xBA\xA0\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xAF\x91\x90\x81\x01\x90a&LV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16cT\xFDMP`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x14\x1A\x91\x90\x81\x01\x90a&LV[\x81RP\x83\x83\x81Q\x81\x10a\x14/Wa\x14/a&\rV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x11\xDBV[`\0`\x05\x82\x81T\x81\x10a\r\xD9Wa\r\xD9a&\rV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\x82W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7FT\xE4a'\x88\xF9\x03\x84\xE6\x842\x98\xD7\x85D6\xF3\xA5\x85\xB2\xC3\x83\x1A\xB6j\xBF\x1D\xE6;\xFAl-\x90`\0\x90\xA2PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\xF6W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x81\x14a\x15\x16W`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x83\x81\x10\x15a\x18qW`\0`\x04`\0\x87\x87\x85\x81\x81\x10a\x159Wa\x159a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x15N\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\x80\x1B\x03\x16\x90P\x83\x83\x83\x81\x81\x10a\x15\x85Wa\x15\x85a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x15\x9A\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15a\x15\xAFWP\x80\x15\x15[\x15a\x15\xE8Wa\x15\xE3\x86\x86\x84\x81\x81\x10a\x15\xC9Wa\x15\xC9a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x15\xDE\x91\x90a\x1F\xDBV[a\x1D\xD9V[a\x17\xDBV[\x83\x83\x83\x81\x81\x10a\x15\xFAWa\x15\xFAa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x16\x0F\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x16%WP\x80\x15\x15[\x15a\x16\xB4Wa\x15\xE3\x86\x86\x84\x81\x81\x10a\x16?Wa\x16?a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x16T\x91\x90a\x1F\xDBV[\x85\x85\x85\x81\x81\x10a\x16fWa\x16fa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x16{\x91\x90a&#V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[\x83\x83\x83\x81\x81\x10a\x16\xC6Wa\x16\xC6a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x16\xDB\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x16\xF0WP\x80\x15[\x15a\x17\xD5Wa\x15\xE3\x86\x86\x84\x81\x81\x10a\x17\nWa\x17\na&\rV[\x90P` \x02\x01` \x81\x01\x90a\x17\x1F\x91\x90a\x1F\xDBV[\x85\x85\x85\x81\x81\x10a\x171Wa\x171a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x17F\x91\x90a&#V[`\x03\x80T`\x01\x81\x01\x90\x91U\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x94\x16\x84\x17\x90U`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x81R\x91\x83\x16` \x83\x81\x01\x91\x82R`\0\x95\x86R`\x04\x90R\x93 \x90Q\x92Q\x82\x16`\x01`\x80\x1B\x02\x92\x90\x91\x16\x91\x90\x91\x17\x90UV[Pa\x18iV[\x83\x83\x83\x81\x81\x10a\x17\xEDWa\x17\xEDa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x18\x02\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x86\x86\x84\x81\x81\x10a\x18\x1DWa\x18\x1Da&\rV[\x90P` \x02\x01` \x81\x01\x90a\x182\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD8@\xEA\x8C\xB0B\xBC\x84\r>U\xA0F\x18\xAB&\x844\xD3\xD0\xA2\x18c\x83`\xA3\x0F\xDB\x80\xDEc\xF6`@Q`@Q\x80\x91\x03\x90\xA3P[`\x01\x01a\x15\x19V[PPPPPV[``\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x92Wa\x18\x92a#\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xD2W\x81` \x01[`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x18\xB0W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B\xA2W`@Q\x80` \x01`@R\x80`\x04`\0\x87\x87\x86\x81\x81\x10a\x19\x01Wa\x19\x01a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x19\x16\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\x80\x1B\x03\x16\x90R\x82Q\x83\x90\x83\x90\x81\x10a\x19OWa\x19Oa&\rV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x18\xD8V[a\x19\x8D`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81R`\x04` \x81\x81R\x85\x83 T`\x01`\x01`\x80\x1B\x03\x16\x85R\x85Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x95Q\x88\x96\x91\x86\x01\x94\x93c\x06\xFD\xDE\x03\x93\x83\x81\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\tZW=`\0\x80>=`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x06` R`@\x90 T`\x05T`\x01`\x80\x1B\x90\x91\x04`\x01`\x01`\x80\x1B\x03\x16\x90a\x1A0`\x01\x82a&\xB9V[\x82`\x01`\x01`\x80\x1B\x03\x16\x14a\x1A\xE8W`\0`\x05a\x1AN`\x01\x84a&\xB9V[\x81T\x81\x10a\x1A^Wa\x1A^a&\rV[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80\x83R`\x06\x90\x91R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x80\x87\x16`\x01`\x80\x1B\x81\x02\x91\x90\x92\x16\x17\x90\x91U`\x05\x80T\x92\x93P\x83\x92\x90\x91\x90\x81\x10a\x1A\xB8Wa\x1A\xB8a&\rV[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPP[`\x05\x80T\x80a\x1A\xF9Wa\x1A\xF9a(\xD5V[`\0\x82\x81R` \x80\x82 \x83\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U\x93\x01\x90\x93U`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x85R`\x06\x90\x91R`@\x84 \x93\x84U`\x01\x93\x90\x93\x01\x80T\x90\x93\x16\x90\x92UPPV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x06` R`@\x90 `\x01\x01T\x16\x80\x15\x80\x15\x90a\x1B\x8CWP\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x80a\x1C$WP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15a\x1B\xB1WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[\x80\x15a\x1C$WP`@Qc5\xA2s_`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x83\x16\x90ckD\xE6\xBE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\"\x91\x90a(\xEBV[\x15[\x15a\x1CBW`@QczD\xDB\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x90\x81R`\x06` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x82U`\x01\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x1D\x13WP`@Qc5\xA2s_`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x82\x16\x90ckD\xE6\xBE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xEDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x11\x91\x90a(\xEBV[\x15[\x15a\x1D1W`@QczD\xDB\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80T`\x01\x80\x82\x01\x90\x92U\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB0\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x81\x17\x90\x92U`@\x80Q``\x81\x01\x82R`\x01`\x01`\x80\x1B\x03\x97\x88\x16\x81R\x93\x87\x16` \x85\x81\x01\x91\x82R\x96\x89\x16\x85\x83\x01\x90\x81R`\0\x94\x85R`\x06\x90\x97R\x92 \x92Q\x91Q\x86\x16`\x01`\x80\x1B\x02\x91\x90\x95\x16\x17\x81U\x91Q\x91\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x04` R`@\x90 T`\x03T`\x01`\x80\x1B\x90\x91\x04`\x01`\x01`\x80\x1B\x03\x16\x90a\x1E\x12`\x01\x82a&\xB9V[\x82`\x01`\x01`\x80\x1B\x03\x16\x14a\x1E\xCAW`\0`\x03a\x1E0`\x01\x84a&\xB9V[\x81T\x81\x10a\x1E@Wa\x1E@a&\rV[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80\x83R`\x04\x90\x91R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x80\x87\x16`\x01`\x80\x1B\x81\x02\x91\x90\x92\x16\x17\x90\x91U`\x03\x80T\x92\x93P\x83\x92\x90\x91\x90\x81\x10a\x1E\x9AWa\x1E\x9Aa&\rV[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPP[`\x03\x80T\x80a\x1E\xDBWa\x1E\xDBa(\xD5V[`\0\x82\x81R` \x80\x82 \x83\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x90\x92\x01\x90\x92U`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x81R`\x04\x90\x93RPP`@\x81 UV[`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0[\x83\x81\x10\x15a\x1FpW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1FXV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1F\x91\x81` \x86\x01` \x86\x01a\x1FUV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1F\xB8` \x83\x01\x84a\x1FyV[\x93\x92PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1F\xD6W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1F\xEDW`\0\x80\xFD[a\x1F\xB8\x82a\x1F\xBFV[\x81Q\x81R` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`@\x81\x01a\x0B\xA4V[`\0\x80\x83`\x1F\x84\x01\x12a (W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a ?W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a ZW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a zW`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a \x91W`\0\x80\xFD[a \x9D\x8A\x83\x8B\x01a \x16V[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a \xB6W`\0\x80\xFD[a \xC2\x8A\x83\x8B\x01a \x16V[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15a \xDBW`\0\x80\xFD[Pa \xE8\x89\x82\x8A\x01a \x16V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[\x80Q\x82R`\x01\x80`\xA0\x1B\x03` \x82\x01Q\x16` \x83\x01R`\0`@\x82\x01Q`\xA0`@\x85\x01Ra!+`\xA0\x85\x01\x82a\x1FyV[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01Ra!D\x82\x82a\x1FyV[\x91PP`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra!^\x82\x82a\x1FyV[\x95\x94PPPPPV[` \x81R`\0a\x1F\xB8` \x83\x01\x84a \xFAV[`\0\x80` \x83\x85\x03\x12\x15a!\x8DW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xA3W`\0\x80\xFD[a!\xAF\x85\x82\x86\x01a \x16V[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\"\x0EWa!\xFE\x84\x83Q\x80Q\x82R` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a!\xD8V[P\x91\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\".W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\"~W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\"YV[P\x90\x96\x95PPPPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\"\x9FW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\"\xB6W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\"\xCAW`\0\x80\xFD[\x815\x81\x81\x11\x15a\"\xD9W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\"\xEBW`\0\x80\xFD[` \x92\x83\x01\x95P\x93Pa#\x01\x91\x86\x01\x90Pa\x1F\xBFV[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a#HWa#Ha#\nV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a#iWa#ia#\nV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15a#\x89W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a#\x9FW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a#\xB0W`\0\x80\xFD[\x805a#\xC3a#\xBE\x82a#PV[a# V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a#\xD8W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a$\x08W`\0\x80\xFD[P5\x91\x90PV[\x80Q\x82R`\0` \x82\x01Q`\x80` \x85\x01Ra$.`\x80\x85\x01\x82a\x1FyV[\x90P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra$G\x82\x82a\x1FyV[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra!^\x82\x82a\x1FyV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a$\xB8W`?\x19\x88\x86\x03\x01\x84Ra$\xA6\x85\x83Qa$\x0FV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a$\x8AV[P\x92\x97\x96PPPPPPPV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a$\xB8W`?\x19\x88\x86\x03\x01\x84Ra%\n\x85\x83Qa \xFAV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a$\xEEV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a%2W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a%IW`\0\x80\xFD[a%U\x88\x83\x89\x01a \x16V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a%nW`\0\x80\xFD[Pa%{\x87\x82\x88\x01a \x16V[\x95\x98\x94\x97P\x95PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\"~W\x83QQ\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a%\xA3V[` \x81R`\0a\x1F\xB8` \x83\x01\x84a$\x0FV[`\x01\x81\x81\x1C\x90\x82\x16\x80a%\xE7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a&\x07WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a&5W`\0\x80\xFD[\x815`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x1F\xB8W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a&^W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&tW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a&\x85W`\0\x80\xFD[\x80Qa&\x93a#\xBE\x82a#PV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a&\xA8W`\0\x80\xFD[a!^\x82` \x83\x01` \x86\x01a\x1FUV[\x81\x81\x03\x81\x81\x11\x15a\x0B\xA4WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a'&W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a'\x03WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a'\"W\x82\x81U`\x01\x01a'\x0FV[PPP[PPPV[`\x01`\x01`@\x1B\x03\x83\x11\x15a'BWa'Ba#\nV[a'V\x83a'P\x83Ta%\xD3V[\x83a&\xDAV[`\0`\x1F\x84\x11`\x01\x81\x14a'\x8AW`\0\x85\x15a'rWP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x18qV[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a'\xBBW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a'\x9BV[P\x86\x82\x10\x15a'\xD8W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a(\x13Wa(\x13a#\nV[a('\x81a(!\x84Ta%\xD3V[\x84a&\xDAV[` \x80`\x1F\x83\x11`\x01\x81\x14a(\\W`\0\x84\x15a(DWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua'\"V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a(\x8BW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a(lV[P\x85\x82\x10\x15a(\xA9W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x82Qa(\xCB\x81\x84` \x87\x01a\x1FUV[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a(\xFDW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1F\xB8W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x85\xE9\x9E/\x89\xF6V\xB1U\xC9\xE7\x99\xCB\xB3\x19!\xB29\xA4t\x7FL\x0C\xDA\x1E\xFAk\xC4zrTCdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static HYPERDRIVEREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01XW`\x005`\xE0\x1C\x80c\x9BrJ\xD4\x11a\0\xC3W\x80c\xE2\xF2s\xBD\x11a\0|W\x80c\xE2\xF2s\xBD\x14a\x03\xF8W\x80c\xE9g\xE3\x88\x14a\x04\x0BW\x80c\xEA5\x03!\x14a\x04\x1EW\x80c\xF3,\x9E4\x14a\x04>W\x80c\xF5\x9D\0\xB9\x14a\x04^W\x80c\xF8Q\xA4@\x14a\x04fW`\0\x80\xFD[\x80c\x9BrJ\xD4\x14a\x03\x10W\x80c\xA5\x87\xBB\xE1\x14a\x03gW\x80c\xB7>?\xAB\x14a\x03\x92W\x80c\xBC0\xE7\xA1\x14a\x03\xB2W\x80c\xD2\xF7-R\x14a\x03\xC5W\x80c\xDA\xAC$\xDA\x14a\x03\xE5W`\0\x80\xFD[\x80cM\xB6\xC0\xE0\x11a\x01\x15W\x80cM\xB6\xC0\xE0\x14a\x02rW\x80cT\xFDMP\x14a\x02\x92W\x80cn\x95\xD6|\x14a\x02\xB8W\x80cqk\xA5\xF6\x14a\x02\xCAW\x80cz\xB43\x9D\x14a\x02\xEAW\x80c\x84\xDA\x92\xA7\x14a\x02\xFDW`\0\x80\xFD[\x80c\x04\xBA\xA0\x0B\x14a\x01]W\x80c\x06\xFD\xDE\x03\x14a\x01\xA4W\x80c\x18\xBB;T\x14a\x01\xACW\x80c\x1F\xF3\n\xD2\x14a\x02 W\x80c*\xD1\x9D\xE8\x14a\x025W\x80c9.S\xCD\x14a\x02UW[`\0\x80\xFD[a\x01\x8E`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qHyperdriveRegistry`p\x1B\x81RP\x81V[`@Qa\x01\x9B\x91\x90a\x1F\xA5V[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Ea\x04yV[a\x02\x13a\x01\xBA6`\x04a\x1F\xDBV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x81\x81R`\x06` \x81\x81R\x94\x82 \x80T`\x01`\x01`\x80\x1B\x03\x16\x85R\x92\x90\x91R\x83R`\x01\x01T\x90\x92\x16\x90\x82\x01R\x90V[`@Qa\x01\x9B\x91\x90a\x1F\xF6V[a\x023a\x02.6`\x04a aV[a\x05\x07V[\0[a\x02Ha\x02C6`\x04a\x1F\xDBV[a\x08\xD2V[`@Qa\x01\x9B\x91\x90a!gV[`\0Ta\x02b\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\x9BV[a\x02\x85a\x02\x806`\x04a!zV[a\naV[`@Qa\x01\x9B\x91\x90a!\xBBV[a\x01\x8E`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f\x07c\x12\xE3\x02\xE3#`\xCC\x1B\x81RP\x81V[`\x05T[`@Q\x90\x81R` \x01a\x01\x9BV[a\x02\xDDa\x02\xD86`\x04a\"\x1BV[a\x0B\xAAV[`@Qa\x01\x9B\x91\x90a\"=V[a\x023a\x02\xF86`\x04a\"\x8AV[a\x0C\xA9V[a\x023a\x03\x0B6`\x04a#wV[a\rLV[a\x03Xa\x03\x1E6`\x04a\x1F\xDBV[`@\x80Q` \x80\x82\x01\x83R`\0\x91\x82\x90R\x82Q\x80\x82\x01\x84R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x82R`\x04\x90R T`\x01`\x01`\x80\x1B\x03\x16\x81R\x90V[`@Q\x90Q\x81R` \x01a\x01\x9BV[a\x03za\x03u6`\x04a#\xF6V[a\r\xC4V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x9BV[a\x03\xA5a\x03\xA06`\x04a!zV[a\r\xF4V[`@Qa\x01\x9B\x91\x90a$aV[a\x02\xDDa\x03\xC06`\x04a\"\x1BV[a\x10\x83V[a\x03\xD8a\x03\xD36`\x04a!zV[a\x11\x82V[`@Qa\x01\x9B\x91\x90a$\xC5V[a\x03za\x03\xF36`\x04a#\xF6V[a\x14CV[a\x023a\x04\x066`\x04a\x1F\xDBV[a\x14XV[a\x023a\x04\x196`\x04a%\x1CV[a\x14\xCCV[a\x041a\x04,6`\x04a!zV[a\x18xV[`@Qa\x01\x9B\x91\x90a%\x87V[a\x04Qa\x04L6`\x04a\x1F\xDBV[a\x19bV[`@Qa\x01\x9B\x91\x90a%\xC0V[`\x03Ta\x02\xBCV[`\x02Ta\x03z\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01\x80Ta\x04\x86\x90a%\xD3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xB2\x90a%\xD3V[\x80\x15a\x04\xFFW\x80`\x1F\x10a\x04\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xFFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xE2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x051W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x83\x14\x15\x80a\x05@WP\x84\x81\x14\x15[\x15a\x05^W`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x85\x81\x10\x15a\x08\xC9W`\0`\x06`\0\x89\x89\x85\x81\x81\x10a\x05\x81Wa\x05\x81a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x05\x96\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\x80\x1B\x03\x16\x90P\x85\x85\x83\x81\x81\x10a\x05\xCDWa\x05\xCDa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x05\xE2\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15a\x05\xF7WP\x80\x15\x15[\x15a\x06\x80W`\0\x84\x84\x84\x81\x81\x10a\x06\x10Wa\x06\x10a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x06%\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06LW`@QczD\xDB\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06{\x88\x88\x84\x81\x81\x10a\x06aWa\x06aa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x06v\x91\x90a\x1F\xDBV[a\x19\xF7V[a\x08\x03V[\x85\x85\x83\x81\x81\x10a\x06\x92Wa\x06\x92a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x06\xA7\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x06\xBDWP\x80\x15\x15[\x15a\x07?Wa\x06{\x88\x88\x84\x81\x81\x10a\x06\xD7Wa\x06\xD7a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x06\xEC\x91\x90a\x1F\xDBV[\x87\x87\x85\x81\x81\x10a\x06\xFEWa\x06\xFEa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x07\x13\x91\x90a&#V[\x86\x86\x86\x81\x81\x10a\x07%Wa\x07%a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x07:\x91\x90a\x1F\xDBV[a\x1BMV[\x85\x85\x83\x81\x81\x10a\x07QWa\x07Qa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x07f\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x07{WP\x80\x15[\x15a\x07\xFDWa\x06{\x88\x88\x84\x81\x81\x10a\x07\x95Wa\x07\x95a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x07\xAA\x91\x90a\x1F\xDBV[\x87\x87\x85\x81\x81\x10a\x07\xBCWa\x07\xBCa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x07\xD1\x91\x90a&#V[\x86\x86\x86\x81\x81\x10a\x07\xE3Wa\x07\xE3a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x07\xF8\x91\x90a\x1F\xDBV[a\x1C\x94V[Pa\x08\xC1V[\x83\x83\x83\x81\x81\x10a\x08\x15Wa\x08\x15a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x08*\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x84\x81\x81\x10a\x08EWa\x08Ea&\rV[\x90P` \x02\x01` \x81\x01\x90a\x08Z\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x89\x89\x85\x81\x81\x10a\x08uWa\x08ua&\rV[\x90P` \x02\x01` \x81\x01\x90a\x08\x8A\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDC\xDD\xA2\xB8&{\x8F\xE0\xEB\xFE\xB2\xCC\x8F&h\x07\xB4\x12\xBE\xC0\x96\xD1l\xBB\xE5v\xD4m\x12%S\xE0`@Q`@Q\x80\x91\x03\x90\xA4P[`\x01\x01a\x05aV[PPPPPPPV[a\x08\xDAa\x1F\x1DV[`@\x80Q`\xA0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x06` \x81\x81R\x86\x83 \x80T`\x01`\x01`\x80\x1B\x03\x16\x87R\x84\x84R\x91\x81R`\x01\x90\x91\x01T\x90\x93\x16\x92\x84\x01\x92\x90\x92R\x83Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x84Q\x87\x95\x85\x01\x93c\x06\xFD\xDE\x03\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\tZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\x82\x91\x90\x81\x01\x90a&LV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x04\xBA\xA0\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\xED\x91\x90\x81\x01\x90a&LV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16cT\xFDMP`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\nX\x91\x90\x81\x01\x90a&LV[\x90R\x93\x92PPPV[``\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\n{Wa\n{a#\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xC0W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\n\x99W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B\xA2W`@Q\x80`@\x01`@R\x80`\x06`\0\x87\x87\x86\x81\x81\x10a\n\xEFWa\n\xEFa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x0B\x04\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 T`\x01`\x01`\x80\x1B\x03\x16\x83R\x91\x01\x90`\x06\x90\x87\x87\x86\x81\x81\x10a\x0BDWa\x0BDa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x0BY\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x92\x90\x92R`@\x01`\0 `\x01\x01T\x16\x90R\x82Q\x83\x90\x83\x90\x81\x10a\x0B\x8FWa\x0B\x8Fa&\rV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\n\xC6V[P[\x92\x91PPV[``\x81\x83\x10a\x0B\xCCW`@Qc;'5\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T\x82\x11\x15a\x0B\xEFW`@Qc\xE0\xF7\xBE\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xF9\x83\x83a&\xB9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\x10Wa\x0C\x10a#\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82[\x82\x81\x10\x15a\x0B\xA2W`\x03\x81\x81T\x81\x10a\x0CYWa\x0CYa&\rV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x82a\x0Cy\x86\x84a&\xB9V[\x81Q\x81\x10a\x0C\x89Wa\x0C\x89a&\rV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x0C>V[`\0T`\xFF\x16\x15a\x0C\xCDW`@Qcr,9[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91Ua\x0C\xE8\x83\x85\x83a'+V[P`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Qa\r\x17\x90\x85\x90\x85\x90a'\xEAV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xE5t9\xD87qU\x89\xA7Q\xE3\x91\xF7:\x95\x92\x9E\xCC\x96\xAEj\xA0\x9A\x16\xBE\x8A\xE3\x12\xD6\x14qW\x90`\0\x90\xA3PPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\rvW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01a\r\x82\x82\x82a'\xFAV[P\x80`@Qa\r\x91\x91\x90a(\xB9V[`@Q\x90\x81\x90\x03\x81 \x90\x7F\x9Fv\x88\xA9\x7F\x1A\xC5\x1F\xE0;\xAC\x18\xAF\x18\xD6\x81\x0F\x9F\x11\xF0\xDB\x08\xC5\x9B\x198\xA9\xAC\x82^\xF7D\x90`\0\x90\xA2PV[`\0`\x03\x82\x81T\x81\x10a\r\xD9Wa\r\xD9a&\rV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[``\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\x0EWa\x0E\x0Ea#\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0EjW\x81` \x01[a\x0EW`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0E,W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B\xA2W`\0\x84\x84\x83\x81\x81\x10a\x0E\x8CWa\x0E\x8Ca&\rV[\x90P` \x02\x01` \x81\x01\x90a\x0E\xA1\x91\x90a\x1F\xDBV[\x90P`@Q\x80`\x80\x01`@R\x80`\x04`\0\x88\x88\x87\x81\x81\x10a\x0E\xC4Wa\x0E\xC4a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x0E\xD9\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x06\xFD\xDE\x03`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F\x84\x91\x90\x81\x01\x90a&LV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x04\xBA\xA0\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F\xEF\x91\x90\x81\x01\x90a&LV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16cT\xFDMP`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x102W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x10Z\x91\x90\x81\x01\x90a&LV[\x81RP\x83\x83\x81Q\x81\x10a\x10oWa\x10oa&\rV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0EpV[``\x81\x83\x10a\x10\xA5W`@Qc;'5\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T\x82\x11\x15a\x10\xC8W`@Qc\xE0\xF7\xBE\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xD2\x83\x83a&\xB9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\xE9Wa\x10\xE9a#\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\x12W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82[\x82\x81\x10\x15a\x0B\xA2W`\x05\x81\x81T\x81\x10a\x112Wa\x112a&\rV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x82a\x11R\x86\x84a&\xB9V[\x81Q\x81\x10a\x11bWa\x11ba&\rV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x11\x17V[``\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x9CWa\x11\x9Ca#\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xD5W\x81` \x01[a\x11\xC2a\x1F\x1DV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x11\xBAW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B\xA2W`\0\x84\x84\x83\x81\x81\x10a\x11\xF7Wa\x11\xF7a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x12\x0C\x91\x90a\x1F\xDBV[\x90P`@Q\x80`\xA0\x01`@R\x80`\x06`\0\x88\x88\x87\x81\x81\x10a\x12/Wa\x12/a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x12D\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 T`\x01`\x01`\x80\x1B\x03\x16\x83R\x91\x01\x90`\x06\x90\x88\x88\x87\x81\x81\x10a\x12\x84Wa\x12\x84a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x12\x99\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x06\xFD\xDE\x03`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13D\x91\x90\x81\x01\x90a&LV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x04\xBA\xA0\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xAF\x91\x90\x81\x01\x90a&LV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16cT\xFDMP`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x14\x1A\x91\x90\x81\x01\x90a&LV[\x81RP\x83\x83\x81Q\x81\x10a\x14/Wa\x14/a&\rV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x11\xDBV[`\0`\x05\x82\x81T\x81\x10a\r\xD9Wa\r\xD9a&\rV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\x82W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7FT\xE4a'\x88\xF9\x03\x84\xE6\x842\x98\xD7\x85D6\xF3\xA5\x85\xB2\xC3\x83\x1A\xB6j\xBF\x1D\xE6;\xFAl-\x90`\0\x90\xA2PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\xF6W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x81\x14a\x15\x16W`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x83\x81\x10\x15a\x18qW`\0`\x04`\0\x87\x87\x85\x81\x81\x10a\x159Wa\x159a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x15N\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\x80\x1B\x03\x16\x90P\x83\x83\x83\x81\x81\x10a\x15\x85Wa\x15\x85a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x15\x9A\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15a\x15\xAFWP\x80\x15\x15[\x15a\x15\xE8Wa\x15\xE3\x86\x86\x84\x81\x81\x10a\x15\xC9Wa\x15\xC9a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x15\xDE\x91\x90a\x1F\xDBV[a\x1D\xD9V[a\x17\xDBV[\x83\x83\x83\x81\x81\x10a\x15\xFAWa\x15\xFAa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x16\x0F\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x16%WP\x80\x15\x15[\x15a\x16\xB4Wa\x15\xE3\x86\x86\x84\x81\x81\x10a\x16?Wa\x16?a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x16T\x91\x90a\x1F\xDBV[\x85\x85\x85\x81\x81\x10a\x16fWa\x16fa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x16{\x91\x90a&#V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[\x83\x83\x83\x81\x81\x10a\x16\xC6Wa\x16\xC6a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x16\xDB\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x16\xF0WP\x80\x15[\x15a\x17\xD5Wa\x15\xE3\x86\x86\x84\x81\x81\x10a\x17\nWa\x17\na&\rV[\x90P` \x02\x01` \x81\x01\x90a\x17\x1F\x91\x90a\x1F\xDBV[\x85\x85\x85\x81\x81\x10a\x171Wa\x171a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x17F\x91\x90a&#V[`\x03\x80T`\x01\x81\x01\x90\x91U\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x94\x16\x84\x17\x90U`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x81R\x91\x83\x16` \x83\x81\x01\x91\x82R`\0\x95\x86R`\x04\x90R\x93 \x90Q\x92Q\x82\x16`\x01`\x80\x1B\x02\x92\x90\x91\x16\x91\x90\x91\x17\x90UV[Pa\x18iV[\x83\x83\x83\x81\x81\x10a\x17\xEDWa\x17\xEDa&\rV[\x90P` \x02\x01` \x81\x01\x90a\x18\x02\x91\x90a&#V[`\x01`\x01`\x80\x1B\x03\x16\x86\x86\x84\x81\x81\x10a\x18\x1DWa\x18\x1Da&\rV[\x90P` \x02\x01` \x81\x01\x90a\x182\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD8@\xEA\x8C\xB0B\xBC\x84\r>U\xA0F\x18\xAB&\x844\xD3\xD0\xA2\x18c\x83`\xA3\x0F\xDB\x80\xDEc\xF6`@Q`@Q\x80\x91\x03\x90\xA3P[`\x01\x01a\x15\x19V[PPPPPV[``\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x92Wa\x18\x92a#\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xD2W\x81` \x01[`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x18\xB0W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B\xA2W`@Q\x80` \x01`@R\x80`\x04`\0\x87\x87\x86\x81\x81\x10a\x19\x01Wa\x19\x01a&\rV[\x90P` \x02\x01` \x81\x01\x90a\x19\x16\x91\x90a\x1F\xDBV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\x80\x1B\x03\x16\x90R\x82Q\x83\x90\x83\x90\x81\x10a\x19OWa\x19Oa&\rV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x18\xD8V[a\x19\x8D`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81R`\x04` \x81\x81R\x85\x83 T`\x01`\x01`\x80\x1B\x03\x16\x85R\x85Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x95Q\x88\x96\x91\x86\x01\x94\x93c\x06\xFD\xDE\x03\x93\x83\x81\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\tZW=`\0\x80>=`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x06` R`@\x90 T`\x05T`\x01`\x80\x1B\x90\x91\x04`\x01`\x01`\x80\x1B\x03\x16\x90a\x1A0`\x01\x82a&\xB9V[\x82`\x01`\x01`\x80\x1B\x03\x16\x14a\x1A\xE8W`\0`\x05a\x1AN`\x01\x84a&\xB9V[\x81T\x81\x10a\x1A^Wa\x1A^a&\rV[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80\x83R`\x06\x90\x91R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x80\x87\x16`\x01`\x80\x1B\x81\x02\x91\x90\x92\x16\x17\x90\x91U`\x05\x80T\x92\x93P\x83\x92\x90\x91\x90\x81\x10a\x1A\xB8Wa\x1A\xB8a&\rV[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPP[`\x05\x80T\x80a\x1A\xF9Wa\x1A\xF9a(\xD5V[`\0\x82\x81R` \x80\x82 \x83\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U\x93\x01\x90\x93U`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x85R`\x06\x90\x91R`@\x84 \x93\x84U`\x01\x93\x90\x93\x01\x80T\x90\x93\x16\x90\x92UPPV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x06` R`@\x90 `\x01\x01T\x16\x80\x15\x80\x15\x90a\x1B\x8CWP\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x80a\x1C$WP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15a\x1B\xB1WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[\x80\x15a\x1C$WP`@Qc5\xA2s_`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x83\x16\x90ckD\xE6\xBE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\"\x91\x90a(\xEBV[\x15[\x15a\x1CBW`@QczD\xDB\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x90\x81R`\x06` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x82U`\x01\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x1D\x13WP`@Qc5\xA2s_`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x82\x16\x90ckD\xE6\xBE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xEDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x11\x91\x90a(\xEBV[\x15[\x15a\x1D1W`@QczD\xDB\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80T`\x01\x80\x82\x01\x90\x92U\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB0\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x81\x17\x90\x92U`@\x80Q``\x81\x01\x82R`\x01`\x01`\x80\x1B\x03\x97\x88\x16\x81R\x93\x87\x16` \x85\x81\x01\x91\x82R\x96\x89\x16\x85\x83\x01\x90\x81R`\0\x94\x85R`\x06\x90\x97R\x92 \x92Q\x91Q\x86\x16`\x01`\x80\x1B\x02\x91\x90\x95\x16\x17\x81U\x91Q\x91\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x04` R`@\x90 T`\x03T`\x01`\x80\x1B\x90\x91\x04`\x01`\x01`\x80\x1B\x03\x16\x90a\x1E\x12`\x01\x82a&\xB9V[\x82`\x01`\x01`\x80\x1B\x03\x16\x14a\x1E\xCAW`\0`\x03a\x1E0`\x01\x84a&\xB9V[\x81T\x81\x10a\x1E@Wa\x1E@a&\rV[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80\x83R`\x04\x90\x91R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x80\x87\x16`\x01`\x80\x1B\x81\x02\x91\x90\x92\x16\x17\x90\x91U`\x03\x80T\x92\x93P\x83\x92\x90\x91\x90\x81\x10a\x1E\x9AWa\x1E\x9Aa&\rV[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPP[`\x03\x80T\x80a\x1E\xDBWa\x1E\xDBa(\xD5V[`\0\x82\x81R` \x80\x82 \x83\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x90\x92\x01\x90\x92U`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x81R`\x04\x90\x93RPP`@\x81 UV[`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0[\x83\x81\x10\x15a\x1FpW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1FXV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1F\x91\x81` \x86\x01` \x86\x01a\x1FUV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1F\xB8` \x83\x01\x84a\x1FyV[\x93\x92PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1F\xD6W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1F\xEDW`\0\x80\xFD[a\x1F\xB8\x82a\x1F\xBFV[\x81Q\x81R` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`@\x81\x01a\x0B\xA4V[`\0\x80\x83`\x1F\x84\x01\x12a (W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a ?W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a ZW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a zW`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a \x91W`\0\x80\xFD[a \x9D\x8A\x83\x8B\x01a \x16V[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a \xB6W`\0\x80\xFD[a \xC2\x8A\x83\x8B\x01a \x16V[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15a \xDBW`\0\x80\xFD[Pa \xE8\x89\x82\x8A\x01a \x16V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[\x80Q\x82R`\x01\x80`\xA0\x1B\x03` \x82\x01Q\x16` \x83\x01R`\0`@\x82\x01Q`\xA0`@\x85\x01Ra!+`\xA0\x85\x01\x82a\x1FyV[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01Ra!D\x82\x82a\x1FyV[\x91PP`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra!^\x82\x82a\x1FyV[\x95\x94PPPPPV[` \x81R`\0a\x1F\xB8` \x83\x01\x84a \xFAV[`\0\x80` \x83\x85\x03\x12\x15a!\x8DW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xA3W`\0\x80\xFD[a!\xAF\x85\x82\x86\x01a \x16V[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\"\x0EWa!\xFE\x84\x83Q\x80Q\x82R` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a!\xD8V[P\x91\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\".W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\"~W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\"YV[P\x90\x96\x95PPPPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\"\x9FW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\"\xB6W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\"\xCAW`\0\x80\xFD[\x815\x81\x81\x11\x15a\"\xD9W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\"\xEBW`\0\x80\xFD[` \x92\x83\x01\x95P\x93Pa#\x01\x91\x86\x01\x90Pa\x1F\xBFV[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a#HWa#Ha#\nV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a#iWa#ia#\nV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15a#\x89W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a#\x9FW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a#\xB0W`\0\x80\xFD[\x805a#\xC3a#\xBE\x82a#PV[a# V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a#\xD8W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a$\x08W`\0\x80\xFD[P5\x91\x90PV[\x80Q\x82R`\0` \x82\x01Q`\x80` \x85\x01Ra$.`\x80\x85\x01\x82a\x1FyV[\x90P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra$G\x82\x82a\x1FyV[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra!^\x82\x82a\x1FyV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a$\xB8W`?\x19\x88\x86\x03\x01\x84Ra$\xA6\x85\x83Qa$\x0FV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a$\x8AV[P\x92\x97\x96PPPPPPPV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a$\xB8W`?\x19\x88\x86\x03\x01\x84Ra%\n\x85\x83Qa \xFAV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a$\xEEV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a%2W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a%IW`\0\x80\xFD[a%U\x88\x83\x89\x01a \x16V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a%nW`\0\x80\xFD[Pa%{\x87\x82\x88\x01a \x16V[\x95\x98\x94\x97P\x95PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\"~W\x83QQ\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a%\xA3V[` \x81R`\0a\x1F\xB8` \x83\x01\x84a$\x0FV[`\x01\x81\x81\x1C\x90\x82\x16\x80a%\xE7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a&\x07WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a&5W`\0\x80\xFD[\x815`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x1F\xB8W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a&^W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&tW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a&\x85W`\0\x80\xFD[\x80Qa&\x93a#\xBE\x82a#PV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a&\xA8W`\0\x80\xFD[a!^\x82` \x83\x01` \x86\x01a\x1FUV[\x81\x81\x03\x81\x81\x11\x15a\x0B\xA4WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a'&W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a'\x03WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a'\"W\x82\x81U`\x01\x01a'\x0FV[PPP[PPPV[`\x01`\x01`@\x1B\x03\x83\x11\x15a'BWa'Ba#\nV[a'V\x83a'P\x83Ta%\xD3V[\x83a&\xDAV[`\0`\x1F\x84\x11`\x01\x81\x14a'\x8AW`\0\x85\x15a'rWP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x18qV[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a'\xBBW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a'\x9BV[P\x86\x82\x10\x15a'\xD8W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a(\x13Wa(\x13a#\nV[a('\x81a(!\x84Ta%\xD3V[\x84a&\xDAV[` \x80`\x1F\x83\x11`\x01\x81\x14a(\\W`\0\x84\x15a(DWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua'\"V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a(\x8BW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a(lV[P\x85\x82\x10\x15a(\xA9W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x82Qa(\xCB\x81\x84` \x87\x01a\x1FUV[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a(\xFDW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1F\xB8W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x85\xE9\x9E/\x89\xF6V\xB1U\xC9\xE7\x99\xCB\xB3\x19!\xB29\xA4t\x7FL\x0C\xDA\x1E\xFAk\xC4zrTCdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static HYPERDRIVEREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct HyperdriveRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for HyperdriveRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for HyperdriveRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for HyperdriveRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for HyperdriveRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(HyperdriveRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> HyperdriveRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    HYPERDRIVEREGISTRY_ABI.clone(),
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
                HYPERDRIVEREGISTRY_ABI.clone(),
                HYPERDRIVEREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `admin` (0xf851a440) function
        pub fn admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFactoriesInRange` (0x716ba5f6) function
        pub fn get_factories_in_range(
            &self,
            start_index: ::ethers::core::types::U256,
            end_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([113, 107, 165, 246], (start_index, end_index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFactoryAtIndex` (0xa587bbe1) function
        pub fn get_factory_at_index(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([165, 135, 187, 225], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFactoryInfo` (0x9b724ad4) function
        pub fn get_factory_info(
            &self,
            factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, FactoryInfo> {
            self.0
                .method_hash([155, 114, 74, 212], factory)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFactoryInfoWithMetadata` (0xf32c9e34) function
        pub fn get_factory_info_with_metadata(
            &self,
            factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, FactoryInfoWithMetadata> {
            self.0
                .method_hash([243, 44, 158, 52], factory)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFactoryInfos` (0xea350321) function
        pub fn get_factory_infos(
            &self,
            factories: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<FactoryInfo>,
        > {
            self.0
                .method_hash([234, 53, 3, 33], factories)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFactoryInfosWithMetadata` (0xb73e3fab) function
        pub fn get_factory_infos_with_metadata(
            &self,
            factories: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<FactoryInfoWithMetadata>,
        > {
            self.0
                .method_hash([183, 62, 63, 171], factories)
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
        ///Calls the contract's `getInstanceInfo` (0x18bb3b54) function
        pub fn get_instance_info(
            &self,
            instance: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, InstanceInfo> {
            self.0
                .method_hash([24, 187, 59, 84], instance)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInstanceInfoWithMetadata` (0x2ad19de8) function
        pub fn get_instance_info_with_metadata(
            &self,
            instance: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, InstanceInfoWithMetadata> {
            self.0
                .method_hash([42, 209, 157, 232], instance)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInstanceInfos` (0x4db6c0e0) function
        pub fn get_instance_infos(
            &self,
            instances: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<InstanceInfo>,
        > {
            self.0
                .method_hash([77, 182, 192, 224], instances)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInstanceInfosWithMetadata` (0xd2f72d52) function
        pub fn get_instance_infos_with_metadata(
            &self,
            instances: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<InstanceInfoWithMetadata>,
        > {
            self.0
                .method_hash([210, 247, 45, 82], instances)
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
        ///Calls the contract's `getNumberOfFactories` (0xf59d00b9) function
        pub fn get_number_of_factories(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([245, 157, 0, 185], ())
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
        ///Calls the contract's `initialize` (0x7ab4339d) function
        pub fn initialize(
            &self,
            name: ::std::string::String,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 180, 51, 157], (name, admin))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isInitialized` (0x392e53cd) function
        pub fn is_initialized(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 46, 83, 205], ())
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
        ///Calls the contract's `setFactoryInfo` (0xe967e388) function
        pub fn set_factory_info(
            &self,
            factories: ::std::vec::Vec<::ethers::core::types::Address>,
            data: ::std::vec::Vec<u128>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 103, 227, 136], (factories, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setInstanceInfo` (0x1ff30ad2) function
        pub fn set_instance_info(
            &self,
            instances: ::std::vec::Vec<::ethers::core::types::Address>,
            data: ::std::vec::Vec<u128>,
            factories: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 243, 10, 210], (instances, data, factories))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateAdmin` (0xe2f273bd) function
        pub fn update_admin(
            &self,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 242, 115, 189], admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateName` (0x84da92a7) function
        pub fn update_name(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 218, 146, 167], name)
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
        ///Gets the contract's `AdminUpdated` event
        pub fn admin_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FactoryInfoUpdated` event
        pub fn factory_info_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FactoryInfoUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `InstanceInfoUpdated` event
        pub fn instance_info_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InstanceInfoUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NameUpdated` event
        pub fn name_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NameUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            HyperdriveRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for HyperdriveRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Custom Error type `InputLengthMismatch` with signature `InputLengthMismatch()` and selector `0xaaad13f7`
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
    #[etherror(name = "InputLengthMismatch", abi = "InputLengthMismatch()")]
    pub struct InputLengthMismatch;
    ///Custom Error type `InvalidFactory` with signature `InvalidFactory()` and selector `0x7a44db95`
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
    #[etherror(name = "InvalidFactory", abi = "InvalidFactory()")]
    pub struct InvalidFactory;
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
    ///Custom Error type `RegistryAlreadyInitialized` with signature `RegistryAlreadyInitialized()` and selector `0xe45872b6`
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
        name = "RegistryAlreadyInitialized",
        abi = "RegistryAlreadyInitialized()"
    )]
    pub struct RegistryAlreadyInitialized;
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
    pub enum HyperdriveRegistryErrors {
        EndIndexTooLarge(EndIndexTooLarge),
        InputLengthMismatch(InputLengthMismatch),
        InvalidFactory(InvalidFactory),
        InvalidIndexes(InvalidIndexes),
        RegistryAlreadyInitialized(RegistryAlreadyInitialized),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for HyperdriveRegistryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <EndIndexTooLarge as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EndIndexTooLarge(decoded));
            }
            if let Ok(decoded) = <InputLengthMismatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InputLengthMismatch(decoded));
            }
            if let Ok(decoded) = <InvalidFactory as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidFactory(decoded));
            }
            if let Ok(decoded) = <InvalidIndexes as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidIndexes(decoded));
            }
            if let Ok(decoded) = <RegistryAlreadyInitialized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegistryAlreadyInitialized(decoded));
            }
            if let Ok(decoded) = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for HyperdriveRegistryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::EndIndexTooLarge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InputLengthMismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidIndexes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegistryAlreadyInitialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for HyperdriveRegistryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <EndIndexTooLarge as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InputLengthMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidFactory as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidIndexes as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RegistryAlreadyInitialized as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for HyperdriveRegistryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EndIndexTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::InputLengthMismatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidIndexes(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistryAlreadyInitialized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for HyperdriveRegistryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<EndIndexTooLarge> for HyperdriveRegistryErrors {
        fn from(value: EndIndexTooLarge) -> Self {
            Self::EndIndexTooLarge(value)
        }
    }
    impl ::core::convert::From<InputLengthMismatch> for HyperdriveRegistryErrors {
        fn from(value: InputLengthMismatch) -> Self {
            Self::InputLengthMismatch(value)
        }
    }
    impl ::core::convert::From<InvalidFactory> for HyperdriveRegistryErrors {
        fn from(value: InvalidFactory) -> Self {
            Self::InvalidFactory(value)
        }
    }
    impl ::core::convert::From<InvalidIndexes> for HyperdriveRegistryErrors {
        fn from(value: InvalidIndexes) -> Self {
            Self::InvalidIndexes(value)
        }
    }
    impl ::core::convert::From<RegistryAlreadyInitialized> for HyperdriveRegistryErrors {
        fn from(value: RegistryAlreadyInitialized) -> Self {
            Self::RegistryAlreadyInitialized(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for HyperdriveRegistryErrors {
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
    #[ethevent(name = "AdminUpdated", abi = "AdminUpdated(address)")]
    pub struct AdminUpdatedFilter {
        #[ethevent(indexed)]
        pub admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "FactoryInfoUpdated", abi = "FactoryInfoUpdated(address,uint256)")]
    pub struct FactoryInfoUpdatedFilter {
        #[ethevent(indexed)]
        pub factory: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub data: ::ethers::core::types::U256,
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
    #[ethevent(name = "Initialized", abi = "Initialized(string,address)")]
    pub struct InitializedFilter {
        #[ethevent(indexed)]
        pub name: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub admin: ::ethers::core::types::Address,
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
        name = "InstanceInfoUpdated",
        abi = "InstanceInfoUpdated(address,uint256,address)"
    )]
    pub struct InstanceInfoUpdatedFilter {
        #[ethevent(indexed)]
        pub instance: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub data: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub factory: ::ethers::core::types::Address,
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
    #[ethevent(name = "NameUpdated", abi = "NameUpdated(string)")]
    pub struct NameUpdatedFilter {
        #[ethevent(indexed)]
        pub name: ::ethers::core::types::H256,
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
    pub enum HyperdriveRegistryEvents {
        AdminUpdatedFilter(AdminUpdatedFilter),
        FactoryInfoUpdatedFilter(FactoryInfoUpdatedFilter),
        InitializedFilter(InitializedFilter),
        InstanceInfoUpdatedFilter(InstanceInfoUpdatedFilter),
        NameUpdatedFilter(NameUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for HyperdriveRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveRegistryEvents::AdminUpdatedFilter(decoded));
            }
            if let Ok(decoded) = FactoryInfoUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveRegistryEvents::FactoryInfoUpdatedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(HyperdriveRegistryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = InstanceInfoUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveRegistryEvents::InstanceInfoUpdatedFilter(decoded));
            }
            if let Ok(decoded) = NameUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveRegistryEvents::NameUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for HyperdriveRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FactoryInfoUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InstanceInfoUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NameUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminUpdatedFilter> for HyperdriveRegistryEvents {
        fn from(value: AdminUpdatedFilter) -> Self {
            Self::AdminUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<FactoryInfoUpdatedFilter> for HyperdriveRegistryEvents {
        fn from(value: FactoryInfoUpdatedFilter) -> Self {
            Self::FactoryInfoUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for HyperdriveRegistryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<InstanceInfoUpdatedFilter> for HyperdriveRegistryEvents {
        fn from(value: InstanceInfoUpdatedFilter) -> Self {
            Self::InstanceInfoUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<NameUpdatedFilter> for HyperdriveRegistryEvents {
        fn from(value: NameUpdatedFilter) -> Self {
            Self::NameUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `admin` function with signature `admin()` and selector `0xf851a440`
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
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    ///Container type for all input parameters for the `getFactoriesInRange` function with signature `getFactoriesInRange(uint256,uint256)` and selector `0x716ba5f6`
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
        name = "getFactoriesInRange",
        abi = "getFactoriesInRange(uint256,uint256)"
    )]
    pub struct GetFactoriesInRangeCall {
        pub start_index: ::ethers::core::types::U256,
        pub end_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getFactoryAtIndex` function with signature `getFactoryAtIndex(uint256)` and selector `0xa587bbe1`
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
    #[ethcall(name = "getFactoryAtIndex", abi = "getFactoryAtIndex(uint256)")]
    pub struct GetFactoryAtIndexCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getFactoryInfo` function with signature `getFactoryInfo(address)` and selector `0x9b724ad4`
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
    #[ethcall(name = "getFactoryInfo", abi = "getFactoryInfo(address)")]
    pub struct GetFactoryInfoCall {
        pub factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getFactoryInfoWithMetadata` function with signature `getFactoryInfoWithMetadata(address)` and selector `0xf32c9e34`
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
        name = "getFactoryInfoWithMetadata",
        abi = "getFactoryInfoWithMetadata(address)"
    )]
    pub struct GetFactoryInfoWithMetadataCall {
        pub factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getFactoryInfos` function with signature `getFactoryInfos(address[])` and selector `0xea350321`
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
    #[ethcall(name = "getFactoryInfos", abi = "getFactoryInfos(address[])")]
    pub struct GetFactoryInfosCall {
        pub factories: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getFactoryInfosWithMetadata` function with signature `getFactoryInfosWithMetadata(address[])` and selector `0xb73e3fab`
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
        name = "getFactoryInfosWithMetadata",
        abi = "getFactoryInfosWithMetadata(address[])"
    )]
    pub struct GetFactoryInfosWithMetadataCall {
        pub factories: ::std::vec::Vec<::ethers::core::types::Address>,
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
    ///Container type for all input parameters for the `getInstanceInfo` function with signature `getInstanceInfo(address)` and selector `0x18bb3b54`
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
    #[ethcall(name = "getInstanceInfo", abi = "getInstanceInfo(address)")]
    pub struct GetInstanceInfoCall {
        pub instance: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getInstanceInfoWithMetadata` function with signature `getInstanceInfoWithMetadata(address)` and selector `0x2ad19de8`
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
        name = "getInstanceInfoWithMetadata",
        abi = "getInstanceInfoWithMetadata(address)"
    )]
    pub struct GetInstanceInfoWithMetadataCall {
        pub instance: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getInstanceInfos` function with signature `getInstanceInfos(address[])` and selector `0x4db6c0e0`
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
    #[ethcall(name = "getInstanceInfos", abi = "getInstanceInfos(address[])")]
    pub struct GetInstanceInfosCall {
        pub instances: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getInstanceInfosWithMetadata` function with signature `getInstanceInfosWithMetadata(address[])` and selector `0xd2f72d52`
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
        name = "getInstanceInfosWithMetadata",
        abi = "getInstanceInfosWithMetadata(address[])"
    )]
    pub struct GetInstanceInfosWithMetadataCall {
        pub instances: ::std::vec::Vec<::ethers::core::types::Address>,
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
    ///Container type for all input parameters for the `getNumberOfFactories` function with signature `getNumberOfFactories()` and selector `0xf59d00b9`
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
    #[ethcall(name = "getNumberOfFactories", abi = "getNumberOfFactories()")]
    pub struct GetNumberOfFactoriesCall;
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(string,address)` and selector `0x7ab4339d`
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
    #[ethcall(name = "initialize", abi = "initialize(string,address)")]
    pub struct InitializeCall {
        pub name: ::std::string::String,
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isInitialized` function with signature `isInitialized()` and selector `0x392e53cd`
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
    #[ethcall(name = "isInitialized", abi = "isInitialized()")]
    pub struct IsInitializedCall;
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
    ///Container type for all input parameters for the `setFactoryInfo` function with signature `setFactoryInfo(address[],uint128[])` and selector `0xe967e388`
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
    #[ethcall(name = "setFactoryInfo", abi = "setFactoryInfo(address[],uint128[])")]
    pub struct SetFactoryInfoCall {
        pub factories: ::std::vec::Vec<::ethers::core::types::Address>,
        pub data: ::std::vec::Vec<u128>,
    }
    ///Container type for all input parameters for the `setInstanceInfo` function with signature `setInstanceInfo(address[],uint128[],address[])` and selector `0x1ff30ad2`
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
        name = "setInstanceInfo",
        abi = "setInstanceInfo(address[],uint128[],address[])"
    )]
    pub struct SetInstanceInfoCall {
        pub instances: ::std::vec::Vec<::ethers::core::types::Address>,
        pub data: ::std::vec::Vec<u128>,
        pub factories: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `updateAdmin` function with signature `updateAdmin(address)` and selector `0xe2f273bd`
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
    #[ethcall(name = "updateAdmin", abi = "updateAdmin(address)")]
    pub struct UpdateAdminCall {
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateName` function with signature `updateName(string)` and selector `0x84da92a7`
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
    #[ethcall(name = "updateName", abi = "updateName(string)")]
    pub struct UpdateNameCall {
        pub name: ::std::string::String,
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
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum HyperdriveRegistryCalls {
        Admin(AdminCall),
        GetFactoriesInRange(GetFactoriesInRangeCall),
        GetFactoryAtIndex(GetFactoryAtIndexCall),
        GetFactoryInfo(GetFactoryInfoCall),
        GetFactoryInfoWithMetadata(GetFactoryInfoWithMetadataCall),
        GetFactoryInfos(GetFactoryInfosCall),
        GetFactoryInfosWithMetadata(GetFactoryInfosWithMetadataCall),
        GetInstanceAtIndex(GetInstanceAtIndexCall),
        GetInstanceInfo(GetInstanceInfoCall),
        GetInstanceInfoWithMetadata(GetInstanceInfoWithMetadataCall),
        GetInstanceInfos(GetInstanceInfosCall),
        GetInstanceInfosWithMetadata(GetInstanceInfosWithMetadataCall),
        GetInstancesInRange(GetInstancesInRangeCall),
        GetNumberOfFactories(GetNumberOfFactoriesCall),
        GetNumberOfInstances(GetNumberOfInstancesCall),
        Initialize(InitializeCall),
        IsInitialized(IsInitializedCall),
        Kind(KindCall),
        Name(NameCall),
        SetFactoryInfo(SetFactoryInfoCall),
        SetInstanceInfo(SetInstanceInfoCall),
        UpdateAdmin(UpdateAdminCall),
        UpdateName(UpdateNameCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for HyperdriveRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Admin(decoded));
            }
            if let Ok(decoded) = <GetFactoriesInRangeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFactoriesInRange(decoded));
            }
            if let Ok(decoded) = <GetFactoryAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFactoryAtIndex(decoded));
            }
            if let Ok(decoded) = <GetFactoryInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFactoryInfo(decoded));
            }
            if let Ok(decoded) = <GetFactoryInfoWithMetadataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFactoryInfoWithMetadata(decoded));
            }
            if let Ok(decoded) = <GetFactoryInfosCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFactoryInfos(decoded));
            }
            if let Ok(decoded) = <GetFactoryInfosWithMetadataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFactoryInfosWithMetadata(decoded));
            }
            if let Ok(decoded) = <GetInstanceAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInstanceAtIndex(decoded));
            }
            if let Ok(decoded) = <GetInstanceInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInstanceInfo(decoded));
            }
            if let Ok(decoded) = <GetInstanceInfoWithMetadataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInstanceInfoWithMetadata(decoded));
            }
            if let Ok(decoded) = <GetInstanceInfosCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInstanceInfos(decoded));
            }
            if let Ok(decoded) = <GetInstanceInfosWithMetadataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInstanceInfosWithMetadata(decoded));
            }
            if let Ok(decoded) = <GetInstancesInRangeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInstancesInRange(decoded));
            }
            if let Ok(decoded) = <GetNumberOfFactoriesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNumberOfFactories(decoded));
            }
            if let Ok(decoded) = <GetNumberOfInstancesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNumberOfInstances(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IsInitializedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsInitialized(decoded));
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
            if let Ok(decoded) = <SetFactoryInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFactoryInfo(decoded));
            }
            if let Ok(decoded) = <SetInstanceInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetInstanceInfo(decoded));
            }
            if let Ok(decoded) = <UpdateAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateAdmin(decoded));
            }
            if let Ok(decoded) = <UpdateNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateName(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for HyperdriveRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Admin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetFactoriesInRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFactoryAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFactoryInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFactoryInfoWithMetadata(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFactoryInfos(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFactoryInfosWithMetadata(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInstanceAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInstanceInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInstanceInfoWithMetadata(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInstanceInfos(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInstanceInfosWithMetadata(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInstancesInRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNumberOfFactories(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNumberOfInstances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsInitialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Kind(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFactoryInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetInstanceInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateName(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for HyperdriveRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFactoriesInRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetFactoryAtIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFactoryInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFactoryInfoWithMetadata(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetFactoryInfos(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFactoryInfosWithMetadata(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetInstanceAtIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetInstanceInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInstanceInfoWithMetadata(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetInstanceInfos(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInstanceInfosWithMetadata(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetInstancesInRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetNumberOfFactories(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetNumberOfInstances(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsInitialized(element) => ::core::fmt::Display::fmt(element, f),
                Self::Kind(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFactoryInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetInstanceInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateName(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminCall> for HyperdriveRegistryCalls {
        fn from(value: AdminCall) -> Self {
            Self::Admin(value)
        }
    }
    impl ::core::convert::From<GetFactoriesInRangeCall> for HyperdriveRegistryCalls {
        fn from(value: GetFactoriesInRangeCall) -> Self {
            Self::GetFactoriesInRange(value)
        }
    }
    impl ::core::convert::From<GetFactoryAtIndexCall> for HyperdriveRegistryCalls {
        fn from(value: GetFactoryAtIndexCall) -> Self {
            Self::GetFactoryAtIndex(value)
        }
    }
    impl ::core::convert::From<GetFactoryInfoCall> for HyperdriveRegistryCalls {
        fn from(value: GetFactoryInfoCall) -> Self {
            Self::GetFactoryInfo(value)
        }
    }
    impl ::core::convert::From<GetFactoryInfoWithMetadataCall>
    for HyperdriveRegistryCalls {
        fn from(value: GetFactoryInfoWithMetadataCall) -> Self {
            Self::GetFactoryInfoWithMetadata(value)
        }
    }
    impl ::core::convert::From<GetFactoryInfosCall> for HyperdriveRegistryCalls {
        fn from(value: GetFactoryInfosCall) -> Self {
            Self::GetFactoryInfos(value)
        }
    }
    impl ::core::convert::From<GetFactoryInfosWithMetadataCall>
    for HyperdriveRegistryCalls {
        fn from(value: GetFactoryInfosWithMetadataCall) -> Self {
            Self::GetFactoryInfosWithMetadata(value)
        }
    }
    impl ::core::convert::From<GetInstanceAtIndexCall> for HyperdriveRegistryCalls {
        fn from(value: GetInstanceAtIndexCall) -> Self {
            Self::GetInstanceAtIndex(value)
        }
    }
    impl ::core::convert::From<GetInstanceInfoCall> for HyperdriveRegistryCalls {
        fn from(value: GetInstanceInfoCall) -> Self {
            Self::GetInstanceInfo(value)
        }
    }
    impl ::core::convert::From<GetInstanceInfoWithMetadataCall>
    for HyperdriveRegistryCalls {
        fn from(value: GetInstanceInfoWithMetadataCall) -> Self {
            Self::GetInstanceInfoWithMetadata(value)
        }
    }
    impl ::core::convert::From<GetInstanceInfosCall> for HyperdriveRegistryCalls {
        fn from(value: GetInstanceInfosCall) -> Self {
            Self::GetInstanceInfos(value)
        }
    }
    impl ::core::convert::From<GetInstanceInfosWithMetadataCall>
    for HyperdriveRegistryCalls {
        fn from(value: GetInstanceInfosWithMetadataCall) -> Self {
            Self::GetInstanceInfosWithMetadata(value)
        }
    }
    impl ::core::convert::From<GetInstancesInRangeCall> for HyperdriveRegistryCalls {
        fn from(value: GetInstancesInRangeCall) -> Self {
            Self::GetInstancesInRange(value)
        }
    }
    impl ::core::convert::From<GetNumberOfFactoriesCall> for HyperdriveRegistryCalls {
        fn from(value: GetNumberOfFactoriesCall) -> Self {
            Self::GetNumberOfFactories(value)
        }
    }
    impl ::core::convert::From<GetNumberOfInstancesCall> for HyperdriveRegistryCalls {
        fn from(value: GetNumberOfInstancesCall) -> Self {
            Self::GetNumberOfInstances(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for HyperdriveRegistryCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsInitializedCall> for HyperdriveRegistryCalls {
        fn from(value: IsInitializedCall) -> Self {
            Self::IsInitialized(value)
        }
    }
    impl ::core::convert::From<KindCall> for HyperdriveRegistryCalls {
        fn from(value: KindCall) -> Self {
            Self::Kind(value)
        }
    }
    impl ::core::convert::From<NameCall> for HyperdriveRegistryCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<SetFactoryInfoCall> for HyperdriveRegistryCalls {
        fn from(value: SetFactoryInfoCall) -> Self {
            Self::SetFactoryInfo(value)
        }
    }
    impl ::core::convert::From<SetInstanceInfoCall> for HyperdriveRegistryCalls {
        fn from(value: SetInstanceInfoCall) -> Self {
            Self::SetInstanceInfo(value)
        }
    }
    impl ::core::convert::From<UpdateAdminCall> for HyperdriveRegistryCalls {
        fn from(value: UpdateAdminCall) -> Self {
            Self::UpdateAdmin(value)
        }
    }
    impl ::core::convert::From<UpdateNameCall> for HyperdriveRegistryCalls {
        fn from(value: UpdateNameCall) -> Self {
            Self::UpdateName(value)
        }
    }
    impl ::core::convert::From<VersionCall> for HyperdriveRegistryCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `admin` function with signature `admin()` and selector `0xf851a440`
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
    pub struct AdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getFactoriesInRange` function with signature `getFactoriesInRange(uint256,uint256)` and selector `0x716ba5f6`
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
    pub struct GetFactoriesInRangeReturn {
        pub factories: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `getFactoryAtIndex` function with signature `getFactoryAtIndex(uint256)` and selector `0xa587bbe1`
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
    pub struct GetFactoryAtIndexReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getFactoryInfo` function with signature `getFactoryInfo(address)` and selector `0x9b724ad4`
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
    pub struct GetFactoryInfoReturn {
        pub info: FactoryInfo,
    }
    ///Container type for all return fields from the `getFactoryInfoWithMetadata` function with signature `getFactoryInfoWithMetadata(address)` and selector `0xf32c9e34`
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
    pub struct GetFactoryInfoWithMetadataReturn {
        pub info: FactoryInfoWithMetadata,
    }
    ///Container type for all return fields from the `getFactoryInfos` function with signature `getFactoryInfos(address[])` and selector `0xea350321`
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
    pub struct GetFactoryInfosReturn {
        pub info: ::std::vec::Vec<FactoryInfo>,
    }
    ///Container type for all return fields from the `getFactoryInfosWithMetadata` function with signature `getFactoryInfosWithMetadata(address[])` and selector `0xb73e3fab`
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
    pub struct GetFactoryInfosWithMetadataReturn {
        pub info: ::std::vec::Vec<FactoryInfoWithMetadata>,
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
    ///Container type for all return fields from the `getInstanceInfo` function with signature `getInstanceInfo(address)` and selector `0x18bb3b54`
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
    pub struct GetInstanceInfoReturn {
        pub info: InstanceInfo,
    }
    ///Container type for all return fields from the `getInstanceInfoWithMetadata` function with signature `getInstanceInfoWithMetadata(address)` and selector `0x2ad19de8`
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
    pub struct GetInstanceInfoWithMetadataReturn {
        pub info: InstanceInfoWithMetadata,
    }
    ///Container type for all return fields from the `getInstanceInfos` function with signature `getInstanceInfos(address[])` and selector `0x4db6c0e0`
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
    pub struct GetInstanceInfosReturn {
        pub info: ::std::vec::Vec<InstanceInfo>,
    }
    ///Container type for all return fields from the `getInstanceInfosWithMetadata` function with signature `getInstanceInfosWithMetadata(address[])` and selector `0xd2f72d52`
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
    pub struct GetInstanceInfosWithMetadataReturn {
        pub info: ::std::vec::Vec<InstanceInfoWithMetadata>,
    }
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
        pub instances: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `getNumberOfFactories` function with signature `getNumberOfFactories()` and selector `0xf59d00b9`
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
    pub struct GetNumberOfFactoriesReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `isInitialized` function with signature `isInitialized()` and selector `0x392e53cd`
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
    pub struct IsInitializedReturn(pub bool);
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
    ///`FactoryInfo(uint256)`
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
    pub struct FactoryInfo {
        pub data: ::ethers::core::types::U256,
    }
    ///`FactoryInfoWithMetadata(uint256,string,string,string)`
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
    pub struct FactoryInfoWithMetadata {
        pub data: ::ethers::core::types::U256,
        pub name: ::std::string::String,
        pub kind: ::std::string::String,
        pub version: ::std::string::String,
    }
    ///`InstanceInfo(uint256,address)`
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
    pub struct InstanceInfo {
        pub data: ::ethers::core::types::U256,
        pub factory: ::ethers::core::types::Address,
    }
    ///`InstanceInfoWithMetadata(uint256,address,string,string,string)`
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
    pub struct InstanceInfoWithMetadata {
        pub data: ::ethers::core::types::U256,
        pub factory: ::ethers::core::types::Address,
        pub name: ::std::string::String,
        pub kind: ::std::string::String,
        pub version: ::std::string::String,
    }
}
