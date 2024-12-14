pub use mock_fixed_point_math::*;
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
pub mod mock_fixed_point_math {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("divDown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("divDown"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                    ::std::borrow::ToOwned::to_owned("divUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("divUp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                    ::std::borrow::ToOwned::to_owned("exp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
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
                    ::std::borrow::ToOwned::to_owned("ln"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ln"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                    ::std::borrow::ToOwned::to_owned("mulDivDown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mulDivDown"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
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
                                    name: ::std::borrow::ToOwned::to_owned("d"),
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
                                    name: ::std::borrow::ToOwned::to_owned("z"),
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
                    ::std::borrow::ToOwned::to_owned("mulDivUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mulDivUp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
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
                                    name: ::std::borrow::ToOwned::to_owned("d"),
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
                                    name: ::std::borrow::ToOwned::to_owned("z"),
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
                    ::std::borrow::ToOwned::to_owned("mulDown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mulDown"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                    ::std::borrow::ToOwned::to_owned("mulUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mulUp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                    ::std::borrow::ToOwned::to_owned("pow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
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
                    ::std::borrow::ToOwned::to_owned("updateWeightedAverage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateWeightedAverage",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_average"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_totalWeight"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_deltaWeight"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_isAdding"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("average"),
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
    pub static MOCKFIXEDPOINTMATH_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\t8\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c\x9B\xA5\xF5.\x11a\0fW\x80c\x9B\xA5\xF5.\x14a\x01\x14W\x80c\xB6{\xEE\x04\x14a\x01'W\x80c\xCB\xE8b?\x14a\x01:W\x80c\xE4gQ\xE3\x14a\x01MW\x80c\xF3\xE4\xF8|\x14a\x01`W`\0\x80\xFD[\x80c\x0C\x9B\x98\x81\x14a\0\xA3W\x80c\x12\xBDj\xC0\x14a\0\xC8W\x80c.Li\x7F\x14a\0\xDBW\x80c\x81\xE5\xF7\xC9\x14a\0\xEEW\x80c\x8Eo#S\x14a\x01\x01W[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x07\xD1V[a\x01sV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\0\xD66`\x04a\x07\xF3V[a\x01\x8AV[a\0\xB6a\0\xE96`\x04a\x07\xD1V[a\x01\xA1V[a\0\xB6a\0\xFC6`\x04a\x08\x1FV[a\x01\xAEV[a\0\xB6a\x01\x0F6`\x04a\x08qV[a\x01\xC9V[a\0\xB6a\x01\"6`\x04a\x07\xD1V[a\x01\xDCV[a\0\xB6a\x0156`\x04a\x07\xF3V[a\x01\xE9V[a\0\xB6a\x01H6`\x04a\x07\xD1V[a\x01\xF7V[a\0\xB6a\x01[6`\x04a\x08qV[a\x02\x04V[a\0\xB6a\x01n6`\x04a\x07\xD1V[a\x02\x10V[`\0\x80a\x01\x80\x84\x84a\x02\x1DV[\x91PP[\x92\x91PPV[`\0\x80a\x01\x98\x85\x85\x85a\x022V[\x95\x94PPPPPV[`\0\x80a\x01\x80\x84\x84a\x02XV[`\0\x80a\x01\xBE\x87\x87\x87\x87\x87a\x02\xCDV[\x97\x96PPPPPPPV[`\0\x80a\x01\xD5\x83a\x03qV[\x93\x92PPPV[`\0\x80a\x01\x80\x84\x84a\x05\xA0V[`\0\x80a\x01\x98\x85\x85\x85a\x05\xB1V[`\0\x80a\x01\x80\x84\x84a\x05\xCFV[`\0\x80a\x01\xD5\x83a\x05\xE4V[`\0\x80a\x01\x80\x84\x84a\x07yV[`\0a\x01\xD5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x05\xB1V[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x02IW`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`\0\x81`\0\x03a\x02qWPg\r\xE0\xB6\xB3\xA7d\0\0a\x01\x84V[\x82`\0\x03a\x02\x81WP`\0a\x01\x84V[`\0a\x02\x8C\x83a\x07\x8EV[\x90P`\0a\x02\xA1a\x02\x9C\x86a\x07\x8EV[a\x03qV[\x90P\x81\x81\x02a\x02\xB8g\r\xE0\xB6\xB3\xA7d\0\0\x82a\x08\xA0V[\x90Pa\x02\xC3\x81a\x05\xE4V[\x96\x95PPPPPPV[`\0\x82`\0\x03a\x02\xDEWP\x84a\x01\x98V[\x81\x15a\x037Wa\x03\x15a\x02\xF1\x84\x87a\x08\xDCV[a\x02\xFB\x85\x87a\x02\x1DV[a\x03\x05\x88\x8Aa\x02\x1DV[a\x03\x0F\x91\x90a\x08\xDCV[\x90a\x05\xA0V[\x90P`\0a\x03#\x85\x88a\x07\xBCV[\x90P\x80\x82\x10\x15a\x031W\x80\x91P[Pa\x01\x98V[\x82\x85\x03a\x03FWP`\0a\x01\x98V[a\x02\xC3a\x03S\x84\x87a\x08\xEFV[a\x03]\x85\x87a\x07yV[a\x03g\x88\x8Aa\x02\x1DV[a\x03\x0F\x91\x90a\x08\xEFV[`\0\x80\x82\x13a\x03\x93W`@Qc\xE6\x1BIu`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x82\x81\x1C`\x0F\x10`\x02\x1B\x17\x82\x81\x1C\x90\x91\x10`\x01\x90\x81\x1B\x90\x91\x17\x82\x81\x1C\x90\x91\x10\x17`\x9F\x81\x81\x03``\x01\x92\x90\x92\x1B\x91`_\x19\x82\x01\x90a\x04\x1F\x90\x84\x90\x1Ca\x07\x8EV[lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x91\x90\x91\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\x01\xD5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x05\xC8W`\0\x80\xFD[P\x91\x02\x04\x90V[`\0a\x01\xD5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x022V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x05\xFFWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x06(W`@Qcs\xA2\xD6\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x92l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x84\x01\x84\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x85\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x85\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x85\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x85\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x87\x01\x87\x02\x83\x1D\x90\x81\x01\x90\x87\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x81\x02\x90\x92\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x86\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x81\x81\x05\x95P\x92\x93P\x90\x91\x90a\x02\xC3t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x86\x02`\xC3\x86\x90\x03\x1Ca\x07\x8EV[`\0a\x01\xD5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x022V[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x07\xB8W`@Qc9n\xA7\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0\x81\x83\x11a\x07\xCBW\x82a\x01\xD5V[P\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\xE4W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x08\x08W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x087W`\0\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015\x80\x15\x15\x81\x14a\x08cW`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0` \x82\x84\x03\x12\x15a\x08\x83W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x08\xBDWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x08\xD7Wa\x08\xD7a\x08\x8AV[P\x05\x90V[\x80\x82\x01\x80\x82\x11\x15a\x01\x84Wa\x01\x84a\x08\x8AV[\x81\x81\x03\x81\x81\x11\x15a\x01\x84Wa\x01\x84a\x08\x8AV\xFE\xA2dipfsX\"\x12 \x90\xC3\xBDT\xEC9}\x10.-\xE17\x01\xFD\x81\xF7\x08\xC9!\x97v\xC5\xA6\xDE\x05#\xB8\x95.b\xA3\xC0dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static MOCKFIXEDPOINTMATH_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c\x9B\xA5\xF5.\x11a\0fW\x80c\x9B\xA5\xF5.\x14a\x01\x14W\x80c\xB6{\xEE\x04\x14a\x01'W\x80c\xCB\xE8b?\x14a\x01:W\x80c\xE4gQ\xE3\x14a\x01MW\x80c\xF3\xE4\xF8|\x14a\x01`W`\0\x80\xFD[\x80c\x0C\x9B\x98\x81\x14a\0\xA3W\x80c\x12\xBDj\xC0\x14a\0\xC8W\x80c.Li\x7F\x14a\0\xDBW\x80c\x81\xE5\xF7\xC9\x14a\0\xEEW\x80c\x8Eo#S\x14a\x01\x01W[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x07\xD1V[a\x01sV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\0\xD66`\x04a\x07\xF3V[a\x01\x8AV[a\0\xB6a\0\xE96`\x04a\x07\xD1V[a\x01\xA1V[a\0\xB6a\0\xFC6`\x04a\x08\x1FV[a\x01\xAEV[a\0\xB6a\x01\x0F6`\x04a\x08qV[a\x01\xC9V[a\0\xB6a\x01\"6`\x04a\x07\xD1V[a\x01\xDCV[a\0\xB6a\x0156`\x04a\x07\xF3V[a\x01\xE9V[a\0\xB6a\x01H6`\x04a\x07\xD1V[a\x01\xF7V[a\0\xB6a\x01[6`\x04a\x08qV[a\x02\x04V[a\0\xB6a\x01n6`\x04a\x07\xD1V[a\x02\x10V[`\0\x80a\x01\x80\x84\x84a\x02\x1DV[\x91PP[\x92\x91PPV[`\0\x80a\x01\x98\x85\x85\x85a\x022V[\x95\x94PPPPPV[`\0\x80a\x01\x80\x84\x84a\x02XV[`\0\x80a\x01\xBE\x87\x87\x87\x87\x87a\x02\xCDV[\x97\x96PPPPPPPV[`\0\x80a\x01\xD5\x83a\x03qV[\x93\x92PPPV[`\0\x80a\x01\x80\x84\x84a\x05\xA0V[`\0\x80a\x01\x98\x85\x85\x85a\x05\xB1V[`\0\x80a\x01\x80\x84\x84a\x05\xCFV[`\0\x80a\x01\xD5\x83a\x05\xE4V[`\0\x80a\x01\x80\x84\x84a\x07yV[`\0a\x01\xD5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x05\xB1V[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x02IW`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`\0\x81`\0\x03a\x02qWPg\r\xE0\xB6\xB3\xA7d\0\0a\x01\x84V[\x82`\0\x03a\x02\x81WP`\0a\x01\x84V[`\0a\x02\x8C\x83a\x07\x8EV[\x90P`\0a\x02\xA1a\x02\x9C\x86a\x07\x8EV[a\x03qV[\x90P\x81\x81\x02a\x02\xB8g\r\xE0\xB6\xB3\xA7d\0\0\x82a\x08\xA0V[\x90Pa\x02\xC3\x81a\x05\xE4V[\x96\x95PPPPPPV[`\0\x82`\0\x03a\x02\xDEWP\x84a\x01\x98V[\x81\x15a\x037Wa\x03\x15a\x02\xF1\x84\x87a\x08\xDCV[a\x02\xFB\x85\x87a\x02\x1DV[a\x03\x05\x88\x8Aa\x02\x1DV[a\x03\x0F\x91\x90a\x08\xDCV[\x90a\x05\xA0V[\x90P`\0a\x03#\x85\x88a\x07\xBCV[\x90P\x80\x82\x10\x15a\x031W\x80\x91P[Pa\x01\x98V[\x82\x85\x03a\x03FWP`\0a\x01\x98V[a\x02\xC3a\x03S\x84\x87a\x08\xEFV[a\x03]\x85\x87a\x07yV[a\x03g\x88\x8Aa\x02\x1DV[a\x03\x0F\x91\x90a\x08\xEFV[`\0\x80\x82\x13a\x03\x93W`@Qc\xE6\x1BIu`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x82\x81\x1C`\x0F\x10`\x02\x1B\x17\x82\x81\x1C\x90\x91\x10`\x01\x90\x81\x1B\x90\x91\x17\x82\x81\x1C\x90\x91\x10\x17`\x9F\x81\x81\x03``\x01\x92\x90\x92\x1B\x91`_\x19\x82\x01\x90a\x04\x1F\x90\x84\x90\x1Ca\x07\x8EV[lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x91\x90\x91\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\x01\xD5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x05\xC8W`\0\x80\xFD[P\x91\x02\x04\x90V[`\0a\x01\xD5\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x022V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x05\xFFWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x06(W`@Qcs\xA2\xD6\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x92l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x84\x01\x84\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x85\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x85\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x85\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x85\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x87\x01\x87\x02\x83\x1D\x90\x81\x01\x90\x87\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x81\x02\x90\x92\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x86\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x81\x81\x05\x95P\x92\x93P\x90\x91\x90a\x02\xC3t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x86\x02`\xC3\x86\x90\x03\x1Ca\x07\x8EV[`\0a\x01\xD5\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x022V[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x07\xB8W`@Qc9n\xA7\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0\x81\x83\x11a\x07\xCBW\x82a\x01\xD5V[P\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\xE4W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x08\x08W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x087W`\0\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015\x80\x15\x15\x81\x14a\x08cW`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0` \x82\x84\x03\x12\x15a\x08\x83W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x08\xBDWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x08\xD7Wa\x08\xD7a\x08\x8AV[P\x05\x90V[\x80\x82\x01\x80\x82\x11\x15a\x01\x84Wa\x01\x84a\x08\x8AV[\x81\x81\x03\x81\x81\x11\x15a\x01\x84Wa\x01\x84a\x08\x8AV\xFE\xA2dipfsX\"\x12 \x90\xC3\xBDT\xEC9}\x10.-\xE17\x01\xFD\x81\xF7\x08\xC9!\x97v\xC5\xA6\xDE\x05#\xB8\x95.b\xA3\xC0dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKFIXEDPOINTMATH_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockFixedPointMath<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockFixedPointMath<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockFixedPointMath<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockFixedPointMath<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockFixedPointMath<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockFixedPointMath))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockFixedPointMath<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKFIXEDPOINTMATH_ABI.clone(),
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
                MOCKFIXEDPOINTMATH_ABI.clone(),
                MOCKFIXEDPOINTMATH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `divDown` (0x9ba5f52e) function
        pub fn div_down(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([155, 165, 245, 46], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `divUp` (0xcbe8623f) function
        pub fn div_up(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([203, 232, 98, 63], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exp` (0xe46751e3) function
        pub fn exp(
            &self,
            x: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([228, 103, 81, 227], x)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ln` (0x8e6f2353) function
        pub fn ln(
            &self,
            x: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([142, 111, 35, 83], x)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mulDivDown` (0xb67bee04) function
        pub fn mul_div_down(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            d: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([182, 123, 238, 4], (x, y, d))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mulDivUp` (0x12bd6ac0) function
        pub fn mul_div_up(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
            d: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([18, 189, 106, 192], (x, y, d))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mulDown` (0x0c9b9881) function
        pub fn mul_down(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([12, 155, 152, 129], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mulUp` (0xf3e4f87c) function
        pub fn mul_up(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([243, 228, 248, 124], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pow` (0x2e4c697f) function
        pub fn pow(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([46, 76, 105, 127], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateWeightedAverage` (0x81e5f7c9) function
        pub fn update_weighted_average(
            &self,
            average: ::ethers::core::types::U256,
            total_weight: ::ethers::core::types::U256,
            delta: ::ethers::core::types::U256,
            delta_weight: ::ethers::core::types::U256,
            is_adding: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [129, 229, 247, 201],
                    (average, total_weight, delta, delta_weight, is_adding),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockFixedPointMath<M> {
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
    pub enum MockFixedPointMathErrors {
        ExpInvalidExponent(ExpInvalidExponent),
        LnInvalidInput(LnInvalidInput),
        UnsafeCastToInt256(UnsafeCastToInt256),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockFixedPointMathErrors {
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
    impl ::ethers::core::abi::AbiEncode for MockFixedPointMathErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ExpInvalidExponent(element) => {
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
    impl ::ethers::contract::ContractRevert for MockFixedPointMathErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ExpInvalidExponent as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for MockFixedPointMathErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExpInvalidExponent(element) => {
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
    impl ::core::convert::From<::std::string::String> for MockFixedPointMathErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ExpInvalidExponent> for MockFixedPointMathErrors {
        fn from(value: ExpInvalidExponent) -> Self {
            Self::ExpInvalidExponent(value)
        }
    }
    impl ::core::convert::From<LnInvalidInput> for MockFixedPointMathErrors {
        fn from(value: LnInvalidInput) -> Self {
            Self::LnInvalidInput(value)
        }
    }
    impl ::core::convert::From<UnsafeCastToInt256> for MockFixedPointMathErrors {
        fn from(value: UnsafeCastToInt256) -> Self {
            Self::UnsafeCastToInt256(value)
        }
    }
    ///Container type for all input parameters for the `divDown` function with signature `divDown(uint256,uint256)` and selector `0x9ba5f52e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "divDown", abi = "divDown(uint256,uint256)")]
    pub struct DivDownCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `divUp` function with signature `divUp(uint256,uint256)` and selector `0xcbe8623f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "divUp", abi = "divUp(uint256,uint256)")]
    pub struct DivUpCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `exp` function with signature `exp(int256)` and selector `0xe46751e3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "exp", abi = "exp(int256)")]
    pub struct ExpCall {
        pub x: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `ln` function with signature `ln(int256)` and selector `0x8e6f2353`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ln", abi = "ln(int256)")]
    pub struct LnCall {
        pub x: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `mulDivDown` function with signature `mulDivDown(uint256,uint256,uint256)` and selector `0xb67bee04`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mulDivDown", abi = "mulDivDown(uint256,uint256,uint256)")]
    pub struct MulDivDownCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub d: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mulDivUp` function with signature `mulDivUp(uint256,uint256,uint256)` and selector `0x12bd6ac0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mulDivUp", abi = "mulDivUp(uint256,uint256,uint256)")]
    pub struct MulDivUpCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub d: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mulDown` function with signature `mulDown(uint256,uint256)` and selector `0x0c9b9881`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mulDown", abi = "mulDown(uint256,uint256)")]
    pub struct MulDownCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mulUp` function with signature `mulUp(uint256,uint256)` and selector `0xf3e4f87c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mulUp", abi = "mulUp(uint256,uint256)")]
    pub struct MulUpCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pow` function with signature `pow(uint256,uint256)` and selector `0x2e4c697f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pow", abi = "pow(uint256,uint256)")]
    pub struct PowCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateWeightedAverage` function with signature `updateWeightedAverage(uint256,uint256,uint256,uint256,bool)` and selector `0x81e5f7c9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "updateWeightedAverage",
        abi = "updateWeightedAverage(uint256,uint256,uint256,uint256,bool)"
    )]
    pub struct UpdateWeightedAverageCall {
        pub average: ::ethers::core::types::U256,
        pub total_weight: ::ethers::core::types::U256,
        pub delta: ::ethers::core::types::U256,
        pub delta_weight: ::ethers::core::types::U256,
        pub is_adding: bool,
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
    pub enum MockFixedPointMathCalls {
        DivDown(DivDownCall),
        DivUp(DivUpCall),
        Exp(ExpCall),
        Ln(LnCall),
        MulDivDown(MulDivDownCall),
        MulDivUp(MulDivUpCall),
        MulDown(MulDownCall),
        MulUp(MulUpCall),
        Pow(PowCall),
        UpdateWeightedAverage(UpdateWeightedAverageCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockFixedPointMathCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DivDownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DivDown(decoded));
            }
            if let Ok(decoded) = <DivUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DivUp(decoded));
            }
            if let Ok(decoded) = <ExpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Exp(decoded));
            }
            if let Ok(decoded) = <LnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ln(decoded));
            }
            if let Ok(decoded) = <MulDivDownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MulDivDown(decoded));
            }
            if let Ok(decoded) = <MulDivUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MulDivUp(decoded));
            }
            if let Ok(decoded) = <MulDownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MulDown(decoded));
            }
            if let Ok(decoded) = <MulUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MulUp(decoded));
            }
            if let Ok(decoded) = <PowCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pow(decoded));
            }
            if let Ok(decoded) = <UpdateWeightedAverageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateWeightedAverage(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockFixedPointMathCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DivDown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DivUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Exp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ln(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MulDivDown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MulDivUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MulDown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MulUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateWeightedAverage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockFixedPointMathCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DivDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Exp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ln(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulDivDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulDivUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pow(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateWeightedAverage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DivDownCall> for MockFixedPointMathCalls {
        fn from(value: DivDownCall) -> Self {
            Self::DivDown(value)
        }
    }
    impl ::core::convert::From<DivUpCall> for MockFixedPointMathCalls {
        fn from(value: DivUpCall) -> Self {
            Self::DivUp(value)
        }
    }
    impl ::core::convert::From<ExpCall> for MockFixedPointMathCalls {
        fn from(value: ExpCall) -> Self {
            Self::Exp(value)
        }
    }
    impl ::core::convert::From<LnCall> for MockFixedPointMathCalls {
        fn from(value: LnCall) -> Self {
            Self::Ln(value)
        }
    }
    impl ::core::convert::From<MulDivDownCall> for MockFixedPointMathCalls {
        fn from(value: MulDivDownCall) -> Self {
            Self::MulDivDown(value)
        }
    }
    impl ::core::convert::From<MulDivUpCall> for MockFixedPointMathCalls {
        fn from(value: MulDivUpCall) -> Self {
            Self::MulDivUp(value)
        }
    }
    impl ::core::convert::From<MulDownCall> for MockFixedPointMathCalls {
        fn from(value: MulDownCall) -> Self {
            Self::MulDown(value)
        }
    }
    impl ::core::convert::From<MulUpCall> for MockFixedPointMathCalls {
        fn from(value: MulUpCall) -> Self {
            Self::MulUp(value)
        }
    }
    impl ::core::convert::From<PowCall> for MockFixedPointMathCalls {
        fn from(value: PowCall) -> Self {
            Self::Pow(value)
        }
    }
    impl ::core::convert::From<UpdateWeightedAverageCall> for MockFixedPointMathCalls {
        fn from(value: UpdateWeightedAverageCall) -> Self {
            Self::UpdateWeightedAverage(value)
        }
    }
    ///Container type for all return fields from the `divDown` function with signature `divDown(uint256,uint256)` and selector `0x9ba5f52e`
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
    pub struct DivDownReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `divUp` function with signature `divUp(uint256,uint256)` and selector `0xcbe8623f`
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
    pub struct DivUpReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `exp` function with signature `exp(int256)` and selector `0xe46751e3`
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
    pub struct ExpReturn {
        pub r: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `ln` function with signature `ln(int256)` and selector `0x8e6f2353`
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
    pub struct LnReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `mulDivDown` function with signature `mulDivDown(uint256,uint256,uint256)` and selector `0xb67bee04`
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
    pub struct MulDivDownReturn {
        pub z: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `mulDivUp` function with signature `mulDivUp(uint256,uint256,uint256)` and selector `0x12bd6ac0`
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
    pub struct MulDivUpReturn {
        pub z: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `mulDown` function with signature `mulDown(uint256,uint256)` and selector `0x0c9b9881`
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
    pub struct MulDownReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mulUp` function with signature `mulUp(uint256,uint256)` and selector `0xf3e4f87c`
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
    pub struct MulUpReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pow` function with signature `pow(uint256,uint256)` and selector `0x2e4c697f`
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
    pub struct PowReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `updateWeightedAverage` function with signature `updateWeightedAverage(uint256,uint256,uint256,uint256,bool)` and selector `0x81e5f7c9`
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
    pub struct UpdateWeightedAverageReturn {
        pub average: ::ethers::core::types::U256,
    }
}
