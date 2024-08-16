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
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0'c8\x03\x80b\0'c\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0rV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U`\0b\0\0T\x82\x82b\0\x01\xD8V[PPb\0\x02\xA4V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15b\0\0\x86W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\0\x9EW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12b\0\0\xB3W`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\0\xC8Wb\0\0\xC8b\0\0\\V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0\0\xF3Wb\0\0\xF3b\0\0\\V[\x81`@R\x82\x81R\x88\x86\x84\x87\x01\x01\x11\x15b\0\x01\x0CW`\0\x80\xFD[`\0\x93P[\x82\x84\x10\x15b\0\x010W\x84\x84\x01\x86\x01Q\x81\x85\x01\x87\x01R\x92\x85\x01\x92b\0\x01\x11V[`\0\x86\x84\x83\x01\x01R\x80\x96PPPPPPP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x01\\W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x01}WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x01\xD3W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x01\xAEWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x01\xCFW\x82\x81U`\x01\x01b\0\x01\xBAV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x01\xF4Wb\0\x01\xF4b\0\0\\V[b\0\x02\x0C\x81b\0\x02\x05\x84Tb\0\x01GV[\x84b\0\x01\x83V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x02DW`\0\x84\x15b\0\x02+WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x01\xCFV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x02uW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x02TV[P\x85\x82\x10\x15b\0\x02\x94W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a$\xAF\x80b\0\x02\xB4`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x017W`\x005`\xE0\x1C\x80c\xA5\x87\xBB\xE1\x11a\0\xB8W\x80c\xE2\xF2s\xBD\x11a\0|W\x80c\xE2\xF2s\xBD\x14a\x03\x94W\x80c\xE9g\xE3\x88\x14a\x03\xA7W\x80c\xEA5\x03!\x14a\x03\xBAW\x80c\xF3,\x9E4\x14a\x03\xDAW\x80c\xF5\x9D\0\xB9\x14a\x03\xFAW\x80c\xF8Q\xA4@\x14a\x04\x02W`\0\x80\xFD[\x80c\xA5\x87\xBB\xE1\x14a\x03\x03W\x80c\xB7>?\xAB\x14a\x03.W\x80c\xBC0\xE7\xA1\x14a\x03NW\x80c\xD2\xF7-R\x14a\x03aW\x80c\xDA\xAC$\xDA\x14a\x03\x81W`\0\x80\xFD[\x80cM\xB6\xC0\xE0\x11a\0\xFFW\x80cM\xB6\xC0\xE0\x14a\x024W\x80cT\xFDMP\x14a\x02TW\x80cn\x95\xD6|\x14a\x02zW\x80cqk\xA5\xF6\x14a\x02\x8CW\x80c\x9BrJ\xD4\x14a\x02\xACW`\0\x80\xFD[\x80c\x04\xBA\xA0\x0B\x14a\x01<W\x80c\x06\xFD\xDE\x03\x14a\x01\x83W\x80c\x18\xBB;T\x14a\x01\x8BW\x80c\x1F\xF3\n\xD2\x14a\x01\xFFW\x80c*\xD1\x9D\xE8\x14a\x02\x14W[`\0\x80\xFD[a\x01m`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qHyperdriveRegistry`p\x1B\x81RP\x81V[`@Qa\x01z\x91\x90a\x1E,V[`@Q\x80\x91\x03\x90\xF3[a\x01ma\x04\x15V[a\x01\xF2a\x01\x996`\x04a\x1EFV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x81\x81R`\x05` \x81\x81R\x94\x82 \x80T`\x01`\x01`\x80\x1B\x03\x16\x85R\x92\x90\x91R\x83R`\x01\x01T\x90\x92\x16\x90\x82\x01R\x90V[`@Qa\x01z\x91\x90a\x1EoV[a\x02\x12a\x02\r6`\x04a\x1E\xDBV[a\x04\xA3V[\0[a\x02'a\x02\"6`\x04a\x1EFV[a\x08nV[`@Qa\x01z\x91\x90a\x1F\xE2V[a\x02Ga\x02B6`\x04a\x1F\xF5V[a\t\xFDV[`@Qa\x01z\x91\x90a 7V[a\x01m`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fv1.0.17`\xC8\x1B\x81RP\x81V[`\x04T[`@Q\x90\x81R` \x01a\x01zV[a\x02\x9Fa\x02\x9A6`\x04a \x97V[a\x0BGV[`@Qa\x01z\x91\x90a \xB9V[a\x02\xF4a\x02\xBA6`\x04a\x1EFV[`@\x80Q` \x80\x82\x01\x83R`\0\x91\x82\x90R\x82Q\x80\x82\x01\x84R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x82R`\x03\x90R T`\x01`\x01`\x80\x1B\x03\x16\x81R\x90V[`@Q\x90Q\x81R` \x01a\x01zV[a\x03\x16a\x03\x116`\x04a!\x06V[a\x0CGV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01zV[a\x03Aa\x03<6`\x04a\x1F\xF5V[a\x0CwV[`@Qa\x01z\x91\x90a!qV[a\x02\x9Fa\x03\\6`\x04a \x97V[a\x0F\x07V[a\x03ta\x03o6`\x04a\x1F\xF5V[a\x10\x07V[`@Qa\x01z\x91\x90a!\xD5V[a\x03\x16a\x03\x8F6`\x04a!\x06V[a\x12\xC9V[a\x02\x12a\x03\xA26`\x04a\x1EFV[a\x12\xDEV[a\x02\x12a\x03\xB56`\x04a\",V[a\x13RV[a\x03\xCDa\x03\xC86`\x04a\x1F\xF5V[a\x16\xFEV[`@Qa\x01z\x91\x90a\"\x98V[a\x03\xEDa\x03\xE86`\x04a\x1EFV[a\x17\xE9V[`@Qa\x01z\x91\x90a\"\xD1V[`\x02Ta\x02~V[`\x01Ta\x03\x16\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80Ta\x04\"\x90a\"\xE4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04N\x90a\"\xE4V[\x80\x15a\x04\x9BW\x80`\x1F\x10a\x04pWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\x9BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04~W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xCDW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x83\x14\x15\x80a\x04\xDCWP\x84\x81\x14\x15[\x15a\x04\xFAW`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x85\x81\x10\x15a\x08eW`\0`\x05`\0\x89\x89\x85\x81\x81\x10a\x05\x1DWa\x05\x1Da#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x052\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\x80\x1B\x03\x16\x90P\x85\x85\x83\x81\x81\x10a\x05iWa\x05ia#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x05~\x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15a\x05\x93WP\x80\x15\x15[\x15a\x06\x1CW`\0\x84\x84\x84\x81\x81\x10a\x05\xACWa\x05\xACa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x05\xC1\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\xE8W`@QczD\xDB\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\x17\x88\x88\x84\x81\x81\x10a\x05\xFDWa\x05\xFDa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x06\x12\x91\x90a\x1EFV[a\x18~V[a\x07\x9FV[\x85\x85\x83\x81\x81\x10a\x06.Wa\x06.a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x06C\x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x06YWP\x80\x15\x15[\x15a\x06\xDBWa\x06\x17\x88\x88\x84\x81\x81\x10a\x06sWa\x06sa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x06\x88\x91\x90a\x1EFV[\x87\x87\x85\x81\x81\x10a\x06\x9AWa\x06\x9Aa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x06\xAF\x91\x90a#4V[\x86\x86\x86\x81\x81\x10a\x06\xC1Wa\x06\xC1a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x06\xD6\x91\x90a\x1EFV[a\x19\xD4V[\x85\x85\x83\x81\x81\x10a\x06\xEDWa\x06\xEDa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x07\x02\x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x07\x17WP\x80\x15[\x15a\x07\x99Wa\x06\x17\x88\x88\x84\x81\x81\x10a\x071Wa\x071a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x07F\x91\x90a\x1EFV[\x87\x87\x85\x81\x81\x10a\x07XWa\x07Xa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x07m\x91\x90a#4V[\x86\x86\x86\x81\x81\x10a\x07\x7FWa\x07\x7Fa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x07\x94\x91\x90a\x1EFV[a\x1B\x1BV[Pa\x08]V[\x83\x83\x83\x81\x81\x10a\x07\xB1Wa\x07\xB1a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x07\xC6\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x84\x81\x81\x10a\x07\xE1Wa\x07\xE1a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x07\xF6\x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x89\x89\x85\x81\x81\x10a\x08\x11Wa\x08\x11a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x08&\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDC\xDD\xA2\xB8&{\x8F\xE0\xEB\xFE\xB2\xCC\x8F&h\x07\xB4\x12\xBE\xC0\x96\xD1l\xBB\xE5v\xD4m\x12%S\xE0`@Q`@Q\x80\x91\x03\x90\xA4P[`\x01\x01a\x04\xFDV[PPPPPPPV[a\x08va\x1D\xA4V[`@\x80Q`\xA0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x05` \x81\x81R\x86\x83 \x80T`\x01`\x01`\x80\x1B\x03\x16\x87R\x84\x84R\x91\x81R`\x01\x90\x91\x01T\x90\x93\x16\x92\x84\x01\x92\x90\x92R\x83Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x84Q\x87\x95\x85\x01\x93c\x06\xFD\xDE\x03\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x08\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\x1E\x91\x90\x81\x01\x90a#sV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x04\xBA\xA0\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\taW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\x89\x91\x90\x81\x01\x90a#sV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16cT\xFDMP`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\xF4\x91\x90\x81\x01\x90a#sV[\x90R\x93\x92PPPV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\x18Wa\n\x18a#]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n]W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\n6W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B?W`@Q\x80`@\x01`@R\x80`\x05`\0\x87\x87\x86\x81\x81\x10a\n\x8CWa\n\x8Ca#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\n\xA1\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 T`\x01`\x01`\x80\x1B\x03\x16\x83R\x91\x01\x90`\x05\x90\x87\x87\x86\x81\x81\x10a\n\xE1Wa\n\xE1a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\n\xF6\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x92\x90\x92R`@\x01`\0 `\x01\x01T\x16\x90R\x82Q\x83\x90\x83\x90\x81\x10a\x0B,Wa\x0B,a#\x1EV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\ncV[P[\x92\x91PPV[``\x81\x83\x10a\x0BiW`@Qc;'5\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02T\x82\x11\x15a\x0B\x8CW`@Qc\xE0\xF7\xBE\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\x96\x83\x83a$ V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xAEWa\x0B\xAEa#]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xD7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82[\x82\x81\x10\x15a\x0B?W`\x02\x81\x81T\x81\x10a\x0B\xF7Wa\x0B\xF7a#\x1EV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x82a\x0C\x17\x86\x84a$ V[\x81Q\x81\x10a\x0C'Wa\x0C'a#\x1EV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x0B\xDCV[`\0`\x02\x82\x81T\x81\x10a\x0C\\Wa\x0C\\a#\x1EV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x92Wa\x0C\x92a#]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xEEW\x81` \x01[a\x0C\xDB`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xB0W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B?W`\0\x84\x84\x83\x81\x81\x10a\r\x10Wa\r\x10a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\r%\x91\x90a\x1EFV[\x90P`@Q\x80`\x80\x01`@R\x80`\x03`\0\x88\x88\x87\x81\x81\x10a\rHWa\rHa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\r]\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x06\xFD\xDE\x03`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\x08\x91\x90\x81\x01\x90a#sV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x04\xBA\xA0\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EKW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0Es\x91\x90\x81\x01\x90a#sV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16cT\xFDMP`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\xDE\x91\x90\x81\x01\x90a#sV[\x81RP\x83\x83\x81Q\x81\x10a\x0E\xF3Wa\x0E\xF3a#\x1EV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0C\xF4V[``\x81\x83\x10a\x0F)W`@Qc;'5\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04T\x82\x11\x15a\x0FLW`@Qc\xE0\xF7\xBE\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0FV\x83\x83a$ V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FnWa\x0Fna#]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x97W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82[\x82\x81\x10\x15a\x0B?W`\x04\x81\x81T\x81\x10a\x0F\xB7Wa\x0F\xB7a#\x1EV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x82a\x0F\xD7\x86\x84a$ V[\x81Q\x81\x10a\x0F\xE7Wa\x0F\xE7a#\x1EV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x0F\x9CV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\"Wa\x10\"a#]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10[W\x81` \x01[a\x10Ha\x1D\xA4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x10@W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B?W`\0\x84\x84\x83\x81\x81\x10a\x10}Wa\x10}a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x10\x92\x91\x90a\x1EFV[\x90P`@Q\x80`\xA0\x01`@R\x80`\x05`\0\x88\x88\x87\x81\x81\x10a\x10\xB5Wa\x10\xB5a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x10\xCA\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 T`\x01`\x01`\x80\x1B\x03\x16\x83R\x91\x01\x90`\x05\x90\x88\x88\x87\x81\x81\x10a\x11\nWa\x11\na#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x11\x1F\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x06\xFD\xDE\x03`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\xCA\x91\x90\x81\x01\x90a#sV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x04\xBA\xA0\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x125\x91\x90\x81\x01\x90a#sV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16cT\xFDMP`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\xA0\x91\x90\x81\x01\x90a#sV[\x81RP\x83\x83\x81Q\x81\x10a\x12\xB5Wa\x12\xB5a#\x1EV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x10aV[`\0`\x04\x82\x81T\x81\x10a\x0C\\Wa\x0C\\a#\x1EV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13\x08W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7FT\xE4a'\x88\xF9\x03\x84\xE6\x842\x98\xD7\x85D6\xF3\xA5\x85\xB2\xC3\x83\x1A\xB6j\xBF\x1D\xE6;\xFAl-\x90`\0\x90\xA2PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13|W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x81\x14a\x13\x9CW`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x83\x81\x10\x15a\x16\xF7W`\0`\x03`\0\x87\x87\x85\x81\x81\x10a\x13\xBFWa\x13\xBFa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x13\xD4\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\x80\x1B\x03\x16\x90P\x83\x83\x83\x81\x81\x10a\x14\x0BWa\x14\x0Ba#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x14 \x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15a\x145WP\x80\x15\x15[\x15a\x14nWa\x14i\x86\x86\x84\x81\x81\x10a\x14OWa\x14Oa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x14d\x91\x90a\x1EFV[a\x1C`V[a\x16aV[\x83\x83\x83\x81\x81\x10a\x14\x80Wa\x14\x80a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x14\x95\x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x14\xABWP\x80\x15\x15[\x15a\x15:Wa\x14i\x86\x86\x84\x81\x81\x10a\x14\xC5Wa\x14\xC5a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x14\xDA\x91\x90a\x1EFV[\x85\x85\x85\x81\x81\x10a\x14\xECWa\x14\xECa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x15\x01\x91\x90a#4V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x03` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[\x83\x83\x83\x81\x81\x10a\x15LWa\x15La#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x15a\x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x15vWP\x80\x15[\x15a\x16[Wa\x14i\x86\x86\x84\x81\x81\x10a\x15\x90Wa\x15\x90a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x15\xA5\x91\x90a\x1EFV[\x85\x85\x85\x81\x81\x10a\x15\xB7Wa\x15\xB7a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x15\xCC\x91\x90a#4V[`\x02\x80T`\x01\x81\x01\x90\x91U\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCE\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x94\x16\x84\x17\x90U`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x81R\x91\x83\x16` \x83\x81\x01\x91\x82R`\0\x95\x86R`\x03\x90R\x93 \x90Q\x92Q\x82\x16`\x01`\x80\x1B\x02\x92\x90\x91\x16\x91\x90\x91\x17\x90UV[Pa\x16\xEFV[\x83\x83\x83\x81\x81\x10a\x16sWa\x16sa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x16\x88\x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x86\x86\x84\x81\x81\x10a\x16\xA3Wa\x16\xA3a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x16\xB8\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD8@\xEA\x8C\xB0B\xBC\x84\r>U\xA0F\x18\xAB&\x844\xD3\xD0\xA2\x18c\x83`\xA3\x0F\xDB\x80\xDEc\xF6`@Q`@Q\x80\x91\x03\x90\xA3P[`\x01\x01a\x13\x9FV[PPPPPV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x19Wa\x17\x19a#]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17YW\x81` \x01[`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x177W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B?W`@Q\x80` \x01`@R\x80`\x03`\0\x87\x87\x86\x81\x81\x10a\x17\x88Wa\x17\x88a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x17\x9D\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\x80\x1B\x03\x16\x90R\x82Q\x83\x90\x83\x90\x81\x10a\x17\xD6Wa\x17\xD6a#\x1EV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x17_V[a\x18\x14`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81R`\x03` \x90\x81R\x84\x82 T`\x01`\x01`\x80\x1B\x03\x16\x84R\x84Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x94Q\x87\x95\x91\x85\x01\x93\x92c\x06\xFD\xDE\x03\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x08\xF6W=`\0\x80>=`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x05` R`@\x90 T`\x04T`\x01`\x80\x1B\x90\x91\x04`\x01`\x01`\x80\x1B\x03\x16\x90a\x18\xB7`\x01\x82a$ V[\x82`\x01`\x01`\x80\x1B\x03\x16\x14a\x19oW`\0`\x04a\x18\xD5`\x01\x84a$ V[\x81T\x81\x10a\x18\xE5Wa\x18\xE5a#\x1EV[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80\x83R`\x05\x90\x91R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x80\x87\x16`\x01`\x80\x1B\x81\x02\x91\x90\x92\x16\x17\x90\x91U`\x04\x80T\x92\x93P\x83\x92\x90\x91\x90\x81\x10a\x19?Wa\x19?a#\x1EV[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPP[`\x04\x80T\x80a\x19\x80Wa\x19\x80a$AV[`\0\x82\x81R` \x80\x82 \x83\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U\x93\x01\x90\x93U`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x85R`\x05\x90\x91R`@\x84 \x93\x84U`\x01\x93\x90\x93\x01\x80T\x90\x93\x16\x90\x92UPPV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x05` R`@\x90 `\x01\x01T\x16\x80\x15\x80\x15\x90a\x1A\x13WP\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x80a\x1A\xABWP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15a\x1A8WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[\x80\x15a\x1A\xABWP`@Qc5\xA2s_`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x83\x16\x90ckD\xE6\xBE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xA9\x91\x90a$WV[\x15[\x15a\x1A\xC9W`@QczD\xDB\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x90\x81R`\x05` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x82U`\x01\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x1B\x9AWP`@Qc5\xA2s_`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x82\x16\x90ckD\xE6\xBE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BtW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x98\x91\x90a$WV[\x15[\x15a\x1B\xB8W`@QczD\xDB\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x80T`\x01\x80\x82\x01\x90\x92U\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x81\x17\x90\x92U`@\x80Q``\x81\x01\x82R`\x01`\x01`\x80\x1B\x03\x97\x88\x16\x81R\x93\x87\x16` \x85\x81\x01\x91\x82R\x96\x89\x16\x85\x83\x01\x90\x81R`\0\x94\x85R`\x05\x90\x97R\x92 \x92Q\x91Q\x86\x16`\x01`\x80\x1B\x02\x91\x90\x95\x16\x17\x81U\x91Q\x91\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` R`@\x90 T`\x02T`\x01`\x80\x1B\x90\x91\x04`\x01`\x01`\x80\x1B\x03\x16\x90a\x1C\x99`\x01\x82a$ V[\x82`\x01`\x01`\x80\x1B\x03\x16\x14a\x1DQW`\0`\x02a\x1C\xB7`\x01\x84a$ V[\x81T\x81\x10a\x1C\xC7Wa\x1C\xC7a#\x1EV[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80\x83R`\x03\x90\x91R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x80\x87\x16`\x01`\x80\x1B\x81\x02\x91\x90\x92\x16\x17\x90\x91U`\x02\x80T\x92\x93P\x83\x92\x90\x91\x90\x81\x10a\x1D!Wa\x1D!a#\x1EV[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPP[`\x02\x80T\x80a\x1DbWa\x1Dba$AV[`\0\x82\x81R` \x80\x82 \x83\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x90\x92\x01\x90\x92U`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x81R`\x03\x90\x93RPP`@\x81 UV[`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0[\x83\x81\x10\x15a\x1D\xF7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1D\xDFV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1E\x18\x81` \x86\x01` \x86\x01a\x1D\xDCV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1E?` \x83\x01\x84a\x1E\0V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1EXW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1E?W`\0\x80\xFD[\x81Q\x81R` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`@\x81\x01a\x0BAV[`\0\x80\x83`\x1F\x84\x01\x12a\x1E\xA1W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xB9W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1E\xD4W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a\x1E\xF4W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1F\x0CW`\0\x80\xFD[a\x1F\x18\x8A\x83\x8B\x01a\x1E\x8FV[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a\x1F1W`\0\x80\xFD[a\x1F=\x8A\x83\x8B\x01a\x1E\x8FV[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15a\x1FVW`\0\x80\xFD[Pa\x1Fc\x89\x82\x8A\x01a\x1E\x8FV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[\x80Q\x82R`\x01\x80`\xA0\x1B\x03` \x82\x01Q\x16` \x83\x01R`\0`@\x82\x01Q`\xA0`@\x85\x01Ra\x1F\xA6`\xA0\x85\x01\x82a\x1E\0V[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01Ra\x1F\xBF\x82\x82a\x1E\0V[\x91PP`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra\x1F\xD9\x82\x82a\x1E\0V[\x95\x94PPPPPV[` \x81R`\0a\x1E?` \x83\x01\x84a\x1FuV[`\0\x80` \x83\x85\x03\x12\x15a \x08W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \x1FW`\0\x80\xFD[a +\x85\x82\x86\x01a\x1E\x8FV[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a \x8AWa z\x84\x83Q\x80Q\x82R` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a TV[P\x91\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a \xAAW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a \xFAW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a \xD5V[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a!\x18W`\0\x80\xFD[P5\x91\x90PV[\x80Q\x82R`\0` \x82\x01Q`\x80` \x85\x01Ra!>`\x80\x85\x01\x82a\x1E\0V[\x90P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra!W\x82\x82a\x1E\0V[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra\x1F\xD9\x82\x82a\x1E\0V[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a!\xC8W`?\x19\x88\x86\x03\x01\x84Ra!\xB6\x85\x83Qa!\x1FV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a!\x9AV[P\x92\x97\x96PPPPPPPV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a!\xC8W`?\x19\x88\x86\x03\x01\x84Ra\"\x1A\x85\x83Qa\x1FuV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a!\xFEV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\"BW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"ZW`\0\x80\xFD[a\"f\x88\x83\x89\x01a\x1E\x8FV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\"\x7FW`\0\x80\xFD[Pa\"\x8C\x87\x82\x88\x01a\x1E\x8FV[\x95\x98\x94\x97P\x95PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a \xFAW\x83QQ\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\"\xB4V[` \x81R`\0a\x1E?` \x83\x01\x84a!\x1FV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\"\xF8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a#\x18WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a#FW`\0\x80\xFD[\x815`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x1E?W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a#\x85W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#\x9DW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a#\xB1W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a#\xC3Wa#\xC3a#]V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a#\xEBWa#\xEBa#]V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a$\x04W`\0\x80\xFD[a$\x15\x83` \x83\x01` \x88\x01a\x1D\xDCV[\x97\x96PPPPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x0BAWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a$iW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1E?W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xE4\xB7\xDC\x10\x03D\x8D\xE1;k\xCA\x05d\x85F\xEF\xBAP{k\x15\x98r\xEF\xCA\x0E\xC9\x9A\x88\xB2\xF3\xC1dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static HYPERDRIVEREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x017W`\x005`\xE0\x1C\x80c\xA5\x87\xBB\xE1\x11a\0\xB8W\x80c\xE2\xF2s\xBD\x11a\0|W\x80c\xE2\xF2s\xBD\x14a\x03\x94W\x80c\xE9g\xE3\x88\x14a\x03\xA7W\x80c\xEA5\x03!\x14a\x03\xBAW\x80c\xF3,\x9E4\x14a\x03\xDAW\x80c\xF5\x9D\0\xB9\x14a\x03\xFAW\x80c\xF8Q\xA4@\x14a\x04\x02W`\0\x80\xFD[\x80c\xA5\x87\xBB\xE1\x14a\x03\x03W\x80c\xB7>?\xAB\x14a\x03.W\x80c\xBC0\xE7\xA1\x14a\x03NW\x80c\xD2\xF7-R\x14a\x03aW\x80c\xDA\xAC$\xDA\x14a\x03\x81W`\0\x80\xFD[\x80cM\xB6\xC0\xE0\x11a\0\xFFW\x80cM\xB6\xC0\xE0\x14a\x024W\x80cT\xFDMP\x14a\x02TW\x80cn\x95\xD6|\x14a\x02zW\x80cqk\xA5\xF6\x14a\x02\x8CW\x80c\x9BrJ\xD4\x14a\x02\xACW`\0\x80\xFD[\x80c\x04\xBA\xA0\x0B\x14a\x01<W\x80c\x06\xFD\xDE\x03\x14a\x01\x83W\x80c\x18\xBB;T\x14a\x01\x8BW\x80c\x1F\xF3\n\xD2\x14a\x01\xFFW\x80c*\xD1\x9D\xE8\x14a\x02\x14W[`\0\x80\xFD[a\x01m`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qHyperdriveRegistry`p\x1B\x81RP\x81V[`@Qa\x01z\x91\x90a\x1E,V[`@Q\x80\x91\x03\x90\xF3[a\x01ma\x04\x15V[a\x01\xF2a\x01\x996`\x04a\x1EFV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x81\x81R`\x05` \x81\x81R\x94\x82 \x80T`\x01`\x01`\x80\x1B\x03\x16\x85R\x92\x90\x91R\x83R`\x01\x01T\x90\x92\x16\x90\x82\x01R\x90V[`@Qa\x01z\x91\x90a\x1EoV[a\x02\x12a\x02\r6`\x04a\x1E\xDBV[a\x04\xA3V[\0[a\x02'a\x02\"6`\x04a\x1EFV[a\x08nV[`@Qa\x01z\x91\x90a\x1F\xE2V[a\x02Ga\x02B6`\x04a\x1F\xF5V[a\t\xFDV[`@Qa\x01z\x91\x90a 7V[a\x01m`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fv1.0.17`\xC8\x1B\x81RP\x81V[`\x04T[`@Q\x90\x81R` \x01a\x01zV[a\x02\x9Fa\x02\x9A6`\x04a \x97V[a\x0BGV[`@Qa\x01z\x91\x90a \xB9V[a\x02\xF4a\x02\xBA6`\x04a\x1EFV[`@\x80Q` \x80\x82\x01\x83R`\0\x91\x82\x90R\x82Q\x80\x82\x01\x84R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x82R`\x03\x90R T`\x01`\x01`\x80\x1B\x03\x16\x81R\x90V[`@Q\x90Q\x81R` \x01a\x01zV[a\x03\x16a\x03\x116`\x04a!\x06V[a\x0CGV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01zV[a\x03Aa\x03<6`\x04a\x1F\xF5V[a\x0CwV[`@Qa\x01z\x91\x90a!qV[a\x02\x9Fa\x03\\6`\x04a \x97V[a\x0F\x07V[a\x03ta\x03o6`\x04a\x1F\xF5V[a\x10\x07V[`@Qa\x01z\x91\x90a!\xD5V[a\x03\x16a\x03\x8F6`\x04a!\x06V[a\x12\xC9V[a\x02\x12a\x03\xA26`\x04a\x1EFV[a\x12\xDEV[a\x02\x12a\x03\xB56`\x04a\",V[a\x13RV[a\x03\xCDa\x03\xC86`\x04a\x1F\xF5V[a\x16\xFEV[`@Qa\x01z\x91\x90a\"\x98V[a\x03\xEDa\x03\xE86`\x04a\x1EFV[a\x17\xE9V[`@Qa\x01z\x91\x90a\"\xD1V[`\x02Ta\x02~V[`\x01Ta\x03\x16\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80Ta\x04\"\x90a\"\xE4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04N\x90a\"\xE4V[\x80\x15a\x04\x9BW\x80`\x1F\x10a\x04pWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\x9BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04~W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xCDW`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x83\x14\x15\x80a\x04\xDCWP\x84\x81\x14\x15[\x15a\x04\xFAW`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x85\x81\x10\x15a\x08eW`\0`\x05`\0\x89\x89\x85\x81\x81\x10a\x05\x1DWa\x05\x1Da#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x052\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\x80\x1B\x03\x16\x90P\x85\x85\x83\x81\x81\x10a\x05iWa\x05ia#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x05~\x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15a\x05\x93WP\x80\x15\x15[\x15a\x06\x1CW`\0\x84\x84\x84\x81\x81\x10a\x05\xACWa\x05\xACa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x05\xC1\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\xE8W`@QczD\xDB\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\x17\x88\x88\x84\x81\x81\x10a\x05\xFDWa\x05\xFDa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x06\x12\x91\x90a\x1EFV[a\x18~V[a\x07\x9FV[\x85\x85\x83\x81\x81\x10a\x06.Wa\x06.a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x06C\x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x06YWP\x80\x15\x15[\x15a\x06\xDBWa\x06\x17\x88\x88\x84\x81\x81\x10a\x06sWa\x06sa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x06\x88\x91\x90a\x1EFV[\x87\x87\x85\x81\x81\x10a\x06\x9AWa\x06\x9Aa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x06\xAF\x91\x90a#4V[\x86\x86\x86\x81\x81\x10a\x06\xC1Wa\x06\xC1a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x06\xD6\x91\x90a\x1EFV[a\x19\xD4V[\x85\x85\x83\x81\x81\x10a\x06\xEDWa\x06\xEDa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x07\x02\x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x07\x17WP\x80\x15[\x15a\x07\x99Wa\x06\x17\x88\x88\x84\x81\x81\x10a\x071Wa\x071a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x07F\x91\x90a\x1EFV[\x87\x87\x85\x81\x81\x10a\x07XWa\x07Xa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x07m\x91\x90a#4V[\x86\x86\x86\x81\x81\x10a\x07\x7FWa\x07\x7Fa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x07\x94\x91\x90a\x1EFV[a\x1B\x1BV[Pa\x08]V[\x83\x83\x83\x81\x81\x10a\x07\xB1Wa\x07\xB1a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x07\xC6\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x84\x81\x81\x10a\x07\xE1Wa\x07\xE1a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x07\xF6\x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x89\x89\x85\x81\x81\x10a\x08\x11Wa\x08\x11a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x08&\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDC\xDD\xA2\xB8&{\x8F\xE0\xEB\xFE\xB2\xCC\x8F&h\x07\xB4\x12\xBE\xC0\x96\xD1l\xBB\xE5v\xD4m\x12%S\xE0`@Q`@Q\x80\x91\x03\x90\xA4P[`\x01\x01a\x04\xFDV[PPPPPPPV[a\x08va\x1D\xA4V[`@\x80Q`\xA0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x05` \x81\x81R\x86\x83 \x80T`\x01`\x01`\x80\x1B\x03\x16\x87R\x84\x84R\x91\x81R`\x01\x90\x91\x01T\x90\x93\x16\x92\x84\x01\x92\x90\x92R\x83Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x84Q\x87\x95\x85\x01\x93c\x06\xFD\xDE\x03\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x08\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\x1E\x91\x90\x81\x01\x90a#sV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x04\xBA\xA0\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\taW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\x89\x91\x90\x81\x01\x90a#sV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16cT\xFDMP`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\xF4\x91\x90\x81\x01\x90a#sV[\x90R\x93\x92PPPV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\x18Wa\n\x18a#]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n]W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\n6W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B?W`@Q\x80`@\x01`@R\x80`\x05`\0\x87\x87\x86\x81\x81\x10a\n\x8CWa\n\x8Ca#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\n\xA1\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 T`\x01`\x01`\x80\x1B\x03\x16\x83R\x91\x01\x90`\x05\x90\x87\x87\x86\x81\x81\x10a\n\xE1Wa\n\xE1a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\n\xF6\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x92\x90\x92R`@\x01`\0 `\x01\x01T\x16\x90R\x82Q\x83\x90\x83\x90\x81\x10a\x0B,Wa\x0B,a#\x1EV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\ncV[P[\x92\x91PPV[``\x81\x83\x10a\x0BiW`@Qc;'5\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02T\x82\x11\x15a\x0B\x8CW`@Qc\xE0\xF7\xBE\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\x96\x83\x83a$ V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xAEWa\x0B\xAEa#]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xD7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82[\x82\x81\x10\x15a\x0B?W`\x02\x81\x81T\x81\x10a\x0B\xF7Wa\x0B\xF7a#\x1EV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x82a\x0C\x17\x86\x84a$ V[\x81Q\x81\x10a\x0C'Wa\x0C'a#\x1EV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x0B\xDCV[`\0`\x02\x82\x81T\x81\x10a\x0C\\Wa\x0C\\a#\x1EV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x92Wa\x0C\x92a#]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xEEW\x81` \x01[a\x0C\xDB`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xB0W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B?W`\0\x84\x84\x83\x81\x81\x10a\r\x10Wa\r\x10a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\r%\x91\x90a\x1EFV[\x90P`@Q\x80`\x80\x01`@R\x80`\x03`\0\x88\x88\x87\x81\x81\x10a\rHWa\rHa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\r]\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x06\xFD\xDE\x03`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\x08\x91\x90\x81\x01\x90a#sV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x04\xBA\xA0\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EKW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0Es\x91\x90\x81\x01\x90a#sV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16cT\xFDMP`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\xDE\x91\x90\x81\x01\x90a#sV[\x81RP\x83\x83\x81Q\x81\x10a\x0E\xF3Wa\x0E\xF3a#\x1EV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0C\xF4V[``\x81\x83\x10a\x0F)W`@Qc;'5\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04T\x82\x11\x15a\x0FLW`@Qc\xE0\xF7\xBE\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0FV\x83\x83a$ V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FnWa\x0Fna#]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x97W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82[\x82\x81\x10\x15a\x0B?W`\x04\x81\x81T\x81\x10a\x0F\xB7Wa\x0F\xB7a#\x1EV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x82a\x0F\xD7\x86\x84a$ V[\x81Q\x81\x10a\x0F\xE7Wa\x0F\xE7a#\x1EV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x0F\x9CV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\"Wa\x10\"a#]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10[W\x81` \x01[a\x10Ha\x1D\xA4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x10@W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B?W`\0\x84\x84\x83\x81\x81\x10a\x10}Wa\x10}a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x10\x92\x91\x90a\x1EFV[\x90P`@Q\x80`\xA0\x01`@R\x80`\x05`\0\x88\x88\x87\x81\x81\x10a\x10\xB5Wa\x10\xB5a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x10\xCA\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 T`\x01`\x01`\x80\x1B\x03\x16\x83R\x91\x01\x90`\x05\x90\x88\x88\x87\x81\x81\x10a\x11\nWa\x11\na#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x11\x1F\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x06\xFD\xDE\x03`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\xCA\x91\x90\x81\x01\x90a#sV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16c\x04\xBA\xA0\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x125\x91\x90\x81\x01\x90a#sV[\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16cT\xFDMP`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\xA0\x91\x90\x81\x01\x90a#sV[\x81RP\x83\x83\x81Q\x81\x10a\x12\xB5Wa\x12\xB5a#\x1EV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x10aV[`\0`\x04\x82\x81T\x81\x10a\x0C\\Wa\x0C\\a#\x1EV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13\x08W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7FT\xE4a'\x88\xF9\x03\x84\xE6\x842\x98\xD7\x85D6\xF3\xA5\x85\xB2\xC3\x83\x1A\xB6j\xBF\x1D\xE6;\xFAl-\x90`\0\x90\xA2PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13|W`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x81\x14a\x13\x9CW`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x83\x81\x10\x15a\x16\xF7W`\0`\x03`\0\x87\x87\x85\x81\x81\x10a\x13\xBFWa\x13\xBFa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x13\xD4\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\x80\x1B\x03\x16\x90P\x83\x83\x83\x81\x81\x10a\x14\x0BWa\x14\x0Ba#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x14 \x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15a\x145WP\x80\x15\x15[\x15a\x14nWa\x14i\x86\x86\x84\x81\x81\x10a\x14OWa\x14Oa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x14d\x91\x90a\x1EFV[a\x1C`V[a\x16aV[\x83\x83\x83\x81\x81\x10a\x14\x80Wa\x14\x80a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x14\x95\x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x14\xABWP\x80\x15\x15[\x15a\x15:Wa\x14i\x86\x86\x84\x81\x81\x10a\x14\xC5Wa\x14\xC5a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x14\xDA\x91\x90a\x1EFV[\x85\x85\x85\x81\x81\x10a\x14\xECWa\x14\xECa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x15\x01\x91\x90a#4V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x03` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[\x83\x83\x83\x81\x81\x10a\x15LWa\x15La#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x15a\x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15\x90a\x15vWP\x80\x15[\x15a\x16[Wa\x14i\x86\x86\x84\x81\x81\x10a\x15\x90Wa\x15\x90a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x15\xA5\x91\x90a\x1EFV[\x85\x85\x85\x81\x81\x10a\x15\xB7Wa\x15\xB7a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x15\xCC\x91\x90a#4V[`\x02\x80T`\x01\x81\x01\x90\x91U\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCE\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x94\x16\x84\x17\x90U`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x81R\x91\x83\x16` \x83\x81\x01\x91\x82R`\0\x95\x86R`\x03\x90R\x93 \x90Q\x92Q\x82\x16`\x01`\x80\x1B\x02\x92\x90\x91\x16\x91\x90\x91\x17\x90UV[Pa\x16\xEFV[\x83\x83\x83\x81\x81\x10a\x16sWa\x16sa#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x16\x88\x91\x90a#4V[`\x01`\x01`\x80\x1B\x03\x16\x86\x86\x84\x81\x81\x10a\x16\xA3Wa\x16\xA3a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x16\xB8\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD8@\xEA\x8C\xB0B\xBC\x84\r>U\xA0F\x18\xAB&\x844\xD3\xD0\xA2\x18c\x83`\xA3\x0F\xDB\x80\xDEc\xF6`@Q`@Q\x80\x91\x03\x90\xA3P[`\x01\x01a\x13\x9FV[PPPPPV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x19Wa\x17\x19a#]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17YW\x81` \x01[`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x177W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0B?W`@Q\x80` \x01`@R\x80`\x03`\0\x87\x87\x86\x81\x81\x10a\x17\x88Wa\x17\x88a#\x1EV[\x90P` \x02\x01` \x81\x01\x90a\x17\x9D\x91\x90a\x1EFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\x80\x1B\x03\x16\x90R\x82Q\x83\x90\x83\x90\x81\x10a\x17\xD6Wa\x17\xD6a#\x1EV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x17_V[a\x18\x14`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81R`\x03` \x90\x81R\x84\x82 T`\x01`\x01`\x80\x1B\x03\x16\x84R\x84Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x94Q\x87\x95\x91\x85\x01\x93\x92c\x06\xFD\xDE\x03\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x08\xF6W=`\0\x80>=`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x05` R`@\x90 T`\x04T`\x01`\x80\x1B\x90\x91\x04`\x01`\x01`\x80\x1B\x03\x16\x90a\x18\xB7`\x01\x82a$ V[\x82`\x01`\x01`\x80\x1B\x03\x16\x14a\x19oW`\0`\x04a\x18\xD5`\x01\x84a$ V[\x81T\x81\x10a\x18\xE5Wa\x18\xE5a#\x1EV[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80\x83R`\x05\x90\x91R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x80\x87\x16`\x01`\x80\x1B\x81\x02\x91\x90\x92\x16\x17\x90\x91U`\x04\x80T\x92\x93P\x83\x92\x90\x91\x90\x81\x10a\x19?Wa\x19?a#\x1EV[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPP[`\x04\x80T\x80a\x19\x80Wa\x19\x80a$AV[`\0\x82\x81R` \x80\x82 \x83\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U\x93\x01\x90\x93U`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x85R`\x05\x90\x91R`@\x84 \x93\x84U`\x01\x93\x90\x93\x01\x80T\x90\x93\x16\x90\x92UPPV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x05` R`@\x90 `\x01\x01T\x16\x80\x15\x80\x15\x90a\x1A\x13WP\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x80a\x1A\xABWP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15a\x1A8WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[\x80\x15a\x1A\xABWP`@Qc5\xA2s_`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x83\x16\x90ckD\xE6\xBE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xA9\x91\x90a$WV[\x15[\x15a\x1A\xC9W`@QczD\xDB\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x90\x81R`\x05` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x82U`\x01\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x1B\x9AWP`@Qc5\xA2s_`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x82\x16\x90ckD\xE6\xBE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BtW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x98\x91\x90a$WV[\x15[\x15a\x1B\xB8W`@QczD\xDB\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x80T`\x01\x80\x82\x01\x90\x92U\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x81\x17\x90\x92U`@\x80Q``\x81\x01\x82R`\x01`\x01`\x80\x1B\x03\x97\x88\x16\x81R\x93\x87\x16` \x85\x81\x01\x91\x82R\x96\x89\x16\x85\x83\x01\x90\x81R`\0\x94\x85R`\x05\x90\x97R\x92 \x92Q\x91Q\x86\x16`\x01`\x80\x1B\x02\x91\x90\x95\x16\x17\x81U\x91Q\x91\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` R`@\x90 T`\x02T`\x01`\x80\x1B\x90\x91\x04`\x01`\x01`\x80\x1B\x03\x16\x90a\x1C\x99`\x01\x82a$ V[\x82`\x01`\x01`\x80\x1B\x03\x16\x14a\x1DQW`\0`\x02a\x1C\xB7`\x01\x84a$ V[\x81T\x81\x10a\x1C\xC7Wa\x1C\xC7a#\x1EV[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80\x83R`\x03\x90\x91R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x80\x87\x16`\x01`\x80\x1B\x81\x02\x91\x90\x92\x16\x17\x90\x91U`\x02\x80T\x92\x93P\x83\x92\x90\x91\x90\x81\x10a\x1D!Wa\x1D!a#\x1EV[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPP[`\x02\x80T\x80a\x1DbWa\x1Dba$AV[`\0\x82\x81R` \x80\x82 \x83\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x90\x92\x01\x90\x92U`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x81R`\x03\x90\x93RPP`@\x81 UV[`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0[\x83\x81\x10\x15a\x1D\xF7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1D\xDFV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1E\x18\x81` \x86\x01` \x86\x01a\x1D\xDCV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1E?` \x83\x01\x84a\x1E\0V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1EXW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1E?W`\0\x80\xFD[\x81Q\x81R` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`@\x81\x01a\x0BAV[`\0\x80\x83`\x1F\x84\x01\x12a\x1E\xA1W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xB9W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1E\xD4W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a\x1E\xF4W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1F\x0CW`\0\x80\xFD[a\x1F\x18\x8A\x83\x8B\x01a\x1E\x8FV[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a\x1F1W`\0\x80\xFD[a\x1F=\x8A\x83\x8B\x01a\x1E\x8FV[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15a\x1FVW`\0\x80\xFD[Pa\x1Fc\x89\x82\x8A\x01a\x1E\x8FV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[\x80Q\x82R`\x01\x80`\xA0\x1B\x03` \x82\x01Q\x16` \x83\x01R`\0`@\x82\x01Q`\xA0`@\x85\x01Ra\x1F\xA6`\xA0\x85\x01\x82a\x1E\0V[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01Ra\x1F\xBF\x82\x82a\x1E\0V[\x91PP`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra\x1F\xD9\x82\x82a\x1E\0V[\x95\x94PPPPPV[` \x81R`\0a\x1E?` \x83\x01\x84a\x1FuV[`\0\x80` \x83\x85\x03\x12\x15a \x08W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \x1FW`\0\x80\xFD[a +\x85\x82\x86\x01a\x1E\x8FV[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a \x8AWa z\x84\x83Q\x80Q\x82R` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a TV[P\x91\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a \xAAW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a \xFAW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a \xD5V[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a!\x18W`\0\x80\xFD[P5\x91\x90PV[\x80Q\x82R`\0` \x82\x01Q`\x80` \x85\x01Ra!>`\x80\x85\x01\x82a\x1E\0V[\x90P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra!W\x82\x82a\x1E\0V[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra\x1F\xD9\x82\x82a\x1E\0V[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a!\xC8W`?\x19\x88\x86\x03\x01\x84Ra!\xB6\x85\x83Qa!\x1FV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a!\x9AV[P\x92\x97\x96PPPPPPPV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a!\xC8W`?\x19\x88\x86\x03\x01\x84Ra\"\x1A\x85\x83Qa\x1FuV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a!\xFEV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\"BW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"ZW`\0\x80\xFD[a\"f\x88\x83\x89\x01a\x1E\x8FV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\"\x7FW`\0\x80\xFD[Pa\"\x8C\x87\x82\x88\x01a\x1E\x8FV[\x95\x98\x94\x97P\x95PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a \xFAW\x83QQ\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\"\xB4V[` \x81R`\0a\x1E?` \x83\x01\x84a!\x1FV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\"\xF8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a#\x18WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a#FW`\0\x80\xFD[\x815`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x1E?W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a#\x85W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#\x9DW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a#\xB1W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a#\xC3Wa#\xC3a#]V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a#\xEBWa#\xEBa#]V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a$\x04W`\0\x80\xFD[a$\x15\x83` \x83\x01` \x88\x01a\x1D\xDCV[\x97\x96PPPPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x0BAWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a$iW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1E?W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xE4\xB7\xDC\x10\x03D\x8D\xE1;k\xCA\x05d\x85F\xEF\xBAP{k\x15\x98r\xEF\xCA\x0E\xC9\x9A\x88\xB2\xF3\xC1dsolcC\0\x08\x16\x003";
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
        InstanceInfoUpdatedFilter(InstanceInfoUpdatedFilter),
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
            if let Ok(decoded) = InstanceInfoUpdatedFilter::decode_log(log) {
                return Ok(HyperdriveRegistryEvents::InstanceInfoUpdatedFilter(decoded));
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
                Self::InstanceInfoUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<InstanceInfoUpdatedFilter> for HyperdriveRegistryEvents {
        fn from(value: InstanceInfoUpdatedFilter) -> Self {
            Self::InstanceInfoUpdatedFilter(value)
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
        Kind(KindCall),
        Name(NameCall),
        SetFactoryInfo(SetFactoryInfoCall),
        SetInstanceInfo(SetInstanceInfoCall),
        UpdateAdmin(UpdateAdminCall),
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
                Self::Kind(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFactoryInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetInstanceInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateAdmin(element) => ::core::fmt::Display::fmt(element, f),
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
