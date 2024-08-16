pub use mock_lido::*;
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
pub mod mock_lido {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_initialRate"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_admin"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_isCompetitionMode"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bool"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_maxMintAmount"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
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
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_owner"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                    ::std::borrow::ToOwned::to_owned("getBufferedEther"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBufferedEther"),
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
                    ::std::borrow::ToOwned::to_owned("getPooledEthByShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPooledEthByShares",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sharesAmount"),
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
                    ::std::borrow::ToOwned::to_owned("getRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRate"),
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
                    ::std::borrow::ToOwned::to_owned("getSharesByPooledEth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSharesByPooledEth",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ethAmount"),
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
                    ::std::borrow::ToOwned::to_owned("getTotalPooledEther"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalPooledEther",
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
                    ::std::borrow::ToOwned::to_owned("getTotalShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTotalShares"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                    ::std::borrow::ToOwned::to_owned("setRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setRate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rate_"),
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
                    ::std::borrow::ToOwned::to_owned("sharesOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sharesOf"),
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
                    ::std::borrow::ToOwned::to_owned("submit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("submit"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                (
                    ::std::borrow::ToOwned::to_owned("transferShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferShares"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sharesAmount"),
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
                    ::std::borrow::ToOwned::to_owned("transferSharesFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferSharesFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sharesAmount"),
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
                    ::std::borrow::ToOwned::to_owned("TransferShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TransferShares"),
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
                                    name: ::std::borrow::ToOwned::to_owned("sharesValue"),
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InsufficientAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InsufficientAllowance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("allowance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("needed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InsufficientBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("needed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidApprover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InvalidApprover",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approver"),
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
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidReceiver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InvalidReceiver",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
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
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ERC20InvalidSender"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
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
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidSpender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InvalidSpender",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKLIDO_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0!\xFC8\x03\x80b\0!\xFC\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\\V[`@\x80Q\x80\x82\x01\x82R`\x17\x81R\x7FLiquid staked Ether 2.0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x84R`\x05\x81Rd\x0En\x88\xAA\x89`\xDB\x1B\x91\x81\x01\x91\x90\x91R`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x89\x16\x90\x81\x17\x83U`\x01\x80T0\x93\x16\x83\x17\x90U\x94Q\x93\x94\x92\x93\x88\x93\x91\x92\x84\x92\x84\x92\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x903\x90\x7F\xA39o\xD7\xF6\xE0\xA2\x1BP\xE5\x08\x9D-\xA7\rZ\xC0\xA3\xBB\xBD\x1Faz\x93\xF14\xB7c\x89\x98\x01\x98\x90`\0\x90\xA3PPPP\x81`\t\x90\x81b\0\x011\x91\x90b\0\x02aV[P`\nb\0\x01@\x82\x82b\0\x02aV[PPP`\r\x93\x90\x93UB`\x0EU\x15\x15`\x80RP`\x0BUb\0\x03-V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x01sW`\0\x80\xFD[\x84Q` \x86\x01Q\x90\x94P`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x93W`\0\x80\xFD[`@\x86\x01Q\x90\x93P\x80\x15\x15\x81\x14b\0\x01\xAAW`\0\x80\xFD[``\x95\x90\x95\x01Q\x93\x96\x92\x95PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x01\xE5W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x02\x06WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x02\\W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x027WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x02XW\x82\x81U`\x01\x01b\0\x02CV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x02}Wb\0\x02}b\0\x01\xBAV[b\0\x02\x95\x81b\0\x02\x8E\x84Tb\0\x01\xD0V[\x84b\0\x02\x0CV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x02\xCDW`\0\x84\x15b\0\x02\xB4WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x02XV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x02\xFEW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x02\xDDV[P\x85\x82\x10\x15b\0\x03\x1DW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Qa\x1E\x82b\0\x03z`\09`\0\x81\x81a\x05\x87\x01R\x81\x81a\x08\xAD\x01R\x81\x81a\t\xCB\x01R\x81\x81a\x0B\xC4\x01R\x81\x81a\x0CE\x01R\x81\x81a\x0C\xAB\x01R\x81\x81a\x11L\x01Ra\x11\xAE\x01Ra\x1E\x82`\0\xF3\xFE`\x80`@R`\x046\x10a\x02gW`\x005`\xE0\x1C\x80cr\x8B\x95+\x11a\x01DW\x80c\xAE\xD3\x07w\x11a\0\xB6W\x80c\xDDb\xED>\x11a\0zW\x80c\xDDb\xED>\x14a\x07dW\x80c\xE6\x88t{\x14a\x07\xAAW\x80c\xEA|\xA2v\x14a\x07\xEDW\x80c\xED\r\x0E\xFB\x14a\x081W\x80c\xF2\xFD\xE3\x8B\x14a\x08^W\x80c\xF5\xEBB\xDC\x14a\x08~W`\0\x80\xFD[\x80c\xAE\xD3\x07w\x14a\x06\xA9W\x80c\xB7\0\x96\x13\x14a\x06\xD9W\x80c\xBF~!O\x14a\x06\xF9W\x80c\xC5:9\x85\x14a\x07\x19W\x80c\xD5\0/.\x14a\x07OW`\0\x80\xFD[\x80c\x8F\xCBN[\x11a\x01\x08W\x80c\x8F\xCBN[\x14a\x06\x01W\x80c\x95\xD8\x9BA\x14a\x06!W\x80c\x9D\xC2\x9F\xAC\x14a\x066W\x80c\xA0q-h\x14a\x06VW\x80c\xA1\x90>\xAB\x14a\x06vW\x80c\xA9\x05\x9C\xBB\x14a\x06\x89W`\0\x80\xFD[\x80cr\x8B\x95+\x14a\x055W\x80cz(\xFB\x88\x14a\x05UW\x80cz\x8Cc\xB5\x14a\x05uW\x80cz\x9E^K\x14a\x05\xA9W\x80c\x8D\xA5\xCB[\x14a\x05\xC9W`\0\x80\xFD[\x80c1<\xE5g\x11a\x01\xDDW\x80cG\xB7\x14\xE0\x11a\x01\xA1W\x80cG\xB7\x14\xE0\x14a\x04\x8CW\x80cKQY\xDA\x14a\x04\xA0W\x80cg\x9A\xEF\xCE\x14a\x04\xC0W\x80cg\xAF\xF4\x84\x14a\x04\xD5W\x80cmx\x04Y\x14a\x04\xF5W\x80cp\xA0\x821\x14a\x05\x15W`\0\x80\xFD[\x80c1<\xE5g\x14a\x03\xFBW\x80c4\xFC\xF47\x14a\x04\x17W\x80c7\xCF\xDA\xCA\x14a\x047W\x80c@\xC1\x0F\x19\x14a\x04LW\x80cB\x96lh\x14a\x04lW`\0\x80\xFD[\x80c\x0B\xAD\xE8\xA4\x11a\x02/W\x80c\x0B\xAD\xE8\xA4\x14a\x03@W\x80c\x0E\xA9\xB7[\x14a\x03pW\x80c\x18\x16\r\xDD\x14a\x03\x90W\x80c\x19 \x84Q\x14a\x03\xA5W\x80c#\x9Cp\xAE\x14a\x03\xC5W\x80c#\xB8r\xDD\x14a\x03\xDBW`\0\x80\xFD[\x80c\x05\xF0Z\x94\x14a\x02lW\x80c\x06\xA3j\xEE\x14a\x02\x8EW\x80c\x06\xFD\xDE\x03\x14a\x02\xCEW\x80c\x08\x8AN\xD0\x14a\x02\xF0W\x80c\t^\xA7\xB3\x14a\x03\x10W[`\0\x80\xFD[4\x80\x15a\x02xW`\0\x80\xFD[Pa\x02\x8Ca\x02\x876`\x04a\x1A\x16V[a\x08\xABV[\0[4\x80\x15a\x02\x9AW`\0\x80\xFD[Pa\x02\xBBa\x02\xA96`\x04a\x1AOV[`\x03` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xDAW`\0\x80\xFD[Pa\x02\xE3a\t7V[`@Qa\x02\xC5\x91\x90a\x1AlV[4\x80\x15a\x02\xFCW`\0\x80\xFD[Pa\x02\x8Ca\x03\x0B6`\x04a\x1A\xBBV[a\t\xC9V[4\x80\x15a\x03\x1CW`\0\x80\xFD[Pa\x030a\x03+6`\x04a\x1A\xD4V[a\n&V[`@Q\x90\x15\x15\x81R` \x01a\x02\xC5V[4\x80\x15a\x03LW`\0\x80\xFD[Pa\x030a\x03[6`\x04a\x1B\x1DV[`\x04` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x03|W`\0\x80\xFD[Pa\x02\x8Ca\x03\x8B6`\x04a\x1BIV[a\n@V[4\x80\x15a\x03\x9CW`\0\x80\xFD[P`\x08Ta\x02\xBBV[4\x80\x15a\x03\xB1W`\0\x80\xFD[Pa\x02\xBBa\x03\xC06`\x04a\x1A\xBBV[a\x0B\x18V[4\x80\x15a\x03\xD1W`\0\x80\xFD[Pa\x02\xBB`\x0BT\x81V[4\x80\x15a\x03\xE7W`\0\x80\xFD[Pa\x030a\x03\xF66`\x04a\x1B\x90V[a\x0B6V[4\x80\x15a\x04\x07W`\0\x80\xFD[P`@Q`\x12\x81R` \x01a\x02\xC5V[4\x80\x15a\x04#W`\0\x80\xFD[Pa\x02\x8Ca\x0426`\x04a\x1A\xBBV[a\x0B\xC2V[4\x80\x15a\x04CW`\0\x80\xFD[Pa\x02\xBBa\x0C'V[4\x80\x15a\x04XW`\0\x80\xFD[Pa\x02\x8Ca\x04g6`\x04a\x1A\xD4V[a\x0CCV[4\x80\x15a\x04xW`\0\x80\xFD[Pa\x02\x8Ca\x04\x876`\x04a\x1A\xBBV[a\x0C\xA9V[4\x80\x15a\x04\x98W`\0\x80\xFD[P`\0a\x02\xBBV[4\x80\x15a\x04\xACW`\0\x80\xFD[Pa\x02\x8Ca\x04\xBB6`\x04a\x1B\xD1V[a\r\x0EV[4\x80\x15a\x04\xCCW`\0\x80\xFD[P`\rTa\x02\xBBV[4\x80\x15a\x04\xE1W`\0\x80\xFD[Pa\x02\x8Ca\x04\xF06`\x04a\x1B\xEDV[a\r\xA0V[4\x80\x15a\x05\x01W`\0\x80\xFD[Pa\x02\xBBa\x05\x106`\x04a\x1B\x90V[a\x0EhV[4\x80\x15a\x05!W`\0\x80\xFD[Pa\x02\xBBa\x0506`\x04a\x1AOV[a\x0E\xFFV[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x02\x8Ca\x05P6`\x04a\x1C\x1BV[a\x0F!V[4\x80\x15a\x05aW`\0\x80\xFD[Pa\x02\xBBa\x05p6`\x04a\x1A\xBBV[a\x0F\xAAV[4\x80\x15a\x05\x81W`\0\x80\xFD[Pa\x030\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\xB5W`\0\x80\xFD[Pa\x02\x8Ca\x05\xC46`\x04a\x1AOV[a\x0F\xBFV[4\x80\x15a\x05\xD5W`\0\x80\xFD[P`\0Ta\x05\xE9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xC5V[4\x80\x15a\x06\rW`\0\x80\xFD[Pa\x02\xBBa\x06\x1C6`\x04a\x1A\xD4V[a\x10\xA9V[4\x80\x15a\x06-W`\0\x80\xFD[Pa\x02\xE3a\x11;V[4\x80\x15a\x06BW`\0\x80\xFD[Pa\x02\x8Ca\x06Q6`\x04a\x1A\xD4V[a\x11JV[4\x80\x15a\x06bW`\0\x80\xFD[Pa\x02\x8Ca\x06q6`\x04a\x1A\xBBV[a\x11\xACV[a\x02\xBBa\x06\x846`\x04a\x1AOV[a\x12\x0EV[4\x80\x15a\x06\x95W`\0\x80\xFD[Pa\x030a\x06\xA46`\x04a\x1A\xD4V[a\x12\xD3V[4\x80\x15a\x06\xB5W`\0\x80\xFD[Pa\x030a\x06\xC46`\x04a\x1AOV[`\x0C` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x06\xE5W`\0\x80\xFD[Pa\x030a\x06\xF46`\x04a\x1CIV[a\x13[V[4\x80\x15a\x07\x05W`\0\x80\xFD[P`\x01Ta\x05\xE9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07%W`\0\x80\xFD[Pa\x05\xE9a\x0746`\x04a\x1AOV[`\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07[W`\0\x80\xFD[P`\x10Ta\x02\xBBV[4\x80\x15a\x07pW`\0\x80\xFD[Pa\x02\xBBa\x07\x7F6`\x04a\x1C\x1BV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[4\x80\x15a\x07\xB6W`\0\x80\xFD[Pa\x030a\x07\xC56`\x04a\x1C\x90V[`\x01`\x01`\xE0\x1B\x03\x19\x16`\0\x90\x81R`\x05` R`@\x90 T`\xFF\x91\x90\x91\x16\x1C`\x01\x16\x15\x15\x90V[4\x80\x15a\x07\xF9W`\0\x80\xFD[Pa\x030a\x08\x086`\x04a\x1C\xC3V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x03` R`@\x90 T`\x01`\xFF\x90\x92\x16\x1C\x16\x15\x15\x90V[4\x80\x15a\x08=W`\0\x80\xFD[Pa\x02\xBBa\x08L6`\x04a\x1B\x1DV[`\x05` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x08jW`\0\x80\xFD[Pa\x02\x8Ca\x08y6`\x04a\x1AOV[a\x14YV[4\x80\x15a\x08\x8AW`\0\x80\xFD[Pa\x02\xBBa\x08\x996`\x04a\x1AOV[`\x11` R`\0\x90\x81R`@\x90 T\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\t\x0CWa\x08\xE73`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\t\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1C\xEFV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x0C` R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[```\t\x80Ta\tF\x90a\x1D&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tr\x90a\x1D&V[\x80\x15a\t\xBFW\x80`\x1F\x10a\t\x94Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xBFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xA2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\n!Wa\n\x053`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\n!W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1C\xEFV[`\x0BUV[`\x003a\n4\x81\x85\x85a\x15\x80V[`\x01\x91PP[\x92\x91PPV[a\nV3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\nrW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1D`V[\x80\x15a\n\xA2W`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x90\x81R`\x05` R`@\x90 \x80T`\x01`\xFF\x86\x16\x1B\x17\x90Ua\n\xC9V[`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x90\x81R`\x05` R`@\x90 \x80T`\x01`\xFF\x86\x16\x1B\x19\x16\x90U[\x81`\x01`\x01`\xE0\x1B\x03\x19\x16\x83`\xFF\x16\x7F\xBF\xE1k,5\xCE#\xDF\xD1\xAB\x0E{]\x08j\x10\x06\x0C\x9BR\xD1WN\x16\x80\xC8\x81\xB3\xB3\xA2\xB1Q\x83`@Qa\x0B\x0B\x91\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[`\0a\n:a\x0B&`\x10T\x90V[a\x0B.a\x0C'V[\x84\x91\x90a\x15\x92V[`\0a\x0B@a\x15\xB0V[`\0a\x0BK\x83a\x0B\x18V[\x90Pa\x0BX\x85\x85\x83a\x15\xE1V[P`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x903\x90`\0\x80Q` a\x1E-\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA3`@Q\x81\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x903\x90`\0\x80Q` a\x1E\r\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA3`\x01\x91PP[\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0C\x1AWa\x0B\xFE3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\x0C\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1C\xEFV[a\x0C\"a\x15\xB0V[`\rUV[`\0a\x0C1a\x16]V[`\x0FTa\x0C>\x91\x90a\x1D\x9CV[\x90P\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0C\x9BWa\x0C\x7F3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\x0C\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1C\xEFV[a\x0C\xA5\x82\x82a\x16\xC2V[PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\r\x01Wa\x0C\xE53`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\r\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1C\xEFV[a\r\x0B3\x82a\x17\xB4V[PV[a\r$3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\r@W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1D`V[`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x81\x81R`\x04` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F6\xD2\x81&\xBE\xF2\x1AO7e\xD7\xFC\xB7\xC4\\\xEA\xD4c\xAELA\tN\xF3\xB7q\xED\xE5\x98TA\x03\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[a\r\xB63`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\r\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1D`V[\x80\x15a\x0E\x01W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80T`\x01`\xFF\x85\x16\x1B\x17\x90Ua\x0E'V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80T`\x01`\xFF\x85\x16\x1B\x19\x16\x90U[\x81`\xFF\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7FL\x9B\xDD\x0C\x8E\x07>\xB5\xED\xA2%\x0B\x18\xD8\xE5\x12\x1F\xF2{b\x06O\xBE\xEE\xEDHi\xBB\x99\xBC[\xF2\x83`@Qa\x0B\x0B\x91\x15\x15\x81R` \x01\x90V[`\0a\x0Era\x15\xB0V[`\0a\x0E\x7F\x85\x85\x85a\x15\xE1V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a\x1E-\x839\x81Q\x91R\x83`@Qa\x0E\xB4\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a\x1E\r\x839\x81Q\x91R\x85`@Qa\x0E\xEF\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x11` R`@\x81 Ta\n:\x90a\x0F\xAAV[a\x0F73`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\x0FSW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1D`V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x86\x16\x94\x85\x17\x90UQ\x7F\xA4\x90\x8E\x11\xA5\xF8\x95\xB1=QRl3\x1A\xC9<\xDD0\xE5\x97r6\x1C]\x07\x87N\xB3k\xFF e\x91\x90\xA3PPV[`\0a\n:a\x0F\xB7a\x0C'V[`\x10Ta\x0B.V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x10TWP`\x01T`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB7\0\x96\x13\x90a\x10\x13\x903\x900\x90`\x01`\x01`\xE0\x1B\x03\x19`\x005\x16\x90`\x04\x01a\x1D\xAFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x100W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10T\x91\x90a\x1D\xDCV[a\x10]W`\0\x80\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90\x7F\xA39o\xD7\xF6\xE0\xA2\x1BP\xE5\x08\x9D-\xA7\rZ\xC0\xA3\xBB\xBD\x1Faz\x93\xF14\xB7c\x89\x98\x01\x98\x90`\0\x90\xA3PV[`\0a\x10\xB3a\x15\xB0V[`\0a\x10\xBF\x84\x84a\x18\x1EV[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a\x1E-\x839\x81Q\x91R\x83`@Qa\x10\xF4\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x903\x90`\0\x80Q` a\x1E\r\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA3a\x113\x83a\x0F\xAAV[\x94\x93PPPPV[```\n\x80Ta\tF\x90a\x1D&V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x11\xA2Wa\x11\x863`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\x11\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1C\xEFV[a\x0C\xA5\x82\x82a\x17\xB4V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x12\x04Wa\x11\xE83`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\x12\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1C\xEFV[a\r\x0B3\x82a\x16\xC2V[`\0a\x12\x18a\x15\xB0V[`\x10T`\0\x03a\x12XW4`\x10\x81\x90U`\x0F\x81\x90U3`\0\x90\x81R`\x11` R`@\x81 \x80T\x90\x91\x90a\x12L\x90\x84\x90a\x1D\x9CV[\x90\x91UP4\x93\x92PPPV[`\0a\x12va\x12f`\x10T\x90V[a\x12na\x0C'V[4\x91\x90a\x15\x92V[\x90P4`\x0F`\0\x82\x82Ta\x12\x8A\x91\x90a\x1D\x9CV[\x92PP\x81\x90UP\x80`\x10`\0\x82\x82Ta\x12\xA3\x91\x90a\x1D\x9CV[\x90\x91UPP3`\0\x90\x81R`\x11` R`@\x81 \x80T\x83\x92\x90a\x12\xC7\x90\x84\x90a\x1D\x9CV[\x90\x91UP\x90\x93\x92PPPV[`\0a\x12\xDDa\x15\xB0V[`\0a\x12\xE8\x83a\x0B\x18V[\x90Pa\x12\xF4\x84\x82a\x18\x1EV[P`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x903\x90`\0\x80Q` a\x1E-\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA3`@Q\x81\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x903\x90`\0\x80Q` a\x1E\r\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA3P`\x01\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x02` R`@\x81 T\x90\x91\x16\x80\x15a\x13\xF7W`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB7\0\x96\x13\x90a\x13\xAE\x90\x88\x90\x88\x90\x88\x90`\x04\x01a\x1D\xAFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xEF\x91\x90a\x1D\xDCV[\x91PPa\x0B\xBBV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16`\0\x90\x81R`\x04` R`@\x90 T`\xFF\x16\x80a\x14PWP`\x01`\x01`\xE0\x1B\x03\x19\x83\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 T`\x01`\x01`\xA0\x1B\x03\x89\x16\x84R`\x03\x90\x92R\x90\x91 T\x16\x15\x15[\x95\x94PPPPPV[a\x14o3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\x14\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1D`V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\x01T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x80\x15\x90a\x15`WP`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB7\0\x96\x13\x90a\x15\x1F\x90\x87\x900\x90\x88\x90`\x04\x01a\x1D\xAFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15`\x91\x90a\x1D\xDCV[\x80a\x113WP`\0T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x91\x16\x14\x91PP\x92\x91PPV[a\x15\x8D\x83\x83\x83`\x01a\x18{V[PPPV[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x15\xA9W`\0\x80\xFD[P\x91\x02\x04\x90V[`\0a\x15\xBAa\x16]V[\x90P\x80\x15a\x15\xDAW\x80`\x0F`\0\x82\x82Ta\x15\xD4\x91\x90a\x1D\x9CV[\x90\x91UPP[PB`\x0EUV[`\0\x80a\x15\xED\x83a\x0F\xAAV[\x90Pa\x15\xFA\x853\x83a\x19QV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x11` R`@\x81 \x80T\x85\x92\x90a\x16\"\x90\x84\x90a\x1D\xF9V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x11` R`@\x81 \x80T\x85\x92\x90a\x16O\x90\x84\x90a\x1D\x9CV[\x90\x91UP\x90\x95\x94PPPPPV[`\0`\rT`\0\x03a\x16oWP`\0\x90V[`\x0ETB\x10\x15a\x16\x7FWP`\0\x90V[`\0a\x16\x9Dc\x01\xE13\x80`\x0ETBa\x16\x97\x91\x90a\x1D\xF9V[\x90a\x19\xC9V[\x90P`\0a\x0B\xBBa\x16\xB9\x83`\rTa\x19\xDE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0FT\x90a\x19\xDEV[3`\0\x90\x81R`\x0C` R`@\x90 T`\xFF\x16a\x17+W`\x0BT\x81\x11\x15a\x17+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FMockLido: Invalid mint amount\0\0\0`D\x82\x01R`d\x01a\t\x03V[`\0a\x176`\x10T\x90V[`\0\x03a\x17DWP\x80a\x17PV[a\x17M\x82a\x0B\x18V[\x90P[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x11` R`@\x81 \x80T\x83\x92\x90a\x17x\x90\x84\x90a\x1D\x9CV[\x92PP\x81\x90UP\x81`\x0F`\0\x82\x82Ta\x17\x91\x91\x90a\x1D\x9CV[\x92PP\x81\x90UP\x80`\x10`\0\x82\x82Ta\x17\xAA\x91\x90a\x1D\x9CV[\x90\x91UPPPPPV[`\0a\x17\xBF\x82a\x0B\x18V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x11` R`@\x81 \x80T\x92\x93P\x83\x92\x90\x91\x90a\x17\xEC\x90\x84\x90a\x1D\xF9V[\x92PP\x81\x90UP\x81`\x0F`\0\x82\x82Ta\x18\x05\x91\x90a\x1D\xF9V[\x92PP\x81\x90UP\x80`\x10`\0\x82\x82Ta\x17\xAA\x91\x90a\x1D\xF9V[3`\0\x90\x81R`\x11` R`@\x81 \x80T\x83\x91\x90\x83\x90a\x18?\x90\x84\x90a\x1D\xF9V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x11` R`@\x81 \x80T\x84\x92\x90a\x18l\x90\x84\x90a\x1D\x9CV[\x90\x91UPa\x0B\xBB\x90P\x82a\x0F\xAAV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x18\xA5W`@Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\t\x03V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x18\xCFW`@QcJ\x14\x06\xB1`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\t\x03V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x82\x90U\x80\x15a\x19KW\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x19B\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T`\0\x19\x81\x14a\x19KW\x81\x81\x10\x15a\x19\xBAW`@Qc}\xC7\xA0\xD9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\t\x03V[a\x19K\x84\x84\x84\x84\x03`\0a\x18{V[`\0a\x0B\xBB\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x15\x92V[`\0a\x0B\xBB\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x15\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\x0BW`\0\x80\xFD[\x80\x15\x15\x81\x14a\r\x0BW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x1A)W`\0\x80\xFD[\x825a\x1A4\x81a\x19\xF3V[\x91P` \x83\x015a\x1AD\x81a\x1A\x08V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x1AaW`\0\x80\xFD[\x815a\x0B\xBB\x81a\x19\xF3V[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x1A\x9AW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x1A~V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1A\xCDW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1A\xE7W`\0\x80\xFD[\x825a\x1A\xF2\x81a\x19\xF3V[\x94` \x93\x90\x93\x015\x93PPPV[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1B\x18W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1B/W`\0\x80\xFD[a\x0B\xBB\x82a\x1B\0V[\x805`\xFF\x81\x16\x81\x14a\x1B\x18W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1B^W`\0\x80\xFD[a\x1Bg\x84a\x1B8V[\x92Pa\x1Bu` \x85\x01a\x1B\0V[\x91P`@\x84\x015a\x1B\x85\x81a\x1A\x08V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1B\xA5W`\0\x80\xFD[\x835a\x1B\xB0\x81a\x19\xF3V[\x92P` \x84\x015a\x1B\xC0\x81a\x19\xF3V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x1B\xE4W`\0\x80\xFD[a\x1A4\x83a\x1B\0V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C\x02W`\0\x80\xFD[\x835a\x1C\r\x81a\x19\xF3V[\x92Pa\x1Bu` \x85\x01a\x1B8V[`\0\x80`@\x83\x85\x03\x12\x15a\x1C.W`\0\x80\xFD[\x825a\x1C9\x81a\x19\xF3V[\x91P` \x83\x015a\x1AD\x81a\x19\xF3V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C^W`\0\x80\xFD[\x835a\x1Ci\x81a\x19\xF3V[\x92P` \x84\x015a\x1Cy\x81a\x19\xF3V[\x91Pa\x1C\x87`@\x85\x01a\x1B\0V[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\xA3W`\0\x80\xFD[a\x1C\xAC\x83a\x1B8V[\x91Pa\x1C\xBA` \x84\x01a\x1B\0V[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\xD6W`\0\x80\xFD[\x825a\x1C\xE1\x81a\x19\xF3V[\x91Pa\x1C\xBA` \x84\x01a\x1B8V[` \x80\x82R`\x18\x90\x82\x01R\x7FMockLido: not authorized\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1D:W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1DZWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\n:Wa\n:a\x1D\x86V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1D\xEEW`\0\x80\xFD[\x81Qa\x0B\xBB\x81a\x1A\x08V[\x81\x81\x03\x81\x81\x11\x15a\n:Wa\n:a\x1D\x86V\xFE\x9D\x9C\x90\x92\x96\xD9\xC6tE\x1C\x0C$\xF0,\xB6I\x81\xEB;r\x7F\x99\x86Y9\x19/\x88\nu]\xCB\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xA5S+<\x02\x16Q\xB8:\x87\xF9D\x0C\xB4q)-\x7F~I\x97p\x8F\xC0\xA7\xEE\xF4\x107\xAC\xB8\x8BdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static MOCKLIDO_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x02gW`\x005`\xE0\x1C\x80cr\x8B\x95+\x11a\x01DW\x80c\xAE\xD3\x07w\x11a\0\xB6W\x80c\xDDb\xED>\x11a\0zW\x80c\xDDb\xED>\x14a\x07dW\x80c\xE6\x88t{\x14a\x07\xAAW\x80c\xEA|\xA2v\x14a\x07\xEDW\x80c\xED\r\x0E\xFB\x14a\x081W\x80c\xF2\xFD\xE3\x8B\x14a\x08^W\x80c\xF5\xEBB\xDC\x14a\x08~W`\0\x80\xFD[\x80c\xAE\xD3\x07w\x14a\x06\xA9W\x80c\xB7\0\x96\x13\x14a\x06\xD9W\x80c\xBF~!O\x14a\x06\xF9W\x80c\xC5:9\x85\x14a\x07\x19W\x80c\xD5\0/.\x14a\x07OW`\0\x80\xFD[\x80c\x8F\xCBN[\x11a\x01\x08W\x80c\x8F\xCBN[\x14a\x06\x01W\x80c\x95\xD8\x9BA\x14a\x06!W\x80c\x9D\xC2\x9F\xAC\x14a\x066W\x80c\xA0q-h\x14a\x06VW\x80c\xA1\x90>\xAB\x14a\x06vW\x80c\xA9\x05\x9C\xBB\x14a\x06\x89W`\0\x80\xFD[\x80cr\x8B\x95+\x14a\x055W\x80cz(\xFB\x88\x14a\x05UW\x80cz\x8Cc\xB5\x14a\x05uW\x80cz\x9E^K\x14a\x05\xA9W\x80c\x8D\xA5\xCB[\x14a\x05\xC9W`\0\x80\xFD[\x80c1<\xE5g\x11a\x01\xDDW\x80cG\xB7\x14\xE0\x11a\x01\xA1W\x80cG\xB7\x14\xE0\x14a\x04\x8CW\x80cKQY\xDA\x14a\x04\xA0W\x80cg\x9A\xEF\xCE\x14a\x04\xC0W\x80cg\xAF\xF4\x84\x14a\x04\xD5W\x80cmx\x04Y\x14a\x04\xF5W\x80cp\xA0\x821\x14a\x05\x15W`\0\x80\xFD[\x80c1<\xE5g\x14a\x03\xFBW\x80c4\xFC\xF47\x14a\x04\x17W\x80c7\xCF\xDA\xCA\x14a\x047W\x80c@\xC1\x0F\x19\x14a\x04LW\x80cB\x96lh\x14a\x04lW`\0\x80\xFD[\x80c\x0B\xAD\xE8\xA4\x11a\x02/W\x80c\x0B\xAD\xE8\xA4\x14a\x03@W\x80c\x0E\xA9\xB7[\x14a\x03pW\x80c\x18\x16\r\xDD\x14a\x03\x90W\x80c\x19 \x84Q\x14a\x03\xA5W\x80c#\x9Cp\xAE\x14a\x03\xC5W\x80c#\xB8r\xDD\x14a\x03\xDBW`\0\x80\xFD[\x80c\x05\xF0Z\x94\x14a\x02lW\x80c\x06\xA3j\xEE\x14a\x02\x8EW\x80c\x06\xFD\xDE\x03\x14a\x02\xCEW\x80c\x08\x8AN\xD0\x14a\x02\xF0W\x80c\t^\xA7\xB3\x14a\x03\x10W[`\0\x80\xFD[4\x80\x15a\x02xW`\0\x80\xFD[Pa\x02\x8Ca\x02\x876`\x04a\x1A\x16V[a\x08\xABV[\0[4\x80\x15a\x02\x9AW`\0\x80\xFD[Pa\x02\xBBa\x02\xA96`\x04a\x1AOV[`\x03` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xDAW`\0\x80\xFD[Pa\x02\xE3a\t7V[`@Qa\x02\xC5\x91\x90a\x1AlV[4\x80\x15a\x02\xFCW`\0\x80\xFD[Pa\x02\x8Ca\x03\x0B6`\x04a\x1A\xBBV[a\t\xC9V[4\x80\x15a\x03\x1CW`\0\x80\xFD[Pa\x030a\x03+6`\x04a\x1A\xD4V[a\n&V[`@Q\x90\x15\x15\x81R` \x01a\x02\xC5V[4\x80\x15a\x03LW`\0\x80\xFD[Pa\x030a\x03[6`\x04a\x1B\x1DV[`\x04` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x03|W`\0\x80\xFD[Pa\x02\x8Ca\x03\x8B6`\x04a\x1BIV[a\n@V[4\x80\x15a\x03\x9CW`\0\x80\xFD[P`\x08Ta\x02\xBBV[4\x80\x15a\x03\xB1W`\0\x80\xFD[Pa\x02\xBBa\x03\xC06`\x04a\x1A\xBBV[a\x0B\x18V[4\x80\x15a\x03\xD1W`\0\x80\xFD[Pa\x02\xBB`\x0BT\x81V[4\x80\x15a\x03\xE7W`\0\x80\xFD[Pa\x030a\x03\xF66`\x04a\x1B\x90V[a\x0B6V[4\x80\x15a\x04\x07W`\0\x80\xFD[P`@Q`\x12\x81R` \x01a\x02\xC5V[4\x80\x15a\x04#W`\0\x80\xFD[Pa\x02\x8Ca\x0426`\x04a\x1A\xBBV[a\x0B\xC2V[4\x80\x15a\x04CW`\0\x80\xFD[Pa\x02\xBBa\x0C'V[4\x80\x15a\x04XW`\0\x80\xFD[Pa\x02\x8Ca\x04g6`\x04a\x1A\xD4V[a\x0CCV[4\x80\x15a\x04xW`\0\x80\xFD[Pa\x02\x8Ca\x04\x876`\x04a\x1A\xBBV[a\x0C\xA9V[4\x80\x15a\x04\x98W`\0\x80\xFD[P`\0a\x02\xBBV[4\x80\x15a\x04\xACW`\0\x80\xFD[Pa\x02\x8Ca\x04\xBB6`\x04a\x1B\xD1V[a\r\x0EV[4\x80\x15a\x04\xCCW`\0\x80\xFD[P`\rTa\x02\xBBV[4\x80\x15a\x04\xE1W`\0\x80\xFD[Pa\x02\x8Ca\x04\xF06`\x04a\x1B\xEDV[a\r\xA0V[4\x80\x15a\x05\x01W`\0\x80\xFD[Pa\x02\xBBa\x05\x106`\x04a\x1B\x90V[a\x0EhV[4\x80\x15a\x05!W`\0\x80\xFD[Pa\x02\xBBa\x0506`\x04a\x1AOV[a\x0E\xFFV[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x02\x8Ca\x05P6`\x04a\x1C\x1BV[a\x0F!V[4\x80\x15a\x05aW`\0\x80\xFD[Pa\x02\xBBa\x05p6`\x04a\x1A\xBBV[a\x0F\xAAV[4\x80\x15a\x05\x81W`\0\x80\xFD[Pa\x030\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\xB5W`\0\x80\xFD[Pa\x02\x8Ca\x05\xC46`\x04a\x1AOV[a\x0F\xBFV[4\x80\x15a\x05\xD5W`\0\x80\xFD[P`\0Ta\x05\xE9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xC5V[4\x80\x15a\x06\rW`\0\x80\xFD[Pa\x02\xBBa\x06\x1C6`\x04a\x1A\xD4V[a\x10\xA9V[4\x80\x15a\x06-W`\0\x80\xFD[Pa\x02\xE3a\x11;V[4\x80\x15a\x06BW`\0\x80\xFD[Pa\x02\x8Ca\x06Q6`\x04a\x1A\xD4V[a\x11JV[4\x80\x15a\x06bW`\0\x80\xFD[Pa\x02\x8Ca\x06q6`\x04a\x1A\xBBV[a\x11\xACV[a\x02\xBBa\x06\x846`\x04a\x1AOV[a\x12\x0EV[4\x80\x15a\x06\x95W`\0\x80\xFD[Pa\x030a\x06\xA46`\x04a\x1A\xD4V[a\x12\xD3V[4\x80\x15a\x06\xB5W`\0\x80\xFD[Pa\x030a\x06\xC46`\x04a\x1AOV[`\x0C` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x06\xE5W`\0\x80\xFD[Pa\x030a\x06\xF46`\x04a\x1CIV[a\x13[V[4\x80\x15a\x07\x05W`\0\x80\xFD[P`\x01Ta\x05\xE9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07%W`\0\x80\xFD[Pa\x05\xE9a\x0746`\x04a\x1AOV[`\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07[W`\0\x80\xFD[P`\x10Ta\x02\xBBV[4\x80\x15a\x07pW`\0\x80\xFD[Pa\x02\xBBa\x07\x7F6`\x04a\x1C\x1BV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[4\x80\x15a\x07\xB6W`\0\x80\xFD[Pa\x030a\x07\xC56`\x04a\x1C\x90V[`\x01`\x01`\xE0\x1B\x03\x19\x16`\0\x90\x81R`\x05` R`@\x90 T`\xFF\x91\x90\x91\x16\x1C`\x01\x16\x15\x15\x90V[4\x80\x15a\x07\xF9W`\0\x80\xFD[Pa\x030a\x08\x086`\x04a\x1C\xC3V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x03` R`@\x90 T`\x01`\xFF\x90\x92\x16\x1C\x16\x15\x15\x90V[4\x80\x15a\x08=W`\0\x80\xFD[Pa\x02\xBBa\x08L6`\x04a\x1B\x1DV[`\x05` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x08jW`\0\x80\xFD[Pa\x02\x8Ca\x08y6`\x04a\x1AOV[a\x14YV[4\x80\x15a\x08\x8AW`\0\x80\xFD[Pa\x02\xBBa\x08\x996`\x04a\x1AOV[`\x11` R`\0\x90\x81R`@\x90 T\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\t\x0CWa\x08\xE73`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\t\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1C\xEFV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x0C` R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[```\t\x80Ta\tF\x90a\x1D&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tr\x90a\x1D&V[\x80\x15a\t\xBFW\x80`\x1F\x10a\t\x94Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xBFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xA2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\n!Wa\n\x053`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\n!W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1C\xEFV[`\x0BUV[`\x003a\n4\x81\x85\x85a\x15\x80V[`\x01\x91PP[\x92\x91PPV[a\nV3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\nrW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1D`V[\x80\x15a\n\xA2W`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x90\x81R`\x05` R`@\x90 \x80T`\x01`\xFF\x86\x16\x1B\x17\x90Ua\n\xC9V[`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x90\x81R`\x05` R`@\x90 \x80T`\x01`\xFF\x86\x16\x1B\x19\x16\x90U[\x81`\x01`\x01`\xE0\x1B\x03\x19\x16\x83`\xFF\x16\x7F\xBF\xE1k,5\xCE#\xDF\xD1\xAB\x0E{]\x08j\x10\x06\x0C\x9BR\xD1WN\x16\x80\xC8\x81\xB3\xB3\xA2\xB1Q\x83`@Qa\x0B\x0B\x91\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[`\0a\n:a\x0B&`\x10T\x90V[a\x0B.a\x0C'V[\x84\x91\x90a\x15\x92V[`\0a\x0B@a\x15\xB0V[`\0a\x0BK\x83a\x0B\x18V[\x90Pa\x0BX\x85\x85\x83a\x15\xE1V[P`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x903\x90`\0\x80Q` a\x1E-\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA3`@Q\x81\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x903\x90`\0\x80Q` a\x1E\r\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA3`\x01\x91PP[\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0C\x1AWa\x0B\xFE3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\x0C\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1C\xEFV[a\x0C\"a\x15\xB0V[`\rUV[`\0a\x0C1a\x16]V[`\x0FTa\x0C>\x91\x90a\x1D\x9CV[\x90P\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0C\x9BWa\x0C\x7F3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\x0C\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1C\xEFV[a\x0C\xA5\x82\x82a\x16\xC2V[PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\r\x01Wa\x0C\xE53`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\r\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1C\xEFV[a\r\x0B3\x82a\x17\xB4V[PV[a\r$3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\r@W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1D`V[`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x81\x81R`\x04` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F6\xD2\x81&\xBE\xF2\x1AO7e\xD7\xFC\xB7\xC4\\\xEA\xD4c\xAELA\tN\xF3\xB7q\xED\xE5\x98TA\x03\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[a\r\xB63`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\r\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1D`V[\x80\x15a\x0E\x01W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80T`\x01`\xFF\x85\x16\x1B\x17\x90Ua\x0E'V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80T`\x01`\xFF\x85\x16\x1B\x19\x16\x90U[\x81`\xFF\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7FL\x9B\xDD\x0C\x8E\x07>\xB5\xED\xA2%\x0B\x18\xD8\xE5\x12\x1F\xF2{b\x06O\xBE\xEE\xEDHi\xBB\x99\xBC[\xF2\x83`@Qa\x0B\x0B\x91\x15\x15\x81R` \x01\x90V[`\0a\x0Era\x15\xB0V[`\0a\x0E\x7F\x85\x85\x85a\x15\xE1V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a\x1E-\x839\x81Q\x91R\x83`@Qa\x0E\xB4\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a\x1E\r\x839\x81Q\x91R\x85`@Qa\x0E\xEF\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x11` R`@\x81 Ta\n:\x90a\x0F\xAAV[a\x0F73`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\x0FSW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1D`V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x86\x16\x94\x85\x17\x90UQ\x7F\xA4\x90\x8E\x11\xA5\xF8\x95\xB1=QRl3\x1A\xC9<\xDD0\xE5\x97r6\x1C]\x07\x87N\xB3k\xFF e\x91\x90\xA3PPV[`\0a\n:a\x0F\xB7a\x0C'V[`\x10Ta\x0B.V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x10TWP`\x01T`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB7\0\x96\x13\x90a\x10\x13\x903\x900\x90`\x01`\x01`\xE0\x1B\x03\x19`\x005\x16\x90`\x04\x01a\x1D\xAFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x100W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10T\x91\x90a\x1D\xDCV[a\x10]W`\0\x80\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90\x7F\xA39o\xD7\xF6\xE0\xA2\x1BP\xE5\x08\x9D-\xA7\rZ\xC0\xA3\xBB\xBD\x1Faz\x93\xF14\xB7c\x89\x98\x01\x98\x90`\0\x90\xA3PV[`\0a\x10\xB3a\x15\xB0V[`\0a\x10\xBF\x84\x84a\x18\x1EV[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a\x1E-\x839\x81Q\x91R\x83`@Qa\x10\xF4\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x903\x90`\0\x80Q` a\x1E\r\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA3a\x113\x83a\x0F\xAAV[\x94\x93PPPPV[```\n\x80Ta\tF\x90a\x1D&V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x11\xA2Wa\x11\x863`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\x11\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1C\xEFV[a\x0C\xA5\x82\x82a\x17\xB4V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x12\x04Wa\x11\xE83`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\x12\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1C\xEFV[a\r\x0B3\x82a\x16\xC2V[`\0a\x12\x18a\x15\xB0V[`\x10T`\0\x03a\x12XW4`\x10\x81\x90U`\x0F\x81\x90U3`\0\x90\x81R`\x11` R`@\x81 \x80T\x90\x91\x90a\x12L\x90\x84\x90a\x1D\x9CV[\x90\x91UP4\x93\x92PPPV[`\0a\x12va\x12f`\x10T\x90V[a\x12na\x0C'V[4\x91\x90a\x15\x92V[\x90P4`\x0F`\0\x82\x82Ta\x12\x8A\x91\x90a\x1D\x9CV[\x92PP\x81\x90UP\x80`\x10`\0\x82\x82Ta\x12\xA3\x91\x90a\x1D\x9CV[\x90\x91UPP3`\0\x90\x81R`\x11` R`@\x81 \x80T\x83\x92\x90a\x12\xC7\x90\x84\x90a\x1D\x9CV[\x90\x91UP\x90\x93\x92PPPV[`\0a\x12\xDDa\x15\xB0V[`\0a\x12\xE8\x83a\x0B\x18V[\x90Pa\x12\xF4\x84\x82a\x18\x1EV[P`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x903\x90`\0\x80Q` a\x1E-\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA3`@Q\x81\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x903\x90`\0\x80Q` a\x1E\r\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA3P`\x01\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x02` R`@\x81 T\x90\x91\x16\x80\x15a\x13\xF7W`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB7\0\x96\x13\x90a\x13\xAE\x90\x88\x90\x88\x90\x88\x90`\x04\x01a\x1D\xAFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xEF\x91\x90a\x1D\xDCV[\x91PPa\x0B\xBBV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16`\0\x90\x81R`\x04` R`@\x90 T`\xFF\x16\x80a\x14PWP`\x01`\x01`\xE0\x1B\x03\x19\x83\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 T`\x01`\x01`\xA0\x1B\x03\x89\x16\x84R`\x03\x90\x92R\x90\x91 T\x16\x15\x15[\x95\x94PPPPPV[a\x14o3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x14\xD6V[a\x14\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x03\x90a\x1D`V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\x01T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x80\x15\x90a\x15`WP`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB7\0\x96\x13\x90a\x15\x1F\x90\x87\x900\x90\x88\x90`\x04\x01a\x1D\xAFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15`\x91\x90a\x1D\xDCV[\x80a\x113WP`\0T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x91\x16\x14\x91PP\x92\x91PPV[a\x15\x8D\x83\x83\x83`\x01a\x18{V[PPPV[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x15\xA9W`\0\x80\xFD[P\x91\x02\x04\x90V[`\0a\x15\xBAa\x16]V[\x90P\x80\x15a\x15\xDAW\x80`\x0F`\0\x82\x82Ta\x15\xD4\x91\x90a\x1D\x9CV[\x90\x91UPP[PB`\x0EUV[`\0\x80a\x15\xED\x83a\x0F\xAAV[\x90Pa\x15\xFA\x853\x83a\x19QV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x11` R`@\x81 \x80T\x85\x92\x90a\x16\"\x90\x84\x90a\x1D\xF9V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x11` R`@\x81 \x80T\x85\x92\x90a\x16O\x90\x84\x90a\x1D\x9CV[\x90\x91UP\x90\x95\x94PPPPPV[`\0`\rT`\0\x03a\x16oWP`\0\x90V[`\x0ETB\x10\x15a\x16\x7FWP`\0\x90V[`\0a\x16\x9Dc\x01\xE13\x80`\x0ETBa\x16\x97\x91\x90a\x1D\xF9V[\x90a\x19\xC9V[\x90P`\0a\x0B\xBBa\x16\xB9\x83`\rTa\x19\xDE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0FT\x90a\x19\xDEV[3`\0\x90\x81R`\x0C` R`@\x90 T`\xFF\x16a\x17+W`\x0BT\x81\x11\x15a\x17+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FMockLido: Invalid mint amount\0\0\0`D\x82\x01R`d\x01a\t\x03V[`\0a\x176`\x10T\x90V[`\0\x03a\x17DWP\x80a\x17PV[a\x17M\x82a\x0B\x18V[\x90P[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x11` R`@\x81 \x80T\x83\x92\x90a\x17x\x90\x84\x90a\x1D\x9CV[\x92PP\x81\x90UP\x81`\x0F`\0\x82\x82Ta\x17\x91\x91\x90a\x1D\x9CV[\x92PP\x81\x90UP\x80`\x10`\0\x82\x82Ta\x17\xAA\x91\x90a\x1D\x9CV[\x90\x91UPPPPPV[`\0a\x17\xBF\x82a\x0B\x18V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x11` R`@\x81 \x80T\x92\x93P\x83\x92\x90\x91\x90a\x17\xEC\x90\x84\x90a\x1D\xF9V[\x92PP\x81\x90UP\x81`\x0F`\0\x82\x82Ta\x18\x05\x91\x90a\x1D\xF9V[\x92PP\x81\x90UP\x80`\x10`\0\x82\x82Ta\x17\xAA\x91\x90a\x1D\xF9V[3`\0\x90\x81R`\x11` R`@\x81 \x80T\x83\x91\x90\x83\x90a\x18?\x90\x84\x90a\x1D\xF9V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x11` R`@\x81 \x80T\x84\x92\x90a\x18l\x90\x84\x90a\x1D\x9CV[\x90\x91UPa\x0B\xBB\x90P\x82a\x0F\xAAV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x18\xA5W`@Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\t\x03V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x18\xCFW`@QcJ\x14\x06\xB1`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\t\x03V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x82\x90U\x80\x15a\x19KW\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x19B\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T`\0\x19\x81\x14a\x19KW\x81\x81\x10\x15a\x19\xBAW`@Qc}\xC7\xA0\xD9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\t\x03V[a\x19K\x84\x84\x84\x84\x03`\0a\x18{V[`\0a\x0B\xBB\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x15\x92V[`\0a\x0B\xBB\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x15\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\x0BW`\0\x80\xFD[\x80\x15\x15\x81\x14a\r\x0BW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x1A)W`\0\x80\xFD[\x825a\x1A4\x81a\x19\xF3V[\x91P` \x83\x015a\x1AD\x81a\x1A\x08V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x1AaW`\0\x80\xFD[\x815a\x0B\xBB\x81a\x19\xF3V[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x1A\x9AW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x1A~V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1A\xCDW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1A\xE7W`\0\x80\xFD[\x825a\x1A\xF2\x81a\x19\xF3V[\x94` \x93\x90\x93\x015\x93PPPV[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1B\x18W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1B/W`\0\x80\xFD[a\x0B\xBB\x82a\x1B\0V[\x805`\xFF\x81\x16\x81\x14a\x1B\x18W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1B^W`\0\x80\xFD[a\x1Bg\x84a\x1B8V[\x92Pa\x1Bu` \x85\x01a\x1B\0V[\x91P`@\x84\x015a\x1B\x85\x81a\x1A\x08V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1B\xA5W`\0\x80\xFD[\x835a\x1B\xB0\x81a\x19\xF3V[\x92P` \x84\x015a\x1B\xC0\x81a\x19\xF3V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x1B\xE4W`\0\x80\xFD[a\x1A4\x83a\x1B\0V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C\x02W`\0\x80\xFD[\x835a\x1C\r\x81a\x19\xF3V[\x92Pa\x1Bu` \x85\x01a\x1B8V[`\0\x80`@\x83\x85\x03\x12\x15a\x1C.W`\0\x80\xFD[\x825a\x1C9\x81a\x19\xF3V[\x91P` \x83\x015a\x1AD\x81a\x19\xF3V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C^W`\0\x80\xFD[\x835a\x1Ci\x81a\x19\xF3V[\x92P` \x84\x015a\x1Cy\x81a\x19\xF3V[\x91Pa\x1C\x87`@\x85\x01a\x1B\0V[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\xA3W`\0\x80\xFD[a\x1C\xAC\x83a\x1B8V[\x91Pa\x1C\xBA` \x84\x01a\x1B\0V[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\xD6W`\0\x80\xFD[\x825a\x1C\xE1\x81a\x19\xF3V[\x91Pa\x1C\xBA` \x84\x01a\x1B8V[` \x80\x82R`\x18\x90\x82\x01R\x7FMockLido: not authorized\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1D:W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1DZWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\n:Wa\n:a\x1D\x86V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1D\xEEW`\0\x80\xFD[\x81Qa\x0B\xBB\x81a\x1A\x08V[\x81\x81\x03\x81\x81\x11\x15a\n:Wa\n:a\x1D\x86V\xFE\x9D\x9C\x90\x92\x96\xD9\xC6tE\x1C\x0C$\xF0,\xB6I\x81\xEB;r\x7F\x99\x86Y9\x19/\x88\nu]\xCB\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xA5S+<\x02\x16Q\xB8:\x87\xF9D\x0C\xB4q)-\x7F~I\x97p\x8F\xC0\xA7\xEE\xF4\x107\xAC\xB8\x8BdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKLIDO_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockLido<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockLido<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockLido<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockLido<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockLido<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockLido)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockLido<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKLIDO_ABI.clone(),
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
                MOCKLIDO_ABI.clone(),
                MOCKLIDO_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
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
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
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
        pub fn burn_with_target(
            &self,
            target: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 194, 159, 172], (target, amount))
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
        ///Calls the contract's `getBufferedEther` (0x47b714e0) function
        pub fn get_buffered_ether(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([71, 183, 20, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPooledEthByShares` (0x7a28fb88) function
        pub fn get_pooled_eth_by_shares(
            &self,
            shares_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([122, 40, 251, 136], shares_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRate` (0x679aefce) function
        pub fn get_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([103, 154, 239, 206], ())
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
        ///Calls the contract's `getSharesByPooledEth` (0x19208451) function
        pub fn get_shares_by_pooled_eth(
            &self,
            eth_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([25, 32, 132, 81], eth_amount)
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
        ///Calls the contract's `getTotalPooledEther` (0x37cfdaca) function
        pub fn get_total_pooled_ether(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([55, 207, 218, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalShares` (0xd5002f2e) function
        pub fn get_total_shares(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([213, 0, 47, 46], ())
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
        pub fn mint_with_recipient(
            &self,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (recipient, amount))
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
        ///Calls the contract's `setRate` (0x34fcf437) function
        pub fn set_rate(
            &self,
            rate: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 252, 244, 55], rate)
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
        ///Calls the contract's `sharesOf` (0xf5eb42dc) function
        pub fn shares_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([245, 235, 66, 220], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submit` (0xa1903eab) function
        pub fn submit(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([161, 144, 62, 171], p0)
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
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
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
        ///Calls the contract's `transferShares` (0x8fcb4e5b) function
        pub fn transfer_shares(
            &self,
            recipient: ::ethers::core::types::Address,
            shares_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([143, 203, 78, 91], (recipient, shares_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferSharesFrom` (0x6d780459) function
        pub fn transfer_shares_from(
            &self,
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            shares_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([109, 120, 4, 89], (sender, recipient, shares_amount))
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
        ///Gets the contract's `TransferShares` event
        pub fn transfer_shares_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferSharesFilter,
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
            MockLidoEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockLido<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ERC20InsufficientAllowance` with signature `ERC20InsufficientAllowance(address,uint256,uint256)` and selector `0xfb8f41b2`
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
        name = "ERC20InsufficientAllowance",
        abi = "ERC20InsufficientAllowance(address,uint256,uint256)"
    )]
    pub struct ERC20InsufficientAllowance {
        pub spender: ::ethers::core::types::Address,
        pub allowance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC20InsufficientBalance` with signature `ERC20InsufficientBalance(address,uint256,uint256)` and selector `0xe450d38c`
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
        name = "ERC20InsufficientBalance",
        abi = "ERC20InsufficientBalance(address,uint256,uint256)"
    )]
    pub struct ERC20InsufficientBalance {
        pub sender: ::ethers::core::types::Address,
        pub balance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC20InvalidApprover` with signature `ERC20InvalidApprover(address)` and selector `0xe602df05`
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
    #[etherror(name = "ERC20InvalidApprover", abi = "ERC20InvalidApprover(address)")]
    pub struct ERC20InvalidApprover {
        pub approver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidReceiver` with signature `ERC20InvalidReceiver(address)` and selector `0xec442f05`
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
    #[etherror(name = "ERC20InvalidReceiver", abi = "ERC20InvalidReceiver(address)")]
    pub struct ERC20InvalidReceiver {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidSender` with signature `ERC20InvalidSender(address)` and selector `0x96c6fd1e`
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
    #[etherror(name = "ERC20InvalidSender", abi = "ERC20InvalidSender(address)")]
    pub struct ERC20InvalidSender {
        pub sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidSpender` with signature `ERC20InvalidSpender(address)` and selector `0x94280d62`
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
    #[etherror(name = "ERC20InvalidSpender", abi = "ERC20InvalidSpender(address)")]
    pub struct ERC20InvalidSpender {
        pub spender: ::ethers::core::types::Address,
    }
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
    pub enum MockLidoErrors {
        ERC20InsufficientAllowance(ERC20InsufficientAllowance),
        ERC20InsufficientBalance(ERC20InsufficientBalance),
        ERC20InvalidApprover(ERC20InvalidApprover),
        ERC20InvalidReceiver(ERC20InvalidReceiver),
        ERC20InvalidSender(ERC20InvalidSender),
        ERC20InvalidSpender(ERC20InvalidSpender),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockLidoErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ERC20InsufficientAllowance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InsufficientAllowance(decoded));
            }
            if let Ok(decoded) = <ERC20InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InsufficientBalance(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidApprover as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidApprover(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidReceiver as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidReceiver(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidSender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidSender(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidSpender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidSpender(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockLidoErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ERC20InsufficientAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidSpender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MockLidoErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ERC20InsufficientAllowance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidApprover as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidReceiver as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidSpender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MockLidoErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ERC20InsufficientAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidApprover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidReceiver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidSpender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MockLidoErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientAllowance> for MockLidoErrors {
        fn from(value: ERC20InsufficientAllowance) -> Self {
            Self::ERC20InsufficientAllowance(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientBalance> for MockLidoErrors {
        fn from(value: ERC20InsufficientBalance) -> Self {
            Self::ERC20InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidApprover> for MockLidoErrors {
        fn from(value: ERC20InvalidApprover) -> Self {
            Self::ERC20InvalidApprover(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidReceiver> for MockLidoErrors {
        fn from(value: ERC20InvalidReceiver) -> Self {
            Self::ERC20InvalidReceiver(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSender> for MockLidoErrors {
        fn from(value: ERC20InvalidSender) -> Self {
            Self::ERC20InvalidSender(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSpender> for MockLidoErrors {
        fn from(value: ERC20InvalidSpender) -> Self {
            Self::ERC20InvalidSpender(value)
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
    #[ethevent(name = "TransferShares", abi = "TransferShares(address,address,uint256)")]
    pub struct TransferSharesFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub shares_value: ::ethers::core::types::U256,
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
    pub enum MockLidoEvents {
        ApprovalFilter(ApprovalFilter),
        AuthorityUpdatedFilter(AuthorityUpdatedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PublicCapabilityUpdatedFilter(PublicCapabilityUpdatedFilter),
        RoleCapabilityUpdatedFilter(RoleCapabilityUpdatedFilter),
        TargetCustomAuthorityUpdatedFilter(TargetCustomAuthorityUpdatedFilter),
        TransferFilter(TransferFilter),
        TransferSharesFilter(TransferSharesFilter),
        UserRoleUpdatedFilter(UserRoleUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockLidoEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockLidoEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = AuthorityUpdatedFilter::decode_log(log) {
                return Ok(MockLidoEvents::AuthorityUpdatedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MockLidoEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PublicCapabilityUpdatedFilter::decode_log(log) {
                return Ok(MockLidoEvents::PublicCapabilityUpdatedFilter(decoded));
            }
            if let Ok(decoded) = RoleCapabilityUpdatedFilter::decode_log(log) {
                return Ok(MockLidoEvents::RoleCapabilityUpdatedFilter(decoded));
            }
            if let Ok(decoded) = TargetCustomAuthorityUpdatedFilter::decode_log(log) {
                return Ok(MockLidoEvents::TargetCustomAuthorityUpdatedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockLidoEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = TransferSharesFilter::decode_log(log) {
                return Ok(MockLidoEvents::TransferSharesFilter(decoded));
            }
            if let Ok(decoded) = UserRoleUpdatedFilter::decode_log(log) {
                return Ok(MockLidoEvents::UserRoleUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockLidoEvents {
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
                Self::TransferSharesFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UserRoleUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for MockLidoEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<AuthorityUpdatedFilter> for MockLidoEvents {
        fn from(value: AuthorityUpdatedFilter) -> Self {
            Self::AuthorityUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for MockLidoEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PublicCapabilityUpdatedFilter> for MockLidoEvents {
        fn from(value: PublicCapabilityUpdatedFilter) -> Self {
            Self::PublicCapabilityUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<RoleCapabilityUpdatedFilter> for MockLidoEvents {
        fn from(value: RoleCapabilityUpdatedFilter) -> Self {
            Self::RoleCapabilityUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TargetCustomAuthorityUpdatedFilter> for MockLidoEvents {
        fn from(value: TargetCustomAuthorityUpdatedFilter) -> Self {
            Self::TargetCustomAuthorityUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for MockLidoEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<TransferSharesFilter> for MockLidoEvents {
        fn from(value: TransferSharesFilter) -> Self {
            Self::TransferSharesFilter(value)
        }
    }
    impl ::core::convert::From<UserRoleUpdatedFilter> for MockLidoEvents {
        fn from(value: UserRoleUpdatedFilter) -> Self {
            Self::UserRoleUpdatedFilter(value)
        }
    }
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
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
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
        pub value: ::ethers::core::types::U256,
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
    pub struct BalanceOfCall {
        pub owner: ::ethers::core::types::Address,
    }
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
    pub struct BurnWithTargetCall {
        pub target: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `getBufferedEther` function with signature `getBufferedEther()` and selector `0x47b714e0`
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
    #[ethcall(name = "getBufferedEther", abi = "getBufferedEther()")]
    pub struct GetBufferedEtherCall;
    ///Container type for all input parameters for the `getPooledEthByShares` function with signature `getPooledEthByShares(uint256)` and selector `0x7a28fb88`
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
    #[ethcall(name = "getPooledEthByShares", abi = "getPooledEthByShares(uint256)")]
    pub struct GetPooledEthBySharesCall {
        pub shares_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRate` function with signature `getRate()` and selector `0x679aefce`
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
    #[ethcall(name = "getRate", abi = "getRate()")]
    pub struct GetRateCall;
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
    ///Container type for all input parameters for the `getSharesByPooledEth` function with signature `getSharesByPooledEth(uint256)` and selector `0x19208451`
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
    #[ethcall(name = "getSharesByPooledEth", abi = "getSharesByPooledEth(uint256)")]
    pub struct GetSharesByPooledEthCall {
        pub eth_amount: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `getTotalPooledEther` function with signature `getTotalPooledEther()` and selector `0x37cfdaca`
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
    #[ethcall(name = "getTotalPooledEther", abi = "getTotalPooledEther()")]
    pub struct GetTotalPooledEtherCall;
    ///Container type for all input parameters for the `getTotalShares` function with signature `getTotalShares()` and selector `0xd5002f2e`
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
    #[ethcall(name = "getTotalShares", abi = "getTotalShares()")]
    pub struct GetTotalSharesCall;
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
    pub struct MintWithRecipientCall {
        pub recipient: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `setRate` function with signature `setRate(uint256)` and selector `0x34fcf437`
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
    #[ethcall(name = "setRate", abi = "setRate(uint256)")]
    pub struct SetRateCall {
        pub rate: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `sharesOf` function with signature `sharesOf(address)` and selector `0xf5eb42dc`
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
    #[ethcall(name = "sharesOf", abi = "sharesOf(address)")]
    pub struct SharesOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `submit` function with signature `submit(address)` and selector `0xa1903eab`
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
    #[ethcall(name = "submit", abi = "submit(address)")]
    pub struct SubmitCall(pub ::ethers::core::types::Address);
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
        pub recipient: ::ethers::core::types::Address,
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
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `transferShares` function with signature `transferShares(address,uint256)` and selector `0x8fcb4e5b`
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
    #[ethcall(name = "transferShares", abi = "transferShares(address,uint256)")]
    pub struct TransferSharesCall {
        pub recipient: ::ethers::core::types::Address,
        pub shares_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferSharesFrom` function with signature `transferSharesFrom(address,address,uint256)` and selector `0x6d780459`
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
        name = "transferSharesFrom",
        abi = "transferSharesFrom(address,address,uint256)"
    )]
    pub struct TransferSharesFromCall {
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub shares_amount: ::ethers::core::types::U256,
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
    pub enum MockLidoCalls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        Authority(AuthorityCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        BurnWithTarget(BurnWithTargetCall),
        CanCall(CanCallCall),
        Decimals(DecimalsCall),
        DoesRoleHaveCapability(DoesRoleHaveCapabilityCall),
        DoesUserHaveRole(DoesUserHaveRoleCall),
        GetBufferedEther(GetBufferedEtherCall),
        GetPooledEthByShares(GetPooledEthBySharesCall),
        GetRate(GetRateCall),
        GetRolesWithCapability(GetRolesWithCapabilityCall),
        GetSharesByPooledEth(GetSharesByPooledEthCall),
        GetTargetCustomAuthority(GetTargetCustomAuthorityCall),
        GetTotalPooledEther(GetTotalPooledEtherCall),
        GetTotalShares(GetTotalSharesCall),
        GetUserRoles(GetUserRolesCall),
        IsCapabilityPublic(IsCapabilityPublicCall),
        IsCompetitionMode(IsCompetitionModeCall),
        IsUnrestricted(IsUnrestrictedCall),
        MaxMintAmount(MaxMintAmountCall),
        MintWithRecipient(MintWithRecipientCall),
        Mint(MintCall),
        Name(NameCall),
        Owner(OwnerCall),
        SetAuthority(SetAuthorityCall),
        SetMaxMintAmount(SetMaxMintAmountCall),
        SetPublicCapability(SetPublicCapabilityCall),
        SetRate(SetRateCall),
        SetRoleCapability(SetRoleCapabilityCall),
        SetTargetCustomAuthority(SetTargetCustomAuthorityCall),
        SetUnrestrictedMintStatus(SetUnrestrictedMintStatusCall),
        SetUserRole(SetUserRoleCall),
        SharesOf(SharesOfCall),
        Submit(SubmitCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        TransferShares(TransferSharesCall),
        TransferSharesFrom(TransferSharesFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockLidoCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
            if let Ok(decoded) = <BurnWithTargetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BurnWithTarget(decoded));
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
            if let Ok(decoded) = <GetBufferedEtherCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBufferedEther(decoded));
            }
            if let Ok(decoded) = <GetPooledEthBySharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPooledEthByShares(decoded));
            }
            if let Ok(decoded) = <GetRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRate(decoded));
            }
            if let Ok(decoded) = <GetRolesWithCapabilityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRolesWithCapability(decoded));
            }
            if let Ok(decoded) = <GetSharesByPooledEthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSharesByPooledEth(decoded));
            }
            if let Ok(decoded) = <GetTargetCustomAuthorityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTargetCustomAuthority(decoded));
            }
            if let Ok(decoded) = <GetTotalPooledEtherCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalPooledEther(decoded));
            }
            if let Ok(decoded) = <GetTotalSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalShares(decoded));
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
            if let Ok(decoded) = <MintWithRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintWithRecipient(decoded));
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
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
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
            if let Ok(decoded) = <SetRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetRate(decoded));
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
            if let Ok(decoded) = <SharesOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SharesOf(decoded));
            }
            if let Ok(decoded) = <SubmitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Submit(decoded));
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
            if let Ok(decoded) = <TransferSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferShares(decoded));
            }
            if let Ok(decoded) = <TransferSharesFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferSharesFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockLidoCalls {
        fn encode(self) -> Vec<u8> {
            match self {
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
                Self::BurnWithTarget(element) => {
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
                Self::GetBufferedEther(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPooledEthByShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRolesWithCapability(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSharesByPooledEth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTargetCustomAuthority(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalPooledEther(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalShares(element) => {
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
                Self::MintWithRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAuthority(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMaxMintAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPublicCapability(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRate(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::SharesOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Submit(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::TransferShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferSharesFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockLidoCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::Authority(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnWithTarget(element) => ::core::fmt::Display::fmt(element, f),
                Self::CanCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::DoesRoleHaveCapability(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DoesUserHaveRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBufferedEther(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPooledEthByShares(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRolesWithCapability(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSharesByPooledEth(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTargetCustomAuthority(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalPooledEther(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUserRoles(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsCapabilityPublic(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsCompetitionMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsUnrestricted(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxMintAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintWithRecipient(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAuthority(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMaxMintAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPublicCapability(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRoleCapability(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTargetCustomAuthority(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetUnrestrictedMintStatus(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetUserRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SharesOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Submit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferSharesFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AllowanceCall> for MockLidoCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for MockLidoCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<AuthorityCall> for MockLidoCalls {
        fn from(value: AuthorityCall) -> Self {
            Self::Authority(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MockLidoCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for MockLidoCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<BurnWithTargetCall> for MockLidoCalls {
        fn from(value: BurnWithTargetCall) -> Self {
            Self::BurnWithTarget(value)
        }
    }
    impl ::core::convert::From<CanCallCall> for MockLidoCalls {
        fn from(value: CanCallCall) -> Self {
            Self::CanCall(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for MockLidoCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DoesRoleHaveCapabilityCall> for MockLidoCalls {
        fn from(value: DoesRoleHaveCapabilityCall) -> Self {
            Self::DoesRoleHaveCapability(value)
        }
    }
    impl ::core::convert::From<DoesUserHaveRoleCall> for MockLidoCalls {
        fn from(value: DoesUserHaveRoleCall) -> Self {
            Self::DoesUserHaveRole(value)
        }
    }
    impl ::core::convert::From<GetBufferedEtherCall> for MockLidoCalls {
        fn from(value: GetBufferedEtherCall) -> Self {
            Self::GetBufferedEther(value)
        }
    }
    impl ::core::convert::From<GetPooledEthBySharesCall> for MockLidoCalls {
        fn from(value: GetPooledEthBySharesCall) -> Self {
            Self::GetPooledEthByShares(value)
        }
    }
    impl ::core::convert::From<GetRateCall> for MockLidoCalls {
        fn from(value: GetRateCall) -> Self {
            Self::GetRate(value)
        }
    }
    impl ::core::convert::From<GetRolesWithCapabilityCall> for MockLidoCalls {
        fn from(value: GetRolesWithCapabilityCall) -> Self {
            Self::GetRolesWithCapability(value)
        }
    }
    impl ::core::convert::From<GetSharesByPooledEthCall> for MockLidoCalls {
        fn from(value: GetSharesByPooledEthCall) -> Self {
            Self::GetSharesByPooledEth(value)
        }
    }
    impl ::core::convert::From<GetTargetCustomAuthorityCall> for MockLidoCalls {
        fn from(value: GetTargetCustomAuthorityCall) -> Self {
            Self::GetTargetCustomAuthority(value)
        }
    }
    impl ::core::convert::From<GetTotalPooledEtherCall> for MockLidoCalls {
        fn from(value: GetTotalPooledEtherCall) -> Self {
            Self::GetTotalPooledEther(value)
        }
    }
    impl ::core::convert::From<GetTotalSharesCall> for MockLidoCalls {
        fn from(value: GetTotalSharesCall) -> Self {
            Self::GetTotalShares(value)
        }
    }
    impl ::core::convert::From<GetUserRolesCall> for MockLidoCalls {
        fn from(value: GetUserRolesCall) -> Self {
            Self::GetUserRoles(value)
        }
    }
    impl ::core::convert::From<IsCapabilityPublicCall> for MockLidoCalls {
        fn from(value: IsCapabilityPublicCall) -> Self {
            Self::IsCapabilityPublic(value)
        }
    }
    impl ::core::convert::From<IsCompetitionModeCall> for MockLidoCalls {
        fn from(value: IsCompetitionModeCall) -> Self {
            Self::IsCompetitionMode(value)
        }
    }
    impl ::core::convert::From<IsUnrestrictedCall> for MockLidoCalls {
        fn from(value: IsUnrestrictedCall) -> Self {
            Self::IsUnrestricted(value)
        }
    }
    impl ::core::convert::From<MaxMintAmountCall> for MockLidoCalls {
        fn from(value: MaxMintAmountCall) -> Self {
            Self::MaxMintAmount(value)
        }
    }
    impl ::core::convert::From<MintWithRecipientCall> for MockLidoCalls {
        fn from(value: MintWithRecipientCall) -> Self {
            Self::MintWithRecipient(value)
        }
    }
    impl ::core::convert::From<MintCall> for MockLidoCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for MockLidoCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MockLidoCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<SetAuthorityCall> for MockLidoCalls {
        fn from(value: SetAuthorityCall) -> Self {
            Self::SetAuthority(value)
        }
    }
    impl ::core::convert::From<SetMaxMintAmountCall> for MockLidoCalls {
        fn from(value: SetMaxMintAmountCall) -> Self {
            Self::SetMaxMintAmount(value)
        }
    }
    impl ::core::convert::From<SetPublicCapabilityCall> for MockLidoCalls {
        fn from(value: SetPublicCapabilityCall) -> Self {
            Self::SetPublicCapability(value)
        }
    }
    impl ::core::convert::From<SetRateCall> for MockLidoCalls {
        fn from(value: SetRateCall) -> Self {
            Self::SetRate(value)
        }
    }
    impl ::core::convert::From<SetRoleCapabilityCall> for MockLidoCalls {
        fn from(value: SetRoleCapabilityCall) -> Self {
            Self::SetRoleCapability(value)
        }
    }
    impl ::core::convert::From<SetTargetCustomAuthorityCall> for MockLidoCalls {
        fn from(value: SetTargetCustomAuthorityCall) -> Self {
            Self::SetTargetCustomAuthority(value)
        }
    }
    impl ::core::convert::From<SetUnrestrictedMintStatusCall> for MockLidoCalls {
        fn from(value: SetUnrestrictedMintStatusCall) -> Self {
            Self::SetUnrestrictedMintStatus(value)
        }
    }
    impl ::core::convert::From<SetUserRoleCall> for MockLidoCalls {
        fn from(value: SetUserRoleCall) -> Self {
            Self::SetUserRole(value)
        }
    }
    impl ::core::convert::From<SharesOfCall> for MockLidoCalls {
        fn from(value: SharesOfCall) -> Self {
            Self::SharesOf(value)
        }
    }
    impl ::core::convert::From<SubmitCall> for MockLidoCalls {
        fn from(value: SubmitCall) -> Self {
            Self::Submit(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for MockLidoCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for MockLidoCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for MockLidoCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for MockLidoCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for MockLidoCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TransferSharesCall> for MockLidoCalls {
        fn from(value: TransferSharesCall) -> Self {
            Self::TransferShares(value)
        }
    }
    impl ::core::convert::From<TransferSharesFromCall> for MockLidoCalls {
        fn from(value: TransferSharesFromCall) -> Self {
            Self::TransferSharesFrom(value)
        }
    }
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
    ///Container type for all return fields from the `getBufferedEther` function with signature `getBufferedEther()` and selector `0x47b714e0`
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
    pub struct GetBufferedEtherReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPooledEthByShares` function with signature `getPooledEthByShares(uint256)` and selector `0x7a28fb88`
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
    pub struct GetPooledEthBySharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRate` function with signature `getRate()` and selector `0x679aefce`
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
    pub struct GetRateReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `getSharesByPooledEth` function with signature `getSharesByPooledEth(uint256)` and selector `0x19208451`
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
    pub struct GetSharesByPooledEthReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `getTotalPooledEther` function with signature `getTotalPooledEther()` and selector `0x37cfdaca`
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
    pub struct GetTotalPooledEtherReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTotalShares` function with signature `getTotalShares()` and selector `0xd5002f2e`
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
    pub struct GetTotalSharesReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `sharesOf` function with signature `sharesOf(address)` and selector `0xf5eb42dc`
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
    pub struct SharesOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `submit` function with signature `submit(address)` and selector `0xa1903eab`
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
    pub struct SubmitReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `transferShares` function with signature `transferShares(address,uint256)` and selector `0x8fcb4e5b`
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
    pub struct TransferSharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transferSharesFrom` function with signature `transferSharesFrom(address,address,uint256)` and selector `0x6d780459`
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
    pub struct TransferSharesFromReturn(pub ::ethers::core::types::U256);
}
