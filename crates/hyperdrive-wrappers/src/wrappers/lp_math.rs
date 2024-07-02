pub use lp_math::*;
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
pub mod lp_math {
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
                    ::std::borrow::ToOwned::to_owned("calculatePresentValueSafe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculatePresentValueSafe",
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
                    ::std::borrow::ToOwned::to_owned("calculateUpdateLiquiditySafe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateUpdateLiquiditySafe",
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
    pub static LPMATH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a!(a\0:`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14a\0-WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0aW`\x005`\xE0\x1C\x80c,\x03\xEFh\x14a\0fW\x80cZ\x1BA\x9E\x14a\0\x93W\x80c\xB2\x9E\x1D\x1E\x14a\0\xBBW\x80c\xBF:\xA1V\x14a\0\xE9W\x80c\xD0\xB4W\xCE\x14a\x01\nW[`\0\x80\xFD[a\0ya\0t6`\x04a\x1E\xDCV[a\x01?V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA6a\0\xA16`\x04a\x1F{V[a\x021V[`@\x80Q\x92\x83R\x90\x15\x15` \x83\x01R\x01a\0\x8AV[a\0\xCEa\0\xC96`\x04a\x1F\x98V[a\x02\xBEV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\x8AV[a\0\xFCa\0\xF76`\x04a\x1F{V[a\x03tV[`@Q\x90\x81R` \x01a\0\x8AV[a\x01\x1Da\x01\x186`\x04a\x1F\xDBV[a\x03\xABV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R\x15\x15``\x82\x01R`\x80\x01a\0\x8AV[`\0\x80`\0\x80a\x01W\x86`\xC0\x01Q\x87`\xE0\x01Qa\x04\xC8V[\x91P\x91P\x80a\x01nW`\0\x80\x93P\x93PPPa\x02*V[`\0a\x01z\x87\x84a\x05\x04V[\x92P\x90P\x81\x15\x80a\x01\x89WP\x80\x15[\x15a\x01\x9DW`\0\x80\x94P\x94PPPPa\x02*V[`\0a\x01\xA9\x88\x83a\x06\x13V[\x90P\x80`\0\x03a\x01\xC3W`\0\x80\x95P\x95PPPPPa\x02*V[\x87``\x01Q\x81\x11a\x01\xDAW\x94P\x92Pa\x02*\x91PPV[P``\x87\x01Q`\0a\x01\xEE\x89\x86\x85\x8Ba\x06\xEBV[\x90P\x80`\0\x03a\x02\tW`\0\x80\x96P\x96PPPPPPa\x02*V[\x82\x81\x10a\x02!W`\0\x80\x96P\x96PPPPPPa\x02*V[\x90\x95P\x93PPPP[\x92P\x92\x90PV[`\0\x80`\0\x80`\0a\x02B\x86a\x0C\x08V[\x91P\x91P\x80a\x02YWP`\0\x95\x86\x95P\x93PPPPV[a\x02f\x86`\xA0\x01Qa\x0F7V[a\x02o\x87a\x0FeV[\x83a\x02}\x89`\0\x01Qa\x0F7V[a\x02\x87\x91\x90a ,V[a\x02\x91\x91\x90a ,V[a\x02\x9B\x91\x90a TV[\x92PPP`\0\x81\x12\x15a\x02\xB4WP`\0\x93\x84\x93P\x91PPV[\x93`\x01\x93P\x91PPV[`\0\x80\x80\x80a\x02\xD1\x86c\x01\xE13\x80a\x0F\xD3V[\x90P`\0a\x03\x03a\x02\xE2\x89\x84a\x0F\xEFV[a\x02\xF4\x90g\r\xE0\xB6\xB3\xA7d\0\0a tV[g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x10\x04V[\x8B\x95P\x90Pa\x03Wa\x03\x15\x8B\x87a\x0F\xEFV[a\x03\x1F\x8B\x84a\x10\x19V[a\x03Ea\x03>a\x037g\r\xE0\xB6\xB3\xA7d\0\0\x8Ca\x0F\xD3V[\x86\x90a\x10.V[\x8E\x90a\x10\x19V[a\x03O\x91\x90a tV[\x8B\x91\x90a\x10\x99V[\x92Pa\x03d\x83\x82\x8Ca\x10\x99V[\x93PPP\x96P\x96P\x96\x93PPPPV[`\0\x80`\0a\x03\x82\x84a\x021V[\x91P\x91P\x80a\x03\xA4W`@QcU\x162\x8B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x92\x91PPV[`\0\x80`\0\x80\x84`\0\x03a\x03\xCAWP\x87\x92P\x86\x91P\x85\x90P`\x01a\x04\xBCV[`\0\x85a\x03\xD6\x8Ba\x0F7V[a\x03\xE0\x91\x90a ,V[\x90Pa\x03\xEB\x87a\x0F7V[\x81\x12\x15a\x04\x06W`\0\x80`\0\x80\x94P\x94P\x94P\x94PPa\x04\xBCV[\x80\x94P`\0\x89\x12a\x04+Wa\x04$a\x04\x1F\x86\x8B\x8Da\x10\x99V[a\x0F7V[\x93Pa\x04NV[a\x04Ba\x04\x1Fa\x04:\x8Ba \x87V[\x87\x90\x8Da\x10\xB7V[a\x04K\x90a \x87V[\x93P[`\0\x80a\x04[\x8C\x8Ca\x04\xC8V[\x91P\x91P\x80a\x04zW`\0\x80`\0\x80\x96P\x96P\x96P\x96PPPPa\x04\xBCV[`\0a\x04\x86\x88\x88a\x04\xC8V[\x92P\x90P\x81a\x04\xA6W`\0\x80`\0\x80\x97P\x97P\x97P\x97PPPPPa\x04\xBCV[a\x04\xB1\x8B\x82\x85a\x10\x99V[\x95P`\x01\x94PPPPP[\x95P\x95P\x95P\x95\x91PPV[`\0\x80`\0\x83a\x04\xD7\x86a\x0F7V[a\x04\xE1\x91\x90a TV[\x90P`\0\x81\x12\x15a\x04\xF9W`\0\x80\x92P\x92PPa\x02*V[\x94`\x01\x94P\x92PPPV[`\0\x80`\0\x84`\xA0\x01Q\x12a\x05!WPP`\x80\x82\x01Q`\x01a\x02*V[`\0\x84`\xA0\x01Qa\x051\x90a \x87V[\x90P`\0a\x05m\x85\x87a\x01\0\x01Q\x88`\0\x01Q`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x05[\x91\x90a \xA3V[\x89Q``\x81\x01Q`\x80\x90\x91\x01Qa\x10\xDDV[\x93P\x90P\x82\x15\x80a\x05|WP\x80\x15[\x15a\x05\x8FW`\0\x80\x93P\x93PPPa\x02*V[`\0a\x05\x9B\x83\x83a\x10\x04V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x11a\x05\xD7W\x80g\r\xE0\xB6\xB3\xA7d\0\0\x03\x94Pa\x05\xD0\x87`\xC0\x01Q\x86a\x0F\xEF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94Pa\x05\xE6V[`\0\x80\x94P\x94PPPPa\x02*V[\x86`\x80\x01Q\x85\x11\x15a\x06\x05W\x86`\x80\x01Q`\x01\x94P\x94PPPPa\x02*V[P`\x01\x92PPP\x92P\x92\x90PV[`\0\x80a\x06D\x84`\xC0\x01Q\x85`\xE0\x01Q\x86a\x01\0\x01Q\x87`\0\x01Q`\xA0\x01Qa\x06;\x88a\x0F7V[a\x01\x18\x90a \x87V[\x87Q`@\x81\x01\x92\x90\x92R` \x82\x01\x92\x90\x92R\x91\x90\x91R\x90P\x80a\x06kW`\0\x91PPa\x06\xE5V[`\0a\x06z\x85`\0\x01Qa\x021V[\x92P\x90P\x81a\x06\x8EW`\0\x92PPPa\x06\xE5V[\x84` \x01Q\x81\x10a\x06\xA4W`\0\x92PPPa\x06\xE5V[`\0\x85``\x01Q\x86`@\x01Qa\x06\xBA\x91\x90a tV[\x90Pa\x06\xD5\x82\x87` \x01Q\x83a\x10\x99\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x06\xDF\x90\x82a \xA3V[\x93PPPP[\x92\x91PPV[`\0\x80\x85``\x01Q\x86`@\x01Qa\x07\x02\x91\x90a tV[` \x87\x01Q``\x88\x01Q\x91\x92P`\0\x91a\x07\x1C\x91\x84a\x10\x99V[\x90P\x86`\xA0\x01Q`\0\x03a\x073W\x91Pa\x0C\0\x90PV[`\0\x80\x80\x89`\x04\x88\x10\x15a\x07FW`\x04\x97P[`\0[\x88\x81\x10\x15a\n\xA8Wa\x07[\x86\x8Ba\x11\x90V[\x95P`\0a\x07\x84\x83`\xC0\x01Q\x84`\xE0\x01Q\x85a\x01\0\x01Q\x86`\0\x01Q`\xA0\x01Qa\x06;\x8Ca\x0F7V[\x86Q`@\x81\x01\x92\x90\x92R` \x82\x01\x92\x90\x92R\x91\x90\x91R\x90P\x80a\x07\xB2W`\0\x98PPPPPPPPPa\x0C\0V[`\0a\x07\xC1\x8E`\0\x01Qa\x021V[\x92P\x90P\x81a\x07\xDCW`\0\x99PPPPPPPPPPa\x0C\0V[a\x07\xE7\x84\x82\x8Ba\x11\xA5V[\x15a\x07\xFDW\x87\x99PPPPPPPPPPa\x0C\0V[`\0\x84`\xA0\x01Q\x13\x15a\tfW\x83Q\x80Q` \x82\x01Q`@\x83\x01Q`\xA0\x84\x01Q`\xE0\x90\x94\x01Q`\0\x94a\x08Q\x94\x93\x92\x91a\x08?\x90g\r\xE0\xB6\xB3\xA7d\0\0a \xA3V[\x8AQ``\x81\x01Q`\x80\x90\x91\x01Qa\x12\x11V[\x93P\x90P\x82a\x08mW`\0\x9APPPPPPPPPPPa\x0C\0V[\x80\x85`\xA0\x01Q\x10a\tdWa\x08\x81\x85a\x13\x1CV[\x90\x99P\x92P\x82a\x08\x9EW`\0\x9APPPPPPPPPPPa\x0C\0V[a\x08\xC3\x85`\xC0\x01Q\x86`\xE0\x01Q\x87a\x01\0\x01Q\x88`\0\x01Q`\xA0\x01Qa\x06;\x8Ea\x0F7V[\x88Q`@\x81\x01\x92\x90\x92R` \x82\x01\x92\x90\x92R\x91\x90\x91R\x92P\x82a\x08\xF3W`\0\x9APPPPPPPPPPPa\x0C\0V[\x84Q\x80Q` \x82\x01Q`@\x83\x01Q`\xA0\x84\x01Q`\xE0\x90\x94\x01Qa\t#\x94\x90a\x08?\x90g\r\xE0\xB6\xB3\xA7d\0\0a \xA3V[\x93P\x90P\x82a\t?W`\0\x9APPPPPPPPPPPa\x0C\0V[\x8E`\xA0\x01Q\x81\x11a\t\\W\x88\x9APPPPPPPPPPPa\x0C\0V[PPPa\x07IV[P[`\0a\tw\x85\x8F\x87`\xA0\x01Qa\x13\xF2V[\x93P\x90P\x82\x15\x80a\t\x90WPg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10\x15[\x15a\t\xA8W`\0\x9APPPPPPPPPPPa\x0C\0V[\x80g\r\xE0\xB6\xB3\xA7d\0\0\x03\x90P`\0a\t\xD5a\x04\x1F\x87`@\x01Q\x88` \x01Qa\x10\x19\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\t\xE2a\x04\x1F\x85\x8Ea\x0F\xEFV[a\t\xEC\x91\x90a TV[\x90P\x88\x15\x80a\n\nWPa\t\xFF\x89a\x17PV[a\n\x08\x82a\x17PV[\x10[\x15a\n\x19W\x80\x98P\x89\x97P\x82\x96P[`\0\x81\x13\x15a\nGWa\n6\x8Ba\n0\x83\x85a\x0F\xD3V[\x90a\x0F\xD3V[a\n@\x90\x8Ba tV[\x99Pa\n\x99V[`\0\x81\x12\x15a\n\x90W`\0a\na\x8Ca\n0\x85\x81\x86a \x87V[\x90P\x8A\x81\x10\x15a\nuW\x80\x8B\x03\x9APa\n\x8AV[`\0\x9CPPPPPPPPPPPPPa\x0C\0V[Pa\n\x99V[PPPPa\n\xA8V[\x84`\x01\x01\x94PPPPPa\x07IV[P`\0a\n\xD0\x82`\xC0\x01Q\x83`\xE0\x01Q\x84a\x01\0\x01Q\x85`\0\x01Q`\xA0\x01Qa\x06;\x8Ba\x0F7V[\x85Q`@\x81\x01\x92\x90\x92R` \x82\x01\x92\x90\x92R\x91\x90\x91R\x90P\x80a\n\xFDW`\0\x97PPPPPPPPa\x0C\0V[`\0a\x0B\x0C\x83`\0\x01Qa\x03tV[\x90P`\0a\x0B.a\x04\x1F\x85`@\x01Q\x86` \x01Qa\x10\x19\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0B;a\x04\x1F\x84\x8Ca\x0F\xEFV[a\x0BE\x91\x90a TV[\x90Pa\x0BP\x87a\x17PV[a\x0BY\x82a\x17PV[\x10\x15a\x0BfW\x87\x95P\x81\x94P[a\x0B\x8Da\x0B\x81eZ\xF3\x10z@\0g\r\xE0\xB6\xB3\xA7d\0\0a \xA3V[` \x86\x01Q\x90\x8Ba\x10\xB7V[`@\x85\x01Qa\x0B\x9D\x90\x87\x90a\x0F\xD3V[\x10\x80a\x0B\xDDWPa\x0B\xCBa\x0B\xBFeZ\xF3\x10z@\0g\r\xE0\xB6\xB3\xA7d\0\0a tV[` \x86\x01Q\x90\x8Ba\x10\x99V[`@\x85\x01Qa\x0B\xDB\x90\x87\x90a\x10\x04V[\x11[\x15a\x0B\xF4W`\0\x99PPPPPPPPPPa\x0C\0V[P\x93\x97PPPPPPPP[\x94\x93PPPPV[`\0\x80`\0a\x0C-a\x04\x1F\x85a\x01`\x01Q\x86a\x01@\x01Qa\x0F\xEF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0CMa\x04\x1F\x86a\x01 \x01Q\x87a\x01\0\x01Qa\x10\x19\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0CW\x91\x90a TV[\x90P`\0\x80a\x0Cn\x86`\0\x01Q\x87` \x01Qa\x04\xC8V[\x91P\x91P\x80a\x0C\x85WP`\0\x95\x86\x95P\x93PPPPV[`\0\x83\x13\x15a\r\xD5W`\0\x83\x90P`\0a\x0C\xD3\x88`\0\x01Q\x89` \x01Q\x8A`@\x01Q\x8B`\xA0\x01Q\x8C`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0C\xC4\x91\x90a \xA3V[\x8D``\x01Q\x8E`\x80\x01Qa\x12\x11V[\x93P\x90P\x82a\x0C\xECWP`\0\x97\x88\x97P\x95PPPPPPV[\x81\x81\x10a\r\x8AW`\0a\r&\x85\x8A`@\x01Q\x85\x8C`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\r\x17\x91\x90a \xA3V[\x8D``\x01Q\x8E`\x80\x01Qa\x17cV[\x94P\x90P\x83\x15\x80\x15a\r;WP\x88`\xC0\x01Q\x83\x10[\x15a\rRWP`\0\x98`\x01\x98P\x96PPPPPPPV[\x83a\rhWP`\0\x98\x89\x98P\x96PPPPPPPV[a\rq\x81a\x0F7V[a\rz\x90a \x87V[\x99`\x01\x99P\x97PPPPPPPPV[`\0\x88` \x01Q\x12a\r\xC1Wa\r\xA9\x88`\xA0\x01Q\x85a\x04\x1F\x91\x90a \xA3V[a\r\xB2\x90a \x87V[\x98`\x01\x98P\x96PPPPPPPV[`\xA0\x88\x01Q\x88Qa\r\xA9\x91a\x04\x1F\x91a \xA3V[`\0\x83\x12\x15a\x0F(W`\0a\r\xE9\x84a \x87V[\x90P`\0a\x0E\x1D\x84\x89`@\x01Q\x8A`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0E\x0E\x91\x90a \xA3V[\x8B``\x01Q\x8C`\x80\x01Qa\x10\xDDV[\x93P\x90P\x82a\x0E6WP`\0\x97\x88\x97P\x95PPPPPPV[\x81\x81\x10a\x0E\xBBW`\0a\x0Ep\x85\x8A`@\x01Q\x85\x8C`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0Ea\x91\x90a \xA3V[\x8D``\x01Q\x8E`\x80\x01Qa\x18&V[\x94P\x90P\x83\x15\x80\x15a\x0E\x85WP\x88`\xC0\x01Q\x83\x10[\x15a\x0E\x9CWP`\0\x98`\x01\x98P\x96PPPPPPPV[\x83a\x0E\xB2WP`\0\x98\x89\x98P\x96PPPPPPPV[a\rz\x81a\x0F7V[`\0a\x0E\xED\x85\x8A`@\x01Q\x8B`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0E\xDE\x91\x90a \xA3V[\x8C``\x01Q\x8D`\x80\x01Qa\x18\xFDV[\x94P\x90P\x83a\x0F\x07WP`\0\x98\x89\x98P\x96PPPPPPPV[a\rza\x0F\x1E\x8A``\x01Q\x84\x86a\n0\x91\x90a \xA3V[a\x04\x1F\x90\x83a tV[P`\0\x95`\x01\x95P\x93PPPPV[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x0FaW`@Qc9n\xA7\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0a\x0F\x98a\x04\x1F\x83a\x01 \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0F\x86\x91\x90a \xA3V[``\x85\x01Qa\x01\0\x86\x01Q\x91\x90a\x10\xB7V[a\x0F\xC9a\x04\x1F\x84a\x01`\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xB7\x91\x90a \xA3V[``\x86\x01Qa\x01@\x87\x01Q\x91\x90a\x10\x99V[a\x06\xE5\x91\x90a TV[`\0a\x0F\xE8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\x99V[\x93\x92PPPV[`\0a\x0F\xE8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\x99V[`\0a\x0F\xE8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\xB7V[`\0a\x0F\xE8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\xB7V[`\0\x81`\0\x03a\x10GWPg\r\xE0\xB6\xB3\xA7d\0\0a\x06\xE5V[\x82`\0\x03a\x10WWP`\0a\x06\xE5V[`\0a\x10b\x83a\x0F7V[\x90P`\0a\x10wa\x10r\x86a\x0F7V[a\x19\xB3V[\x90P\x81\x81\x02a\x10\x8Eg\r\xE0\xB6\xB3\xA7d\0\0\x82a \xB6V[\x90Pa\x06\xDF\x81a\x1B\xE2V[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x10\xB0W`\0\x80\xFD[P\x91\x02\x04\x90V[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x10\xCEW`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`\0\x80`\0a\x10\xEF\x88\x88\x88\x88\x88a\x1D\x81V[\x90P`\0a\x11\x1Ag\r\xE0\xB6\xB3\xA7d\0\0a\x11\t\x88\x88a\x0F\xD3V[a\x11\x13\x91\x90a tV[\x83\x90a\x10\x04V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x11NWa\x11Ga\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\x04V[\x82\x90a\x10.V[\x90Pa\x11fV[a\x11ca\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x0F\xD3V[\x90P[\x80\x88\x10\x15a\x11|W`\0\x80\x93P\x93PPPa\x11\x86V[\x87\x03\x92P`\x01\x91PP[\x95P\x95\x93PPPPV[`\0\x81\x83\x11a\x11\x9FW\x82a\x0F\xE8V[P\x91\x90PV[` \x83\x01Q`\0\x90a\x11\xB7\x90\x84a\x10\x04V[`@\x85\x01Qa\x11\xC7\x90\x84\x90a\x0F\xD3V[\x10\x15\x80\x15a\x0C\0WP` \x84\x01Qa\x11\xF7\x90\x84a\x11\xF0c;\x9A\xCA\0g\r\xE0\xB6\xB3\xA7d\0\0a tV[\x91\x90a\x10\x99V[`@\x85\x01Qa\x12\x07\x90\x84\x90a\x10\x04V[\x11\x15\x94\x93PPPPV[`\0\x80`\0\x88\x12\x15a\x123Wa\x12&\x88a \x87V[a\x120\x90\x87a tV[\x95P[`\0\x80a\x12@\x8B\x8Ba\x04\xC8V[\x91P\x91P\x80a\x12WW`\0\x80\x93P\x93PPPa\x13\x10V[`\0a\x12f\x83\x8B\x8A\x8A\x8Aa\x1D\xB0V[\x90P`\0a\x12\x89a\x12\x81\x8Aa\x12{\x8A\x8Ea\x10\x19V[\x90a\x10.V[\x89\x90\x89a\x10\xB7V[\x90P\x80\x82\x10\x15a\x12\xA3W`\0\x80\x95P\x95PPPPPa\x13\x10V[\x80\x82\x03g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x12\xD1Wa\x12\xCAa\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x8Ca\x0F\xD3V[\x90Pa\x12\xE9V[a\x12\xE6a\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x8Ca\x10\x04V[\x90P[\x8B\x81\x10\x15a\x13\x02W`\0\x80\x96P\x96PPPPPPa\x13\x10V[\x8B\x90\x03\x95P`\x01\x94PPPPP[\x97P\x97\x95PPPPPPV[`\0\x80`\0\x83`\xE0\x01Q\x13a\x136WP`\0\x92\x83\x92P\x90PV[`\0a\x13E\x84`\0\x01Qa\x0FeV[\x90P`\0a\x13q\x85`@\x01Q\x86``\x01Q\x87`@\x01Qa\x13e\x91\x90a tV[` \x88\x01Q\x91\x90a\x10\xB7V[\x90P`\0\x82\x12a\x13\x98W\x80\x82\x10\x15a\x13\x8BW\x81\x90\x03a\x13\xAEV[P`\0\x94\x85\x94P\x92PPPV[a\x13\xA1\x82a \x87V[a\x13\xAB\x90\x82a tV[\x90P[`\xE0\x85\x01Q`\xC0\x86\x01Qa\x13\xC3\x91\x83\x90a\x10\xB7V[\x90P\x80\x85`\xC0\x01Q\x10\x15a\x13\xDEWP`\0\x94\x85\x94P\x92PPPV[\x80\x85`\xC0\x01Q\x03`\x01\x93P\x93PPP\x91P\x91V[`\0\x80`\0\x80\x84\x12a\x14\x16W\x85Q`@\x01Qa\x14\x0F\x90\x85\x90a tV[\x90Pa\x14QV[`\0a\x14!\x85a \x87V[\x87Q`@\x01Q\x90\x91P\x81\x10\x15a\x14AW\x86Q`@\x01Q\x81\x90\x03\x91Pa\x14OV[`\0\x80\x93P\x93PPPa\x17HV[P[\x85Q\x80Q` \x90\x91\x01Q`\0\x91\x82\x91a\x14j\x91\x90a\x04\xC8V[\x91P\x91P\x80a\x14\x82W`\0\x80\x94P\x94PPPPa\x17HV[\x87Q`\xE0\x81\x01Q`@\x90\x91\x01Q`\0\x91a\x14\xB7\x91a\x14\xAB\x91a\x14\xA4\x91\x90a\x10.V[\x8A\x90a\x0F\xEFV[a\x01\0\x8B\x01Q\x90a\x10\x04V[\x89Q`\xE0\x81\x01Q`\x80\x90\x91\x01Qa\x14\xE2\x91a\x14\xD6\x91a\x12{\x90\x88a\x0F\xEFV[\x8BQ``\x01Q\x90a\x10\x04V[a\x14\xEC\x91\x90a tV[\x90P`\0a\x15$a\x15\x18a\x15\x11\x8C`\0\x01Q`\xE0\x01Q\x88a\x10.\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x8B\x90a\x10\x19V[a\x01\0\x8C\x01Q\x90a\x0F\xD3V[\x90P\x80\x82\x10\x15a\x15?W`\0\x80\x96P\x96PPPPPPa\x17HV[\x80\x82\x03\x91P`\0a\x15\x81\x85\x8C`\0\x01Q`@\x01Q\x8D`\0\x01Q`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x15o\x91\x90a \xA3V[\x8EQ``\x81\x01Q`\x80\x90\x91\x01Qa\x1D\x81V[\x90P`\0a\x15\xAB\x8C`\0\x01Q`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x15\xA4\x91\x90a \xA3V[\x88\x90a\x10.V[\x90P\x80\x82\x10\x15a\x15\xC8W`\0\x80\x98P\x98PPPPPPPPa\x17HV[\x8BQ`\x80\x81\x01Q``\x90\x91\x01Q\x91\x83\x03\x91a\x15\xE4\x91\x83\x91a\x10\xB7V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x16(W\x8BQ`\xE0\x01Qa\x16!\x90a\x11@\x90a\x16\x15\x90g\r\xE0\xB6\xB3\xA7d\0\0a \xA3V[\x8EQ`\xE0\x01Q\x90a\x10\x04V[\x90Pa\x16WV[\x8BQ`\xE0\x01Qa\x16T\x90a\x11@\x90a\x16H\x90g\r\xE0\xB6\xB3\xA7d\0\0a \xA3V[\x8EQ`\xE0\x01Q\x90a\x0F\xD3V[\x90P[\x8BQ``\x01Qa\x16j\x90\x85\x90\x83\x90a\x10\xB7V[\x93P\x83g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a\x16\x8EW\x83g\r\xE0\xB6\xB3\xA7d\0\0\x03\x93Pa\x16\xA2V[`\0`\x01\x98P\x98PPPPPPPPa\x17HV[`\0\x8C`\xE0\x01Q\x12a\x17\x05W`\xC0\x8C\x01Q`\xE0\x8D\x01Qa\x16\xC1\x91a\x10\x04V[\x92Pg\r\xE0\xB6\xB3\xA7d\0\0\x83\x11\x15a\x16\xE6W`\0\x80\x98P\x98PPPPPPPPa\x17HV[g\r\xE0\xB6\xB3\xA7d\0\0\x92\x90\x92\x03\x91a\x16\xFE\x84\x84a\x0F\xEFV[\x93Pa\x17:V[a\x177a\x17\x1E\x8D`\xC0\x01Q\x8E`\xE0\x01Qa\n0\x90a \x87V[a\x170\x90g\r\xE0\xB6\xB3\xA7d\0\0a tV[\x85\x90a\x0F\xEFV[\x93P[P\x91\x96P`\x01\x95PPPPPP[\x93P\x93\x91PPV[`\0\x80\x82\x12\x15a\x0FaW\x81`\0\x03a\x06\xE5V[`\0\x80`\0a\x17u\x89\x89\x88\x88\x88a\x1D\x81V[\x90Pa\x17\x85\x86a\x12{\x89\x8Ba tV[\x97P\x87\x81\x10\x15a\x17\x9CW`\0\x80\x92P\x92PPa\x18\x1BV[\x87\x81\x03a\x17\xAA\x81\x86\x88a\x10\xB7V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x17\xD7Wa\x17\xD0a\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\x04V[\x90Pa\x17\xEFV[a\x17\xECa\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x0F\xD3V[\x90P[a\x17\xF9\x81\x86a\x10\x04V[\x90P\x80\x8A\x10\x15a\x18\x11W`\0\x80\x93P\x93PPPa\x18\x1BV[\x89\x03\x92P`\x01\x91PP[\x96P\x96\x94PPPPPV[`\0\x80`\0a\x188\x89\x89\x88\x88\x88a\x1D\x81V[\x90P\x86\x88\x10\x15a\x18OW`\0\x80\x92P\x92PPa\x18\x1BV[\x96\x86\x90\x03\x96a\x18^\x88\x87a\x10.V[\x97P\x87\x81\x10\x15a\x18uW`\0\x80\x92P\x92PPa\x18\x1BV[\x87\x81\x03a\x18\x83\x81\x86\x88a\x10\xB7V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x18\xB0Wa\x18\xA9a\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\x04V[\x90Pa\x18\xC8V[a\x18\xC5a\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x0F\xD3V[\x90P[a\x18\xD2\x81\x86a\x10\x04V[\x90P\x89\x81\x10\x15a\x18\xEAW`\0\x80\x93P\x93PPPa\x18\x1BV[\x98\x90\x98\x03\x98`\x01\x98P\x96PPPPPPPV[`\0\x80`\0a\x19\x0F\x88\x88\x88\x88\x88a\x1D\xB0V[\x90P`\0a\x19:g\r\xE0\xB6\xB3\xA7d\0\0a\x19)\x88\x88a\x10\x04V[a\x193\x91\x90a tV[\x83\x90a\x0F\xD3V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x19gWa\x19`a\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x0F\xD3V[\x90Pa\x19\x7FV[a\x19|a\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\x04V[\x90P[a\x19\x89\x81\x86a\x0F\xD3V[\x90P\x88\x81\x10\x15a\x19\xA1W`\0\x80\x93P\x93PPPa\x11\x86V[\x97\x90\x97\x03\x97`\x01\x97P\x95PPPPPPV[`\0\x80\x82\x13a\x19\xD5W`@Qc\xE6\x1BIu`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x82\x81\x1C`\x0F\x10`\x02\x1B\x17\x82\x81\x1C\x90\x91\x10`\x01\x90\x81\x1B\x90\x91\x17\x82\x81\x1C\x90\x91\x10\x17`\x9F\x81\x81\x03``\x01\x92\x90\x92\x1B\x91`_\x19\x82\x01\x90a\x1Aa\x90\x84\x90\x1Ca\x0F7V[lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x91\x90\x91\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1B\xFDWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1C&W`@Qcs\xA2\xD6\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x92l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x84\x01\x84\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x85\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x85\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x85\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x85\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x87\x01\x87\x02\x83\x1D\x90\x81\x01\x90\x87\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x81\x02\x90\x92\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x86\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x81\x81\x05\x95P\x92\x93P\x90\x91\x90a\x1Dwt\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x86\x02`\xC3\x86\x90\x03\x1Ca\x0F7V[\x96\x95PPPPPPV[`\0a\x1D\x8D\x85\x85a\x10.V[a\x1D\xA6a\x1D\x9E\x86a\x12{\x86\x8Ba\x10\x19V[\x85\x90\x85a\x10\xB7V[a\x1Dw\x91\x90a tV[`\0a\x1D\xBC\x85\x85a\x10.V[a\x1D\xA6a\x1D\xCD\x86a\x12{\x86\x8Ba\x0F\xEFV[\x85\x90\x85a\x10\x99V[`@Qa\x01\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\x07WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@Qa\x01 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\x07WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0a\x01\x80\x82\x84\x03\x12\x15a\x1ERW`\0\x80\xFD[a\x1EZa\x1D\xD5V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015`\x80\x82\x01R`\xA0\x82\x015`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R`\xE0\x82\x015`\xE0\x82\x01Ra\x01\0\x80\x83\x015\x81\x83\x01RPa\x01 \x80\x83\x015\x81\x83\x01RPa\x01@\x80\x83\x015\x81\x83\x01RPa\x01`\x80\x83\x015\x81\x83\x01RP\x92\x91PPV[`\0\x80\x82\x84\x03a\x02\xA0\x81\x12\x15a\x1E\xF1W`\0\x80\xFD[a\x02\x80\x80\x82\x12\x15a\x1F\x01W`\0\x80\xFD[a\x1F\ta\x1E\rV[\x91Pa\x1F\x15\x86\x86a\x1E?V[\x82Ra\x01\x80\x85\x015` \x83\x01Ra\x01\xA0\x85\x015`@\x83\x01Ra\x01\xC0\x85\x015``\x83\x01Ra\x01\xE0\x85\x015`\x80\x83\x01Ra\x02\0\x85\x015`\xA0\x83\x01Ra\x02 \x85\x015`\xC0\x83\x01Ra\x02@\x85\x015`\xE0\x83\x01Ra\x02`\x85\x015a\x01\0\x83\x01R\x90\x95\x93\x015\x93PPPV[`\0a\x01\x80\x82\x84\x03\x12\x15a\x1F\x8EW`\0\x80\xFD[a\x0F\xE8\x83\x83a\x1E?V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x1F\xB1W`\0\x80\xFD[PP\x845\x96` \x86\x015\x96P`@\x86\x015\x95``\x81\x015\x95P`\x80\x81\x015\x94P`\xA0\x015\x92P\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1F\xF3W`\0\x80\xFD[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a LWa La \x16V[PP\x92\x91PPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x03\xA4Wa\x03\xA4a \x16V[\x80\x82\x01\x80\x82\x11\x15a\x06\xE5Wa\x06\xE5a \x16V[`\0`\x01`\xFF\x1B\x82\x01a \x9CWa \x9Ca \x16V[P`\0\x03\x90V[\x81\x81\x03\x81\x81\x11\x15a\x06\xE5Wa\x06\xE5a \x16V[`\0\x82a \xD3WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a \xEDWa \xEDa \x16V[P\x05\x90V\xFE\xA2dipfsX\"\x12 \x0F,g\xF9\xB2\xAC\x04L\xE4&A\x16\xBF/\x0C\x0F\xE6\x9A\xD4\xDFp&\x90#\xF3\xEDr\xE7&\x0C\xFE\xD7dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static LPMATH_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0aW`\x005`\xE0\x1C\x80c,\x03\xEFh\x14a\0fW\x80cZ\x1BA\x9E\x14a\0\x93W\x80c\xB2\x9E\x1D\x1E\x14a\0\xBBW\x80c\xBF:\xA1V\x14a\0\xE9W\x80c\xD0\xB4W\xCE\x14a\x01\nW[`\0\x80\xFD[a\0ya\0t6`\x04a\x1E\xDCV[a\x01?V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA6a\0\xA16`\x04a\x1F{V[a\x021V[`@\x80Q\x92\x83R\x90\x15\x15` \x83\x01R\x01a\0\x8AV[a\0\xCEa\0\xC96`\x04a\x1F\x98V[a\x02\xBEV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\x8AV[a\0\xFCa\0\xF76`\x04a\x1F{V[a\x03tV[`@Q\x90\x81R` \x01a\0\x8AV[a\x01\x1Da\x01\x186`\x04a\x1F\xDBV[a\x03\xABV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R\x15\x15``\x82\x01R`\x80\x01a\0\x8AV[`\0\x80`\0\x80a\x01W\x86`\xC0\x01Q\x87`\xE0\x01Qa\x04\xC8V[\x91P\x91P\x80a\x01nW`\0\x80\x93P\x93PPPa\x02*V[`\0a\x01z\x87\x84a\x05\x04V[\x92P\x90P\x81\x15\x80a\x01\x89WP\x80\x15[\x15a\x01\x9DW`\0\x80\x94P\x94PPPPa\x02*V[`\0a\x01\xA9\x88\x83a\x06\x13V[\x90P\x80`\0\x03a\x01\xC3W`\0\x80\x95P\x95PPPPPa\x02*V[\x87``\x01Q\x81\x11a\x01\xDAW\x94P\x92Pa\x02*\x91PPV[P``\x87\x01Q`\0a\x01\xEE\x89\x86\x85\x8Ba\x06\xEBV[\x90P\x80`\0\x03a\x02\tW`\0\x80\x96P\x96PPPPPPa\x02*V[\x82\x81\x10a\x02!W`\0\x80\x96P\x96PPPPPPa\x02*V[\x90\x95P\x93PPPP[\x92P\x92\x90PV[`\0\x80`\0\x80`\0a\x02B\x86a\x0C\x08V[\x91P\x91P\x80a\x02YWP`\0\x95\x86\x95P\x93PPPPV[a\x02f\x86`\xA0\x01Qa\x0F7V[a\x02o\x87a\x0FeV[\x83a\x02}\x89`\0\x01Qa\x0F7V[a\x02\x87\x91\x90a ,V[a\x02\x91\x91\x90a ,V[a\x02\x9B\x91\x90a TV[\x92PPP`\0\x81\x12\x15a\x02\xB4WP`\0\x93\x84\x93P\x91PPV[\x93`\x01\x93P\x91PPV[`\0\x80\x80\x80a\x02\xD1\x86c\x01\xE13\x80a\x0F\xD3V[\x90P`\0a\x03\x03a\x02\xE2\x89\x84a\x0F\xEFV[a\x02\xF4\x90g\r\xE0\xB6\xB3\xA7d\0\0a tV[g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x10\x04V[\x8B\x95P\x90Pa\x03Wa\x03\x15\x8B\x87a\x0F\xEFV[a\x03\x1F\x8B\x84a\x10\x19V[a\x03Ea\x03>a\x037g\r\xE0\xB6\xB3\xA7d\0\0\x8Ca\x0F\xD3V[\x86\x90a\x10.V[\x8E\x90a\x10\x19V[a\x03O\x91\x90a tV[\x8B\x91\x90a\x10\x99V[\x92Pa\x03d\x83\x82\x8Ca\x10\x99V[\x93PPP\x96P\x96P\x96\x93PPPPV[`\0\x80`\0a\x03\x82\x84a\x021V[\x91P\x91P\x80a\x03\xA4W`@QcU\x162\x8B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x92\x91PPV[`\0\x80`\0\x80\x84`\0\x03a\x03\xCAWP\x87\x92P\x86\x91P\x85\x90P`\x01a\x04\xBCV[`\0\x85a\x03\xD6\x8Ba\x0F7V[a\x03\xE0\x91\x90a ,V[\x90Pa\x03\xEB\x87a\x0F7V[\x81\x12\x15a\x04\x06W`\0\x80`\0\x80\x94P\x94P\x94P\x94PPa\x04\xBCV[\x80\x94P`\0\x89\x12a\x04+Wa\x04$a\x04\x1F\x86\x8B\x8Da\x10\x99V[a\x0F7V[\x93Pa\x04NV[a\x04Ba\x04\x1Fa\x04:\x8Ba \x87V[\x87\x90\x8Da\x10\xB7V[a\x04K\x90a \x87V[\x93P[`\0\x80a\x04[\x8C\x8Ca\x04\xC8V[\x91P\x91P\x80a\x04zW`\0\x80`\0\x80\x96P\x96P\x96P\x96PPPPa\x04\xBCV[`\0a\x04\x86\x88\x88a\x04\xC8V[\x92P\x90P\x81a\x04\xA6W`\0\x80`\0\x80\x97P\x97P\x97P\x97PPPPPa\x04\xBCV[a\x04\xB1\x8B\x82\x85a\x10\x99V[\x95P`\x01\x94PPPPP[\x95P\x95P\x95P\x95\x91PPV[`\0\x80`\0\x83a\x04\xD7\x86a\x0F7V[a\x04\xE1\x91\x90a TV[\x90P`\0\x81\x12\x15a\x04\xF9W`\0\x80\x92P\x92PPa\x02*V[\x94`\x01\x94P\x92PPPV[`\0\x80`\0\x84`\xA0\x01Q\x12a\x05!WPP`\x80\x82\x01Q`\x01a\x02*V[`\0\x84`\xA0\x01Qa\x051\x90a \x87V[\x90P`\0a\x05m\x85\x87a\x01\0\x01Q\x88`\0\x01Q`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x05[\x91\x90a \xA3V[\x89Q``\x81\x01Q`\x80\x90\x91\x01Qa\x10\xDDV[\x93P\x90P\x82\x15\x80a\x05|WP\x80\x15[\x15a\x05\x8FW`\0\x80\x93P\x93PPPa\x02*V[`\0a\x05\x9B\x83\x83a\x10\x04V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x11a\x05\xD7W\x80g\r\xE0\xB6\xB3\xA7d\0\0\x03\x94Pa\x05\xD0\x87`\xC0\x01Q\x86a\x0F\xEF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94Pa\x05\xE6V[`\0\x80\x94P\x94PPPPa\x02*V[\x86`\x80\x01Q\x85\x11\x15a\x06\x05W\x86`\x80\x01Q`\x01\x94P\x94PPPPa\x02*V[P`\x01\x92PPP\x92P\x92\x90PV[`\0\x80a\x06D\x84`\xC0\x01Q\x85`\xE0\x01Q\x86a\x01\0\x01Q\x87`\0\x01Q`\xA0\x01Qa\x06;\x88a\x0F7V[a\x01\x18\x90a \x87V[\x87Q`@\x81\x01\x92\x90\x92R` \x82\x01\x92\x90\x92R\x91\x90\x91R\x90P\x80a\x06kW`\0\x91PPa\x06\xE5V[`\0a\x06z\x85`\0\x01Qa\x021V[\x92P\x90P\x81a\x06\x8EW`\0\x92PPPa\x06\xE5V[\x84` \x01Q\x81\x10a\x06\xA4W`\0\x92PPPa\x06\xE5V[`\0\x85``\x01Q\x86`@\x01Qa\x06\xBA\x91\x90a tV[\x90Pa\x06\xD5\x82\x87` \x01Q\x83a\x10\x99\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x06\xDF\x90\x82a \xA3V[\x93PPPP[\x92\x91PPV[`\0\x80\x85``\x01Q\x86`@\x01Qa\x07\x02\x91\x90a tV[` \x87\x01Q``\x88\x01Q\x91\x92P`\0\x91a\x07\x1C\x91\x84a\x10\x99V[\x90P\x86`\xA0\x01Q`\0\x03a\x073W\x91Pa\x0C\0\x90PV[`\0\x80\x80\x89`\x04\x88\x10\x15a\x07FW`\x04\x97P[`\0[\x88\x81\x10\x15a\n\xA8Wa\x07[\x86\x8Ba\x11\x90V[\x95P`\0a\x07\x84\x83`\xC0\x01Q\x84`\xE0\x01Q\x85a\x01\0\x01Q\x86`\0\x01Q`\xA0\x01Qa\x06;\x8Ca\x0F7V[\x86Q`@\x81\x01\x92\x90\x92R` \x82\x01\x92\x90\x92R\x91\x90\x91R\x90P\x80a\x07\xB2W`\0\x98PPPPPPPPPa\x0C\0V[`\0a\x07\xC1\x8E`\0\x01Qa\x021V[\x92P\x90P\x81a\x07\xDCW`\0\x99PPPPPPPPPPa\x0C\0V[a\x07\xE7\x84\x82\x8Ba\x11\xA5V[\x15a\x07\xFDW\x87\x99PPPPPPPPPPa\x0C\0V[`\0\x84`\xA0\x01Q\x13\x15a\tfW\x83Q\x80Q` \x82\x01Q`@\x83\x01Q`\xA0\x84\x01Q`\xE0\x90\x94\x01Q`\0\x94a\x08Q\x94\x93\x92\x91a\x08?\x90g\r\xE0\xB6\xB3\xA7d\0\0a \xA3V[\x8AQ``\x81\x01Q`\x80\x90\x91\x01Qa\x12\x11V[\x93P\x90P\x82a\x08mW`\0\x9APPPPPPPPPPPa\x0C\0V[\x80\x85`\xA0\x01Q\x10a\tdWa\x08\x81\x85a\x13\x1CV[\x90\x99P\x92P\x82a\x08\x9EW`\0\x9APPPPPPPPPPPa\x0C\0V[a\x08\xC3\x85`\xC0\x01Q\x86`\xE0\x01Q\x87a\x01\0\x01Q\x88`\0\x01Q`\xA0\x01Qa\x06;\x8Ea\x0F7V[\x88Q`@\x81\x01\x92\x90\x92R` \x82\x01\x92\x90\x92R\x91\x90\x91R\x92P\x82a\x08\xF3W`\0\x9APPPPPPPPPPPa\x0C\0V[\x84Q\x80Q` \x82\x01Q`@\x83\x01Q`\xA0\x84\x01Q`\xE0\x90\x94\x01Qa\t#\x94\x90a\x08?\x90g\r\xE0\xB6\xB3\xA7d\0\0a \xA3V[\x93P\x90P\x82a\t?W`\0\x9APPPPPPPPPPPa\x0C\0V[\x8E`\xA0\x01Q\x81\x11a\t\\W\x88\x9APPPPPPPPPPPa\x0C\0V[PPPa\x07IV[P[`\0a\tw\x85\x8F\x87`\xA0\x01Qa\x13\xF2V[\x93P\x90P\x82\x15\x80a\t\x90WPg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10\x15[\x15a\t\xA8W`\0\x9APPPPPPPPPPPa\x0C\0V[\x80g\r\xE0\xB6\xB3\xA7d\0\0\x03\x90P`\0a\t\xD5a\x04\x1F\x87`@\x01Q\x88` \x01Qa\x10\x19\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\t\xE2a\x04\x1F\x85\x8Ea\x0F\xEFV[a\t\xEC\x91\x90a TV[\x90P\x88\x15\x80a\n\nWPa\t\xFF\x89a\x17PV[a\n\x08\x82a\x17PV[\x10[\x15a\n\x19W\x80\x98P\x89\x97P\x82\x96P[`\0\x81\x13\x15a\nGWa\n6\x8Ba\n0\x83\x85a\x0F\xD3V[\x90a\x0F\xD3V[a\n@\x90\x8Ba tV[\x99Pa\n\x99V[`\0\x81\x12\x15a\n\x90W`\0a\na\x8Ca\n0\x85\x81\x86a \x87V[\x90P\x8A\x81\x10\x15a\nuW\x80\x8B\x03\x9APa\n\x8AV[`\0\x9CPPPPPPPPPPPPPa\x0C\0V[Pa\n\x99V[PPPPa\n\xA8V[\x84`\x01\x01\x94PPPPPa\x07IV[P`\0a\n\xD0\x82`\xC0\x01Q\x83`\xE0\x01Q\x84a\x01\0\x01Q\x85`\0\x01Q`\xA0\x01Qa\x06;\x8Ba\x0F7V[\x85Q`@\x81\x01\x92\x90\x92R` \x82\x01\x92\x90\x92R\x91\x90\x91R\x90P\x80a\n\xFDW`\0\x97PPPPPPPPa\x0C\0V[`\0a\x0B\x0C\x83`\0\x01Qa\x03tV[\x90P`\0a\x0B.a\x04\x1F\x85`@\x01Q\x86` \x01Qa\x10\x19\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0B;a\x04\x1F\x84\x8Ca\x0F\xEFV[a\x0BE\x91\x90a TV[\x90Pa\x0BP\x87a\x17PV[a\x0BY\x82a\x17PV[\x10\x15a\x0BfW\x87\x95P\x81\x94P[a\x0B\x8Da\x0B\x81eZ\xF3\x10z@\0g\r\xE0\xB6\xB3\xA7d\0\0a \xA3V[` \x86\x01Q\x90\x8Ba\x10\xB7V[`@\x85\x01Qa\x0B\x9D\x90\x87\x90a\x0F\xD3V[\x10\x80a\x0B\xDDWPa\x0B\xCBa\x0B\xBFeZ\xF3\x10z@\0g\r\xE0\xB6\xB3\xA7d\0\0a tV[` \x86\x01Q\x90\x8Ba\x10\x99V[`@\x85\x01Qa\x0B\xDB\x90\x87\x90a\x10\x04V[\x11[\x15a\x0B\xF4W`\0\x99PPPPPPPPPPa\x0C\0V[P\x93\x97PPPPPPPP[\x94\x93PPPPV[`\0\x80`\0a\x0C-a\x04\x1F\x85a\x01`\x01Q\x86a\x01@\x01Qa\x0F\xEF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0CMa\x04\x1F\x86a\x01 \x01Q\x87a\x01\0\x01Qa\x10\x19\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0CW\x91\x90a TV[\x90P`\0\x80a\x0Cn\x86`\0\x01Q\x87` \x01Qa\x04\xC8V[\x91P\x91P\x80a\x0C\x85WP`\0\x95\x86\x95P\x93PPPPV[`\0\x83\x13\x15a\r\xD5W`\0\x83\x90P`\0a\x0C\xD3\x88`\0\x01Q\x89` \x01Q\x8A`@\x01Q\x8B`\xA0\x01Q\x8C`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0C\xC4\x91\x90a \xA3V[\x8D``\x01Q\x8E`\x80\x01Qa\x12\x11V[\x93P\x90P\x82a\x0C\xECWP`\0\x97\x88\x97P\x95PPPPPPV[\x81\x81\x10a\r\x8AW`\0a\r&\x85\x8A`@\x01Q\x85\x8C`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\r\x17\x91\x90a \xA3V[\x8D``\x01Q\x8E`\x80\x01Qa\x17cV[\x94P\x90P\x83\x15\x80\x15a\r;WP\x88`\xC0\x01Q\x83\x10[\x15a\rRWP`\0\x98`\x01\x98P\x96PPPPPPPV[\x83a\rhWP`\0\x98\x89\x98P\x96PPPPPPPV[a\rq\x81a\x0F7V[a\rz\x90a \x87V[\x99`\x01\x99P\x97PPPPPPPPV[`\0\x88` \x01Q\x12a\r\xC1Wa\r\xA9\x88`\xA0\x01Q\x85a\x04\x1F\x91\x90a \xA3V[a\r\xB2\x90a \x87V[\x98`\x01\x98P\x96PPPPPPPV[`\xA0\x88\x01Q\x88Qa\r\xA9\x91a\x04\x1F\x91a \xA3V[`\0\x83\x12\x15a\x0F(W`\0a\r\xE9\x84a \x87V[\x90P`\0a\x0E\x1D\x84\x89`@\x01Q\x8A`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0E\x0E\x91\x90a \xA3V[\x8B``\x01Q\x8C`\x80\x01Qa\x10\xDDV[\x93P\x90P\x82a\x0E6WP`\0\x97\x88\x97P\x95PPPPPPV[\x81\x81\x10a\x0E\xBBW`\0a\x0Ep\x85\x8A`@\x01Q\x85\x8C`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0Ea\x91\x90a \xA3V[\x8D``\x01Q\x8E`\x80\x01Qa\x18&V[\x94P\x90P\x83\x15\x80\x15a\x0E\x85WP\x88`\xC0\x01Q\x83\x10[\x15a\x0E\x9CWP`\0\x98`\x01\x98P\x96PPPPPPPV[\x83a\x0E\xB2WP`\0\x98\x89\x98P\x96PPPPPPPV[a\rz\x81a\x0F7V[`\0a\x0E\xED\x85\x8A`@\x01Q\x8B`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0E\xDE\x91\x90a \xA3V[\x8C``\x01Q\x8D`\x80\x01Qa\x18\xFDV[\x94P\x90P\x83a\x0F\x07WP`\0\x98\x89\x98P\x96PPPPPPPV[a\rza\x0F\x1E\x8A``\x01Q\x84\x86a\n0\x91\x90a \xA3V[a\x04\x1F\x90\x83a tV[P`\0\x95`\x01\x95P\x93PPPPV[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x0FaW`@Qc9n\xA7\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0a\x0F\x98a\x04\x1F\x83a\x01 \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0F\x86\x91\x90a \xA3V[``\x85\x01Qa\x01\0\x86\x01Q\x91\x90a\x10\xB7V[a\x0F\xC9a\x04\x1F\x84a\x01`\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xB7\x91\x90a \xA3V[``\x86\x01Qa\x01@\x87\x01Q\x91\x90a\x10\x99V[a\x06\xE5\x91\x90a TV[`\0a\x0F\xE8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\x99V[\x93\x92PPPV[`\0a\x0F\xE8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\x99V[`\0a\x0F\xE8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\xB7V[`\0a\x0F\xE8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\xB7V[`\0\x81`\0\x03a\x10GWPg\r\xE0\xB6\xB3\xA7d\0\0a\x06\xE5V[\x82`\0\x03a\x10WWP`\0a\x06\xE5V[`\0a\x10b\x83a\x0F7V[\x90P`\0a\x10wa\x10r\x86a\x0F7V[a\x19\xB3V[\x90P\x81\x81\x02a\x10\x8Eg\r\xE0\xB6\xB3\xA7d\0\0\x82a \xB6V[\x90Pa\x06\xDF\x81a\x1B\xE2V[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x10\xB0W`\0\x80\xFD[P\x91\x02\x04\x90V[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x10\xCEW`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`\0\x80`\0a\x10\xEF\x88\x88\x88\x88\x88a\x1D\x81V[\x90P`\0a\x11\x1Ag\r\xE0\xB6\xB3\xA7d\0\0a\x11\t\x88\x88a\x0F\xD3V[a\x11\x13\x91\x90a tV[\x83\x90a\x10\x04V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x11NWa\x11Ga\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\x04V[\x82\x90a\x10.V[\x90Pa\x11fV[a\x11ca\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x0F\xD3V[\x90P[\x80\x88\x10\x15a\x11|W`\0\x80\x93P\x93PPPa\x11\x86V[\x87\x03\x92P`\x01\x91PP[\x95P\x95\x93PPPPV[`\0\x81\x83\x11a\x11\x9FW\x82a\x0F\xE8V[P\x91\x90PV[` \x83\x01Q`\0\x90a\x11\xB7\x90\x84a\x10\x04V[`@\x85\x01Qa\x11\xC7\x90\x84\x90a\x0F\xD3V[\x10\x15\x80\x15a\x0C\0WP` \x84\x01Qa\x11\xF7\x90\x84a\x11\xF0c;\x9A\xCA\0g\r\xE0\xB6\xB3\xA7d\0\0a tV[\x91\x90a\x10\x99V[`@\x85\x01Qa\x12\x07\x90\x84\x90a\x10\x04V[\x11\x15\x94\x93PPPPV[`\0\x80`\0\x88\x12\x15a\x123Wa\x12&\x88a \x87V[a\x120\x90\x87a tV[\x95P[`\0\x80a\x12@\x8B\x8Ba\x04\xC8V[\x91P\x91P\x80a\x12WW`\0\x80\x93P\x93PPPa\x13\x10V[`\0a\x12f\x83\x8B\x8A\x8A\x8Aa\x1D\xB0V[\x90P`\0a\x12\x89a\x12\x81\x8Aa\x12{\x8A\x8Ea\x10\x19V[\x90a\x10.V[\x89\x90\x89a\x10\xB7V[\x90P\x80\x82\x10\x15a\x12\xA3W`\0\x80\x95P\x95PPPPPa\x13\x10V[\x80\x82\x03g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x12\xD1Wa\x12\xCAa\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x8Ca\x0F\xD3V[\x90Pa\x12\xE9V[a\x12\xE6a\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x8Ca\x10\x04V[\x90P[\x8B\x81\x10\x15a\x13\x02W`\0\x80\x96P\x96PPPPPPa\x13\x10V[\x8B\x90\x03\x95P`\x01\x94PPPPP[\x97P\x97\x95PPPPPPV[`\0\x80`\0\x83`\xE0\x01Q\x13a\x136WP`\0\x92\x83\x92P\x90PV[`\0a\x13E\x84`\0\x01Qa\x0FeV[\x90P`\0a\x13q\x85`@\x01Q\x86``\x01Q\x87`@\x01Qa\x13e\x91\x90a tV[` \x88\x01Q\x91\x90a\x10\xB7V[\x90P`\0\x82\x12a\x13\x98W\x80\x82\x10\x15a\x13\x8BW\x81\x90\x03a\x13\xAEV[P`\0\x94\x85\x94P\x92PPPV[a\x13\xA1\x82a \x87V[a\x13\xAB\x90\x82a tV[\x90P[`\xE0\x85\x01Q`\xC0\x86\x01Qa\x13\xC3\x91\x83\x90a\x10\xB7V[\x90P\x80\x85`\xC0\x01Q\x10\x15a\x13\xDEWP`\0\x94\x85\x94P\x92PPPV[\x80\x85`\xC0\x01Q\x03`\x01\x93P\x93PPP\x91P\x91V[`\0\x80`\0\x80\x84\x12a\x14\x16W\x85Q`@\x01Qa\x14\x0F\x90\x85\x90a tV[\x90Pa\x14QV[`\0a\x14!\x85a \x87V[\x87Q`@\x01Q\x90\x91P\x81\x10\x15a\x14AW\x86Q`@\x01Q\x81\x90\x03\x91Pa\x14OV[`\0\x80\x93P\x93PPPa\x17HV[P[\x85Q\x80Q` \x90\x91\x01Q`\0\x91\x82\x91a\x14j\x91\x90a\x04\xC8V[\x91P\x91P\x80a\x14\x82W`\0\x80\x94P\x94PPPPa\x17HV[\x87Q`\xE0\x81\x01Q`@\x90\x91\x01Q`\0\x91a\x14\xB7\x91a\x14\xAB\x91a\x14\xA4\x91\x90a\x10.V[\x8A\x90a\x0F\xEFV[a\x01\0\x8B\x01Q\x90a\x10\x04V[\x89Q`\xE0\x81\x01Q`\x80\x90\x91\x01Qa\x14\xE2\x91a\x14\xD6\x91a\x12{\x90\x88a\x0F\xEFV[\x8BQ``\x01Q\x90a\x10\x04V[a\x14\xEC\x91\x90a tV[\x90P`\0a\x15$a\x15\x18a\x15\x11\x8C`\0\x01Q`\xE0\x01Q\x88a\x10.\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x8B\x90a\x10\x19V[a\x01\0\x8C\x01Q\x90a\x0F\xD3V[\x90P\x80\x82\x10\x15a\x15?W`\0\x80\x96P\x96PPPPPPa\x17HV[\x80\x82\x03\x91P`\0a\x15\x81\x85\x8C`\0\x01Q`@\x01Q\x8D`\0\x01Q`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x15o\x91\x90a \xA3V[\x8EQ``\x81\x01Q`\x80\x90\x91\x01Qa\x1D\x81V[\x90P`\0a\x15\xAB\x8C`\0\x01Q`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x15\xA4\x91\x90a \xA3V[\x88\x90a\x10.V[\x90P\x80\x82\x10\x15a\x15\xC8W`\0\x80\x98P\x98PPPPPPPPa\x17HV[\x8BQ`\x80\x81\x01Q``\x90\x91\x01Q\x91\x83\x03\x91a\x15\xE4\x91\x83\x91a\x10\xB7V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x16(W\x8BQ`\xE0\x01Qa\x16!\x90a\x11@\x90a\x16\x15\x90g\r\xE0\xB6\xB3\xA7d\0\0a \xA3V[\x8EQ`\xE0\x01Q\x90a\x10\x04V[\x90Pa\x16WV[\x8BQ`\xE0\x01Qa\x16T\x90a\x11@\x90a\x16H\x90g\r\xE0\xB6\xB3\xA7d\0\0a \xA3V[\x8EQ`\xE0\x01Q\x90a\x0F\xD3V[\x90P[\x8BQ``\x01Qa\x16j\x90\x85\x90\x83\x90a\x10\xB7V[\x93P\x83g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a\x16\x8EW\x83g\r\xE0\xB6\xB3\xA7d\0\0\x03\x93Pa\x16\xA2V[`\0`\x01\x98P\x98PPPPPPPPa\x17HV[`\0\x8C`\xE0\x01Q\x12a\x17\x05W`\xC0\x8C\x01Q`\xE0\x8D\x01Qa\x16\xC1\x91a\x10\x04V[\x92Pg\r\xE0\xB6\xB3\xA7d\0\0\x83\x11\x15a\x16\xE6W`\0\x80\x98P\x98PPPPPPPPa\x17HV[g\r\xE0\xB6\xB3\xA7d\0\0\x92\x90\x92\x03\x91a\x16\xFE\x84\x84a\x0F\xEFV[\x93Pa\x17:V[a\x177a\x17\x1E\x8D`\xC0\x01Q\x8E`\xE0\x01Qa\n0\x90a \x87V[a\x170\x90g\r\xE0\xB6\xB3\xA7d\0\0a tV[\x85\x90a\x0F\xEFV[\x93P[P\x91\x96P`\x01\x95PPPPPP[\x93P\x93\x91PPV[`\0\x80\x82\x12\x15a\x0FaW\x81`\0\x03a\x06\xE5V[`\0\x80`\0a\x17u\x89\x89\x88\x88\x88a\x1D\x81V[\x90Pa\x17\x85\x86a\x12{\x89\x8Ba tV[\x97P\x87\x81\x10\x15a\x17\x9CW`\0\x80\x92P\x92PPa\x18\x1BV[\x87\x81\x03a\x17\xAA\x81\x86\x88a\x10\xB7V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x17\xD7Wa\x17\xD0a\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\x04V[\x90Pa\x17\xEFV[a\x17\xECa\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x0F\xD3V[\x90P[a\x17\xF9\x81\x86a\x10\x04V[\x90P\x80\x8A\x10\x15a\x18\x11W`\0\x80\x93P\x93PPPa\x18\x1BV[\x89\x03\x92P`\x01\x91PP[\x96P\x96\x94PPPPPV[`\0\x80`\0a\x188\x89\x89\x88\x88\x88a\x1D\x81V[\x90P\x86\x88\x10\x15a\x18OW`\0\x80\x92P\x92PPa\x18\x1BV[\x96\x86\x90\x03\x96a\x18^\x88\x87a\x10.V[\x97P\x87\x81\x10\x15a\x18uW`\0\x80\x92P\x92PPa\x18\x1BV[\x87\x81\x03a\x18\x83\x81\x86\x88a\x10\xB7V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x18\xB0Wa\x18\xA9a\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\x04V[\x90Pa\x18\xC8V[a\x18\xC5a\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x0F\xD3V[\x90P[a\x18\xD2\x81\x86a\x10\x04V[\x90P\x89\x81\x10\x15a\x18\xEAW`\0\x80\x93P\x93PPPa\x18\x1BV[\x98\x90\x98\x03\x98`\x01\x98P\x96PPPPPPPV[`\0\x80`\0a\x19\x0F\x88\x88\x88\x88\x88a\x1D\xB0V[\x90P`\0a\x19:g\r\xE0\xB6\xB3\xA7d\0\0a\x19)\x88\x88a\x10\x04V[a\x193\x91\x90a tV[\x83\x90a\x0F\xD3V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\x19gWa\x19`a\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x0F\xD3V[\x90Pa\x19\x7FV[a\x19|a\x11@g\r\xE0\xB6\xB3\xA7d\0\0\x89a\x10\x04V[\x90P[a\x19\x89\x81\x86a\x0F\xD3V[\x90P\x88\x81\x10\x15a\x19\xA1W`\0\x80\x93P\x93PPPa\x11\x86V[\x97\x90\x97\x03\x97`\x01\x97P\x95PPPPPPV[`\0\x80\x82\x13a\x19\xD5W`@Qc\xE6\x1BIu`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x82\x81\x1C`\x0F\x10`\x02\x1B\x17\x82\x81\x1C\x90\x91\x10`\x01\x90\x81\x1B\x90\x91\x17\x82\x81\x1C\x90\x91\x10\x17`\x9F\x81\x81\x03``\x01\x92\x90\x92\x1B\x91`_\x19\x82\x01\x90a\x1Aa\x90\x84\x90\x1Ca\x0F7V[lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x91\x90\x91\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1B\xFDWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1C&W`@Qcs\xA2\xD6\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x92l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x84\x01\x84\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x85\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x85\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x85\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x85\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x87\x01\x87\x02\x83\x1D\x90\x81\x01\x90\x87\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x81\x02\x90\x92\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x86\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x81\x81\x05\x95P\x92\x93P\x90\x91\x90a\x1Dwt\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x86\x02`\xC3\x86\x90\x03\x1Ca\x0F7V[\x96\x95PPPPPPV[`\0a\x1D\x8D\x85\x85a\x10.V[a\x1D\xA6a\x1D\x9E\x86a\x12{\x86\x8Ba\x10\x19V[\x85\x90\x85a\x10\xB7V[a\x1Dw\x91\x90a tV[`\0a\x1D\xBC\x85\x85a\x10.V[a\x1D\xA6a\x1D\xCD\x86a\x12{\x86\x8Ba\x0F\xEFV[\x85\x90\x85a\x10\x99V[`@Qa\x01\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\x07WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@Qa\x01 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\x07WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0a\x01\x80\x82\x84\x03\x12\x15a\x1ERW`\0\x80\xFD[a\x1EZa\x1D\xD5V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015`\x80\x82\x01R`\xA0\x82\x015`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R`\xE0\x82\x015`\xE0\x82\x01Ra\x01\0\x80\x83\x015\x81\x83\x01RPa\x01 \x80\x83\x015\x81\x83\x01RPa\x01@\x80\x83\x015\x81\x83\x01RPa\x01`\x80\x83\x015\x81\x83\x01RP\x92\x91PPV[`\0\x80\x82\x84\x03a\x02\xA0\x81\x12\x15a\x1E\xF1W`\0\x80\xFD[a\x02\x80\x80\x82\x12\x15a\x1F\x01W`\0\x80\xFD[a\x1F\ta\x1E\rV[\x91Pa\x1F\x15\x86\x86a\x1E?V[\x82Ra\x01\x80\x85\x015` \x83\x01Ra\x01\xA0\x85\x015`@\x83\x01Ra\x01\xC0\x85\x015``\x83\x01Ra\x01\xE0\x85\x015`\x80\x83\x01Ra\x02\0\x85\x015`\xA0\x83\x01Ra\x02 \x85\x015`\xC0\x83\x01Ra\x02@\x85\x015`\xE0\x83\x01Ra\x02`\x85\x015a\x01\0\x83\x01R\x90\x95\x93\x015\x93PPPV[`\0a\x01\x80\x82\x84\x03\x12\x15a\x1F\x8EW`\0\x80\xFD[a\x0F\xE8\x83\x83a\x1E?V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x1F\xB1W`\0\x80\xFD[PP\x845\x96` \x86\x015\x96P`@\x86\x015\x95``\x81\x015\x95P`\x80\x81\x015\x94P`\xA0\x015\x92P\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1F\xF3W`\0\x80\xFD[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a LWa La \x16V[PP\x92\x91PPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x03\xA4Wa\x03\xA4a \x16V[\x80\x82\x01\x80\x82\x11\x15a\x06\xE5Wa\x06\xE5a \x16V[`\0`\x01`\xFF\x1B\x82\x01a \x9CWa \x9Ca \x16V[P`\0\x03\x90V[\x81\x81\x03\x81\x81\x11\x15a\x06\xE5Wa\x06\xE5a \x16V[`\0\x82a \xD3WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a \xEDWa \xEDa \x16V[P\x05\x90V\xFE\xA2dipfsX\"\x12 \x0F,g\xF9\xB2\xAC\x04L\xE4&A\x16\xBF/\x0C\x0F\xE6\x9A\xD4\xDFp&\x90#\xF3\xEDr\xE7&\x0C\xFE\xD7dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static LPMATH_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LPMath<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LPMath<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LPMath<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LPMath<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LPMath<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LPMath)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LPMath<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LPMATH_ABI.clone(),
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
                LPMATH_ABI.clone(),
                LPMATH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `calculatePresentValue` (0xc00b11d3) function
        pub fn calculate_present_value(
            &self,
            params: PresentValueParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([192, 11, 17, 211], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculatePresentValueSafe` (0x5f716349) function
        pub fn calculate_present_value_safe(
            &self,
            params: PresentValueParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, bool),
        > {
            self.0
                .method_hash([95, 113, 99, 73], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateUpdateLiquiditySafe` (0xd0b457ce) function
        pub fn calculate_update_liquidity_safe(
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
                bool,
            ),
        > {
            self.0
                .method_hash(
                    [208, 180, 87, 206],
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LPMath<M> {
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
    pub enum LPMathErrors {
        ExpInvalidExponent(ExpInvalidExponent),
        InvalidPresentValue(InvalidPresentValue),
        LnInvalidInput(LnInvalidInput),
        UnsafeCastToInt256(UnsafeCastToInt256),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LPMathErrors {
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
    impl ::ethers::core::abi::AbiEncode for LPMathErrors {
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
    impl ::ethers::contract::ContractRevert for LPMathErrors {
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
    impl ::core::fmt::Display for LPMathErrors {
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
    impl ::core::convert::From<::std::string::String> for LPMathErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ExpInvalidExponent> for LPMathErrors {
        fn from(value: ExpInvalidExponent) -> Self {
            Self::ExpInvalidExponent(value)
        }
    }
    impl ::core::convert::From<InvalidPresentValue> for LPMathErrors {
        fn from(value: InvalidPresentValue) -> Self {
            Self::InvalidPresentValue(value)
        }
    }
    impl ::core::convert::From<LnInvalidInput> for LPMathErrors {
        fn from(value: LnInvalidInput) -> Self {
            Self::LnInvalidInput(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToInt256> for LPMathErrors {
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
    ///Container type for all input parameters for the `calculatePresentValueSafe` function with signature `calculatePresentValueSafe((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0x5f716349`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculatePresentValueSafe",
        abi = "calculatePresentValueSafe((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct CalculatePresentValueSafeCall {
        pub params: PresentValueParams,
    }
    ///Container type for all input parameters for the `calculateUpdateLiquiditySafe` function with signature `calculateUpdateLiquiditySafe(uint256,int256,uint256,uint256,int256)` and selector `0xd0b457ce`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateUpdateLiquiditySafe",
        abi = "calculateUpdateLiquiditySafe(uint256,int256,uint256,uint256,int256)"
    )]
    pub struct CalculateUpdateLiquiditySafeCall {
        pub share_reserves: ::ethers::core::types::U256,
        pub share_adjustment: ::ethers::core::types::I256,
        pub bond_reserves: ::ethers::core::types::U256,
        pub minimum_share_reserves: ::ethers::core::types::U256,
        pub share_reserves_delta: ::ethers::core::types::I256,
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
    pub enum LPMathCalls {
        CalculateDistributeExcessIdle(CalculateDistributeExcessIdleCall),
        CalculateInitialReserves(CalculateInitialReservesCall),
        CalculatePresentValue(CalculatePresentValueCall),
        CalculatePresentValueSafe(CalculatePresentValueSafeCall),
        CalculateUpdateLiquiditySafe(CalculateUpdateLiquiditySafeCall),
    }
    impl ::ethers::core::abi::AbiDecode for LPMathCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CalculateDistributeExcessIdleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateDistributeExcessIdle(decoded));
            }
            if let Ok(decoded) = <CalculateInitialReservesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateInitialReserves(decoded));
            }
            if let Ok(decoded) = <CalculatePresentValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculatePresentValue(decoded));
            }
            if let Ok(decoded) = <CalculatePresentValueSafeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculatePresentValueSafe(decoded));
            }
            if let Ok(decoded) = <CalculateUpdateLiquiditySafeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateUpdateLiquiditySafe(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LPMathCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CalculateDistributeExcessIdle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateInitialReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculatePresentValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculatePresentValueSafe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateUpdateLiquiditySafe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LPMathCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CalculateDistributeExcessIdle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateInitialReserves(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculatePresentValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculatePresentValueSafe(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateUpdateLiquiditySafe(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CalculateDistributeExcessIdleCall> for LPMathCalls {
        fn from(value: CalculateDistributeExcessIdleCall) -> Self {
            Self::CalculateDistributeExcessIdle(value)
        }
    }
    impl ::core::convert::From<CalculateInitialReservesCall> for LPMathCalls {
        fn from(value: CalculateInitialReservesCall) -> Self {
            Self::CalculateInitialReserves(value)
        }
    }
    impl ::core::convert::From<CalculatePresentValueCall> for LPMathCalls {
        fn from(value: CalculatePresentValueCall) -> Self {
            Self::CalculatePresentValue(value)
        }
    }
    impl ::core::convert::From<CalculatePresentValueSafeCall> for LPMathCalls {
        fn from(value: CalculatePresentValueSafeCall) -> Self {
            Self::CalculatePresentValueSafe(value)
        }
    }
    impl ::core::convert::From<CalculateUpdateLiquiditySafeCall> for LPMathCalls {
        fn from(value: CalculateUpdateLiquiditySafeCall) -> Self {
            Self::CalculateUpdateLiquiditySafe(value)
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
    ///Container type for all return fields from the `calculatePresentValueSafe` function with signature `calculatePresentValueSafe((uint256,int256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0x5f716349`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculatePresentValueSafeReturn(
        pub ::ethers::core::types::U256,
        pub bool,
    );
    ///Container type for all return fields from the `calculateUpdateLiquiditySafe` function with signature `calculateUpdateLiquiditySafe(uint256,int256,uint256,uint256,int256)` and selector `0xd0b457ce`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculateUpdateLiquiditySafeReturn {
        pub share_reserves: ::ethers::core::types::U256,
        pub share_adjustment: ::ethers::core::types::I256,
        pub bond_reserves: ::ethers::core::types::U256,
        pub p3: bool,
    }
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
