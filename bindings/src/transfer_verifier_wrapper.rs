pub use transfer_verifier_wrapper::*;
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
pub mod transfer_verifier_wrapper {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_iverifier"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract i_transfer_verifier",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_sampleInput"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_sampleProof"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("checkSampleInputsAndProof"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "checkSampleInputsAndProof",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("createRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createRequest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ask"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ProofMarketPlace.Ask",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("secretType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ProofMarketPlace.SecretType",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("secret_inputs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("acl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "encodeInputAndProofForVerification",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "encodeInputAndProofForVerification",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("inputs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        5usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[5]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        8usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[8]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeInputs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("encodeInputs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("inputs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        5usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[5]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeProof"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("encodeProof"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        8usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[8]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("iverifier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("iverifier"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract i_transfer_verifier",
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
                    ::std::borrow::ToOwned::to_owned("proofMarketPlace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proofMarketPlace"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ProofMarketPlace",
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
                    ::std::borrow::ToOwned::to_owned("sampleInput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sampleInput"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sampleProof"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sampleProof"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setProofMarketPlaceContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setProofMarketPlaceContract",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proofMarketplace"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ProofMarketPlace",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("verify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("encodedData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("verifyAgainstSampleInputs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verifyAgainstSampleInputs",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("encodedProof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("verifyInputs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifyInputs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("inputs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static TRANSFER_VERIFIER_WRAPPER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x13\x1D8\x03\x80b\0\x13\x1D\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01-V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\x80R`\x01b\0\0O\x83\x82b\0\x02FV[P`\x02b\0\0^\x82\x82b\0\x02FV[PPPPb\0\x03\x12V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12b\0\0\x90W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\0\xADWb\0\0\xADb\0\0hV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\0\xD8Wb\0\0\xD8b\0\0hV[\x81`@R\x83\x81R` \x92P\x86\x83\x85\x88\x01\x01\x11\x15b\0\0\xF5W`\0\x80\xFD[`\0\x91P[\x83\x82\x10\x15b\0\x01\x19W\x85\x82\x01\x83\x01Q\x81\x83\x01\x84\x01R\x90\x82\x01\x90b\0\0\xFAV[`\0\x93\x81\x01\x90\x92\x01\x92\x90\x92R\x94\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01CW`\0\x80\xFD[\x83Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01[W`\0\x80\xFD[` \x85\x01Q\x90\x93P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x01yW`\0\x80\xFD[b\0\x01\x87\x87\x83\x88\x01b\0\0~V[\x93P`@\x86\x01Q\x91P\x80\x82\x11\x15b\0\x01\x9EW`\0\x80\xFD[Pb\0\x01\xAD\x86\x82\x87\x01b\0\0~V[\x91PP\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x01\xCCW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x01\xEDWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x02AW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x02\x1CWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x02=W\x82\x81U`\x01\x01b\0\x02(V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x02bWb\0\x02bb\0\0hV[b\0\x02z\x81b\0\x02s\x84Tb\0\x01\xB7V[\x84b\0\x01\xF3V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x02\xB2W`\0\x84\x15b\0\x02\x99WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x02=V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x02\xE3W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x02\xC2V[P\x85\x82\x10\x15b\0\x03\x02W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Qa\x0F\xE8b\0\x035`\09`\0\x81\x81a\x01\xD8\x01Ra\x03\xF4\x01Ra\x0F\xE8`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x99fY\x9F\x11a\0\x8CW\x80c\xC4o\xC3\xEF\x11a\0fW\x80c\xC4o\xC3\xEF\x14a\x01\x98W\x80c\xD2#^\xAC\x14a\x01\xADW\x80c\xDF\xD4\xAC\x1B\x14a\x01\xC0W\x80c\xE7\xF5\xB8\x1D\x14a\x01\xD3W`\0\x80\xFD[\x80c\x99fY\x9F\x14a\x01jW\x80c\xA6\xDF\xBC\x7F\x14a\x01}W\x80c\xA7l\x05Q\x14a\x01\x90W`\0\x80\xFD[\x80c\x02\xF7}\x19\x14a\0\xD4W\x80c\x10\xA5By\x14a\0\xFCW\x80cd\xF1\xBC\xC7\x14a\x01\x04W\x80cs*\x9Dc\x14a\x01$W\x80c}\x8A\xD4+\x14a\x01OW\x80c\x8Ev\n\xFE\x14a\x01WW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x08\x06V[a\x01\xFAV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xE7a\x022V[a\x01\x17a\x01\x126`\x04a\x08\xD7V[a\x02\xCCV[`@Qa\0\xF3\x91\x90a\tDV[`\0Ta\x017\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF3V[a\x01\x17a\x02\xF5V[a\0\xE7a\x01e6`\x04a\x08\x06V[a\x03\x83V[a\x01\x17a\x01x6`\x04a\t\x82V[a\x04vV[a\0\xE7a\x01\x8B6`\x04a\n\x01V[a\x04\xB3V[a\x01\x17a\x04\xCBV[a\x01\xABa\x01\xA66`\x04a\n[V[a\x04\xD8V[\0[a\x01\xABa\x01\xBB6`\x04a\nxV[a\x05FV[a\x01\x17a\x01\xCE6`\x04a\x0B-V[a\x06xV[a\x017\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80`\x01\x83`@Q` \x01a\x02\x11\x92\x91\x90a\x0B\x83V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x02+\x81a\x03\x83V[\x93\x92PPPV[`\0a\x02\xC7`\x02\x80Ta\x02D\x90a\x0BIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02p\x90a\x0BIV[\x80\x15a\x02\xBDW\x80`\x1F\x10a\x02\x92Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xBDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xA0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x01\xFAV[\x90P\x90V[``\x81`@Q` \x01a\x02\xDF\x91\x90a\x0ClV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\x01\x80Ta\x03\x02\x90a\x0BIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03.\x90a\x0BIV[\x80\x15a\x03{W\x80`\x1F\x10a\x03PWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03{V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03^W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0a\x03\x8Da\x07\rV[a\x03\x95a\x07+V[`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x03\xAC\x91\x90a\x0C\xC8V[\x91P\x91P\x81\x80` \x01\x90Q\x81\x01\x90a\x03\xC4\x91\x90a\r,V[\x93P\x80\x80` \x01\x90Q\x81\x01\x90a\x03\xDA\x91\x90a\r\x81V[`@QchDM\xC7`\xE0\x1B\x81R\x90\x93P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90chDM\xC7\x90a\x04+\x90\x87\x90\x87\x90`\x04\x01a\x0E\x05V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04HW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04l\x91\x90a\x0E!V[\x96\x95PPPPPPV[``a\x04\x81\x83a\x06xV[a\x04\x8A\x83a\x02\xCCV[`@Q` \x01a\x04\x9B\x92\x91\x90a\x0ECV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[\x92\x91PPV[`\0a\x04\xC1\x82\x84\x01\x84a\x0B-V[P`\x01\x93\x92PPPV[`\x02\x80Ta\x03\x02\x90a\x0BIV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x05$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x10[\x1C\x99XY\x1EH\x14\xD9]`\xAA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0`@Q\x80`\xE0\x01`@R\x80\x88`\0\x015\x81R` \x01\x88` \x015\x81R` \x01\x88`@\x015\x81R` \x01\x88``\x015\x81R` \x01\x88`\x80\x015\x81R` \x01\x88`\xA0\x01` \x81\x01\x90a\x05\x98\x91\x90a\n[V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x05\xBEa\x01\xCEa\x05\xB9`\xC0\x8C\x01\x8Ca\x0EhV[a\x06\x8BV[\x90R`\0T`@Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90cpS\x8F\xCA\x90\x83\x90\x89\x90a\x05\xED\x90\x8A\x90\x8A\x90` \x01a\x0E\xAFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x87\x87`@Q` \x01a\x06\x0F\x92\x91\x90a\x0E\xAFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06=\x94\x93\x92\x91\x90a\x0F\0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06WW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06kW=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[``\x81`@Q` \x01a\x02\xDF\x91\x90a\x0F\xA4V[a\x06\x93a\x07\rV[a\x06\x9D\x83\x83a\x04\xB3V[a\x07\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FTransfer Verifier Wrapper: Inval`D\x82\x01Rn\x1AY\x08\x1A[\x9C\x1D]\x08\x19\x9B\xDC\x9BX]`\x8A\x1B`d\x82\x01R`\x84\x01a\x05\x1BV[a\x02+\x82\x84\x01\x84a\x0B-V[`@Q\x80`\xA0\x01`@R\x80`\x05\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01\0\x01`@R\x80`\x08\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x84Wa\x07\x84a\x07JV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x84Wa\x07\x84a\x07JV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xD6Wa\x07\xD6a\x07JV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\xF8Wa\x07\xF8a\x07JV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15a\x08\x18W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08/W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x08@W`\0\x80\xFD[\x805a\x08Sa\x08N\x82a\x07\xDEV[a\x07\xADV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x08hW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a\x08\x97W`\0\x80\xFD[a\x08\x9Fa\x07`V[\x80a\x01\0\x84\x01\x85\x81\x11\x15a\x08\xB2W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x08\xCCW\x805\x84R` \x93\x84\x01\x93\x01a\x08\xB4V[P\x90\x95\x94PPPPPV[`\0a\x01\0\x82\x84\x03\x12\x15a\x08\xEAW`\0\x80\xFD[a\x02+\x83\x83a\x08\x86V[`\0[\x83\x81\x10\x15a\t\x0FW\x81\x81\x01Q\x83\x82\x01R` \x01a\x08\xF7V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\t0\x81` \x86\x01` \x86\x01a\x08\xF4V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x02+` \x83\x01\x84a\t\x18V[`\0\x82`\x1F\x83\x01\x12a\thW`\0\x80\xFD[a\tpa\x07\x8AV[\x80`\xA0\x84\x01\x85\x81\x11\x15a\x08\xB2W`\0\x80\xFD[`\0\x80a\x01\xA0\x83\x85\x03\x12\x15a\t\x96W`\0\x80\xFD[a\t\xA0\x84\x84a\tWV[\x91Pa\t\xAF\x84`\xA0\x85\x01a\x08\x86V[\x90P\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\t\xCAW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xE2W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\t\xFAW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\n\x14W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n+W`\0\x80\xFD[a\n7\x85\x82\x86\x01a\t\xB8V[\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\nXW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\nmW`\0\x80\xFD[\x815a\x02+\x81a\nCV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\n\x91W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xA9W`\0\x80\xFD[\x90\x88\x01\x90`\xE0\x82\x8B\x03\x12\x15a\n\xBDW`\0\x80\xFD[\x90\x96P` \x88\x015\x90`\x03\x82\x10a\n\xD3W`\0\x80\xFD[\x90\x95P`@\x88\x015\x90\x80\x82\x11\x15a\n\xE9W`\0\x80\xFD[a\n\xF5\x8A\x83\x8B\x01a\t\xB8V[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15a\x0B\x0EW`\0\x80\xFD[Pa\x0B\x1B\x89\x82\x8A\x01a\t\xB8V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0`\xA0\x82\x84\x03\x12\x15a\x0B?W`\0\x80\xFD[a\x02+\x83\x83a\tWV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0B]W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0B}WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`@\x81R`\0\x80\x84T\x81`\x01\x82\x81\x1C\x91P\x80\x83\x16\x80a\x0B\xA3W`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03a\x0B\xC2WcNH{q`\xE0\x1B\x86R`\"`\x04R`$\x86\xFD[`@\x88\x01\x84\x90R``\x88\x01\x82\x80\x15a\x0B\xE1W`\x01\x81\x14a\x0B\xF7Wa\x0C\"V[`\xFF\x19\x87\x16\x82R\x85\x15\x15`\x05\x1B\x82\x01\x97Pa\x0C\"V[`\0\x8C\x81R` \x90 `\0[\x87\x81\x10\x15a\x0C\x1CW\x81T\x84\x82\x01R\x90\x86\x01\x90\x84\x01a\x0C\x03V[\x83\x01\x98PP[PP\x87\x86\x03\x81\x89\x01RPPPPPa\x0C:\x81\x85a\t\x18V[\x95\x94PPPPPV[\x80`\0[`\x08\x81\x10\x15a\x0CfW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x0CGV[PPPPV[a\x01\0\x81\x01a\x04\xAD\x82\x84a\x0CCV[`\0\x82`\x1F\x83\x01\x12a\x0C\x8CW`\0\x80\xFD[\x81Qa\x0C\x9Aa\x08N\x82a\x07\xDEV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0C\xAFW`\0\x80\xFD[a\x0C\xC0\x82` \x83\x01` \x87\x01a\x08\xF4V[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xDBW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\xF3W`\0\x80\xFD[a\x0C\xFF\x86\x83\x87\x01a\x0C{V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\r\x15W`\0\x80\xFD[Pa\r\"\x85\x82\x86\x01a\x0C{V[\x91PP\x92P\x92\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a\r>W`\0\x80\xFD[\x82`\x1F\x83\x01\x12a\rMW`\0\x80\xFD[a\rUa\x07\x8AV[\x80`\xA0\x84\x01\x85\x81\x11\x15a\rgW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x08\xCCW\x80Q\x84R` \x93\x84\x01\x93\x01a\riV[`\0a\x01\0\x80\x83\x85\x03\x12\x15a\r\x95W`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\r\xA4W`\0\x80\xFD[a\r\xACa\x07`V[\x90\x83\x01\x90\x80\x85\x83\x11\x15a\r\xBEW`\0\x80\xFD[\x84[\x83\x81\x10\x15a\r\xD8W\x80Q\x83R` \x92\x83\x01\x92\x01a\r\xC0V[P\x95\x94PPPPPV[\x80`\0[`\x05\x81\x10\x15a\x0CfW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\r\xE6V[a\x01\xA0\x81\x01a\x0E\x14\x82\x85a\r\xE2V[a\x02+`\xA0\x83\x01\x84a\x0CCV[`\0` \x82\x84\x03\x12\x15a\x0E3W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02+W`\0\x80\xFD[`@\x81R`\0a\x0EV`@\x83\x01\x85a\t\x18V[\x82\x81\x03` \x84\x01Ra\x0C:\x81\x85a\t\x18V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0E\x7FW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\x9AW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\t\xFAW`\0\x80\xFD[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\x03\x81\x10a\x0E\xFCWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`\x80\x81R\x84Q`\x80\x82\x01R` \x85\x01Q`\xA0\x82\x01R`@\x85\x01Q`\xC0\x82\x01R``\x85\x01Q`\xE0\x82\x01R`\x80\x85\x01Qa\x01\0\x82\x01R`\x01\x80`\xA0\x1B\x03`\xA0\x86\x01Q\x16a\x01 \x82\x01R`\0`\xC0\x86\x01Q`\xE0a\x01@\x84\x01Ra\x0Fda\x01`\x84\x01\x82a\t\x18V[\x90Pa\x0Fs` \x84\x01\x87a\x0E\xDEV[\x82\x81\x03`@\x84\x01Ra\x0F\x85\x81\x86a\t\x18V[\x90P\x82\x81\x03``\x84\x01Ra\x0F\x99\x81\x85a\t\x18V[\x97\x96PPPPPPPV[`\xA0\x81\x01a\x04\xAD\x82\x84a\r\xE2V\xFE\xA2dipfsX\"\x12 LNx\xDC\xD6\x1F[Ht\xC6m\xB9\xDD\xAB\xFBe\xDBZA\xA4W\xF0\t\xEA\x14\x0FqD\xAF\x8E\x02\x8AdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static TRANSFER_VERIFIER_WRAPPER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x99fY\x9F\x11a\0\x8CW\x80c\xC4o\xC3\xEF\x11a\0fW\x80c\xC4o\xC3\xEF\x14a\x01\x98W\x80c\xD2#^\xAC\x14a\x01\xADW\x80c\xDF\xD4\xAC\x1B\x14a\x01\xC0W\x80c\xE7\xF5\xB8\x1D\x14a\x01\xD3W`\0\x80\xFD[\x80c\x99fY\x9F\x14a\x01jW\x80c\xA6\xDF\xBC\x7F\x14a\x01}W\x80c\xA7l\x05Q\x14a\x01\x90W`\0\x80\xFD[\x80c\x02\xF7}\x19\x14a\0\xD4W\x80c\x10\xA5By\x14a\0\xFCW\x80cd\xF1\xBC\xC7\x14a\x01\x04W\x80cs*\x9Dc\x14a\x01$W\x80c}\x8A\xD4+\x14a\x01OW\x80c\x8Ev\n\xFE\x14a\x01WW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x08\x06V[a\x01\xFAV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xE7a\x022V[a\x01\x17a\x01\x126`\x04a\x08\xD7V[a\x02\xCCV[`@Qa\0\xF3\x91\x90a\tDV[`\0Ta\x017\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF3V[a\x01\x17a\x02\xF5V[a\0\xE7a\x01e6`\x04a\x08\x06V[a\x03\x83V[a\x01\x17a\x01x6`\x04a\t\x82V[a\x04vV[a\0\xE7a\x01\x8B6`\x04a\n\x01V[a\x04\xB3V[a\x01\x17a\x04\xCBV[a\x01\xABa\x01\xA66`\x04a\n[V[a\x04\xD8V[\0[a\x01\xABa\x01\xBB6`\x04a\nxV[a\x05FV[a\x01\x17a\x01\xCE6`\x04a\x0B-V[a\x06xV[a\x017\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80`\x01\x83`@Q` \x01a\x02\x11\x92\x91\x90a\x0B\x83V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x02+\x81a\x03\x83V[\x93\x92PPPV[`\0a\x02\xC7`\x02\x80Ta\x02D\x90a\x0BIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02p\x90a\x0BIV[\x80\x15a\x02\xBDW\x80`\x1F\x10a\x02\x92Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xBDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xA0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x01\xFAV[\x90P\x90V[``\x81`@Q` \x01a\x02\xDF\x91\x90a\x0ClV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\x01\x80Ta\x03\x02\x90a\x0BIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03.\x90a\x0BIV[\x80\x15a\x03{W\x80`\x1F\x10a\x03PWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03{V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03^W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0a\x03\x8Da\x07\rV[a\x03\x95a\x07+V[`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x03\xAC\x91\x90a\x0C\xC8V[\x91P\x91P\x81\x80` \x01\x90Q\x81\x01\x90a\x03\xC4\x91\x90a\r,V[\x93P\x80\x80` \x01\x90Q\x81\x01\x90a\x03\xDA\x91\x90a\r\x81V[`@QchDM\xC7`\xE0\x1B\x81R\x90\x93P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90chDM\xC7\x90a\x04+\x90\x87\x90\x87\x90`\x04\x01a\x0E\x05V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04HW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04l\x91\x90a\x0E!V[\x96\x95PPPPPPV[``a\x04\x81\x83a\x06xV[a\x04\x8A\x83a\x02\xCCV[`@Q` \x01a\x04\x9B\x92\x91\x90a\x0ECV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[\x92\x91PPV[`\0a\x04\xC1\x82\x84\x01\x84a\x0B-V[P`\x01\x93\x92PPPV[`\x02\x80Ta\x03\x02\x90a\x0BIV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x05$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x10[\x1C\x99XY\x1EH\x14\xD9]`\xAA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0`@Q\x80`\xE0\x01`@R\x80\x88`\0\x015\x81R` \x01\x88` \x015\x81R` \x01\x88`@\x015\x81R` \x01\x88``\x015\x81R` \x01\x88`\x80\x015\x81R` \x01\x88`\xA0\x01` \x81\x01\x90a\x05\x98\x91\x90a\n[V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x05\xBEa\x01\xCEa\x05\xB9`\xC0\x8C\x01\x8Ca\x0EhV[a\x06\x8BV[\x90R`\0T`@Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90cpS\x8F\xCA\x90\x83\x90\x89\x90a\x05\xED\x90\x8A\x90\x8A\x90` \x01a\x0E\xAFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x87\x87`@Q` \x01a\x06\x0F\x92\x91\x90a\x0E\xAFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06=\x94\x93\x92\x91\x90a\x0F\0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06WW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06kW=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[``\x81`@Q` \x01a\x02\xDF\x91\x90a\x0F\xA4V[a\x06\x93a\x07\rV[a\x06\x9D\x83\x83a\x04\xB3V[a\x07\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FTransfer Verifier Wrapper: Inval`D\x82\x01Rn\x1AY\x08\x1A[\x9C\x1D]\x08\x19\x9B\xDC\x9BX]`\x8A\x1B`d\x82\x01R`\x84\x01a\x05\x1BV[a\x02+\x82\x84\x01\x84a\x0B-V[`@Q\x80`\xA0\x01`@R\x80`\x05\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01\0\x01`@R\x80`\x08\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x84Wa\x07\x84a\x07JV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x84Wa\x07\x84a\x07JV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xD6Wa\x07\xD6a\x07JV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\xF8Wa\x07\xF8a\x07JV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15a\x08\x18W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08/W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x08@W`\0\x80\xFD[\x805a\x08Sa\x08N\x82a\x07\xDEV[a\x07\xADV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x08hW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a\x08\x97W`\0\x80\xFD[a\x08\x9Fa\x07`V[\x80a\x01\0\x84\x01\x85\x81\x11\x15a\x08\xB2W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x08\xCCW\x805\x84R` \x93\x84\x01\x93\x01a\x08\xB4V[P\x90\x95\x94PPPPPV[`\0a\x01\0\x82\x84\x03\x12\x15a\x08\xEAW`\0\x80\xFD[a\x02+\x83\x83a\x08\x86V[`\0[\x83\x81\x10\x15a\t\x0FW\x81\x81\x01Q\x83\x82\x01R` \x01a\x08\xF7V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\t0\x81` \x86\x01` \x86\x01a\x08\xF4V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x02+` \x83\x01\x84a\t\x18V[`\0\x82`\x1F\x83\x01\x12a\thW`\0\x80\xFD[a\tpa\x07\x8AV[\x80`\xA0\x84\x01\x85\x81\x11\x15a\x08\xB2W`\0\x80\xFD[`\0\x80a\x01\xA0\x83\x85\x03\x12\x15a\t\x96W`\0\x80\xFD[a\t\xA0\x84\x84a\tWV[\x91Pa\t\xAF\x84`\xA0\x85\x01a\x08\x86V[\x90P\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\t\xCAW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xE2W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\t\xFAW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\n\x14W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n+W`\0\x80\xFD[a\n7\x85\x82\x86\x01a\t\xB8V[\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\nXW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\nmW`\0\x80\xFD[\x815a\x02+\x81a\nCV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\n\x91W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xA9W`\0\x80\xFD[\x90\x88\x01\x90`\xE0\x82\x8B\x03\x12\x15a\n\xBDW`\0\x80\xFD[\x90\x96P` \x88\x015\x90`\x03\x82\x10a\n\xD3W`\0\x80\xFD[\x90\x95P`@\x88\x015\x90\x80\x82\x11\x15a\n\xE9W`\0\x80\xFD[a\n\xF5\x8A\x83\x8B\x01a\t\xB8V[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15a\x0B\x0EW`\0\x80\xFD[Pa\x0B\x1B\x89\x82\x8A\x01a\t\xB8V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0`\xA0\x82\x84\x03\x12\x15a\x0B?W`\0\x80\xFD[a\x02+\x83\x83a\tWV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0B]W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0B}WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`@\x81R`\0\x80\x84T\x81`\x01\x82\x81\x1C\x91P\x80\x83\x16\x80a\x0B\xA3W`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03a\x0B\xC2WcNH{q`\xE0\x1B\x86R`\"`\x04R`$\x86\xFD[`@\x88\x01\x84\x90R``\x88\x01\x82\x80\x15a\x0B\xE1W`\x01\x81\x14a\x0B\xF7Wa\x0C\"V[`\xFF\x19\x87\x16\x82R\x85\x15\x15`\x05\x1B\x82\x01\x97Pa\x0C\"V[`\0\x8C\x81R` \x90 `\0[\x87\x81\x10\x15a\x0C\x1CW\x81T\x84\x82\x01R\x90\x86\x01\x90\x84\x01a\x0C\x03V[\x83\x01\x98PP[PP\x87\x86\x03\x81\x89\x01RPPPPPa\x0C:\x81\x85a\t\x18V[\x95\x94PPPPPV[\x80`\0[`\x08\x81\x10\x15a\x0CfW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x0CGV[PPPPV[a\x01\0\x81\x01a\x04\xAD\x82\x84a\x0CCV[`\0\x82`\x1F\x83\x01\x12a\x0C\x8CW`\0\x80\xFD[\x81Qa\x0C\x9Aa\x08N\x82a\x07\xDEV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0C\xAFW`\0\x80\xFD[a\x0C\xC0\x82` \x83\x01` \x87\x01a\x08\xF4V[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xDBW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\xF3W`\0\x80\xFD[a\x0C\xFF\x86\x83\x87\x01a\x0C{V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\r\x15W`\0\x80\xFD[Pa\r\"\x85\x82\x86\x01a\x0C{V[\x91PP\x92P\x92\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a\r>W`\0\x80\xFD[\x82`\x1F\x83\x01\x12a\rMW`\0\x80\xFD[a\rUa\x07\x8AV[\x80`\xA0\x84\x01\x85\x81\x11\x15a\rgW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x08\xCCW\x80Q\x84R` \x93\x84\x01\x93\x01a\riV[`\0a\x01\0\x80\x83\x85\x03\x12\x15a\r\x95W`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\r\xA4W`\0\x80\xFD[a\r\xACa\x07`V[\x90\x83\x01\x90\x80\x85\x83\x11\x15a\r\xBEW`\0\x80\xFD[\x84[\x83\x81\x10\x15a\r\xD8W\x80Q\x83R` \x92\x83\x01\x92\x01a\r\xC0V[P\x95\x94PPPPPV[\x80`\0[`\x05\x81\x10\x15a\x0CfW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\r\xE6V[a\x01\xA0\x81\x01a\x0E\x14\x82\x85a\r\xE2V[a\x02+`\xA0\x83\x01\x84a\x0CCV[`\0` \x82\x84\x03\x12\x15a\x0E3W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02+W`\0\x80\xFD[`@\x81R`\0a\x0EV`@\x83\x01\x85a\t\x18V[\x82\x81\x03` \x84\x01Ra\x0C:\x81\x85a\t\x18V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0E\x7FW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\x9AW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\t\xFAW`\0\x80\xFD[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\x03\x81\x10a\x0E\xFCWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`\x80\x81R\x84Q`\x80\x82\x01R` \x85\x01Q`\xA0\x82\x01R`@\x85\x01Q`\xC0\x82\x01R``\x85\x01Q`\xE0\x82\x01R`\x80\x85\x01Qa\x01\0\x82\x01R`\x01\x80`\xA0\x1B\x03`\xA0\x86\x01Q\x16a\x01 \x82\x01R`\0`\xC0\x86\x01Q`\xE0a\x01@\x84\x01Ra\x0Fda\x01`\x84\x01\x82a\t\x18V[\x90Pa\x0Fs` \x84\x01\x87a\x0E\xDEV[\x82\x81\x03`@\x84\x01Ra\x0F\x85\x81\x86a\t\x18V[\x90P\x82\x81\x03``\x84\x01Ra\x0F\x99\x81\x85a\t\x18V[\x97\x96PPPPPPPV[`\xA0\x81\x01a\x04\xAD\x82\x84a\r\xE2V\xFE\xA2dipfsX\"\x12 LNx\xDC\xD6\x1F[Ht\xC6m\xB9\xDD\xAB\xFBe\xDBZA\xA4W\xF0\t\xEA\x14\x0FqD\xAF\x8E\x02\x8AdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static TRANSFER_VERIFIER_WRAPPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct transfer_verifier_wrapper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for transfer_verifier_wrapper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for transfer_verifier_wrapper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for transfer_verifier_wrapper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for transfer_verifier_wrapper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(transfer_verifier_wrapper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> transfer_verifier_wrapper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TRANSFER_VERIFIER_WRAPPER_ABI.clone(),
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
                TRANSFER_VERIFIER_WRAPPER_ABI.clone(),
                TRANSFER_VERIFIER_WRAPPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `checkSampleInputsAndProof` (0x10a54279) function
        pub fn check_sample_inputs_and_proof(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([16, 165, 66, 121], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createRequest` (0xd2235eac) function
        pub fn create_request(
            &self,
            ask: Ask,
            secret_type: u8,
            secret_inputs: ::ethers::core::types::Bytes,
            acl: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 35, 94, 172], (ask, secret_type, secret_inputs, acl))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeInputAndProofForVerification` (0x9966599f) function
        pub fn encode_input_and_proof_for_verification(
            &self,
            inputs: [::ethers::core::types::U256; 5],
            proof: [::ethers::core::types::U256; 8],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([153, 102, 89, 159], (inputs, proof))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeInputs` (0xdfd4ac1b) function
        pub fn encode_inputs(
            &self,
            inputs: [::ethers::core::types::U256; 5],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([223, 212, 172, 27], inputs)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeProof` (0x64f1bcc7) function
        pub fn encode_proof(
            &self,
            proof: [::ethers::core::types::U256; 8],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([100, 241, 188, 199], proof)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `iverifier` (0xe7f5b81d) function
        pub fn iverifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([231, 245, 184, 29], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proofMarketPlace` (0x732a9d63) function
        pub fn proof_market_place(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([115, 42, 157, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sampleInput` (0x7d8ad42b) function
        pub fn sample_input(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([125, 138, 212, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sampleProof` (0xa76c0551) function
        pub fn sample_proof(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([167, 108, 5, 81], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProofMarketPlaceContract` (0xc46fc3ef) function
        pub fn set_proof_market_place_contract(
            &self,
            proof_marketplace: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 111, 195, 239], proof_marketplace)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify` (0x8e760afe) function
        pub fn verify(
            &self,
            encoded_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([142, 118, 10, 254], encoded_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyAgainstSampleInputs` (0x02f77d19) function
        pub fn verify_against_sample_inputs(
            &self,
            encoded_proof: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([2, 247, 125, 25], encoded_proof)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyInputs` (0xa6dfbc7f) function
        pub fn verify_inputs(
            &self,
            inputs: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([166, 223, 188, 127], inputs)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for transfer_verifier_wrapper<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `checkSampleInputsAndProof` function with signature `checkSampleInputsAndProof()` and selector `0x10a54279`
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
    #[ethcall(name = "checkSampleInputsAndProof", abi = "checkSampleInputsAndProof()")]
    pub struct CheckSampleInputsAndProofCall;
    ///Container type for all input parameters for the `createRequest` function with signature `createRequest((uint256,uint256,uint256,uint256,uint256,address,bytes),uint8,bytes,bytes)` and selector `0xd2235eac`
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
        name = "createRequest",
        abi = "createRequest((uint256,uint256,uint256,uint256,uint256,address,bytes),uint8,bytes,bytes)"
    )]
    pub struct CreateRequestCall {
        pub ask: Ask,
        pub secret_type: u8,
        pub secret_inputs: ::ethers::core::types::Bytes,
        pub acl: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `encodeInputAndProofForVerification` function with signature `encodeInputAndProofForVerification(uint256[5],uint256[8])` and selector `0x9966599f`
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
        name = "encodeInputAndProofForVerification",
        abi = "encodeInputAndProofForVerification(uint256[5],uint256[8])"
    )]
    pub struct EncodeInputAndProofForVerificationCall {
        pub inputs: [::ethers::core::types::U256; 5],
        pub proof: [::ethers::core::types::U256; 8],
    }
    ///Container type for all input parameters for the `encodeInputs` function with signature `encodeInputs(uint256[5])` and selector `0xdfd4ac1b`
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
    #[ethcall(name = "encodeInputs", abi = "encodeInputs(uint256[5])")]
    pub struct EncodeInputsCall {
        pub inputs: [::ethers::core::types::U256; 5],
    }
    ///Container type for all input parameters for the `encodeProof` function with signature `encodeProof(uint256[8])` and selector `0x64f1bcc7`
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
    #[ethcall(name = "encodeProof", abi = "encodeProof(uint256[8])")]
    pub struct EncodeProofCall {
        pub proof: [::ethers::core::types::U256; 8],
    }
    ///Container type for all input parameters for the `iverifier` function with signature `iverifier()` and selector `0xe7f5b81d`
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
    #[ethcall(name = "iverifier", abi = "iverifier()")]
    pub struct IverifierCall;
    ///Container type for all input parameters for the `proofMarketPlace` function with signature `proofMarketPlace()` and selector `0x732a9d63`
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
    #[ethcall(name = "proofMarketPlace", abi = "proofMarketPlace()")]
    pub struct ProofMarketPlaceCall;
    ///Container type for all input parameters for the `sampleInput` function with signature `sampleInput()` and selector `0x7d8ad42b`
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
    #[ethcall(name = "sampleInput", abi = "sampleInput()")]
    pub struct SampleInputCall;
    ///Container type for all input parameters for the `sampleProof` function with signature `sampleProof()` and selector `0xa76c0551`
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
    #[ethcall(name = "sampleProof", abi = "sampleProof()")]
    pub struct SampleProofCall;
    ///Container type for all input parameters for the `setProofMarketPlaceContract` function with signature `setProofMarketPlaceContract(address)` and selector `0xc46fc3ef`
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
        name = "setProofMarketPlaceContract",
        abi = "setProofMarketPlaceContract(address)"
    )]
    pub struct SetProofMarketPlaceContractCall {
        pub proof_marketplace: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `verify` function with signature `verify(bytes)` and selector `0x8e760afe`
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
    #[ethcall(name = "verify", abi = "verify(bytes)")]
    pub struct VerifyCall {
        pub encoded_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `verifyAgainstSampleInputs` function with signature `verifyAgainstSampleInputs(bytes)` and selector `0x02f77d19`
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
        name = "verifyAgainstSampleInputs",
        abi = "verifyAgainstSampleInputs(bytes)"
    )]
    pub struct VerifyAgainstSampleInputsCall {
        pub encoded_proof: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `verifyInputs` function with signature `verifyInputs(bytes)` and selector `0xa6dfbc7f`
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
    #[ethcall(name = "verifyInputs", abi = "verifyInputs(bytes)")]
    pub struct VerifyInputsCall {
        pub inputs: ::ethers::core::types::Bytes,
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
    pub enum transfer_verifier_wrapperCalls {
        CheckSampleInputsAndProof(CheckSampleInputsAndProofCall),
        CreateRequest(CreateRequestCall),
        EncodeInputAndProofForVerification(EncodeInputAndProofForVerificationCall),
        EncodeInputs(EncodeInputsCall),
        EncodeProof(EncodeProofCall),
        Iverifier(IverifierCall),
        ProofMarketPlace(ProofMarketPlaceCall),
        SampleInput(SampleInputCall),
        SampleProof(SampleProofCall),
        SetProofMarketPlaceContract(SetProofMarketPlaceContractCall),
        Verify(VerifyCall),
        VerifyAgainstSampleInputs(VerifyAgainstSampleInputsCall),
        VerifyInputs(VerifyInputsCall),
    }
    impl ::ethers::core::abi::AbiDecode for transfer_verifier_wrapperCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CheckSampleInputsAndProofCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckSampleInputsAndProof(decoded));
            }
            if let Ok(decoded) = <CreateRequestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateRequest(decoded));
            }
            if let Ok(decoded) = <EncodeInputAndProofForVerificationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EncodeInputAndProofForVerification(decoded));
            }
            if let Ok(decoded) = <EncodeInputsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EncodeInputs(decoded));
            }
            if let Ok(decoded) = <EncodeProofCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EncodeProof(decoded));
            }
            if let Ok(decoded) = <IverifierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Iverifier(decoded));
            }
            if let Ok(decoded) = <ProofMarketPlaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProofMarketPlace(decoded));
            }
            if let Ok(decoded) = <SampleInputCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SampleInput(decoded));
            }
            if let Ok(decoded) = <SampleProofCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SampleProof(decoded));
            }
            if let Ok(decoded) = <SetProofMarketPlaceContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetProofMarketPlaceContract(decoded));
            }
            if let Ok(decoded) = <VerifyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Verify(decoded));
            }
            if let Ok(decoded) = <VerifyAgainstSampleInputsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifyAgainstSampleInputs(decoded));
            }
            if let Ok(decoded) = <VerifyInputsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifyInputs(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for transfer_verifier_wrapperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckSampleInputsAndProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeInputAndProofForVerification(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeInputs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Iverifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProofMarketPlace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SampleInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SampleProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetProofMarketPlaceContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyAgainstSampleInputs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyInputs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for transfer_verifier_wrapperCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckSampleInputsAndProof(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateRequest(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeInputAndProofForVerification(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeInputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::Iverifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofMarketPlace(element) => ::core::fmt::Display::fmt(element, f),
                Self::SampleInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::SampleProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProofMarketPlaceContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyAgainstSampleInputs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyInputs(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckSampleInputsAndProofCall>
    for transfer_verifier_wrapperCalls {
        fn from(value: CheckSampleInputsAndProofCall) -> Self {
            Self::CheckSampleInputsAndProof(value)
        }
    }
    impl ::core::convert::From<CreateRequestCall> for transfer_verifier_wrapperCalls {
        fn from(value: CreateRequestCall) -> Self {
            Self::CreateRequest(value)
        }
    }
    impl ::core::convert::From<EncodeInputAndProofForVerificationCall>
    for transfer_verifier_wrapperCalls {
        fn from(value: EncodeInputAndProofForVerificationCall) -> Self {
            Self::EncodeInputAndProofForVerification(value)
        }
    }
    impl ::core::convert::From<EncodeInputsCall> for transfer_verifier_wrapperCalls {
        fn from(value: EncodeInputsCall) -> Self {
            Self::EncodeInputs(value)
        }
    }
    impl ::core::convert::From<EncodeProofCall> for transfer_verifier_wrapperCalls {
        fn from(value: EncodeProofCall) -> Self {
            Self::EncodeProof(value)
        }
    }
    impl ::core::convert::From<IverifierCall> for transfer_verifier_wrapperCalls {
        fn from(value: IverifierCall) -> Self {
            Self::Iverifier(value)
        }
    }
    impl ::core::convert::From<ProofMarketPlaceCall> for transfer_verifier_wrapperCalls {
        fn from(value: ProofMarketPlaceCall) -> Self {
            Self::ProofMarketPlace(value)
        }
    }
    impl ::core::convert::From<SampleInputCall> for transfer_verifier_wrapperCalls {
        fn from(value: SampleInputCall) -> Self {
            Self::SampleInput(value)
        }
    }
    impl ::core::convert::From<SampleProofCall> for transfer_verifier_wrapperCalls {
        fn from(value: SampleProofCall) -> Self {
            Self::SampleProof(value)
        }
    }
    impl ::core::convert::From<SetProofMarketPlaceContractCall>
    for transfer_verifier_wrapperCalls {
        fn from(value: SetProofMarketPlaceContractCall) -> Self {
            Self::SetProofMarketPlaceContract(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for transfer_verifier_wrapperCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    impl ::core::convert::From<VerifyAgainstSampleInputsCall>
    for transfer_verifier_wrapperCalls {
        fn from(value: VerifyAgainstSampleInputsCall) -> Self {
            Self::VerifyAgainstSampleInputs(value)
        }
    }
    impl ::core::convert::From<VerifyInputsCall> for transfer_verifier_wrapperCalls {
        fn from(value: VerifyInputsCall) -> Self {
            Self::VerifyInputs(value)
        }
    }
    ///Container type for all return fields from the `checkSampleInputsAndProof` function with signature `checkSampleInputsAndProof()` and selector `0x10a54279`
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
    pub struct CheckSampleInputsAndProofReturn(pub bool);
    ///Container type for all return fields from the `encodeInputAndProofForVerification` function with signature `encodeInputAndProofForVerification(uint256[5],uint256[8])` and selector `0x9966599f`
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
    pub struct EncodeInputAndProofForVerificationReturn(
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all return fields from the `encodeInputs` function with signature `encodeInputs(uint256[5])` and selector `0xdfd4ac1b`
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
    pub struct EncodeInputsReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `encodeProof` function with signature `encodeProof(uint256[8])` and selector `0x64f1bcc7`
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
    pub struct EncodeProofReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `iverifier` function with signature `iverifier()` and selector `0xe7f5b81d`
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
    pub struct IverifierReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proofMarketPlace` function with signature `proofMarketPlace()` and selector `0x732a9d63`
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
    pub struct ProofMarketPlaceReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `sampleInput` function with signature `sampleInput()` and selector `0x7d8ad42b`
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
    pub struct SampleInputReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `sampleProof` function with signature `sampleProof()` and selector `0xa76c0551`
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
    pub struct SampleProofReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `verify` function with signature `verify(bytes)` and selector `0x8e760afe`
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
    pub struct VerifyReturn(pub bool);
    ///Container type for all return fields from the `verifyAgainstSampleInputs` function with signature `verifyAgainstSampleInputs(bytes)` and selector `0x02f77d19`
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
    pub struct VerifyAgainstSampleInputsReturn(pub bool);
    ///Container type for all return fields from the `verifyInputs` function with signature `verifyInputs(bytes)` and selector `0xa6dfbc7f`
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
    pub struct VerifyInputsReturn(pub bool);
}
