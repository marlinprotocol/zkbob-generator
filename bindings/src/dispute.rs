pub use dispute::*;
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
pub mod dispute {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("checkDisputeUsingAttesation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "checkDisputeUsingAttesation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("askId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proverData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("attestationData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expectedImageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "invalidProofSignature",
                                    ),
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
                (
                    ::std::borrow::ToOwned::to_owned(
                        "checkDisputeUsingAttestationAndOrSignature",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "checkDisputeUsingAttestationAndOrSignature",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("askId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proverData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("completeData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expectedImageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultIvsSigner"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkDisputeUsingSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "checkDisputeUsingSignature",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("askId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proverData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "invalidProofSignature",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expectedSigner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isPublic"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
    pub static DISPUTE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0C*\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0@W`\x005`\xE0\x1C\x80b.'\x18\x14a\0EW\x80cq\xB5\x82\xFF\x14a\0lW\x80c\xB2\xDC\xD8\xC8\x14a\0\x7FW[`\0\x80\xFD[a\0Xa\0S6`\x04a\x06\xA7V[a\0\x92V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0Xa\0z6`\x04a\x08\x18V[a\x01\x10V[a\0Xa\0\x8D6`\x04a\x08\xADV[a\x02AV[`\0\x80\x80\x80a\0\xA3\x87\x89\x01\x89a\tMV[\x92P\x92P\x92P\x80\x15a\0\xF1Wa\0\xE7\x8B\x8B\x8B\x85\x89\x8B\x15\x80a\0zWP\x7F\x99\xFF\r\x91%\xE1\xFC\x951\xA1\x12b\xE1Z\xEB,`P\x9A\x07\x8CL\xC4\xC6L\xEF\xDF\xB0o\xF6\x86G\x8C\x14a\x01\x10V[\x93PPPPa\x01\x05V[a\0\xFF\x8B\x8B\x8B\x86\x8A\x87a\x02AV[\x93PPPP[\x97\x96PPPPPPPV[`\0\x80\x82\x15a\x01KW\x87\x87\x87`@Q` \x01a\x01.\x93\x92\x91\x90a\t\xC1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x01pV[`@\x80Q` \x81\x01\x8A\x90R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[`\0a\x01\xC9\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90P`\0a\x01\xD7\x82\x88a\x02\xE1V[\x90P\x85`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG7`\xF0\x1B\x81RP\x90a\x020W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02'\x91\x90a\n\x1BV[`@Q\x80\x91\x03\x90\xFD[P`\x01\x9A\x99PPPPPPPPPPV[`\0\x80a\x02M\x85a\x03\x05V[\x90P\x83\x81\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA11`\xE8\x1B\x81RP\x90a\x02\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02'\x91\x90a\n\x1BV[P`\0a\x02\x98\x86a\x03>V[\x91Pa\x02\xD4\x90P\x89\x89\x89\x87\x85\x8A\x15\x80a\0zWP\x7F\x99\xFF\r\x91%\xE1\xFC\x951\xA1\x12b\xE1Z\xEB,`P\x9A\x07\x8CL\xC4\xC6L\xEF\xDF\xB0o\xF6\x86G\x8B\x14a\x01\x10V[\x99\x98PPPPPPPPPV[`\0\x80`\0a\x02\xF0\x85\x85a\x03tV[\x91P\x91Pa\x02\xFD\x81a\x03\xB9V[P\x93\x92PPPV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03\x1F\x91\x90a\n\xA6V[PP\x95P\x95P\x95PPPPa\x035\x83\x83\x83a\x05\x06V[\x95\x94PPPPPV[```\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a\x03W\x91\x90a\n\xA6V[PPPPP\x92PPP\x80a\x03j\x82a\x05?V[\x92P\x92PP\x91P\x91V[`\0\x80\x82Q`A\x03a\x03\xAAW` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x03\x9E\x87\x82\x85\x85a\x05\x8CV[\x94P\x94PPPPa\x03\xB2V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x03\xCDWa\x03\xCDa\x0B\x9BV[\x03a\x03\xD5WPV[`\x01\x81`\x04\x81\x11\x15a\x03\xE9Wa\x03\xE9a\x0B\x9BV[\x03a\x046W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02'V[`\x02\x81`\x04\x81\x11\x15a\x04JWa\x04Ja\x0B\x9BV[\x03a\x04\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x02'V[`\x03\x81`\x04\x81\x11\x15a\x04\xABWa\x04\xABa\x0B\x9BV[\x03a\x05\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x02'V[PV[`\0\x80\x84\x84\x84`@Q` \x01a\x05\x1E\x93\x92\x91\x90a\x0B\xB1V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`\0\x81Q`@\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG7`\xF0\x1B\x81RP\x90a\x05\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02'\x91\x90a\n\x1BV[PP\x80Q` \x90\x91\x01 \x90V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x05\xC3WP`\0\x90P`\x03a\x06GV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x06\x17W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06@W`\0`\x01\x92P\x92PPa\x06GV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x06bW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06zW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x03\xB2W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x03W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15a\x06\xC2W`\0\x80\xFD[\x875\x96P` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xE1W`\0\x80\xFD[a\x06\xED\x8B\x83\x8C\x01a\x06PV[\x90\x98P\x96P`@\x8A\x015\x91P\x80\x82\x11\x15a\x07\x06W`\0\x80\xFD[Pa\x07\x13\x8A\x82\x8B\x01a\x06PV[\x90\x95P\x93PP``\x88\x015\x91P`\x80\x88\x015a\x07.\x81a\x06\x92V[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07}Wa\x07}a\x07>V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\x9FWa\x07\x9Fa\x07>V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x07\xBEW`\0\x80\xFD[\x815a\x07\xD1a\x07\xCC\x82a\x07\x85V[a\x07TV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x07\xE6W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805\x80\x15\x15\x81\x14a\x08\x13W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x081W`\0\x80\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08PW`\0\x80\xFD[a\x08\\\x8A\x83\x8B\x01a\x06PV[\x90\x97P\x95P`@\x89\x015\x91P\x80\x82\x11\x15a\x08uW`\0\x80\xFD[Pa\x08\x82\x89\x82\x8A\x01a\x07\xADV[\x93PP``\x87\x015a\x08\x93\x81a\x06\x92V[\x91Pa\x08\xA1`\x80\x88\x01a\x08\x03V[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x08\xC6W`\0\x80\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\xE5W`\0\x80\xFD[a\x08\xF1\x8A\x83\x8B\x01a\x06PV[\x90\x97P\x95P`@\x89\x015\x91P\x80\x82\x11\x15a\t\nW`\0\x80\xFD[a\t\x16\x8A\x83\x8B\x01a\x07\xADV[\x94P``\x89\x015\x93P`\x80\x89\x015\x91P\x80\x82\x11\x15a\t3W`\0\x80\xFD[Pa\t@\x89\x82\x8A\x01a\x07\xADV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0``\x84\x86\x03\x12\x15a\tbW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\tzW`\0\x80\xFD[a\t\x86\x87\x83\x88\x01a\x07\xADV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a\t\x9CW`\0\x80\xFD[Pa\t\xA9\x86\x82\x87\x01a\x07\xADV[\x92PPa\t\xB8`@\x85\x01a\x08\x03V[\x90P\x92P\x92P\x92V[\x83\x81R`@` \x82\x01R\x81`@\x82\x01R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\0[\x83\x81\x10\x15a\n\x12W\x81\x81\x01Q\x83\x82\x01R` \x01a\t\xFAV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\n:\x81`@\x85\x01` \x87\x01a\t\xF7V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\n_W`\0\x80\xFD[\x81Qa\nma\x07\xCC\x82a\x07\x85V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\n\x82W`\0\x80\xFD[a\n\x93\x82` \x83\x01` \x87\x01a\t\xF7V[\x94\x93PPPPV[\x80Qa\x08\x13\x81a\x06\x92V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\n\xC3W`\0\x80\xFD[\x88Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xDBW`\0\x80\xFD[a\n\xE7\x8C\x83\x8D\x01a\nNV[\x99Pa\n\xF5` \x8C\x01a\n\x9BV[\x98P`@\x8B\x01Q\x91P\x80\x82\x11\x15a\x0B\x0BW`\0\x80\xFD[a\x0B\x17\x8C\x83\x8D\x01a\nNV[\x97P``\x8B\x01Q\x91P\x80\x82\x11\x15a\x0B-W`\0\x80\xFD[a\x0B9\x8C\x83\x8D\x01a\nNV[\x96P`\x80\x8B\x01Q\x91P\x80\x82\x11\x15a\x0BOW`\0\x80\xFD[a\x0B[\x8C\x83\x8D\x01a\nNV[\x95P`\xA0\x8B\x01Q\x91P\x80\x82\x11\x15a\x0BqW`\0\x80\xFD[Pa\x0B~\x8B\x82\x8C\x01a\nNV[`\xC0\x8B\x01Q`\xE0\x90\x9B\x01Q\x99\x9C\x98\x9BP\x96\x99\x95\x98\x94\x97\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x84Qa\x0B\xC3\x81\x84` \x89\x01a\t\xF7V[\x84Q\x90\x83\x01\x90a\x0B\xD7\x81\x83` \x89\x01a\t\xF7V[\x84Q\x91\x01\x90a\x0B\xEA\x81\x83` \x88\x01a\t\xF7V[\x01\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \xE6\xBC$\xB9\xB9e\xFA\xAC\xF6\xA2+9\xBD\xB3\xD4\x0B\xBC *l\x1B\xF9\x16I<-=\xD1\xB7\xB6\x0F\x7FdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static DISPUTE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0@W`\x005`\xE0\x1C\x80b.'\x18\x14a\0EW\x80cq\xB5\x82\xFF\x14a\0lW\x80c\xB2\xDC\xD8\xC8\x14a\0\x7FW[`\0\x80\xFD[a\0Xa\0S6`\x04a\x06\xA7V[a\0\x92V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0Xa\0z6`\x04a\x08\x18V[a\x01\x10V[a\0Xa\0\x8D6`\x04a\x08\xADV[a\x02AV[`\0\x80\x80\x80a\0\xA3\x87\x89\x01\x89a\tMV[\x92P\x92P\x92P\x80\x15a\0\xF1Wa\0\xE7\x8B\x8B\x8B\x85\x89\x8B\x15\x80a\0zWP\x7F\x99\xFF\r\x91%\xE1\xFC\x951\xA1\x12b\xE1Z\xEB,`P\x9A\x07\x8CL\xC4\xC6L\xEF\xDF\xB0o\xF6\x86G\x8C\x14a\x01\x10V[\x93PPPPa\x01\x05V[a\0\xFF\x8B\x8B\x8B\x86\x8A\x87a\x02AV[\x93PPPP[\x97\x96PPPPPPPV[`\0\x80\x82\x15a\x01KW\x87\x87\x87`@Q` \x01a\x01.\x93\x92\x91\x90a\t\xC1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x01pV[`@\x80Q` \x81\x01\x8A\x90R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[`\0a\x01\xC9\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90P`\0a\x01\xD7\x82\x88a\x02\xE1V[\x90P\x85`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG7`\xF0\x1B\x81RP\x90a\x020W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02'\x91\x90a\n\x1BV[`@Q\x80\x91\x03\x90\xFD[P`\x01\x9A\x99PPPPPPPPPPV[`\0\x80a\x02M\x85a\x03\x05V[\x90P\x83\x81\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA11`\xE8\x1B\x81RP\x90a\x02\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02'\x91\x90a\n\x1BV[P`\0a\x02\x98\x86a\x03>V[\x91Pa\x02\xD4\x90P\x89\x89\x89\x87\x85\x8A\x15\x80a\0zWP\x7F\x99\xFF\r\x91%\xE1\xFC\x951\xA1\x12b\xE1Z\xEB,`P\x9A\x07\x8CL\xC4\xC6L\xEF\xDF\xB0o\xF6\x86G\x8B\x14a\x01\x10V[\x99\x98PPPPPPPPPV[`\0\x80`\0a\x02\xF0\x85\x85a\x03tV[\x91P\x91Pa\x02\xFD\x81a\x03\xB9V[P\x93\x92PPPV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03\x1F\x91\x90a\n\xA6V[PP\x95P\x95P\x95PPPPa\x035\x83\x83\x83a\x05\x06V[\x95\x94PPPPPV[```\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a\x03W\x91\x90a\n\xA6V[PPPPP\x92PPP\x80a\x03j\x82a\x05?V[\x92P\x92PP\x91P\x91V[`\0\x80\x82Q`A\x03a\x03\xAAW` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x03\x9E\x87\x82\x85\x85a\x05\x8CV[\x94P\x94PPPPa\x03\xB2V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x03\xCDWa\x03\xCDa\x0B\x9BV[\x03a\x03\xD5WPV[`\x01\x81`\x04\x81\x11\x15a\x03\xE9Wa\x03\xE9a\x0B\x9BV[\x03a\x046W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02'V[`\x02\x81`\x04\x81\x11\x15a\x04JWa\x04Ja\x0B\x9BV[\x03a\x04\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x02'V[`\x03\x81`\x04\x81\x11\x15a\x04\xABWa\x04\xABa\x0B\x9BV[\x03a\x05\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x02'V[PV[`\0\x80\x84\x84\x84`@Q` \x01a\x05\x1E\x93\x92\x91\x90a\x0B\xB1V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`\0\x81Q`@\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG7`\xF0\x1B\x81RP\x90a\x05\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02'\x91\x90a\n\x1BV[PP\x80Q` \x90\x91\x01 \x90V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x05\xC3WP`\0\x90P`\x03a\x06GV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x06\x17W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06@W`\0`\x01\x92P\x92PPa\x06GV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x06bW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06zW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x03\xB2W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x03W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15a\x06\xC2W`\0\x80\xFD[\x875\x96P` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xE1W`\0\x80\xFD[a\x06\xED\x8B\x83\x8C\x01a\x06PV[\x90\x98P\x96P`@\x8A\x015\x91P\x80\x82\x11\x15a\x07\x06W`\0\x80\xFD[Pa\x07\x13\x8A\x82\x8B\x01a\x06PV[\x90\x95P\x93PP``\x88\x015\x91P`\x80\x88\x015a\x07.\x81a\x06\x92V[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07}Wa\x07}a\x07>V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\x9FWa\x07\x9Fa\x07>V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x07\xBEW`\0\x80\xFD[\x815a\x07\xD1a\x07\xCC\x82a\x07\x85V[a\x07TV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x07\xE6W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805\x80\x15\x15\x81\x14a\x08\x13W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x081W`\0\x80\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08PW`\0\x80\xFD[a\x08\\\x8A\x83\x8B\x01a\x06PV[\x90\x97P\x95P`@\x89\x015\x91P\x80\x82\x11\x15a\x08uW`\0\x80\xFD[Pa\x08\x82\x89\x82\x8A\x01a\x07\xADV[\x93PP``\x87\x015a\x08\x93\x81a\x06\x92V[\x91Pa\x08\xA1`\x80\x88\x01a\x08\x03V[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x08\xC6W`\0\x80\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\xE5W`\0\x80\xFD[a\x08\xF1\x8A\x83\x8B\x01a\x06PV[\x90\x97P\x95P`@\x89\x015\x91P\x80\x82\x11\x15a\t\nW`\0\x80\xFD[a\t\x16\x8A\x83\x8B\x01a\x07\xADV[\x94P``\x89\x015\x93P`\x80\x89\x015\x91P\x80\x82\x11\x15a\t3W`\0\x80\xFD[Pa\t@\x89\x82\x8A\x01a\x07\xADV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0``\x84\x86\x03\x12\x15a\tbW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\tzW`\0\x80\xFD[a\t\x86\x87\x83\x88\x01a\x07\xADV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a\t\x9CW`\0\x80\xFD[Pa\t\xA9\x86\x82\x87\x01a\x07\xADV[\x92PPa\t\xB8`@\x85\x01a\x08\x03V[\x90P\x92P\x92P\x92V[\x83\x81R`@` \x82\x01R\x81`@\x82\x01R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\0[\x83\x81\x10\x15a\n\x12W\x81\x81\x01Q\x83\x82\x01R` \x01a\t\xFAV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\n:\x81`@\x85\x01` \x87\x01a\t\xF7V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\n_W`\0\x80\xFD[\x81Qa\nma\x07\xCC\x82a\x07\x85V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\n\x82W`\0\x80\xFD[a\n\x93\x82` \x83\x01` \x87\x01a\t\xF7V[\x94\x93PPPPV[\x80Qa\x08\x13\x81a\x06\x92V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\n\xC3W`\0\x80\xFD[\x88Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xDBW`\0\x80\xFD[a\n\xE7\x8C\x83\x8D\x01a\nNV[\x99Pa\n\xF5` \x8C\x01a\n\x9BV[\x98P`@\x8B\x01Q\x91P\x80\x82\x11\x15a\x0B\x0BW`\0\x80\xFD[a\x0B\x17\x8C\x83\x8D\x01a\nNV[\x97P``\x8B\x01Q\x91P\x80\x82\x11\x15a\x0B-W`\0\x80\xFD[a\x0B9\x8C\x83\x8D\x01a\nNV[\x96P`\x80\x8B\x01Q\x91P\x80\x82\x11\x15a\x0BOW`\0\x80\xFD[a\x0B[\x8C\x83\x8D\x01a\nNV[\x95P`\xA0\x8B\x01Q\x91P\x80\x82\x11\x15a\x0BqW`\0\x80\xFD[Pa\x0B~\x8B\x82\x8C\x01a\nNV[`\xC0\x8B\x01Q`\xE0\x90\x9B\x01Q\x99\x9C\x98\x9BP\x96\x99\x95\x98\x94\x97\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x84Qa\x0B\xC3\x81\x84` \x89\x01a\t\xF7V[\x84Q\x90\x83\x01\x90a\x0B\xD7\x81\x83` \x89\x01a\t\xF7V[\x84Q\x91\x01\x90a\x0B\xEA\x81\x83` \x88\x01a\t\xF7V[\x01\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \xE6\xBC$\xB9\xB9e\xFA\xAC\xF6\xA2+9\xBD\xB3\xD4\x0B\xBC *l\x1B\xF9\x16I<-=\xD1\xB7\xB6\x0F\x7FdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static DISPUTE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Dispute<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Dispute<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Dispute<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Dispute<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Dispute<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Dispute)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Dispute<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DISPUTE_ABI.clone(),
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
                DISPUTE_ABI.clone(),
                DISPUTE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `checkDisputeUsingAttesation` (0xb2dcd8c8) function
        pub fn check_dispute_using_attesation(
            &self,
            ask_id: ::ethers::core::types::U256,
            prover_data: ::ethers::core::types::Bytes,
            attestation_data: ::ethers::core::types::Bytes,
            expected_image_id: [u8; 32],
            invalid_proof_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [178, 220, 216, 200],
                    (
                        ask_id,
                        prover_data,
                        attestation_data,
                        expected_image_id,
                        invalid_proof_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkDisputeUsingAttestationAndOrSignature` (0x002e2718) function
        pub fn check_dispute_using_attestation_and_or_signature(
            &self,
            ask_id: ::ethers::core::types::U256,
            prover_data: ::ethers::core::types::Bytes,
            complete_data: ::ethers::core::types::Bytes,
            expected_image_id: [u8; 32],
            default_ivs_signer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [0, 46, 39, 24],
                    (
                        ask_id,
                        prover_data,
                        complete_data,
                        expected_image_id,
                        default_ivs_signer,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkDisputeUsingSignature` (0x71b582ff) function
        pub fn check_dispute_using_signature(
            &self,
            ask_id: ::ethers::core::types::U256,
            prover_data: ::ethers::core::types::Bytes,
            invalid_proof_signature: ::ethers::core::types::Bytes,
            expected_signer: ::ethers::core::types::Address,
            is_public: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [113, 181, 130, 255],
                    (
                        ask_id,
                        prover_data,
                        invalid_proof_signature,
                        expected_signer,
                        is_public,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Dispute<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `checkDisputeUsingAttesation` function with signature `checkDisputeUsingAttesation(uint256,bytes,bytes,bytes32,bytes)` and selector `0xb2dcd8c8`
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
        name = "checkDisputeUsingAttesation",
        abi = "checkDisputeUsingAttesation(uint256,bytes,bytes,bytes32,bytes)"
    )]
    pub struct CheckDisputeUsingAttesationCall {
        pub ask_id: ::ethers::core::types::U256,
        pub prover_data: ::ethers::core::types::Bytes,
        pub attestation_data: ::ethers::core::types::Bytes,
        pub expected_image_id: [u8; 32],
        pub invalid_proof_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `checkDisputeUsingAttestationAndOrSignature` function with signature `checkDisputeUsingAttestationAndOrSignature(uint256,bytes,bytes,bytes32,address)` and selector `0x002e2718`
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
        name = "checkDisputeUsingAttestationAndOrSignature",
        abi = "checkDisputeUsingAttestationAndOrSignature(uint256,bytes,bytes,bytes32,address)"
    )]
    pub struct CheckDisputeUsingAttestationAndOrSignatureCall {
        pub ask_id: ::ethers::core::types::U256,
        pub prover_data: ::ethers::core::types::Bytes,
        pub complete_data: ::ethers::core::types::Bytes,
        pub expected_image_id: [u8; 32],
        pub default_ivs_signer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `checkDisputeUsingSignature` function with signature `checkDisputeUsingSignature(uint256,bytes,bytes,address,bool)` and selector `0x71b582ff`
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
        name = "checkDisputeUsingSignature",
        abi = "checkDisputeUsingSignature(uint256,bytes,bytes,address,bool)"
    )]
    pub struct CheckDisputeUsingSignatureCall {
        pub ask_id: ::ethers::core::types::U256,
        pub prover_data: ::ethers::core::types::Bytes,
        pub invalid_proof_signature: ::ethers::core::types::Bytes,
        pub expected_signer: ::ethers::core::types::Address,
        pub is_public: bool,
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
    pub enum DisputeCalls {
        CheckDisputeUsingAttesation(CheckDisputeUsingAttesationCall),
        CheckDisputeUsingAttestationAndOrSignature(
            CheckDisputeUsingAttestationAndOrSignatureCall,
        ),
        CheckDisputeUsingSignature(CheckDisputeUsingSignatureCall),
    }
    impl ::ethers::core::abi::AbiDecode for DisputeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CheckDisputeUsingAttesationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckDisputeUsingAttesation(decoded));
            }
            if let Ok(decoded) = <CheckDisputeUsingAttestationAndOrSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckDisputeUsingAttestationAndOrSignature(decoded));
            }
            if let Ok(decoded) = <CheckDisputeUsingSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckDisputeUsingSignature(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DisputeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckDisputeUsingAttesation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckDisputeUsingAttestationAndOrSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckDisputeUsingSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DisputeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckDisputeUsingAttesation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckDisputeUsingAttestationAndOrSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckDisputeUsingSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CheckDisputeUsingAttesationCall> for DisputeCalls {
        fn from(value: CheckDisputeUsingAttesationCall) -> Self {
            Self::CheckDisputeUsingAttesation(value)
        }
    }
    impl ::core::convert::From<CheckDisputeUsingAttestationAndOrSignatureCall>
    for DisputeCalls {
        fn from(value: CheckDisputeUsingAttestationAndOrSignatureCall) -> Self {
            Self::CheckDisputeUsingAttestationAndOrSignature(value)
        }
    }
    impl ::core::convert::From<CheckDisputeUsingSignatureCall> for DisputeCalls {
        fn from(value: CheckDisputeUsingSignatureCall) -> Self {
            Self::CheckDisputeUsingSignature(value)
        }
    }
    ///Container type for all return fields from the `checkDisputeUsingAttesation` function with signature `checkDisputeUsingAttesation(uint256,bytes,bytes,bytes32,bytes)` and selector `0xb2dcd8c8`
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
    pub struct CheckDisputeUsingAttesationReturn(pub bool);
    ///Container type for all return fields from the `checkDisputeUsingAttestationAndOrSignature` function with signature `checkDisputeUsingAttestationAndOrSignature(uint256,bytes,bytes,bytes32,address)` and selector `0x002e2718`
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
    pub struct CheckDisputeUsingAttestationAndOrSignatureReturn(pub bool);
    ///Container type for all return fields from the `checkDisputeUsingSignature` function with signature `checkDisputeUsingSignature(uint256,bytes,bytes,address,bool)` and selector `0x71b582ff`
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
    pub struct CheckDisputeUsingSignatureReturn(pub bool);
}
