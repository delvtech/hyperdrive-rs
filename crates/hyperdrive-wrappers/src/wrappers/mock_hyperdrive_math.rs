pub use mock_hyperdrive_math::*;
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
pub mod mock_hyperdrive_math {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("calculateAbsoluteMaxLong"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateAbsoluteMaxLong",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
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
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct HyperdriveUtils.MaxTradeParams",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_effectiveShareReserves",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_spotPrice"),
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
                    ::std::borrow::ToOwned::to_owned("calculateCloseLong"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateCloseLong"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_effectiveShareReserves",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bondReserves"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_normalizedTimeRemaining",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_timeStretch"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_initialVaultSharePrice",
                                    ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("calculateCloseShort"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateCloseShort",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_effectiveShareReserves",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bondReserves"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_normalizedTimeRemaining",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_timeStretch"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_initialVaultSharePrice",
                                    ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("calculateEffectiveShareReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateEffectiveShareReserves",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_shareReserves"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_shareAdjustment"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                    ::std::borrow::ToOwned::to_owned("calculateMaxLong"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateMaxLong"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
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
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct HyperdriveUtils.MaxTradeParams",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_checkpointExposure",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxIterations"),
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
                    ::std::borrow::ToOwned::to_owned("calculateMaxShort"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateMaxShort"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
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
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct HyperdriveUtils.MaxTradeParams",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_checkpointExposure",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxIterations"),
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
                    ::std::borrow::ToOwned::to_owned("calculateNegativeInterestOnClose"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateNegativeInterestOnClose",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_shareProceeds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_shareReservesDelta",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_shareCurveDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_totalGovernanceFee",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_openVaultSharePrice",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_closeVaultSharePrice",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_isLong"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("calculateOpenLong"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateOpenLong"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_effectiveShareReserves",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bondReserves"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_timeStretch"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_initialVaultSharePrice",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("calculateOpenShort"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateOpenShort"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_effectiveShareReserves",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bondReserves"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_timeStretch"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_initialVaultSharePrice",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("calculateShortProceedsDown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateShortProceedsDown",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bondAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_shareAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_openVaultSharePrice",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_closeVaultSharePrice",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_flatFee"),
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
                    ::std::borrow::ToOwned::to_owned("calculateShortProceedsUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateShortProceedsUp",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bondAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_shareAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_openVaultSharePrice",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_closeVaultSharePrice",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_vaultSharePrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_flatFee"),
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
                    ::std::borrow::ToOwned::to_owned("calculateSpotAPR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateSpotAPR"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_effectiveShareReserves",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bondReserves"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_initialVaultSharePrice",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_positionDuration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_timeStretch"),
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
                    ::std::borrow::ToOwned::to_owned("calculateSpotPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateSpotPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_shareReserves"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bondReserves"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_initialVaultSharePrice",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_timeStretch"),
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
                    ::std::borrow::ToOwned::to_owned("calculateTimeStretch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateTimeStretch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("apr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionDuration"),
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
    pub static MOCKHYPERDRIVEMATH_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa#\xAC\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\x94\x16\x9DI\x11a\0\x8CW\x80c\xBC\x1E\x10e\x11a\0fW\x80c\xBC\x1E\x10e\x14a\x02\x18W\x80c\xDB\xAB\x85\x12\x14a\x02+W\x80c\xE7\xAC\xD3\xAD\x14a\x02>W\x80c\xF8xE\xF2\x14a\x02QW`\0\x80\xFD[\x80c\x94\x16\x9DI\x14a\x01\xC4W\x80c\x9EH\x9B\x99\x14a\x01\xF2W\x80c\xA2\x80\xA2\x82\x14a\x02\x05W`\0\x80\xFD[\x80c^m\x9D6\x11a\0\xC8W\x80c^m\x9D6\x14a\x01PW\x80cl\xEE\xBE\x1D\x14a\x01cW\x80c~\xF9\x9F\x87\x14a\x01vW\x80c\x90+\x10\x99\x14a\x01\xB1W`\0\x80\xFD[\x80c$q\xCA\xED\x14a\0\xEFW\x80c(\x10\xA06\x14a\x01\x15W\x80c?w\xB6\x17\x14a\x01(W[`\0\x80\xFD[a\x01\x02a\0\xFD6`\x04a \x03V[a\x02dV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x02a\x01#6`\x04a 5V[a\x02}V[a\x01;a\x0166`\x04a!EV[a\x02\x98V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x0CV[a\x01\x02a\x01^6`\x04a!|V[a\x02\xB9V[a\x01\x02a\x01q6`\x04a!|V[a\x02\xD6V[a\x01\x89a\x01\x846`\x04a!\xBFV[a\x02\xE7V[`@\x80Q\x95\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01\x0CV[a\x01\x02a\x01\xBF6`\x04a!|V[a\x03kV[a\x01\xD7a\x01\xD26`\x04a\"$V[a\x03|V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\x0CV[a\x01\x02a\x02\x006`\x04a!EV[a\x03\xACV[a\x01;a\x02\x136`\x04a!EV[a\x03\xC3V[a\x01\x02a\x02&6`\x04a\"pV[a\x03\xD4V[a\x01\x02a\x0296`\x04a\"pV[a\x03\xEBV[a\x01\x02a\x02L6`\x04a!|V[a\x03\xF8V[a\x01\xD7a\x02_6`\x04a\"$V[a\x04\tV[`\0\x80a\x02s\x86\x86\x86\x86a\x04!V[\x96\x95PPPPPPV[`\0\x80a\x02\x8D\x87\x87\x87\x87\x87a\x04AV[\x97\x96PPPPPPPV[`\0\x80`\0\x80a\x02\xA9\x87\x87\x87a\x04\\V[\x90\x94P\x92PPP[\x93P\x93\x91PPV[`\0\x80a\x02\xCA\x88\x88\x88\x88\x88\x88a\x06pV[\x98\x97PPPPPPPPV[`\0\x80a\x02\xCA\x88\x88\x88\x88\x88\x88a\x06\x91V[`\0\x80`\0\x80`\0a\x03!`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x030\x8D\x8D\x8D\x8D\x8D\x8D\x8Da\x06\xB2V[`\x80\x86\x01\x81\x90R``\x86\x01\x82\x90R`@\x86\x01\x83\x90R` \x86\x01\x84\x90R\x94\x84\x90R\x92\x98P\x90\x96P\x94P\x92P\x90P\x97P\x97P\x97P\x97P\x97\x92PPPV[`\0\x80a\x02\xCA\x88\x88\x88\x88\x88\x88a\x07QV[`\0\x80`\0\x80`\0\x80a\x03\x94\x8D\x8D\x8D\x8D\x8D\x8D\x8Da\x07\x9BV[\x91\x97P\x95P\x93PPPP[\x97P\x97P\x97\x94PPPPPV[`\0a\x03\xB9\x84\x84\x84a\x08\nV[\x90P[\x93\x92PPPV[`\0\x80`\0\x80a\x02\xA9\x87\x87\x87a\t\xBCV[`\0\x80a\x03\xE1\x84\x84a\x0C\x18V[\x91PP[\x92\x91PPV[`\0\x80a\x03\xE1\x84\x84a\x0C>V[`\0\x80a\x02\xCA\x88\x88\x88\x88\x88\x88a\x0C\xD5V[`\0\x80`\0\x80`\0\x80a\x03\x94\x8D\x8D\x8D\x8D\x8D\x8D\x8Da\x0C\xF4V[`\0a\x048\x82a\x042\x85\x88\x88a\rHV[\x90a\rfV[\x95\x94PPPPPV[`\0\x80a\x04P\x87\x87\x87\x86a\x04!V[\x90Pa\x02\x8D\x81\x85a\r\xCCV[`\0\x80`\0a\x04s\x86`\0\x01Q\x87` \x01Qa\x0C\x18V[\x90P`\0a\x04\x8F\x82\x88`@\x01Q\x89`\xE0\x01Q\x8A`\xA0\x01Qa\x04!V[\x90P`\0\x80a\x04\x9F\x89\x85\x85a\t\xBCV[\x90\x92P\x90P`\0a\x04\xB3\x8A\x8A\x85\x85\x88a\r\xF2V[\x91PP\x80\x15a\x04\xCAWP\x90\x94P\x92Pa\x02\xB1\x91PPV[Pa\x04\xD7\x89\x83\x8A\x86a\x0F*V[\x95Pa\x04\xE5\x89\x87\x86\x86a\x0F\xCAV[\x94P`\0\x80a\x04\xF7\x8B\x8B\x8A\x8A\x89a\r\xF2V[\x91P\x91P\x80a\x05gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FInitial guess in `calculateMaxLo`D\x82\x01Rp73\xB0\x104\xB9\x904\xB79\xB7\xB6;2\xB7:\x17`y\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x89\x81\x10\x15a\x06aW\x84\x89\x10a\x05\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FReached absolute max bond amount`D\x82\x01Rr\x104\xB7\x1003\xB2\xBA/\xB6\xB0\xBC/\xB67\xB73\xB0\x17`i\x1B`d\x82\x01R`\x84\x01a\x05^V[`\0a\x05\xEB\x8D\x8B\x8A\x8Aa\x10\x1DV[\x93P\x90P\x82a\x05\xFAWPa\x06aV[`\0a\x06\x06\x85\x83a\x10\xB6V[a\x06\x10\x90\x8Ca\"\xA8V[\x90P`\0a\x06 \x8F\x83\x8C\x8Ca\x0F\xCAV[\x90Pa\x06/\x8F\x8F\x84\x84\x8Da\r\xF2V[\x90\x96P\x94P\x84\x15a\x06EW\x81\x9BP\x80\x9APa\x06MV[PPPa\x06aV[PPP\x80a\x06Z\x90a\"\xBBV[\x90Pa\x05jV[PPPPPPP\x93P\x93\x91PPV[`\0a\x02\x8D\x87\x87\x87a\x06\x8A\x88g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x87\x87a\x10\xCBV[`\0a\x02\x8D\x87\x87\x87a\x06\xAB\x88g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x87\x87a\x10\xEEV[`\0\x80`\0\x80`\0\x80\x88\x88\x10\x15a\x07!W\x86\x15a\x06\xD7Wa\x06\xD4\x8D\x89\x8Ba\rHV[\x9CP[a\x06\xE2\x8C\x89\x8Ba\rHV[\x9BPa\x06\xED\x8Ba\x10\xFFV[a\x06\xF6\x8Da\x10\xFFV[a\x07\0\x91\x90a\"\xE7V[\x90Pa\x07\r\x8B\x89\x8Ba\rHV[\x9APa\x07\x1A\x8A\x89\x8Ba\rHV[\x99Pa\x07@V[a\x07*\x8Ba\x10\xFFV[a\x073\x8Da\x10\xFFV[a\x07=\x91\x90a\"\xE7V[\x90P[\x9B\x9C\x9A\x9B\x99\x9A\x97PPPPPPPPV[`\0\x80a\x07i\x84a\x07c\x8A\x88\x8Aa\x11-V[\x90a\x11SV[\x90Pa\x07v\x88\x84\x86a\x11-V[a\x07\x80\x90\x82a\"\xA8V[\x90P\x86\x81\x11\x15a\x07\x90W\x86\x81\x03\x91P[P\x96\x95PPPPPPV[`\0\x80\x80a\x07\xBCa\x07\xB4\x88g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x89\x90\x87a\rHV[\x90P\x86\x15a\x03\x9FWa\x07\xCE\x88\x88a\x11hV[\x91Pa\x07\xEF\x8A\x8A\x84a\x07\xE8\x8Ag\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x89\x89a\x10\xEEV[\x92Pa\x07\xFB\x83\x82a\"\xA8V[\x90P\x97P\x97P\x97\x94PPPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R`\0a\x08:\x86`\0\x01Q\x87` \x01Qa\x0C\x18V[\x90P`\0a\x08V\x82\x88`@\x01Q\x89`\xE0\x01Q\x8A`\xA0\x01Qa\x04!V[\x90P`\0a\x08d\x88\x84a\x11}V[\x90Pa\x08s\x88\x82\x85\x85\x8Ba\x12\xA2V[\x15\x80\x15`@\x87\x01R\x90\x85Ra\x08\x8DW\x93Pa\x03\xBC\x92PPPV[`\0a\x08\x9A\x89\x84\x8Aa\x14\x02V[\x90Pa\x08\xA9\x89\x82\x86\x86\x8Ca\x12\xA2V[\x15\x15`@\x87\x01\x81\x90R\x90\x86Ra\t\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FInitial guess in `calculateMaxSh`D\x82\x01Rp\x1B\xDC\x9D\x18\x08\x1A\\\xC8\x1A[\x9C\xDB\xDB\x1D\x99[\x9D`z\x1B`d\x82\x01R`\x84\x01a\x05^V[`\0[\x87\x81\x10\x15a\t\xAFWa\t2\x8A\x83\x86\x88a\x14\xD8V[\x15\x80\x15`@\x89\x01R` \x88\x01\x91\x90\x91Ra\t\xAFW` \x86\x01Q\x86Q`\0\x91a\tZ\x91\x90a\x10\xB6V[a\td\x90\x84a\"\xA8V[\x90P\x83\x81\x11\x15a\ttWPa\t\xAFV[a\t\x81\x8B\x82\x88\x88\x8Ea\x12\xA2V[\x15\x80\x15`@\x8A\x01R\x90\x88Ra\t\x98W\x80\x92Pa\t\x9EV[Pa\t\xAFV[Pa\t\xA8\x81a\"\xBBV[\x90Pa\t\x1EV[P\x98\x97PPPPPPPPV[`\0\x80`\0\x80a\t\xF2\x86\x88`@\x01Q\x89`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\t\xE3\x91\x90a\"\xD4V[\x8A`\xC0\x01Q\x8B`\xE0\x01Qa\x15_V[\x90Pa\nB\x87a\x01@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\n\x10\x91\x90a\"\xD4V[a\n<g\r\xE0\xB6\xB3\xA7d\0\0a\n&\x81\x8Aa\x11SV[a\n0\x91\x90a\"\xD4V[a\x01 \x8B\x01Q\x90a\x15\x8EV[\x90a\x15\x8EV[\x91Pa\nr\x87a\x01@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\n`\x91\x90a\"\xD4V[a\x07c\x84g\r\xE0\xB6\xB3\xA7d\0\0a\"\xA8V[`\xA0\x88\x01Q\x90\x92Pa\n\xA1\x90a\n\x9A\x90a\n\x94\x81g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x90a\x10\xB6V[\x83\x90a\rfV[\x91Pa\n\xBE\x87`\xE0\x01Q\x88`\xC0\x01Qa\x11S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xC8\x90\x83a\"\xA8V[\x91Pa\n\xD4\x81\x83a\x10\xB6V[\x91Pa\x0B\x03a\n\x9A\x88`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\n\xF4\x91\x90a\"\xD4V[g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x10\xB6V[\x91PP`\0a\x0B\x1F\x87`\xE0\x01Q\x83a\x10\xB6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x80a\x0Br\x89a\x01@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0B@\x91\x90a\"\xD4V[a\x0Blg\r\xE0\xB6\xB3\xA7d\0\0a\x0BV\x81\x8Ca\x10\xB6V[a\x0B`\x91\x90a\"\xD4V[a\x01 \x8D\x01Q\x90a\x11hV[\x90a\x11hV[\x90Pa\x0B\xC8\x84a\x0Bla\x0B\x9A\x8C`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x11S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x042\x8Da\x01@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0B\xB6\x91\x90a\"\xD4V[a\n\x94\x87g\r\xE0\xB6\xB3\xA7d\0\0a\"\xA8V[\x91PPa\x0B\xDF\x88`\xC0\x01Q\x88\x84a\x0Bl\x91\x90a\"\xD4V[\x94Pa\x0B\xF1\x85\x87\x8Aa\x01 \x01Qa\x15\xA3V[\x81\x89`@\x01Qa\x0C\x01\x91\x90a\"\xD4V[a\x0C\x0B\x91\x90a\"\xD4V[\x93PPPP\x93P\x93\x91PPV[`\0\x80a\x0C%\x84\x84a\x15\xD0V[\x90\x92P\x90P\x80a\x0C7Wa\x0C7a\x16\x0FV[P\x92\x91PPV[`\0\x80a\x0Coa\x0C`a\x0CR\x86`da#\x07V[f\xA5\xBB\xED\x86\xC5\xA0\0\x90a\x11hV[gH\xCD@r(\x1E\0\0\x90a\x10\xB6V[\x90Pa\x0C\x83g\r\xE0\xB6\xB3\xA7d\0\0\x82a\x10\xB6V[\x90Pa\x03\xE1\x81a\x0Bla\x0C\xAEa\x0C\xA9a\x0C\xA4\x89g\r\xE0\xB6\xB3\xA7d\0\0a\"\xA8V[a\x10\xFFV[a\x16(V[a\n\x94a\x0C\xA9a\x0C\xC3\x8A\x8Ac\x01\xE13\x80a\rHV[a\x0C\xA4\x90g\r\xE0\xB6\xB3\xA7d\0\0a\"\xA8V[`\0\x80a\x0C\xE7\x84a\n\x94\x8A\x88\x8Aa\rHV[\x90Pa\x07v\x88\x84\x86a\rHV[`\0\x80\x80a\r\x15a\r\r\x88g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x89\x90\x87a\x11-V[\x90P\x86\x15a\x03\x9FWa\r'\x88\x88a\x15\x8EV[\x91Pa\x07\xEF\x8A\x8A\x84a\rA\x8Ag\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x89\x89a\x18WV[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\r_W`\0\x80\xFD[P\x91\x02\x04\x90V[`\0\x81`\0\x03a\r\x7FWPg\r\xE0\xB6\xB3\xA7d\0\0a\x03\xE5V[\x82`\0\x03a\r\x8FWP`\0a\x03\xE5V[`\0a\r\x9A\x83a\x10\xFFV[\x90P`\0a\r\xAAa\x0C\xA9\x86a\x10\xFFV[\x90P\x81\x81\x02a\r\xC1g\r\xE0\xB6\xB3\xA7d\0\0\x82a#\x1EV[\x90Pa\x02s\x81a\x18hV[`\0a\x03\xBCa\r\xE0\x84\x84c\x01\xE13\x80a\x11-V[a\n\x94\x85g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[`\0\x80`\0a\x0E\r\x86\x85\x8Aa\x01 \x01Q\x8Ba\x01`\x01Qa\x19\xFDV[\x90P`\0a\x0E(\x89`\xC0\x01Q\x83a\x10\xB6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xC0\x8A\x01Qa\x0E8\x90\x89\x90a\x10\xB6V[\x8AQa\x0ED\x91\x90a\"\xA8V[a\x0EN\x91\x90a\"\xD4V[\x90P`\0\x86\x8A`\x80\x01Qa\x0Eb\x91\x90a\"\xA8V[\x90P`\0a\x0Ep\x8A\x82a\x1A\x12V[a\x0Ey\x90a#ZV[\x90P\x8Aa\x01\0\x01Qa\x0E\x98\x8C`\xC0\x01Q\x84a\x10\xB6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0E\xA2\x91\x90a\"\xA8V[`\xC0\x8C\x01Qa\x0E\xB2\x90\x83\x90a\x10\xB6V[a\x0E\xBC\x90\x85a\"\xA8V[\x10a\x0F\x14Wa\x01\0\x8B\x01Q`\xC0\x8C\x01Qa\x0E\xD7\x90\x84\x90a\x10\xB6V[`\xC0\x8D\x01Qa\x0E\xE7\x90\x84\x90a\x10\xB6V[a\x0E\xF1\x90\x86a\"\xA8V[a\x0E\xFB\x91\x90a\"\xD4V[a\x0F\x05\x91\x90a\"\xD4V[`\x01\x95P\x95PPPPPa\x0F V[`\0\x80\x95P\x95PPPPP[\x95P\x95\x93PPPPV[`\0\x80a\x0F9\x86\x85\x85\x86a\x1A'V[\x90P`\0a\x0F\x80g\x0B\x1A+\xC2\xECP\0\0a\x0Bla\x0Fv\x8A`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0Fg\x91\x90a\"\xD4V[g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x11SV[a\x042\x86\x8Ba\x10\xB6V[\x90P`\0a\x0F\x96g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x11hV[a\x0F\xB2a\x0F\xAB\x84g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x87\x90a\x11hV[a\x0F\xBC\x91\x90a\"\xA8V[\x90Pa\x02\xCA\x88\x87\x87\x84a\x1A'V[`\0\x80a\x10\x01\x84\x87`@\x01Qa\x0F\xED\x89`\xC0\x01Q\x89a\x10\xB6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89`\xA0\x01Q\x8A`\xC0\x01Q\x8B`\xE0\x01Qa\x06pV[\x90Pa\x10\x13\x85\x84\x88a\x01 \x01Qa\x15\xA3V[a\x02s\x90\x82a\"\xD4V[`\0\x80a\x10,\x86\x86\x86\x86a\x1B*V[\x90\x92P\x90P\x80a\x10?W`\0\x91Pa\x10\xADV[a\x10ia\x10T\x84g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[a\x01 \x88\x01Qa\x01`\x89\x01Qa\x0Bl\x91a\x11hV[a\x10s\x90\x83a\"\xA8V[\x91Pa\x10\x87g\r\xE0\xB6\xB3\xA7d\0\0\x83a\"\xD4V[\x91Pa\x10\xAAg\r\xE0\xB6\xB3\xA7d\0\0\x87`\xC0\x01Q\x84a\rH\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91P[\x94P\x94\x92PPPV[`\0a\x03\xBC\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\rHV[`\0\x80a\x10\xDC\x88\x88\x88\x88\x88\x88a\x1CzV[\x90\x92P\x90P\x80a\x07\x90Wa\x07\x90a\x16\x0FV[`\0\x80a\x10\xDC\x88\x88\x88\x88\x88\x88a\x1D4V[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x11)W`@Qc9n\xA7\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x11DW`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`\0a\x03\xBC\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x11-V[`\0a\x03\xBC\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\rHV[`\0\x80a\x11\x98`\0\x85` \x01Qa\x1D\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84a\x01\0\x01Qa\x11\xA8\x91\x90a\"\xA8V[\x90P`\0a\x11\xDC\x84\x86`@\x01Q\x87`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x11\xCD\x91\x90a\"\xD4V[\x88`\xC0\x01Q\x89`\xE0\x01Qa\x15_V[\x90P`\0a\x12+a\x12\x1A\x87`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x11\xFE\x91\x90a\"\xD4V[a\x042a\x12\x0F\x87\x8B` \x01Qa\x0C\x18V[`\xE0\x8B\x01Q\x90a\x15\x8EV[`\xE0\x88\x01Q`\xC0\x89\x01Q\x91\x90a\x11-V[a\x125\x90\x83a\"\xD4V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x12rWa\x12ka\x12d\x87`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\n\xF4\x91\x90a\"\xD4V[\x82\x90a\rfV[\x90Pa\x12\x93V[a\x12\x90a\x12d\x87`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0Fg\x91\x90a\"\xD4V[\x90P[`@\x86\x01Qa\x02s\x90\x82a\"\xD4V[`\0\x80`\0\x80a\x12\xD9\x87\x8A`@\x01Q\x8A\x8C`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x12\xCA\x91\x90a\"\xD4V[\x8D`\xC0\x01Q\x8E`\xE0\x01Qa\x1D4V[\x91P\x91P\x80a\x12\xF0W`\0\x80\x93P\x93PPPa\x0F V[`\0a\x13*\x8A`\xC0\x01Qa\x13\x10\x8B\x8A\x8Ea\x01 \x01Q\x8Fa\x01`\x01Qa\x1E\nV[a\x13 \x8C\x8B\x8Fa\x01 \x01Qa\x1E\x17V[a\x07c\x91\x90a\"\xD4V[\x90P\x80\x83\x10\x15a\x13CW`\0\x80\x94P\x94PPPPa\x0F V[`\0a\x13O\x82\x85a\"\xD4V[\x90P\x80\x8B`\0\x01Q\x10\x15a\x13mW`\0\x80\x95P\x95PPPPPa\x0F V[\x8AQ`\0\x90a\x13}\x90\x83\x90a\"\xD4V[\x90P`\0a\x13\xAC\x8D`\xC0\x01Qa\x13\x9D`\0\x8Ca\x1D\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x8F`\x80\x01Qa\n\x94\x91\x90a\"\xD4V[\x90P\x8Ca\x01\0\x01Q\x81a\x13\xBF\x91\x90a\"\xA8V[\x82\x10a\x13\xF0Wa\x01\0\x8D\x01Qa\x13\xD5\x82\x84a\"\xD4V[a\x13\xDF\x91\x90a\"\xD4V[`\x01\x97P\x97PPPPPPPa\x0F V[`\0\x80\x97P\x97PPPPPPPa\x0F V[`\0\x80\x83\x90P`\0a\x14m\x86a\x01\0\x01Qa\x14.\x88`\xC0\x01Q\x89`\x80\x01Qa\x10\xB6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xC0\x89\x01Qa\x14B\x90a\n\x94\x89`\0a\x1D\xF4V[\x89Qa\x14N\x91\x90a\"\xA8V[a\x14X\x91\x90a\"\xD4V[a\x14b\x91\x90a\"\xD4V[`\xC0\x88\x01Q\x90a\x11hV[\x90Pa\x02sa\x14\x9Ca\x14\x87\x87g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[a\x01 \x89\x01Qa\x01`\x8A\x01Qa\x0Bl\x91a\x11hV[a\x14\xBDa\x14\xB1\x88g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[a\x01 \x8A\x01Q\x90a\x11hV[a\x14\xC7\x90\x85a\"\xD4V[a\x14\xD1\x91\x90a\"\xA8V[\x82\x90a\x10\xB6V[`\0\x80`\0a\x14\xE8\x87\x87\x86a\x1E2V[\x90P`\0a\x151\x88`\xC0\x01Qa\n\x94\x8Aa\x01`\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x15\x10\x91\x90a\"\xD4V[a\x0Bla\x15%\x8Bg\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[a\x01 \x8E\x01Q\x90a\x11hV[\x90P\x80\x82\x10a\x15QWa\x15D\x81\x83a\"\xD4V[`\x01\x93P\x93PPPa\x10\xADV[`\0\x80\x93P\x93PPPa\x10\xADV[`\0a\x15k\x85\x85a\rfV[a\x15\x84a\x15|\x86a\x042\x86\x8Ba\x11hV[\x85\x90\x85a\rHV[a\x02s\x91\x90a\"\xA8V[`\0a\x03\xBC\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11-V[`\0a\x03\xB9\x84a\n<g\r\xE0\xB6\xB3\xA7d\0\0a\x15\xBF\x81\x88a\x11SV[a\x15\xC9\x91\x90a\"\xD4V[\x85\x90a\x15\x8EV[`\0\x80`\0\x83a\x15\xDF\x86a\x10\xFFV[a\x15\xE9\x91\x90a\"\xE7V[\x90P`\0\x81\x12\x15a\x16\x01W`\0\x80\x92P\x92PPa\x16\x08V[\x91P`\x01\x90P[\x92P\x92\x90PV[`@Qc\xBBU\xFD'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x82\x13a\x16JW`@Qc\xE6\x1BIu`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x82\x81\x1C`\x0F\x10`\x02\x1B\x17\x82\x81\x1C\x90\x91\x10`\x01\x90\x81\x1B\x90\x91\x17\x82\x81\x1C\x90\x91\x10\x17`\x9F\x81\x81\x03``\x01\x92\x90\x92\x1B\x91`_\x19\x82\x01\x90a\x16\xD6\x90\x84\x90\x1Ca\x10\xFFV[lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x91\x90\x91\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x80a\x10\xDC\x88\x88\x88\x88\x88\x88a\x1F\x07V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x18\x83WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x18\xACW`@Qcs\xA2\xD6\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x92l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x84\x01\x84\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x85\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x85\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x85\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x85\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x87\x01\x87\x02\x83\x1D\x90\x81\x01\x90\x87\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x81\x02\x90\x92\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x86\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x81\x81\x05\x95P\x92\x93P\x90\x91\x90a\x02st\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x86\x02`\xC3\x86\x90\x03\x1Ca\x10\xFFV[`\0a\x048\x84a\x0Bl\x84a\x0Bl\x89\x89\x89a\x15\xA3V[`\0\x81\x83\x13a\x1A!W\x82a\x03\xBCV[P\x91\x90PV[`\0\x80a\x1A4\x85\x82a\x1A\x12V[a\x1A=\x90a#ZV[\x90P`\0a\x1A\xAA\x87`\xC0\x01Qg\x1B\xC1mgN\xC8\0\0\x89a\x01\0\x01Qa\x1As\x8B`\xC0\x01Q\x8C`\x80\x01Qa\x10\xB6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xC0\x8C\x01Qa\x1A\x83\x90\x88\x90a\x10\xB6V[\x8CQa\x1A\x8F\x91\x90a\"\xA8V[a\x1A\x99\x91\x90a\"\xD4V[a\x1A\xA3\x91\x90a\"\xD4V[\x91\x90a\rHV[\x90Pa\x02\x8Da\x1A\xCFg\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xC5\x81\x89a\x10\xB6V[a\x14\xB1\x91\x90a\"\xD4V[g\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xFAa\x1A\xE5\x89\x83a\"\xD4V[a\x01 \x8C\x01Qa\x01`\x8D\x01Qa\x0Bl\x91a\x11hV[a\x1B\x0Cg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\xB6V[a\x1B\x16\x91\x90a\"\xA8V[a\x1B \x91\x90a\"\xD4V[a\x14\xD1\x91\x90a\"\xD4V[`\0\x80`\0a\x1BF\x87`\xC0\x01Q\x87a\x10\xB6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1Bba\x1BW\x83\x88a\"\xA8V[`\xE0\x8A\x01Q\x90a\x11hV[\x90P`\0a\x1B\x96\x87\x8A`@\x01Q\x8B`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1B\x87\x91\x90a\"\xD4V[\x8C`\xC0\x01Q\x8D`\xE0\x01Qa\x15_V[\x90Pa\x1B\xB2a\n\xF4\x8A`\xA0\x01Q\x84a\rf\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94P`\0a\x1B\xE1a\x1B\xD0\x8B`\xA0\x01Q\x85a\rf\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xE0\x8C\x01Q`\xC0\x8D\x01Q\x91\x90a\rHV[\x90P\x80\x82\x10\x15a\x1B\xFBW`\0\x80\x95P\x95PPPPPa\x10\xADV[a\x1C1a\x0F\xABa\x1C'\x8C`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1C\x1C\x91\x90a\"\xD4V[`\xA0\x8E\x01Q\x90a\x11SV[a\x042\x84\x86a\"\xD4V[\x95Pa\x1C_g\r\xE0\xB6\xB3\xA7d\0\0a\x1CI\x81\x8Aa\x10\xB6V[a\x1CS\x91\x90a\"\xD4V[a\x01 \x8C\x01Q\x90a\x11hV[a\x1Ci\x90\x87a\"\xD4V[\x9A`\x01\x9AP\x98PPPPPPPPPV[`\0\x80`\0a\x1C\x8C\x89\x89\x88\x88\x88a\x1F\xDEV[\x90Pa\x1C\x9F\x86a\x042a\x0F\xAB\x8A\x8Da\"\xA8V[\x98Pa\x1C\xAC\x85\x8A\x86a\rHV[\x98P\x88\x81\x10\x15a\x1C\xC3W`\0\x80\x92P\x92PPa\x1D)V[\x88\x81\x03g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x1C\xF1Wa\x1C\xEAa\x12dg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x11SV[\x90Pa\x1D\tV[a\x1D\x06a\x12dg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\xB6V[\x90P[\x80\x89\x10\x15a\x1D\x1FW`\0\x80\x93P\x93PPPa\x1D)V[\x88\x03\x92P`\x01\x91PP[\x96P\x96\x94PPPPPV[`\0\x80`\0a\x1DF\x89\x89\x88\x88\x88a\x1F\xDEV[\x90Pa\x1DV\x86a\x042\x89\x8Ba\"\xA8V[\x97P\x87\x81\x10\x15a\x1DmW`\0\x80\x92P\x92PPa\x1D)V[\x87\x81\x03a\x1D{\x81\x86\x88a\x11-V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x1D\xA8Wa\x1D\xA1a\x12dg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x11SV[\x90Pa\x1D\xC0V[a\x1D\xBDa\x12dg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\xB6V[\x90P[a\x1D\xCA\x81\x86a\x11SV[\x90P\x80\x8A\x10\x15a\x1D\xE2W`\0\x80\x93P\x93PPPa\x1D)V[\x90\x98\x03\x98`\x01\x98P\x96PPPPPPPV[`\0\x81\x83\x13a\x1E\x03W\x81a\x03\xBCV[P\x90\x91\x90PV[`\0a\x048\x82a\x0Bl\x87\x87\x87[`\0a\x03\xB9\x84a\n<a\x15\xC9\x86g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[`\0\x80a\x1EV\x83\x86`@\x01Q\x87`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x11\xCD\x91\x90a\"\xD4V[\x90P`\0a\x1E\x83a\n\xF4a\x1Ex\x88`\xA0\x01Q\x88\x8A`@\x01Qa\x042\x91\x90a\"\xA8V[`\xC0\x89\x01Q\x90a\x15\x8EV[\x90P`\0a\x1E\xFBa\x1E\xB0\x88`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xA5\x91\x90a\"\xD4V[`\xA0\x8A\x01Q\x90a\x11SV[a\x042a\x1E\xDE\x8A`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xCE\x91\x90a\"\xD4V[\x8A\x8C`@\x01Qa\x042\x91\x90a\"\xA8V[a\x1E\xE8\x90\x87a\"\xD4V[`\xC0\x8B\x01Q`\xE0\x8C\x01Qa\x0Bl\x91a\x10\xB6V[\x90Pa\x02\x8D\x82\x82a\x11hV[`\0\x80`\0a\x1F\x19\x89\x89\x88\x88\x88a\x1F\xDEV[\x90P\x86\x88\x10\x15a\x1F0W`\0\x80\x92P\x92PPa\x1D)V[\x96\x86\x90\x03\x96a\x1F?\x88\x87a\rfV[\x97P\x87\x81\x10\x15a\x1FVW`\0\x80\x92P\x92PPa\x1D)V[\x87\x81\x03a\x1Fd\x81\x86\x88a\x11-V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x1F\x91Wa\x1F\x8Aa\x12dg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x11SV[\x90Pa\x1F\xA9V[a\x1F\xA6a\x12dg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\xB6V[\x90P[a\x1F\xB3\x81\x86a\x11SV[\x90P\x89\x81\x10\x15a\x1F\xCBW`\0\x80\x93P\x93PPPa\x1D)V[\x98\x90\x98\x03\x98`\x01\x98P\x96PPPPPPPV[`\0a\x1F\xEA\x85\x85a\rfV[a\x15\x84a\x1F\xFB\x86a\x042\x86\x8Ba\x15\x8EV[\x85\x90\x85a\x11-V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a \x19W`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a MW`\0\x80\xFD[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`@Qa\x01\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a \xA2WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0a\x01\x80\x82\x84\x03\x12\x15a \xBBW`\0\x80\xFD[a \xC3a pV[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015`\x80\x82\x01R`\xA0\x82\x015`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R`\xE0\x82\x015`\xE0\x82\x01Ra\x01\0\x80\x83\x015\x81\x83\x01RPa\x01 \x80\x83\x015\x81\x83\x01RPa\x01@\x80\x83\x015\x81\x83\x01RPa\x01`\x80\x83\x015\x81\x83\x01RP\x92\x91PPV[`\0\x80`\0a\x01\xC0\x84\x86\x03\x12\x15a![W`\0\x80\xFD[a!e\x85\x85a \xA8V[\x95a\x01\x80\x85\x015\x95Pa\x01\xA0\x90\x94\x015\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a!\x95W`\0\x80\xFD[PP\x845\x96` \x86\x015\x96P`@\x86\x015\x95``\x81\x015\x95P`\x80\x81\x015\x94P`\xA0\x015\x92P\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a!\xDAW`\0\x80\xFD[\x875\x96P` \x88\x015\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015\x80\x15\x15\x81\x14a\"\x14W`\0\x80\xFD[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\"?W`\0\x80\xFD[PP\x855\x97` \x87\x015\x97P`@\x87\x015\x96``\x81\x015\x96P`\x80\x81\x015\x95P`\xA0\x81\x015\x94P`\xC0\x015\x92P\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\"\x83W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x03\xE5Wa\x03\xE5a\"\x92V[`\0`\x01\x82\x01a\"\xCDWa\"\xCDa\"\x92V[P`\x01\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x03\xE5Wa\x03\xE5a\"\x92V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x0C7Wa\x0C7a\"\x92V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xE5Wa\x03\xE5a\"\x92V[`\0\x82a#;WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a#UWa#Ua\"\x92V[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a#oWa#oa\"\x92V[P`\0\x03\x90V\xFE\xA2dipfsX\"\x12 \xC5\xDAb :J2\xA6\x95\xEE\xAE]\xE9\xB2\x94\xE2=qy\x9B\x0F6b\xC6\x8E\xFD\x8D\x91\xA3\x9D\xD7\xA8dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static MOCKHYPERDRIVEMATH_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\x94\x16\x9DI\x11a\0\x8CW\x80c\xBC\x1E\x10e\x11a\0fW\x80c\xBC\x1E\x10e\x14a\x02\x18W\x80c\xDB\xAB\x85\x12\x14a\x02+W\x80c\xE7\xAC\xD3\xAD\x14a\x02>W\x80c\xF8xE\xF2\x14a\x02QW`\0\x80\xFD[\x80c\x94\x16\x9DI\x14a\x01\xC4W\x80c\x9EH\x9B\x99\x14a\x01\xF2W\x80c\xA2\x80\xA2\x82\x14a\x02\x05W`\0\x80\xFD[\x80c^m\x9D6\x11a\0\xC8W\x80c^m\x9D6\x14a\x01PW\x80cl\xEE\xBE\x1D\x14a\x01cW\x80c~\xF9\x9F\x87\x14a\x01vW\x80c\x90+\x10\x99\x14a\x01\xB1W`\0\x80\xFD[\x80c$q\xCA\xED\x14a\0\xEFW\x80c(\x10\xA06\x14a\x01\x15W\x80c?w\xB6\x17\x14a\x01(W[`\0\x80\xFD[a\x01\x02a\0\xFD6`\x04a \x03V[a\x02dV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x02a\x01#6`\x04a 5V[a\x02}V[a\x01;a\x0166`\x04a!EV[a\x02\x98V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x0CV[a\x01\x02a\x01^6`\x04a!|V[a\x02\xB9V[a\x01\x02a\x01q6`\x04a!|V[a\x02\xD6V[a\x01\x89a\x01\x846`\x04a!\xBFV[a\x02\xE7V[`@\x80Q\x95\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x01\x0CV[a\x01\x02a\x01\xBF6`\x04a!|V[a\x03kV[a\x01\xD7a\x01\xD26`\x04a\"$V[a\x03|V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\x0CV[a\x01\x02a\x02\x006`\x04a!EV[a\x03\xACV[a\x01;a\x02\x136`\x04a!EV[a\x03\xC3V[a\x01\x02a\x02&6`\x04a\"pV[a\x03\xD4V[a\x01\x02a\x0296`\x04a\"pV[a\x03\xEBV[a\x01\x02a\x02L6`\x04a!|V[a\x03\xF8V[a\x01\xD7a\x02_6`\x04a\"$V[a\x04\tV[`\0\x80a\x02s\x86\x86\x86\x86a\x04!V[\x96\x95PPPPPPV[`\0\x80a\x02\x8D\x87\x87\x87\x87\x87a\x04AV[\x97\x96PPPPPPPV[`\0\x80`\0\x80a\x02\xA9\x87\x87\x87a\x04\\V[\x90\x94P\x92PPP[\x93P\x93\x91PPV[`\0\x80a\x02\xCA\x88\x88\x88\x88\x88\x88a\x06pV[\x98\x97PPPPPPPPV[`\0\x80a\x02\xCA\x88\x88\x88\x88\x88\x88a\x06\x91V[`\0\x80`\0\x80`\0a\x03!`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x030\x8D\x8D\x8D\x8D\x8D\x8D\x8Da\x06\xB2V[`\x80\x86\x01\x81\x90R``\x86\x01\x82\x90R`@\x86\x01\x83\x90R` \x86\x01\x84\x90R\x94\x84\x90R\x92\x98P\x90\x96P\x94P\x92P\x90P\x97P\x97P\x97P\x97P\x97\x92PPPV[`\0\x80a\x02\xCA\x88\x88\x88\x88\x88\x88a\x07QV[`\0\x80`\0\x80`\0\x80a\x03\x94\x8D\x8D\x8D\x8D\x8D\x8D\x8Da\x07\x9BV[\x91\x97P\x95P\x93PPPP[\x97P\x97P\x97\x94PPPPPV[`\0a\x03\xB9\x84\x84\x84a\x08\nV[\x90P[\x93\x92PPPV[`\0\x80`\0\x80a\x02\xA9\x87\x87\x87a\t\xBCV[`\0\x80a\x03\xE1\x84\x84a\x0C\x18V[\x91PP[\x92\x91PPV[`\0\x80a\x03\xE1\x84\x84a\x0C>V[`\0\x80a\x02\xCA\x88\x88\x88\x88\x88\x88a\x0C\xD5V[`\0\x80`\0\x80`\0\x80a\x03\x94\x8D\x8D\x8D\x8D\x8D\x8D\x8Da\x0C\xF4V[`\0a\x048\x82a\x042\x85\x88\x88a\rHV[\x90a\rfV[\x95\x94PPPPPV[`\0\x80a\x04P\x87\x87\x87\x86a\x04!V[\x90Pa\x02\x8D\x81\x85a\r\xCCV[`\0\x80`\0a\x04s\x86`\0\x01Q\x87` \x01Qa\x0C\x18V[\x90P`\0a\x04\x8F\x82\x88`@\x01Q\x89`\xE0\x01Q\x8A`\xA0\x01Qa\x04!V[\x90P`\0\x80a\x04\x9F\x89\x85\x85a\t\xBCV[\x90\x92P\x90P`\0a\x04\xB3\x8A\x8A\x85\x85\x88a\r\xF2V[\x91PP\x80\x15a\x04\xCAWP\x90\x94P\x92Pa\x02\xB1\x91PPV[Pa\x04\xD7\x89\x83\x8A\x86a\x0F*V[\x95Pa\x04\xE5\x89\x87\x86\x86a\x0F\xCAV[\x94P`\0\x80a\x04\xF7\x8B\x8B\x8A\x8A\x89a\r\xF2V[\x91P\x91P\x80a\x05gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FInitial guess in `calculateMaxLo`D\x82\x01Rp73\xB0\x104\xB9\x904\xB79\xB7\xB6;2\xB7:\x17`y\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x89\x81\x10\x15a\x06aW\x84\x89\x10a\x05\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FReached absolute max bond amount`D\x82\x01Rr\x104\xB7\x1003\xB2\xBA/\xB6\xB0\xBC/\xB67\xB73\xB0\x17`i\x1B`d\x82\x01R`\x84\x01a\x05^V[`\0a\x05\xEB\x8D\x8B\x8A\x8Aa\x10\x1DV[\x93P\x90P\x82a\x05\xFAWPa\x06aV[`\0a\x06\x06\x85\x83a\x10\xB6V[a\x06\x10\x90\x8Ca\"\xA8V[\x90P`\0a\x06 \x8F\x83\x8C\x8Ca\x0F\xCAV[\x90Pa\x06/\x8F\x8F\x84\x84\x8Da\r\xF2V[\x90\x96P\x94P\x84\x15a\x06EW\x81\x9BP\x80\x9APa\x06MV[PPPa\x06aV[PPP\x80a\x06Z\x90a\"\xBBV[\x90Pa\x05jV[PPPPPPP\x93P\x93\x91PPV[`\0a\x02\x8D\x87\x87\x87a\x06\x8A\x88g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x87\x87a\x10\xCBV[`\0a\x02\x8D\x87\x87\x87a\x06\xAB\x88g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x87\x87a\x10\xEEV[`\0\x80`\0\x80`\0\x80\x88\x88\x10\x15a\x07!W\x86\x15a\x06\xD7Wa\x06\xD4\x8D\x89\x8Ba\rHV[\x9CP[a\x06\xE2\x8C\x89\x8Ba\rHV[\x9BPa\x06\xED\x8Ba\x10\xFFV[a\x06\xF6\x8Da\x10\xFFV[a\x07\0\x91\x90a\"\xE7V[\x90Pa\x07\r\x8B\x89\x8Ba\rHV[\x9APa\x07\x1A\x8A\x89\x8Ba\rHV[\x99Pa\x07@V[a\x07*\x8Ba\x10\xFFV[a\x073\x8Da\x10\xFFV[a\x07=\x91\x90a\"\xE7V[\x90P[\x9B\x9C\x9A\x9B\x99\x9A\x97PPPPPPPPV[`\0\x80a\x07i\x84a\x07c\x8A\x88\x8Aa\x11-V[\x90a\x11SV[\x90Pa\x07v\x88\x84\x86a\x11-V[a\x07\x80\x90\x82a\"\xA8V[\x90P\x86\x81\x11\x15a\x07\x90W\x86\x81\x03\x91P[P\x96\x95PPPPPPV[`\0\x80\x80a\x07\xBCa\x07\xB4\x88g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x89\x90\x87a\rHV[\x90P\x86\x15a\x03\x9FWa\x07\xCE\x88\x88a\x11hV[\x91Pa\x07\xEF\x8A\x8A\x84a\x07\xE8\x8Ag\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x89\x89a\x10\xEEV[\x92Pa\x07\xFB\x83\x82a\"\xA8V[\x90P\x97P\x97P\x97\x94PPPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R`\0a\x08:\x86`\0\x01Q\x87` \x01Qa\x0C\x18V[\x90P`\0a\x08V\x82\x88`@\x01Q\x89`\xE0\x01Q\x8A`\xA0\x01Qa\x04!V[\x90P`\0a\x08d\x88\x84a\x11}V[\x90Pa\x08s\x88\x82\x85\x85\x8Ba\x12\xA2V[\x15\x80\x15`@\x87\x01R\x90\x85Ra\x08\x8DW\x93Pa\x03\xBC\x92PPPV[`\0a\x08\x9A\x89\x84\x8Aa\x14\x02V[\x90Pa\x08\xA9\x89\x82\x86\x86\x8Ca\x12\xA2V[\x15\x15`@\x87\x01\x81\x90R\x90\x86Ra\t\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FInitial guess in `calculateMaxSh`D\x82\x01Rp\x1B\xDC\x9D\x18\x08\x1A\\\xC8\x1A[\x9C\xDB\xDB\x1D\x99[\x9D`z\x1B`d\x82\x01R`\x84\x01a\x05^V[`\0[\x87\x81\x10\x15a\t\xAFWa\t2\x8A\x83\x86\x88a\x14\xD8V[\x15\x80\x15`@\x89\x01R` \x88\x01\x91\x90\x91Ra\t\xAFW` \x86\x01Q\x86Q`\0\x91a\tZ\x91\x90a\x10\xB6V[a\td\x90\x84a\"\xA8V[\x90P\x83\x81\x11\x15a\ttWPa\t\xAFV[a\t\x81\x8B\x82\x88\x88\x8Ea\x12\xA2V[\x15\x80\x15`@\x8A\x01R\x90\x88Ra\t\x98W\x80\x92Pa\t\x9EV[Pa\t\xAFV[Pa\t\xA8\x81a\"\xBBV[\x90Pa\t\x1EV[P\x98\x97PPPPPPPPV[`\0\x80`\0\x80a\t\xF2\x86\x88`@\x01Q\x89`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\t\xE3\x91\x90a\"\xD4V[\x8A`\xC0\x01Q\x8B`\xE0\x01Qa\x15_V[\x90Pa\nB\x87a\x01@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\n\x10\x91\x90a\"\xD4V[a\n<g\r\xE0\xB6\xB3\xA7d\0\0a\n&\x81\x8Aa\x11SV[a\n0\x91\x90a\"\xD4V[a\x01 \x8B\x01Q\x90a\x15\x8EV[\x90a\x15\x8EV[\x91Pa\nr\x87a\x01@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\n`\x91\x90a\"\xD4V[a\x07c\x84g\r\xE0\xB6\xB3\xA7d\0\0a\"\xA8V[`\xA0\x88\x01Q\x90\x92Pa\n\xA1\x90a\n\x9A\x90a\n\x94\x81g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x90a\x10\xB6V[\x83\x90a\rfV[\x91Pa\n\xBE\x87`\xE0\x01Q\x88`\xC0\x01Qa\x11S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xC8\x90\x83a\"\xA8V[\x91Pa\n\xD4\x81\x83a\x10\xB6V[\x91Pa\x0B\x03a\n\x9A\x88`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\n\xF4\x91\x90a\"\xD4V[g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x10\xB6V[\x91PP`\0a\x0B\x1F\x87`\xE0\x01Q\x83a\x10\xB6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x80a\x0Br\x89a\x01@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0B@\x91\x90a\"\xD4V[a\x0Blg\r\xE0\xB6\xB3\xA7d\0\0a\x0BV\x81\x8Ca\x10\xB6V[a\x0B`\x91\x90a\"\xD4V[a\x01 \x8D\x01Q\x90a\x11hV[\x90a\x11hV[\x90Pa\x0B\xC8\x84a\x0Bla\x0B\x9A\x8C`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x11S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x042\x8Da\x01@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0B\xB6\x91\x90a\"\xD4V[a\n\x94\x87g\r\xE0\xB6\xB3\xA7d\0\0a\"\xA8V[\x91PPa\x0B\xDF\x88`\xC0\x01Q\x88\x84a\x0Bl\x91\x90a\"\xD4V[\x94Pa\x0B\xF1\x85\x87\x8Aa\x01 \x01Qa\x15\xA3V[\x81\x89`@\x01Qa\x0C\x01\x91\x90a\"\xD4V[a\x0C\x0B\x91\x90a\"\xD4V[\x93PPPP\x93P\x93\x91PPV[`\0\x80a\x0C%\x84\x84a\x15\xD0V[\x90\x92P\x90P\x80a\x0C7Wa\x0C7a\x16\x0FV[P\x92\x91PPV[`\0\x80a\x0Coa\x0C`a\x0CR\x86`da#\x07V[f\xA5\xBB\xED\x86\xC5\xA0\0\x90a\x11hV[gH\xCD@r(\x1E\0\0\x90a\x10\xB6V[\x90Pa\x0C\x83g\r\xE0\xB6\xB3\xA7d\0\0\x82a\x10\xB6V[\x90Pa\x03\xE1\x81a\x0Bla\x0C\xAEa\x0C\xA9a\x0C\xA4\x89g\r\xE0\xB6\xB3\xA7d\0\0a\"\xA8V[a\x10\xFFV[a\x16(V[a\n\x94a\x0C\xA9a\x0C\xC3\x8A\x8Ac\x01\xE13\x80a\rHV[a\x0C\xA4\x90g\r\xE0\xB6\xB3\xA7d\0\0a\"\xA8V[`\0\x80a\x0C\xE7\x84a\n\x94\x8A\x88\x8Aa\rHV[\x90Pa\x07v\x88\x84\x86a\rHV[`\0\x80\x80a\r\x15a\r\r\x88g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x89\x90\x87a\x11-V[\x90P\x86\x15a\x03\x9FWa\r'\x88\x88a\x15\x8EV[\x91Pa\x07\xEF\x8A\x8A\x84a\rA\x8Ag\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x89\x89a\x18WV[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\r_W`\0\x80\xFD[P\x91\x02\x04\x90V[`\0\x81`\0\x03a\r\x7FWPg\r\xE0\xB6\xB3\xA7d\0\0a\x03\xE5V[\x82`\0\x03a\r\x8FWP`\0a\x03\xE5V[`\0a\r\x9A\x83a\x10\xFFV[\x90P`\0a\r\xAAa\x0C\xA9\x86a\x10\xFFV[\x90P\x81\x81\x02a\r\xC1g\r\xE0\xB6\xB3\xA7d\0\0\x82a#\x1EV[\x90Pa\x02s\x81a\x18hV[`\0a\x03\xBCa\r\xE0\x84\x84c\x01\xE13\x80a\x11-V[a\n\x94\x85g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[`\0\x80`\0a\x0E\r\x86\x85\x8Aa\x01 \x01Q\x8Ba\x01`\x01Qa\x19\xFDV[\x90P`\0a\x0E(\x89`\xC0\x01Q\x83a\x10\xB6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xC0\x8A\x01Qa\x0E8\x90\x89\x90a\x10\xB6V[\x8AQa\x0ED\x91\x90a\"\xA8V[a\x0EN\x91\x90a\"\xD4V[\x90P`\0\x86\x8A`\x80\x01Qa\x0Eb\x91\x90a\"\xA8V[\x90P`\0a\x0Ep\x8A\x82a\x1A\x12V[a\x0Ey\x90a#ZV[\x90P\x8Aa\x01\0\x01Qa\x0E\x98\x8C`\xC0\x01Q\x84a\x10\xB6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0E\xA2\x91\x90a\"\xA8V[`\xC0\x8C\x01Qa\x0E\xB2\x90\x83\x90a\x10\xB6V[a\x0E\xBC\x90\x85a\"\xA8V[\x10a\x0F\x14Wa\x01\0\x8B\x01Q`\xC0\x8C\x01Qa\x0E\xD7\x90\x84\x90a\x10\xB6V[`\xC0\x8D\x01Qa\x0E\xE7\x90\x84\x90a\x10\xB6V[a\x0E\xF1\x90\x86a\"\xA8V[a\x0E\xFB\x91\x90a\"\xD4V[a\x0F\x05\x91\x90a\"\xD4V[`\x01\x95P\x95PPPPPa\x0F V[`\0\x80\x95P\x95PPPPP[\x95P\x95\x93PPPPV[`\0\x80a\x0F9\x86\x85\x85\x86a\x1A'V[\x90P`\0a\x0F\x80g\x0B\x1A+\xC2\xECP\0\0a\x0Bla\x0Fv\x8A`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0Fg\x91\x90a\"\xD4V[g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x11SV[a\x042\x86\x8Ba\x10\xB6V[\x90P`\0a\x0F\x96g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x11hV[a\x0F\xB2a\x0F\xAB\x84g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[\x87\x90a\x11hV[a\x0F\xBC\x91\x90a\"\xA8V[\x90Pa\x02\xCA\x88\x87\x87\x84a\x1A'V[`\0\x80a\x10\x01\x84\x87`@\x01Qa\x0F\xED\x89`\xC0\x01Q\x89a\x10\xB6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89`\xA0\x01Q\x8A`\xC0\x01Q\x8B`\xE0\x01Qa\x06pV[\x90Pa\x10\x13\x85\x84\x88a\x01 \x01Qa\x15\xA3V[a\x02s\x90\x82a\"\xD4V[`\0\x80a\x10,\x86\x86\x86\x86a\x1B*V[\x90\x92P\x90P\x80a\x10?W`\0\x91Pa\x10\xADV[a\x10ia\x10T\x84g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[a\x01 \x88\x01Qa\x01`\x89\x01Qa\x0Bl\x91a\x11hV[a\x10s\x90\x83a\"\xA8V[\x91Pa\x10\x87g\r\xE0\xB6\xB3\xA7d\0\0\x83a\"\xD4V[\x91Pa\x10\xAAg\r\xE0\xB6\xB3\xA7d\0\0\x87`\xC0\x01Q\x84a\rH\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91P[\x94P\x94\x92PPPV[`\0a\x03\xBC\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\rHV[`\0\x80a\x10\xDC\x88\x88\x88\x88\x88\x88a\x1CzV[\x90\x92P\x90P\x80a\x07\x90Wa\x07\x90a\x16\x0FV[`\0\x80a\x10\xDC\x88\x88\x88\x88\x88\x88a\x1D4V[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x11)W`@Qc9n\xA7\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x11DW`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`\0a\x03\xBC\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x11-V[`\0a\x03\xBC\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\rHV[`\0\x80a\x11\x98`\0\x85` \x01Qa\x1D\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84a\x01\0\x01Qa\x11\xA8\x91\x90a\"\xA8V[\x90P`\0a\x11\xDC\x84\x86`@\x01Q\x87`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x11\xCD\x91\x90a\"\xD4V[\x88`\xC0\x01Q\x89`\xE0\x01Qa\x15_V[\x90P`\0a\x12+a\x12\x1A\x87`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x11\xFE\x91\x90a\"\xD4V[a\x042a\x12\x0F\x87\x8B` \x01Qa\x0C\x18V[`\xE0\x8B\x01Q\x90a\x15\x8EV[`\xE0\x88\x01Q`\xC0\x89\x01Q\x91\x90a\x11-V[a\x125\x90\x83a\"\xD4V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x12rWa\x12ka\x12d\x87`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\n\xF4\x91\x90a\"\xD4V[\x82\x90a\rfV[\x90Pa\x12\x93V[a\x12\x90a\x12d\x87`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0Fg\x91\x90a\"\xD4V[\x90P[`@\x86\x01Qa\x02s\x90\x82a\"\xD4V[`\0\x80`\0\x80a\x12\xD9\x87\x8A`@\x01Q\x8A\x8C`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x12\xCA\x91\x90a\"\xD4V[\x8D`\xC0\x01Q\x8E`\xE0\x01Qa\x1D4V[\x91P\x91P\x80a\x12\xF0W`\0\x80\x93P\x93PPPa\x0F V[`\0a\x13*\x8A`\xC0\x01Qa\x13\x10\x8B\x8A\x8Ea\x01 \x01Q\x8Fa\x01`\x01Qa\x1E\nV[a\x13 \x8C\x8B\x8Fa\x01 \x01Qa\x1E\x17V[a\x07c\x91\x90a\"\xD4V[\x90P\x80\x83\x10\x15a\x13CW`\0\x80\x94P\x94PPPPa\x0F V[`\0a\x13O\x82\x85a\"\xD4V[\x90P\x80\x8B`\0\x01Q\x10\x15a\x13mW`\0\x80\x95P\x95PPPPPa\x0F V[\x8AQ`\0\x90a\x13}\x90\x83\x90a\"\xD4V[\x90P`\0a\x13\xAC\x8D`\xC0\x01Qa\x13\x9D`\0\x8Ca\x1D\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x8F`\x80\x01Qa\n\x94\x91\x90a\"\xD4V[\x90P\x8Ca\x01\0\x01Q\x81a\x13\xBF\x91\x90a\"\xA8V[\x82\x10a\x13\xF0Wa\x01\0\x8D\x01Qa\x13\xD5\x82\x84a\"\xD4V[a\x13\xDF\x91\x90a\"\xD4V[`\x01\x97P\x97PPPPPPPa\x0F V[`\0\x80\x97P\x97PPPPPPPa\x0F V[`\0\x80\x83\x90P`\0a\x14m\x86a\x01\0\x01Qa\x14.\x88`\xC0\x01Q\x89`\x80\x01Qa\x10\xB6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xC0\x89\x01Qa\x14B\x90a\n\x94\x89`\0a\x1D\xF4V[\x89Qa\x14N\x91\x90a\"\xA8V[a\x14X\x91\x90a\"\xD4V[a\x14b\x91\x90a\"\xD4V[`\xC0\x88\x01Q\x90a\x11hV[\x90Pa\x02sa\x14\x9Ca\x14\x87\x87g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[a\x01 \x89\x01Qa\x01`\x8A\x01Qa\x0Bl\x91a\x11hV[a\x14\xBDa\x14\xB1\x88g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[a\x01 \x8A\x01Q\x90a\x11hV[a\x14\xC7\x90\x85a\"\xD4V[a\x14\xD1\x91\x90a\"\xA8V[\x82\x90a\x10\xB6V[`\0\x80`\0a\x14\xE8\x87\x87\x86a\x1E2V[\x90P`\0a\x151\x88`\xC0\x01Qa\n\x94\x8Aa\x01`\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x15\x10\x91\x90a\"\xD4V[a\x0Bla\x15%\x8Bg\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[a\x01 \x8E\x01Q\x90a\x11hV[\x90P\x80\x82\x10a\x15QWa\x15D\x81\x83a\"\xD4V[`\x01\x93P\x93PPPa\x10\xADV[`\0\x80\x93P\x93PPPa\x10\xADV[`\0a\x15k\x85\x85a\rfV[a\x15\x84a\x15|\x86a\x042\x86\x8Ba\x11hV[\x85\x90\x85a\rHV[a\x02s\x91\x90a\"\xA8V[`\0a\x03\xBC\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11-V[`\0a\x03\xB9\x84a\n<g\r\xE0\xB6\xB3\xA7d\0\0a\x15\xBF\x81\x88a\x11SV[a\x15\xC9\x91\x90a\"\xD4V[\x85\x90a\x15\x8EV[`\0\x80`\0\x83a\x15\xDF\x86a\x10\xFFV[a\x15\xE9\x91\x90a\"\xE7V[\x90P`\0\x81\x12\x15a\x16\x01W`\0\x80\x92P\x92PPa\x16\x08V[\x91P`\x01\x90P[\x92P\x92\x90PV[`@Qc\xBBU\xFD'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x82\x13a\x16JW`@Qc\xE6\x1BIu`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x82\x81\x1C`\x0F\x10`\x02\x1B\x17\x82\x81\x1C\x90\x91\x10`\x01\x90\x81\x1B\x90\x91\x17\x82\x81\x1C\x90\x91\x10\x17`\x9F\x81\x81\x03``\x01\x92\x90\x92\x1B\x91`_\x19\x82\x01\x90a\x16\xD6\x90\x84\x90\x1Ca\x10\xFFV[lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x91\x90\x91\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x80a\x10\xDC\x88\x88\x88\x88\x88\x88a\x1F\x07V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x18\x83WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x18\xACW`@Qcs\xA2\xD6\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x92l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x84\x01\x84\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x85\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x85\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x85\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x85\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x87\x01\x87\x02\x83\x1D\x90\x81\x01\x90\x87\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x81\x02\x90\x92\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x86\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x81\x81\x05\x95P\x92\x93P\x90\x91\x90a\x02st\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x86\x02`\xC3\x86\x90\x03\x1Ca\x10\xFFV[`\0a\x048\x84a\x0Bl\x84a\x0Bl\x89\x89\x89a\x15\xA3V[`\0\x81\x83\x13a\x1A!W\x82a\x03\xBCV[P\x91\x90PV[`\0\x80a\x1A4\x85\x82a\x1A\x12V[a\x1A=\x90a#ZV[\x90P`\0a\x1A\xAA\x87`\xC0\x01Qg\x1B\xC1mgN\xC8\0\0\x89a\x01\0\x01Qa\x1As\x8B`\xC0\x01Q\x8C`\x80\x01Qa\x10\xB6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xC0\x8C\x01Qa\x1A\x83\x90\x88\x90a\x10\xB6V[\x8CQa\x1A\x8F\x91\x90a\"\xA8V[a\x1A\x99\x91\x90a\"\xD4V[a\x1A\xA3\x91\x90a\"\xD4V[\x91\x90a\rHV[\x90Pa\x02\x8Da\x1A\xCFg\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xC5\x81\x89a\x10\xB6V[a\x14\xB1\x91\x90a\"\xD4V[g\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xFAa\x1A\xE5\x89\x83a\"\xD4V[a\x01 \x8C\x01Qa\x01`\x8D\x01Qa\x0Bl\x91a\x11hV[a\x1B\x0Cg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\xB6V[a\x1B\x16\x91\x90a\"\xA8V[a\x1B \x91\x90a\"\xD4V[a\x14\xD1\x91\x90a\"\xD4V[`\0\x80`\0a\x1BF\x87`\xC0\x01Q\x87a\x10\xB6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1Bba\x1BW\x83\x88a\"\xA8V[`\xE0\x8A\x01Q\x90a\x11hV[\x90P`\0a\x1B\x96\x87\x8A`@\x01Q\x8B`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1B\x87\x91\x90a\"\xD4V[\x8C`\xC0\x01Q\x8D`\xE0\x01Qa\x15_V[\x90Pa\x1B\xB2a\n\xF4\x8A`\xA0\x01Q\x84a\rf\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94P`\0a\x1B\xE1a\x1B\xD0\x8B`\xA0\x01Q\x85a\rf\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xE0\x8C\x01Q`\xC0\x8D\x01Q\x91\x90a\rHV[\x90P\x80\x82\x10\x15a\x1B\xFBW`\0\x80\x95P\x95PPPPPa\x10\xADV[a\x1C1a\x0F\xABa\x1C'\x8C`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1C\x1C\x91\x90a\"\xD4V[`\xA0\x8E\x01Q\x90a\x11SV[a\x042\x84\x86a\"\xD4V[\x95Pa\x1C_g\r\xE0\xB6\xB3\xA7d\0\0a\x1CI\x81\x8Aa\x10\xB6V[a\x1CS\x91\x90a\"\xD4V[a\x01 \x8C\x01Q\x90a\x11hV[a\x1Ci\x90\x87a\"\xD4V[\x9A`\x01\x9AP\x98PPPPPPPPPV[`\0\x80`\0a\x1C\x8C\x89\x89\x88\x88\x88a\x1F\xDEV[\x90Pa\x1C\x9F\x86a\x042a\x0F\xAB\x8A\x8Da\"\xA8V[\x98Pa\x1C\xAC\x85\x8A\x86a\rHV[\x98P\x88\x81\x10\x15a\x1C\xC3W`\0\x80\x92P\x92PPa\x1D)V[\x88\x81\x03g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x1C\xF1Wa\x1C\xEAa\x12dg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x11SV[\x90Pa\x1D\tV[a\x1D\x06a\x12dg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\xB6V[\x90P[\x80\x89\x10\x15a\x1D\x1FW`\0\x80\x93P\x93PPPa\x1D)V[\x88\x03\x92P`\x01\x91PP[\x96P\x96\x94PPPPPV[`\0\x80`\0a\x1DF\x89\x89\x88\x88\x88a\x1F\xDEV[\x90Pa\x1DV\x86a\x042\x89\x8Ba\"\xA8V[\x97P\x87\x81\x10\x15a\x1DmW`\0\x80\x92P\x92PPa\x1D)V[\x87\x81\x03a\x1D{\x81\x86\x88a\x11-V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x1D\xA8Wa\x1D\xA1a\x12dg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x11SV[\x90Pa\x1D\xC0V[a\x1D\xBDa\x12dg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\xB6V[\x90P[a\x1D\xCA\x81\x86a\x11SV[\x90P\x80\x8A\x10\x15a\x1D\xE2W`\0\x80\x93P\x93PPPa\x1D)V[\x90\x98\x03\x98`\x01\x98P\x96PPPPPPPV[`\0\x81\x83\x13a\x1E\x03W\x81a\x03\xBCV[P\x90\x91\x90PV[`\0a\x048\x82a\x0Bl\x87\x87\x87[`\0a\x03\xB9\x84a\n<a\x15\xC9\x86g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD4V[`\0\x80a\x1EV\x83\x86`@\x01Q\x87`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x11\xCD\x91\x90a\"\xD4V[\x90P`\0a\x1E\x83a\n\xF4a\x1Ex\x88`\xA0\x01Q\x88\x8A`@\x01Qa\x042\x91\x90a\"\xA8V[`\xC0\x89\x01Q\x90a\x15\x8EV[\x90P`\0a\x1E\xFBa\x1E\xB0\x88`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xA5\x91\x90a\"\xD4V[`\xA0\x8A\x01Q\x90a\x11SV[a\x042a\x1E\xDE\x8A`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xCE\x91\x90a\"\xD4V[\x8A\x8C`@\x01Qa\x042\x91\x90a\"\xA8V[a\x1E\xE8\x90\x87a\"\xD4V[`\xC0\x8B\x01Q`\xE0\x8C\x01Qa\x0Bl\x91a\x10\xB6V[\x90Pa\x02\x8D\x82\x82a\x11hV[`\0\x80`\0a\x1F\x19\x89\x89\x88\x88\x88a\x1F\xDEV[\x90P\x86\x88\x10\x15a\x1F0W`\0\x80\x92P\x92PPa\x1D)V[\x96\x86\x90\x03\x96a\x1F?\x88\x87a\rfV[\x97P\x87\x81\x10\x15a\x1FVW`\0\x80\x92P\x92PPa\x1D)V[\x87\x81\x03a\x1Fd\x81\x86\x88a\x11-V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x1F\x91Wa\x1F\x8Aa\x12dg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x11SV[\x90Pa\x1F\xA9V[a\x1F\xA6a\x12dg\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\xB6V[\x90P[a\x1F\xB3\x81\x86a\x11SV[\x90P\x89\x81\x10\x15a\x1F\xCBW`\0\x80\x93P\x93PPPa\x1D)V[\x98\x90\x98\x03\x98`\x01\x98P\x96PPPPPPPV[`\0a\x1F\xEA\x85\x85a\rfV[a\x15\x84a\x1F\xFB\x86a\x042\x86\x8Ba\x15\x8EV[\x85\x90\x85a\x11-V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a \x19W`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a MW`\0\x80\xFD[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`@Qa\x01\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a \xA2WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0a\x01\x80\x82\x84\x03\x12\x15a \xBBW`\0\x80\xFD[a \xC3a pV[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015`\x80\x82\x01R`\xA0\x82\x015`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R`\xE0\x82\x015`\xE0\x82\x01Ra\x01\0\x80\x83\x015\x81\x83\x01RPa\x01 \x80\x83\x015\x81\x83\x01RPa\x01@\x80\x83\x015\x81\x83\x01RPa\x01`\x80\x83\x015\x81\x83\x01RP\x92\x91PPV[`\0\x80`\0a\x01\xC0\x84\x86\x03\x12\x15a![W`\0\x80\xFD[a!e\x85\x85a \xA8V[\x95a\x01\x80\x85\x015\x95Pa\x01\xA0\x90\x94\x015\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a!\x95W`\0\x80\xFD[PP\x845\x96` \x86\x015\x96P`@\x86\x015\x95``\x81\x015\x95P`\x80\x81\x015\x94P`\xA0\x015\x92P\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a!\xDAW`\0\x80\xFD[\x875\x96P` \x88\x015\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015\x80\x15\x15\x81\x14a\"\x14W`\0\x80\xFD[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\"?W`\0\x80\xFD[PP\x855\x97` \x87\x015\x97P`@\x87\x015\x96``\x81\x015\x96P`\x80\x81\x015\x95P`\xA0\x81\x015\x94P`\xC0\x015\x92P\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\"\x83W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x03\xE5Wa\x03\xE5a\"\x92V[`\0`\x01\x82\x01a\"\xCDWa\"\xCDa\"\x92V[P`\x01\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x03\xE5Wa\x03\xE5a\"\x92V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x0C7Wa\x0C7a\"\x92V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xE5Wa\x03\xE5a\"\x92V[`\0\x82a#;WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a#UWa#Ua\"\x92V[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a#oWa#oa\"\x92V[P`\0\x03\x90V\xFE\xA2dipfsX\"\x12 \xC5\xDAb :J2\xA6\x95\xEE\xAE]\xE9\xB2\x94\xE2=qy\x9B\x0F6b\xC6\x8E\xFD\x8D\x91\xA3\x9D\xD7\xA8dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKHYPERDRIVEMATH_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockHyperdriveMath<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockHyperdriveMath<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockHyperdriveMath<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockHyperdriveMath<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockHyperdriveMath<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockHyperdriveMath))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockHyperdriveMath<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKHYPERDRIVEMATH_ABI.clone(),
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
                MOCKHYPERDRIVEMATH_ABI.clone(),
                MOCKHYPERDRIVEMATH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `calculateAbsoluteMaxLong` (0xa280a282) function
        pub fn calculate_absolute_max_long(
            &self,
            params: MaxTradeParams,
            effective_share_reserves: ::ethers::core::types::U256,
            spot_price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [162, 128, 162, 130],
                    (params, effective_share_reserves, spot_price),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateCloseLong` (0x94169d49) function
        pub fn calculate_close_long(
            &self,
            effective_share_reserves: ::ethers::core::types::U256,
            bond_reserves: ::ethers::core::types::U256,
            amount_in: ::ethers::core::types::U256,
            normalized_time_remaining: ::ethers::core::types::U256,
            time_stretch: ::ethers::core::types::U256,
            vault_share_price: ::ethers::core::types::U256,
            initial_vault_share_price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [148, 22, 157, 73],
                    (
                        effective_share_reserves,
                        bond_reserves,
                        amount_in,
                        normalized_time_remaining,
                        time_stretch,
                        vault_share_price,
                        initial_vault_share_price,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateCloseShort` (0xf87845f2) function
        pub fn calculate_close_short(
            &self,
            effective_share_reserves: ::ethers::core::types::U256,
            bond_reserves: ::ethers::core::types::U256,
            amount_out: ::ethers::core::types::U256,
            normalized_time_remaining: ::ethers::core::types::U256,
            time_stretch: ::ethers::core::types::U256,
            vault_share_price: ::ethers::core::types::U256,
            initial_vault_share_price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [248, 120, 69, 242],
                    (
                        effective_share_reserves,
                        bond_reserves,
                        amount_out,
                        normalized_time_remaining,
                        time_stretch,
                        vault_share_price,
                        initial_vault_share_price,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateEffectiveShareReserves` (0xbc1e1065) function
        pub fn calculate_effective_share_reserves(
            &self,
            share_reserves: ::ethers::core::types::U256,
            share_adjustment: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([188, 30, 16, 101], (share_reserves, share_adjustment))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateMaxLong` (0x3f77b617) function
        pub fn calculate_max_long(
            &self,
            params: MaxTradeParams,
            checkpoint_exposure: ::ethers::core::types::I256,
            max_iterations: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [63, 119, 182, 23],
                    (params, checkpoint_exposure, max_iterations),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateMaxShort` (0x9e489b99) function
        pub fn calculate_max_short(
            &self,
            params: MaxTradeParams,
            checkpoint_exposure: ::ethers::core::types::I256,
            max_iterations: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [158, 72, 155, 153],
                    (params, checkpoint_exposure, max_iterations),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateNegativeInterestOnClose` (0x7ef99f87) function
        pub fn calculate_negative_interest_on_close(
            &self,
            share_proceeds: ::ethers::core::types::U256,
            share_reserves_delta: ::ethers::core::types::U256,
            share_curve_delta: ::ethers::core::types::U256,
            total_governance_fee: ::ethers::core::types::U256,
            open_vault_share_price: ::ethers::core::types::U256,
            close_vault_share_price: ::ethers::core::types::U256,
            is_long: bool,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [126, 249, 159, 135],
                    (
                        share_proceeds,
                        share_reserves_delta,
                        share_curve_delta,
                        total_governance_fee,
                        open_vault_share_price,
                        close_vault_share_price,
                        is_long,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateOpenLong` (0x5e6d9d36) function
        pub fn calculate_open_long(
            &self,
            effective_share_reserves: ::ethers::core::types::U256,
            bond_reserves: ::ethers::core::types::U256,
            amount_in: ::ethers::core::types::U256,
            time_stretch: ::ethers::core::types::U256,
            vault_share_price: ::ethers::core::types::U256,
            initial_vault_share_price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [94, 109, 157, 54],
                    (
                        effective_share_reserves,
                        bond_reserves,
                        amount_in,
                        time_stretch,
                        vault_share_price,
                        initial_vault_share_price,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateOpenShort` (0x6ceebe1d) function
        pub fn calculate_open_short(
            &self,
            effective_share_reserves: ::ethers::core::types::U256,
            bond_reserves: ::ethers::core::types::U256,
            amount_in: ::ethers::core::types::U256,
            time_stretch: ::ethers::core::types::U256,
            vault_share_price: ::ethers::core::types::U256,
            initial_vault_share_price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [108, 238, 190, 29],
                    (
                        effective_share_reserves,
                        bond_reserves,
                        amount_in,
                        time_stretch,
                        vault_share_price,
                        initial_vault_share_price,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateShortProceedsDown` (0xe7acd3ad) function
        pub fn calculate_short_proceeds_down(
            &self,
            bond_amount: ::ethers::core::types::U256,
            share_amount: ::ethers::core::types::U256,
            open_vault_share_price: ::ethers::core::types::U256,
            close_vault_share_price: ::ethers::core::types::U256,
            vault_share_price: ::ethers::core::types::U256,
            flat_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [231, 172, 211, 173],
                    (
                        bond_amount,
                        share_amount,
                        open_vault_share_price,
                        close_vault_share_price,
                        vault_share_price,
                        flat_fee,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateShortProceedsUp` (0x902b1099) function
        pub fn calculate_short_proceeds_up(
            &self,
            bond_amount: ::ethers::core::types::U256,
            share_amount: ::ethers::core::types::U256,
            open_vault_share_price: ::ethers::core::types::U256,
            close_vault_share_price: ::ethers::core::types::U256,
            vault_share_price: ::ethers::core::types::U256,
            flat_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [144, 43, 16, 153],
                    (
                        bond_amount,
                        share_amount,
                        open_vault_share_price,
                        close_vault_share_price,
                        vault_share_price,
                        flat_fee,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateSpotAPR` (0x2810a036) function
        pub fn calculate_spot_apr(
            &self,
            effective_share_reserves: ::ethers::core::types::U256,
            bond_reserves: ::ethers::core::types::U256,
            initial_vault_share_price: ::ethers::core::types::U256,
            position_duration: ::ethers::core::types::U256,
            time_stretch: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [40, 16, 160, 54],
                    (
                        effective_share_reserves,
                        bond_reserves,
                        initial_vault_share_price,
                        position_duration,
                        time_stretch,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateSpotPrice` (0x2471caed) function
        pub fn calculate_spot_price(
            &self,
            share_reserves: ::ethers::core::types::U256,
            bond_reserves: ::ethers::core::types::U256,
            initial_vault_share_price: ::ethers::core::types::U256,
            time_stretch: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [36, 113, 202, 237],
                    (
                        share_reserves,
                        bond_reserves,
                        initial_vault_share_price,
                        time_stretch,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateTimeStretch` (0xdbab8512) function
        pub fn calculate_time_stretch(
            &self,
            apr: ::ethers::core::types::U256,
            position_duration: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([219, 171, 133, 18], (apr, position_duration))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockHyperdriveMath<M> {
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
    pub enum MockHyperdriveMathErrors {
        ExpInvalidExponent(ExpInvalidExponent),
        InsufficientLiquidity(InsufficientLiquidity),
        LnInvalidInput(LnInvalidInput),
        UnsafeCastToInt256(UnsafeCastToInt256),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockHyperdriveMathErrors {
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
    impl ::ethers::core::abi::AbiEncode for MockHyperdriveMathErrors {
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
    impl ::ethers::contract::ContractRevert for MockHyperdriveMathErrors {
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
    impl ::core::fmt::Display for MockHyperdriveMathErrors {
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
    impl ::core::convert::From<::std::string::String> for MockHyperdriveMathErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ExpInvalidExponent> for MockHyperdriveMathErrors {
        fn from(value: ExpInvalidExponent) -> Self {
            Self::ExpInvalidExponent(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidity> for MockHyperdriveMathErrors {
        fn from(value: InsufficientLiquidity) -> Self {
            Self::InsufficientLiquidity(value)
        }
    }
    impl ::core::convert::From<LnInvalidInput> for MockHyperdriveMathErrors {
        fn from(value: LnInvalidInput) -> Self {
            Self::LnInvalidInput(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToInt256> for MockHyperdriveMathErrors {
        fn from(value: UnsafeCastToInt256) -> Self {
            Self::UnsafeCastToInt256(value)
        }
    }
    ///Container type for all input parameters for the `calculateAbsoluteMaxLong` function with signature `calculateAbsoluteMaxLong((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256)` and selector `0xa280a282`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateAbsoluteMaxLong",
        abi = "calculateAbsoluteMaxLong((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256)"
    )]
    pub struct CalculateAbsoluteMaxLongCall {
        pub params: MaxTradeParams,
        pub effective_share_reserves: ::ethers::core::types::U256,
        pub spot_price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateCloseLong` function with signature `calculateCloseLong(uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x94169d49`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateCloseLong",
        abi = "calculateCloseLong(uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateCloseLongCall {
        pub effective_share_reserves: ::ethers::core::types::U256,
        pub bond_reserves: ::ethers::core::types::U256,
        pub amount_in: ::ethers::core::types::U256,
        pub normalized_time_remaining: ::ethers::core::types::U256,
        pub time_stretch: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub initial_vault_share_price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateCloseShort` function with signature `calculateCloseShort(uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xf87845f2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateCloseShort",
        abi = "calculateCloseShort(uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateCloseShortCall {
        pub effective_share_reserves: ::ethers::core::types::U256,
        pub bond_reserves: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
        pub normalized_time_remaining: ::ethers::core::types::U256,
        pub time_stretch: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub initial_vault_share_price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateEffectiveShareReserves` function with signature `calculateEffectiveShareReserves(uint256,int256)` and selector `0xbc1e1065`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateEffectiveShareReserves",
        abi = "calculateEffectiveShareReserves(uint256,int256)"
    )]
    pub struct CalculateEffectiveShareReservesCall {
        pub share_reserves: ::ethers::core::types::U256,
        pub share_adjustment: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `calculateMaxLong` function with signature `calculateMaxLong((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),int256,uint256)` and selector `0x3f77b617`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateMaxLong",
        abi = "calculateMaxLong((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),int256,uint256)"
    )]
    pub struct CalculateMaxLongCall {
        pub params: MaxTradeParams,
        pub checkpoint_exposure: ::ethers::core::types::I256,
        pub max_iterations: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateMaxShort` function with signature `calculateMaxShort((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),int256,uint256)` and selector `0x9e489b99`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateMaxShort",
        abi = "calculateMaxShort((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),int256,uint256)"
    )]
    pub struct CalculateMaxShortCall {
        pub params: MaxTradeParams,
        pub checkpoint_exposure: ::ethers::core::types::I256,
        pub max_iterations: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateNegativeInterestOnClose` function with signature `calculateNegativeInterestOnClose(uint256,uint256,uint256,uint256,uint256,uint256,bool)` and selector `0x7ef99f87`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateNegativeInterestOnClose",
        abi = "calculateNegativeInterestOnClose(uint256,uint256,uint256,uint256,uint256,uint256,bool)"
    )]
    pub struct CalculateNegativeInterestOnCloseCall {
        pub share_proceeds: ::ethers::core::types::U256,
        pub share_reserves_delta: ::ethers::core::types::U256,
        pub share_curve_delta: ::ethers::core::types::U256,
        pub total_governance_fee: ::ethers::core::types::U256,
        pub open_vault_share_price: ::ethers::core::types::U256,
        pub close_vault_share_price: ::ethers::core::types::U256,
        pub is_long: bool,
    }
    ///Container type for all input parameters for the `calculateOpenLong` function with signature `calculateOpenLong(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x5e6d9d36`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateOpenLong",
        abi = "calculateOpenLong(uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateOpenLongCall {
        pub effective_share_reserves: ::ethers::core::types::U256,
        pub bond_reserves: ::ethers::core::types::U256,
        pub amount_in: ::ethers::core::types::U256,
        pub time_stretch: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub initial_vault_share_price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateOpenShort` function with signature `calculateOpenShort(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x6ceebe1d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateOpenShort",
        abi = "calculateOpenShort(uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateOpenShortCall {
        pub effective_share_reserves: ::ethers::core::types::U256,
        pub bond_reserves: ::ethers::core::types::U256,
        pub amount_in: ::ethers::core::types::U256,
        pub time_stretch: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub initial_vault_share_price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateShortProceedsDown` function with signature `calculateShortProceedsDown(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xe7acd3ad`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateShortProceedsDown",
        abi = "calculateShortProceedsDown(uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateShortProceedsDownCall {
        pub bond_amount: ::ethers::core::types::U256,
        pub share_amount: ::ethers::core::types::U256,
        pub open_vault_share_price: ::ethers::core::types::U256,
        pub close_vault_share_price: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub flat_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateShortProceedsUp` function with signature `calculateShortProceedsUp(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x902b1099`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateShortProceedsUp",
        abi = "calculateShortProceedsUp(uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateShortProceedsUpCall {
        pub bond_amount: ::ethers::core::types::U256,
        pub share_amount: ::ethers::core::types::U256,
        pub open_vault_share_price: ::ethers::core::types::U256,
        pub close_vault_share_price: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub flat_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateSpotAPR` function with signature `calculateSpotAPR(uint256,uint256,uint256,uint256,uint256)` and selector `0x2810a036`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateSpotAPR",
        abi = "calculateSpotAPR(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateSpotAPRCall {
        pub effective_share_reserves: ::ethers::core::types::U256,
        pub bond_reserves: ::ethers::core::types::U256,
        pub initial_vault_share_price: ::ethers::core::types::U256,
        pub position_duration: ::ethers::core::types::U256,
        pub time_stretch: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateSpotPrice` function with signature `calculateSpotPrice(uint256,uint256,uint256,uint256)` and selector `0x2471caed`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateSpotPrice",
        abi = "calculateSpotPrice(uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateSpotPriceCall {
        pub share_reserves: ::ethers::core::types::U256,
        pub bond_reserves: ::ethers::core::types::U256,
        pub initial_vault_share_price: ::ethers::core::types::U256,
        pub time_stretch: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateTimeStretch` function with signature `calculateTimeStretch(uint256,uint256)` and selector `0xdbab8512`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateTimeStretch",
        abi = "calculateTimeStretch(uint256,uint256)"
    )]
    pub struct CalculateTimeStretchCall {
        pub apr: ::ethers::core::types::U256,
        pub position_duration: ::ethers::core::types::U256,
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
    pub enum MockHyperdriveMathCalls {
        CalculateAbsoluteMaxLong(CalculateAbsoluteMaxLongCall),
        CalculateCloseLong(CalculateCloseLongCall),
        CalculateCloseShort(CalculateCloseShortCall),
        CalculateEffectiveShareReserves(CalculateEffectiveShareReservesCall),
        CalculateMaxLong(CalculateMaxLongCall),
        CalculateMaxShort(CalculateMaxShortCall),
        CalculateNegativeInterestOnClose(CalculateNegativeInterestOnCloseCall),
        CalculateOpenLong(CalculateOpenLongCall),
        CalculateOpenShort(CalculateOpenShortCall),
        CalculateShortProceedsDown(CalculateShortProceedsDownCall),
        CalculateShortProceedsUp(CalculateShortProceedsUpCall),
        CalculateSpotAPR(CalculateSpotAPRCall),
        CalculateSpotPrice(CalculateSpotPriceCall),
        CalculateTimeStretch(CalculateTimeStretchCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockHyperdriveMathCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CalculateAbsoluteMaxLongCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateAbsoluteMaxLong(decoded));
            }
            if let Ok(decoded) = <CalculateCloseLongCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateCloseLong(decoded));
            }
            if let Ok(decoded) = <CalculateCloseShortCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateCloseShort(decoded));
            }
            if let Ok(decoded) = <CalculateEffectiveShareReservesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateEffectiveShareReserves(decoded));
            }
            if let Ok(decoded) = <CalculateMaxLongCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateMaxLong(decoded));
            }
            if let Ok(decoded) = <CalculateMaxShortCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateMaxShort(decoded));
            }
            if let Ok(decoded) = <CalculateNegativeInterestOnCloseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateNegativeInterestOnClose(decoded));
            }
            if let Ok(decoded) = <CalculateOpenLongCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateOpenLong(decoded));
            }
            if let Ok(decoded) = <CalculateOpenShortCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateOpenShort(decoded));
            }
            if let Ok(decoded) = <CalculateShortProceedsDownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateShortProceedsDown(decoded));
            }
            if let Ok(decoded) = <CalculateShortProceedsUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateShortProceedsUp(decoded));
            }
            if let Ok(decoded) = <CalculateSpotAPRCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateSpotAPR(decoded));
            }
            if let Ok(decoded) = <CalculateSpotPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateSpotPrice(decoded));
            }
            if let Ok(decoded) = <CalculateTimeStretchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateTimeStretch(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockHyperdriveMathCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CalculateAbsoluteMaxLong(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateCloseLong(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateCloseShort(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateEffectiveShareReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateMaxLong(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateMaxShort(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateNegativeInterestOnClose(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateOpenLong(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateOpenShort(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateShortProceedsDown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateShortProceedsUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateSpotAPR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateSpotPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateTimeStretch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockHyperdriveMathCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CalculateAbsoluteMaxLong(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateCloseLong(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateCloseShort(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateEffectiveShareReserves(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateMaxLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateMaxShort(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateNegativeInterestOnClose(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateOpenLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateOpenShort(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateShortProceedsDown(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateShortProceedsUp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateSpotAPR(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateSpotPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateTimeStretch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CalculateAbsoluteMaxLongCall>
    for MockHyperdriveMathCalls {
        fn from(value: CalculateAbsoluteMaxLongCall) -> Self {
            Self::CalculateAbsoluteMaxLong(value)
        }
    }
    impl ::core::convert::From<CalculateCloseLongCall> for MockHyperdriveMathCalls {
        fn from(value: CalculateCloseLongCall) -> Self {
            Self::CalculateCloseLong(value)
        }
    }
    impl ::core::convert::From<CalculateCloseShortCall> for MockHyperdriveMathCalls {
        fn from(value: CalculateCloseShortCall) -> Self {
            Self::CalculateCloseShort(value)
        }
    }
    impl ::core::convert::From<CalculateEffectiveShareReservesCall>
    for MockHyperdriveMathCalls {
        fn from(value: CalculateEffectiveShareReservesCall) -> Self {
            Self::CalculateEffectiveShareReserves(value)
        }
    }
    impl ::core::convert::From<CalculateMaxLongCall> for MockHyperdriveMathCalls {
        fn from(value: CalculateMaxLongCall) -> Self {
            Self::CalculateMaxLong(value)
        }
    }
    impl ::core::convert::From<CalculateMaxShortCall> for MockHyperdriveMathCalls {
        fn from(value: CalculateMaxShortCall) -> Self {
            Self::CalculateMaxShort(value)
        }
    }
    impl ::core::convert::From<CalculateNegativeInterestOnCloseCall>
    for MockHyperdriveMathCalls {
        fn from(value: CalculateNegativeInterestOnCloseCall) -> Self {
            Self::CalculateNegativeInterestOnClose(value)
        }
    }
    impl ::core::convert::From<CalculateOpenLongCall> for MockHyperdriveMathCalls {
        fn from(value: CalculateOpenLongCall) -> Self {
            Self::CalculateOpenLong(value)
        }
    }
    impl ::core::convert::From<CalculateOpenShortCall> for MockHyperdriveMathCalls {
        fn from(value: CalculateOpenShortCall) -> Self {
            Self::CalculateOpenShort(value)
        }
    }
    impl ::core::convert::From<CalculateShortProceedsDownCall>
    for MockHyperdriveMathCalls {
        fn from(value: CalculateShortProceedsDownCall) -> Self {
            Self::CalculateShortProceedsDown(value)
        }
    }
    impl ::core::convert::From<CalculateShortProceedsUpCall>
    for MockHyperdriveMathCalls {
        fn from(value: CalculateShortProceedsUpCall) -> Self {
            Self::CalculateShortProceedsUp(value)
        }
    }
    impl ::core::convert::From<CalculateSpotAPRCall> for MockHyperdriveMathCalls {
        fn from(value: CalculateSpotAPRCall) -> Self {
            Self::CalculateSpotAPR(value)
        }
    }
    impl ::core::convert::From<CalculateSpotPriceCall> for MockHyperdriveMathCalls {
        fn from(value: CalculateSpotPriceCall) -> Self {
            Self::CalculateSpotPrice(value)
        }
    }
    impl ::core::convert::From<CalculateTimeStretchCall> for MockHyperdriveMathCalls {
        fn from(value: CalculateTimeStretchCall) -> Self {
            Self::CalculateTimeStretch(value)
        }
    }
    ///Container type for all return fields from the `calculateAbsoluteMaxLong` function with signature `calculateAbsoluteMaxLong((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256)` and selector `0xa280a282`
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
    pub struct CalculateAbsoluteMaxLongReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `calculateCloseLong` function with signature `calculateCloseLong(uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x94169d49`
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
    pub struct CalculateCloseLongReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `calculateCloseShort` function with signature `calculateCloseShort(uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xf87845f2`
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
    pub struct CalculateCloseShortReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `calculateEffectiveShareReserves` function with signature `calculateEffectiveShareReserves(uint256,int256)` and selector `0xbc1e1065`
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
    pub struct CalculateEffectiveShareReservesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateMaxLong` function with signature `calculateMaxLong((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),int256,uint256)` and selector `0x3f77b617`
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
    pub struct CalculateMaxLongReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `calculateMaxShort` function with signature `calculateMaxShort((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),int256,uint256)` and selector `0x9e489b99`
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
    pub struct CalculateMaxShortReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateNegativeInterestOnClose` function with signature `calculateNegativeInterestOnClose(uint256,uint256,uint256,uint256,uint256,uint256,bool)` and selector `0x7ef99f87`
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
    pub struct CalculateNegativeInterestOnCloseReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::I256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `calculateOpenLong` function with signature `calculateOpenLong(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x5e6d9d36`
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
    pub struct CalculateOpenLongReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateOpenShort` function with signature `calculateOpenShort(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x6ceebe1d`
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
    pub struct CalculateOpenShortReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateShortProceedsDown` function with signature `calculateShortProceedsDown(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xe7acd3ad`
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
    pub struct CalculateShortProceedsDownReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateShortProceedsUp` function with signature `calculateShortProceedsUp(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x902b1099`
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
    pub struct CalculateShortProceedsUpReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateSpotAPR` function with signature `calculateSpotAPR(uint256,uint256,uint256,uint256,uint256)` and selector `0x2810a036`
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
    pub struct CalculateSpotAPRReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateSpotPrice` function with signature `calculateSpotPrice(uint256,uint256,uint256,uint256)` and selector `0x2471caed`
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
    pub struct CalculateSpotPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateTimeStretch` function with signature `calculateTimeStretch(uint256,uint256)` and selector `0xdbab8512`
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
    pub struct CalculateTimeStretchReturn(pub ::ethers::core::types::U256);
    ///`MaxTradeParams(uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)`
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
    pub struct MaxTradeParams {
        pub share_reserves: ::ethers::core::types::U256,
        pub share_adjustment: ::ethers::core::types::I256,
        pub bond_reserves: ::ethers::core::types::U256,
        pub longs_outstanding: ::ethers::core::types::U256,
        pub long_exposure: ::ethers::core::types::U256,
        pub time_stretch: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub initial_vault_share_price: ::ethers::core::types::U256,
        pub minimum_share_reserves: ::ethers::core::types::U256,
        pub curve_fee: ::ethers::core::types::U256,
        pub flat_fee: ::ethers::core::types::U256,
        pub governance_lp_fee: ::ethers::core::types::U256,
    }
}
