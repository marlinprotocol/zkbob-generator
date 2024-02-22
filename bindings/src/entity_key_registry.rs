pub use entity_key_registry::*;
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
pub mod entity_key_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_attestationVerifier"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IAttestationVerifier",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_admin"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("KEY_REGISTER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("KEY_REGISTER_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("addGeneratorRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addGeneratorRegistry",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_generatorRegistry",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("attestationVerifier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "attestationVerifier",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IAttestationVerifier",
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
                    ::std::borrow::ToOwned::to_owned("dedicated_pub_key_per_market"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "dedicated_pub_key_per_market",
                            ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("pub_key"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pub_key"),
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
                    ::std::borrow::ToOwned::to_owned("removePubkey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removePubkey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyIndex"),
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
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
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
                    ::std::borrow::ToOwned::to_owned("updatePubkey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updatePubkey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("attestation_data"),
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
                    ::std::borrow::ToOwned::to_owned("usedUpKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("usedUpKey"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("RemoveKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RemoveKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("keyIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UpdateKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("UpdateKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("keyIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
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
    pub static ENTITYKEYREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x12\x018\x03\x80b\0\x12\x01\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\x1FV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\x80Rb\0\0N`\0\x82b\0\0VV[PPb\0\x01^V[b\0\0b\x82\x82b\0\0fV[PPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\0bW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\0\xC23\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x1CW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x013W`\0\x80\xFD[\x82Qb\0\x01@\x81b\0\x01\x06V[` \x84\x01Q\x90\x92Pb\0\x01S\x81b\0\x01\x06V[\x80\x91PP\x92P\x92\x90PV[`\x80Qa\x10\x80b\0\x01\x81`\09`\0\x81\x81a\x01\xE1\x01Ra\x04<\x01Ra\x10\x80`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c\xA2\x17\xFD\xDF\x11a\0\x97W\x80c\xD5Gt\x1F\x11a\0fW\x80c\xD5Gt\x1F\x14a\x02#W\x80c\xD7\x03\xF6\x07\x14a\x026W\x80c\xE1\xA610\x14a\x02YW\x80c\xED8\r\x03\x14a\x02lW`\0\x80\xFD[\x80c\xA2\x17\xFD\xDF\x14a\x01\xC1W\x80c\xAA\x0E\xD0\x9F\x14a\x01\xC9W\x80c\xB4\xB3\xC5\xA0\x14a\x01\xDCW\x80c\xB8\n\xAA\x89\x14a\x02\x1BW`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\0\xD3W\x80c6V\x8A\xBE\x14a\x01hW\x80ci\xFD\xBC\xCA\x14a\x01{W\x80ct\x87\xE5d\x14a\x01\x8EW\x80c\x91\xD1HT\x14a\x01\xAEW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\0\xFAW\x80c$\x8A\x9C\xA3\x14a\x01\"W\x80c//\xF1]\x14a\x01SW[`\0\x80\xFD[a\x01\ra\x01\x086`\x04a\x0B\x7FV[a\x02\x7FV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01Ea\x0106`\x04a\x0B\xA9V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\x19V[a\x01fa\x01a6`\x04a\x0B\xDEV[a\x02\xB6V[\0[a\x01fa\x01v6`\x04a\x0B\xDEV[a\x02\xE0V[a\x01fa\x01\x896`\x04a\x0CSV[a\x03cV[a\x01\xA1a\x01\x9C6`\x04a\x0C\xDDV[a\x05\xF7V[`@Qa\x01\x19\x91\x90a\rWV[a\x01\ra\x01\xBC6`\x04a\x0B\xDEV[a\x06\x9CV[a\x01E`\0\x81V[a\x01fa\x01\xD76`\x04a\rjV[a\x06\xC5V[a\x02\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x19V[a\x01Ea\x06\xF2V[a\x01fa\x0216`\x04a\x0B\xDEV[a\x07\x0EV[a\x01\ra\x02D6`\x04a\rjV[`\x02` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x01\xA1a\x02g6`\x04a\x0C\xDDV[a\x073V[a\x01fa\x02z6`\x04a\x0C\xDDV[a\x07WV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x02\xB0WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x02\xD1\x81a\x07\xE0V[a\x02\xDB\x83\x83a\x07\xEDV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x03UW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03_\x82\x82a\x08qV[PPV[a\x03|`\x01`\0\x80Q` a\x10+\x839\x81Q\x91Ra\r\x9BV[a\x03\x85\x81a\x07\xE0V[\x84\x84`\0a\x03\xC8\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x08\xD6\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x03\x83RbG13`\xE8\x1B\x91\x83\x01\x91\x90\x91R\x91\x92P\x90`\xFF\x16\x15a\x04$W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03L\x91\x90a\rWV[P`@QcG;\x05\x7F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x8Ev\n\xFE\x90a\x04s\x90\x89\x90\x89\x90`\x04\x01a\r\xAEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB4\x91\x90a\r\xDDV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a \x9B`\xF1\x1B\x81RP\x90a\x04\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03L\x91\x90a\rWV[P`@\x80Q\x80\x82\x01\x82R`\x02\x81RaG7`\xF0\x1B` \x82\x01R\x90\x88\x14a\x05&W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03L\x91\x90a\rWV[P`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x8C\x84R\x90\x91R\x90 a\x05U\x88\x8A\x83a\x0E\x9DV[P`\0a\x05\x97\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x08\xD6\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x02` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x92\x93P\x8C\x92\x91\x8E\x16\x91\x7FR\x1E=\xAC\x83\xE2\x8EU\xEF\xB9A\xFDXz\xBA\xB2\ty|\xEAqA\xEFP\xAA2\xF0\\\n;\xDB\x17\x91\x90\xA3PPPPPPPPPPPV[`\x03` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80Ta\x06\x1B\x90a\x0E\x15V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06G\x90a\x0E\x15V[\x80\x15a\x06\x94W\x80`\x1F\x10a\x06iWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x94V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06wW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0a\x06\xD0\x81a\x07\xE0V[a\x03_a\x06\xEC`\x01`\0\x80Q` a\x10+\x839\x81Q\x91Ra\r\x9BV[\x83a\x07\xEDV[a\x07\x0B`\x01`\0\x80Q` a\x10+\x839\x81Q\x91Ra\r\x9BV[\x81V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x07)\x81a\x07\xE0V[a\x02\xDB\x83\x83a\x08qV[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80Ta\x06\x1B\x90a\x0E\x15V[a\x07p`\x01`\0\x80Q` a\x10+\x839\x81Q\x91Ra\r\x9BV[a\x07y\x81a\x07\xE0V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 a\x07\xA5\x91a\x0B1V[`@Q\x82\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xD4\x08O{z\x07\x9E|\xBF\xEE\x05\x96\xA4r1\xBAK\x1F\x97\x83a,\xB2r\x0E\xDE{\xF3\x8Eg\x10\xA2\x90`\0\x90\xA3PPPV[a\x07\xEA\x813a\t#V[PV[a\x07\xF7\x82\x82a\x06\x9CV[a\x03_W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x08-3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x08{\x82\x82a\x06\x9CV[\x15a\x03_W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0\x81Q`@\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG7`\xF0\x1B\x81RP\x90a\t\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03L\x91\x90a\rWV[PP\x80Q` \x90\x91\x01 \x90V[a\t-\x82\x82a\x06\x9CV[a\x03_Wa\t:\x81a\t|V[a\tE\x83` a\t\x8EV[`@Q` \x01a\tV\x92\x91\x90a\x0F^V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03L\x91`\x04\x01a\rWV[``a\x02\xB0`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\t\x9D\x83`\x02a\x0F\xD3V[a\t\xA8\x90`\x02a\x0F\xEAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xC0Wa\t\xC0a\r\xFFV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\t\xEAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\n\x05Wa\n\x05a\x0F\xFDV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\n4Wa\n4a\x0F\xFDV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\nX\x84`\x02a\x0F\xD3V[a\nc\x90`\x01a\x0F\xEAV[\x90P[`\x01\x81\x11\x15a\n\xDBWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\n\x97Wa\n\x97a\x0F\xFDV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\n\xADWa\n\xADa\x0F\xFDV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\n\xD4\x81a\x10\x13V[\x90Pa\nfV[P\x83\x15a\x0B*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x03LV[\x93\x92PPPV[P\x80Ta\x0B=\x90a\x0E\x15V[`\0\x82U\x80`\x1F\x10a\x0BMWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x07\xEA\x91\x90[\x80\x82\x11\x15a\x0B{W`\0\x81U`\x01\x01a\x0BgV[P\x90V[`\0` \x82\x84\x03\x12\x15a\x0B\x91W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0B*W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0B\xBBW`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xD9W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xF1W`\0\x80\xFD[\x825\x91Pa\x0C\x01` \x84\x01a\x0B\xC2V[\x90P\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x0C\x1CW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C4W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0CLW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\x0ClW`\0\x80\xFD[a\x0Cu\x87a\x0B\xC2V[\x95P` \x87\x015\x94P`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\x99W`\0\x80\xFD[a\x0C\xA5\x8A\x83\x8B\x01a\x0C\nV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15a\x0C\xBEW`\0\x80\xFD[Pa\x0C\xCB\x89\x82\x8A\x01a\x0C\nV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xF0W`\0\x80\xFD[a\x0C\xF9\x83a\x0B\xC2V[\x94` \x93\x90\x93\x015\x93PPPV[`\0[\x83\x81\x10\x15a\r\"W\x81\x81\x01Q\x83\x82\x01R` \x01a\r\nV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\rC\x81` \x86\x01` \x86\x01a\r\x07V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0B*` \x83\x01\x84a\r+V[`\0` \x82\x84\x03\x12\x15a\r|W`\0\x80\xFD[a\x0B*\x82a\x0B\xC2V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\xB0Wa\x02\xB0a\r\x85V[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\r\xEFW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0B*W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0E)W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0EIWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x02\xDBW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0EvWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0E\x95W\x82\x81U`\x01\x01a\x0E\x82V[PPPPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x0E\xB5Wa\x0E\xB5a\r\xFFV[a\x0E\xC9\x83a\x0E\xC3\x83Ta\x0E\x15V[\x83a\x0EOV[`\0`\x1F\x84\x11`\x01\x81\x14a\x0E\xFDW`\0\x85\x15a\x0E\xE5WP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x0FWV[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x0F.W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x0F\x0EV[P\x86\x82\x10\x15a\x0FKW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x0F\x96\x81`\x17\x85\x01` \x88\x01a\r\x07V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x0F\xC7\x81`(\x84\x01` \x88\x01a\r\x07V[\x01`(\x01\x94\x93PPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\xB0Wa\x02\xB0a\r\x85V[\x80\x82\x01\x80\x82\x11\x15a\x02\xB0Wa\x02\xB0a\r\x85V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a\x10\"Wa\x10\"a\r\x85V[P`\0\x19\x01\x90V\xFE\xDCi|\xE6\xC0\xC3\xF8\x86\xFB\xBC\x8E\r#:\xBF\xACZ\xC56S\xAC\x08\xB3c[\xB0\xAE\xC7\x0C9\\B\xA2dipfsX\"\x12 \x1E\xE6\x0E\x9A[\xD8\x04%l\xD9\x167n\xB8P\xAC\x97\x12t\x80rfZ\x05\x0C\xE8p\xC2\xF6u\xE2\xBCdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static ENTITYKEYREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c\xA2\x17\xFD\xDF\x11a\0\x97W\x80c\xD5Gt\x1F\x11a\0fW\x80c\xD5Gt\x1F\x14a\x02#W\x80c\xD7\x03\xF6\x07\x14a\x026W\x80c\xE1\xA610\x14a\x02YW\x80c\xED8\r\x03\x14a\x02lW`\0\x80\xFD[\x80c\xA2\x17\xFD\xDF\x14a\x01\xC1W\x80c\xAA\x0E\xD0\x9F\x14a\x01\xC9W\x80c\xB4\xB3\xC5\xA0\x14a\x01\xDCW\x80c\xB8\n\xAA\x89\x14a\x02\x1BW`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\0\xD3W\x80c6V\x8A\xBE\x14a\x01hW\x80ci\xFD\xBC\xCA\x14a\x01{W\x80ct\x87\xE5d\x14a\x01\x8EW\x80c\x91\xD1HT\x14a\x01\xAEW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\0\xFAW\x80c$\x8A\x9C\xA3\x14a\x01\"W\x80c//\xF1]\x14a\x01SW[`\0\x80\xFD[a\x01\ra\x01\x086`\x04a\x0B\x7FV[a\x02\x7FV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01Ea\x0106`\x04a\x0B\xA9V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\x19V[a\x01fa\x01a6`\x04a\x0B\xDEV[a\x02\xB6V[\0[a\x01fa\x01v6`\x04a\x0B\xDEV[a\x02\xE0V[a\x01fa\x01\x896`\x04a\x0CSV[a\x03cV[a\x01\xA1a\x01\x9C6`\x04a\x0C\xDDV[a\x05\xF7V[`@Qa\x01\x19\x91\x90a\rWV[a\x01\ra\x01\xBC6`\x04a\x0B\xDEV[a\x06\x9CV[a\x01E`\0\x81V[a\x01fa\x01\xD76`\x04a\rjV[a\x06\xC5V[a\x02\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x19V[a\x01Ea\x06\xF2V[a\x01fa\x0216`\x04a\x0B\xDEV[a\x07\x0EV[a\x01\ra\x02D6`\x04a\rjV[`\x02` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x01\xA1a\x02g6`\x04a\x0C\xDDV[a\x073V[a\x01fa\x02z6`\x04a\x0C\xDDV[a\x07WV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x02\xB0WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x02\xD1\x81a\x07\xE0V[a\x02\xDB\x83\x83a\x07\xEDV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x03UW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03_\x82\x82a\x08qV[PPV[a\x03|`\x01`\0\x80Q` a\x10+\x839\x81Q\x91Ra\r\x9BV[a\x03\x85\x81a\x07\xE0V[\x84\x84`\0a\x03\xC8\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x08\xD6\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x03\x83RbG13`\xE8\x1B\x91\x83\x01\x91\x90\x91R\x91\x92P\x90`\xFF\x16\x15a\x04$W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03L\x91\x90a\rWV[P`@QcG;\x05\x7F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x8Ev\n\xFE\x90a\x04s\x90\x89\x90\x89\x90`\x04\x01a\r\xAEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB4\x91\x90a\r\xDDV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a \x9B`\xF1\x1B\x81RP\x90a\x04\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03L\x91\x90a\rWV[P`@\x80Q\x80\x82\x01\x82R`\x02\x81RaG7`\xF0\x1B` \x82\x01R\x90\x88\x14a\x05&W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03L\x91\x90a\rWV[P`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x8C\x84R\x90\x91R\x90 a\x05U\x88\x8A\x83a\x0E\x9DV[P`\0a\x05\x97\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x08\xD6\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x02` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x92\x93P\x8C\x92\x91\x8E\x16\x91\x7FR\x1E=\xAC\x83\xE2\x8EU\xEF\xB9A\xFDXz\xBA\xB2\ty|\xEAqA\xEFP\xAA2\xF0\\\n;\xDB\x17\x91\x90\xA3PPPPPPPPPPPV[`\x03` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80Ta\x06\x1B\x90a\x0E\x15V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06G\x90a\x0E\x15V[\x80\x15a\x06\x94W\x80`\x1F\x10a\x06iWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x94V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06wW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0a\x06\xD0\x81a\x07\xE0V[a\x03_a\x06\xEC`\x01`\0\x80Q` a\x10+\x839\x81Q\x91Ra\r\x9BV[\x83a\x07\xEDV[a\x07\x0B`\x01`\0\x80Q` a\x10+\x839\x81Q\x91Ra\r\x9BV[\x81V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x07)\x81a\x07\xE0V[a\x02\xDB\x83\x83a\x08qV[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80Ta\x06\x1B\x90a\x0E\x15V[a\x07p`\x01`\0\x80Q` a\x10+\x839\x81Q\x91Ra\r\x9BV[a\x07y\x81a\x07\xE0V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 a\x07\xA5\x91a\x0B1V[`@Q\x82\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xD4\x08O{z\x07\x9E|\xBF\xEE\x05\x96\xA4r1\xBAK\x1F\x97\x83a,\xB2r\x0E\xDE{\xF3\x8Eg\x10\xA2\x90`\0\x90\xA3PPPV[a\x07\xEA\x813a\t#V[PV[a\x07\xF7\x82\x82a\x06\x9CV[a\x03_W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x08-3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x08{\x82\x82a\x06\x9CV[\x15a\x03_W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0\x81Q`@\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG7`\xF0\x1B\x81RP\x90a\t\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03L\x91\x90a\rWV[PP\x80Q` \x90\x91\x01 \x90V[a\t-\x82\x82a\x06\x9CV[a\x03_Wa\t:\x81a\t|V[a\tE\x83` a\t\x8EV[`@Q` \x01a\tV\x92\x91\x90a\x0F^V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03L\x91`\x04\x01a\rWV[``a\x02\xB0`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\t\x9D\x83`\x02a\x0F\xD3V[a\t\xA8\x90`\x02a\x0F\xEAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xC0Wa\t\xC0a\r\xFFV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\t\xEAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\n\x05Wa\n\x05a\x0F\xFDV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\n4Wa\n4a\x0F\xFDV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\nX\x84`\x02a\x0F\xD3V[a\nc\x90`\x01a\x0F\xEAV[\x90P[`\x01\x81\x11\x15a\n\xDBWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\n\x97Wa\n\x97a\x0F\xFDV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\n\xADWa\n\xADa\x0F\xFDV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\n\xD4\x81a\x10\x13V[\x90Pa\nfV[P\x83\x15a\x0B*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x03LV[\x93\x92PPPV[P\x80Ta\x0B=\x90a\x0E\x15V[`\0\x82U\x80`\x1F\x10a\x0BMWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x07\xEA\x91\x90[\x80\x82\x11\x15a\x0B{W`\0\x81U`\x01\x01a\x0BgV[P\x90V[`\0` \x82\x84\x03\x12\x15a\x0B\x91W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0B*W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0B\xBBW`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xD9W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xF1W`\0\x80\xFD[\x825\x91Pa\x0C\x01` \x84\x01a\x0B\xC2V[\x90P\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x0C\x1CW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C4W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0CLW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\x0ClW`\0\x80\xFD[a\x0Cu\x87a\x0B\xC2V[\x95P` \x87\x015\x94P`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\x99W`\0\x80\xFD[a\x0C\xA5\x8A\x83\x8B\x01a\x0C\nV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15a\x0C\xBEW`\0\x80\xFD[Pa\x0C\xCB\x89\x82\x8A\x01a\x0C\nV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xF0W`\0\x80\xFD[a\x0C\xF9\x83a\x0B\xC2V[\x94` \x93\x90\x93\x015\x93PPPV[`\0[\x83\x81\x10\x15a\r\"W\x81\x81\x01Q\x83\x82\x01R` \x01a\r\nV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\rC\x81` \x86\x01` \x86\x01a\r\x07V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0B*` \x83\x01\x84a\r+V[`\0` \x82\x84\x03\x12\x15a\r|W`\0\x80\xFD[a\x0B*\x82a\x0B\xC2V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\xB0Wa\x02\xB0a\r\x85V[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\r\xEFW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0B*W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0E)W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0EIWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x02\xDBW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0EvWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0E\x95W\x82\x81U`\x01\x01a\x0E\x82V[PPPPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x0E\xB5Wa\x0E\xB5a\r\xFFV[a\x0E\xC9\x83a\x0E\xC3\x83Ta\x0E\x15V[\x83a\x0EOV[`\0`\x1F\x84\x11`\x01\x81\x14a\x0E\xFDW`\0\x85\x15a\x0E\xE5WP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x0FWV[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x0F.W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x0F\x0EV[P\x86\x82\x10\x15a\x0FKW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x0F\x96\x81`\x17\x85\x01` \x88\x01a\r\x07V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x0F\xC7\x81`(\x84\x01` \x88\x01a\r\x07V[\x01`(\x01\x94\x93PPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\xB0Wa\x02\xB0a\r\x85V[\x80\x82\x01\x80\x82\x11\x15a\x02\xB0Wa\x02\xB0a\r\x85V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a\x10\"Wa\x10\"a\r\x85V[P`\0\x19\x01\x90V\xFE\xDCi|\xE6\xC0\xC3\xF8\x86\xFB\xBC\x8E\r#:\xBF\xACZ\xC56S\xAC\x08\xB3c[\xB0\xAE\xC7\x0C9\\B\xA2dipfsX\"\x12 \x1E\xE6\x0E\x9A[\xD8\x04%l\xD9\x167n\xB8P\xAC\x97\x12t\x80rfZ\x05\x0C\xE8p\xC2\xF6u\xE2\xBCdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static ENTITYKEYREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct EntityKeyRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EntityKeyRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EntityKeyRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EntityKeyRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EntityKeyRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EntityKeyRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EntityKeyRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ENTITYKEYREGISTRY_ABI.clone(),
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
                ENTITYKEYREGISTRY_ABI.clone(),
                ENTITYKEYREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `KEY_REGISTER_ROLE` (0xb80aaa89) function
        pub fn key_register_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([184, 10, 170, 137], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addGeneratorRegistry` (0xaa0ed09f) function
        pub fn add_generator_registry(
            &self,
            generator_registry: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 14, 208, 159], generator_registry)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `attestationVerifier` (0xb4b3c5a0) function
        pub fn attestation_verifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([180, 179, 197, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dedicated_pub_key_per_market` (0x7487e564) function
        pub fn dedicated_pub_key_per_market(
            &self,
            p0: ::ethers::core::types::Address,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([116, 135, 229, 100], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pub_key` (0xe1a63130) function
        pub fn pub_key(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([225, 166, 49, 48], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removePubkey` (0xed380d03) function
        pub fn remove_pubkey(
            &self,
            key_owner: ::ethers::core::types::Address,
            key_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 56, 13, 3], (key_owner, key_index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePubkey` (0x69fdbcca) function
        pub fn update_pubkey(
            &self,
            key_owner: ::ethers::core::types::Address,
            key_index: ::ethers::core::types::U256,
            pubkey: ::ethers::core::types::Bytes,
            attestation_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [105, 253, 188, 202],
                    (key_owner, key_index, pubkey, attestation_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `usedUpKey` (0xd703f607) function
        pub fn used_up_key(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([215, 3, 246, 7], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `RemoveKey` event
        pub fn remove_key_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemoveKeyFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `UpdateKey` event
        pub fn update_key_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpdateKeyFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EntityKeyRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for EntityKeyRegistry<M> {
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
    #[ethevent(name = "RemoveKey", abi = "RemoveKey(address,uint256)")]
    pub struct RemoveKeyFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub key_index: ::ethers::core::types::U256,
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
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "UpdateKey", abi = "UpdateKey(address,uint256)")]
    pub struct UpdateKeyFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub key_index: ::ethers::core::types::U256,
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
    pub enum EntityKeyRegistryEvents {
        RemoveKeyFilter(RemoveKeyFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        UpdateKeyFilter(UpdateKeyFilter),
    }
    impl ::ethers::contract::EthLogDecode for EntityKeyRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = RemoveKeyFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::RemoveKeyFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = UpdateKeyFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::UpdateKeyFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for EntityKeyRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RemoveKeyFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateKeyFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RemoveKeyFilter> for EntityKeyRegistryEvents {
        fn from(value: RemoveKeyFilter) -> Self {
            Self::RemoveKeyFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for EntityKeyRegistryEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for EntityKeyRegistryEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for EntityKeyRegistryEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<UpdateKeyFilter> for EntityKeyRegistryEvents {
        fn from(value: UpdateKeyFilter) -> Self {
            Self::UpdateKeyFilter(value)
        }
    }
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `KEY_REGISTER_ROLE` function with signature `KEY_REGISTER_ROLE()` and selector `0xb80aaa89`
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
    #[ethcall(name = "KEY_REGISTER_ROLE", abi = "KEY_REGISTER_ROLE()")]
    pub struct KeyRegisterRoleCall;
    ///Container type for all input parameters for the `addGeneratorRegistry` function with signature `addGeneratorRegistry(address)` and selector `0xaa0ed09f`
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
    #[ethcall(name = "addGeneratorRegistry", abi = "addGeneratorRegistry(address)")]
    pub struct AddGeneratorRegistryCall {
        pub generator_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `attestationVerifier` function with signature `attestationVerifier()` and selector `0xb4b3c5a0`
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
    #[ethcall(name = "attestationVerifier", abi = "attestationVerifier()")]
    pub struct AttestationVerifierCall;
    ///Container type for all input parameters for the `dedicated_pub_key_per_market` function with signature `dedicated_pub_key_per_market(address,bytes32)` and selector `0x7487e564`
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
        name = "dedicated_pub_key_per_market",
        abi = "dedicated_pub_key_per_market(address,bytes32)"
    )]
    pub struct DedicatedPubKeyPerMarketCall(
        pub ::ethers::core::types::Address,
        pub [u8; 32],
    );
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `pub_key` function with signature `pub_key(address,uint256)` and selector `0xe1a63130`
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
    #[ethcall(name = "pub_key", abi = "pub_key(address,uint256)")]
    pub struct PubKeyCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `removePubkey` function with signature `removePubkey(address,uint256)` and selector `0xed380d03`
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
    #[ethcall(name = "removePubkey", abi = "removePubkey(address,uint256)")]
    pub struct RemovePubkeyCall {
        pub key_owner: ::ethers::core::types::Address,
        pub key_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `updatePubkey` function with signature `updatePubkey(address,uint256,bytes,bytes)` and selector `0x69fdbcca`
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
    #[ethcall(name = "updatePubkey", abi = "updatePubkey(address,uint256,bytes,bytes)")]
    pub struct UpdatePubkeyCall {
        pub key_owner: ::ethers::core::types::Address,
        pub key_index: ::ethers::core::types::U256,
        pub pubkey: ::ethers::core::types::Bytes,
        pub attestation_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `usedUpKey` function with signature `usedUpKey(address)` and selector `0xd703f607`
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
    #[ethcall(name = "usedUpKey", abi = "usedUpKey(address)")]
    pub struct UsedUpKeyCall(pub ::ethers::core::types::Address);
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
    pub enum EntityKeyRegistryCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        KeyRegisterRole(KeyRegisterRoleCall),
        AddGeneratorRegistry(AddGeneratorRegistryCall),
        AttestationVerifier(AttestationVerifierCall),
        DedicatedPubKeyPerMarket(DedicatedPubKeyPerMarketCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        PubKey(PubKeyCall),
        RemovePubkey(RemovePubkeyCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SupportsInterface(SupportsInterfaceCall),
        UpdatePubkey(UpdatePubkeyCall),
        UsedUpKey(UsedUpKeyCall),
    }
    impl ::ethers::core::abi::AbiDecode for EntityKeyRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <KeyRegisterRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::KeyRegisterRole(decoded));
            }
            if let Ok(decoded) = <AddGeneratorRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddGeneratorRegistry(decoded));
            }
            if let Ok(decoded) = <AttestationVerifierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AttestationVerifier(decoded));
            }
            if let Ok(decoded) = <DedicatedPubKeyPerMarketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DedicatedPubKeyPerMarket(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) = <PubKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PubKey(decoded));
            }
            if let Ok(decoded) = <RemovePubkeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemovePubkey(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <UpdatePubkeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdatePubkey(decoded));
            }
            if let Ok(decoded) = <UsedUpKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UsedUpKey(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EntityKeyRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KeyRegisterRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddGeneratorRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationVerifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DedicatedPubKeyPerMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PubKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemovePubkey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdatePubkey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UsedUpKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for EntityKeyRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::KeyRegisterRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddGeneratorRegistry(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationVerifier(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DedicatedPubKeyPerMarket(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovePubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdatePubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::UsedUpKey(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for EntityKeyRegistryCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<KeyRegisterRoleCall> for EntityKeyRegistryCalls {
        fn from(value: KeyRegisterRoleCall) -> Self {
            Self::KeyRegisterRole(value)
        }
    }
    impl ::core::convert::From<AddGeneratorRegistryCall> for EntityKeyRegistryCalls {
        fn from(value: AddGeneratorRegistryCall) -> Self {
            Self::AddGeneratorRegistry(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierCall> for EntityKeyRegistryCalls {
        fn from(value: AttestationVerifierCall) -> Self {
            Self::AttestationVerifier(value)
        }
    }
    impl ::core::convert::From<DedicatedPubKeyPerMarketCall> for EntityKeyRegistryCalls {
        fn from(value: DedicatedPubKeyPerMarketCall) -> Self {
            Self::DedicatedPubKeyPerMarket(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for EntityKeyRegistryCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for EntityKeyRegistryCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for EntityKeyRegistryCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<PubKeyCall> for EntityKeyRegistryCalls {
        fn from(value: PubKeyCall) -> Self {
            Self::PubKey(value)
        }
    }
    impl ::core::convert::From<RemovePubkeyCall> for EntityKeyRegistryCalls {
        fn from(value: RemovePubkeyCall) -> Self {
            Self::RemovePubkey(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for EntityKeyRegistryCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for EntityKeyRegistryCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for EntityKeyRegistryCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UpdatePubkeyCall> for EntityKeyRegistryCalls {
        fn from(value: UpdatePubkeyCall) -> Self {
            Self::UpdatePubkey(value)
        }
    }
    impl ::core::convert::From<UsedUpKeyCall> for EntityKeyRegistryCalls {
        fn from(value: UsedUpKeyCall) -> Self {
            Self::UsedUpKey(value)
        }
    }
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `KEY_REGISTER_ROLE` function with signature `KEY_REGISTER_ROLE()` and selector `0xb80aaa89`
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
    pub struct KeyRegisterRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `attestationVerifier` function with signature `attestationVerifier()` and selector `0xb4b3c5a0`
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
    pub struct AttestationVerifierReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `dedicated_pub_key_per_market` function with signature `dedicated_pub_key_per_market(address,bytes32)` and selector `0x7487e564`
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
    pub struct DedicatedPubKeyPerMarketReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `pub_key` function with signature `pub_key(address,uint256)` and selector `0xe1a63130`
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
    pub struct PubKeyReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `usedUpKey` function with signature `usedUpKey(address)` and selector `0xd703f607`
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
    pub struct UsedUpKeyReturn(pub bool);
}
