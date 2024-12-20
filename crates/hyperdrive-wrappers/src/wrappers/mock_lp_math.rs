pub use mock_lp_math::*;
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
pub mod mock_lp_math {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("calculateDistributeExcessIdle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateDistributeExcessIdle",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LPMath.DistributeExcessIdleParams",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned(
                        "calculateDistributeExcessIdleShareProceeds",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateDistributeExcessIdleShareProceeds",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LPMath.DistributeExcessIdleParams",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_originalEffectiveShareReserves",
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
                                        "_maxShareReservesDelta",
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
                    ::std::borrow::ToOwned::to_owned(
                        "calculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafe",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafe",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LPMath.DistributeExcessIdleParams",
                                        ),
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
                        "calculateDistributeExcessIdleWithdrawalSharesRedeemed",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateDistributeExcessIdleWithdrawalSharesRedeemed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LPMath.DistributeExcessIdleParams",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("calculateInitialReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateInitialReserves",
                            ),
                            inputs: ::std::vec![
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_targetApr"),
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
                                    name: ::std::borrow::ToOwned::to_owned("shareReserves"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shareAdjustment"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bondReserves"),
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
                        "calculateMaxShareReservesDeltaSafe",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateMaxShareReservesDeltaSafe",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LPMath.DistributeExcessIdleParams",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_originalEffectiveShareReserves",
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
                    ::std::borrow::ToOwned::to_owned("calculateNetCurveTrade"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateNetCurveTrade",
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
                                            "struct LPMath.PresentValueParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateNetFlatTrade"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateNetFlatTrade",
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
                                            "struct LPMath.PresentValueParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculatePresentValue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculatePresentValue",
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
                                            "struct LPMath.PresentValueParams",
                                        ),
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
                        "calculateSharesDeltaGivenBondsDeltaDerivativeSafe",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateSharesDeltaGivenBondsDeltaDerivativeSafe",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LPMath.DistributeExcessIdleParams",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_originalEffectiveShareReserves",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bondAmount"),
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
                    ::std::borrow::ToOwned::to_owned("calculateUpdateLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateUpdateLiquidity",
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
                                        "_minimumShareReserves",
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
                                        "_shareReservesDelta",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shareReserves"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shareAdjustment"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bondReserves"),
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
                        "shouldShortCircuitDistributeExcessIdleShareProceeds",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "shouldShortCircuitDistributeExcessIdleShareProceeds",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LPMath.DistributeExcessIdleParams",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_lpTotalSupply"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_presentValue"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidPresentValue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidPresentValue",
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
    pub static MOCKLPMATH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct MockLPMath<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockLPMath<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockLPMath<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockLPMath<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockLPMath<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockLPMath)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockLPMath<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKLPMATH_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `calculateDistributeExcessIdle` (0x865d65a2) function
        pub fn calculate_distribute_excess_idle(
            &self,
            params: DistributeExcessIdleParams,
            max_iterations: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([134, 93, 101, 162], (params, max_iterations))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateDistributeExcessIdleShareProceeds` (0x3f9ccc01) function
        pub fn calculate_distribute_excess_idle_share_proceeds(
            &self,
            params: DistributeExcessIdleParams,
            original_effective_share_reserves: ::ethers::core::types::U256,
            max_share_reserves_delta: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [63, 156, 204, 1],
                    (params, original_effective_share_reserves, max_share_reserves_delta),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafe` (0xcefef8b7) function
        pub fn calculate_distribute_excess_idle_share_proceeds_net_long_edge_case_safe(
            &self,
            params: DistributeExcessIdleParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, bool),
        > {
            self.0
                .method_hash([206, 254, 248, 183], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateDistributeExcessIdleWithdrawalSharesRedeemed` (0xa0d1643c) function
        pub fn calculate_distribute_excess_idle_withdrawal_shares_redeemed(
            &self,
            params: DistributeExcessIdleParams,
            share_reserves_delta: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([160, 209, 100, 60], (params, share_reserves_delta))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateInitialReserves` (0xb29e1d1e) function
        pub fn calculate_initial_reserves(
            &self,
            share_amount: ::ethers::core::types::U256,
            vault_share_price: ::ethers::core::types::U256,
            initial_vault_share_price: ::ethers::core::types::U256,
            target_apr: ::ethers::core::types::U256,
            position_duration: ::ethers::core::types::U256,
            time_stretch: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [178, 158, 29, 30],
                    (
                        share_amount,
                        vault_share_price,
                        initial_vault_share_price,
                        target_apr,
                        position_duration,
                        time_stretch,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateMaxShareReservesDeltaSafe` (0x9e5fc16b) function
        pub fn calculate_max_share_reserves_delta_safe(
            &self,
            params: DistributeExcessIdleParams,
            original_effective_share_reserves: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, bool),
        > {
            self.0
                .method_hash(
                    [158, 95, 193, 107],
                    (params, original_effective_share_reserves),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateNetCurveTrade` (0xa4fc6f81) function
        pub fn calculate_net_curve_trade(
            &self,
            params: PresentValueParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([164, 252, 111, 129], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateNetFlatTrade` (0x32c5dec4) function
        pub fn calculate_net_flat_trade(
            &self,
            params: PresentValueParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([50, 197, 222, 196], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculatePresentValue` (0xc00b11d3) function
        pub fn calculate_present_value(
            &self,
            params: PresentValueParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([192, 11, 17, 211], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateSharesDeltaGivenBondsDeltaDerivativeSafe` (0xc615b2d9) function
        pub fn calculate_shares_delta_given_bonds_delta_derivative_safe(
            &self,
            params: DistributeExcessIdleParams,
            original_effective_share_reserves: ::ethers::core::types::U256,
            bond_amount: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, bool),
        > {
            self.0
                .method_hash(
                    [198, 21, 178, 217],
                    (params, original_effective_share_reserves, bond_amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateUpdateLiquidity` (0xb500f3cb) function
        pub fn calculate_update_liquidity(
            &self,
            share_reserves: ::ethers::core::types::U256,
            share_adjustment: ::ethers::core::types::I256,
            bond_reserves: ::ethers::core::types::U256,
            minimum_share_reserves: ::ethers::core::types::U256,
            share_reserves_delta: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [181, 0, 243, 203],
                    (
                        share_reserves,
                        share_adjustment,
                        bond_reserves,
                        minimum_share_reserves,
                        share_reserves_delta,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shouldShortCircuitDistributeExcessIdleShareProceeds` (0x8f1505fa) function
        pub fn should_short_circuit_distribute_excess_idle_share_proceeds(
            &self,
            params: DistributeExcessIdleParams,
            lp_total_supply: ::ethers::core::types::U256,
            present_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([143, 21, 5, 250], (params, lp_total_supply, present_value))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockLPMath<M> {
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
    ///Custom Error type `InvalidPresentValue` with signature `InvalidPresentValue()` and selector `0xaa2c6516`
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
    #[etherror(name = "InvalidPresentValue", abi = "InvalidPresentValue()")]
    pub struct InvalidPresentValue;
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
    pub enum MockLPMathErrors {
        ExpInvalidExponent(ExpInvalidExponent),
        InvalidPresentValue(InvalidPresentValue),
        LnInvalidInput(LnInvalidInput),
        UnsafeCastToInt256(UnsafeCastToInt256),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockLPMathErrors {
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
            if let Ok(decoded) = <InvalidPresentValue as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidPresentValue(decoded));
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
    impl ::ethers::core::abi::AbiEncode for MockLPMathErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ExpInvalidExponent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPresentValue(element) => {
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
    impl ::ethers::contract::ContractRevert for MockLPMathErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ExpInvalidExponent as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidPresentValue as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for MockLPMathErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExpInvalidExponent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidPresentValue(element) => {
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
    impl ::core::convert::From<::std::string::String> for MockLPMathErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ExpInvalidExponent> for MockLPMathErrors {
        fn from(value: ExpInvalidExponent) -> Self {
            Self::ExpInvalidExponent(value)
        }
    }
    impl ::core::convert::From<InvalidPresentValue> for MockLPMathErrors {
        fn from(value: InvalidPresentValue) -> Self {
            Self::InvalidPresentValue(value)
        }
    }
    impl ::core::convert::From<LnInvalidInput> for MockLPMathErrors {
        fn from(value: LnInvalidInput) -> Self {
            Self::LnInvalidInput(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToInt256> for MockLPMathErrors {
        fn from(value: UnsafeCastToInt256) -> Self {
            Self::UnsafeCastToInt256(value)
        }
    }
    ///Container type for all input parameters for the `calculateDistributeExcessIdle` function with signature `calculateDistributeExcessIdle(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256)` and selector `0x865d65a2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateDistributeExcessIdle",
        abi = "calculateDistributeExcessIdle(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256)"
    )]
    pub struct CalculateDistributeExcessIdleCall {
        pub params: DistributeExcessIdleParams,
        pub max_iterations: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateDistributeExcessIdleShareProceeds` function with signature `calculateDistributeExcessIdleShareProceeds(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256,uint256)` and selector `0x3f9ccc01`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateDistributeExcessIdleShareProceeds",
        abi = "calculateDistributeExcessIdleShareProceeds(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256,uint256)"
    )]
    pub struct CalculateDistributeExcessIdleShareProceedsCall {
        pub params: DistributeExcessIdleParams,
        pub original_effective_share_reserves: ::ethers::core::types::U256,
        pub max_share_reserves_delta: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafe` function with signature `calculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafe(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256))` and selector `0xcefef8b7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafe",
        abi = "calculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafe(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256))"
    )]
    pub struct CalculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafeCall {
        pub params: DistributeExcessIdleParams,
    }
    ///Container type for all input parameters for the `calculateDistributeExcessIdleWithdrawalSharesRedeemed` function with signature `calculateDistributeExcessIdleWithdrawalSharesRedeemed(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256)` and selector `0xa0d1643c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateDistributeExcessIdleWithdrawalSharesRedeemed",
        abi = "calculateDistributeExcessIdleWithdrawalSharesRedeemed(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256)"
    )]
    pub struct CalculateDistributeExcessIdleWithdrawalSharesRedeemedCall {
        pub params: DistributeExcessIdleParams,
        pub share_reserves_delta: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateInitialReserves` function with signature `calculateInitialReserves(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xb29e1d1e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateInitialReserves",
        abi = "calculateInitialReserves(uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CalculateInitialReservesCall {
        pub share_amount: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub initial_vault_share_price: ::ethers::core::types::U256,
        pub target_apr: ::ethers::core::types::U256,
        pub position_duration: ::ethers::core::types::U256,
        pub time_stretch: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateMaxShareReservesDeltaSafe` function with signature `calculateMaxShareReservesDeltaSafe(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256)` and selector `0x9e5fc16b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateMaxShareReservesDeltaSafe",
        abi = "calculateMaxShareReservesDeltaSafe(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256)"
    )]
    pub struct CalculateMaxShareReservesDeltaSafeCall {
        pub params: DistributeExcessIdleParams,
        pub original_effective_share_reserves: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateNetCurveTrade` function with signature `calculateNetCurveTrade((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0xa4fc6f81`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateNetCurveTrade",
        abi = "calculateNetCurveTrade((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct CalculateNetCurveTradeCall {
        pub params: PresentValueParams,
    }
    ///Container type for all input parameters for the `calculateNetFlatTrade` function with signature `calculateNetFlatTrade((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0x32c5dec4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateNetFlatTrade",
        abi = "calculateNetFlatTrade((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct CalculateNetFlatTradeCall {
        pub params: PresentValueParams,
    }
    ///Container type for all input parameters for the `calculatePresentValue` function with signature `calculatePresentValue((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0xc00b11d3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculatePresentValue",
        abi = "calculatePresentValue((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct CalculatePresentValueCall {
        pub params: PresentValueParams,
    }
    ///Container type for all input parameters for the `calculateSharesDeltaGivenBondsDeltaDerivativeSafe` function with signature `calculateSharesDeltaGivenBondsDeltaDerivativeSafe(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256,int256)` and selector `0xc615b2d9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateSharesDeltaGivenBondsDeltaDerivativeSafe",
        abi = "calculateSharesDeltaGivenBondsDeltaDerivativeSafe(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256,int256)"
    )]
    pub struct CalculateSharesDeltaGivenBondsDeltaDerivativeSafeCall {
        pub params: DistributeExcessIdleParams,
        pub original_effective_share_reserves: ::ethers::core::types::U256,
        pub bond_amount: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `calculateUpdateLiquidity` function with signature `calculateUpdateLiquidity(uint256,int256,uint256,uint256,int256)` and selector `0xb500f3cb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateUpdateLiquidity",
        abi = "calculateUpdateLiquidity(uint256,int256,uint256,uint256,int256)"
    )]
    pub struct CalculateUpdateLiquidityCall {
        pub share_reserves: ::ethers::core::types::U256,
        pub share_adjustment: ::ethers::core::types::I256,
        pub bond_reserves: ::ethers::core::types::U256,
        pub minimum_share_reserves: ::ethers::core::types::U256,
        pub share_reserves_delta: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `shouldShortCircuitDistributeExcessIdleShareProceeds` function with signature `shouldShortCircuitDistributeExcessIdleShareProceeds(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256,uint256)` and selector `0x8f1505fa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "shouldShortCircuitDistributeExcessIdleShareProceeds",
        abi = "shouldShortCircuitDistributeExcessIdleShareProceeds(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256,uint256)"
    )]
    pub struct ShouldShortCircuitDistributeExcessIdleShareProceedsCall {
        pub params: DistributeExcessIdleParams,
        pub lp_total_supply: ::ethers::core::types::U256,
        pub present_value: ::ethers::core::types::U256,
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
    pub enum MockLPMathCalls {
        CalculateDistributeExcessIdle(CalculateDistributeExcessIdleCall),
        CalculateDistributeExcessIdleShareProceeds(
            CalculateDistributeExcessIdleShareProceedsCall,
        ),
        CalculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafe(
            CalculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafeCall,
        ),
        CalculateDistributeExcessIdleWithdrawalSharesRedeemed(
            CalculateDistributeExcessIdleWithdrawalSharesRedeemedCall,
        ),
        CalculateInitialReserves(CalculateInitialReservesCall),
        CalculateMaxShareReservesDeltaSafe(CalculateMaxShareReservesDeltaSafeCall),
        CalculateNetCurveTrade(CalculateNetCurveTradeCall),
        CalculateNetFlatTrade(CalculateNetFlatTradeCall),
        CalculatePresentValue(CalculatePresentValueCall),
        CalculateSharesDeltaGivenBondsDeltaDerivativeSafe(
            CalculateSharesDeltaGivenBondsDeltaDerivativeSafeCall,
        ),
        CalculateUpdateLiquidity(CalculateUpdateLiquidityCall),
        ShouldShortCircuitDistributeExcessIdleShareProceeds(
            ShouldShortCircuitDistributeExcessIdleShareProceedsCall,
        ),
    }
    impl ::ethers::core::abi::AbiDecode for MockLPMathCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CalculateDistributeExcessIdleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateDistributeExcessIdle(decoded));
            }
            if let Ok(decoded) = <CalculateDistributeExcessIdleShareProceedsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateDistributeExcessIdleShareProceeds(decoded));
            }
            if let Ok(decoded) = <CalculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::CalculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafe(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = <CalculateDistributeExcessIdleWithdrawalSharesRedeemedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::CalculateDistributeExcessIdleWithdrawalSharesRedeemed(decoded),
                );
            }
            if let Ok(decoded) = <CalculateInitialReservesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateInitialReserves(decoded));
            }
            if let Ok(decoded) = <CalculateMaxShareReservesDeltaSafeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateMaxShareReservesDeltaSafe(decoded));
            }
            if let Ok(decoded) = <CalculateNetCurveTradeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateNetCurveTrade(decoded));
            }
            if let Ok(decoded) = <CalculateNetFlatTradeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateNetFlatTrade(decoded));
            }
            if let Ok(decoded) = <CalculatePresentValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculatePresentValue(decoded));
            }
            if let Ok(decoded) = <CalculateSharesDeltaGivenBondsDeltaDerivativeSafeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::CalculateSharesDeltaGivenBondsDeltaDerivativeSafe(decoded),
                );
            }
            if let Ok(decoded) = <CalculateUpdateLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateUpdateLiquidity(decoded));
            }
            if let Ok(decoded) = <ShouldShortCircuitDistributeExcessIdleShareProceedsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::ShouldShortCircuitDistributeExcessIdleShareProceeds(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockLPMathCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CalculateDistributeExcessIdle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateDistributeExcessIdleShareProceeds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafe(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CalculateDistributeExcessIdleWithdrawalSharesRedeemed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateInitialReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateMaxShareReservesDeltaSafe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateNetCurveTrade(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateNetFlatTrade(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculatePresentValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateSharesDeltaGivenBondsDeltaDerivativeSafe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateUpdateLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ShouldShortCircuitDistributeExcessIdleShareProceeds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockLPMathCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CalculateDistributeExcessIdle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateDistributeExcessIdleShareProceeds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafe(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateDistributeExcessIdleWithdrawalSharesRedeemed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateInitialReserves(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateMaxShareReservesDeltaSafe(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateNetCurveTrade(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateNetFlatTrade(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculatePresentValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateSharesDeltaGivenBondsDeltaDerivativeSafe(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateUpdateLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ShouldShortCircuitDistributeExcessIdleShareProceeds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CalculateDistributeExcessIdleCall> for MockLPMathCalls {
        fn from(value: CalculateDistributeExcessIdleCall) -> Self {
            Self::CalculateDistributeExcessIdle(value)
        }
    }
    impl ::core::convert::From<CalculateDistributeExcessIdleShareProceedsCall>
    for MockLPMathCalls {
        fn from(value: CalculateDistributeExcessIdleShareProceedsCall) -> Self {
            Self::CalculateDistributeExcessIdleShareProceeds(value)
        }
    }
    impl ::core::convert::From<
        CalculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafeCall,
    > for MockLPMathCalls {
        fn from(
            value: CalculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafeCall,
        ) -> Self {
            Self::CalculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafe(value)
        }
    }
    impl ::core::convert::From<CalculateDistributeExcessIdleWithdrawalSharesRedeemedCall>
    for MockLPMathCalls {
        fn from(
            value: CalculateDistributeExcessIdleWithdrawalSharesRedeemedCall,
        ) -> Self {
            Self::CalculateDistributeExcessIdleWithdrawalSharesRedeemed(value)
        }
    }
    impl ::core::convert::From<CalculateInitialReservesCall> for MockLPMathCalls {
        fn from(value: CalculateInitialReservesCall) -> Self {
            Self::CalculateInitialReserves(value)
        }
    }
    impl ::core::convert::From<CalculateMaxShareReservesDeltaSafeCall>
    for MockLPMathCalls {
        fn from(value: CalculateMaxShareReservesDeltaSafeCall) -> Self {
            Self::CalculateMaxShareReservesDeltaSafe(value)
        }
    }
    impl ::core::convert::From<CalculateNetCurveTradeCall> for MockLPMathCalls {
        fn from(value: CalculateNetCurveTradeCall) -> Self {
            Self::CalculateNetCurveTrade(value)
        }
    }
    impl ::core::convert::From<CalculateNetFlatTradeCall> for MockLPMathCalls {
        fn from(value: CalculateNetFlatTradeCall) -> Self {
            Self::CalculateNetFlatTrade(value)
        }
    }
    impl ::core::convert::From<CalculatePresentValueCall> for MockLPMathCalls {
        fn from(value: CalculatePresentValueCall) -> Self {
            Self::CalculatePresentValue(value)
        }
    }
    impl ::core::convert::From<CalculateSharesDeltaGivenBondsDeltaDerivativeSafeCall>
    for MockLPMathCalls {
        fn from(value: CalculateSharesDeltaGivenBondsDeltaDerivativeSafeCall) -> Self {
            Self::CalculateSharesDeltaGivenBondsDeltaDerivativeSafe(value)
        }
    }
    impl ::core::convert::From<CalculateUpdateLiquidityCall> for MockLPMathCalls {
        fn from(value: CalculateUpdateLiquidityCall) -> Self {
            Self::CalculateUpdateLiquidity(value)
        }
    }
    impl ::core::convert::From<ShouldShortCircuitDistributeExcessIdleShareProceedsCall>
    for MockLPMathCalls {
        fn from(value: ShouldShortCircuitDistributeExcessIdleShareProceedsCall) -> Self {
            Self::ShouldShortCircuitDistributeExcessIdleShareProceeds(value)
        }
    }
    ///Container type for all return fields from the `calculateDistributeExcessIdle` function with signature `calculateDistributeExcessIdle(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256)` and selector `0x865d65a2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculateDistributeExcessIdleReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `calculateDistributeExcessIdleShareProceeds` function with signature `calculateDistributeExcessIdleShareProceeds(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256,uint256)` and selector `0x3f9ccc01`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculateDistributeExcessIdleShareProceedsReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `calculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafe` function with signature `calculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafe(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256))` and selector `0xcefef8b7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculateDistributeExcessIdleShareProceedsNetLongEdgeCaseSafeReturn(
        pub ::ethers::core::types::U256,
        pub bool,
    );
    ///Container type for all return fields from the `calculateDistributeExcessIdleWithdrawalSharesRedeemed` function with signature `calculateDistributeExcessIdleWithdrawalSharesRedeemed(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256)` and selector `0xa0d1643c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculateDistributeExcessIdleWithdrawalSharesRedeemedReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `calculateInitialReserves` function with signature `calculateInitialReserves(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xb29e1d1e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculateInitialReservesReturn {
        pub share_reserves: ::ethers::core::types::U256,
        pub share_adjustment: ::ethers::core::types::I256,
        pub bond_reserves: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `calculateMaxShareReservesDeltaSafe` function with signature `calculateMaxShareReservesDeltaSafe(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256)` and selector `0x9e5fc16b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculateMaxShareReservesDeltaSafeReturn(
        pub ::ethers::core::types::U256,
        pub bool,
    );
    ///Container type for all return fields from the `calculateNetCurveTrade` function with signature `calculateNetCurveTrade((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0xa4fc6f81`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculateNetCurveTradeReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `calculateNetFlatTrade` function with signature `calculateNetFlatTrade((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0x32c5dec4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculateNetFlatTradeReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `calculatePresentValue` function with signature `calculatePresentValue((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0xc00b11d3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculatePresentValueReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateSharesDeltaGivenBondsDeltaDerivativeSafe` function with signature `calculateSharesDeltaGivenBondsDeltaDerivativeSafe(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256,int256)` and selector `0xc615b2d9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculateSharesDeltaGivenBondsDeltaDerivativeSafeReturn(
        pub ::ethers::core::types::U256,
        pub bool,
    );
    ///Container type for all return fields from the `calculateUpdateLiquidity` function with signature `calculateUpdateLiquidity(uint256,int256,uint256,uint256,int256)` and selector `0xb500f3cb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculateUpdateLiquidityReturn {
        pub share_reserves: ::ethers::core::types::U256,
        pub share_adjustment: ::ethers::core::types::I256,
        pub bond_reserves: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `shouldShortCircuitDistributeExcessIdleShareProceeds` function with signature `shouldShortCircuitDistributeExcessIdleShareProceeds(((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256),uint256,uint256)` and selector `0x8f1505fa`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ShouldShortCircuitDistributeExcessIdleShareProceedsReturn(pub bool);
    ///`DistributeExcessIdleParams((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint256,uint256,uint256,uint256,int256,uint256,int256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DistributeExcessIdleParams {
        pub present_value_params: PresentValueParams,
        pub starting_present_value: ::ethers::core::types::U256,
        pub active_lp_total_supply: ::ethers::core::types::U256,
        pub withdrawal_shares_total_supply: ::ethers::core::types::U256,
        pub idle: ::ethers::core::types::U256,
        pub net_curve_trade: ::ethers::core::types::I256,
        pub original_share_reserves: ::ethers::core::types::U256,
        pub original_share_adjustment: ::ethers::core::types::I256,
        pub original_bond_reserves: ::ethers::core::types::U256,
    }
    ///`PresentValueParams(uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PresentValueParams {
        pub share_reserves: ::ethers::core::types::U256,
        pub share_adjustment: ::ethers::core::types::I256,
        pub bond_reserves: ::ethers::core::types::U256,
        pub vault_share_price: ::ethers::core::types::U256,
        pub initial_vault_share_price: ::ethers::core::types::U256,
        pub minimum_share_reserves: ::ethers::core::types::U256,
        pub minimum_transaction_amount: ::ethers::core::types::U256,
        pub time_stretch: ::ethers::core::types::U256,
        pub longs_outstanding: ::ethers::core::types::U256,
        pub long_average_time_remaining: ::ethers::core::types::U256,
        pub shorts_outstanding: ::ethers::core::types::U256,
        pub short_average_time_remaining: ::ethers::core::types::U256,
    }
}

pub struct MockLPMathLibs {
    pub lp_math: ::ethers::types::Address,
}

impl<M: ::ethers::providers::Middleware> MockLPMath<M> {
    pub fn link_and_deploy<T: ::ethers::core::abi::Tokenize>(
        client: ::std::sync::Arc<M>,
        constructor_args: T,
        libs: MockLPMathLibs,
    ) -> ::core::result::Result<
        ::ethers::contract::builders::ContractDeployer<M, Self>,
        ::ethers::contract::ContractError<M>,
    > {
        let factory = crate::linked_factory::create(
            MOCKLPMATH_ABI.clone(),
            "0x608060405234801561001057600080fd5b50612615806100206000396000f3fe608060405234801561001057600080fd5b50600436106100b45760003560e01c8063a4fc6f8111610071578063a4fc6f8114610178578063b29e1d1e1461018b578063b500f3cb146101b9578063c00b11d3146101cc578063c615b2d9146101df578063cefef8b7146101f257600080fd5b806332c5dec4146100b95780633f9ccc01146100df578063865d65a2146100f25780638f1505fa1461011a5780639e5fc16b1461013d578063a0d1643c14610165575b600080fd5b6100cc6100c73660046121ab565b610205565b6040519081526020015b60405180910390f35b6100cc6100ed366004612250565b610216565b610105610100366004612287565b61022d565b604080519283526020830191909152016100d6565b61012d610128366004612250565b6102b5565b60405190151581526020016100d6565b61015061014b366004612287565b6102c2565b604080519283529015156020830152016100d6565b6100cc610173366004612287565b6102cf565b6100cc6101863660046121ab565b6102e2565b61019e6101993660046122b4565b610363565b604080519384526020840192909252908201526060016100d6565b61019e6101c73660046122f7565b610413565b6100cc6101da3660046121ab565b610527565b6101506101ed366004612250565b6105a2565b610150610200366004612332565b6105bd565b6000610210826105d2565b92915050565b60006102258484846004610645565b949350505050565b60008073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__632c03ef6885856040518363ffffffff1660e01b81526004016102699291906123cd565b6040805180830381865af4158015610285573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102a99190612443565b915091505b9250929050565b6000610225848484610b6c565b6000806102a98484610bd8565b60006102db8383610ce7565b9392505050565b60008060006102f084610db4565b915091508061035c5760405162461bcd60e51b815260206004820152602d60248201527f4d6f636b4c504d6174683a2063616c63756c6174654e6574437572766554726160448201526c191954d859994819985a5b1959609a1b60648201526084015b60405180910390fd5b5092915050565b60405163594f0e8f60e11b8152600481018790526024810186905260448101859052606481018490526084810183905260a481018290526000908190819073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__9063b29e1d1e9060c401606060405180830381865af41580156103dd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104019190612467565b919b909a509098509650505050505050565b60405163685a2be760e11b8152600481018690526024810185905260448101849052606481018390526084810182905260009081908190819073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__9063d0b457ce9060a401608060405180830381865af4158015610488573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104ac9190612495565b929650909450925090508061051b5760405162461bcd60e51b815260206004820152602f60248201527f4d6f636b4c504d6174683a2063616c63756c6174655570646174654c6971756960448201526e191a5d1e54d859994819985a5b1959608a1b6064820152608401610353565b50955095509592505050565b604051635f9d50ab60e11b815260009073__$2b4fa6f02a36eedfe41c65e8dd342257d3$__9063bf3aa156906105619085906004016124db565b602060405180830381865af415801561057e573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061021091906124ea565b6000806105b08585856110e3565b915091505b935093915050565b6000806105c98361144b565b91509150915091565b600061060a610605836101200151670de0b6b3a76400006105f39190612519565b60608501516101008601519190611521565b611547565b61063b610605846101600151670de0b6b3a76400006106299190612519565b60608601516101408701519190611575565b610210919061252c565b6000808560600151866040015161065c919061254c565b602087015160608801519192506000916106769184611575565b90508660a0015160000361068d5791506102259050565b600080808960048810156106a057600497505b60005b88811015610a0e576106b5868b611593565b955060006106ec8360c001518460e00151856101000151866000015160a001516106de8c611547565b6106e79061255f565b6115a8565b8651604081019290925260208201929092529190915290508061071a57600098505050505050505050610225565b60006107298e600001516116c0565b92509050816107445760009950505050505050505050610225565b61074f84828b610b6c565b1561076557879950505050505050505050610225565b60008460a0015113156108ce57835180516020820151604083015160a084015160e0909401516000946107b9949392916107a790670de0b6b3a7640000612519565b8a51606081015160809091015161174d565b93509050826107d55760009a5050505050505050505050610225565b808560a00151106108cc576107e98561144b565b9099509250826108065760009a5050505050505050505050610225565b61082b8560c001518660e00151876101000151886000015160a001516106de8e611547565b8851604081019290925260208201929092529190915292508261085b5760009a5050505050505050505050610225565b845180516020820151604083015160a084015160e09094015161088b94906107a790670de0b6b3a7640000612519565b93509050826108a75760009a5050505050505050505050610225565b8e60a0015181116108c457889a5050505050505050505050610225565b505050610a06565b505b60006108df858f8760a001516110e3565b935090508215806108f85750670de0b6b3a76400008110155b156109105760009a5050505050505050505050610225565b80670de0b6b3a7640000039050600061093d6106058760400151886020015161185290919063ffffffff16565b61094a610605858e611867565b610954919061252c565b905088158061097257506109678961187c565b6109708261187c565b105b15610981578098508997508296505b60008113156109af5761099e8b610998838561188f565b9061188f565b6109a8908b61254c565b9950610a01565b60008112156109f85760006109c98c61099885818661255f565b90508a8110156109dd57808b039a506109f2565b60009c50505050505050505050505050610225565b50610a01565b50505050610a0e565b505050505b6001016106a3565b506000610a368260c001518360e00151846101000151856000015160a001516106de8b611547565b85516040810192909252602082019290925291909152905080610a63576000975050505050505050610225565b6000610a7283600001516118a4565b90506000610a946106058560400151866020015161185290919063ffffffff16565b610aa1610605848c611867565b610aab919061252c565b9050610ab68761187c565b610abf8261187c565b1015610acc578795508194505b610af3610ae7655af3107a4000670de0b6b3a7640000612519565b6020860151908b611521565b6040850151610b0390879061188f565b1080610b435750610b31610b25655af3107a4000670de0b6b3a764000061254c565b6020860151908b611575565b6040850151610b419087906118d4565b115b15610b5a5760009950505050505050505050610225565b50939c9b505050505050505050505050565b6020830151600090610b7e90846118d4565b6040850151610b8e90849061188f565b1015801561022557506020840151610bbe9084610bb7633b9aca00670de0b6b3a764000061254c565b9190611575565b6040850151610bce9084906118d4565b1115949350505050565b60008060008460a0015112610bf5575050608082015160016102ae565b60008460a00151610c059061255f565b90506000610c4185876101000151886000015160e00151670de0b6b3a7640000610c2f9190612519565b895160608101516080909101516118e9565b93509050821580610c50575080155b15610c63576000809350935050506102ae565b6000610c6f83836118d4565b9050670de0b6b3a76400008111610cab5780670de0b6b3a7640000039450610ca48760c001518661186790919063ffffffff16565b9450610cba565b600080945094505050506102ae565b8660800151851115610cd95786608001516001945094505050506102ae565b506001925050509250929050565b600080610d0f8460c001518560e00151866101000151876000015160a001516106de88611547565b87516040810192909252602082019290925291909152905080610d36576000915050610210565b6000610d4585600001516116c0565b9250905081610d5957600092505050610210565b84602001518110610d6f57600092505050610210565b600085606001518660400151610d85919061254c565b9050610da0828760200151836115759092919063ffffffff16565b610daa9082612519565b9695505050505050565b6000806000610dd961060585610160015186610140015161186790919063ffffffff16565b610df961060586610120015187610100015161185290919063ffffffff16565b610e03919061252c565b9050600080610e1a86600001518760200151611995565b9150915080610e3157506000958695509350505050565b6000831315610f815760008390506000610e7f886000015189602001518a604001518b60a001518c60e00151670de0b6b3a7640000610e709190612519565b8d606001518e6080015161174d565b9350905082610e98575060009788975095505050505050565b818110610f36576000610ed2858a60400151858c60e00151670de0b6b3a7640000610ec39190612519565b8d606001518e608001516119d1565b9450905083158015610ee757508860c0015183105b15610efe5750600098600198509650505050505050565b83610f1457506000988998509650505050505050565b610f1d81611547565b610f269061255f565b9960019950975050505050505050565b6000886020015112610f6d57610f558860a00151856106059190612519565b610f5e9061255f565b98600198509650505050505050565b60a08801518851610f559161060591612519565b60008312156110d4576000610f958461255f565b90506000610fc98489604001518a60e00151670de0b6b3a7640000610fba9190612519565b8b606001518c608001516118e9565b9350905082610fe2575060009788975095505050505050565b81811061106757600061101c858a60400151858c60e00151670de0b6b3a764000061100d9190612519565b8d606001518e60800151611a94565b945090508315801561103157508860c0015183105b156110485750600098600198509650505050505050565b8361105e57506000988998509650505050505050565b610f2681611547565b6000611099858a604001518b60e00151670de0b6b3a764000061108a9190612519565b8c606001518d60800151611b6b565b94509050836110b357506000988998509650505050505050565b610f266110ca8a6060015184866109989190612519565b610605908361254c565b50600095600195509350505050565b60008060008084126111075785516040015161110090859061254c565b9050611142565b60006111128561255f565b875160400151909150811015611132578651604001518190039150611140565b6000809350935050506105b5565b505b85518051602090910151600091829161115b9190611995565b915091508061117357600080945094505050506105b5565b875160e08101516040909101516000916111a89161119c916111959190611c21565b8a90611867565b6101008b0151906118d4565b895160e08101516080909101516111d9916111cd916111c79088611867565b90611c21565b8b5160600151906118d4565b6111e3919061254c565b9050600061121b61120f6112088c6000015160e0015188611c2190919063ffffffff16565b8b90611852565b6101008c01519061188f565b905080821015611236576000809650965050505050506105b5565b80820391506000611278858c60000151604001518d6000015160e00151670de0b6b3a76400006112669190612519565b8e516060810151608090910151611c8c565b905060006112a28c6000015160e00151670de0b6b3a764000061129b9190612519565b8890611c21565b9050808210156112bf5760008098509850505050505050506105b5565b8b516080810151606090910151918303916112db918391611521565b9050670de0b6b3a76400008110611326578b5160e0015161131f906113189061130c90670de0b6b3a7640000612519565b8e5160e00151906118d4565b8290611c21565b9050611355565b8b5160e00151611352906113189061134690670de0b6b3a7640000612519565b8e5160e001519061188f565b90505b8b51606001516113689085908390611521565b935083670de0b6b3a7640000111561138c5783670de0b6b3a76400000393506113a0565b6000600198509850505050505050506105b5565b60008c60e00151126114035760c08c015160e08d01516113bf916118d4565b9250670de0b6b3a76400008311156113e45760008098509850505050505050506105b5565b670de0b6b3a764000092909203916113fc8484611867565b9350611438565b61143561141c8d60c001518e60e001516109989061255f565b61142e90670de0b6b3a764000061254c565b8590611867565b93505b50919a60019a5098505050505050505050565b60008060008360e001511361146557506000928392509050565b600061147484600001516105d2565b905060006114a0856040015186606001518760400151611494919061254c565b60208801519190611521565b9050600082126114c757808210156114ba578190036114dd565b5060009485945092505050565b6114d08261255f565b6114da908261254c565b90505b60e085015160c08601516114f2918390611521565b9050808560c00151101561150d575060009485945092505050565b808560c00151036001935093505050915091565b600082600019048411830215820261153857600080fd5b50910281810615159190040190565b60006001600160ff1b038211156115715760405163396ea70160e11b815260040160405180910390fd5b5090565b600082600019048411830215820261158c57600080fd5b5091020490565b60008183116115a257826102db565b50919050565b600080600080846000036115c7575087925086915085905060016116b4565b6000856115d38b611547565b6115dd919061257b565b90506115e887611547565b811215611603576000806000809450945094509450506116b4565b809450600089126116235761161c610605868b8d611575565b9350611646565b61163a6106056116328b61255f565b87908d611521565b6116439061255f565b93505b6000806116538c8c611995565b91509150806116725760008060008096509650965096505050506116b4565b600061167e8888611995565b925090508161169e576000806000809750975097509750505050506116b4565b6116a98b8285611575565b955060019450505050505b95509550955095915050565b60008060008060006116d186610db4565b91509150806116e857506000958695509350505050565b6116f58660a00151611547565b6116fe876105d2565b8361170c8960000151611547565b611716919061257b565b611720919061257b565b61172a919061252c565b9250505060008112156117435750600093849350915050565b9360019350915050565b600080600088121561176f576117628861255f565b61176c908761254c565b95505b60008061177c8b8b611995565b915091508061179357600080935093505050611846565b60006117a2838b8a8a8a611cbb565b905060006117bf6117b78a6111c78a8e611852565b899089611521565b9050808210156117d9576000809550955050505050611846565b808203670de0b6b3a7640000811061180757611800611318670de0b6b3a76400008c61188f565b905061181f565b61181c611318670de0b6b3a76400008c6118d4565b90505b8b81101561183857600080965096505050505050611846565b8b9003955060019450505050505b97509795505050505050565b60006102db8383670de0b6b3a7640000611521565b60006102db8383670de0b6b3a7640000611575565b6000808212156115715781600003610210565b60006102db83670de0b6b3a764000084611575565b60008060006118b2846116c0565b915091508061035c57604051635516328b60e11b815260040160405180910390fd5b60006102db83670de0b6b3a764000084611521565b60008060006118fb8888888888611c8c565b90506000611926670de0b6b3a7640000611915888861188f565b61191f919061254c565b83906118d4565b9050670de0b6b3a764000081106119535761194c611318670de0b6b3a7640000896118d4565b905061196b565b611968611318670de0b6b3a76400008961188f565b90505b808810156119815760008093509350505061198b565b8703925060019150505b9550959350505050565b6000806000836119a486611547565b6119ae919061252c565b905060008112156119c65760008092509250506102ae565b946001945092505050565b60008060006119e38989888888611c8c565b90506119f3866111c7898b61254c565b975087811015611a0a576000809250925050611a89565b878103611a18818688611521565b9050670de0b6b3a76400008110611a4557611a3e611318670de0b6b3a7640000896118d4565b9050611a5d565b611a5a611318670de0b6b3a76400008961188f565b90505b611a6781866118d4565b9050808a1015611a7f57600080935093505050611a89565b8903925060019150505b965096945050505050565b6000806000611aa68989888888611c8c565b905086881015611abd576000809250925050611a89565b9686900396611acc8887611c21565b975087811015611ae3576000809250925050611a89565b878103611af1818688611521565b9050670de0b6b3a76400008110611b1e57611b17611318670de0b6b3a7640000896118d4565b9050611b36565b611b33611318670de0b6b3a76400008961188f565b90505b611b4081866118d4565b905089811015611b5857600080935093505050611a89565b9890980398600198509650505050505050565b6000806000611b7d8888888888611cbb565b90506000611ba8670de0b6b3a7640000611b9788886118d4565b611ba1919061254c565b839061188f565b9050670de0b6b3a76400008110611bd557611bce611318670de0b6b3a76400008961188f565b9050611bed565b611bea611318670de0b6b3a7640000896118d4565b90505b611bf7818661188f565b905088811015611c0f5760008093509350505061198b565b97909703976001975095505050505050565b600081600003611c3a5750670de0b6b3a7640000610210565b82600003611c4a57506000610210565b6000611c5583611547565b90506000611c6a611c6586611547565b611ce0565b9050818102611c81670de0b6b3a7640000826125a3565b9050610daa81611f0f565b6000611c988585611c21565b611cb1611ca9866111c7868b611852565b859085611521565b610daa919061254c565b6000611cc78585611c21565b611cb1611cd8866111c7868b611867565b859085611575565b6000808213611d025760405163e61b497560e01b815260040160405180910390fd5b506fffffffffffffffffffffffffffffffff811160071b81811c67ffffffffffffffff1060061b1781811c63ffffffff1060051b1781811c61ffff1060041b1781811c60ff10600390811b90911782811c600f1060021b1782811c909110600190811b90911782811c90911017609f8181036060019290921b91605f19820190611d8e9084901c611547565b6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d6c8c3f38e95a6b1ff2ab1c3b343619018302821d6d02384773bdf1ac5676facced60901901830290911d6cb9a025d814b29c212b8b1a07cd1901909102780a09507084cc699bb0e71ea869ffffffffffffffffffffffff190105711340daa0d5f769dba1915cef59f0815a5506027d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b391909102017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d92915050565b6000680248ce36a70cb26b3e198213611f2a57506000919050565b680755bf798b4a1bf1e58212611f53576040516373a2d6b160e01b815260040160405180910390fd5b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056001605f1b01901d6bb17217f7d1cf79abc9e3b3988102909303926c240c330e9fb2d9cbaf0fd5aafb1984018402606090811d6d0277594991cfc85f6e2461837cd9018502811d6d1a521255e34f6a5061b25ef1c9c319018502811d6db1bbb201f443cf962f1a1d3db4a5018502811d6e02c72388d9f74f51a9331fed693f1419018502811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765087018702831d9081019087016d01d3967ed30fc4f89c02bab570811901810290921d6e0587f503bb6ea29d25fcb7401964500186026d360d7aeea093263ecc6e0ecb291760621b018181059550929350909190610daa74029d9dc38563c32e5c2f6dc192ee70ef65f9978af3860260c38690031c611547565b604051610180810167ffffffffffffffff811182821017156120d657634e487b7160e01b600052604160045260246000fd5b60405290565b604051610120810167ffffffffffffffff811182821017156120d657634e487b7160e01b600052604160045260246000fd5b6000610180828403121561212157600080fd5b6121296120a4565b9050813581526020820135602082015260408201356040820152606082013560608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e082015261010080830135818301525061012080830135818301525061014080830135818301525061016080830135818301525092915050565b600061018082840312156121be57600080fd5b6102db838361210e565b600061028082840312156121db57600080fd5b6121e36120dc565b90506121ef838361210e565b815261018082013560208201526101a082013560408201526101c082013560608201526101e0820135608082015261020082013560a082015261022082013560c082015261024082013560e082015261026082013561010082015292915050565b60008060006102c0848603121561226657600080fd5b61227085856121c8565b9561028085013595506102a0909401359392505050565b6000806102a0838503121561229b57600080fd5b6122a584846121c8565b94610280939093013593505050565b60008060008060008060c087890312156122cd57600080fd5b505084359660208601359650604086013595606081013595506080810135945060a0013592509050565b600080600080600060a0868803121561230f57600080fd5b505083359560208501359550604085013594606081013594506080013592509050565b6000610280828403121561234557600080fd5b6102db83836121c8565b805182526020810151602083015260408101516040830152606081015160608301526080810151608083015260a081015160a083015260c081015160c083015260e081015160e08301526101008082015181840152506101208082015181840152506101408082015181840152506101608082015181840152505050565b60006102a0820190506123e182855161234f565b602084015161018083015260408401516101a083015260608401516101c083015260808401516101e083015260a084015161020083015260c084015161022083015260e084015161024083015261010090930151610260820152610280015290565b6000806040838503121561245657600080fd5b505080516020909101519092909150565b60008060006060848603121561247c57600080fd5b8351925060208401519150604084015190509250925092565b600080600080608085870312156124ab57600080fd5b845193506020850151925060408501519150606085015180151581146124d057600080fd5b939692955090935050565b6101808101610210828461234f565b6000602082840312156124fc57600080fd5b5051919050565b634e487b7160e01b600052601160045260246000fd5b8181038181111561021057610210612503565b818103600083128015838313168383128216171561035c5761035c612503565b8082018082111561021057610210612503565b6000600160ff1b820161257457612574612503565b5060000390565b808201828112600083128015821682158216171561259b5761259b612503565b505092915050565b6000826125c057634e487b7160e01b600052601260045260246000fd5b600160ff1b8214600019841416156125da576125da612503565b50059056fea2646970667358221220e3e139048a616b3fecd42a96e9dbeebb994b8b279c7eac006caa236c11d64f6764736f6c63430008160033",
            [
                (
                    "contracts/src/libraries/LPMath.sol:LPMath",
                    libs.lp_math,
                )
            ],
            client.clone(),
        ).unwrap();
        let deployer = factory.deploy(constructor_args)?;
        let deployer = ::ethers::contract::ContractDeployer::new(deployer);
        Ok(deployer)
    }
}

