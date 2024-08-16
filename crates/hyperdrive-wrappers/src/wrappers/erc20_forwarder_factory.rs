pub use erc20_forwarder_factory::*;
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
pub mod erc20_forwarder_factory {
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
                    ::std::borrow::ToOwned::to_owned("ERC20LINK_HASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ERC20LINK_HASH"),
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
                    ::std::borrow::ToOwned::to_owned("create"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("create"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("__token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IMultiToken"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("__tokenId"),
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
                                        ::std::borrow::ToOwned::to_owned("contract IERC20Forwarder"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDeployDetails"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDeployDetails"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IMultiToken"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getForwarder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getForwarder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("__token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IMultiToken"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("__tokenId"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidForwarderAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidForwarderAddress",
                            ),
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
    pub static ERC20FORWARDERFACTORY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81\x17\x81U`\x02U4\x80\x15a\0%W`\0\x80\xFD[P`@Qa\x18\xF48\x03\x80a\x18\xF4\x839\x81\x01`@\x81\x90Ra\0D\x91a\0mV[`\0a\0P\x82\x82a\x01\xC1V[PPa\x02\x80V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a\0\x80W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\0\x97W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\0\xABW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\0\xBDWa\0\xBDa\0WV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\0\xE5Wa\0\xE5a\0WV[\x81`@R\x82\x81R\x88\x86\x84\x87\x01\x01\x11\x15a\0\xFDW`\0\x80\xFD[`\0\x93P[\x82\x84\x10\x15a\x01\x1FW\x84\x84\x01\x86\x01Q\x81\x85\x01\x87\x01R\x92\x85\x01\x92a\x01\x02V[`\0\x86\x84\x83\x01\x01R\x80\x96PPPPPPP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01JW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01jWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x01\xBCW`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x01\x99WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x01\xB8W\x82\x81U`\x01\x01a\x01\xA5V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xDAWa\x01\xDAa\0WV[a\x01\xEE\x81a\x01\xE8\x84Ta\x016V[\x84a\x01pV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x02#W`\0\x84\x15a\x02\x0BWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x01\xB8V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x02RW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x023V[P\x85\x82\x10\x15a\x02pW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x16e\x80a\x02\x8F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x0E\xCA\xEAs\x11a\0[W\x80c\x0E\xCA\xEAs\x14a\0\xFFW\x80cT\xFDMP\x14a\x01\x12W\x80c`\x0E\xB4\xBA\x14a\x018W\x80c\xD10S\xBB\x14a\x01nW`\0\x80\xFD[\x80c\x04\xBA\xA0\x0B\x14a\0\x82W\x80c\x06\xFD\xDE\x03\x14a\0\xCCW\x80c\x07\x10\xFDX\x14a\0\xD4W[`\0\x80\xFD[a\0\xB6`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01tERC20ForwarderFactory`X\x1B\x81RP\x81V[`@Qa\0\xC3\x91\x90a\x03\xFEV[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\x01\x84V[a\0\xE7a\0\xE26`\x04a\x04MV[a\x02\x12V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC3V[a\0\xE7a\x01\r6`\x04a\x04MV[a\x02\xE7V[a\0\xB6`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fv1.0.17`\xC8\x1B\x81RP\x81V[a\x01O`\x01T`\x02T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\0\xC3V[a\x01va\x03\xC4V[`@Q\x90\x81R` \x01a\0\xC3V[`\0\x80Ta\x01\x91\x90a\x04\x85V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xBD\x90a\x04\x85V[\x80\x15a\x02\nW\x80`\x1F\x10a\x01\xDFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\nV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\xEDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R\x90\x81\x01\x82\x90R`\0\x90\x81\x90``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0`\xFF`\xF8\x1B0\x83`@Q\x80` \x01a\x02d\x90a\x03\xF1V[` \x82\x01\x81\x03\x82R`\x1F\x19`\x1F\x82\x01\x16`@RP\x80Q\x90` \x01 `@Q` \x01a\x02\xC6\x94\x93\x92\x91\x90`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84R``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x84\x01R`\x15\x83\x01R`5\x82\x01R`U\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`\x02\x81\x90U`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@\x80Q` \x81\x01\x92\x90\x92R\x81\x01\x82\x90R`\0\x90\x81\x90``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x81`@Qa\x03J\x90a\x03\xF1V[\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15a\x03jW=`\0\x80>=`\0\xFD[P\x90Pa\x03w\x85\x85a\x02\x12V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\xA7W`@Qb\xE0wU`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81\x17\x81U`\x02U\x94\x93PPPPV[`@Qa\x03\xD3` \x82\x01a\x03\xF1V[` \x82\x01\x81\x03\x82R`\x1F\x19`\x1F\x82\x01\x16`@RP\x80Q\x90` \x01 \x81V[a\x11p\x80a\x04\xC0\x839\x01\x90V[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x04,W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x04\x10V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x04`W`\0\x80\xFD[\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04wW`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x04\x99W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x04\xB9WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV\xFE`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@\x80Qc0\x07Z]`\xE1\x1B\x81R\x81Q3\x92\x83\x92c`\x0E\xB4\xBA\x92`\x04\x80\x83\x01\x93\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\0KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0o\x91\x90a\0\x84V[`\xA0R`\x01`\x01`\xA0\x1B\x03\x16`\x80RPa\0\xBEV[`\0\x80`@\x83\x85\x03\x12\x15a\0\x97W`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xAEW`\0\x80\xFD[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\x80Q`\xA0Qa\x10\x08a\x01h`\09`\0\x81\x81a\x01\x83\x01R\x81\x81a\x02\xF5\x01R\x81\x81a\x03\xA9\x01R\x81\x81a\x04\xA7\x01R\x81\x81a\x05R\x01R\x81\x81a\x06d\x01R\x81\x81a\x07\x19\x01R\x81\x81a\x07\x88\x01R\x81\x81a\nM\x01Ra\x0B\xF5\x01R`\0\x81\x81a\x02\xAD\x01R\x81\x81a\x03\x1E\x01R\x81\x81a\x03\xEF\x01R\x81\x81a\x04\xD0\x01R\x81\x81a\x05\xA0\x01R\x81\x81a\x06\x9D\x01R\x81\x81a\x07B\x01R\x81\x81a\x07\xD8\x01R\x81\x81a\n\x8A\x01R\x81\x81a\x0Bk\x01Ra\x0C3\x01Ra\x10\x08`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x0BW`\x005`\xE0\x1C\x80cT\xFDMP\x11a\0\xA2W\x80c\xA9\x05\x9C\xBB\x11a\0qW\x80c\xA9\x05\x9C\xBB\x14a\x02eW\x80c\xD5\x05\xAC\xCF\x14a\x02xW\x80c\xDDb\xED>\x14a\x02\x8DW\x80c\xF6\x98\xDA%\x14a\x02\xA0W\x80c\xFC\x0CTj\x14a\x02\xA8W`\0\x80\xFD[\x80cT\xFDMP\x14a\x02\x04W\x80cp\xA0\x821\x14a\x02*W\x80c~\xCE\xBE\0\x14a\x02=W\x80c\x95\xD8\x9BA\x14a\x02]W`\0\x80\xFD[\x80c\x18\x16\r\xDD\x11a\0\xDEW\x80c\x18\x16\r\xDD\x14a\x01\xB3W\x80c#\xB8r\xDD\x14a\x01\xBBW\x80c0\xAD\xF8\x1F\x14a\x01\xCEW\x80c1<\xE5g\x14a\x01\xF5W`\0\x80\xFD[\x80c\x04\xBA\xA0\x0B\x14a\x01\x10W\x80c\x06\xFD\xDE\x03\x14a\x01SW\x80c\t^\xA7\xB3\x14a\x01[W\x80c\x17\xD7\x0F|\x14a\x01~W[`\0\x80\xFD[a\x01=`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\"\xA9!\x99\x18#7\xB9;\xB0\xB922\xB9`\x91\x1B\x81RP\x81V[`@Qa\x01J\x91\x90a\rWV[`@Q\x80\x91\x03\x90\xF3[a\x01=a\x02\xE7V[a\x01na\x01i6`\x04a\r\xA6V[a\x03\x9AV[`@Q\x90\x15\x15\x81R` \x01a\x01JV[a\x01\xA5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\x01JV[a\x01\xA5a\x04\x98V[a\x01na\x01\xC96`\x04a\r\xD0V[a\x05CV[a\x01\xA5\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81V[`@Q`\x12\x81R` \x01a\x01JV[a\x01=`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fv1.0.17`\xC8\x1B\x81RP\x81V[a\x01\xA5a\x0286`\x04a\x0E\x0CV[a\x06UV[a\x01\xA5a\x02K6`\x04a\x0E\x0CV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[a\x01=a\x07\nV[a\x01na\x02s6`\x04a\r\xA6V[a\x07yV[a\x02\x8Ba\x02\x866`\x04a\x0E.V[a\x08sV[\0[a\x01\xA5a\x02\x9B6`\x04a\x0E\xA1V[a\x0BAV[a\x01\xA5a\x0C\xA5V[a\x02\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01JV[`@Qb+`\x03`\xE2\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R``\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90b\xAD\x80\x0C\x90`$\x01[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\x95\x91\x90\x81\x01\x90a\x0E\xEAV[\x90P\x90V[`@Qc\x13\xB4\xB5\xAB`\xE2\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90R3`d\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90cN\xD2\xD6\xAC\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x045W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04IW=`\0\x80>=`\0\xFD[PP`@Q\x84\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x92P3\x91P\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90` \x01[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`@Qc\xBD\x85\xB09`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xBD\x85\xB09\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x1FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x95\x91\x90a\x0F\x97V[`@Qc9\x12\x02/`\xE2\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R\x83\x81\x16`D\x83\x01R`d\x82\x01\x83\x90R3`\x84\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xE4H\x08\xBC\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xE6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xFAW=`\0\x80>=`\0\xFD[PPPP\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x84`@Qa\x06C\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x93\x92PPPV[`@Qc\x1B+wa`\xE1\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c6V\xEE\xC2\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x92\x91\x90a\x0F\x97V[`@QcNA\xA1\xFB`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R``\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cNA\xA1\xFB\x90`$\x01a\x03PV[`@Qc9\x12\x02/`\xE2\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R3`$\x82\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`D\x84\x01R`d\x83\x01\x84\x90R`\x84\x83\x01\x91\x90\x91R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xE4H\x08\xBC\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x1EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x082W=`\0\x80>=`\0\xFD[PP`@Q\x84\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x92P3\x91P\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01a\x04\x86V[\x83B\x11\x15a\x08\x94W`@Qc\xF8}\x92q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x87\x16a\x08\xBBW`@Qc\xF0\xDD\x15\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R` \x81\x90R`@\x81 T\x90a\x08\xDDa\x0C\xA5V[`@\x80Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x8D\x16\x92\x82\x01\x92\x90\x92R\x90\x8A\x16``\x82\x01R`\x80\x81\x01\x89\x90R`\xA0\x81\x01\x84\x90R`\xC0\x81\x01\x88\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\tv\x92\x91\x90a\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x80\x85R\x91\x84\x01\x80\x84R\x81\x90R`\xFF\x89\x16\x92\x84\x01\x92\x90\x92R``\x83\x01\x87\x90R`\x80\x83\x01\x86\x90R\x90\x92P\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\t\xE1W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\n\x1FW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x8B\x16`\0\x81\x81R` \x81\x90R`@\x90\x81\x90 `\x01\x87\x01\x90UQc\x13\xB4\xB5\xAB`\xE2\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x8B\x83\x16`$\x82\x01R`D\x81\x01\x8B\x90R`d\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90cN\xD2\xD6\xAC\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\xD0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\xE4W=`\0\x80>=`\0\xFD[PPPP\x88`\x01`\x01`\xA0\x1B\x03\x16\x8A`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x8A`@Qa\x0B-\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPV[`@Qc\xE9\x85\xE9\xC5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x82\x81\x16`$\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xE9\x85\xE9\xC5\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xD8\x91\x90a\x0F\xB0V[\x15a\x0B\xE6WP`\0\x19a\x04\x92V[`@Qc!\xFF2\xA9`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R\x83\x81\x16`D\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c!\xFF2\xA9\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CzW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x9E\x91\x90a\x0F\x97V[\x90Pa\x04\x92V[`@\x80Q\x80\x82\x01\x82R`\x01\x81R`1`\xF8\x1B` \x91\x82\x01R\x81Q\x7F*\xEF\"\xF9\xD7\xDF_\x9D!\xC5m\x14\x02\x923\xF3\xFD\xAA\x91\x91w'\xE1\xEBh\xE5\x04\xD2pr\xD6\xCD\x81\x83\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`\0[\x83\x81\x10\x15a\rNW\x81\x81\x01Q\x83\x82\x01R` \x01a\r6V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\rv\x81`@\x85\x01` \x87\x01a\r3V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\xA1W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\r\xB9W`\0\x80\xFD[a\r\xC2\x83a\r\x8AV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\r\xE5W`\0\x80\xFD[a\r\xEE\x84a\r\x8AV[\x92Pa\r\xFC` \x85\x01a\r\x8AV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0E\x1EW`\0\x80\xFD[a\x0E'\x82a\r\x8AV[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x0EIW`\0\x80\xFD[a\x0ER\x88a\r\x8AV[\x96Pa\x0E`` \x89\x01a\r\x8AV[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0E\x84W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xB4W`\0\x80\xFD[a\x0E\xBD\x83a\r\x8AV[\x91Pa\x0E\xCB` \x84\x01a\r\x8AV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0E\xFCW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\x14W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x0F(W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x0F:Wa\x0F:a\x0E\xD4V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x0FbWa\x0Fba\x0E\xD4V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x0F{W`\0\x80\xFD[a\x0F\x8C\x83` \x83\x01` \x88\x01a\r3V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0F\xA9W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0F\xC2W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0E'W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 w\x92\x18\x15\x1E\xF0\x9C2#\xA4\xFE-]\x9B\x93!E\xA8\x06>\xCC;\xDD\x82T\x12r\x88\xFEt1\x97dsolcC\0\x08\x16\x003\xA2dipfsX\"\x12 \x17g\x11\xFAw*\xB9\x83\x03\xD4\x92q\x1B\xF9\\\xBCxi\xBB0\x96\x04\xD3j\xB7\x0F\xBA*\x14\xAB\xAA\x9BdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static ERC20FORWARDERFACTORY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x0E\xCA\xEAs\x11a\0[W\x80c\x0E\xCA\xEAs\x14a\0\xFFW\x80cT\xFDMP\x14a\x01\x12W\x80c`\x0E\xB4\xBA\x14a\x018W\x80c\xD10S\xBB\x14a\x01nW`\0\x80\xFD[\x80c\x04\xBA\xA0\x0B\x14a\0\x82W\x80c\x06\xFD\xDE\x03\x14a\0\xCCW\x80c\x07\x10\xFDX\x14a\0\xD4W[`\0\x80\xFD[a\0\xB6`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01tERC20ForwarderFactory`X\x1B\x81RP\x81V[`@Qa\0\xC3\x91\x90a\x03\xFEV[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\x01\x84V[a\0\xE7a\0\xE26`\x04a\x04MV[a\x02\x12V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC3V[a\0\xE7a\x01\r6`\x04a\x04MV[a\x02\xE7V[a\0\xB6`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fv1.0.17`\xC8\x1B\x81RP\x81V[a\x01O`\x01T`\x02T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\0\xC3V[a\x01va\x03\xC4V[`@Q\x90\x81R` \x01a\0\xC3V[`\0\x80Ta\x01\x91\x90a\x04\x85V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xBD\x90a\x04\x85V[\x80\x15a\x02\nW\x80`\x1F\x10a\x01\xDFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\nV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\xEDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R\x90\x81\x01\x82\x90R`\0\x90\x81\x90``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0`\xFF`\xF8\x1B0\x83`@Q\x80` \x01a\x02d\x90a\x03\xF1V[` \x82\x01\x81\x03\x82R`\x1F\x19`\x1F\x82\x01\x16`@RP\x80Q\x90` \x01 `@Q` \x01a\x02\xC6\x94\x93\x92\x91\x90`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84R``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x84\x01R`\x15\x83\x01R`5\x82\x01R`U\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`\x02\x81\x90U`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@\x80Q` \x81\x01\x92\x90\x92R\x81\x01\x82\x90R`\0\x90\x81\x90``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x81`@Qa\x03J\x90a\x03\xF1V[\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15a\x03jW=`\0\x80>=`\0\xFD[P\x90Pa\x03w\x85\x85a\x02\x12V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\xA7W`@Qb\xE0wU`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81\x17\x81U`\x02U\x94\x93PPPPV[`@Qa\x03\xD3` \x82\x01a\x03\xF1V[` \x82\x01\x81\x03\x82R`\x1F\x19`\x1F\x82\x01\x16`@RP\x80Q\x90` \x01 \x81V[a\x11p\x80a\x04\xC0\x839\x01\x90V[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x04,W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x04\x10V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x04`W`\0\x80\xFD[\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04wW`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x04\x99W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x04\xB9WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV\xFE`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@\x80Qc0\x07Z]`\xE1\x1B\x81R\x81Q3\x92\x83\x92c`\x0E\xB4\xBA\x92`\x04\x80\x83\x01\x93\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\0KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0o\x91\x90a\0\x84V[`\xA0R`\x01`\x01`\xA0\x1B\x03\x16`\x80RPa\0\xBEV[`\0\x80`@\x83\x85\x03\x12\x15a\0\x97W`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xAEW`\0\x80\xFD[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\x80Q`\xA0Qa\x10\x08a\x01h`\09`\0\x81\x81a\x01\x83\x01R\x81\x81a\x02\xF5\x01R\x81\x81a\x03\xA9\x01R\x81\x81a\x04\xA7\x01R\x81\x81a\x05R\x01R\x81\x81a\x06d\x01R\x81\x81a\x07\x19\x01R\x81\x81a\x07\x88\x01R\x81\x81a\nM\x01Ra\x0B\xF5\x01R`\0\x81\x81a\x02\xAD\x01R\x81\x81a\x03\x1E\x01R\x81\x81a\x03\xEF\x01R\x81\x81a\x04\xD0\x01R\x81\x81a\x05\xA0\x01R\x81\x81a\x06\x9D\x01R\x81\x81a\x07B\x01R\x81\x81a\x07\xD8\x01R\x81\x81a\n\x8A\x01R\x81\x81a\x0Bk\x01Ra\x0C3\x01Ra\x10\x08`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x0BW`\x005`\xE0\x1C\x80cT\xFDMP\x11a\0\xA2W\x80c\xA9\x05\x9C\xBB\x11a\0qW\x80c\xA9\x05\x9C\xBB\x14a\x02eW\x80c\xD5\x05\xAC\xCF\x14a\x02xW\x80c\xDDb\xED>\x14a\x02\x8DW\x80c\xF6\x98\xDA%\x14a\x02\xA0W\x80c\xFC\x0CTj\x14a\x02\xA8W`\0\x80\xFD[\x80cT\xFDMP\x14a\x02\x04W\x80cp\xA0\x821\x14a\x02*W\x80c~\xCE\xBE\0\x14a\x02=W\x80c\x95\xD8\x9BA\x14a\x02]W`\0\x80\xFD[\x80c\x18\x16\r\xDD\x11a\0\xDEW\x80c\x18\x16\r\xDD\x14a\x01\xB3W\x80c#\xB8r\xDD\x14a\x01\xBBW\x80c0\xAD\xF8\x1F\x14a\x01\xCEW\x80c1<\xE5g\x14a\x01\xF5W`\0\x80\xFD[\x80c\x04\xBA\xA0\x0B\x14a\x01\x10W\x80c\x06\xFD\xDE\x03\x14a\x01SW\x80c\t^\xA7\xB3\x14a\x01[W\x80c\x17\xD7\x0F|\x14a\x01~W[`\0\x80\xFD[a\x01=`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\"\xA9!\x99\x18#7\xB9;\xB0\xB922\xB9`\x91\x1B\x81RP\x81V[`@Qa\x01J\x91\x90a\rWV[`@Q\x80\x91\x03\x90\xF3[a\x01=a\x02\xE7V[a\x01na\x01i6`\x04a\r\xA6V[a\x03\x9AV[`@Q\x90\x15\x15\x81R` \x01a\x01JV[a\x01\xA5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\x01JV[a\x01\xA5a\x04\x98V[a\x01na\x01\xC96`\x04a\r\xD0V[a\x05CV[a\x01\xA5\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81V[`@Q`\x12\x81R` \x01a\x01JV[a\x01=`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fv1.0.17`\xC8\x1B\x81RP\x81V[a\x01\xA5a\x0286`\x04a\x0E\x0CV[a\x06UV[a\x01\xA5a\x02K6`\x04a\x0E\x0CV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[a\x01=a\x07\nV[a\x01na\x02s6`\x04a\r\xA6V[a\x07yV[a\x02\x8Ba\x02\x866`\x04a\x0E.V[a\x08sV[\0[a\x01\xA5a\x02\x9B6`\x04a\x0E\xA1V[a\x0BAV[a\x01\xA5a\x0C\xA5V[a\x02\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01JV[`@Qb+`\x03`\xE2\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R``\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90b\xAD\x80\x0C\x90`$\x01[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\x95\x91\x90\x81\x01\x90a\x0E\xEAV[\x90P\x90V[`@Qc\x13\xB4\xB5\xAB`\xE2\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90R3`d\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90cN\xD2\xD6\xAC\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x045W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04IW=`\0\x80>=`\0\xFD[PP`@Q\x84\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x92P3\x91P\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90` \x01[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`@Qc\xBD\x85\xB09`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xBD\x85\xB09\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x1FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x95\x91\x90a\x0F\x97V[`@Qc9\x12\x02/`\xE2\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R\x83\x81\x16`D\x83\x01R`d\x82\x01\x83\x90R3`\x84\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xE4H\x08\xBC\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xE6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xFAW=`\0\x80>=`\0\xFD[PPPP\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x84`@Qa\x06C\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x93\x92PPPV[`@Qc\x1B+wa`\xE1\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c6V\xEE\xC2\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x92\x91\x90a\x0F\x97V[`@QcNA\xA1\xFB`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R``\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cNA\xA1\xFB\x90`$\x01a\x03PV[`@Qc9\x12\x02/`\xE2\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R3`$\x82\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`D\x84\x01R`d\x83\x01\x84\x90R`\x84\x83\x01\x91\x90\x91R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xE4H\x08\xBC\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x1EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x082W=`\0\x80>=`\0\xFD[PP`@Q\x84\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x92P3\x91P\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01a\x04\x86V[\x83B\x11\x15a\x08\x94W`@Qc\xF8}\x92q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x87\x16a\x08\xBBW`@Qc\xF0\xDD\x15\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R` \x81\x90R`@\x81 T\x90a\x08\xDDa\x0C\xA5V[`@\x80Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x8D\x16\x92\x82\x01\x92\x90\x92R\x90\x8A\x16``\x82\x01R`\x80\x81\x01\x89\x90R`\xA0\x81\x01\x84\x90R`\xC0\x81\x01\x88\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\tv\x92\x91\x90a\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x80\x85R\x91\x84\x01\x80\x84R\x81\x90R`\xFF\x89\x16\x92\x84\x01\x92\x90\x92R``\x83\x01\x87\x90R`\x80\x83\x01\x86\x90R\x90\x92P\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\t\xE1W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\n\x1FW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x8B\x16`\0\x81\x81R` \x81\x90R`@\x90\x81\x90 `\x01\x87\x01\x90UQc\x13\xB4\xB5\xAB`\xE2\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x8B\x83\x16`$\x82\x01R`D\x81\x01\x8B\x90R`d\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90cN\xD2\xD6\xAC\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\xD0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\xE4W=`\0\x80>=`\0\xFD[PPPP\x88`\x01`\x01`\xA0\x1B\x03\x16\x8A`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x8A`@Qa\x0B-\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPV[`@Qc\xE9\x85\xE9\xC5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x82\x81\x16`$\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xE9\x85\xE9\xC5\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xD8\x91\x90a\x0F\xB0V[\x15a\x0B\xE6WP`\0\x19a\x04\x92V[`@Qc!\xFF2\xA9`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R\x83\x81\x16`D\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c!\xFF2\xA9\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CzW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x9E\x91\x90a\x0F\x97V[\x90Pa\x04\x92V[`@\x80Q\x80\x82\x01\x82R`\x01\x81R`1`\xF8\x1B` \x91\x82\x01R\x81Q\x7F*\xEF\"\xF9\xD7\xDF_\x9D!\xC5m\x14\x02\x923\xF3\xFD\xAA\x91\x91w'\xE1\xEBh\xE5\x04\xD2pr\xD6\xCD\x81\x83\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`\0[\x83\x81\x10\x15a\rNW\x81\x81\x01Q\x83\x82\x01R` \x01a\r6V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\rv\x81`@\x85\x01` \x87\x01a\r3V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\xA1W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\r\xB9W`\0\x80\xFD[a\r\xC2\x83a\r\x8AV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\r\xE5W`\0\x80\xFD[a\r\xEE\x84a\r\x8AV[\x92Pa\r\xFC` \x85\x01a\r\x8AV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0E\x1EW`\0\x80\xFD[a\x0E'\x82a\r\x8AV[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x0EIW`\0\x80\xFD[a\x0ER\x88a\r\x8AV[\x96Pa\x0E`` \x89\x01a\r\x8AV[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0E\x84W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xB4W`\0\x80\xFD[a\x0E\xBD\x83a\r\x8AV[\x91Pa\x0E\xCB` \x84\x01a\r\x8AV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0E\xFCW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\x14W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x0F(W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x0F:Wa\x0F:a\x0E\xD4V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x0FbWa\x0Fba\x0E\xD4V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x0F{W`\0\x80\xFD[a\x0F\x8C\x83` \x83\x01` \x88\x01a\r3V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0F\xA9W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0F\xC2W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0E'W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 w\x92\x18\x15\x1E\xF0\x9C2#\xA4\xFE-]\x9B\x93!E\xA8\x06>\xCC;\xDD\x82T\x12r\x88\xFEt1\x97dsolcC\0\x08\x16\x003\xA2dipfsX\"\x12 \x17g\x11\xFAw*\xB9\x83\x03\xD4\x92q\x1B\xF9\\\xBCxi\xBB0\x96\x04\xD3j\xB7\x0F\xBA*\x14\xAB\xAA\x9BdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static ERC20FORWARDERFACTORY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ERC20ForwarderFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC20ForwarderFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC20ForwarderFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC20ForwarderFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC20ForwarderFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ERC20ForwarderFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC20ForwarderFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC20FORWARDERFACTORY_ABI.clone(),
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
                ERC20FORWARDERFACTORY_ABI.clone(),
                ERC20FORWARDERFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ERC20LINK_HASH` (0xd13053bb) function
        pub fn erc20link_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([209, 48, 83, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `create` (0x0ecaea73) function
        pub fn create(
            &self,
            token: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([14, 202, 234, 115], (token, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDeployDetails` (0x600eb4ba) function
        pub fn get_deploy_details(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([96, 14, 180, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getForwarder` (0x0710fd58) function
        pub fn get_forwarder(
            &self,
            token: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([7, 16, 253, 88], (token, token_id))
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
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC20ForwarderFactory<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidForwarderAddress` with signature `InvalidForwarderAddress()` and selector `0x381dd540`
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
    #[etherror(name = "InvalidForwarderAddress", abi = "InvalidForwarderAddress()")]
    pub struct InvalidForwarderAddress;
    ///Container type for all input parameters for the `ERC20LINK_HASH` function with signature `ERC20LINK_HASH()` and selector `0xd13053bb`
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
    #[ethcall(name = "ERC20LINK_HASH", abi = "ERC20LINK_HASH()")]
    pub struct Erc20LinkHashCall;
    ///Container type for all input parameters for the `create` function with signature `create(address,uint256)` and selector `0x0ecaea73`
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
    #[ethcall(name = "create", abi = "create(address,uint256)")]
    pub struct CreateCall {
        pub token: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getDeployDetails` function with signature `getDeployDetails()` and selector `0x600eb4ba`
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
    #[ethcall(name = "getDeployDetails", abi = "getDeployDetails()")]
    pub struct GetDeployDetailsCall;
    ///Container type for all input parameters for the `getForwarder` function with signature `getForwarder(address,uint256)` and selector `0x0710fd58`
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
    #[ethcall(name = "getForwarder", abi = "getForwarder(address,uint256)")]
    pub struct GetForwarderCall {
        pub token: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
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
    pub enum ERC20ForwarderFactoryCalls {
        Erc20LinkHash(Erc20LinkHashCall),
        Create(CreateCall),
        GetDeployDetails(GetDeployDetailsCall),
        GetForwarder(GetForwarderCall),
        Kind(KindCall),
        Name(NameCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for ERC20ForwarderFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Erc20LinkHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Erc20LinkHash(decoded));
            }
            if let Ok(decoded) = <CreateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Create(decoded));
            }
            if let Ok(decoded) = <GetDeployDetailsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDeployDetails(decoded));
            }
            if let Ok(decoded) = <GetForwarderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetForwarder(decoded));
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
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC20ForwarderFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Erc20LinkHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Create(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDeployDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetForwarder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Kind(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ERC20ForwarderFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Erc20LinkHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::Create(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDeployDetails(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetForwarder(element) => ::core::fmt::Display::fmt(element, f),
                Self::Kind(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Erc20LinkHashCall> for ERC20ForwarderFactoryCalls {
        fn from(value: Erc20LinkHashCall) -> Self {
            Self::Erc20LinkHash(value)
        }
    }
    impl ::core::convert::From<CreateCall> for ERC20ForwarderFactoryCalls {
        fn from(value: CreateCall) -> Self {
            Self::Create(value)
        }
    }
    impl ::core::convert::From<GetDeployDetailsCall> for ERC20ForwarderFactoryCalls {
        fn from(value: GetDeployDetailsCall) -> Self {
            Self::GetDeployDetails(value)
        }
    }
    impl ::core::convert::From<GetForwarderCall> for ERC20ForwarderFactoryCalls {
        fn from(value: GetForwarderCall) -> Self {
            Self::GetForwarder(value)
        }
    }
    impl ::core::convert::From<KindCall> for ERC20ForwarderFactoryCalls {
        fn from(value: KindCall) -> Self {
            Self::Kind(value)
        }
    }
    impl ::core::convert::From<NameCall> for ERC20ForwarderFactoryCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<VersionCall> for ERC20ForwarderFactoryCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `ERC20LINK_HASH` function with signature `ERC20LINK_HASH()` and selector `0xd13053bb`
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
    pub struct Erc20LinkHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `create` function with signature `create(address,uint256)` and selector `0x0ecaea73`
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
    pub struct CreateReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDeployDetails` function with signature `getDeployDetails()` and selector `0x600eb4ba`
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
    pub struct GetDeployDetailsReturn(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getForwarder` function with signature `getForwarder(address,uint256)` and selector `0x0710fd58`
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
    pub struct GetForwarderReturn(pub ::ethers::core::types::Address);
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
}
