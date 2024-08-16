pub use mock_yield_space_math::*;
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
pub mod mock_yield_space_math {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "calculateBondsOutGivenSharesInDown",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateBondsOutGivenSharesInDown",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ze"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dz"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("t"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("c"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mu"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateMaxBuyBondsOutSafe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateMaxBuyBondsOutSafe",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ze"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("t"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("c"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mu"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateMaxBuySharesInSafe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateMaxBuySharesInSafe",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ze"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("t"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("c"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mu"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateMaxSellBondsInSafe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateMaxSellBondsInSafe",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("z"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("zeta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("zMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("t"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("c"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mu"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "calculateSharesInGivenBondsOutDown",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateSharesInGivenBondsOutDown",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ze"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("t"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("c"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mu"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateSharesInGivenBondsOutUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateSharesInGivenBondsOutUp",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ze"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("t"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("c"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mu"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "calculateSharesOutGivenBondsInDown",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateSharesOutGivenBondsInDown",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ze"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("t"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("c"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mu"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "calculateSharesOutGivenBondsInDownSafe",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateSharesOutGivenBondsInDownSafe",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ze"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("t"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("c"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mu"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("kDown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("kDown"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ze"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("t"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("c"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mu"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("kUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("kUp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ze"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("t"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("c"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mu"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("LnInvalidInput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("LnInvalidInput"),
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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKYIELDSPACEMATH_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0F\xC6\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c\x86\xC8^\x10\x11a\0fW\x80c\x86\xC8^\x10\x14a\x01*W\x80c\xA4\xA6\xF9\xD9\x14a\x01=W\x80c\xA9\xAF\xA3s\x14a\x01PW\x80c\xBCP\xEB\xE6\x14a\x01cW\x80c\xD7\x9D\x085\x14a\x01vW`\0\x80\xFD[\x80c\n\xEAuc\x14a\0\xA3W\x80c'\xD0\xE2e\x14a\0\xC9W\x80c.t\x10\x8C\x14a\0\xDCW\x80c>\xE4\x11J\x14a\0\xEFW\x80cX\x80\xB9\xFD\x14a\x01\x17W[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x0E\x1EV[a\x01\x89V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\0\xD76`\x04a\x0EYV[a\x01\xA4V[a\0\xB6a\0\xEA6`\x04a\x0EYV[a\x01\xC1V[a\x01\x02a\0\xFD6`\x04a\x0EYV[a\x01\xD2V[`@\x80Q\x92\x83R\x90\x15\x15` \x83\x01R\x01a\0\xC0V[a\x01\x02a\x01%6`\x04a\x0E\x1EV[a\x01\xF9V[a\0\xB6a\x0186`\x04a\x0EYV[a\x02\x1EV[a\0\xB6a\x01K6`\x04a\x0EYV[a\x02/V[a\x01\x02a\x01^6`\x04a\x0E\x1EV[a\x02@V[a\x01\x02a\x01q6`\x04a\x0E\x9CV[a\x02SV[a\0\xB6a\x01\x846`\x04a\x0E\x1EV[a\x02|V[`\0\x80a\x01\x99\x87\x87\x87\x87\x87a\x02\x8CV[\x97\x96PPPPPPPV[`\0\x80a\x01\xB5\x88\x88\x88\x88\x88\x88a\x02\xCBV[\x98\x97PPPPPPPPV[`\0\x80a\x01\xB5\x88\x88\x88\x88\x88\x88a\x02\xF9V[`\0\x80`\0\x80a\x01\xE6\x8A\x8A\x8A\x8A\x8A\x8Aa\x03\nV[\x90\x94P\x92PPP[\x96P\x96\x94PPPPPV[`\0\x80`\0\x80a\x02\x0C\x89\x89\x89\x89\x89a\x03\xD1V[\x90\x94P\x92PPP[\x95P\x95\x93PPPPV[`\0\x80a\x01\xB5\x88\x88\x88\x88\x88\x88a\x04\x87V[`\0\x80a\x01\xB5\x88\x88\x88\x88\x88\x88a\x04\x98V[`\0\x80`\0\x80a\x02\x0C\x89\x89\x89\x89\x89a\x05YV[`\0\x80`\0\x80a\x02h\x8B\x8B\x8B\x8B\x8B\x8B\x8Ba\x06\x02V[\x90\x94P\x92PPP[\x97P\x97\x95PPPPPPV[`\0\x80a\x01\x99\x87\x87\x87\x87\x87a\x07\x04V[`\0a\x02\x98\x85\x85a\x07)V[a\x02\xB7a\x02\xAF\x86a\x02\xA9\x86\x8Ba\x07\xA0V[\x90a\x07)V[\x85\x90\x85a\x07\xBCV[a\x02\xC1\x91\x90a\x0E\xFEV[\x96\x95PPPPPPV[`\0\x80a\x02\xDC\x88\x88\x88\x88\x88\x88a\x03\nV[\x90\x92P\x90P\x80a\x02\xEEWa\x02\xEEa\x07\xE2V[P\x96\x95PPPPPPV[`\0\x80a\x02\xDC\x88\x88\x88\x88\x88\x88a\x07\xFBV[`\0\x80`\0a\x03\x1C\x89\x89\x88\x88\x88a\x02\x8CV[\x90Pa\x03,\x86a\x02\xA9\x89\x8Ba\x0E\xFEV[\x97P\x87\x81\x10\x15a\x03CW`\0\x80\x92P\x92PPa\x01\xEEV[\x87\x81\x03a\x03Q\x81\x86\x88a\x07\xBCV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x03\x85Wa\x03~a\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xD2V[\x82\x90a\x07)V[\x90Pa\x03\x9DV[a\x03\x9Aa\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xE7V[\x90P[a\x03\xA7\x81\x86a\x08\xD2V[\x90P\x80\x8A\x10\x15a\x03\xBFW`\0\x80\x93P\x93PPPa\x01\xEEV[\x90\x98\x03\x98`\x01\x98P\x96PPPPPPPV[`\0\x80`\0a\x03\xE3\x88\x88\x88\x88\x88a\x07\x04V[\x90P`\0a\x04\x0Eg\r\xE0\xB6\xB3\xA7d\0\0a\x03\xFD\x88\x88a\x08\xD2V[a\x04\x07\x91\x90a\x0E\xFEV[\x83\x90a\x08\xE7V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x04;Wa\x044a\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xE7V[\x90Pa\x04SV[a\x04Pa\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xD2V[\x90P[a\x04]\x81\x86a\x08\xE7V[\x90P\x88\x81\x10\x15a\x04uW`\0\x80\x93P\x93PPPa\x02\x14V[\x97\x90\x97\x03\x97`\x01\x97P\x95PPPPPPV[`\0\x80a\x02\xDC\x88\x88\x88\x88\x88\x88a\x08\xFCV[`\0\x80a\x04\xA8\x88\x88\x87\x87\x87a\x07\x04V[\x90P\x85\x87\x10\x15a\x04\xBAWa\x04\xBAa\x07\xE2V[\x95\x85\x90\x03\x95a\x04\xC9\x87\x86a\x07)V[\x96P\x86\x81\x10\x15a\x04\xDBWa\x04\xDBa\x07\xE2V[\x86\x81\x03a\x04\xE9\x81\x85\x87a\t\xBAV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x05\x16Wa\x05\x0Fa\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x88a\x08\xE7V[\x90Pa\x05.V[a\x05+a\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x88a\x08\xD2V[\x90P[a\x058\x81\x85a\x08\xE7V[\x90P\x88\x81\x10\x15a\x05JWa\x05Ja\x07\xE2V[\x97\x90\x97\x03\x97\x96PPPPPPPV[`\0\x80`\0a\x05k\x88\x88\x88\x88\x88a\x02\x8CV[\x90P`\0a\x05\x96g\r\xE0\xB6\xB3\xA7d\0\0a\x05\x85\x88\x88a\x08\xE7V[a\x05\x8F\x91\x90a\x0E\xFEV[\x83\x90a\x08\xD2V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x05\xC3Wa\x05\xBCa\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xD2V[\x90Pa\x05\xDBV[a\x05\xD8a\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xE7V[\x90P[\x80\x88\x10\x15a\x05\xF1W`\0\x80\x93P\x93PPPa\x02\x14V[\x90\x96\x03\x97`\x01\x97P\x95PPPPPPV[`\0\x80`\0\x88\x12\x15a\x06$Wa\x06\x17\x88a\x0F\x11V[a\x06!\x90\x87a\x0E\xFEV[\x95P[`\0\x80a\x061\x8B\x8Ba\t\xD8V[\x91P\x91P\x80a\x06HW`\0\x80\x93P\x93PPPa\x02pV[`\0a\x06W\x83\x8B\x8A\x8A\x8Aa\x07\x04V[\x90P`\0a\x06ta\x06l\x8Aa\x02\xA9\x8A\x8Ea\x07\xA0V[\x89\x90\x89a\x07\xBCV[\x90P\x80\x82\x10\x15a\x06\x8EW`\0\x80\x95P\x95PPPPPa\x02pV[\x80\x82\x03g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x06\xBCWa\x06\xB5a\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x8Ca\x08\xE7V[\x90Pa\x06\xD4V[a\x06\xD1a\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x8Ca\x08\xD2V[\x90P[\x8B\x81\x10\x15a\x06\xEDW`\0\x80\x96P\x96PPPPPPa\x02pV[\x9A\x90\x9A\x03\x9C`\x01\x9CP\x9APPPPPPPPPPPV[`\0a\x07\x10\x85\x85a\x07)V[a\x02\xB7a\x07!\x86a\x02\xA9\x86\x8Ba\n\x17V[\x85\x90\x85a\t\xBAV[`\0\x81`\0\x03a\x07BWPg\r\xE0\xB6\xB3\xA7d\0\0a\x07\x9AV[\x82`\0\x03a\x07RWP`\0a\x07\x9AV[`\0a\x07]\x83a\n,V[\x90P`\0a\x07ra\x07m\x86a\n,V[a\nZV[\x90P\x81\x81\x02a\x07\x89g\r\xE0\xB6\xB3\xA7d\0\0\x82a\x0F-V[\x90Pa\x07\x94\x81a\x0C\x89V[\x93PPPP[\x92\x91PPV[`\0a\x07\xB5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x07\xBCV[\x93\x92PPPV[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x07\xD3W`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`@Qc\xBBU\xFD'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0a\x08\r\x89\x89\x88\x88\x88a\x02\x8CV[\x90P\x86\x88\x10\x15a\x08$W`\0\x80\x92P\x92PPa\x01\xEEV[\x96\x86\x90\x03\x96a\x083\x88\x87a\x07)V[\x97P\x87\x81\x10\x15a\x08JW`\0\x80\x92P\x92PPa\x01\xEEV[\x87\x81\x03a\x08X\x81\x86\x88a\x07\xBCV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x08\x85Wa\x08~a\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xD2V[\x90Pa\x08\x9DV[a\x08\x9Aa\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xE7V[\x90P[a\x08\xA7\x81\x86a\x08\xD2V[\x90P\x89\x81\x10\x15a\x08\xBFW`\0\x80\x93P\x93PPPa\x01\xEEV[\x98\x90\x98\x03\x98`\x01\x98P\x96PPPPPPPV[`\0a\x07\xB5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x07\xBCV[`\0a\x07\xB5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\t\xBAV[`\0\x80`\0a\t\x0E\x89\x89\x88\x88\x88a\x02\x8CV[\x90Pa\t(\x86a\x02\xA9a\t!\x8A\x8Da\x0E\xFEV[\x87\x90a\n\x17V[\x98Pa\t5\x85\x8A\x86a\t\xBAV[\x98P\x88\x81\x10\x15a\tLW`\0\x80\x92P\x92PPa\x01\xEEV[\x88\x81\x03g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\tzWa\tsa\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xD2V[\x90Pa\t\x92V[a\t\x8Fa\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xE7V[\x90P[\x80\x89\x10\x15a\t\xA8W`\0\x80\x93P\x93PPPa\x01\xEEV[\x90\x97\x03\x98`\x01\x98P\x96PPPPPPPV[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\t\xD1W`\0\x80\xFD[P\x91\x02\x04\x90V[`\0\x80`\0\x83a\t\xE7\x86a\n,V[a\t\xF1\x91\x90a\x0FiV[\x90P`\0\x81\x12\x15a\n\tW`\0\x80\x92P\x92PPa\n\x10V[\x91P`\x01\x90P[\x92P\x92\x90PV[`\0a\x07\xB5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\t\xBAV[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\nVW`@Qc9n\xA7\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0\x80\x82\x13a\n|W`@Qc\xE6\x1BIu`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x82\x81\x1C`\x0F\x10`\x02\x1B\x17\x82\x81\x1C\x90\x91\x10`\x01\x90\x81\x1B\x90\x91\x17\x82\x81\x1C\x90\x91\x10\x17`\x9F\x81\x81\x03``\x01\x92\x90\x92\x1B\x91`_\x19\x82\x01\x90a\x0B\x08\x90\x84\x90\x1Ca\n,V[lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x91\x90\x91\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x0C\xA4WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x0C\xCDW`@Qcs\xA2\xD6\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x92l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x84\x01\x84\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x85\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x85\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x85\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x85\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x87\x01\x87\x02\x83\x1D\x90\x81\x01\x90\x87\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x81\x02\x90\x92\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x86\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x81\x81\x05\x95P\x92\x93P\x90\x91\x90a\x02\xC1t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x86\x02`\xC3\x86\x90\x03\x1Ca\n,V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x0E6W`\0\x80\xFD[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x0ErW`\0\x80\xFD[PP\x845\x96` \x86\x015\x96P`@\x86\x015\x95``\x81\x015\x95P`\x80\x81\x015\x94P`\xA0\x015\x92P\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x0E\xB7W`\0\x80\xFD[PP\x855\x97` \x87\x015\x97P`@\x87\x015\x96``\x81\x015\x96P`\x80\x81\x015\x95P`\xA0\x81\x015\x94P`\xC0\x015\x92P\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07\x9AWa\x07\x9Aa\x0E\xE8V[`\0`\x01`\xFF\x1B\x82\x01a\x0F&Wa\x0F&a\x0E\xE8V[P`\0\x03\x90V[`\0\x82a\x0FJWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x0FdWa\x0Fda\x0E\xE8V[P\x05\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x0F\x89Wa\x0F\x89a\x0E\xE8V[P\x92\x91PPV\xFE\xA2dipfsX\"\x12 :\xB7IokQ6:\xFB\xAF?\xEBT\x89\xC4&\xB7h*\x1C\xFEb\x13\xEE\x8D\x1CdYo\x0C\x14\xFEdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static MOCKYIELDSPACEMATH_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c\x86\xC8^\x10\x11a\0fW\x80c\x86\xC8^\x10\x14a\x01*W\x80c\xA4\xA6\xF9\xD9\x14a\x01=W\x80c\xA9\xAF\xA3s\x14a\x01PW\x80c\xBCP\xEB\xE6\x14a\x01cW\x80c\xD7\x9D\x085\x14a\x01vW`\0\x80\xFD[\x80c\n\xEAuc\x14a\0\xA3W\x80c'\xD0\xE2e\x14a\0\xC9W\x80c.t\x10\x8C\x14a\0\xDCW\x80c>\xE4\x11J\x14a\0\xEFW\x80cX\x80\xB9\xFD\x14a\x01\x17W[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x0E\x1EV[a\x01\x89V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\0\xD76`\x04a\x0EYV[a\x01\xA4V[a\0\xB6a\0\xEA6`\x04a\x0EYV[a\x01\xC1V[a\x01\x02a\0\xFD6`\x04a\x0EYV[a\x01\xD2V[`@\x80Q\x92\x83R\x90\x15\x15` \x83\x01R\x01a\0\xC0V[a\x01\x02a\x01%6`\x04a\x0E\x1EV[a\x01\xF9V[a\0\xB6a\x0186`\x04a\x0EYV[a\x02\x1EV[a\0\xB6a\x01K6`\x04a\x0EYV[a\x02/V[a\x01\x02a\x01^6`\x04a\x0E\x1EV[a\x02@V[a\x01\x02a\x01q6`\x04a\x0E\x9CV[a\x02SV[a\0\xB6a\x01\x846`\x04a\x0E\x1EV[a\x02|V[`\0\x80a\x01\x99\x87\x87\x87\x87\x87a\x02\x8CV[\x97\x96PPPPPPPV[`\0\x80a\x01\xB5\x88\x88\x88\x88\x88\x88a\x02\xCBV[\x98\x97PPPPPPPPV[`\0\x80a\x01\xB5\x88\x88\x88\x88\x88\x88a\x02\xF9V[`\0\x80`\0\x80a\x01\xE6\x8A\x8A\x8A\x8A\x8A\x8Aa\x03\nV[\x90\x94P\x92PPP[\x96P\x96\x94PPPPPV[`\0\x80`\0\x80a\x02\x0C\x89\x89\x89\x89\x89a\x03\xD1V[\x90\x94P\x92PPP[\x95P\x95\x93PPPPV[`\0\x80a\x01\xB5\x88\x88\x88\x88\x88\x88a\x04\x87V[`\0\x80a\x01\xB5\x88\x88\x88\x88\x88\x88a\x04\x98V[`\0\x80`\0\x80a\x02\x0C\x89\x89\x89\x89\x89a\x05YV[`\0\x80`\0\x80a\x02h\x8B\x8B\x8B\x8B\x8B\x8B\x8Ba\x06\x02V[\x90\x94P\x92PPP[\x97P\x97\x95PPPPPPV[`\0\x80a\x01\x99\x87\x87\x87\x87\x87a\x07\x04V[`\0a\x02\x98\x85\x85a\x07)V[a\x02\xB7a\x02\xAF\x86a\x02\xA9\x86\x8Ba\x07\xA0V[\x90a\x07)V[\x85\x90\x85a\x07\xBCV[a\x02\xC1\x91\x90a\x0E\xFEV[\x96\x95PPPPPPV[`\0\x80a\x02\xDC\x88\x88\x88\x88\x88\x88a\x03\nV[\x90\x92P\x90P\x80a\x02\xEEWa\x02\xEEa\x07\xE2V[P\x96\x95PPPPPPV[`\0\x80a\x02\xDC\x88\x88\x88\x88\x88\x88a\x07\xFBV[`\0\x80`\0a\x03\x1C\x89\x89\x88\x88\x88a\x02\x8CV[\x90Pa\x03,\x86a\x02\xA9\x89\x8Ba\x0E\xFEV[\x97P\x87\x81\x10\x15a\x03CW`\0\x80\x92P\x92PPa\x01\xEEV[\x87\x81\x03a\x03Q\x81\x86\x88a\x07\xBCV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x03\x85Wa\x03~a\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xD2V[\x82\x90a\x07)V[\x90Pa\x03\x9DV[a\x03\x9Aa\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xE7V[\x90P[a\x03\xA7\x81\x86a\x08\xD2V[\x90P\x80\x8A\x10\x15a\x03\xBFW`\0\x80\x93P\x93PPPa\x01\xEEV[\x90\x98\x03\x98`\x01\x98P\x96PPPPPPPV[`\0\x80`\0a\x03\xE3\x88\x88\x88\x88\x88a\x07\x04V[\x90P`\0a\x04\x0Eg\r\xE0\xB6\xB3\xA7d\0\0a\x03\xFD\x88\x88a\x08\xD2V[a\x04\x07\x91\x90a\x0E\xFEV[\x83\x90a\x08\xE7V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x04;Wa\x044a\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xE7V[\x90Pa\x04SV[a\x04Pa\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xD2V[\x90P[a\x04]\x81\x86a\x08\xE7V[\x90P\x88\x81\x10\x15a\x04uW`\0\x80\x93P\x93PPPa\x02\x14V[\x97\x90\x97\x03\x97`\x01\x97P\x95PPPPPPV[`\0\x80a\x02\xDC\x88\x88\x88\x88\x88\x88a\x08\xFCV[`\0\x80a\x04\xA8\x88\x88\x87\x87\x87a\x07\x04V[\x90P\x85\x87\x10\x15a\x04\xBAWa\x04\xBAa\x07\xE2V[\x95\x85\x90\x03\x95a\x04\xC9\x87\x86a\x07)V[\x96P\x86\x81\x10\x15a\x04\xDBWa\x04\xDBa\x07\xE2V[\x86\x81\x03a\x04\xE9\x81\x85\x87a\t\xBAV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x05\x16Wa\x05\x0Fa\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x88a\x08\xE7V[\x90Pa\x05.V[a\x05+a\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x88a\x08\xD2V[\x90P[a\x058\x81\x85a\x08\xE7V[\x90P\x88\x81\x10\x15a\x05JWa\x05Ja\x07\xE2V[\x97\x90\x97\x03\x97\x96PPPPPPPV[`\0\x80`\0a\x05k\x88\x88\x88\x88\x88a\x02\x8CV[\x90P`\0a\x05\x96g\r\xE0\xB6\xB3\xA7d\0\0a\x05\x85\x88\x88a\x08\xE7V[a\x05\x8F\x91\x90a\x0E\xFEV[\x83\x90a\x08\xD2V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x05\xC3Wa\x05\xBCa\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xD2V[\x90Pa\x05\xDBV[a\x05\xD8a\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xE7V[\x90P[\x80\x88\x10\x15a\x05\xF1W`\0\x80\x93P\x93PPPa\x02\x14V[\x90\x96\x03\x97`\x01\x97P\x95PPPPPPV[`\0\x80`\0\x88\x12\x15a\x06$Wa\x06\x17\x88a\x0F\x11V[a\x06!\x90\x87a\x0E\xFEV[\x95P[`\0\x80a\x061\x8B\x8Ba\t\xD8V[\x91P\x91P\x80a\x06HW`\0\x80\x93P\x93PPPa\x02pV[`\0a\x06W\x83\x8B\x8A\x8A\x8Aa\x07\x04V[\x90P`\0a\x06ta\x06l\x8Aa\x02\xA9\x8A\x8Ea\x07\xA0V[\x89\x90\x89a\x07\xBCV[\x90P\x80\x82\x10\x15a\x06\x8EW`\0\x80\x95P\x95PPPPPa\x02pV[\x80\x82\x03g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x06\xBCWa\x06\xB5a\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x8Ca\x08\xE7V[\x90Pa\x06\xD4V[a\x06\xD1a\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x8Ca\x08\xD2V[\x90P[\x8B\x81\x10\x15a\x06\xEDW`\0\x80\x96P\x96PPPPPPa\x02pV[\x9A\x90\x9A\x03\x9C`\x01\x9CP\x9APPPPPPPPPPPV[`\0a\x07\x10\x85\x85a\x07)V[a\x02\xB7a\x07!\x86a\x02\xA9\x86\x8Ba\n\x17V[\x85\x90\x85a\t\xBAV[`\0\x81`\0\x03a\x07BWPg\r\xE0\xB6\xB3\xA7d\0\0a\x07\x9AV[\x82`\0\x03a\x07RWP`\0a\x07\x9AV[`\0a\x07]\x83a\n,V[\x90P`\0a\x07ra\x07m\x86a\n,V[a\nZV[\x90P\x81\x81\x02a\x07\x89g\r\xE0\xB6\xB3\xA7d\0\0\x82a\x0F-V[\x90Pa\x07\x94\x81a\x0C\x89V[\x93PPPP[\x92\x91PPV[`\0a\x07\xB5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x07\xBCV[\x93\x92PPPV[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x07\xD3W`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`@Qc\xBBU\xFD'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0a\x08\r\x89\x89\x88\x88\x88a\x02\x8CV[\x90P\x86\x88\x10\x15a\x08$W`\0\x80\x92P\x92PPa\x01\xEEV[\x96\x86\x90\x03\x96a\x083\x88\x87a\x07)V[\x97P\x87\x81\x10\x15a\x08JW`\0\x80\x92P\x92PPa\x01\xEEV[\x87\x81\x03a\x08X\x81\x86\x88a\x07\xBCV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x08\x85Wa\x08~a\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xD2V[\x90Pa\x08\x9DV[a\x08\x9Aa\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xE7V[\x90P[a\x08\xA7\x81\x86a\x08\xD2V[\x90P\x89\x81\x10\x15a\x08\xBFW`\0\x80\x93P\x93PPPa\x01\xEEV[\x98\x90\x98\x03\x98`\x01\x98P\x96PPPPPPPV[`\0a\x07\xB5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x07\xBCV[`\0a\x07\xB5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\t\xBAV[`\0\x80`\0a\t\x0E\x89\x89\x88\x88\x88a\x02\x8CV[\x90Pa\t(\x86a\x02\xA9a\t!\x8A\x8Da\x0E\xFEV[\x87\x90a\n\x17V[\x98Pa\t5\x85\x8A\x86a\t\xBAV[\x98P\x88\x81\x10\x15a\tLW`\0\x80\x92P\x92PPa\x01\xEEV[\x88\x81\x03g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\tzWa\tsa\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xD2V[\x90Pa\t\x92V[a\t\x8Fa\x03wg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x08\xE7V[\x90P[\x80\x89\x10\x15a\t\xA8W`\0\x80\x93P\x93PPPa\x01\xEEV[\x90\x97\x03\x98`\x01\x98P\x96PPPPPPPV[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\t\xD1W`\0\x80\xFD[P\x91\x02\x04\x90V[`\0\x80`\0\x83a\t\xE7\x86a\n,V[a\t\xF1\x91\x90a\x0FiV[\x90P`\0\x81\x12\x15a\n\tW`\0\x80\x92P\x92PPa\n\x10V[\x91P`\x01\x90P[\x92P\x92\x90PV[`\0a\x07\xB5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\t\xBAV[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\nVW`@Qc9n\xA7\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0\x80\x82\x13a\n|W`@Qc\xE6\x1BIu`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x82\x81\x1C`\x0F\x10`\x02\x1B\x17\x82\x81\x1C\x90\x91\x10`\x01\x90\x81\x1B\x90\x91\x17\x82\x81\x1C\x90\x91\x10\x17`\x9F\x81\x81\x03``\x01\x92\x90\x92\x1B\x91`_\x19\x82\x01\x90a\x0B\x08\x90\x84\x90\x1Ca\n,V[lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x91\x90\x91\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x0C\xA4WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x0C\xCDW`@Qcs\xA2\xD6\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x92l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x84\x01\x84\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x85\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x85\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x85\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x85\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x87\x01\x87\x02\x83\x1D\x90\x81\x01\x90\x87\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x81\x02\x90\x92\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x86\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x81\x81\x05\x95P\x92\x93P\x90\x91\x90a\x02\xC1t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x86\x02`\xC3\x86\x90\x03\x1Ca\n,V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x0E6W`\0\x80\xFD[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x0ErW`\0\x80\xFD[PP\x845\x96` \x86\x015\x96P`@\x86\x015\x95``\x81\x015\x95P`\x80\x81\x015\x94P`\xA0\x015\x92P\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x0E\xB7W`\0\x80\xFD[PP\x855\x97` \x87\x015\x97P`@\x87\x015\x96``\x81\x015\x96P`\x80\x81\x015\x95P`\xA0\x81\x015\x94P`\xC0\x015\x92P\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07\x9AWa\x07\x9Aa\x0E\xE8V[`\0`\x01`\xFF\x1B\x82\x01a\x0F&Wa\x0F&a\x0E\xE8V[P`\0\x03\x90V[`\0\x82a\x0FJWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x0FdWa\x0Fda\x0E\xE8V[P\x05\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x0F\x89Wa\x0F\x89a\x0E\xE8V[P\x92\x91PPV\xFE\xA2dipfsX\"\x12 :\xB7IokQ6:\xFB\xAF?\xEBT\x89\xC4&\xB7h*\x1C\xFEb\x13\xEE\x8D\x1CdYo\x0C\x14\xFEdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKYIELDSPACEMATH_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockYieldSpaceMath<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockYieldSpaceMath<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockYieldSpaceMath<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockYieldSpaceMath<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockYieldSpaceMath<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockYieldSpaceMath))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockYieldSpaceMath<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKYIELDSPACEMATH_ABI.clone(),
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
                MOCKYIELDSPACEMATH_ABI.clone(),
                MOCKYIELDSPACEMATH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `calculateBondsOutGivenSharesInDown` (0x86c85e10) function
        pub fn calculate_bonds_out_given_shares_in_down(
            &self,
            ze: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            dz: ::ethers::core::types::U256,
            t: ::ethers::core::types::U256,
            c: ::ethers::core::types::U256,
            mu: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([134, 200, 94, 16], (ze, y, dz, t, c, mu))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateMaxBuyBondsOutSafe` (0xa9afa373) function
        pub fn calculate_max_buy_bonds_out_safe(
            &self,
            ze: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            t: ::ethers::core::types::U256,
            c: ::ethers::core::types::U256,
            mu: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, bool),
        > {
            self.0
                .method_hash([169, 175, 163, 115], (ze, y, t, c, mu))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateMaxBuySharesInSafe` (0x5880b9fd) function
        pub fn calculate_max_buy_shares_in_safe(
            &self,
            ze: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            t: ::ethers::core::types::U256,
            c: ::ethers::core::types::U256,
            mu: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, bool),
        > {
            self.0
                .method_hash([88, 128, 185, 253], (ze, y, t, c, mu))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateMaxSellBondsInSafe` (0xbc50ebe6) function
        pub fn calculate_max_sell_bonds_in_safe(
            &self,
            z: ::ethers::core::types::U256,
            zeta: ::ethers::core::types::I256,
            y: ::ethers::core::types::U256,
            z_min: ::ethers::core::types::U256,
            t: ::ethers::core::types::U256,
            c: ::ethers::core::types::U256,
            mu: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, bool),
        > {
            self.0
                .method_hash([188, 80, 235, 230], (z, zeta, y, z_min, t, c, mu))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateSharesInGivenBondsOutDown` (0xa4a6f9d9) function
        pub fn calculate_shares_in_given_bonds_out_down(
            &self,
            ze: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            dy: ::ethers::core::types::U256,
            t: ::ethers::core::types::U256,
            c: ::ethers::core::types::U256,
            mu: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([164, 166, 249, 217], (ze, y, dy, t, c, mu))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateSharesInGivenBondsOutUp` (0x2e74108c) function
        pub fn calculate_shares_in_given_bonds_out_up(
            &self,
            ze: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            dy: ::ethers::core::types::U256,
            t: ::ethers::core::types::U256,
            c: ::ethers::core::types::U256,
            mu: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([46, 116, 16, 140], (ze, y, dy, t, c, mu))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateSharesOutGivenBondsInDown` (0x27d0e265) function
        pub fn calculate_shares_out_given_bonds_in_down(
            &self,
            ze: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            dy: ::ethers::core::types::U256,
            t: ::ethers::core::types::U256,
            c: ::ethers::core::types::U256,
            mu: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([39, 208, 226, 101], (ze, y, dy, t, c, mu))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateSharesOutGivenBondsInDownSafe` (0x3ee4114a) function
        pub fn calculate_shares_out_given_bonds_in_down_safe(
            &self,
            ze: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            dy: ::ethers::core::types::U256,
            t: ::ethers::core::types::U256,
            c: ::ethers::core::types::U256,
            mu: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, bool),
        > {
            self.0
                .method_hash([62, 228, 17, 74], (ze, y, dy, t, c, mu))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `kDown` (0xd79d0835) function
        pub fn k_down(
            &self,
            ze: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            t: ::ethers::core::types::U256,
            c: ::ethers::core::types::U256,
            mu: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([215, 157, 8, 53], (ze, y, t, c, mu))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `kUp` (0x0aea7563) function
        pub fn k_up(
            &self,
            ze: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            t: ::ethers::core::types::U256,
            c: ::ethers::core::types::U256,
            mu: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 234, 117, 99], (ze, y, t, c, mu))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockYieldSpaceMath<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    pub enum MockYieldSpaceMathErrors {
        ExpInvalidExponent(ExpInvalidExponent),
        InsufficientLiquidity(InsufficientLiquidity),
        LnInvalidInput(LnInvalidInput),
        UnsafeCastToInt256(UnsafeCastToInt256),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockYieldSpaceMathErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ExpInvalidExponent as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpInvalidExponent(decoded));
            }
            if let Ok(decoded) = <InsufficientLiquidity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientLiquidity(decoded));
            }
            if let Ok(decoded) = <LnInvalidInput as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LnInvalidInput(decoded));
            }
            if let Ok(decoded) = <UnsafeCastToInt256 as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnsafeCastToInt256(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockYieldSpaceMathErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ExpInvalidExponent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LnInvalidInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsafeCastToInt256(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MockYieldSpaceMathErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ExpInvalidExponent as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientLiquidity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <LnInvalidInput as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnsafeCastToInt256 as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MockYieldSpaceMathErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExpInvalidExponent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LnInvalidInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsafeCastToInt256(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MockYieldSpaceMathErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ExpInvalidExponent> for MockYieldSpaceMathErrors {
        fn from(value: ExpInvalidExponent) -> Self {
            Self::ExpInvalidExponent(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidity> for MockYieldSpaceMathErrors {
        fn from(value: InsufficientLiquidity) -> Self {
            Self::InsufficientLiquidity(value)
        }
    }
    impl ::core::convert::From<LnInvalidInput> for MockYieldSpaceMathErrors {
        fn from(value: LnInvalidInput) -> Self {
            Self::LnInvalidInput(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToInt256> for MockYieldSpaceMathErrors {
        fn from(value: UnsafeCastToInt256) -> Self {
            Self::UnsafeCastToInt256(value)
        }
    }
    ///Container type for all input parameters for the `calculateBondsOutGivenSharesInDown` function with signature `calculateBondsOutGivenSharesInDown(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x86c85e10`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateBondsOutGivenSharesInDown",
        abi = "calculateBondsOutGivenSharesInDown(uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateBondsOutGivenSharesInDownCall {
        pub ze: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub dz: ::ethers::core::types::U256,
        pub t: ::ethers::core::types::U256,
        pub c: ::ethers::core::types::U256,
        pub mu: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateMaxBuyBondsOutSafe` function with signature `calculateMaxBuyBondsOutSafe(uint256,uint256,uint256,uint256,uint256)` and selector `0xa9afa373`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateMaxBuyBondsOutSafe",
        abi = "calculateMaxBuyBondsOutSafe(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateMaxBuyBondsOutSafeCall {
        pub ze: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub t: ::ethers::core::types::U256,
        pub c: ::ethers::core::types::U256,
        pub mu: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateMaxBuySharesInSafe` function with signature `calculateMaxBuySharesInSafe(uint256,uint256,uint256,uint256,uint256)` and selector `0x5880b9fd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateMaxBuySharesInSafe",
        abi = "calculateMaxBuySharesInSafe(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateMaxBuySharesInSafeCall {
        pub ze: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub t: ::ethers::core::types::U256,
        pub c: ::ethers::core::types::U256,
        pub mu: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateMaxSellBondsInSafe` function with signature `calculateMaxSellBondsInSafe(uint256,int256,uint256,uint256,uint256,uint256,uint256)` and selector `0xbc50ebe6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateMaxSellBondsInSafe",
        abi = "calculateMaxSellBondsInSafe(uint256,int256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateMaxSellBondsInSafeCall {
        pub z: ::ethers::core::types::U256,
        pub zeta: ::ethers::core::types::I256,
        pub y: ::ethers::core::types::U256,
        pub z_min: ::ethers::core::types::U256,
        pub t: ::ethers::core::types::U256,
        pub c: ::ethers::core::types::U256,
        pub mu: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateSharesInGivenBondsOutDown` function with signature `calculateSharesInGivenBondsOutDown(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xa4a6f9d9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateSharesInGivenBondsOutDown",
        abi = "calculateSharesInGivenBondsOutDown(uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateSharesInGivenBondsOutDownCall {
        pub ze: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub dy: ::ethers::core::types::U256,
        pub t: ::ethers::core::types::U256,
        pub c: ::ethers::core::types::U256,
        pub mu: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateSharesInGivenBondsOutUp` function with signature `calculateSharesInGivenBondsOutUp(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x2e74108c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateSharesInGivenBondsOutUp",
        abi = "calculateSharesInGivenBondsOutUp(uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateSharesInGivenBondsOutUpCall {
        pub ze: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub dy: ::ethers::core::types::U256,
        pub t: ::ethers::core::types::U256,
        pub c: ::ethers::core::types::U256,
        pub mu: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateSharesOutGivenBondsInDown` function with signature `calculateSharesOutGivenBondsInDown(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x27d0e265`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateSharesOutGivenBondsInDown",
        abi = "calculateSharesOutGivenBondsInDown(uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateSharesOutGivenBondsInDownCall {
        pub ze: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub dy: ::ethers::core::types::U256,
        pub t: ::ethers::core::types::U256,
        pub c: ::ethers::core::types::U256,
        pub mu: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateSharesOutGivenBondsInDownSafe` function with signature `calculateSharesOutGivenBondsInDownSafe(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x3ee4114a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateSharesOutGivenBondsInDownSafe",
        abi = "calculateSharesOutGivenBondsInDownSafe(uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateSharesOutGivenBondsInDownSafeCall {
        pub ze: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub dy: ::ethers::core::types::U256,
        pub t: ::ethers::core::types::U256,
        pub c: ::ethers::core::types::U256,
        pub mu: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `kDown` function with signature `kDown(uint256,uint256,uint256,uint256,uint256)` and selector `0xd79d0835`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "kDown", abi = "kDown(uint256,uint256,uint256,uint256,uint256)")]
    pub struct KdownCall {
        pub ze: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub t: ::ethers::core::types::U256,
        pub c: ::ethers::core::types::U256,
        pub mu: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `kUp` function with signature `kUp(uint256,uint256,uint256,uint256,uint256)` and selector `0x0aea7563`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "kUp", abi = "kUp(uint256,uint256,uint256,uint256,uint256)")]
    pub struct KupCall {
        pub ze: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub t: ::ethers::core::types::U256,
        pub c: ::ethers::core::types::U256,
        pub mu: ::ethers::core::types::U256,
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
    pub enum MockYieldSpaceMathCalls {
        CalculateBondsOutGivenSharesInDown(CalculateBondsOutGivenSharesInDownCall),
        CalculateMaxBuyBondsOutSafe(CalculateMaxBuyBondsOutSafeCall),
        CalculateMaxBuySharesInSafe(CalculateMaxBuySharesInSafeCall),
        CalculateMaxSellBondsInSafe(CalculateMaxSellBondsInSafeCall),
        CalculateSharesInGivenBondsOutDown(CalculateSharesInGivenBondsOutDownCall),
        CalculateSharesInGivenBondsOutUp(CalculateSharesInGivenBondsOutUpCall),
        CalculateSharesOutGivenBondsInDown(CalculateSharesOutGivenBondsInDownCall),
        CalculateSharesOutGivenBondsInDownSafe(
            CalculateSharesOutGivenBondsInDownSafeCall,
        ),
        Kdown(KdownCall),
        Kup(KupCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockYieldSpaceMathCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CalculateBondsOutGivenSharesInDownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateBondsOutGivenSharesInDown(decoded));
            }
            if let Ok(decoded) = <CalculateMaxBuyBondsOutSafeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateMaxBuyBondsOutSafe(decoded));
            }
            if let Ok(decoded) = <CalculateMaxBuySharesInSafeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateMaxBuySharesInSafe(decoded));
            }
            if let Ok(decoded) = <CalculateMaxSellBondsInSafeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateMaxSellBondsInSafe(decoded));
            }
            if let Ok(decoded) = <CalculateSharesInGivenBondsOutDownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateSharesInGivenBondsOutDown(decoded));
            }
            if let Ok(decoded) = <CalculateSharesInGivenBondsOutUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateSharesInGivenBondsOutUp(decoded));
            }
            if let Ok(decoded) = <CalculateSharesOutGivenBondsInDownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateSharesOutGivenBondsInDown(decoded));
            }
            if let Ok(decoded) = <CalculateSharesOutGivenBondsInDownSafeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateSharesOutGivenBondsInDownSafe(decoded));
            }
            if let Ok(decoded) = <KdownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Kdown(decoded));
            }
            if let Ok(decoded) = <KupCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Kup(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockYieldSpaceMathCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CalculateBondsOutGivenSharesInDown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateMaxBuyBondsOutSafe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateMaxBuySharesInSafe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateMaxSellBondsInSafe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateSharesInGivenBondsOutDown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateSharesInGivenBondsOutUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateSharesOutGivenBondsInDown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateSharesOutGivenBondsInDownSafe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Kdown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Kup(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockYieldSpaceMathCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CalculateBondsOutGivenSharesInDown(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateMaxBuyBondsOutSafe(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateMaxBuySharesInSafe(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateMaxSellBondsInSafe(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateSharesInGivenBondsOutDown(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateSharesInGivenBondsOutUp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateSharesOutGivenBondsInDown(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateSharesOutGivenBondsInDownSafe(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Kdown(element) => ::core::fmt::Display::fmt(element, f),
                Self::Kup(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CalculateBondsOutGivenSharesInDownCall>
    for MockYieldSpaceMathCalls {
        fn from(value: CalculateBondsOutGivenSharesInDownCall) -> Self {
            Self::CalculateBondsOutGivenSharesInDown(value)
        }
    }
    impl ::core::convert::From<CalculateMaxBuyBondsOutSafeCall>
    for MockYieldSpaceMathCalls {
        fn from(value: CalculateMaxBuyBondsOutSafeCall) -> Self {
            Self::CalculateMaxBuyBondsOutSafe(value)
        }
    }
    impl ::core::convert::From<CalculateMaxBuySharesInSafeCall>
    for MockYieldSpaceMathCalls {
        fn from(value: CalculateMaxBuySharesInSafeCall) -> Self {
            Self::CalculateMaxBuySharesInSafe(value)
        }
    }
    impl ::core::convert::From<CalculateMaxSellBondsInSafeCall>
    for MockYieldSpaceMathCalls {
        fn from(value: CalculateMaxSellBondsInSafeCall) -> Self {
            Self::CalculateMaxSellBondsInSafe(value)
        }
    }
    impl ::core::convert::From<CalculateSharesInGivenBondsOutDownCall>
    for MockYieldSpaceMathCalls {
        fn from(value: CalculateSharesInGivenBondsOutDownCall) -> Self {
            Self::CalculateSharesInGivenBondsOutDown(value)
        }
    }
    impl ::core::convert::From<CalculateSharesInGivenBondsOutUpCall>
    for MockYieldSpaceMathCalls {
        fn from(value: CalculateSharesInGivenBondsOutUpCall) -> Self {
            Self::CalculateSharesInGivenBondsOutUp(value)
        }
    }
    impl ::core::convert::From<CalculateSharesOutGivenBondsInDownCall>
    for MockYieldSpaceMathCalls {
        fn from(value: CalculateSharesOutGivenBondsInDownCall) -> Self {
            Self::CalculateSharesOutGivenBondsInDown(value)
        }
    }
    impl ::core::convert::From<CalculateSharesOutGivenBondsInDownSafeCall>
    for MockYieldSpaceMathCalls {
        fn from(value: CalculateSharesOutGivenBondsInDownSafeCall) -> Self {
            Self::CalculateSharesOutGivenBondsInDownSafe(value)
        }
    }
    impl ::core::convert::From<KdownCall> for MockYieldSpaceMathCalls {
        fn from(value: KdownCall) -> Self {
            Self::Kdown(value)
        }
    }
    impl ::core::convert::From<KupCall> for MockYieldSpaceMathCalls {
        fn from(value: KupCall) -> Self {
            Self::Kup(value)
        }
    }
    ///Container type for all return fields from the `calculateBondsOutGivenSharesInDown` function with signature `calculateBondsOutGivenSharesInDown(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x86c85e10`
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
    pub struct CalculateBondsOutGivenSharesInDownReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateMaxBuyBondsOutSafe` function with signature `calculateMaxBuyBondsOutSafe(uint256,uint256,uint256,uint256,uint256)` and selector `0xa9afa373`
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
    pub struct CalculateMaxBuyBondsOutSafeReturn(
        pub ::ethers::core::types::U256,
        pub bool,
    );
    ///Container type for all return fields from the `calculateMaxBuySharesInSafe` function with signature `calculateMaxBuySharesInSafe(uint256,uint256,uint256,uint256,uint256)` and selector `0x5880b9fd`
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
    pub struct CalculateMaxBuySharesInSafeReturn(
        pub ::ethers::core::types::U256,
        pub bool,
    );
    ///Container type for all return fields from the `calculateMaxSellBondsInSafe` function with signature `calculateMaxSellBondsInSafe(uint256,int256,uint256,uint256,uint256,uint256,uint256)` and selector `0xbc50ebe6`
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
    pub struct CalculateMaxSellBondsInSafeReturn(
        pub ::ethers::core::types::U256,
        pub bool,
    );
    ///Container type for all return fields from the `calculateSharesInGivenBondsOutDown` function with signature `calculateSharesInGivenBondsOutDown(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xa4a6f9d9`
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
    pub struct CalculateSharesInGivenBondsOutDownReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateSharesInGivenBondsOutUp` function with signature `calculateSharesInGivenBondsOutUp(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x2e74108c`
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
    pub struct CalculateSharesInGivenBondsOutUpReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateSharesOutGivenBondsInDown` function with signature `calculateSharesOutGivenBondsInDown(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x27d0e265`
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
    pub struct CalculateSharesOutGivenBondsInDownReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateSharesOutGivenBondsInDownSafe` function with signature `calculateSharesOutGivenBondsInDownSafe(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x3ee4114a`
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
    pub struct CalculateSharesOutGivenBondsInDownSafeReturn(
        pub ::ethers::core::types::U256,
        pub bool,
    );
    ///Container type for all return fields from the `kDown` function with signature `kDown(uint256,uint256,uint256,uint256,uint256)` and selector `0xd79d0835`
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
    pub struct KdownReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `kUp` function with signature `kUp(uint256,uint256,uint256,uint256,uint256)` and selector `0x0aea7563`
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
    pub struct KupReturn(pub ::ethers::core::types::U256);
}
