pub use mock_erc4626::*;
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
pub mod mock_erc4626 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_asset"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ERC20Mintable"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_symbol"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
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
                    ::std::borrow::ToOwned::to_owned("asset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("asset"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("convertToAssets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("convertToAssets"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned("convertToShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("convertToShares"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_receiver"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("maxDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxDeposit"),
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
                    ::std::borrow::ToOwned::to_owned("maxMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxMint"),
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
                    ::std::borrow::ToOwned::to_owned("maxRedeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxRedeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("maxWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxWithdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_receiver"),
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
                    ::std::borrow::ToOwned::to_owned("previewDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("previewDeposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                    ::std::borrow::ToOwned::to_owned("previewMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("previewMint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned("previewRedeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("previewRedeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned("previewWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("previewWithdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                    ::std::borrow::ToOwned::to_owned("redeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("totalAssets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalAssets"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalAssets"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                (
                    ::std::borrow::ToOwned::to_owned("Withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKERC4626_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01 `@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\x000\x1B8\x03\x80b\x000\x1B\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x03\x14V[\x820\x81\x81\x8A\x8A\x8A\x81\x81\x84`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xA3\x91\x90b\0\x03\xD7V[`\0b\0\0\xB1\x84\x82b\0\x04\x92V[P`\x01b\0\0\xC0\x83\x82b\0\x04\x92V[P`\xFF\x81\x16`\x80RF`\xA0Rb\0\0\xD6b\0\x01\x9AV[`\xC0RPPP`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\xE0RPP`\x06\x80T\x84\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x81\x17\x90\x92U`\x07\x80T\x93\x85\x16\x93\x90\x91\x16\x92\x90\x92\x17\x90\x91U`@Q3\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x903\x90\x7F\xA39o\xD7\xF6\xE0\xA2\x1BP\xE5\x08\x9D-\xA7\rZ\xC0\xA3\xBB\xBD\x1Faz\x93\xF14\xB7c\x89\x98\x01\x98\x90`\0\x90\xA3PPP`\x0C\x94\x90\x94UB`\rU\x90\x15\x15a\x01\0R`\x0EUPb\0\x05\xDC\x93PPPPV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qb\0\x01\xCE\x91\x90b\0\x05^V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02LW`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12b\0\x02wW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x02\x94Wb\0\x02\x94b\0\x02OV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x02\xBFWb\0\x02\xBFb\0\x02OV[\x81`@R\x83\x81R` \x92P\x86\x83\x85\x88\x01\x01\x11\x15b\0\x02\xDCW`\0\x80\xFD[`\0\x91P[\x83\x82\x10\x15b\0\x03\0W\x85\x82\x01\x83\x01Q\x81\x83\x01\x84\x01R\x90\x82\x01\x90b\0\x02\xE1V[`\0\x93\x81\x01\x90\x92\x01\x92\x90\x92R\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15b\0\x030W`\0\x80\xFD[\x87Qb\0\x03=\x81b\0\x026V[` \x89\x01Q\x90\x97P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x03[W`\0\x80\xFD[b\0\x03i\x8B\x83\x8C\x01b\0\x02eV[\x97P`@\x8A\x01Q\x91P\x80\x82\x11\x15b\0\x03\x80W`\0\x80\xFD[Pb\0\x03\x8F\x8A\x82\x8B\x01b\0\x02eV[\x95PP``\x88\x01Q\x93P`\x80\x88\x01Qb\0\x03\xA9\x81b\0\x026V[`\xA0\x89\x01Q\x90\x93P\x80\x15\x15\x81\x14b\0\x03\xC0W`\0\x80\xFD[\x80\x92PP`\xC0\x88\x01Q\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0` \x82\x84\x03\x12\x15b\0\x03\xEAW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14b\0\x03\xFCW`\0\x80\xFD[\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x04\x18W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x049WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x04\x8DW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x04hWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x04\x89W\x82\x81U`\x01\x01b\0\x04tV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x04\xAEWb\0\x04\xAEb\0\x02OV[b\0\x04\xC6\x81b\0\x04\xBF\x84Tb\0\x04\x03V[\x84b\0\x04?V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x04\xFEW`\0\x84\x15b\0\x04\xE5WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x04\x89V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x05/W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x05\x0EV[P\x85\x82\x10\x15b\0\x05NW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x80\x83Tb\0\x05n\x81b\0\x04\x03V[`\x01\x82\x81\x16\x80\x15b\0\x05\x89W`\x01\x81\x14b\0\x05\x9FWb\0\x05\xD0V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pb\0\x05\xD0V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15b\0\x05\xC7W\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01b\0\x05\xACV[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa)ub\0\x06\xA6`\09`\0\x81\x81a\x05\xD8\x01R\x81\x81a\t6\x01R\x81\x81a\n}\x01R\x81\x81a\r\x1F\x01R\x81\x81a\r\xDA\x01R\x81\x81a\x0F1\x01R\x81\x81a\x13\xC8\x01Ra\x14\xB2\x01R`\0\x81\x81a\x04\xD7\x01R\x81\x81a\x08\xB2\x01R\x81\x81a\x0E\x9B\x01R\x81\x81a\x0F\x89\x01R\x81\x81a\x13(\x01R\x81\x81a\x14 \x01R\x81\x81a\x15s\x01R\x81\x81a\x1B<\x01R\x81\x81a\x1C\xCB\x01R\x81\x81a\x1E\xE5\x01R\x81\x81a\x1F}\x01R\x81\x81a \xCF\x01Ra\"\x11\x01R`\0a\r\xB6\x01R`\0a\r\x86\x01R`\0a\x04\x83\x01Ra)u`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03]W`\x005`\xE0\x1C\x80cz\x8Cc\xB5\x11a\x01\xD3W\x80c\xBA\x08vR\x11a\x01\x04W\x80c\xD9\x05w~\x11a\0\xA2W\x80c\xEA|\xA2v\x11a\0|W\x80c\xEA|\xA2v\x14a\x08\x15W\x80c\xED\r\x0E\xFB\x14a\x08LW\x80c\xEF\x8B0\xF7\x14a\x08lW\x80c\xF2\xFD\xE3\x8B\x14a\x08\x7FW`\0\x80\xFD[\x80c\xD9\x05w~\x14a\x07\x8BW\x80c\xDDb\xED>\x14a\x07\xB4W\x80c\xE6\x88t{\x14a\x07\xDFW`\0\x80\xFD[\x80c\xC6=u\xB6\x11a\0\xDEW\x80c\xC6=u\xB6\x14a\x05\x11W\x80c\xC6\xE6\xF5\x92\x14a\x07RW\x80c\xCE\x96\xCBw\x14a\x07eW\x80c\xD5\x05\xAC\xCF\x14a\x07xW`\0\x80\xFD[\x80c\xBA\x08vR\x14a\x07\x03W\x80c\xBF~!O\x14a\x07\x16W\x80c\xC5:9\x85\x14a\x07)W`\0\x80\xFD[\x80c\x9D\xC2\x9F\xAC\x11a\x01qW\x80c\xAE\xD3\x07w\x11a\x01KW\x80c\xAE\xD3\x07w\x14a\x06\xA7W\x80c\xB3\xD7\xF6\xB9\x14a\x06\xCAW\x80c\xB4`\xAF\x94\x14a\x06\xDDW\x80c\xB7\0\x96\x13\x14a\x06\xF0W`\0\x80\xFD[\x80c\x9D\xC2\x9F\xAC\x14a\x06nW\x80c\xA0q-h\x14a\x06\x81W\x80c\xA9\x05\x9C\xBB\x14a\x06\x94W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\x01\xADW\x80c\x8D\xA5\xCB[\x14a\x06-W\x80c\x94`XW\x14a\x06@W\x80c\x94\xBF\x80M\x14a\x06SW\x80c\x95\xD8\x9BA\x14a\x06fW`\0\x80\xFD[\x80cz\x8Cc\xB5\x14a\x05\xD3W\x80cz\x9E^K\x14a\x05\xFAW\x80c~\xCE\xBE\0\x14a\x06\rW`\0\x80\xFD[\x80c1<\xE5g\x11a\x02\xADW\x80cKQY\xDA\x11a\x02KW\x80cg\xAF\xF4\x84\x11a\x02%W\x80cg\xAF\xF4\x84\x14a\x05zW\x80cnU?e\x14a\x05\x8DW\x80cp\xA0\x821\x14a\x05\xA0W\x80cr\x8B\x95+\x14a\x05\xC0W`\0\x80\xFD[\x80cKQY\xDA\x14a\x05LW\x80cL\xDA\xD5\x06\x14a\x05_W\x80cg\x9A\xEF\xCE\x14a\x05rW`\0\x80\xFD[\x80c8\xD5.\x0F\x11a\x02\x87W\x80c8\xD5.\x0F\x14a\x04\xD2W\x80c@-&}\x14a\x05\x11W\x80c@\xC1\x0F\x19\x14a\x05&W\x80cB\x96lh\x14a\x059W`\0\x80\xFD[\x80c1<\xE5g\x14a\x04~W\x80c4\xFC\xF47\x14a\x04\xB7W\x80c6D\xE5\x15\x14a\x04\xCAW`\0\x80\xFD[\x80c\t^\xA7\xB3\x11a\x03\x1AW\x80c\x0E\xA9\xB7[\x11a\x02\xF4W\x80c\x0E\xA9\xB7[\x14a\x04FW\x80c\x18\x16\r\xDD\x14a\x04YW\x80c#\x9Cp\xAE\x14a\x04bW\x80c#\xB8r\xDD\x14a\x04kW`\0\x80\xFD[\x80c\t^\xA7\xB3\x14a\x03\xEDW\x80c\n(\xA4w\x14a\x04\x10W\x80c\x0B\xAD\xE8\xA4\x14a\x04#W`\0\x80\xFD[\x80c\x01\xE1\xD1\x14\x14a\x03bW\x80c\x05\xF0Z\x94\x14a\x03}W\x80c\x06\xA3j\xEE\x14a\x03\x92W\x80c\x06\xFD\xDE\x03\x14a\x03\xB2W\x80c\x07\xA2\xD1:\x14a\x03\xC7W\x80c\x08\x8AN\xD0\x14a\x03\xDAW[`\0\x80\xFD[a\x03ja\x08\x92V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\x90a\x03\x8B6`\x04a#\xA8V[a\t4V[\0[a\x03ja\x03\xA06`\x04a#\xE1V[`\t` R`\0\x90\x81R`@\x90 T\x81V[a\x03\xBAa\t\xC0V[`@Qa\x03t\x91\x90a#\xFEV[a\x03ja\x03\xD56`\x04a$LV[a\nNV[a\x03\x90a\x03\xE86`\x04a$LV[a\n{V[a\x04\0a\x03\xFB6`\x04a$eV[a\n\xD8V[`@Q\x90\x15\x15\x81R` \x01a\x03tV[a\x03ja\x04\x1E6`\x04a$LV[a\x0BEV[a\x04\0a\x0416`\x04a$\xAEV[`\n` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03\x90a\x04T6`\x04a$\xDAV[a\x0BeV[a\x03j`\x02T\x81V[a\x03j`\x0ET\x81V[a\x04\0a\x04y6`\x04a%!V[a\x0C=V[a\x04\xA5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x03tV[a\x03\x90a\x04\xC56`\x04a$LV[a\r\x1DV[a\x03ja\r\x82V[a\x04\xF9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03tV[a\x03ja\x05\x1F6`\x04a#\xE1V[P`\0\x19\x90V[a\x03\x90a\x0546`\x04a$eV[a\r\xD8V[a\x03\x90a\x05G6`\x04a$LV[a\x0F/V[a\x03\x90a\x05Z6`\x04a%bV[a\x10\x1CV[a\x03ja\x05m6`\x04a$LV[a\x10\xAEV[`\x0CTa\x03jV[a\x03\x90a\x05\x886`\x04a%~V[a\x10\xB9V[a\x03ja\x05\x9B6`\x04a%\xACV[a\x11\x81V[a\x03ja\x05\xAE6`\x04a#\xE1V[`\x03` R`\0\x90\x81R`@\x90 T\x81V[a\x03\x90a\x05\xCE6`\x04a%\xD1V[a\x11\x95V[a\x04\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x90a\x06\x086`\x04a#\xE1V[a\x12\x1EV[a\x03ja\x06\x1B6`\x04a#\xE1V[`\x05` R`\0\x90\x81R`@\x90 T\x81V[`\x06Ta\x04\xF9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03ja\x06N6`\x04a$LV[a\x13\x08V[a\x03ja\x06a6`\x04a%\xACV[a\x13\xA5V[a\x03\xBAa\x13\xB9V[a\x03\x90a\x06|6`\x04a$eV[a\x13\xC6V[a\x03\x90a\x06\x8F6`\x04a$LV[a\x14\xB0V[a\x04\0a\x06\xA26`\x04a$eV[a\x16\x03V[a\x04\0a\x06\xB56`\x04a#\xE1V[`\x0F` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03ja\x06\xD86`\x04a$LV[a\x16iV[a\x03ja\x06\xEB6`\x04a%\xFFV[a\x16\x88V[a\x04\0a\x06\xFE6`\x04a&6V[a\x16\xA5V[a\x03ja\x07\x116`\x04a%\xFFV[a\x17\xA3V[`\x07Ta\x04\xF9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\xF9a\x0776`\x04a#\xE1V[`\x08` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03ja\x07`6`\x04a$LV[a\x17\xB8V[a\x03ja\x07s6`\x04a#\xE1V[a\x17\xD8V[a\x03\x90a\x07\x866`\x04a&}V[a\x17\xFAV[a\x03ja\x07\x996`\x04a#\xE1V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x03ja\x07\xC26`\x04a%\xD1V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x04\0a\x07\xED6`\x04a&\xEBV[`\x01`\x01`\xE0\x1B\x03\x19\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x91\x90\x91\x16\x1C`\x01\x16\x15\x15\x90V[a\x04\0a\x08#6`\x04a'\x1EV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\t` R`@\x90 T`\x01`\xFF\x90\x92\x16\x1C\x16\x15\x15\x90V[a\x03ja\x08Z6`\x04a$\xAEV[`\x0B` R`\0\x90\x81R`@\x90 T\x81V[a\x03ja\x08z6`\x04a$LV[a\x1A>V[a\x03\x90a\x08\x8D6`\x04a#\xE1V[a\x1AIV[`\0a\x08\x9DBa\x1A\xC7V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x01W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t%\x91\x90a'JV[a\t/\x91\x90a'yV[\x90P\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\t\x95Wa\tp3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\t\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\x8CV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x0F` R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\0\x80Ta\t\xCD\x90a'\xC3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xF9\x90a'\xC3V[\x80\x15a\nFW\x80`\x1F\x10a\n\x1BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\nFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n)W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x02T`\0\x90\x80\x15a\nrWa\nma\nea\x08\x92V[\x84\x90\x83a\x1C^V[a\ntV[\x82[\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\n\xD3Wa\n\xB73`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\n\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\x8CV[`\x0EUV[3`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x85\x90UQ\x91\x92\x90\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90a\x0B3\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`\x02T`\0\x90\x80\x15a\nrWa\nm\x81a\x0B]a\x08\x92V[\x85\x91\x90a\x1C|V[a\x0B{3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x0B\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\xFDV[\x80\x15a\x0B\xC7W`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x90\x81R`\x0B` R`@\x90 \x80T`\x01`\xFF\x86\x16\x1B\x17\x90Ua\x0B\xEEV[`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x90\x81R`\x0B` R`@\x90 \x80T`\x01`\xFF\x86\x16\x1B\x19\x16\x90U[\x81`\x01`\x01`\xE0\x1B\x03\x19\x16\x83`\xFF\x16\x7F\xBF\xE1k,5\xCE#\xDF\xD1\xAB\x0E{]\x08j\x10\x06\x0C\x9BR\xD1WN\x16\x80\xC8\x81\xB3\xB3\xA2\xB1Q\x83`@Qa\x0C0\x91\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x81\x14a\x0C\x99Wa\x0Ct\x83\x82a(#V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x85\x92\x90a\x0C\xC1\x90\x84\x90a(#V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x87\x01\x90UQ\x90\x91\x87\x16\x90`\0\x80Q` a) \x839\x81Q\x91R\x90a\r\n\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\ruWa\rY3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\ruW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\x8CV[a\r}a\x1C\xA2V[`\x0CUV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\r\xB3Wa\t/a\x1D7V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0E0Wa\x0E\x143`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x0E0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\x8CV[3`\0\x90\x81R`\x0F` R`@\x90 T`\xFF\x16a\x0E\x99W`\x0ET\x81\x11\x15a\x0E\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FMockERC4626: Invalid mint amount`D\x82\x01R`d\x01a\t\x8CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA0q-ha\x0E\xD1\x83a\nNV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xEF\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\tW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x1DW=`\0\x80>=`\0\xFD[PPPPa\x0F+\x82\x82a\x1D\xD1V[PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0F\x87Wa\x0Fk3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x0F\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\x8CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cB\x96lha\x0F\xBF\x83a\nNV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xDD\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xF7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\x0BW=`\0\x80>=`\0\xFD[PPPPa\x10\x193\x82a\x1E+V[PV[a\x1023`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x10NW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\xFDV[`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x81\x81R`\n` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F6\xD2\x81&\xBE\xF2\x1AO7e\xD7\xFC\xB7\xC4\\\xEA\xD4c\xAELA\tN\xF3\xB7q\xED\xE5\x98TA\x03\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0a\x0B?\x82a\nNV[a\x10\xCF3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x10\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\xFDV[\x80\x15a\x11\x1AW`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\t` R`@\x90 \x80T`\x01`\xFF\x85\x16\x1B\x17\x90Ua\x11@V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\t` R`@\x90 \x80T`\x01`\xFF\x85\x16\x1B\x19\x16\x90U[\x81`\xFF\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7FL\x9B\xDD\x0C\x8E\x07>\xB5\xED\xA2%\x0B\x18\xD8\xE5\x12\x1F\xF2{b\x06O\xBE\xEE\xEDHi\xBB\x99\xBC[\xF2\x83`@Qa\x0C0\x91\x15\x15\x81R` \x01\x90V[`\0a\x11\x8Ba\x1C\xA2V[a\nt\x83\x83a\x1E\x8DV[a\x11\xAB3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x11\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\xFDV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x08` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x86\x16\x94\x85\x17\x90UQ\x7F\xA4\x90\x8E\x11\xA5\xF8\x95\xB1=QRl3\x1A\xC9<\xDD0\xE5\x97r6\x1C]\x07\x87N\xB3k\xFF e\x91\x90\xA3PPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x12\xB3WP`\x07T`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB7\0\x96\x13\x90a\x12r\x903\x900\x90`\x01`\x01`\xE0\x1B\x03\x19`\x005\x16\x90`\x04\x01a(6V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x8FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xB3\x91\x90a(cV[a\x12\xBCW`\0\x80\xFD[`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90\x7F\xA39o\xD7\xF6\xE0\xA2\x1BP\xE5\x08\x9D-\xA7\rZ\xC0\xA3\xBB\xBD\x1Faz\x93\xF14\xB7c\x89\x98\x01\x98\x90`\0\x90\xA3PV[`\0a\x13\x13\x82a\x1A\xC7V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13wW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x9B\x91\x90a'JV[a\x0B?\x91\x90a'yV[`\0a\x13\xAFa\x1C\xA2V[a\nt\x83\x83a\x1FcV[`\x01\x80Ta\t\xCD\x90a'\xC3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x14\x1EWa\x14\x023`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x14\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\x8CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cB\x96lha\x14V\x83a\nNV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14t\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\x8EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\xA2W=`\0\x80>=`\0\xFD[PPPPa\x0F+\x82\x82a\x1E+V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x15\x08Wa\x14\xEC3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x15\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\x8CV[3`\0\x90\x81R`\x0F` R`@\x90 T`\xFF\x16a\x15qW`\x0ET\x81\x11\x15a\x15qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FMockERC4626: Invalid mint amount`D\x82\x01R`d\x01a\t\x8CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA0q-ha\x15\xA9\x83a\nNV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\xC7\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15\xF5W=`\0\x80>=`\0\xFD[PPPPa\x10\x193\x82a\x1D\xD1V[3`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x91\x90\x83\x90a\x16$\x90\x84\x90a(#V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x85\x01\x90UQ3\x90`\0\x80Q` a) \x839\x81Q\x91R\x90a\x0B3\x90\x86\x81R` \x01\x90V[`\x02T`\0\x90\x80\x15a\nrWa\nma\x16\x80a\x08\x92V[\x84\x90\x83a\x1C|V[`\0a\x16\x92a\x1C\xA2V[a\x16\x9D\x84\x84\x84a\x1F\xF2V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x08` R`@\x81 T\x90\x91\x16\x80\x15a\x17AW`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB7\0\x96\x13\x90a\x16\xF8\x90\x88\x90\x88\x90\x88\x90`\x04\x01a(6V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x179\x91\x90a(cV[\x91PPa\ntV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16\x80a\x17\x9AWP`\x01`\x01`\xE0\x1B\x03\x19\x83\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 T`\x01`\x01`\xA0\x1B\x03\x89\x16\x84R`\t\x90\x92R\x90\x91 T\x16\x15\x15[\x95\x94PPPPPV[`\0a\x17\xADa\x1C\xA2V[a\x16\x9D\x84\x84\x84a \xF6V[`\x02T`\0\x90\x80\x15a\nrWa\nm\x81a\x17\xD0a\x08\x92V[\x85\x91\x90a\x1C^V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` R`@\x81 Ta\x0B?\x90a\nNV[B\x84\x10\x15a\x18JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\x8CV[`\0`\x01a\x18Va\r\x82V[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x80T`\x01\x81\x01\x90\x91U\x82Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81\x84\x01R\x80\x84\x01\x94\x90\x94R\x93\x8D\x16``\x84\x01R`\x80\x83\x01\x8C\x90R`\xA0\x83\x01\x93\x90\x93R`\xC0\x80\x83\x01\x8B\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x83\x01\x90\x91R\x80Q\x92\x01\x91\x90\x91 a\x19\x01`\xF0\x1Ba\x01\0\x83\x01Ra\x01\x02\x82\x01\x92\x90\x92Ra\x01\"\x81\x01\x91\x90\x91Ra\x01B\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83RR`\xFF\x87\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x19bW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x19\x98WP\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x19\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x01a\t\x8CV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x8A\x85\x16\x80\x85R\x90\x83R\x92\x81\x90 \x89\x90UQ\x88\x81R\x91\x92\x8A\x16\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPV[`\0a\x0B?\x82a\x17\xB8V[a\x1A_3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x1A{W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\xFDV[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PV[`\0`\x0CT`\0\x03a\x1A\xDBWP`\0\x91\x90PV[`\rT\x82\x10\x15a\x1A\xEDWP`\0\x91\x90PV[`\0a\x1B\x0Bc\x01\xE13\x80`\rT\x85a\x1B\x05\x91\x90a(#V[\x90a\"8V[\x90P`\0a\x16\x9Da\x1B'\x83`\x0CTa\"M\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xAF\x91\x90a'JV[\x90a\"MV[`\x07T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x80\x15\x90a\x1C?WP`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB7\0\x96\x13\x90a\x1B\xFE\x90\x87\x900\x90\x88\x90`\x04\x01a(6V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x1BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C?\x91\x90a(cV[\x80a\x16\x9DWP`\x06T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x91\x16\x14\x94\x93PPPPV[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x1CuW`\0\x80\xFD[P\x91\x02\x04\x90V[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x1C\x93W`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`\0a\x1C\xADBa\x1A\xC7V[\x90P\x80\x15a\x1D0W`@Qc\x14\x0E%\xAD`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA0q-h\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\x17W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1D+W=`\0\x80>=`\0\xFD[PPPP[PB`\rUV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa\x1Di\x91\x90a(\x80V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x80`\x02`\0\x82\x82Ta\x1D\xE3\x91\x90a'yV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R`\0\x80Q` a) \x839\x81Q\x91R\x91\x01[`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x92\x90a\x1ES\x90\x84\x90a(#V[\x90\x91UPP`\x02\x80T\x82\x90\x03\x90U`@Q\x81\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90`\0\x80Q` a) \x839\x81Q\x91R\x90` \x01a\x1E\x1FV[`\0a\x1E\x98\x83a\x1A>V[\x90P\x80`\0\x03a\x1E\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjZERO_SHARES`\xA8\x1B`D\x82\x01R`d\x01a\t\x8CV[a\x1F\r`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1630\x86a\"bV[a\x1F\x17\x82\x82a\x1D\xD1V[`@\x80Q\x84\x81R` \x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x913\x91\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x91\x01[`@Q\x80\x91\x03\x90\xA3a\x0B?V[`\0a\x1Fn\x83a\x16iV[\x90Pa\x1F\xA5`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1630\x84a\"bV[a\x1F\xAF\x82\x84a\x1D\xD1V[`@\x80Q\x82\x81R` \x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x913\x91\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x91\x01a\x1FVV[`\0a\x1F\xFD\x84a\x0BEV[\x90P3`\x01`\x01`\xA0\x1B\x03\x83\x16\x14a mW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\0\x19\x81\x14a kWa F\x82\x82a(#V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[P[a w\x82\x82a\x1E+V[`@\x80Q\x85\x81R` \x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x92\x90\x86\x16\x913\x91\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x91\x01`@Q\x80\x91\x03\x90\xA4a\nt`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x86a\"\xFEV[`\x003`\x01`\x01`\xA0\x1B\x03\x83\x16\x14a!fW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\0\x19\x81\x14a!dWa!?\x85\x82a(#V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[P[a!o\x84a\x10\xAEV[\x90P\x80`\0\x03a!\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjZERO_ASSETS`\xA8\x1B`D\x82\x01R`d\x01a\t\x8CV[a!\xB9\x82\x85a\x1E+V[`@\x80Q\x82\x81R` \x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x92\x90\x86\x16\x913\x91\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x91\x01`@Q\x80\x91\x03\x90\xA4a\nt`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x83a\"\xFEV[`\0a\nt\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1C^V[`\0a\nt\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1C^V[`\0`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\"\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\t\x8CV[PPPPPV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a#\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\t\x8CV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\x19W`\0\x80\xFD[\x80\x15\x15\x81\x14a\x10\x19W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a#\xBBW`\0\x80\xFD[\x825a#\xC6\x81a#\x85V[\x91P` \x83\x015a#\xD6\x81a#\x9AV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a#\xF3W`\0\x80\xFD[\x815a\nt\x81a#\x85V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a$+W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a$\x0FV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a$^W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$xW`\0\x80\xFD[\x825a$\x83\x81a#\x85V[\x94` \x93\x90\x93\x015\x93PPPV[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a$\xA9W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a$\xC0W`\0\x80\xFD[a\nt\x82a$\x91V[\x805`\xFF\x81\x16\x81\x14a$\xA9W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a$\xEFW`\0\x80\xFD[a$\xF8\x84a$\xC9V[\x92Pa%\x06` \x85\x01a$\x91V[\x91P`@\x84\x015a%\x16\x81a#\x9AV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a%6W`\0\x80\xFD[\x835a%A\x81a#\x85V[\x92P` \x84\x015a%Q\x81a#\x85V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a%uW`\0\x80\xFD[a#\xC6\x83a$\x91V[`\0\x80`\0``\x84\x86\x03\x12\x15a%\x93W`\0\x80\xFD[\x835a%\x9E\x81a#\x85V[\x92Pa%\x06` \x85\x01a$\xC9V[`\0\x80`@\x83\x85\x03\x12\x15a%\xBFW`\0\x80\xFD[\x825\x91P` \x83\x015a#\xD6\x81a#\x85V[`\0\x80`@\x83\x85\x03\x12\x15a%\xE4W`\0\x80\xFD[\x825a%\xEF\x81a#\x85V[\x91P` \x83\x015a#\xD6\x81a#\x85V[`\0\x80`\0``\x84\x86\x03\x12\x15a&\x14W`\0\x80\xFD[\x835\x92P` \x84\x015a&&\x81a#\x85V[\x91P`@\x84\x015a%\x16\x81a#\x85V[`\0\x80`\0``\x84\x86\x03\x12\x15a&KW`\0\x80\xFD[\x835a&V\x81a#\x85V[\x92P` \x84\x015a&f\x81a#\x85V[\x91Pa&t`@\x85\x01a$\x91V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a&\x98W`\0\x80\xFD[\x875a&\xA3\x81a#\x85V[\x96P` \x88\x015a&\xB3\x81a#\x85V[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa&\xCF`\x80\x89\x01a$\xC9V[\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`@\x83\x85\x03\x12\x15a&\xFEW`\0\x80\xFD[a'\x07\x83a$\xC9V[\x91Pa'\x15` \x84\x01a$\x91V[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a'1W`\0\x80\xFD[\x825a'<\x81a#\x85V[\x91Pa'\x15` \x84\x01a$\xC9V[`\0` \x82\x84\x03\x12\x15a'\\W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0B?Wa\x0B?a'cV[` \x80\x82R`\x1B\x90\x82\x01R\x7FMockERC4626: not authorized\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a'\xD7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a'\xF7WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0B?Wa\x0B?a'cV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a(uW`\0\x80\xFD[\x81Qa\nt\x81a#\x9AV[`\0\x80\x83T\x81`\x01\x82\x81\x1C\x91P\x80\x83\x16\x80a(\x9CW`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03a(\xBBWcNH{q`\xE0\x1B\x86R`\"`\x04R`$\x86\xFD[\x81\x80\x15a(\xCFW`\x01\x81\x14a(\xE4Wa)\x11V[`\xFF\x19\x86\x16\x89R\x84\x15\x15\x85\x02\x89\x01\x96Pa)\x11V[`\0\x8A\x81R` \x90 `\0[\x86\x81\x10\x15a)\tW\x81T\x8B\x82\x01R\x90\x85\x01\x90\x83\x01a(\xF0V[PP\x84\x89\x01\x96P[P\x94\x98\x97PPPPPPPPV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xD1\xE0w\x07i\xEA\xFE\x1C\x0E!\x18\xF0\x96q<\xA0^\xF4&\xD5\x02o\x0F\xD8 \xED\x08Z#\xA6vGdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static MOCKERC4626_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03]W`\x005`\xE0\x1C\x80cz\x8Cc\xB5\x11a\x01\xD3W\x80c\xBA\x08vR\x11a\x01\x04W\x80c\xD9\x05w~\x11a\0\xA2W\x80c\xEA|\xA2v\x11a\0|W\x80c\xEA|\xA2v\x14a\x08\x15W\x80c\xED\r\x0E\xFB\x14a\x08LW\x80c\xEF\x8B0\xF7\x14a\x08lW\x80c\xF2\xFD\xE3\x8B\x14a\x08\x7FW`\0\x80\xFD[\x80c\xD9\x05w~\x14a\x07\x8BW\x80c\xDDb\xED>\x14a\x07\xB4W\x80c\xE6\x88t{\x14a\x07\xDFW`\0\x80\xFD[\x80c\xC6=u\xB6\x11a\0\xDEW\x80c\xC6=u\xB6\x14a\x05\x11W\x80c\xC6\xE6\xF5\x92\x14a\x07RW\x80c\xCE\x96\xCBw\x14a\x07eW\x80c\xD5\x05\xAC\xCF\x14a\x07xW`\0\x80\xFD[\x80c\xBA\x08vR\x14a\x07\x03W\x80c\xBF~!O\x14a\x07\x16W\x80c\xC5:9\x85\x14a\x07)W`\0\x80\xFD[\x80c\x9D\xC2\x9F\xAC\x11a\x01qW\x80c\xAE\xD3\x07w\x11a\x01KW\x80c\xAE\xD3\x07w\x14a\x06\xA7W\x80c\xB3\xD7\xF6\xB9\x14a\x06\xCAW\x80c\xB4`\xAF\x94\x14a\x06\xDDW\x80c\xB7\0\x96\x13\x14a\x06\xF0W`\0\x80\xFD[\x80c\x9D\xC2\x9F\xAC\x14a\x06nW\x80c\xA0q-h\x14a\x06\x81W\x80c\xA9\x05\x9C\xBB\x14a\x06\x94W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\x01\xADW\x80c\x8D\xA5\xCB[\x14a\x06-W\x80c\x94`XW\x14a\x06@W\x80c\x94\xBF\x80M\x14a\x06SW\x80c\x95\xD8\x9BA\x14a\x06fW`\0\x80\xFD[\x80cz\x8Cc\xB5\x14a\x05\xD3W\x80cz\x9E^K\x14a\x05\xFAW\x80c~\xCE\xBE\0\x14a\x06\rW`\0\x80\xFD[\x80c1<\xE5g\x11a\x02\xADW\x80cKQY\xDA\x11a\x02KW\x80cg\xAF\xF4\x84\x11a\x02%W\x80cg\xAF\xF4\x84\x14a\x05zW\x80cnU?e\x14a\x05\x8DW\x80cp\xA0\x821\x14a\x05\xA0W\x80cr\x8B\x95+\x14a\x05\xC0W`\0\x80\xFD[\x80cKQY\xDA\x14a\x05LW\x80cL\xDA\xD5\x06\x14a\x05_W\x80cg\x9A\xEF\xCE\x14a\x05rW`\0\x80\xFD[\x80c8\xD5.\x0F\x11a\x02\x87W\x80c8\xD5.\x0F\x14a\x04\xD2W\x80c@-&}\x14a\x05\x11W\x80c@\xC1\x0F\x19\x14a\x05&W\x80cB\x96lh\x14a\x059W`\0\x80\xFD[\x80c1<\xE5g\x14a\x04~W\x80c4\xFC\xF47\x14a\x04\xB7W\x80c6D\xE5\x15\x14a\x04\xCAW`\0\x80\xFD[\x80c\t^\xA7\xB3\x11a\x03\x1AW\x80c\x0E\xA9\xB7[\x11a\x02\xF4W\x80c\x0E\xA9\xB7[\x14a\x04FW\x80c\x18\x16\r\xDD\x14a\x04YW\x80c#\x9Cp\xAE\x14a\x04bW\x80c#\xB8r\xDD\x14a\x04kW`\0\x80\xFD[\x80c\t^\xA7\xB3\x14a\x03\xEDW\x80c\n(\xA4w\x14a\x04\x10W\x80c\x0B\xAD\xE8\xA4\x14a\x04#W`\0\x80\xFD[\x80c\x01\xE1\xD1\x14\x14a\x03bW\x80c\x05\xF0Z\x94\x14a\x03}W\x80c\x06\xA3j\xEE\x14a\x03\x92W\x80c\x06\xFD\xDE\x03\x14a\x03\xB2W\x80c\x07\xA2\xD1:\x14a\x03\xC7W\x80c\x08\x8AN\xD0\x14a\x03\xDAW[`\0\x80\xFD[a\x03ja\x08\x92V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\x90a\x03\x8B6`\x04a#\xA8V[a\t4V[\0[a\x03ja\x03\xA06`\x04a#\xE1V[`\t` R`\0\x90\x81R`@\x90 T\x81V[a\x03\xBAa\t\xC0V[`@Qa\x03t\x91\x90a#\xFEV[a\x03ja\x03\xD56`\x04a$LV[a\nNV[a\x03\x90a\x03\xE86`\x04a$LV[a\n{V[a\x04\0a\x03\xFB6`\x04a$eV[a\n\xD8V[`@Q\x90\x15\x15\x81R` \x01a\x03tV[a\x03ja\x04\x1E6`\x04a$LV[a\x0BEV[a\x04\0a\x0416`\x04a$\xAEV[`\n` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03\x90a\x04T6`\x04a$\xDAV[a\x0BeV[a\x03j`\x02T\x81V[a\x03j`\x0ET\x81V[a\x04\0a\x04y6`\x04a%!V[a\x0C=V[a\x04\xA5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x03tV[a\x03\x90a\x04\xC56`\x04a$LV[a\r\x1DV[a\x03ja\r\x82V[a\x04\xF9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03tV[a\x03ja\x05\x1F6`\x04a#\xE1V[P`\0\x19\x90V[a\x03\x90a\x0546`\x04a$eV[a\r\xD8V[a\x03\x90a\x05G6`\x04a$LV[a\x0F/V[a\x03\x90a\x05Z6`\x04a%bV[a\x10\x1CV[a\x03ja\x05m6`\x04a$LV[a\x10\xAEV[`\x0CTa\x03jV[a\x03\x90a\x05\x886`\x04a%~V[a\x10\xB9V[a\x03ja\x05\x9B6`\x04a%\xACV[a\x11\x81V[a\x03ja\x05\xAE6`\x04a#\xE1V[`\x03` R`\0\x90\x81R`@\x90 T\x81V[a\x03\x90a\x05\xCE6`\x04a%\xD1V[a\x11\x95V[a\x04\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x90a\x06\x086`\x04a#\xE1V[a\x12\x1EV[a\x03ja\x06\x1B6`\x04a#\xE1V[`\x05` R`\0\x90\x81R`@\x90 T\x81V[`\x06Ta\x04\xF9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03ja\x06N6`\x04a$LV[a\x13\x08V[a\x03ja\x06a6`\x04a%\xACV[a\x13\xA5V[a\x03\xBAa\x13\xB9V[a\x03\x90a\x06|6`\x04a$eV[a\x13\xC6V[a\x03\x90a\x06\x8F6`\x04a$LV[a\x14\xB0V[a\x04\0a\x06\xA26`\x04a$eV[a\x16\x03V[a\x04\0a\x06\xB56`\x04a#\xE1V[`\x0F` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03ja\x06\xD86`\x04a$LV[a\x16iV[a\x03ja\x06\xEB6`\x04a%\xFFV[a\x16\x88V[a\x04\0a\x06\xFE6`\x04a&6V[a\x16\xA5V[a\x03ja\x07\x116`\x04a%\xFFV[a\x17\xA3V[`\x07Ta\x04\xF9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\xF9a\x0776`\x04a#\xE1V[`\x08` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03ja\x07`6`\x04a$LV[a\x17\xB8V[a\x03ja\x07s6`\x04a#\xE1V[a\x17\xD8V[a\x03\x90a\x07\x866`\x04a&}V[a\x17\xFAV[a\x03ja\x07\x996`\x04a#\xE1V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x03ja\x07\xC26`\x04a%\xD1V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x04\0a\x07\xED6`\x04a&\xEBV[`\x01`\x01`\xE0\x1B\x03\x19\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x91\x90\x91\x16\x1C`\x01\x16\x15\x15\x90V[a\x04\0a\x08#6`\x04a'\x1EV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\t` R`@\x90 T`\x01`\xFF\x90\x92\x16\x1C\x16\x15\x15\x90V[a\x03ja\x08Z6`\x04a$\xAEV[`\x0B` R`\0\x90\x81R`@\x90 T\x81V[a\x03ja\x08z6`\x04a$LV[a\x1A>V[a\x03\x90a\x08\x8D6`\x04a#\xE1V[a\x1AIV[`\0a\x08\x9DBa\x1A\xC7V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x01W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t%\x91\x90a'JV[a\t/\x91\x90a'yV[\x90P\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\t\x95Wa\tp3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\t\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\x8CV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x0F` R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\0\x80Ta\t\xCD\x90a'\xC3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xF9\x90a'\xC3V[\x80\x15a\nFW\x80`\x1F\x10a\n\x1BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\nFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n)W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x02T`\0\x90\x80\x15a\nrWa\nma\nea\x08\x92V[\x84\x90\x83a\x1C^V[a\ntV[\x82[\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\n\xD3Wa\n\xB73`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\n\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\x8CV[`\x0EUV[3`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x85\x90UQ\x91\x92\x90\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90a\x0B3\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`\x02T`\0\x90\x80\x15a\nrWa\nm\x81a\x0B]a\x08\x92V[\x85\x91\x90a\x1C|V[a\x0B{3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x0B\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\xFDV[\x80\x15a\x0B\xC7W`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x90\x81R`\x0B` R`@\x90 \x80T`\x01`\xFF\x86\x16\x1B\x17\x90Ua\x0B\xEEV[`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x90\x81R`\x0B` R`@\x90 \x80T`\x01`\xFF\x86\x16\x1B\x19\x16\x90U[\x81`\x01`\x01`\xE0\x1B\x03\x19\x16\x83`\xFF\x16\x7F\xBF\xE1k,5\xCE#\xDF\xD1\xAB\x0E{]\x08j\x10\x06\x0C\x9BR\xD1WN\x16\x80\xC8\x81\xB3\xB3\xA2\xB1Q\x83`@Qa\x0C0\x91\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x81\x14a\x0C\x99Wa\x0Ct\x83\x82a(#V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x85\x92\x90a\x0C\xC1\x90\x84\x90a(#V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x87\x01\x90UQ\x90\x91\x87\x16\x90`\0\x80Q` a) \x839\x81Q\x91R\x90a\r\n\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\ruWa\rY3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\ruW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\x8CV[a\r}a\x1C\xA2V[`\x0CUV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\r\xB3Wa\t/a\x1D7V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0E0Wa\x0E\x143`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x0E0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\x8CV[3`\0\x90\x81R`\x0F` R`@\x90 T`\xFF\x16a\x0E\x99W`\x0ET\x81\x11\x15a\x0E\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FMockERC4626: Invalid mint amount`D\x82\x01R`d\x01a\t\x8CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA0q-ha\x0E\xD1\x83a\nNV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xEF\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\tW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x1DW=`\0\x80>=`\0\xFD[PPPPa\x0F+\x82\x82a\x1D\xD1V[PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0F\x87Wa\x0Fk3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x0F\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\x8CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cB\x96lha\x0F\xBF\x83a\nNV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xDD\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xF7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\x0BW=`\0\x80>=`\0\xFD[PPPPa\x10\x193\x82a\x1E+V[PV[a\x1023`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x10NW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\xFDV[`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`\0\x81\x81R`\n` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F6\xD2\x81&\xBE\xF2\x1AO7e\xD7\xFC\xB7\xC4\\\xEA\xD4c\xAELA\tN\xF3\xB7q\xED\xE5\x98TA\x03\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0a\x0B?\x82a\nNV[a\x10\xCF3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x10\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\xFDV[\x80\x15a\x11\x1AW`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\t` R`@\x90 \x80T`\x01`\xFF\x85\x16\x1B\x17\x90Ua\x11@V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\t` R`@\x90 \x80T`\x01`\xFF\x85\x16\x1B\x19\x16\x90U[\x81`\xFF\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7FL\x9B\xDD\x0C\x8E\x07>\xB5\xED\xA2%\x0B\x18\xD8\xE5\x12\x1F\xF2{b\x06O\xBE\xEE\xEDHi\xBB\x99\xBC[\xF2\x83`@Qa\x0C0\x91\x15\x15\x81R` \x01\x90V[`\0a\x11\x8Ba\x1C\xA2V[a\nt\x83\x83a\x1E\x8DV[a\x11\xAB3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x11\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\xFDV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x08` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x86\x16\x94\x85\x17\x90UQ\x7F\xA4\x90\x8E\x11\xA5\xF8\x95\xB1=QRl3\x1A\xC9<\xDD0\xE5\x97r6\x1C]\x07\x87N\xB3k\xFF e\x91\x90\xA3PPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x12\xB3WP`\x07T`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB7\0\x96\x13\x90a\x12r\x903\x900\x90`\x01`\x01`\xE0\x1B\x03\x19`\x005\x16\x90`\x04\x01a(6V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x8FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xB3\x91\x90a(cV[a\x12\xBCW`\0\x80\xFD[`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90\x7F\xA39o\xD7\xF6\xE0\xA2\x1BP\xE5\x08\x9D-\xA7\rZ\xC0\xA3\xBB\xBD\x1Faz\x93\xF14\xB7c\x89\x98\x01\x98\x90`\0\x90\xA3PV[`\0a\x13\x13\x82a\x1A\xC7V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13wW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x9B\x91\x90a'JV[a\x0B?\x91\x90a'yV[`\0a\x13\xAFa\x1C\xA2V[a\nt\x83\x83a\x1FcV[`\x01\x80Ta\t\xCD\x90a'\xC3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x14\x1EWa\x14\x023`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x14\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\x8CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cB\x96lha\x14V\x83a\nNV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14t\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\x8EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\xA2W=`\0\x80>=`\0\xFD[PPPPa\x0F+\x82\x82a\x1E+V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x15\x08Wa\x14\xEC3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x15\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\x8CV[3`\0\x90\x81R`\x0F` R`@\x90 T`\xFF\x16a\x15qW`\x0ET\x81\x11\x15a\x15qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FMockERC4626: Invalid mint amount`D\x82\x01R`d\x01a\t\x8CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA0q-ha\x15\xA9\x83a\nNV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\xC7\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15\xF5W=`\0\x80>=`\0\xFD[PPPPa\x10\x193\x82a\x1D\xD1V[3`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x91\x90\x83\x90a\x16$\x90\x84\x90a(#V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x85\x01\x90UQ3\x90`\0\x80Q` a) \x839\x81Q\x91R\x90a\x0B3\x90\x86\x81R` \x01\x90V[`\x02T`\0\x90\x80\x15a\nrWa\nma\x16\x80a\x08\x92V[\x84\x90\x83a\x1C|V[`\0a\x16\x92a\x1C\xA2V[a\x16\x9D\x84\x84\x84a\x1F\xF2V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x08` R`@\x81 T\x90\x91\x16\x80\x15a\x17AW`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB7\0\x96\x13\x90a\x16\xF8\x90\x88\x90\x88\x90\x88\x90`\x04\x01a(6V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x179\x91\x90a(cV[\x91PPa\ntV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16\x80a\x17\x9AWP`\x01`\x01`\xE0\x1B\x03\x19\x83\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 T`\x01`\x01`\xA0\x1B\x03\x89\x16\x84R`\t\x90\x92R\x90\x91 T\x16\x15\x15[\x95\x94PPPPPV[`\0a\x17\xADa\x1C\xA2V[a\x16\x9D\x84\x84\x84a \xF6V[`\x02T`\0\x90\x80\x15a\nrWa\nm\x81a\x17\xD0a\x08\x92V[\x85\x91\x90a\x1C^V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` R`@\x81 Ta\x0B?\x90a\nNV[B\x84\x10\x15a\x18JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\x8CV[`\0`\x01a\x18Va\r\x82V[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x80T`\x01\x81\x01\x90\x91U\x82Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81\x84\x01R\x80\x84\x01\x94\x90\x94R\x93\x8D\x16``\x84\x01R`\x80\x83\x01\x8C\x90R`\xA0\x83\x01\x93\x90\x93R`\xC0\x80\x83\x01\x8B\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x83\x01\x90\x91R\x80Q\x92\x01\x91\x90\x91 a\x19\x01`\xF0\x1Ba\x01\0\x83\x01Ra\x01\x02\x82\x01\x92\x90\x92Ra\x01\"\x81\x01\x91\x90\x91Ra\x01B\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83RR`\xFF\x87\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x19bW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x19\x98WP\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x19\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x01a\t\x8CV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x8A\x85\x16\x80\x85R\x90\x83R\x92\x81\x90 \x89\x90UQ\x88\x81R\x91\x92\x8A\x16\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPV[`\0a\x0B?\x82a\x17\xB8V[a\x1A_3`\x005`\x01`\x01`\xE0\x1B\x03\x19\x16a\x1B\xB5V[a\x1A{W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a'\xFDV[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PV[`\0`\x0CT`\0\x03a\x1A\xDBWP`\0\x91\x90PV[`\rT\x82\x10\x15a\x1A\xEDWP`\0\x91\x90PV[`\0a\x1B\x0Bc\x01\xE13\x80`\rT\x85a\x1B\x05\x91\x90a(#V[\x90a\"8V[\x90P`\0a\x16\x9Da\x1B'\x83`\x0CTa\"M\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xAF\x91\x90a'JV[\x90a\"MV[`\x07T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x80\x15\x90a\x1C?WP`@Qc\xB7\0\x96\x13`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB7\0\x96\x13\x90a\x1B\xFE\x90\x87\x900\x90\x88\x90`\x04\x01a(6V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x1BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C?\x91\x90a(cV[\x80a\x16\x9DWP`\x06T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x91\x16\x14\x94\x93PPPPV[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x1CuW`\0\x80\xFD[P\x91\x02\x04\x90V[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x1C\x93W`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`\0a\x1C\xADBa\x1A\xC7V[\x90P\x80\x15a\x1D0W`@Qc\x14\x0E%\xAD`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA0q-h\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\x17W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1D+W=`\0\x80>=`\0\xFD[PPPP[PB`\rUV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa\x1Di\x91\x90a(\x80V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x80`\x02`\0\x82\x82Ta\x1D\xE3\x91\x90a'yV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R`\0\x80Q` a) \x839\x81Q\x91R\x91\x01[`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x92\x90a\x1ES\x90\x84\x90a(#V[\x90\x91UPP`\x02\x80T\x82\x90\x03\x90U`@Q\x81\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90`\0\x80Q` a) \x839\x81Q\x91R\x90` \x01a\x1E\x1FV[`\0a\x1E\x98\x83a\x1A>V[\x90P\x80`\0\x03a\x1E\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjZERO_SHARES`\xA8\x1B`D\x82\x01R`d\x01a\t\x8CV[a\x1F\r`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1630\x86a\"bV[a\x1F\x17\x82\x82a\x1D\xD1V[`@\x80Q\x84\x81R` \x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x913\x91\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x91\x01[`@Q\x80\x91\x03\x90\xA3a\x0B?V[`\0a\x1Fn\x83a\x16iV[\x90Pa\x1F\xA5`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1630\x84a\"bV[a\x1F\xAF\x82\x84a\x1D\xD1V[`@\x80Q\x82\x81R` \x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x913\x91\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x91\x01a\x1FVV[`\0a\x1F\xFD\x84a\x0BEV[\x90P3`\x01`\x01`\xA0\x1B\x03\x83\x16\x14a mW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\0\x19\x81\x14a kWa F\x82\x82a(#V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[P[a w\x82\x82a\x1E+V[`@\x80Q\x85\x81R` \x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x92\x90\x86\x16\x913\x91\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x91\x01`@Q\x80\x91\x03\x90\xA4a\nt`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x86a\"\xFEV[`\x003`\x01`\x01`\xA0\x1B\x03\x83\x16\x14a!fW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\0\x19\x81\x14a!dWa!?\x85\x82a(#V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[P[a!o\x84a\x10\xAEV[\x90P\x80`\0\x03a!\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjZERO_ASSETS`\xA8\x1B`D\x82\x01R`d\x01a\t\x8CV[a!\xB9\x82\x85a\x1E+V[`@\x80Q\x82\x81R` \x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x92\x90\x86\x16\x913\x91\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x91\x01`@Q\x80\x91\x03\x90\xA4a\nt`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x83a\"\xFEV[`\0a\nt\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1C^V[`\0a\nt\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1C^V[`\0`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\"\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\t\x8CV[PPPPPV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a#\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\t\x8CV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\x19W`\0\x80\xFD[\x80\x15\x15\x81\x14a\x10\x19W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a#\xBBW`\0\x80\xFD[\x825a#\xC6\x81a#\x85V[\x91P` \x83\x015a#\xD6\x81a#\x9AV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a#\xF3W`\0\x80\xFD[\x815a\nt\x81a#\x85V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a$+W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a$\x0FV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a$^W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$xW`\0\x80\xFD[\x825a$\x83\x81a#\x85V[\x94` \x93\x90\x93\x015\x93PPPV[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a$\xA9W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a$\xC0W`\0\x80\xFD[a\nt\x82a$\x91V[\x805`\xFF\x81\x16\x81\x14a$\xA9W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a$\xEFW`\0\x80\xFD[a$\xF8\x84a$\xC9V[\x92Pa%\x06` \x85\x01a$\x91V[\x91P`@\x84\x015a%\x16\x81a#\x9AV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a%6W`\0\x80\xFD[\x835a%A\x81a#\x85V[\x92P` \x84\x015a%Q\x81a#\x85V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a%uW`\0\x80\xFD[a#\xC6\x83a$\x91V[`\0\x80`\0``\x84\x86\x03\x12\x15a%\x93W`\0\x80\xFD[\x835a%\x9E\x81a#\x85V[\x92Pa%\x06` \x85\x01a$\xC9V[`\0\x80`@\x83\x85\x03\x12\x15a%\xBFW`\0\x80\xFD[\x825\x91P` \x83\x015a#\xD6\x81a#\x85V[`\0\x80`@\x83\x85\x03\x12\x15a%\xE4W`\0\x80\xFD[\x825a%\xEF\x81a#\x85V[\x91P` \x83\x015a#\xD6\x81a#\x85V[`\0\x80`\0``\x84\x86\x03\x12\x15a&\x14W`\0\x80\xFD[\x835\x92P` \x84\x015a&&\x81a#\x85V[\x91P`@\x84\x015a%\x16\x81a#\x85V[`\0\x80`\0``\x84\x86\x03\x12\x15a&KW`\0\x80\xFD[\x835a&V\x81a#\x85V[\x92P` \x84\x015a&f\x81a#\x85V[\x91Pa&t`@\x85\x01a$\x91V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a&\x98W`\0\x80\xFD[\x875a&\xA3\x81a#\x85V[\x96P` \x88\x015a&\xB3\x81a#\x85V[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa&\xCF`\x80\x89\x01a$\xC9V[\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`@\x83\x85\x03\x12\x15a&\xFEW`\0\x80\xFD[a'\x07\x83a$\xC9V[\x91Pa'\x15` \x84\x01a$\x91V[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a'1W`\0\x80\xFD[\x825a'<\x81a#\x85V[\x91Pa'\x15` \x84\x01a$\xC9V[`\0` \x82\x84\x03\x12\x15a'\\W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0B?Wa\x0B?a'cV[` \x80\x82R`\x1B\x90\x82\x01R\x7FMockERC4626: not authorized\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a'\xD7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a'\xF7WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0B?Wa\x0B?a'cV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a(uW`\0\x80\xFD[\x81Qa\nt\x81a#\x9AV[`\0\x80\x83T\x81`\x01\x82\x81\x1C\x91P\x80\x83\x16\x80a(\x9CW`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03a(\xBBWcNH{q`\xE0\x1B\x86R`\"`\x04R`$\x86\xFD[\x81\x80\x15a(\xCFW`\x01\x81\x14a(\xE4Wa)\x11V[`\xFF\x19\x86\x16\x89R\x84\x15\x15\x85\x02\x89\x01\x96Pa)\x11V[`\0\x8A\x81R` \x90 `\0[\x86\x81\x10\x15a)\tW\x81T\x8B\x82\x01R\x90\x85\x01\x90\x83\x01a(\xF0V[PP\x84\x89\x01\x96P[P\x94\x98\x97PPPPPPPPV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xD1\xE0w\x07i\xEA\xFE\x1C\x0E!\x18\xF0\x96q<\xA0^\xF4&\xD5\x02o\x0F\xD8 \xED\x08Z#\xA6vGdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKERC4626_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockERC4626<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockERC4626<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockERC4626<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockERC4626<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockERC4626<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockERC4626))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockERC4626<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKERC4626_ABI.clone(),
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
                MOCKERC4626_ABI.clone(),
                MOCKERC4626_BYTECODE.clone().into(),
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
        ///Calls the contract's `asset` (0x38d52e0f) function
        pub fn asset(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([56, 213, 46, 15], ())
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
        ///Calls the contract's `convertToAssets` (0x07a2d13a) function
        pub fn convert_to_assets(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([7, 162, 209, 58], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertToShares` (0xc6e6f592) function
        pub fn convert_to_shares(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([198, 230, 245, 146], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x6e553f65) function
        pub fn deposit(
            &self,
            assets: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([110, 85, 63, 101], (assets, receiver))
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
        ///Calls the contract's `maxDeposit` (0x402d267d) function
        pub fn max_deposit(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([64, 45, 38, 125], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxMint` (0xc63d75b6) function
        pub fn max_mint(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([198, 61, 117, 182], p0)
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
        ///Calls the contract's `maxRedeem` (0xd905777e) function
        pub fn max_redeem(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([217, 5, 119, 126], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxWithdraw` (0xce96cb77) function
        pub fn max_withdraw(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([206, 150, 203, 119], owner)
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
        ///Calls the contract's `mint` (0x94bf804d) function
        pub fn mint_with_shares(
            &self,
            shares: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([148, 191, 128, 77], (shares, receiver))
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
        ///Calls the contract's `previewDeposit` (0xef8b30f7) function
        pub fn preview_deposit(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([239, 139, 48, 247], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewMint` (0xb3d7f6b9) function
        pub fn preview_mint(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([179, 215, 246, 185], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewRedeem` (0x4cdad506) function
        pub fn preview_redeem(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([76, 218, 213, 6], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewWithdraw` (0x0a28a477) function
        pub fn preview_withdraw(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 40, 164, 119], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeem` (0xba087652) function
        pub fn redeem(
            &self,
            shares: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([186, 8, 118, 82], (shares, receiver, owner))
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
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalAssets` (0x01e1d114) function
        pub fn total_assets(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([1, 225, 209, 20], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalAssets` (0x94605857) function
        pub fn total_assets_with_timestamp(
            &self,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([148, 96, 88, 87], timestamp)
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
        ///Calls the contract's `withdraw` (0xb460af94) function
        pub fn withdraw(
            &self,
            assets: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([180, 96, 175, 148], (assets, receiver, owner))
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
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
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
        ///Gets the contract's `Withdraw` event
        pub fn withdraw_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MockERC4626Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockERC4626<M> {
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,address,uint256,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
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
        name = "Withdraw",
        abi = "Withdraw(address,address,address,uint256,uint256)"
    )]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
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
    pub enum MockERC4626Events {
        ApprovalFilter(ApprovalFilter),
        AuthorityUpdatedFilter(AuthorityUpdatedFilter),
        DepositFilter(DepositFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PublicCapabilityUpdatedFilter(PublicCapabilityUpdatedFilter),
        RoleCapabilityUpdatedFilter(RoleCapabilityUpdatedFilter),
        TargetCustomAuthorityUpdatedFilter(TargetCustomAuthorityUpdatedFilter),
        TransferFilter(TransferFilter),
        UserRoleUpdatedFilter(UserRoleUpdatedFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockERC4626Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockERC4626Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = AuthorityUpdatedFilter::decode_log(log) {
                return Ok(MockERC4626Events::AuthorityUpdatedFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(MockERC4626Events::DepositFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MockERC4626Events::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PublicCapabilityUpdatedFilter::decode_log(log) {
                return Ok(MockERC4626Events::PublicCapabilityUpdatedFilter(decoded));
            }
            if let Ok(decoded) = RoleCapabilityUpdatedFilter::decode_log(log) {
                return Ok(MockERC4626Events::RoleCapabilityUpdatedFilter(decoded));
            }
            if let Ok(decoded) = TargetCustomAuthorityUpdatedFilter::decode_log(log) {
                return Ok(
                    MockERC4626Events::TargetCustomAuthorityUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockERC4626Events::TransferFilter(decoded));
            }
            if let Ok(decoded) = UserRoleUpdatedFilter::decode_log(log) {
                return Ok(MockERC4626Events::UserRoleUpdatedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(MockERC4626Events::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockERC4626Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuthorityUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for MockERC4626Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<AuthorityUpdatedFilter> for MockERC4626Events {
        fn from(value: AuthorityUpdatedFilter) -> Self {
            Self::AuthorityUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for MockERC4626Events {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for MockERC4626Events {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PublicCapabilityUpdatedFilter> for MockERC4626Events {
        fn from(value: PublicCapabilityUpdatedFilter) -> Self {
            Self::PublicCapabilityUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<RoleCapabilityUpdatedFilter> for MockERC4626Events {
        fn from(value: RoleCapabilityUpdatedFilter) -> Self {
            Self::RoleCapabilityUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TargetCustomAuthorityUpdatedFilter>
    for MockERC4626Events {
        fn from(value: TargetCustomAuthorityUpdatedFilter) -> Self {
            Self::TargetCustomAuthorityUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for MockERC4626Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<UserRoleUpdatedFilter> for MockERC4626Events {
        fn from(value: UserRoleUpdatedFilter) -> Self {
            Self::UserRoleUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for MockERC4626Events {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
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
    ///Container type for all input parameters for the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    #[ethcall(name = "asset", abi = "asset()")]
    pub struct AssetCall;
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
    ///Container type for all input parameters for the `convertToAssets` function with signature `convertToAssets(uint256)` and selector `0x07a2d13a`
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
    #[ethcall(name = "convertToAssets", abi = "convertToAssets(uint256)")]
    pub struct ConvertToAssetsCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `convertToShares` function with signature `convertToShares(uint256)` and selector `0xc6e6f592`
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
    #[ethcall(name = "convertToShares", abi = "convertToShares(uint256)")]
    pub struct ConvertToSharesCall {
        pub assets: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `deposit` function with signature `deposit(uint256,address)` and selector `0x6e553f65`
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
    #[ethcall(name = "deposit", abi = "deposit(uint256,address)")]
    pub struct DepositCall {
        pub assets: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `maxDeposit` function with signature `maxDeposit(address)` and selector `0x402d267d`
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
    #[ethcall(name = "maxDeposit", abi = "maxDeposit(address)")]
    pub struct MaxDepositCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `maxMint` function with signature `maxMint(address)` and selector `0xc63d75b6`
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
    #[ethcall(name = "maxMint", abi = "maxMint(address)")]
    pub struct MaxMintCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `maxRedeem` function with signature `maxRedeem(address)` and selector `0xd905777e`
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
    #[ethcall(name = "maxRedeem", abi = "maxRedeem(address)")]
    pub struct MaxRedeemCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `maxWithdraw` function with signature `maxWithdraw(address)` and selector `0xce96cb77`
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
    #[ethcall(name = "maxWithdraw", abi = "maxWithdraw(address)")]
    pub struct MaxWithdrawCall {
        pub owner: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `mint` function with signature `mint(uint256,address)` and selector `0x94bf804d`
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
    #[ethcall(name = "mint", abi = "mint(uint256,address)")]
    pub struct MintWithSharesCall {
        pub shares: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `previewDeposit` function with signature `previewDeposit(uint256)` and selector `0xef8b30f7`
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
    #[ethcall(name = "previewDeposit", abi = "previewDeposit(uint256)")]
    pub struct PreviewDepositCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `previewMint` function with signature `previewMint(uint256)` and selector `0xb3d7f6b9`
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
    #[ethcall(name = "previewMint", abi = "previewMint(uint256)")]
    pub struct PreviewMintCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `previewRedeem` function with signature `previewRedeem(uint256)` and selector `0x4cdad506`
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
    #[ethcall(name = "previewRedeem", abi = "previewRedeem(uint256)")]
    pub struct PreviewRedeemCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `previewWithdraw` function with signature `previewWithdraw(uint256)` and selector `0x0a28a477`
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
    #[ethcall(name = "previewWithdraw", abi = "previewWithdraw(uint256)")]
    pub struct PreviewWithdrawCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `redeem` function with signature `redeem(uint256,address,address)` and selector `0xba087652`
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
    #[ethcall(name = "redeem", abi = "redeem(uint256,address,address)")]
    pub struct RedeemCall {
        pub shares: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `totalAssets` function with signature `totalAssets()` and selector `0x01e1d114`
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
    #[ethcall(name = "totalAssets", abi = "totalAssets()")]
    pub struct TotalAssetsCall;
    ///Container type for all input parameters for the `totalAssets` function with signature `totalAssets(uint256)` and selector `0x94605857`
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
    #[ethcall(name = "totalAssets", abi = "totalAssets(uint256)")]
    pub struct TotalAssetsWithTimestampCall {
        pub timestamp: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256,address,address)` and selector `0xb460af94`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,address,address)")]
    pub struct WithdrawCall {
        pub assets: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
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
    pub enum MockERC4626Calls {
        DomainSeparator(DomainSeparatorCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        Asset(AssetCall),
        Authority(AuthorityCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        BurnWithDestination(BurnWithDestinationCall),
        CanCall(CanCallCall),
        ConvertToAssets(ConvertToAssetsCall),
        ConvertToShares(ConvertToSharesCall),
        Decimals(DecimalsCall),
        Deposit(DepositCall),
        DoesRoleHaveCapability(DoesRoleHaveCapabilityCall),
        DoesUserHaveRole(DoesUserHaveRoleCall),
        GetRate(GetRateCall),
        GetRolesWithCapability(GetRolesWithCapabilityCall),
        GetTargetCustomAuthority(GetTargetCustomAuthorityCall),
        GetUserRoles(GetUserRolesCall),
        IsCapabilityPublic(IsCapabilityPublicCall),
        IsCompetitionMode(IsCompetitionModeCall),
        IsUnrestricted(IsUnrestrictedCall),
        MaxDeposit(MaxDepositCall),
        MaxMint(MaxMintCall),
        MaxMintAmount(MaxMintAmountCall),
        MaxRedeem(MaxRedeemCall),
        MaxWithdraw(MaxWithdrawCall),
        MintWithDestination(MintWithDestinationCall),
        MintWithShares(MintWithSharesCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        Permit(PermitCall),
        PreviewDeposit(PreviewDepositCall),
        PreviewMint(PreviewMintCall),
        PreviewRedeem(PreviewRedeemCall),
        PreviewWithdraw(PreviewWithdrawCall),
        Redeem(RedeemCall),
        SetAuthority(SetAuthorityCall),
        SetMaxMintAmount(SetMaxMintAmountCall),
        SetPublicCapability(SetPublicCapabilityCall),
        SetRate(SetRateCall),
        SetRoleCapability(SetRoleCapabilityCall),
        SetTargetCustomAuthority(SetTargetCustomAuthorityCall),
        SetUnrestrictedMintStatus(SetUnrestrictedMintStatusCall),
        SetUserRole(SetUserRoleCall),
        Symbol(SymbolCall),
        TotalAssets(TotalAssetsCall),
        TotalAssetsWithTimestamp(TotalAssetsWithTimestampCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockERC4626Calls {
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
            if let Ok(decoded) = <AssetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Asset(decoded));
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
            if let Ok(decoded) = <ConvertToAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConvertToAssets(decoded));
            }
            if let Ok(decoded) = <ConvertToSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConvertToShares(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
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
            if let Ok(decoded) = <MaxDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxDeposit(decoded));
            }
            if let Ok(decoded) = <MaxMintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxMint(decoded));
            }
            if let Ok(decoded) = <MaxMintAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxMintAmount(decoded));
            }
            if let Ok(decoded) = <MaxRedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxRedeem(decoded));
            }
            if let Ok(decoded) = <MaxWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxWithdraw(decoded));
            }
            if let Ok(decoded) = <MintWithDestinationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintWithDestination(decoded));
            }
            if let Ok(decoded) = <MintWithSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintWithShares(decoded));
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
            if let Ok(decoded) = <PreviewDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreviewDeposit(decoded));
            }
            if let Ok(decoded) = <PreviewMintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreviewMint(decoded));
            }
            if let Ok(decoded) = <PreviewRedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreviewRedeem(decoded));
            }
            if let Ok(decoded) = <PreviewWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreviewWithdraw(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Redeem(decoded));
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
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalAssets(decoded));
            }
            if let Ok(decoded) = <TotalAssetsWithTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalAssetsWithTimestamp(decoded));
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
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockERC4626Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Asset(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::ConvertToAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertToShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DoesRoleHaveCapability(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DoesUserHaveRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRate(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::MaxDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxMint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxMintAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxRedeem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintWithDestination(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintWithShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PreviewDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviewMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviewRedeem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviewWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Redeem(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalAssetsWithTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockERC4626Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::Asset(element) => ::core::fmt::Display::fmt(element, f),
                Self::Authority(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnWithDestination(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CanCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DoesRoleHaveCapability(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DoesUserHaveRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRate(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::MaxDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxMintAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxRedeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintWithDestination(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MintWithShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewRedeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::Redeem(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalAssetsWithTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for MockERC4626Calls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for MockERC4626Calls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for MockERC4626Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<AssetCall> for MockERC4626Calls {
        fn from(value: AssetCall) -> Self {
            Self::Asset(value)
        }
    }
    impl ::core::convert::From<AuthorityCall> for MockERC4626Calls {
        fn from(value: AuthorityCall) -> Self {
            Self::Authority(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MockERC4626Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for MockERC4626Calls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<BurnWithDestinationCall> for MockERC4626Calls {
        fn from(value: BurnWithDestinationCall) -> Self {
            Self::BurnWithDestination(value)
        }
    }
    impl ::core::convert::From<CanCallCall> for MockERC4626Calls {
        fn from(value: CanCallCall) -> Self {
            Self::CanCall(value)
        }
    }
    impl ::core::convert::From<ConvertToAssetsCall> for MockERC4626Calls {
        fn from(value: ConvertToAssetsCall) -> Self {
            Self::ConvertToAssets(value)
        }
    }
    impl ::core::convert::From<ConvertToSharesCall> for MockERC4626Calls {
        fn from(value: ConvertToSharesCall) -> Self {
            Self::ConvertToShares(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for MockERC4626Calls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DepositCall> for MockERC4626Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DoesRoleHaveCapabilityCall> for MockERC4626Calls {
        fn from(value: DoesRoleHaveCapabilityCall) -> Self {
            Self::DoesRoleHaveCapability(value)
        }
    }
    impl ::core::convert::From<DoesUserHaveRoleCall> for MockERC4626Calls {
        fn from(value: DoesUserHaveRoleCall) -> Self {
            Self::DoesUserHaveRole(value)
        }
    }
    impl ::core::convert::From<GetRateCall> for MockERC4626Calls {
        fn from(value: GetRateCall) -> Self {
            Self::GetRate(value)
        }
    }
    impl ::core::convert::From<GetRolesWithCapabilityCall> for MockERC4626Calls {
        fn from(value: GetRolesWithCapabilityCall) -> Self {
            Self::GetRolesWithCapability(value)
        }
    }
    impl ::core::convert::From<GetTargetCustomAuthorityCall> for MockERC4626Calls {
        fn from(value: GetTargetCustomAuthorityCall) -> Self {
            Self::GetTargetCustomAuthority(value)
        }
    }
    impl ::core::convert::From<GetUserRolesCall> for MockERC4626Calls {
        fn from(value: GetUserRolesCall) -> Self {
            Self::GetUserRoles(value)
        }
    }
    impl ::core::convert::From<IsCapabilityPublicCall> for MockERC4626Calls {
        fn from(value: IsCapabilityPublicCall) -> Self {
            Self::IsCapabilityPublic(value)
        }
    }
    impl ::core::convert::From<IsCompetitionModeCall> for MockERC4626Calls {
        fn from(value: IsCompetitionModeCall) -> Self {
            Self::IsCompetitionMode(value)
        }
    }
    impl ::core::convert::From<IsUnrestrictedCall> for MockERC4626Calls {
        fn from(value: IsUnrestrictedCall) -> Self {
            Self::IsUnrestricted(value)
        }
    }
    impl ::core::convert::From<MaxDepositCall> for MockERC4626Calls {
        fn from(value: MaxDepositCall) -> Self {
            Self::MaxDeposit(value)
        }
    }
    impl ::core::convert::From<MaxMintCall> for MockERC4626Calls {
        fn from(value: MaxMintCall) -> Self {
            Self::MaxMint(value)
        }
    }
    impl ::core::convert::From<MaxMintAmountCall> for MockERC4626Calls {
        fn from(value: MaxMintAmountCall) -> Self {
            Self::MaxMintAmount(value)
        }
    }
    impl ::core::convert::From<MaxRedeemCall> for MockERC4626Calls {
        fn from(value: MaxRedeemCall) -> Self {
            Self::MaxRedeem(value)
        }
    }
    impl ::core::convert::From<MaxWithdrawCall> for MockERC4626Calls {
        fn from(value: MaxWithdrawCall) -> Self {
            Self::MaxWithdraw(value)
        }
    }
    impl ::core::convert::From<MintWithDestinationCall> for MockERC4626Calls {
        fn from(value: MintWithDestinationCall) -> Self {
            Self::MintWithDestination(value)
        }
    }
    impl ::core::convert::From<MintWithSharesCall> for MockERC4626Calls {
        fn from(value: MintWithSharesCall) -> Self {
            Self::MintWithShares(value)
        }
    }
    impl ::core::convert::From<MintCall> for MockERC4626Calls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for MockERC4626Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for MockERC4626Calls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MockERC4626Calls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PermitCall> for MockERC4626Calls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<PreviewDepositCall> for MockERC4626Calls {
        fn from(value: PreviewDepositCall) -> Self {
            Self::PreviewDeposit(value)
        }
    }
    impl ::core::convert::From<PreviewMintCall> for MockERC4626Calls {
        fn from(value: PreviewMintCall) -> Self {
            Self::PreviewMint(value)
        }
    }
    impl ::core::convert::From<PreviewRedeemCall> for MockERC4626Calls {
        fn from(value: PreviewRedeemCall) -> Self {
            Self::PreviewRedeem(value)
        }
    }
    impl ::core::convert::From<PreviewWithdrawCall> for MockERC4626Calls {
        fn from(value: PreviewWithdrawCall) -> Self {
            Self::PreviewWithdraw(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for MockERC4626Calls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<SetAuthorityCall> for MockERC4626Calls {
        fn from(value: SetAuthorityCall) -> Self {
            Self::SetAuthority(value)
        }
    }
    impl ::core::convert::From<SetMaxMintAmountCall> for MockERC4626Calls {
        fn from(value: SetMaxMintAmountCall) -> Self {
            Self::SetMaxMintAmount(value)
        }
    }
    impl ::core::convert::From<SetPublicCapabilityCall> for MockERC4626Calls {
        fn from(value: SetPublicCapabilityCall) -> Self {
            Self::SetPublicCapability(value)
        }
    }
    impl ::core::convert::From<SetRateCall> for MockERC4626Calls {
        fn from(value: SetRateCall) -> Self {
            Self::SetRate(value)
        }
    }
    impl ::core::convert::From<SetRoleCapabilityCall> for MockERC4626Calls {
        fn from(value: SetRoleCapabilityCall) -> Self {
            Self::SetRoleCapability(value)
        }
    }
    impl ::core::convert::From<SetTargetCustomAuthorityCall> for MockERC4626Calls {
        fn from(value: SetTargetCustomAuthorityCall) -> Self {
            Self::SetTargetCustomAuthority(value)
        }
    }
    impl ::core::convert::From<SetUnrestrictedMintStatusCall> for MockERC4626Calls {
        fn from(value: SetUnrestrictedMintStatusCall) -> Self {
            Self::SetUnrestrictedMintStatus(value)
        }
    }
    impl ::core::convert::From<SetUserRoleCall> for MockERC4626Calls {
        fn from(value: SetUserRoleCall) -> Self {
            Self::SetUserRole(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for MockERC4626Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalAssetsCall> for MockERC4626Calls {
        fn from(value: TotalAssetsCall) -> Self {
            Self::TotalAssets(value)
        }
    }
    impl ::core::convert::From<TotalAssetsWithTimestampCall> for MockERC4626Calls {
        fn from(value: TotalAssetsWithTimestampCall) -> Self {
            Self::TotalAssetsWithTimestamp(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for MockERC4626Calls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for MockERC4626Calls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for MockERC4626Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for MockERC4626Calls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for MockERC4626Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
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
    ///Container type for all return fields from the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    pub struct AssetReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `convertToAssets` function with signature `convertToAssets(uint256)` and selector `0x07a2d13a`
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
    pub struct ConvertToAssetsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `convertToShares` function with signature `convertToShares(uint256)` and selector `0xc6e6f592`
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
    pub struct ConvertToSharesReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `deposit` function with signature `deposit(uint256,address)` and selector `0x6e553f65`
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
    pub struct DepositReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `maxDeposit` function with signature `maxDeposit(address)` and selector `0x402d267d`
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
    pub struct MaxDepositReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxMint` function with signature `maxMint(address)` and selector `0xc63d75b6`
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
    pub struct MaxMintReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `maxRedeem` function with signature `maxRedeem(address)` and selector `0xd905777e`
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
    pub struct MaxRedeemReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxWithdraw` function with signature `maxWithdraw(address)` and selector `0xce96cb77`
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
    pub struct MaxWithdrawReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mint` function with signature `mint(uint256,address)` and selector `0x94bf804d`
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
    pub struct MintWithSharesReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `previewDeposit` function with signature `previewDeposit(uint256)` and selector `0xef8b30f7`
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
    pub struct PreviewDepositReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `previewMint` function with signature `previewMint(uint256)` and selector `0xb3d7f6b9`
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
    pub struct PreviewMintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `previewRedeem` function with signature `previewRedeem(uint256)` and selector `0x4cdad506`
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
    pub struct PreviewRedeemReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `previewWithdraw` function with signature `previewWithdraw(uint256)` and selector `0x0a28a477`
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
    pub struct PreviewWithdrawReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `redeem` function with signature `redeem(uint256,address,address)` and selector `0xba087652`
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
    pub struct RedeemReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `totalAssets` function with signature `totalAssets()` and selector `0x01e1d114`
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
    pub struct TotalAssetsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalAssets` function with signature `totalAssets(uint256)` and selector `0x94605857`
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
    pub struct TotalAssetsWithTimestampReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `withdraw` function with signature `withdraw(uint256,address,address)` and selector `0xb460af94`
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
    pub struct WithdrawReturn(pub ::ethers::core::types::U256);
}
