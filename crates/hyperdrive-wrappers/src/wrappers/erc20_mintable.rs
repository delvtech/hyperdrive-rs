pub use erc20_mintable::*;
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
pub mod erc20_mintable {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("symbol"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("decimals"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint8"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("admin"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("isCompetitionMode_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bool"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("maxMintAmount_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
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
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("authority"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("authority"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Authority"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
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
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("destination"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("canCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("canCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("functionSig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("doesRoleHaveCapability"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "doesRoleHaveCapability",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("functionSig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                    ::std::borrow::ToOwned::to_owned("doesUserHaveRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("doesUserHaveRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("getRolesWithCapability"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getRolesWithCapability",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("getTargetCustomAuthority"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTargetCustomAuthority",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Authority"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUserRoles"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUserRoles"),
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
                    ::std::borrow::ToOwned::to_owned("isCapabilityPublic"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isCapabilityPublic"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                    ::std::borrow::ToOwned::to_owned("isCompetitionMode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isCompetitionMode"),
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
                    ::std::borrow::ToOwned::to_owned("isUnrestricted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isUnrestricted"),
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
                    ::std::borrow::ToOwned::to_owned("maxMintAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxMintAmount"),
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
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("destination"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
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
                    ::std::borrow::ToOwned::to_owned("setAuthority"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAuthority"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAuthority"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Authority"),
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
                    ::std::borrow::ToOwned::to_owned("setMaxMintAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMaxMintAmount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxMintAmount"),
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
                    ::std::borrow::ToOwned::to_owned("setPublicCapability"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setPublicCapability",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("functionSig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("setRoleCapability"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setRoleCapability"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("functionSig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("setTargetCustomAuthority"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setTargetCustomAuthority",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("customAuthority"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Authority"),
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
                    ::std::borrow::ToOwned::to_owned("setUnrestrictedMintStatus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setUnrestrictedMintStatus",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("setUserRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUserRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
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
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
            ]),
            events: ::core::convert::From::from([
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("AuthorityUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AuthorityUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAuthority"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PublicCapabilityUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PublicCapabilityUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("functionSig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleCapabilityUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RoleCapabilityUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("functionSig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TargetCustomAuthorityUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TargetCustomAuthorityUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("authority"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("UserRoleUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("UserRoleUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ERC20MINTABLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0 58\x03\x80b\0 5\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x02\x98V[\x820\x81\x81\x89\x89\x89`\0b\0\0J\x84\x82b\0\x03\xE8V[P`\x01b\0\0Y\x83\x82b\0\x03\xE8V[P`\xFF\x81\x16`\x80RF`\xA0Rb\0\0ob\0\x01\x1EV[`\xC0RPP`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x81\x17\x90\x93U`\x07\x80T\x91\x86\x16\x91\x90\x92\x16\x17\x90U`@Q\x90\x91P3\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x903\x90\x7F\xA39o\xD7\xF6\xE0\xA2\x1BP\xE5\x08\x9D-\xA7\rZ\xC0\xA3\xBB\xBD\x1Faz\x93\xF14\xB7c\x89\x98\x01\x98\x90`\0\x90\xA3PPPP\x90\x15\x15`\xE0R`\x0CUPb\0\x052\x92PPPV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qb\0\x01R\x91\x90b\0\x04\xB4V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12b\0\x01\xE2W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x01\xFFWb\0\x01\xFFb\0\x01\xBAV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x02*Wb\0\x02*b\0\x01\xBAV[\x81`@R\x83\x81R` \x92P\x86` \x85\x88\x01\x01\x11\x15b\0\x02HW`\0\x80\xFD[`\0\x91P[\x83\x82\x10\x15b\0\x02lW\x85\x82\x01\x83\x01Q\x81\x83\x01\x84\x01R\x90\x82\x01\x90b\0\x02MV[`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x80Q\x80\x15\x15\x81\x14b\0\x02\x93W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15b\0\x02\xB2W`\0\x80\xFD[\x86Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x02\xCAW`\0\x80\xFD[b\0\x02\xD8\x8A\x83\x8B\x01b\0\x01\xD0V[\x97P` \x89\x01Q\x91P\x80\x82\x11\x15b\0\x02\xEFW`\0\x80\xFD[Pb\0\x02\xFE\x89\x82\x8A\x01b\0\x01\xD0V[\x95PP`@\x87\x01Q`\xFF\x81\x16\x81\x14b\0\x03\x16W`\0\x80\xFD[``\x88\x01Q\x90\x94P`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x034W`\0\x80\xFD[\x92Pb\0\x03D`\x80\x88\x01b\0\x02\x82V[\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x03lW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x03\x8DWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x03\xE3W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x03\xBEWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x03\xDFW\x82\x81U`\x01\x01b\0\x03\xCAV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x04\x04Wb\0\x04\x04b\0\x01\xBAV[b\0\x04\x1C\x81b\0\x04\x15\x84Tb\0\x03WV[\x84b\0\x03\x93V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x04TW`\0\x84\x15b\0\x04;WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x03\xDFV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x04\x85W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x04dV[P\x85\x82\x10\x15b\0\x04\xA4W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x80\x83Tb\0\x04\xC4\x81b\0\x03WV[`\x01\x82\x81\x16\x80\x15b\0\x04\xDFW`\x01\x81\x14b\0\x04\xF5Wb\0\x05&V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pb\0\x05&V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15b\0\x05\x1DW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01b\0\x05\x02V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x1A\x9Fb\0\x05\x96`\09`\0\x81\x81a\x03\xD4\x01R\x81\x81a\x05\xE7\x01R\x81\x81a\x07\x01\x01R\x81\x81a\t\xE0\x01R\x81\x81a\n\x7F\x01R\x81\x81a\r\xBE\x01Ra\x0E \x01R`\0a\t\xBC\x01R`\0a\t\x87\x01R`\0a\x03\x14\x01Ra\x1A\x9F`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x1CW`\x005`\xE0\x1C\x80cz\x8Cc\xB5\x11a\x01%W\x80c\xB7\0\x96\x13\x11a\0\xADW\x80c\xDDb\xED>\x11a\0|W\x80c\xDDb\xED>\x14a\x05\x1AW\x80c\xE6\x88t{\x14a\x05EW\x80c\xEA|\xA2v\x14a\x05{W\x80c\xED\r\x0E\xFB\x14a\x05\xB2W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xD2W`\0\x80\xFD[\x80c\xB7\0\x96\x13\x14a\x04\xB8W\x80c\xBF~!O\x14a\x04\xCBW\x80c\xC5:9\x85\x14a\x04\xDEW\x80c\xD5\x05\xAC\xCF\x14a\x05\x07W`\0\x80\xFD[\x80c\x95\xD8\x9BA\x11a\0\xF4W\x80c\x95\xD8\x9BA\x14a\x04TW\x80c\x9D\xC2\x9F\xAC\x14a\x04\\W\x80c\xA0q-h\x14a\x04oW\x80c\xA9\x05\x9C\xBB\x14a\x04\x82W\x80c\xAE\xD3\x07w\x14a\x04\x95W`\0\x80\xFD[\x80cz\x8Cc\xB5\x14a\x03\xCFW\x80cz\x9E^K\x14a\x03\xF6W\x80c~\xCE\xBE\0\x14a\x04\tW\x80c\x8D\xA5\xCB[\x14a\x04)W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x01\xA8W\x80cB\x96lh\x11a\x01wW\x80cB\x96lh\x14a\x03cW\x80cKQY\xDA\x14a\x03vW\x80cg\xAF\xF4\x84\x14a\x03\x89W\x80cp\xA0\x821\x14a\x03\x9CW\x80cr\x8B\x95+\x14a\x03\xBCW`\0\x80\xFD[\x80c#\xB8r\xDD\x14a\x02\xFCW\x80c1<\xE5g\x14a\x03\x0FW\x80c6D\xE5\x15\x14a\x03HW\x80c@\xC1\x0F\x19\x14a\x03PW`\0\x80\xFD[\x80c\t^\xA7\xB3\x11a\x01\xEFW\x80c\t^\xA7\xB3\x14a\x02\x91W\x80c\x0B\xAD\xE8\xA4\x14a\x02\xB4W\x80c\x0E\xA9\xB7[\x14a\x02\xD7W\x80c\x18\x16\r\xDD\x14a\x02\xEAW\x80c#\x9Cp\xAE\x14a\x02\xF3W`\0\x80\xFD[\x80c\x05\xF0Z\x94\x14a\x02!W\x80c\x06\xA3j\xEE\x14a\x026W\x80c\x06\xFD\xDE\x03\x14a\x02iW\x80c\x08\x8AN\xD0\x14a\x02~W[`\0\x80\xFD[a\x024a\x02/6`\x04a\x15\x02V[a\x05\xE5V[\0[a\x02Va\x02D6`\x04a\x15;V[`\t` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02qa\x06qV[`@Qa\x02`\x91\x90a\x15XV[a\x024a\x02\x8C6`\x04a\x15\xA7V[a\x06\xFFV[a\x02\xA4a\x02\x9F6`\x04a\x15\xC0V[a\x07\\V[`@Q\x90\x15\x15\x81R` \x01a\x02`V[a\x02\xA4a\x02\xC26`\x04a\x16\tV[`\n` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x024a\x02\xE56`\x04a\x165V[a\x07\xC9V[a\x02V`\x02T\x81V[a\x02V`\x0CT\x81V[a\x02\xA4a\x03\n6`\x04a\x16|V[a\x08\xA1V[a\x036\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02`V[a\x02Va\t\x83V[a\x024a\x03^6`\x04a\x15\xC0V[a\t\xDEV[a\x024a\x03q6`\x04a\x15\xA7V[a\n}V[a\x024a\x03\x846`\x04a\x16\xBDV[a\n\xE2V[a\x024a\x03\x976`\x04a\x16\xD9V[a\x0BtV[a\x02Va\x03\xAA6`\x04a\x15;V[`\x03` R`\0\x90\x81R`@\x90 T\x81V[a\x024a\x03\xCA6`\x04a\x17\x07V[a\x0C<V[a\x02\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x024a\x04\x046`\x04a\x15;V[a\x0C\xC5V[a\x02Va\x04\x176`\x04a\x15;V[`\x05` R`\0\x90\x81R`@\x90 T\x81V[`\x06Ta\x04<\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02`V[a\x02qa\r\xAFV[a\x024a\x04j6`\x04a\x15\xC0V[a\r\xBCV[a\x024a\x04}6`\x04a\x15\xA7V[a\x0E\x1EV[a\x02\xA4a\x04\x906`\x04a\x15\xC0V[a\x0E\xB9V[a\x02\xA4a\x04\xA36`\x04a\x15;V[`\r` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xA4a\x04\xC66`\x04a\x175V[a\x0F\x1FV[`\x07Ta\x04<\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04<a\x04\xEC6`\x04a\x15;V[`\x08` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x024a\x05\x156`\x04a\x17|V[a\x10\x1DV[a\x02Va\x05(6`\x04a\x17\x07V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x02\xA4a\x05S6`\x04a\x17\xEAV[`\x01`\x01`\xE0\x1B\x03\x19\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x91\x90\x91\x16\x1C`\x01\x16\x15\x15\x90V[a\x02\xA4a\x05\x896`\x04a\x18\x1DV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\t` R`@\x90 T`\x01`\xFF\x90\x92\x16\x1C\x16\x15\x15\x90V[a\x02Va\x05\xC06`\x04a\x16\tV[`\x0B` R`\0\x90\x81R`@\x90 T\x81V[a\x024a\x05\xE06`\x04a\x15;V[a\x12aV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x06FWa\x06!3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x06FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18IV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\r` R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\0\x80Ta\x06~\x90a\x18\x80V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xAA\x90a\x18\x80V[\x80\x15a\x06\xF7W\x80`\x1F\x10a\x06\xCCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xF7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xDAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x07WWa\x07;3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x07WW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18IV[`\x0CUV[3`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x85\x90UQ\x91\x92\x90\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90a\x07\xB7\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[a\x07\xDF3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x07\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18\xBAV[\x80\x15a\x08+W`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x90\x81R`\x0B` R`@\x90 \x80T`\x01`\xFF\x86\x16\x1B\x17\x90Ua\x08RV[`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x90\x81R`\x0B` R`@\x90 \x80T`\x01`\xFF\x86\x16\x1B\x19\x16\x90U[\x81`\x01`\x01`\xE0\x1B\x03\x19\x16\x83`\xFF\x16\x7F\xBF\xE1k,5\xCE#\xDF\xD1\xAB\x0E{]\x08j\x10\x06\x0C\x9BR\xD1WN\x16\x80\xC8\x81\xB3\xB3\xA2\xB1Q\x83`@Qa\x08\x94\x91\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x81\x14a\x08\xFDWa\x08\xD8\x83\x82a\x18\xF6V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x85\x92\x90a\t%\x90\x84\x90a\x18\xF6V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x87\x01\x90UQ\x90\x91\x87\x16\x90`\0\x80Q` a\x1AJ\x839\x81Q\x91R\x90a\tn\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3`\x01\x91PP[\x93\x92PPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\t\xB9Wa\t\xB4a\x13\x89V[\x90P\x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\n6Wa\n\x1A3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\n6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18IV[3`\0\x90\x81R`\r` R`@\x90 T`\xFF\x16a\noW`\x0CT\x81\x11\x15a\noW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x19\tV[a\ny\x82\x82a\x14#V[PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\n\xD5Wa\n\xB93`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\n\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18IV[a\n\xDF3\x82a\x14}V[PV[a\n\xF83`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x0B\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18\xBAV[`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x81\x81R`\n` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F6\xD2\x81&\xBE\xF2\x1AO7e\xD7\xFC\xB7\xC4\\\xEA\xD4c\xAELA\tN\xF3\xB7q\xED\xE5\x98TA\x03\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[a\x0B\x8A3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x0B\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18\xBAV[\x80\x15a\x0B\xD5W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\t` R`@\x90 \x80T`\x01`\xFF\x85\x16\x1B\x17\x90Ua\x0B\xFBV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\t` R`@\x90 \x80T`\x01`\xFF\x85\x16\x1B\x19\x16\x90U[\x81`\xFF\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7FL\x9B\xDD\x0C\x8E\x07>\xB5\xED\xA2%\x0B\x18\xD8\xE5\x12\x1F\xF2{b\x06O\xBE\xEE\xEDHi\xBB\x99\xBC[\xF2\x83`@Qa\x08\x94\x91\x15\x15\x81R` \x01\x90V[a\x0CR3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x0CnW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x08` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x86\x16\x94\x85\x17\x90UQ\x7F\xA4\x90\x8E\x11\xA5\xF8\x95\xB1=QRl3\x1A\xC9<\xDD0\xE5\x97r6\x1C]\x07\x87N\xB3k\xFF e\x91\x90\xA3PPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\rZWP`\x07T`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB7\0\x96\x13\x90a\r\x19\x903\x900\x90`\x01`\x01`\xE0\x1B\x03\x19`\x005\x16\x90`\x04\x01a\x19KV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rZ\x91\x90a\x19xV[a\rcW`\0\x80\xFD[`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90\x7F\xA39o\xD7\xF6\xE0\xA2\x1BP\xE5\x08\x9D-\xA7\rZ\xC0\xA3\xBB\xBD\x1Faz\x93\xF14\xB7c\x89\x98\x01\x98\x90`\0\x90\xA3PV[`\x01\x80Ta\x06~\x90a\x18\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0E\x14Wa\r\xF83`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x0E\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18IV[a\ny\x82\x82a\x14}V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0EvWa\x0EZ3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x0EvW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18IV[3`\0\x90\x81R`\r` R`@\x90 T`\xFF\x16a\x0E\xAFW`\x0CT\x81\x11\x15a\x0E\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x19\tV[a\n\xDF3\x82a\x14#V[3`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x91\x90\x83\x90a\x0E\xDA\x90\x84\x90a\x18\xF6V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x85\x01\x90UQ3\x90`\0\x80Q` a\x1AJ\x839\x81Q\x91R\x90a\x07\xB7\x90\x86\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x08` R`@\x81 T\x90\x91\x16\x80\x15a\x0F\xBBW`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB7\0\x96\x13\x90a\x0Fr\x90\x88\x90\x88\x90\x88\x90`\x04\x01a\x19KV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x8FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xB3\x91\x90a\x19xV[\x91PPa\t|V[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16\x80a\x10\x14WP`\x01`\x01`\xE0\x1B\x03\x19\x83\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 T`\x01`\x01`\xA0\x1B\x03\x89\x16\x84R`\t\x90\x92R\x90\x91 T\x16\x15\x15[\x95\x94PPPPPV[B\x84\x10\x15a\x10mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06=V[`\0`\x01a\x10ya\t\x83V[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x80T`\x01\x81\x01\x90\x91U\x82Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81\x84\x01R\x80\x84\x01\x94\x90\x94R\x93\x8D\x16``\x84\x01R`\x80\x83\x01\x8C\x90R`\xA0\x83\x01\x93\x90\x93R`\xC0\x80\x83\x01\x8B\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x83\x01\x90\x91R\x80Q\x92\x01\x91\x90\x91 a\x19\x01`\xF0\x1Ba\x01\0\x83\x01Ra\x01\x02\x82\x01\x92\x90\x92Ra\x01\"\x81\x01\x91\x90\x91Ra\x01B\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83RR`\xFF\x87\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x11\x85W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x11\xBBWP\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x11\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x01a\x06=V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x8A\x85\x16\x80\x85R\x90\x83R\x92\x81\x90 \x89\x90UQ\x88\x81R\x91\x92\x8A\x16\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPV[a\x12w3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x12\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18\xBAV[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PV[`\x07T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x80\x15\x90a\x13iWP`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB7\0\x96\x13\x90a\x13(\x90\x87\x900\x90\x88\x90`\x04\x01a\x19KV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13i\x91\x90a\x19xV[\x80a\x13\x81WP`\x06T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x91\x16\x14[\x94\x93PPPPV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa\x13\xBB\x91\x90a\x19\x95V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x80`\x02`\0\x82\x82Ta\x145\x91\x90a\x1A6V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R`\0\x80Q` a\x1AJ\x839\x81Q\x91R\x91\x01[`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x92\x90a\x14\xA5\x90\x84\x90a\x18\xF6V[\x90\x91UPP`\x02\x80T\x82\x90\x03\x90U`@Q\x81\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90`\0\x80Q` a\x1AJ\x839\x81Q\x91R\x90` \x01a\x14qV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xDFW`\0\x80\xFD[\x80\x15\x15\x81\x14a\n\xDFW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x15\x15W`\0\x80\xFD[\x825a\x15 \x81a\x14\xDFV[\x91P` \x83\x015a\x150\x81a\x14\xF4V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x15MW`\0\x80\xFD[\x815a\t|\x81a\x14\xDFV[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x15\x86W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x15jV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x15\xB9W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\xD3W`\0\x80\xFD[\x825a\x15\xDE\x81a\x14\xDFV[\x94` \x93\x90\x93\x015\x93PPPV[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x16\x04W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x16\x1BW`\0\x80\xFD[a\t|\x82a\x15\xECV[\x805`\xFF\x81\x16\x81\x14a\x16\x04W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16JW`\0\x80\xFD[a\x16S\x84a\x16$V[\x92Pa\x16a` \x85\x01a\x15\xECV[\x91P`@\x84\x015a\x16q\x81a\x14\xF4V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16\x91W`\0\x80\xFD[\x835a\x16\x9C\x81a\x14\xDFV[\x92P` \x84\x015a\x16\xAC\x81a\x14\xDFV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x16\xD0W`\0\x80\xFD[a\x15 \x83a\x15\xECV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16\xEEW`\0\x80\xFD[\x835a\x16\xF9\x81a\x14\xDFV[\x92Pa\x16a` \x85\x01a\x16$V[`\0\x80`@\x83\x85\x03\x12\x15a\x17\x1AW`\0\x80\xFD[\x825a\x17%\x81a\x14\xDFV[\x91P` \x83\x015a\x150\x81a\x14\xDFV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x17JW`\0\x80\xFD[\x835a\x17U\x81a\x14\xDFV[\x92P` \x84\x015a\x17e\x81a\x14\xDFV[\x91Pa\x17s`@\x85\x01a\x15\xECV[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x17\x97W`\0\x80\xFD[\x875a\x17\xA2\x81a\x14\xDFV[\x96P` \x88\x015a\x17\xB2\x81a\x14\xDFV[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa\x17\xCE`\x80\x89\x01a\x16$V[\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`@\x83\x85\x03\x12\x15a\x17\xFDW`\0\x80\xFD[a\x18\x06\x83a\x16$V[\x91Pa\x18\x14` \x84\x01a\x15\xECV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x180W`\0\x80\xFD[\x825a\x18;\x81a\x14\xDFV[\x91Pa\x18\x14` \x84\x01a\x16$V[` \x80\x82R`\x1D\x90\x82\x01R\x7FERC20Mintable: not authorized\0\0\0`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x18\x94W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x18\xB4WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07\xC3Wa\x07\xC3a\x18\xE0V[` \x80\x82R`\"\x90\x82\x01R\x7FERC20Mintable: Invalid mint amou`@\x82\x01Ra\x1B\x9D`\xF2\x1B``\x82\x01R`\x80\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x19\x8AW`\0\x80\xFD[\x81Qa\t|\x81a\x14\xF4V[`\0\x80\x83T\x81`\x01\x82`\x01\x1C\x91P`\x01\x83\x16\x80a\x19\xB3W`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03a\x19\xD2WcNH{q`\xE0\x1B\x86R`\"`\x04R`$\x86\xFD[\x81\x80\x15a\x19\xE6W`\x01\x81\x14a\x19\xFBWa\x1A(V[`\xFF\x19\x86\x16\x89R\x84\x15\x15\x85\x02\x89\x01\x96Pa\x1A(V[`\0\x8A\x81R` \x90 `\0[\x86\x81\x10\x15a\x1A W\x81T\x8B\x82\x01R\x90\x85\x01\x90\x83\x01a\x1A\x07V[PP\x84\x89\x01\x96P[P\x94\x98\x97PPPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x07\xC3Wa\x07\xC3a\x18\xE0V\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 S\xCCf\x7F\x1FO \x0Fm\xC5\xA3\xD3\x9C\x1E:\x10+s\xEA\xA1N\xC81\x81\xA1\xC9\x04\x06\x9F\xC3\x1A?dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static ERC20MINTABLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x1CW`\x005`\xE0\x1C\x80cz\x8Cc\xB5\x11a\x01%W\x80c\xB7\0\x96\x13\x11a\0\xADW\x80c\xDDb\xED>\x11a\0|W\x80c\xDDb\xED>\x14a\x05\x1AW\x80c\xE6\x88t{\x14a\x05EW\x80c\xEA|\xA2v\x14a\x05{W\x80c\xED\r\x0E\xFB\x14a\x05\xB2W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xD2W`\0\x80\xFD[\x80c\xB7\0\x96\x13\x14a\x04\xB8W\x80c\xBF~!O\x14a\x04\xCBW\x80c\xC5:9\x85\x14a\x04\xDEW\x80c\xD5\x05\xAC\xCF\x14a\x05\x07W`\0\x80\xFD[\x80c\x95\xD8\x9BA\x11a\0\xF4W\x80c\x95\xD8\x9BA\x14a\x04TW\x80c\x9D\xC2\x9F\xAC\x14a\x04\\W\x80c\xA0q-h\x14a\x04oW\x80c\xA9\x05\x9C\xBB\x14a\x04\x82W\x80c\xAE\xD3\x07w\x14a\x04\x95W`\0\x80\xFD[\x80cz\x8Cc\xB5\x14a\x03\xCFW\x80cz\x9E^K\x14a\x03\xF6W\x80c~\xCE\xBE\0\x14a\x04\tW\x80c\x8D\xA5\xCB[\x14a\x04)W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x01\xA8W\x80cB\x96lh\x11a\x01wW\x80cB\x96lh\x14a\x03cW\x80cKQY\xDA\x14a\x03vW\x80cg\xAF\xF4\x84\x14a\x03\x89W\x80cp\xA0\x821\x14a\x03\x9CW\x80cr\x8B\x95+\x14a\x03\xBCW`\0\x80\xFD[\x80c#\xB8r\xDD\x14a\x02\xFCW\x80c1<\xE5g\x14a\x03\x0FW\x80c6D\xE5\x15\x14a\x03HW\x80c@\xC1\x0F\x19\x14a\x03PW`\0\x80\xFD[\x80c\t^\xA7\xB3\x11a\x01\xEFW\x80c\t^\xA7\xB3\x14a\x02\x91W\x80c\x0B\xAD\xE8\xA4\x14a\x02\xB4W\x80c\x0E\xA9\xB7[\x14a\x02\xD7W\x80c\x18\x16\r\xDD\x14a\x02\xEAW\x80c#\x9Cp\xAE\x14a\x02\xF3W`\0\x80\xFD[\x80c\x05\xF0Z\x94\x14a\x02!W\x80c\x06\xA3j\xEE\x14a\x026W\x80c\x06\xFD\xDE\x03\x14a\x02iW\x80c\x08\x8AN\xD0\x14a\x02~W[`\0\x80\xFD[a\x024a\x02/6`\x04a\x15\x02V[a\x05\xE5V[\0[a\x02Va\x02D6`\x04a\x15;V[`\t` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02qa\x06qV[`@Qa\x02`\x91\x90a\x15XV[a\x024a\x02\x8C6`\x04a\x15\xA7V[a\x06\xFFV[a\x02\xA4a\x02\x9F6`\x04a\x15\xC0V[a\x07\\V[`@Q\x90\x15\x15\x81R` \x01a\x02`V[a\x02\xA4a\x02\xC26`\x04a\x16\tV[`\n` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x024a\x02\xE56`\x04a\x165V[a\x07\xC9V[a\x02V`\x02T\x81V[a\x02V`\x0CT\x81V[a\x02\xA4a\x03\n6`\x04a\x16|V[a\x08\xA1V[a\x036\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02`V[a\x02Va\t\x83V[a\x024a\x03^6`\x04a\x15\xC0V[a\t\xDEV[a\x024a\x03q6`\x04a\x15\xA7V[a\n}V[a\x024a\x03\x846`\x04a\x16\xBDV[a\n\xE2V[a\x024a\x03\x976`\x04a\x16\xD9V[a\x0BtV[a\x02Va\x03\xAA6`\x04a\x15;V[`\x03` R`\0\x90\x81R`@\x90 T\x81V[a\x024a\x03\xCA6`\x04a\x17\x07V[a\x0C<V[a\x02\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x024a\x04\x046`\x04a\x15;V[a\x0C\xC5V[a\x02Va\x04\x176`\x04a\x15;V[`\x05` R`\0\x90\x81R`@\x90 T\x81V[`\x06Ta\x04<\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02`V[a\x02qa\r\xAFV[a\x024a\x04j6`\x04a\x15\xC0V[a\r\xBCV[a\x024a\x04}6`\x04a\x15\xA7V[a\x0E\x1EV[a\x02\xA4a\x04\x906`\x04a\x15\xC0V[a\x0E\xB9V[a\x02\xA4a\x04\xA36`\x04a\x15;V[`\r` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xA4a\x04\xC66`\x04a\x175V[a\x0F\x1FV[`\x07Ta\x04<\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04<a\x04\xEC6`\x04a\x15;V[`\x08` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x024a\x05\x156`\x04a\x17|V[a\x10\x1DV[a\x02Va\x05(6`\x04a\x17\x07V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x02\xA4a\x05S6`\x04a\x17\xEAV[`\x01`\x01`\xE0\x1B\x03\x19\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x91\x90\x91\x16\x1C`\x01\x16\x15\x15\x90V[a\x02\xA4a\x05\x896`\x04a\x18\x1DV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\t` R`@\x90 T`\x01`\xFF\x90\x92\x16\x1C\x16\x15\x15\x90V[a\x02Va\x05\xC06`\x04a\x16\tV[`\x0B` R`\0\x90\x81R`@\x90 T\x81V[a\x024a\x05\xE06`\x04a\x15;V[a\x12aV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x06FWa\x06!3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x06FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18IV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\r` R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\0\x80Ta\x06~\x90a\x18\x80V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xAA\x90a\x18\x80V[\x80\x15a\x06\xF7W\x80`\x1F\x10a\x06\xCCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xF7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xDAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x07WWa\x07;3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x07WW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18IV[`\x0CUV[3`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x85\x90UQ\x91\x92\x90\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90a\x07\xB7\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[a\x07\xDF3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x07\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18\xBAV[\x80\x15a\x08+W`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x90\x81R`\x0B` R`@\x90 \x80T`\x01`\xFF\x86\x16\x1B\x17\x90Ua\x08RV[`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x90\x81R`\x0B` R`@\x90 \x80T`\x01`\xFF\x86\x16\x1B\x19\x16\x90U[\x81`\x01`\x01`\xE0\x1B\x03\x19\x16\x83`\xFF\x16\x7F\xBF\xE1k,5\xCE#\xDF\xD1\xAB\x0E{]\x08j\x10\x06\x0C\x9BR\xD1WN\x16\x80\xC8\x81\xB3\xB3\xA2\xB1Q\x83`@Qa\x08\x94\x91\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x81\x14a\x08\xFDWa\x08\xD8\x83\x82a\x18\xF6V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x85\x92\x90a\t%\x90\x84\x90a\x18\xF6V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x87\x01\x90UQ\x90\x91\x87\x16\x90`\0\x80Q` a\x1AJ\x839\x81Q\x91R\x90a\tn\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3`\x01\x91PP[\x93\x92PPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\t\xB9Wa\t\xB4a\x13\x89V[\x90P\x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\n6Wa\n\x1A3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\n6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18IV[3`\0\x90\x81R`\r` R`@\x90 T`\xFF\x16a\noW`\x0CT\x81\x11\x15a\noW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x19\tV[a\ny\x82\x82a\x14#V[PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\n\xD5Wa\n\xB93`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\n\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18IV[a\n\xDF3\x82a\x14}V[PV[a\n\xF83`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x0B\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18\xBAV[`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x81\x81R`\n` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F6\xD2\x81&\xBE\xF2\x1AO7e\xD7\xFC\xB7\xC4\\\xEA\xD4c\xAELA\tN\xF3\xB7q\xED\xE5\x98TA\x03\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[a\x0B\x8A3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x0B\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18\xBAV[\x80\x15a\x0B\xD5W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\t` R`@\x90 \x80T`\x01`\xFF\x85\x16\x1B\x17\x90Ua\x0B\xFBV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\t` R`@\x90 \x80T`\x01`\xFF\x85\x16\x1B\x19\x16\x90U[\x81`\xFF\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7FL\x9B\xDD\x0C\x8E\x07>\xB5\xED\xA2%\x0B\x18\xD8\xE5\x12\x1F\xF2{b\x06O\xBE\xEE\xEDHi\xBB\x99\xBC[\xF2\x83`@Qa\x08\x94\x91\x15\x15\x81R` \x01\x90V[a\x0CR3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x0CnW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x08` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x86\x16\x94\x85\x17\x90UQ\x7F\xA4\x90\x8E\x11\xA5\xF8\x95\xB1=QRl3\x1A\xC9<\xDD0\xE5\x97r6\x1C]\x07\x87N\xB3k\xFF e\x91\x90\xA3PPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\rZWP`\x07T`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB7\0\x96\x13\x90a\r\x19\x903\x900\x90`\x01`\x01`\xE0\x1B\x03\x19`\x005\x16\x90`\x04\x01a\x19KV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rZ\x91\x90a\x19xV[a\rcW`\0\x80\xFD[`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90\x7F\xA39o\xD7\xF6\xE0\xA2\x1BP\xE5\x08\x9D-\xA7\rZ\xC0\xA3\xBB\xBD\x1Faz\x93\xF14\xB7c\x89\x98\x01\x98\x90`\0\x90\xA3PV[`\x01\x80Ta\x06~\x90a\x18\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0E\x14Wa\r\xF83`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x0E\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18IV[a\ny\x82\x82a\x14}V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0EvWa\x0EZ3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x0EvW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18IV[3`\0\x90\x81R`\r` R`@\x90 T`\xFF\x16a\x0E\xAFW`\x0CT\x81\x11\x15a\x0E\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x19\tV[a\n\xDF3\x82a\x14#V[3`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x91\x90\x83\x90a\x0E\xDA\x90\x84\x90a\x18\xF6V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x85\x01\x90UQ3\x90`\0\x80Q` a\x1AJ\x839\x81Q\x91R\x90a\x07\xB7\x90\x86\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x08` R`@\x81 T\x90\x91\x16\x80\x15a\x0F\xBBW`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB7\0\x96\x13\x90a\x0Fr\x90\x88\x90\x88\x90\x88\x90`\x04\x01a\x19KV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x8FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xB3\x91\x90a\x19xV[\x91PPa\t|V[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16\x80a\x10\x14WP`\x01`\x01`\xE0\x1B\x03\x19\x83\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 T`\x01`\x01`\xA0\x1B\x03\x89\x16\x84R`\t\x90\x92R\x90\x91 T\x16\x15\x15[\x95\x94PPPPPV[B\x84\x10\x15a\x10mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06=V[`\0`\x01a\x10ya\t\x83V[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x80T`\x01\x81\x01\x90\x91U\x82Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81\x84\x01R\x80\x84\x01\x94\x90\x94R\x93\x8D\x16``\x84\x01R`\x80\x83\x01\x8C\x90R`\xA0\x83\x01\x93\x90\x93R`\xC0\x80\x83\x01\x8B\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x83\x01\x90\x91R\x80Q\x92\x01\x91\x90\x91 a\x19\x01`\xF0\x1Ba\x01\0\x83\x01Ra\x01\x02\x82\x01\x92\x90\x92Ra\x01\"\x81\x01\x91\x90\x91Ra\x01B\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83RR`\xFF\x87\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x11\x85W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x11\xBBWP\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x11\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x01a\x06=V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x8A\x85\x16\x80\x85R\x90\x83R\x92\x81\x90 \x89\x90UQ\x88\x81R\x91\x92\x8A\x16\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPV[a\x12w3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x12\xDFV[a\x12\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06=\x90a\x18\xBAV[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PV[`\x07T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x80\x15\x90a\x13iWP`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB7\0\x96\x13\x90a\x13(\x90\x87\x900\x90\x88\x90`\x04\x01a\x19KV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13i\x91\x90a\x19xV[\x80a\x13\x81WP`\x06T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x91\x16\x14[\x94\x93PPPPV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa\x13\xBB\x91\x90a\x19\x95V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x80`\x02`\0\x82\x82Ta\x145\x91\x90a\x1A6V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R`\0\x80Q` a\x1AJ\x839\x81Q\x91R\x91\x01[`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x92\x90a\x14\xA5\x90\x84\x90a\x18\xF6V[\x90\x91UPP`\x02\x80T\x82\x90\x03\x90U`@Q\x81\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90`\0\x80Q` a\x1AJ\x839\x81Q\x91R\x90` \x01a\x14qV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xDFW`\0\x80\xFD[\x80\x15\x15\x81\x14a\n\xDFW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x15\x15W`\0\x80\xFD[\x825a\x15 \x81a\x14\xDFV[\x91P` \x83\x015a\x150\x81a\x14\xF4V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x15MW`\0\x80\xFD[\x815a\t|\x81a\x14\xDFV[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x15\x86W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x15jV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x15\xB9W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\xD3W`\0\x80\xFD[\x825a\x15\xDE\x81a\x14\xDFV[\x94` \x93\x90\x93\x015\x93PPPV[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x16\x04W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x16\x1BW`\0\x80\xFD[a\t|\x82a\x15\xECV[\x805`\xFF\x81\x16\x81\x14a\x16\x04W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16JW`\0\x80\xFD[a\x16S\x84a\x16$V[\x92Pa\x16a` \x85\x01a\x15\xECV[\x91P`@\x84\x015a\x16q\x81a\x14\xF4V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16\x91W`\0\x80\xFD[\x835a\x16\x9C\x81a\x14\xDFV[\x92P` \x84\x015a\x16\xAC\x81a\x14\xDFV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x16\xD0W`\0\x80\xFD[a\x15 \x83a\x15\xECV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16\xEEW`\0\x80\xFD[\x835a\x16\xF9\x81a\x14\xDFV[\x92Pa\x16a` \x85\x01a\x16$V[`\0\x80`@\x83\x85\x03\x12\x15a\x17\x1AW`\0\x80\xFD[\x825a\x17%\x81a\x14\xDFV[\x91P` \x83\x015a\x150\x81a\x14\xDFV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x17JW`\0\x80\xFD[\x835a\x17U\x81a\x14\xDFV[\x92P` \x84\x015a\x17e\x81a\x14\xDFV[\x91Pa\x17s`@\x85\x01a\x15\xECV[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x17\x97W`\0\x80\xFD[\x875a\x17\xA2\x81a\x14\xDFV[\x96P` \x88\x015a\x17\xB2\x81a\x14\xDFV[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa\x17\xCE`\x80\x89\x01a\x16$V[\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`@\x83\x85\x03\x12\x15a\x17\xFDW`\0\x80\xFD[a\x18\x06\x83a\x16$V[\x91Pa\x18\x14` \x84\x01a\x15\xECV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x180W`\0\x80\xFD[\x825a\x18;\x81a\x14\xDFV[\x91Pa\x18\x14` \x84\x01a\x16$V[` \x80\x82R`\x1D\x90\x82\x01R\x7FERC20Mintable: not authorized\0\0\0`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x18\x94W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x18\xB4WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07\xC3Wa\x07\xC3a\x18\xE0V[` \x80\x82R`\"\x90\x82\x01R\x7FERC20Mintable: Invalid mint amou`@\x82\x01Ra\x1B\x9D`\xF2\x1B``\x82\x01R`\x80\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x19\x8AW`\0\x80\xFD[\x81Qa\t|\x81a\x14\xF4V[`\0\x80\x83T\x81`\x01\x82`\x01\x1C\x91P`\x01\x83\x16\x80a\x19\xB3W`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03a\x19\xD2WcNH{q`\xE0\x1B\x86R`\"`\x04R`$\x86\xFD[\x81\x80\x15a\x19\xE6W`\x01\x81\x14a\x19\xFBWa\x1A(V[`\xFF\x19\x86\x16\x89R\x84\x15\x15\x85\x02\x89\x01\x96Pa\x1A(V[`\0\x8A\x81R` \x90 `\0[\x86\x81\x10\x15a\x1A W\x81T\x8B\x82\x01R\x90\x85\x01\x90\x83\x01a\x1A\x07V[PP\x84\x89\x01\x96P[P\x94\x98\x97PPPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x07\xC3Wa\x07\xC3a\x18\xE0V\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 S\xCCf\x7F\x1FO \x0Fm\xC5\xA3\xD3\x9C\x1E:\x10+s\xEA\xA1N\xC81\x81\xA1\xC9\x04\x06\x9F\xC3\x1A?dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static ERC20MINTABLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ERC20Mintable<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC20Mintable<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC20Mintable<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC20Mintable<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC20Mintable<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ERC20Mintable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC20Mintable<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC20MINTABLE_ABI.clone(),
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
                ERC20MINTABLE_ABI.clone(),
                ERC20MINTABLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `authority` (0xbf7e214f) function
        pub fn authority(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([191, 126, 33, 79], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x42966c68) function
        pub fn burn(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x9dc29fac) function
        pub fn burn_with_destination(
            &self,
            destination: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 194, 159, 172], (destination, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `canCall` (0xb7009613) function
        pub fn can_call(
            &self,
            user: ::ethers::core::types::Address,
            target: ::ethers::core::types::Address,
            function_sig: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([183, 0, 150, 19], (user, target, function_sig))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `doesRoleHaveCapability` (0xe688747b) function
        pub fn does_role_have_capability(
            &self,
            role: u8,
            function_sig: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([230, 136, 116, 123], (role, function_sig))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `doesUserHaveRole` (0xea7ca276) function
        pub fn does_user_have_role(
            &self,
            user: ::ethers::core::types::Address,
            role: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([234, 124, 162, 118], (user, role))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRolesWithCapability` (0xed0d0efb) function
        pub fn get_roles_with_capability(
            &self,
            p0: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([237, 13, 14, 251], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTargetCustomAuthority` (0xc53a3985) function
        pub fn get_target_custom_authority(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([197, 58, 57, 133], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserRoles` (0x06a36aee) function
        pub fn get_user_roles(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([6, 163, 106, 238], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isCapabilityPublic` (0x0bade8a4) function
        pub fn is_capability_public(
            &self,
            p0: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([11, 173, 232, 164], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isCompetitionMode` (0x7a8c63b5) function
        pub fn is_competition_mode(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([122, 140, 99, 181], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isUnrestricted` (0xaed30777) function
        pub fn is_unrestricted(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([174, 211, 7, 119], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxMintAmount` (0x239c70ae) function
        pub fn max_mint_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([35, 156, 112, 174], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x40c10f19) function
        pub fn mint_with_destination(
            &self,
            destination: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (destination, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0xa0712d68) function
        pub fn mint(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 113, 45, 104], amount)
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
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAuthority` (0x7a9e5e4b) function
        pub fn set_authority(
            &self,
            new_authority: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 158, 94, 75], new_authority)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMaxMintAmount` (0x088a4ed0) function
        pub fn set_max_mint_amount(
            &self,
            max_mint_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([8, 138, 78, 208], max_mint_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPublicCapability` (0x4b5159da) function
        pub fn set_public_capability(
            &self,
            function_sig: [u8; 4],
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 81, 89, 218], (function_sig, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRoleCapability` (0x0ea9b75b) function
        pub fn set_role_capability(
            &self,
            role: u8,
            function_sig: [u8; 4],
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 169, 183, 91], (role, function_sig, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTargetCustomAuthority` (0x728b952b) function
        pub fn set_target_custom_authority(
            &self,
            target: ::ethers::core::types::Address,
            custom_authority: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 139, 149, 43], (target, custom_authority))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUnrestrictedMintStatus` (0x05f05a94) function
        pub fn set_unrestricted_mint_status(
            &self,
            target: ::ethers::core::types::Address,
            status: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 240, 90, 148], (target, status))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUserRole` (0x67aff484) function
        pub fn set_user_role(
            &self,
            user: ::ethers::core::types::Address,
            role: u8,
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 175, 244, 132], (user, role, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
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
        ///Gets the contract's `AuthorityUpdated` event
        pub fn authority_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AuthorityUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PublicCapabilityUpdated` event
        pub fn public_capability_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PublicCapabilityUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleCapabilityUpdated` event
        pub fn role_capability_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleCapabilityUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TargetCustomAuthorityUpdated` event
        pub fn target_custom_authority_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TargetCustomAuthorityUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `UserRoleUpdated` event
        pub fn user_role_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UserRoleUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ERC20MintableEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC20Mintable<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "AuthorityUpdated", abi = "AuthorityUpdated(address,address)")]
    pub struct AuthorityUpdatedFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_authority: ::ethers::core::types::Address,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
        name = "PublicCapabilityUpdated",
        abi = "PublicCapabilityUpdated(bytes4,bool)"
    )]
    pub struct PublicCapabilityUpdatedFilter {
        #[ethevent(indexed)]
        pub function_sig: [u8; 4],
        pub enabled: bool,
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
        name = "RoleCapabilityUpdated",
        abi = "RoleCapabilityUpdated(uint8,bytes4,bool)"
    )]
    pub struct RoleCapabilityUpdatedFilter {
        #[ethevent(indexed)]
        pub role: u8,
        #[ethevent(indexed)]
        pub function_sig: [u8; 4],
        pub enabled: bool,
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
        name = "TargetCustomAuthorityUpdated",
        abi = "TargetCustomAuthorityUpdated(address,address)"
    )]
    pub struct TargetCustomAuthorityUpdatedFilter {
        #[ethevent(indexed)]
        pub target: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub authority: ::ethers::core::types::Address,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "UserRoleUpdated", abi = "UserRoleUpdated(address,uint8,bool)")]
    pub struct UserRoleUpdatedFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub role: u8,
        pub enabled: bool,
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
    pub enum ERC20MintableEvents {
        ApprovalFilter(ApprovalFilter),
        AuthorityUpdatedFilter(AuthorityUpdatedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PublicCapabilityUpdatedFilter(PublicCapabilityUpdatedFilter),
        RoleCapabilityUpdatedFilter(RoleCapabilityUpdatedFilter),
        TargetCustomAuthorityUpdatedFilter(TargetCustomAuthorityUpdatedFilter),
        TransferFilter(TransferFilter),
        UserRoleUpdatedFilter(UserRoleUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ERC20MintableEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC20MintableEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = AuthorityUpdatedFilter::decode_log(log) {
                return Ok(ERC20MintableEvents::AuthorityUpdatedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ERC20MintableEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PublicCapabilityUpdatedFilter::decode_log(log) {
                return Ok(ERC20MintableEvents::PublicCapabilityUpdatedFilter(decoded));
            }
            if let Ok(decoded) = RoleCapabilityUpdatedFilter::decode_log(log) {
                return Ok(ERC20MintableEvents::RoleCapabilityUpdatedFilter(decoded));
            }
            if let Ok(decoded) = TargetCustomAuthorityUpdatedFilter::decode_log(log) {
                return Ok(
                    ERC20MintableEvents::TargetCustomAuthorityUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ERC20MintableEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = UserRoleUpdatedFilter::decode_log(log) {
                return Ok(ERC20MintableEvents::UserRoleUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ERC20MintableEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuthorityUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PublicCapabilityUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleCapabilityUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TargetCustomAuthorityUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserRoleUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for ERC20MintableEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<AuthorityUpdatedFilter> for ERC20MintableEvents {
        fn from(value: AuthorityUpdatedFilter) -> Self {
            Self::AuthorityUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for ERC20MintableEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PublicCapabilityUpdatedFilter> for ERC20MintableEvents {
        fn from(value: PublicCapabilityUpdatedFilter) -> Self {
            Self::PublicCapabilityUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<RoleCapabilityUpdatedFilter> for ERC20MintableEvents {
        fn from(value: RoleCapabilityUpdatedFilter) -> Self {
            Self::RoleCapabilityUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TargetCustomAuthorityUpdatedFilter>
    for ERC20MintableEvents {
        fn from(value: TargetCustomAuthorityUpdatedFilter) -> Self {
            Self::TargetCustomAuthorityUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for ERC20MintableEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<UserRoleUpdatedFilter> for ERC20MintableEvents {
        fn from(value: UserRoleUpdatedFilter) -> Self {
            Self::UserRoleUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `authority` function with signature `authority()` and selector `0xbf7e214f`
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
    #[ethcall(name = "authority", abi = "authority()")]
    pub struct AuthorityCall;
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `burn` function with signature `burn(uint256)` and selector `0x42966c68`
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
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `burn` function with signature `burn(address,uint256)` and selector `0x9dc29fac`
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
    #[ethcall(name = "burn", abi = "burn(address,uint256)")]
    pub struct BurnWithDestinationCall {
        pub destination: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `canCall` function with signature `canCall(address,address,bytes4)` and selector `0xb7009613`
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
    #[ethcall(name = "canCall", abi = "canCall(address,address,bytes4)")]
    pub struct CanCallCall {
        pub user: ::ethers::core::types::Address,
        pub target: ::ethers::core::types::Address,
        pub function_sig: [u8; 4],
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `doesRoleHaveCapability` function with signature `doesRoleHaveCapability(uint8,bytes4)` and selector `0xe688747b`
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
        name = "doesRoleHaveCapability",
        abi = "doesRoleHaveCapability(uint8,bytes4)"
    )]
    pub struct DoesRoleHaveCapabilityCall {
        pub role: u8,
        pub function_sig: [u8; 4],
    }
    ///Container type for all input parameters for the `doesUserHaveRole` function with signature `doesUserHaveRole(address,uint8)` and selector `0xea7ca276`
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
    #[ethcall(name = "doesUserHaveRole", abi = "doesUserHaveRole(address,uint8)")]
    pub struct DoesUserHaveRoleCall {
        pub user: ::ethers::core::types::Address,
        pub role: u8,
    }
    ///Container type for all input parameters for the `getRolesWithCapability` function with signature `getRolesWithCapability(bytes4)` and selector `0xed0d0efb`
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
    #[ethcall(name = "getRolesWithCapability", abi = "getRolesWithCapability(bytes4)")]
    pub struct GetRolesWithCapabilityCall(pub [u8; 4]);
    ///Container type for all input parameters for the `getTargetCustomAuthority` function with signature `getTargetCustomAuthority(address)` and selector `0xc53a3985`
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
        name = "getTargetCustomAuthority",
        abi = "getTargetCustomAuthority(address)"
    )]
    pub struct GetTargetCustomAuthorityCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getUserRoles` function with signature `getUserRoles(address)` and selector `0x06a36aee`
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
    #[ethcall(name = "getUserRoles", abi = "getUserRoles(address)")]
    pub struct GetUserRolesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `isCapabilityPublic` function with signature `isCapabilityPublic(bytes4)` and selector `0x0bade8a4`
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
    #[ethcall(name = "isCapabilityPublic", abi = "isCapabilityPublic(bytes4)")]
    pub struct IsCapabilityPublicCall(pub [u8; 4]);
    ///Container type for all input parameters for the `isCompetitionMode` function with signature `isCompetitionMode()` and selector `0x7a8c63b5`
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
    #[ethcall(name = "isCompetitionMode", abi = "isCompetitionMode()")]
    pub struct IsCompetitionModeCall;
    ///Container type for all input parameters for the `isUnrestricted` function with signature `isUnrestricted(address)` and selector `0xaed30777`
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
    #[ethcall(name = "isUnrestricted", abi = "isUnrestricted(address)")]
    pub struct IsUnrestrictedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `maxMintAmount` function with signature `maxMintAmount()` and selector `0x239c70ae`
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
    #[ethcall(name = "maxMintAmount", abi = "maxMintAmount()")]
    pub struct MaxMintAmountCall;
    ///Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `0x40c10f19`
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
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintWithDestinationCall {
        pub destination: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mint` function with signature `mint(uint256)` and selector `0xa0712d68`
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
    #[ethcall(name = "mint", abi = "mint(uint256)")]
    pub struct MintCall {
        pub amount: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `setAuthority` function with signature `setAuthority(address)` and selector `0x7a9e5e4b`
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
    #[ethcall(name = "setAuthority", abi = "setAuthority(address)")]
    pub struct SetAuthorityCall {
        pub new_authority: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setMaxMintAmount` function with signature `setMaxMintAmount(uint256)` and selector `0x088a4ed0`
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
    #[ethcall(name = "setMaxMintAmount", abi = "setMaxMintAmount(uint256)")]
    pub struct SetMaxMintAmountCall {
        pub max_mint_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPublicCapability` function with signature `setPublicCapability(bytes4,bool)` and selector `0x4b5159da`
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
    #[ethcall(name = "setPublicCapability", abi = "setPublicCapability(bytes4,bool)")]
    pub struct SetPublicCapabilityCall {
        pub function_sig: [u8; 4],
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `setRoleCapability` function with signature `setRoleCapability(uint8,bytes4,bool)` and selector `0x0ea9b75b`
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
    #[ethcall(name = "setRoleCapability", abi = "setRoleCapability(uint8,bytes4,bool)")]
    pub struct SetRoleCapabilityCall {
        pub role: u8,
        pub function_sig: [u8; 4],
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `setTargetCustomAuthority` function with signature `setTargetCustomAuthority(address,address)` and selector `0x728b952b`
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
        name = "setTargetCustomAuthority",
        abi = "setTargetCustomAuthority(address,address)"
    )]
    pub struct SetTargetCustomAuthorityCall {
        pub target: ::ethers::core::types::Address,
        pub custom_authority: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setUnrestrictedMintStatus` function with signature `setUnrestrictedMintStatus(address,bool)` and selector `0x05f05a94`
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
        name = "setUnrestrictedMintStatus",
        abi = "setUnrestrictedMintStatus(address,bool)"
    )]
    pub struct SetUnrestrictedMintStatusCall {
        pub target: ::ethers::core::types::Address,
        pub status: bool,
    }
    ///Container type for all input parameters for the `setUserRole` function with signature `setUserRole(address,uint8,bool)` and selector `0x67aff484`
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
    #[ethcall(name = "setUserRole", abi = "setUserRole(address,uint8,bool)")]
    pub struct SetUserRoleCall {
        pub user: ::ethers::core::types::Address,
        pub role: u8,
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum ERC20MintableCalls {
        DomainSeparator(DomainSeparatorCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        Authority(AuthorityCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        BurnWithDestination(BurnWithDestinationCall),
        CanCall(CanCallCall),
        Decimals(DecimalsCall),
        DoesRoleHaveCapability(DoesRoleHaveCapabilityCall),
        DoesUserHaveRole(DoesUserHaveRoleCall),
        GetRolesWithCapability(GetRolesWithCapabilityCall),
        GetTargetCustomAuthority(GetTargetCustomAuthorityCall),
        GetUserRoles(GetUserRolesCall),
        IsCapabilityPublic(IsCapabilityPublicCall),
        IsCompetitionMode(IsCompetitionModeCall),
        IsUnrestricted(IsUnrestrictedCall),
        MaxMintAmount(MaxMintAmountCall),
        MintWithDestination(MintWithDestinationCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        Permit(PermitCall),
        SetAuthority(SetAuthorityCall),
        SetMaxMintAmount(SetMaxMintAmountCall),
        SetPublicCapability(SetPublicCapabilityCall),
        SetRoleCapability(SetRoleCapabilityCall),
        SetTargetCustomAuthority(SetTargetCustomAuthorityCall),
        SetUnrestrictedMintStatus(SetUnrestrictedMintStatusCall),
        SetUserRole(SetUserRoleCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for ERC20MintableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <AuthorityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Authority(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded) = <BurnWithDestinationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BurnWithDestination(decoded));
            }
            if let Ok(decoded) = <CanCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CanCall(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DoesRoleHaveCapabilityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DoesRoleHaveCapability(decoded));
            }
            if let Ok(decoded) = <DoesUserHaveRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DoesUserHaveRole(decoded));
            }
            if let Ok(decoded) = <GetRolesWithCapabilityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRolesWithCapability(decoded));
            }
            if let Ok(decoded) = <GetTargetCustomAuthorityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTargetCustomAuthority(decoded));
            }
            if let Ok(decoded) = <GetUserRolesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUserRoles(decoded));
            }
            if let Ok(decoded) = <IsCapabilityPublicCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsCapabilityPublic(decoded));
            }
            if let Ok(decoded) = <IsCompetitionModeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsCompetitionMode(decoded));
            }
            if let Ok(decoded) = <IsUnrestrictedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsUnrestricted(decoded));
            }
            if let Ok(decoded) = <MaxMintAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxMintAmount(decoded));
            }
            if let Ok(decoded) = <MintWithDestinationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintWithDestination(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded) = <SetAuthorityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAuthority(decoded));
            }
            if let Ok(decoded) = <SetMaxMintAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetMaxMintAmount(decoded));
            }
            if let Ok(decoded) = <SetPublicCapabilityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPublicCapability(decoded));
            }
            if let Ok(decoded) = <SetRoleCapabilityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetRoleCapability(decoded));
            }
            if let Ok(decoded) = <SetTargetCustomAuthorityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTargetCustomAuthority(decoded));
            }
            if let Ok(decoded) = <SetUnrestrictedMintStatusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUnrestrictedMintStatus(decoded));
            }
            if let Ok(decoded) = <SetUserRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUserRole(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC20MintableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Authority(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BurnWithDestination(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CanCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DoesRoleHaveCapability(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DoesUserHaveRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRolesWithCapability(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTargetCustomAuthority(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserRoles(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsCapabilityPublic(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsCompetitionMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsUnrestricted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxMintAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintWithDestination(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAuthority(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMaxMintAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPublicCapability(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRoleCapability(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTargetCustomAuthority(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUnrestrictedMintStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUserRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ERC20MintableCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::Authority(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnWithDestination(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CanCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::DoesRoleHaveCapability(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DoesUserHaveRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRolesWithCapability(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTargetCustomAuthority(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUserRoles(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsCapabilityPublic(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsCompetitionMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsUnrestricted(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxMintAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintWithDestination(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAuthority(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMaxMintAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPublicCapability(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetRoleCapability(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTargetCustomAuthority(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetUnrestrictedMintStatus(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetUserRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for ERC20MintableCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for ERC20MintableCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for ERC20MintableCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<AuthorityCall> for ERC20MintableCalls {
        fn from(value: AuthorityCall) -> Self {
            Self::Authority(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for ERC20MintableCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for ERC20MintableCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<BurnWithDestinationCall> for ERC20MintableCalls {
        fn from(value: BurnWithDestinationCall) -> Self {
            Self::BurnWithDestination(value)
        }
    }
    impl ::core::convert::From<CanCallCall> for ERC20MintableCalls {
        fn from(value: CanCallCall) -> Self {
            Self::CanCall(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for ERC20MintableCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DoesRoleHaveCapabilityCall> for ERC20MintableCalls {
        fn from(value: DoesRoleHaveCapabilityCall) -> Self {
            Self::DoesRoleHaveCapability(value)
        }
    }
    impl ::core::convert::From<DoesUserHaveRoleCall> for ERC20MintableCalls {
        fn from(value: DoesUserHaveRoleCall) -> Self {
            Self::DoesUserHaveRole(value)
        }
    }
    impl ::core::convert::From<GetRolesWithCapabilityCall> for ERC20MintableCalls {
        fn from(value: GetRolesWithCapabilityCall) -> Self {
            Self::GetRolesWithCapability(value)
        }
    }
    impl ::core::convert::From<GetTargetCustomAuthorityCall> for ERC20MintableCalls {
        fn from(value: GetTargetCustomAuthorityCall) -> Self {
            Self::GetTargetCustomAuthority(value)
        }
    }
    impl ::core::convert::From<GetUserRolesCall> for ERC20MintableCalls {
        fn from(value: GetUserRolesCall) -> Self {
            Self::GetUserRoles(value)
        }
    }
    impl ::core::convert::From<IsCapabilityPublicCall> for ERC20MintableCalls {
        fn from(value: IsCapabilityPublicCall) -> Self {
            Self::IsCapabilityPublic(value)
        }
    }
    impl ::core::convert::From<IsCompetitionModeCall> for ERC20MintableCalls {
        fn from(value: IsCompetitionModeCall) -> Self {
            Self::IsCompetitionMode(value)
        }
    }
    impl ::core::convert::From<IsUnrestrictedCall> for ERC20MintableCalls {
        fn from(value: IsUnrestrictedCall) -> Self {
            Self::IsUnrestricted(value)
        }
    }
    impl ::core::convert::From<MaxMintAmountCall> for ERC20MintableCalls {
        fn from(value: MaxMintAmountCall) -> Self {
            Self::MaxMintAmount(value)
        }
    }
    impl ::core::convert::From<MintWithDestinationCall> for ERC20MintableCalls {
        fn from(value: MintWithDestinationCall) -> Self {
            Self::MintWithDestination(value)
        }
    }
    impl ::core::convert::From<MintCall> for ERC20MintableCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for ERC20MintableCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for ERC20MintableCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ERC20MintableCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PermitCall> for ERC20MintableCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<SetAuthorityCall> for ERC20MintableCalls {
        fn from(value: SetAuthorityCall) -> Self {
            Self::SetAuthority(value)
        }
    }
    impl ::core::convert::From<SetMaxMintAmountCall> for ERC20MintableCalls {
        fn from(value: SetMaxMintAmountCall) -> Self {
            Self::SetMaxMintAmount(value)
        }
    }
    impl ::core::convert::From<SetPublicCapabilityCall> for ERC20MintableCalls {
        fn from(value: SetPublicCapabilityCall) -> Self {
            Self::SetPublicCapability(value)
        }
    }
    impl ::core::convert::From<SetRoleCapabilityCall> for ERC20MintableCalls {
        fn from(value: SetRoleCapabilityCall) -> Self {
            Self::SetRoleCapability(value)
        }
    }
    impl ::core::convert::From<SetTargetCustomAuthorityCall> for ERC20MintableCalls {
        fn from(value: SetTargetCustomAuthorityCall) -> Self {
            Self::SetTargetCustomAuthority(value)
        }
    }
    impl ::core::convert::From<SetUnrestrictedMintStatusCall> for ERC20MintableCalls {
        fn from(value: SetUnrestrictedMintStatusCall) -> Self {
            Self::SetUnrestrictedMintStatus(value)
        }
    }
    impl ::core::convert::From<SetUserRoleCall> for ERC20MintableCalls {
        fn from(value: SetUserRoleCall) -> Self {
            Self::SetUserRole(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for ERC20MintableCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for ERC20MintableCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for ERC20MintableCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for ERC20MintableCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ERC20MintableCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `authority` function with signature `authority()` and selector `0xbf7e214f`
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
    pub struct AuthorityReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `canCall` function with signature `canCall(address,address,bytes4)` and selector `0xb7009613`
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
    pub struct CanCallReturn(pub bool);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `doesRoleHaveCapability` function with signature `doesRoleHaveCapability(uint8,bytes4)` and selector `0xe688747b`
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
    pub struct DoesRoleHaveCapabilityReturn(pub bool);
    ///Container type for all return fields from the `doesUserHaveRole` function with signature `doesUserHaveRole(address,uint8)` and selector `0xea7ca276`
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
    pub struct DoesUserHaveRoleReturn(pub bool);
    ///Container type for all return fields from the `getRolesWithCapability` function with signature `getRolesWithCapability(bytes4)` and selector `0xed0d0efb`
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
    pub struct GetRolesWithCapabilityReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getTargetCustomAuthority` function with signature `getTargetCustomAuthority(address)` and selector `0xc53a3985`
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
    pub struct GetTargetCustomAuthorityReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getUserRoles` function with signature `getUserRoles(address)` and selector `0x06a36aee`
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
    pub struct GetUserRolesReturn(pub [u8; 32]);
    ///Container type for all return fields from the `isCapabilityPublic` function with signature `isCapabilityPublic(bytes4)` and selector `0x0bade8a4`
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
    pub struct IsCapabilityPublicReturn(pub bool);
    ///Container type for all return fields from the `isCompetitionMode` function with signature `isCompetitionMode()` and selector `0x7a8c63b5`
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
    pub struct IsCompetitionModeReturn(pub bool);
    ///Container type for all return fields from the `isUnrestricted` function with signature `isUnrestricted(address)` and selector `0xaed30777`
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
    pub struct IsUnrestrictedReturn(pub bool);
    ///Container type for all return fields from the `maxMintAmount` function with signature `maxMintAmount()` and selector `0x239c70ae`
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
    pub struct MaxMintAmountReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
}
