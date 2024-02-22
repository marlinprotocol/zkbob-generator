pub use attestation_verifier::*;
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
pub mod attestation_verifier {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ATTESTATION_PREFIX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ATTESTATION_PREFIX"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleMember"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMember"),
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
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("images"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct AttestationVerifier.EnclaveImage[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveKeys"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isVerified"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isVerified"),
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
                    ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proxiableUUID"),
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
                    ::std::borrow::ToOwned::to_owned("revokeWhitelistedEnclave"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "revokeWhitelistedEnclave",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveKey"),
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
                    ::std::borrow::ToOwned::to_owned("revokeWhitelistedImage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "revokeWhitelistedImage",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
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
                    ::std::borrow::ToOwned::to_owned("safeVerify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeVerify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeVerify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("attestation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceEnclaveKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCR0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCR1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCR2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveCPUs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveMemory"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("upgradeTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
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
                    ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("attestation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceEnclaveKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCR0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCR1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCR2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveCPUs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveMemory"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifyEnclaveKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifyEnclaveKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("attestation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceEnclaveKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveCPUs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveMemory"),
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
                    ::std::borrow::ToOwned::to_owned("whitelistEnclaveKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "whitelistEnclaveKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
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
                    ::std::borrow::ToOwned::to_owned("whitelistImage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("whitelistImage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCR0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCR1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCR2"),
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
                    ::std::borrow::ToOwned::to_owned("whitelistedImages"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("whitelistedImages"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("PCR0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCR1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCR2"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BeaconUpgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BeaconUpgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("beacon"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveImageWhitelisted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EnclaveImageWhitelisted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("PCR0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("PCR1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("PCR2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveKeyVerified"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EnclaveKeyVerified"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
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
                    ::std::borrow::ToOwned::to_owned("EnclaveKeyWhitelisted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EnclaveKeyWhitelisted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
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
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WhitelistedEnclaveKeyRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WhitelistedEnclaveKeyRevoked",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
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
                    ::std::borrow::ToOwned::to_owned("WhitelistedImageRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WhitelistedImageRevoked",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ATTESTATIONVERIFIER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R0`\x80R4\x80\x15b\0\0\x15W`\0\x80\xFD[P`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\x007WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\0SWP0;\x15\x80\x15b\0\0SWP`\0T`\xFF\x16`\x01\x14[b\0\0\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\0\xDFW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[\x80\x15b\0\x01&W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[P`\x80Qa2\xB7b\0\x01_`\09`\0\x81\x81a\t\x19\x01R\x81\x81a\tY\x01R\x81\x81a\t\xF8\x01R\x81\x81a\n8\x01Ra\n\xC7\x01Ra2\xB7`\0\xF3\xFE`\x80`@R`\x046\x10a\x01fW`\x005`\xE0\x1C\x80c\x91\xD1HT\x11a\0\xD1W\x80c\xCDE[\xD9\x11a\0\x8AW\x80c\xE3\xA1*\xA3\x11a\0dW\x80c\xE3\xA1*\xA3\x14a\x04RW\x80c\xEF\x98\xBE?\x14a\x04rW\x80c\xF4\x8FZ\xB6\x14a\x04\x92W\x80c\xF5\xE6O\x92\x14a\x04\xB2W`\0\x80\xFD[\x80c\xCDE[\xD9\x14a\x03\xF2W\x80c\xD5Gt\x1F\x14a\x04\x12W\x80c\xDA5\x0Er\x14a\x042W`\0\x80\xFD[\x80c\x91\xD1HT\x14a\x03/W\x80c\x94\x8BDY\x14a\x03OW\x80c\xA2\x17\xFD\xDF\x14a\x03oW\x80c\xB2GI\x9B\x14a\x03\x84W\x80c\xB9 \x9E3\x14a\x03\xA4W\x80c\xCA\x15\xC8s\x14a\x03\xD2W`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\x01#W\x80c6V\x8A\xBE\x14a\x02oW\x80c6Y\xCF\xE6\x14a\x02\x8FW\x80cO\x1E\xF2\x86\x14a\x02\xAFW\x80cR\xD1\x90-\x14a\x02\xC2W\x80c\x8Ev\n\xFE\x14a\x02\xD7W\x80c\x90\x10\xD0|\x14a\x02\xF7W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01kW\x80c\x06\xD5\xA2\xED\x14a\x01\xA0W\x80c\x12UJ\x8D\x14a\x01\xC2W\x80c$\x8A\x9C\xA3\x14a\x01\xE2W\x80c//\xF1]\x14a\x02 W\x80c/\x9B\n\xD7\x14a\x02@W[`\0\x80\xFD[4\x80\x15a\x01wW`\0\x80\xFD[Pa\x01\x8Ba\x01\x866`\x04a%\xFDV[a\x05\x08V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xACW`\0\x80\xFD[Pa\x01\xC0a\x01\xBB6`\x04a'\x12V[a\x05\x19V[\0[4\x80\x15a\x01\xCEW`\0\x80\xFD[Pa\x01\xC0a\x01\xDD6`\x04a'kV[a\x05\xCFV[4\x80\x15a\x01\xEEW`\0\x80\xFD[Pa\x02\x12a\x01\xFD6`\x04a'\x88V[`\0\x90\x81R`e` R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\x97V[4\x80\x15a\x02,W`\0\x80\xFD[Pa\x01\xC0a\x02;6`\x04a'\xA1V[a\x06\xACV[4\x80\x15a\x02LW`\0\x80\xFD[Pa\x02`a\x02[6`\x04a'\x88V[a\x06\xD6V[`@Qa\x01\x97\x93\x92\x91\x90a(!V[4\x80\x15a\x02{W`\0\x80\xFD[Pa\x01\xC0a\x02\x8A6`\x04a'\xA1V[a\x08\x91V[4\x80\x15a\x02\x9BW`\0\x80\xFD[Pa\x01\xC0a\x02\xAA6`\x04a'kV[a\t\x0FV[a\x01\xC0a\x02\xBD6`\x04a(ZV[a\t\xEEV[4\x80\x15a\x02\xCEW`\0\x80\xFD[Pa\x02\x12a\n\xBAV[4\x80\x15a\x02\xE3W`\0\x80\xFD[Pa\x01\x8Ba\x02\xF26`\x04a'\x12V[a\x0BmV[4\x80\x15a\x03\x03W`\0\x80\xFD[Pa\x03\x17a\x03\x126`\x04a(\xA9V[a\x0B\xD5V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x97V[4\x80\x15a\x03;W`\0\x80\xFD[Pa\x01\x8Ba\x03J6`\x04a'\xA1V[a\x0B\xF4V[4\x80\x15a\x03[W`\0\x80\xFD[Pa\x01\xC0a\x03j6`\x04a(\xCBV[a\x0C\x1FV[4\x80\x15a\x03{W`\0\x80\xFD[Pa\x02\x12`\0\x81V[4\x80\x15a\x03\x90W`\0\x80\xFD[Pa\x01\x8Ba\x03\x9F6`\x04a(\xCBV[a\x0C\x9FV[4\x80\x15a\x03\xB0W`\0\x80\xFD[Pa\x02\x12a\x03\xBF6`\x04a'kV[a\x03\"` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x03\xDEW`\0\x80\xFD[Pa\x02\x12a\x03\xED6`\x04a'\x88V[a\x0C\xD6V[4\x80\x15a\x03\xFEW`\0\x80\xFD[Pa\x01\xC0a\x04\r6`\x04a)\xACV[a\x0C\xEDV[4\x80\x15a\x04\x1EW`\0\x80\xFD[Pa\x01\xC0a\x04-6`\x04a'\xA1V[a\r=V[4\x80\x15a\x04>W`\0\x80\xFD[Pa\x01\xC0a\x04M6`\x04a*\xC5V[a\rbV[4\x80\x15a\x04^W`\0\x80\xFD[Pa\x01\xC0a\x04m6`\x04a,&V[a\x10\x11V[4\x80\x15a\x04~W`\0\x80\xFD[Pa\x01\xC0a\x04\x8D6`\x04a'\x88V[a\x11\xB3V[4\x80\x15a\x04\x9EW`\0\x80\xFD[Pa\x01\xC0a\x04\xAD6`\x04a,RV[a\x12\xABV[4\x80\x15a\x04\xBEW`\0\x80\xFD[Pa\x04\xFB`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FEnclave Attestation Verified\0\0\0\0\x81RP\x81V[`@Qa\x01\x97\x91\x90a,\xCFV[`\0a\x05\x13\x82a\x164V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80\x88\x80` \x01\x90Q\x81\x01\x90a\x059\x91\x90a-2V[\x97P\x97P\x97P\x97P\x97P\x97P\x97P\x97P`\0a\x05s\x89\x89\x89`@Q\x80``\x01`@R\x80\x8B\x81R` \x01\x8A\x81R` \x01\x89\x81RP\x87\x87a\x16YV[\x90P\x80a\x05\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx \xAB\x1D)\xAB\x16\xB4\xB7;0\xB64\xB2\x100\xBA:2\xB9\xBA0\xBA4\xB7\xB7`9\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPV[a\x05\xDA`\x003a\x0B\xF4V[a\x05\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.\x12V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x03\"` R`@\x90 Ta\x06\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FAV:RWE-Enclave key not verified\0`D\x82\x01R`d\x01a\x05\xBAV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81Ra\x03\"` R`@\x80\x82 \x80T\x90\x83\x90U\x90Q\x90\x92\x83\x92\x90\x91\x7F~\x96m\xE5\x05=\t\x8C\x1D\xA2\xFEI\xE7F\x99\x81\x15aw\xA9ut@\xF4\xAD\xCD\nN\xF1\xA9\x18\x17\x91\x90\xA3PPV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\x06\xC7\x81a\x17\xE4V[a\x06\xD1\x83\x83a\x17\xEEV[PPPV[a\x03!` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x06\xF2\x90a.6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\x1E\x90a.6V[\x80\x15a\x07kW\x80`\x1F\x10a\x07@Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07kV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07NW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x07\x80\x90a.6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xAC\x90a.6V[\x80\x15a\x07\xF9W\x80`\x1F\x10a\x07\xCEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xF9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xDCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta\x08\x0E\x90a.6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08:\x90a.6V[\x80\x15a\x08\x87W\x80`\x1F\x10a\x08\\Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x87V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08jW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\t\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[a\t\x0B\x82\x82a\x17\xF8V[PPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\tWW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.pV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\t\xA0`\0\x80Q` a2;\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.\xBCV[a\t\xCF\x81a\x18[V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Ra\t\xEB\x91\x83\x91\x90a\x18\x82V[PV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\n6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.pV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\n\x7F`\0\x80Q` a2;\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\n\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.\xBCV[a\n\xAE\x82a\x18[V[a\t\x0B\x82\x82`\x01a\x18\x82V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0BZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xBAV[P`\0\x80Q` a2;\x839\x81Q\x91R\x90V[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x89\x80` \x01\x90Q\x81\x01\x90a\x0B\x8F\x91\x90a-2V[\x97P\x97P\x97P\x97P\x97P\x97P\x97P\x97Pa\x0B\xC7\x88\x88\x88`@Q\x80``\x01`@R\x80\x8A\x81R` \x01\x89\x81R` \x01\x88\x81RP\x86\x86a\x16YV[\x9A\x99PPPPPPPPPPV[`\0\x82\x81R`\x97` R`@\x81 a\x0B\xED\x90\x83a\x19\xEDV[\x93\x92PPPV[`\0\x91\x82R`e` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0a\x0CI\x89\x89\x89`@Q\x80``\x01`@R\x80\x8B\x81R` \x01\x8A\x81R` \x01\x89\x81RP\x87\x87a\x16YV[\x90P\x80a\x0C\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx \xAB\x1D)\xAB\x16\xB4\xB7;0\xB64\xB2\x100\xBA:2\xB9\xBA0\xBA4\xB7\xB7`9\x1B`D\x82\x01R`d\x01a\x05\xBAV[PPPPPPPPPV[`\0a\x0C\xC9\x89\x89\x89`@Q\x80``\x01`@R\x80\x8B\x81R` \x01\x8A\x81R` \x01\x89\x81RP\x87\x87a\x16YV[\x99\x98PPPPPPPPPV[`\0\x81\x81R`\x97` R`@\x81 a\x05\x13\x90a\x19\xF9V[a\x0C\xF8`\x003a\x0B\xF4V[a\r\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.\x12V[a\r7`@Q\x80``\x01`@R\x80\x85\x81R` \x01\x84\x81R` \x01\x83\x81RPa\x1A\x03V[PPPPV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\rX\x81a\x17\xE4V[a\x06\xD1\x83\x83a\x17\xF8V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\r\x82WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\r\x9CWP0;\x15\x80\x15a\r\x9CWP`\0T`\xFF\x16`\x01\x14[a\r\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x0E\"W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[\x83Q`\0\x03a\x0E\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FAV:I-At least one image must be `D\x82\x01Rg\x1C\x1C\x9B\xDD\x9AY\x19Y`\xC2\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[\x82Q\x84Q\x14a\x0E\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FAV:I-Image and key length mismat`D\x82\x01Ra\x0Cm`\xF3\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[a\x0E\xE8a\x1BvV[a\x0E\xF0a\x1BvV[a\x0E\xF8a\x1BvV[a\x0F\0a\x1BvV[a\x0F\x08a\x1BvV[a\x0F\x10a\x1BvV[a\x0F\x1B`\0\x83a\x1B\xE3V[`\0[\x83Q\x81\x10\x15a\x0F\xC5W`\0\x84\x82\x81Q\x81\x10a\x0F;Wa\x0F;a/\x08V[` \x02` \x01\x01Q\x90P`\0a\x0Fi\x87\x84\x81Q\x81\x10a\x0F\\Wa\x0F\\a/\x08V[` \x02` \x01\x01Qa\x1A\x03V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x03\"` R`@\x80\x82 \x84\x90UQ\x92\x93P\x83\x92\x7FY+\xBC\xEF\xA9\xF4dD\xAFY\xF1{\x05\xA4\xB6 \xFA\xFD\x14\x05\x03P\x06\xAET\x8E/ (+;\xC7\x91\x90\xA3PP\x80\x80a\x0F\xBD\x90a/4V[\x91PPa\x0F\x1EV[P\x80\x15a\r7W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPV[a\x10\x1C`\x003a\x0B\xF4V[a\x108W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.\x12V[`\0\x81\x81Ra\x03!` R`@\x90 \x80Ta\x10R\x90a.6V[\x90P`\0\x03a\x10\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FAV:WE-Image not whitelisted\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x10\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FAV:WE-Invalid enclave key\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03\"` R`@\x90 T\x15a\x11kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FAV:WE-Enclave key already verifi`D\x82\x01Ra\x19Y`\xF2\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x03\"` R`@\x80\x82 \x84\x90UQ\x83\x92\x91\x7FY+\xBC\xEF\xA9\xF4dD\xAFY\xF1{\x05\xA4\xB6 \xFA\xFD\x14\x05\x03P\x06\xAET\x8E/ (+;\xC7\x91\xA3PPV[a\x11\xBE`\x003a\x0B\xF4V[a\x11\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.\x12V[`\0\x81\x81Ra\x03!` R`@\x90 \x80Ta\x11\xF4\x90a.6V[\x90P`\0\x03a\x12EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FAV:RWI-Image not whitelisted\0\0\0\0`D\x82\x01R`d\x01a\x05\xBAV[`\0\x81\x81Ra\x03!` R`@\x81 \x90a\x12_\x82\x82a%\xAFV[a\x12m`\x01\x83\x01`\0a%\xAFV[a\x12{`\x02\x83\x01`\0a%\xAFV[PP`@Q\x81\x90\x7F\xCB\x96\xC5\xD9H\xD3(ED\xD8&t\x19I\xDE\x7F\x8C\xA8s\xD4\r(t\\b\xEB\xE6\x95\xE2I\xBEo\x90`\0\x90\xA2PV[`\0\x83\x81Ra\x03!` R`@\x90 \x80Ta\x12\xC5\x90a.6V[\x90P`\0\x03a\x13+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAV:V-Enclave image to verify not`D\x82\x01Rk\x08\x1D\xDA\x1A]\x19[\x1A\\\xDD\x19Y`\xA2\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x03\"` R`@\x90 T\x15a\x13\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FAV:V-Enclave key already verifie`D\x82\x01R`\x19`\xFA\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\0\x83\x81Ra\x03!` R`@\x80\x82 \x81Q``\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x13\xC4\x90a.6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\xF0\x90a.6V[\x80\x15a\x14=W\x80`\x1F\x10a\x14\x12Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14=V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14 W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x14V\x90a.6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\x82\x90a.6V[\x80\x15a\x14\xCFW\x80`\x1F\x10a\x14\xA4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14\xCFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\xB2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x14\xE8\x90a.6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x14\x90a.6V[\x80\x15a\x15aW\x80`\x1F\x10a\x156Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15aV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0a\x15|\x88\x88\x88\x85\x88\x88a\x16YV[\x90P\x80a\x15\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FAV:VE-Attestation must be signed`D\x82\x01Rq by source enclave`p\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x81\x81Ra\x03\"` R`@\x80\x82 \x88\x90UQ\x87\x92\x91\x7F\x84\xD3(\xB8\xB8\xEA({#\xCF\x84\x99\x1E,]\x82ot\x04%\xFA_\xA6\xB3\xFF\x1C\xF3\xBC\xF4\x03\\v\x91\xA3PPPPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x05\x13WPa\x05\x13\x82a\x1B\xEDV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81Ra\x03\"` R`@\x81 T\x80a\x16\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FAV:V-Enclave key must be verifie`D\x82\x01R`\x19`\xFA\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\0\x81\x81Ra\x03!` R`@\x90 \x80Ta\x16\xE4\x90a.6V[\x90P`\0\x03a\x17CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FAV:V-Source image must be whitel`D\x82\x01Rd\x1A\\\xDD\x19Y`\xDA\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\0`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FEnclave Attestation Verified\0\0\0\0\x81RP\x87\x87`\0\x01Q\x88` \x01Q\x89`@\x01Q\x89\x89`@Q` \x01a\x17\xA3\x97\x96\x95\x94\x93\x92\x91\x90a/MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x17\xC7\x82\x8Ba\x1C\"V[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x91\x16\x14\x93PPPP\x96\x95PPPPPPV[a\t\xEB\x813a\x1CFV[a\t\x0B\x82\x82a\x1C\x9FV[a\x18\x02\x82\x82a\x1C\xC1V[a\x18\x0C`\0a\x0C\xD6V[`\0\x03a\t\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FAV:RR-All admins cant be removed`D\x82\x01R`d\x01a\x05\xBAV[a\x18f`\x003a\x0B\xF4V[a\t\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.\x12V[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a\x18\xB5Wa\x06\xD1\x83a\x1C\xE3V[\x82`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x19\x0FWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x19\x0C\x91\x81\x01\x90a/\xC0V[`\x01[a\x19rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01Rmon is not UUPS`\x90\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\0\x80Q` a2;\x839\x81Q\x91R\x81\x14a\x19\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01Rh\x1AXX\x9B\x19UURQ`\xBA\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[Pa\x06\xD1\x83\x83\x83a\x1D\x7FV[`\0a\x0B\xED\x83\x83a\x1D\xA4V[`\0a\x05\x13\x82T\x90V[\x80QQ`\0\x90`0\x14\x80\x15a\x1A\x1DWP\x81` \x01QQ`0\x14[\x80\x15a\x1A.WP\x81`@\x01QQ`0\x14[a\x1A\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FAV:IWI-PCR values must be 48 byt`D\x82\x01Raes`\xF0\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\0\x82`\0\x01Q\x83` \x01Q\x84`@\x01Q`@Q` \x01a\x1A\xA8\x93\x92\x91\x90a/\xD9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 ``\x84\x01\x83R\x86Q\x84R\x86\x82\x01Q\x84\x83\x01R\x86\x83\x01Q\x84\x84\x01R`\0\x81\x81Ra\x03!\x90\x92R\x91\x90 \x82Q\x91\x93P\x90\x81\x90a\x1A\xFA\x90\x82a0jV[P` \x82\x01Q`\x01\x82\x01\x90a\x1B\x0F\x90\x82a0jV[P`@\x82\x01Q`\x02\x82\x01\x90a\x1B$\x90\x82a0jV[P\x90PP\x80\x7FR\xB2\x9B\xBD\xD9z\xB9\x83A\x9FP\xFA\x15\x90\xE5\xABu\xE9\x94\"y\xE9^\x10\xA0\x86\x07\xB0l##\x8B\x84`\0\x01Q\x85` \x01Q\x86`@\x01Q`@Qa\x1Bh\x93\x92\x91\x90a(!V[`@Q\x80\x91\x03\x90\xA2\x92\x91PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1B\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[V[a\t\x0B\x82\x82a\x17\xEEV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x05\x13WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x05\x13V[`\0\x80`\0a\x1C1\x85\x85a\x1D\xCEV[\x91P\x91Pa\x1C>\x81a\x1E\x13V[P\x93\x92PPPV[a\x1CP\x82\x82a\x0B\xF4V[a\t\x0BWa\x1C]\x81a\x1F]V[a\x1Ch\x83` a\x1FoV[`@Q` \x01a\x1Cy\x92\x91\x90a1)V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x05\xBA\x91`\x04\x01a,\xCFV[a\x1C\xA9\x82\x82a!\nV[`\0\x82\x81R`\x97` R`@\x90 a\x06\xD1\x90\x82a!\x90V[a\x1C\xCB\x82\x82a!\xA5V[`\0\x82\x81R`\x97` R`@\x90 a\x06\xD1\x90\x82a\"\x0CV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x1DPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\0\x80Q` a2;\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x1D\x88\x83a\"!V[`\0\x82Q\x11\x80a\x1D\x95WP\x80[\x15a\x06\xD1Wa\r7\x83\x83a\"aV[`\0\x82`\0\x01\x82\x81T\x81\x10a\x1D\xBBWa\x1D\xBBa/\x08V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0\x80\x82Q`A\x03a\x1E\x04W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x1D\xF8\x87\x82\x85\x85a\"\x86V[\x94P\x94PPPPa\x1E\x0CV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x1E'Wa\x1E'a1\x9EV[\x03a\x1E/WPV[`\x01\x81`\x04\x81\x11\x15a\x1ECWa\x1ECa1\x9EV[\x03a\x1E\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xBAV[`\x02\x81`\x04\x81\x11\x15a\x1E\xA4Wa\x1E\xA4a1\x9EV[\x03a\x1E\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x05\xBAV[`\x03\x81`\x04\x81\x11\x15a\x1F\x05Wa\x1F\x05a1\x9EV[\x03a\t\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[``a\x05\x13`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x1F~\x83`\x02a1\xB4V[a\x1F\x89\x90`\x02a1\xCBV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xA0Wa\x1F\xA0a&'V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1F\xCAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x1F\xE5Wa\x1F\xE5a/\x08V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a \x14Wa \x14a/\x08V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a 8\x84`\x02a1\xB4V[a C\x90`\x01a1\xCBV[\x90P[`\x01\x81\x11\x15a \xBBWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a wWa wa/\x08V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a \x8DWa \x8Da/\x08V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a \xB4\x81a1\xDEV[\x90Pa FV[P\x83\x15a\x0B\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x05\xBAV[a!\x14\x82\x82a\x0B\xF4V[a\t\x0BW`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua!L3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x0B\xED\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a#JV[a!\xAF\x82\x82a\x0B\xF4V[\x15a\t\x0BW`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x0B\xED\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a#\x99V[a\"*\x81a\x1C\xE3V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a\x0B\xED\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a2[`'\x919a$\x8CV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\"\xBDWP`\0\x90P`\x03a#AV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a#\x11W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a#:W`\0`\x01\x92P\x92PPa#AV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta#\x91WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x05\x13V[P`\0a\x05\x13V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a$\x82W`\0a#\xBD`\x01\x83a1\xF5V[\x85T\x90\x91P`\0\x90a#\xD1\x90`\x01\x90a1\xF5V[\x90P\x81\x81\x14a$6W`\0\x86`\0\x01\x82\x81T\x81\x10a#\xF1Wa#\xF1a/\x08V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a$\x14Wa$\x14a/\x08V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a$GWa$Ga2\x08V[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x05\x13V[`\0\x91PPa\x05\x13V[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa$\xA9\x91\x90a2\x1EV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a$\xE4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a$\xE9V[``\x91P[P\x91P\x91Pa$\xFA\x86\x83\x83\x87a%\x04V[\x96\x95PPPPPPV[``\x83\x15a%sW\x82Q`\0\x03a%lW`\x01`\x01`\xA0\x1B\x03\x85\x16;a%lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05\xBAV[P\x81a%}V[a%}\x83\x83a%\x85V[\x94\x93PPPPV[\x81Q\x15a%\x95W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x91\x90a,\xCFV[P\x80Ta%\xBB\x90a.6V[`\0\x82U\x80`\x1F\x10a%\xCBWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\t\xEB\x91\x90[\x80\x82\x11\x15a%\xF9W`\0\x81U`\x01\x01a%\xE5V[P\x90V[`\0` \x82\x84\x03\x12\x15a&\x0FW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0B\xEDW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a&_Wa&_a&'V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a&\x8DWa&\x8Da&'V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a&\xAEWa&\xAEa&'V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a&\xCDW`\0\x80\xFD[\x815a&\xE0a&\xDB\x82a&\x95V[a&eV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a&\xF5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a'$W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a':W`\0\x80\xFD[a%}\x84\x82\x85\x01a&\xBCV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xEBW`\0\x80\xFD[\x805a'f\x81a'FV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a'}W`\0\x80\xFD[\x815a\x0B\xED\x81a'FV[`\0` \x82\x84\x03\x12\x15a'\x9AW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a'\xB4W`\0\x80\xFD[\x825\x91P` \x83\x015a'\xC6\x81a'FV[\x80\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a'\xECW\x81\x81\x01Q\x83\x82\x01R` \x01a'\xD4V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra(\r\x81` \x86\x01` \x86\x01a'\xD1V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x81R`\0a(4``\x83\x01\x86a'\xF5V[\x82\x81\x03` \x84\x01Ra(F\x81\x86a'\xF5V[\x90P\x82\x81\x03`@\x84\x01Ra$\xFA\x81\x85a'\xF5V[`\0\x80`@\x83\x85\x03\x12\x15a(mW`\0\x80\xFD[\x825a(x\x81a'FV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\x93W`\0\x80\xFD[a(\x9F\x85\x82\x86\x01a&\xBCV[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a(\xBCW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a(\xE8W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a(\xFFW`\0\x80\xFD[a)\x0B\x8C\x83\x8D\x01a&\xBCV[\x99Pa)\x19` \x8C\x01a'[V[\x98Pa)'`@\x8C\x01a'[V[\x97P``\x8B\x015\x91P\x80\x82\x11\x15a)=W`\0\x80\xFD[a)I\x8C\x83\x8D\x01a&\xBCV[\x96P`\x80\x8B\x015\x91P\x80\x82\x11\x15a)_W`\0\x80\xFD[a)k\x8C\x83\x8D\x01a&\xBCV[\x95P`\xA0\x8B\x015\x91P\x80\x82\x11\x15a)\x81W`\0\x80\xFD[Pa)\x8E\x8B\x82\x8C\x01a&\xBCV[\x93PP`\xC0\x89\x015\x91P`\xE0\x89\x015\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x80`\0``\x84\x86\x03\x12\x15a)\xC1W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a)\xD8W`\0\x80\xFD[a)\xE4\x87\x83\x88\x01a&\xBCV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a)\xFAW`\0\x80\xFD[a*\x06\x87\x83\x88\x01a&\xBCV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a*\x1CW`\0\x80\xFD[Pa*)\x86\x82\x87\x01a&\xBCV[\x91PP\x92P\x92P\x92V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a*LWa*La&'V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a*gW`\0\x80\xFD[\x815` a*wa&\xDB\x83a*3V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a*\x96W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a*\xBAW\x805a*\xAD\x81a'FV[\x83R\x91\x83\x01\x91\x83\x01a*\x9AV[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a*\xDAW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a*\xF1W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a+\x05W`\0\x80\xFD[\x815` a+\x15a&\xDB\x83a*3V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a+4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a+\xEAW\x805\x86\x81\x11\x15a+OW`\0\x80\xFD[\x87\x01``\x81\x8E\x03`\x1F\x19\x01\x12\x15a+eW`\0\x80\xFD[a+ma&=V[\x85\x82\x015\x88\x81\x11\x15a+~W`\0\x80\xFD[a+\x8C\x8F\x88\x83\x86\x01\x01a&\xBCV[\x82RP`@\x82\x015\x88\x81\x11\x15a+\xA2W`\0\x80\x81\xFD[a+\xB0\x8F\x88\x83\x86\x01\x01a&\xBCV[\x87\x83\x01RP``\x82\x015\x88\x81\x11\x15a+\xC8W`\0\x80\x81\xFD[a+\xD6\x8F\x88\x83\x86\x01\x01a&\xBCV[`@\x83\x01RP\x84RP\x91\x83\x01\x91\x83\x01a+8V[P\x97PP\x87\x015\x92PP\x80\x82\x11\x15a,\x01W`\0\x80\xFD[Pa,\x0E\x86\x82\x87\x01a*VV[\x92PPa,\x1D`@\x85\x01a'[V[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a,9W`\0\x80\xFD[\x825a,D\x81a'FV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a,kW`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15a,\x81W`\0\x80\xFD[a,\x8D\x89\x82\x8A\x01a&\xBCV[\x96PP` \x87\x015a,\x9E\x81a'FV[\x94P`@\x87\x015a,\xAE\x81a'FV[\x95\x98\x94\x97P\x94\x95``\x81\x015\x95P`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[` \x81R`\0a\x0B\xED` \x83\x01\x84a'\xF5V[`\0\x82`\x1F\x83\x01\x12a,\xF3W`\0\x80\xFD[\x81Qa-\x01a&\xDB\x82a&\x95V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a-\x16W`\0\x80\xFD[a%}\x82` \x83\x01` \x87\x01a'\xD1V[\x80Qa'f\x81a'FV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a-OW`\0\x80\xFD[\x88Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a-fW`\0\x80\xFD[a-r\x8C\x83\x8D\x01a,\xE2V[\x99Pa-\x80` \x8C\x01a-'V[\x98Pa-\x8E`@\x8C\x01a-'V[\x97P``\x8B\x01Q\x91P\x80\x82\x11\x15a-\xA4W`\0\x80\xFD[a-\xB0\x8C\x83\x8D\x01a,\xE2V[\x96P`\x80\x8B\x01Q\x91P\x80\x82\x11\x15a-\xC6W`\0\x80\xFD[a-\xD2\x8C\x83\x8D\x01a,\xE2V[\x95P`\xA0\x8B\x01Q\x91P\x80\x82\x11\x15a-\xE8W`\0\x80\xFD[Pa-\xF5\x8B\x82\x8C\x01a,\xE2V[`\xC0\x8B\x01Q`\xE0\x90\x9B\x01Q\x99\x9C\x98\x9BP\x96\x99\x95\x98\x94\x97\x93PPPPV[` \x80\x82R`\n\x90\x82\x01Ri7\xB76<\x900\xB26\xB4\xB7`\xB1\x1B`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a.JW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a.jWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rk\x19\x19[\x19Y\xD8]\x19X\xD8[\x1B`\xA2\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rkactive proxy`\xA0\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a/FWa/Fa/\x1EV[P`\x01\x01\x90V[`\xE0\x81R`\0a/``\xE0\x83\x01\x8Aa'\xF5V[`\x01`\x01`\xA0\x1B\x03\x89\x16` \x84\x01R\x82\x81\x03`@\x84\x01Ra/\x81\x81\x89a'\xF5V[\x90P\x82\x81\x03``\x84\x01Ra/\x95\x81\x88a'\xF5V[\x90P\x82\x81\x03`\x80\x84\x01Ra/\xA9\x81\x87a'\xF5V[`\xA0\x84\x01\x95\x90\x95RPP`\xC0\x01R\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a/\xD2W`\0\x80\xFD[PQ\x91\x90PV[`\0\x84Qa/\xEB\x81\x84` \x89\x01a'\xD1V[\x84Q\x90\x83\x01\x90a/\xFF\x81\x83` \x89\x01a'\xD1V[\x84Q\x91\x01\x90a0\x12\x81\x83` \x88\x01a'\xD1V[\x01\x95\x94PPPPPV[`\x1F\x82\x11\x15a\x06\xD1W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a0CWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a0bW\x82\x81U`\x01\x01a0OV[PPPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x83Wa0\x83a&'V[a0\x97\x81a0\x91\x84Ta.6V[\x84a0\x1CV[` \x80`\x1F\x83\x11`\x01\x81\x14a0\xCCW`\0\x84\x15a0\xB4WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua0bV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a0\xFBW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a0\xDCV[P\x85\x82\x10\x15a1\x19W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa1a\x81`\x17\x85\x01` \x88\x01a'\xD1V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa1\x92\x81`(\x84\x01` \x88\x01a'\xD1V[\x01`(\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x13Wa\x05\x13a/\x1EV[\x80\x82\x01\x80\x82\x11\x15a\x05\x13Wa\x05\x13a/\x1EV[`\0\x81a1\xEDWa1\xEDa/\x1EV[P`\0\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x05\x13Wa\x05\x13a/\x1EV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0\x82Qa20\x81\x84` \x87\x01a'\xD1V[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCAddress: low-level delegate call failed\xA2dipfsX\"\x12 Q\xCB\xF6\xCD\xA3\xF6\xCB\xD9\x1E\xDA\x98\xB7)Y}\x83\x12\n><\xA3i\\!p\xD7\xE58\x13\t\xD6\xE5dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static ATTESTATIONVERIFIER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01fW`\x005`\xE0\x1C\x80c\x91\xD1HT\x11a\0\xD1W\x80c\xCDE[\xD9\x11a\0\x8AW\x80c\xE3\xA1*\xA3\x11a\0dW\x80c\xE3\xA1*\xA3\x14a\x04RW\x80c\xEF\x98\xBE?\x14a\x04rW\x80c\xF4\x8FZ\xB6\x14a\x04\x92W\x80c\xF5\xE6O\x92\x14a\x04\xB2W`\0\x80\xFD[\x80c\xCDE[\xD9\x14a\x03\xF2W\x80c\xD5Gt\x1F\x14a\x04\x12W\x80c\xDA5\x0Er\x14a\x042W`\0\x80\xFD[\x80c\x91\xD1HT\x14a\x03/W\x80c\x94\x8BDY\x14a\x03OW\x80c\xA2\x17\xFD\xDF\x14a\x03oW\x80c\xB2GI\x9B\x14a\x03\x84W\x80c\xB9 \x9E3\x14a\x03\xA4W\x80c\xCA\x15\xC8s\x14a\x03\xD2W`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\x01#W\x80c6V\x8A\xBE\x14a\x02oW\x80c6Y\xCF\xE6\x14a\x02\x8FW\x80cO\x1E\xF2\x86\x14a\x02\xAFW\x80cR\xD1\x90-\x14a\x02\xC2W\x80c\x8Ev\n\xFE\x14a\x02\xD7W\x80c\x90\x10\xD0|\x14a\x02\xF7W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01kW\x80c\x06\xD5\xA2\xED\x14a\x01\xA0W\x80c\x12UJ\x8D\x14a\x01\xC2W\x80c$\x8A\x9C\xA3\x14a\x01\xE2W\x80c//\xF1]\x14a\x02 W\x80c/\x9B\n\xD7\x14a\x02@W[`\0\x80\xFD[4\x80\x15a\x01wW`\0\x80\xFD[Pa\x01\x8Ba\x01\x866`\x04a%\xFDV[a\x05\x08V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xACW`\0\x80\xFD[Pa\x01\xC0a\x01\xBB6`\x04a'\x12V[a\x05\x19V[\0[4\x80\x15a\x01\xCEW`\0\x80\xFD[Pa\x01\xC0a\x01\xDD6`\x04a'kV[a\x05\xCFV[4\x80\x15a\x01\xEEW`\0\x80\xFD[Pa\x02\x12a\x01\xFD6`\x04a'\x88V[`\0\x90\x81R`e` R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\x97V[4\x80\x15a\x02,W`\0\x80\xFD[Pa\x01\xC0a\x02;6`\x04a'\xA1V[a\x06\xACV[4\x80\x15a\x02LW`\0\x80\xFD[Pa\x02`a\x02[6`\x04a'\x88V[a\x06\xD6V[`@Qa\x01\x97\x93\x92\x91\x90a(!V[4\x80\x15a\x02{W`\0\x80\xFD[Pa\x01\xC0a\x02\x8A6`\x04a'\xA1V[a\x08\x91V[4\x80\x15a\x02\x9BW`\0\x80\xFD[Pa\x01\xC0a\x02\xAA6`\x04a'kV[a\t\x0FV[a\x01\xC0a\x02\xBD6`\x04a(ZV[a\t\xEEV[4\x80\x15a\x02\xCEW`\0\x80\xFD[Pa\x02\x12a\n\xBAV[4\x80\x15a\x02\xE3W`\0\x80\xFD[Pa\x01\x8Ba\x02\xF26`\x04a'\x12V[a\x0BmV[4\x80\x15a\x03\x03W`\0\x80\xFD[Pa\x03\x17a\x03\x126`\x04a(\xA9V[a\x0B\xD5V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x97V[4\x80\x15a\x03;W`\0\x80\xFD[Pa\x01\x8Ba\x03J6`\x04a'\xA1V[a\x0B\xF4V[4\x80\x15a\x03[W`\0\x80\xFD[Pa\x01\xC0a\x03j6`\x04a(\xCBV[a\x0C\x1FV[4\x80\x15a\x03{W`\0\x80\xFD[Pa\x02\x12`\0\x81V[4\x80\x15a\x03\x90W`\0\x80\xFD[Pa\x01\x8Ba\x03\x9F6`\x04a(\xCBV[a\x0C\x9FV[4\x80\x15a\x03\xB0W`\0\x80\xFD[Pa\x02\x12a\x03\xBF6`\x04a'kV[a\x03\"` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x03\xDEW`\0\x80\xFD[Pa\x02\x12a\x03\xED6`\x04a'\x88V[a\x0C\xD6V[4\x80\x15a\x03\xFEW`\0\x80\xFD[Pa\x01\xC0a\x04\r6`\x04a)\xACV[a\x0C\xEDV[4\x80\x15a\x04\x1EW`\0\x80\xFD[Pa\x01\xC0a\x04-6`\x04a'\xA1V[a\r=V[4\x80\x15a\x04>W`\0\x80\xFD[Pa\x01\xC0a\x04M6`\x04a*\xC5V[a\rbV[4\x80\x15a\x04^W`\0\x80\xFD[Pa\x01\xC0a\x04m6`\x04a,&V[a\x10\x11V[4\x80\x15a\x04~W`\0\x80\xFD[Pa\x01\xC0a\x04\x8D6`\x04a'\x88V[a\x11\xB3V[4\x80\x15a\x04\x9EW`\0\x80\xFD[Pa\x01\xC0a\x04\xAD6`\x04a,RV[a\x12\xABV[4\x80\x15a\x04\xBEW`\0\x80\xFD[Pa\x04\xFB`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FEnclave Attestation Verified\0\0\0\0\x81RP\x81V[`@Qa\x01\x97\x91\x90a,\xCFV[`\0a\x05\x13\x82a\x164V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80\x88\x80` \x01\x90Q\x81\x01\x90a\x059\x91\x90a-2V[\x97P\x97P\x97P\x97P\x97P\x97P\x97P\x97P`\0a\x05s\x89\x89\x89`@Q\x80``\x01`@R\x80\x8B\x81R` \x01\x8A\x81R` \x01\x89\x81RP\x87\x87a\x16YV[\x90P\x80a\x05\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx \xAB\x1D)\xAB\x16\xB4\xB7;0\xB64\xB2\x100\xBA:2\xB9\xBA0\xBA4\xB7\xB7`9\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPV[a\x05\xDA`\x003a\x0B\xF4V[a\x05\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.\x12V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x03\"` R`@\x90 Ta\x06\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FAV:RWE-Enclave key not verified\0`D\x82\x01R`d\x01a\x05\xBAV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81Ra\x03\"` R`@\x80\x82 \x80T\x90\x83\x90U\x90Q\x90\x92\x83\x92\x90\x91\x7F~\x96m\xE5\x05=\t\x8C\x1D\xA2\xFEI\xE7F\x99\x81\x15aw\xA9ut@\xF4\xAD\xCD\nN\xF1\xA9\x18\x17\x91\x90\xA3PPV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\x06\xC7\x81a\x17\xE4V[a\x06\xD1\x83\x83a\x17\xEEV[PPPV[a\x03!` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x06\xF2\x90a.6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\x1E\x90a.6V[\x80\x15a\x07kW\x80`\x1F\x10a\x07@Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07kV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07NW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x07\x80\x90a.6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xAC\x90a.6V[\x80\x15a\x07\xF9W\x80`\x1F\x10a\x07\xCEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xF9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xDCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta\x08\x0E\x90a.6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08:\x90a.6V[\x80\x15a\x08\x87W\x80`\x1F\x10a\x08\\Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x87V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08jW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\t\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[a\t\x0B\x82\x82a\x17\xF8V[PPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\tWW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.pV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\t\xA0`\0\x80Q` a2;\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.\xBCV[a\t\xCF\x81a\x18[V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Ra\t\xEB\x91\x83\x91\x90a\x18\x82V[PV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\n6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.pV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\n\x7F`\0\x80Q` a2;\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\n\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.\xBCV[a\n\xAE\x82a\x18[V[a\t\x0B\x82\x82`\x01a\x18\x82V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0BZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xBAV[P`\0\x80Q` a2;\x839\x81Q\x91R\x90V[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x89\x80` \x01\x90Q\x81\x01\x90a\x0B\x8F\x91\x90a-2V[\x97P\x97P\x97P\x97P\x97P\x97P\x97P\x97Pa\x0B\xC7\x88\x88\x88`@Q\x80``\x01`@R\x80\x8A\x81R` \x01\x89\x81R` \x01\x88\x81RP\x86\x86a\x16YV[\x9A\x99PPPPPPPPPPV[`\0\x82\x81R`\x97` R`@\x81 a\x0B\xED\x90\x83a\x19\xEDV[\x93\x92PPPV[`\0\x91\x82R`e` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0a\x0CI\x89\x89\x89`@Q\x80``\x01`@R\x80\x8B\x81R` \x01\x8A\x81R` \x01\x89\x81RP\x87\x87a\x16YV[\x90P\x80a\x0C\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx \xAB\x1D)\xAB\x16\xB4\xB7;0\xB64\xB2\x100\xBA:2\xB9\xBA0\xBA4\xB7\xB7`9\x1B`D\x82\x01R`d\x01a\x05\xBAV[PPPPPPPPPV[`\0a\x0C\xC9\x89\x89\x89`@Q\x80``\x01`@R\x80\x8B\x81R` \x01\x8A\x81R` \x01\x89\x81RP\x87\x87a\x16YV[\x99\x98PPPPPPPPPV[`\0\x81\x81R`\x97` R`@\x81 a\x05\x13\x90a\x19\xF9V[a\x0C\xF8`\x003a\x0B\xF4V[a\r\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.\x12V[a\r7`@Q\x80``\x01`@R\x80\x85\x81R` \x01\x84\x81R` \x01\x83\x81RPa\x1A\x03V[PPPPV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\rX\x81a\x17\xE4V[a\x06\xD1\x83\x83a\x17\xF8V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\r\x82WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\r\x9CWP0;\x15\x80\x15a\r\x9CWP`\0T`\xFF\x16`\x01\x14[a\r\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x0E\"W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[\x83Q`\0\x03a\x0E\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FAV:I-At least one image must be `D\x82\x01Rg\x1C\x1C\x9B\xDD\x9AY\x19Y`\xC2\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[\x82Q\x84Q\x14a\x0E\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FAV:I-Image and key length mismat`D\x82\x01Ra\x0Cm`\xF3\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[a\x0E\xE8a\x1BvV[a\x0E\xF0a\x1BvV[a\x0E\xF8a\x1BvV[a\x0F\0a\x1BvV[a\x0F\x08a\x1BvV[a\x0F\x10a\x1BvV[a\x0F\x1B`\0\x83a\x1B\xE3V[`\0[\x83Q\x81\x10\x15a\x0F\xC5W`\0\x84\x82\x81Q\x81\x10a\x0F;Wa\x0F;a/\x08V[` \x02` \x01\x01Q\x90P`\0a\x0Fi\x87\x84\x81Q\x81\x10a\x0F\\Wa\x0F\\a/\x08V[` \x02` \x01\x01Qa\x1A\x03V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x03\"` R`@\x80\x82 \x84\x90UQ\x92\x93P\x83\x92\x7FY+\xBC\xEF\xA9\xF4dD\xAFY\xF1{\x05\xA4\xB6 \xFA\xFD\x14\x05\x03P\x06\xAET\x8E/ (+;\xC7\x91\x90\xA3PP\x80\x80a\x0F\xBD\x90a/4V[\x91PPa\x0F\x1EV[P\x80\x15a\r7W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPV[a\x10\x1C`\x003a\x0B\xF4V[a\x108W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.\x12V[`\0\x81\x81Ra\x03!` R`@\x90 \x80Ta\x10R\x90a.6V[\x90P`\0\x03a\x10\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FAV:WE-Image not whitelisted\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x10\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FAV:WE-Invalid enclave key\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03\"` R`@\x90 T\x15a\x11kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FAV:WE-Enclave key already verifi`D\x82\x01Ra\x19Y`\xF2\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x03\"` R`@\x80\x82 \x84\x90UQ\x83\x92\x91\x7FY+\xBC\xEF\xA9\xF4dD\xAFY\xF1{\x05\xA4\xB6 \xFA\xFD\x14\x05\x03P\x06\xAET\x8E/ (+;\xC7\x91\xA3PPV[a\x11\xBE`\x003a\x0B\xF4V[a\x11\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.\x12V[`\0\x81\x81Ra\x03!` R`@\x90 \x80Ta\x11\xF4\x90a.6V[\x90P`\0\x03a\x12EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FAV:RWI-Image not whitelisted\0\0\0\0`D\x82\x01R`d\x01a\x05\xBAV[`\0\x81\x81Ra\x03!` R`@\x81 \x90a\x12_\x82\x82a%\xAFV[a\x12m`\x01\x83\x01`\0a%\xAFV[a\x12{`\x02\x83\x01`\0a%\xAFV[PP`@Q\x81\x90\x7F\xCB\x96\xC5\xD9H\xD3(ED\xD8&t\x19I\xDE\x7F\x8C\xA8s\xD4\r(t\\b\xEB\xE6\x95\xE2I\xBEo\x90`\0\x90\xA2PV[`\0\x83\x81Ra\x03!` R`@\x90 \x80Ta\x12\xC5\x90a.6V[\x90P`\0\x03a\x13+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAV:V-Enclave image to verify not`D\x82\x01Rk\x08\x1D\xDA\x1A]\x19[\x1A\\\xDD\x19Y`\xA2\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x03\"` R`@\x90 T\x15a\x13\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FAV:V-Enclave key already verifie`D\x82\x01R`\x19`\xFA\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\0\x83\x81Ra\x03!` R`@\x80\x82 \x81Q``\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x13\xC4\x90a.6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\xF0\x90a.6V[\x80\x15a\x14=W\x80`\x1F\x10a\x14\x12Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14=V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14 W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x14V\x90a.6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\x82\x90a.6V[\x80\x15a\x14\xCFW\x80`\x1F\x10a\x14\xA4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14\xCFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\xB2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x14\xE8\x90a.6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x14\x90a.6V[\x80\x15a\x15aW\x80`\x1F\x10a\x156Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15aV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0a\x15|\x88\x88\x88\x85\x88\x88a\x16YV[\x90P\x80a\x15\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FAV:VE-Attestation must be signed`D\x82\x01Rq by source enclave`p\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x81\x81Ra\x03\"` R`@\x80\x82 \x88\x90UQ\x87\x92\x91\x7F\x84\xD3(\xB8\xB8\xEA({#\xCF\x84\x99\x1E,]\x82ot\x04%\xFA_\xA6\xB3\xFF\x1C\xF3\xBC\xF4\x03\\v\x91\xA3PPPPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x05\x13WPa\x05\x13\x82a\x1B\xEDV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81Ra\x03\"` R`@\x81 T\x80a\x16\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FAV:V-Enclave key must be verifie`D\x82\x01R`\x19`\xFA\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\0\x81\x81Ra\x03!` R`@\x90 \x80Ta\x16\xE4\x90a.6V[\x90P`\0\x03a\x17CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FAV:V-Source image must be whitel`D\x82\x01Rd\x1A\\\xDD\x19Y`\xDA\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\0`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FEnclave Attestation Verified\0\0\0\0\x81RP\x87\x87`\0\x01Q\x88` \x01Q\x89`@\x01Q\x89\x89`@Q` \x01a\x17\xA3\x97\x96\x95\x94\x93\x92\x91\x90a/MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x17\xC7\x82\x8Ba\x1C\"V[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x91\x16\x14\x93PPPP\x96\x95PPPPPPV[a\t\xEB\x813a\x1CFV[a\t\x0B\x82\x82a\x1C\x9FV[a\x18\x02\x82\x82a\x1C\xC1V[a\x18\x0C`\0a\x0C\xD6V[`\0\x03a\t\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FAV:RR-All admins cant be removed`D\x82\x01R`d\x01a\x05\xBAV[a\x18f`\x003a\x0B\xF4V[a\t\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x90a.\x12V[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a\x18\xB5Wa\x06\xD1\x83a\x1C\xE3V[\x82`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x19\x0FWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x19\x0C\x91\x81\x01\x90a/\xC0V[`\x01[a\x19rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01Rmon is not UUPS`\x90\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\0\x80Q` a2;\x839\x81Q\x91R\x81\x14a\x19\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01Rh\x1AXX\x9B\x19UURQ`\xBA\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[Pa\x06\xD1\x83\x83\x83a\x1D\x7FV[`\0a\x0B\xED\x83\x83a\x1D\xA4V[`\0a\x05\x13\x82T\x90V[\x80QQ`\0\x90`0\x14\x80\x15a\x1A\x1DWP\x81` \x01QQ`0\x14[\x80\x15a\x1A.WP\x81`@\x01QQ`0\x14[a\x1A\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FAV:IWI-PCR values must be 48 byt`D\x82\x01Raes`\xF0\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\0\x82`\0\x01Q\x83` \x01Q\x84`@\x01Q`@Q` \x01a\x1A\xA8\x93\x92\x91\x90a/\xD9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 ``\x84\x01\x83R\x86Q\x84R\x86\x82\x01Q\x84\x83\x01R\x86\x83\x01Q\x84\x84\x01R`\0\x81\x81Ra\x03!\x90\x92R\x91\x90 \x82Q\x91\x93P\x90\x81\x90a\x1A\xFA\x90\x82a0jV[P` \x82\x01Q`\x01\x82\x01\x90a\x1B\x0F\x90\x82a0jV[P`@\x82\x01Q`\x02\x82\x01\x90a\x1B$\x90\x82a0jV[P\x90PP\x80\x7FR\xB2\x9B\xBD\xD9z\xB9\x83A\x9FP\xFA\x15\x90\xE5\xABu\xE9\x94\"y\xE9^\x10\xA0\x86\x07\xB0l##\x8B\x84`\0\x01Q\x85` \x01Q\x86`@\x01Q`@Qa\x1Bh\x93\x92\x91\x90a(!V[`@Q\x80\x91\x03\x90\xA2\x92\x91PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1B\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[V[a\t\x0B\x82\x82a\x17\xEEV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x05\x13WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x05\x13V[`\0\x80`\0a\x1C1\x85\x85a\x1D\xCEV[\x91P\x91Pa\x1C>\x81a\x1E\x13V[P\x93\x92PPPV[a\x1CP\x82\x82a\x0B\xF4V[a\t\x0BWa\x1C]\x81a\x1F]V[a\x1Ch\x83` a\x1FoV[`@Q` \x01a\x1Cy\x92\x91\x90a1)V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x05\xBA\x91`\x04\x01a,\xCFV[a\x1C\xA9\x82\x82a!\nV[`\0\x82\x81R`\x97` R`@\x90 a\x06\xD1\x90\x82a!\x90V[a\x1C\xCB\x82\x82a!\xA5V[`\0\x82\x81R`\x97` R`@\x90 a\x06\xD1\x90\x82a\"\x0CV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x1DPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[`\0\x80Q` a2;\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x1D\x88\x83a\"!V[`\0\x82Q\x11\x80a\x1D\x95WP\x80[\x15a\x06\xD1Wa\r7\x83\x83a\"aV[`\0\x82`\0\x01\x82\x81T\x81\x10a\x1D\xBBWa\x1D\xBBa/\x08V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0\x80\x82Q`A\x03a\x1E\x04W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x1D\xF8\x87\x82\x85\x85a\"\x86V[\x94P\x94PPPPa\x1E\x0CV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x1E'Wa\x1E'a1\x9EV[\x03a\x1E/WPV[`\x01\x81`\x04\x81\x11\x15a\x1ECWa\x1ECa1\x9EV[\x03a\x1E\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xBAV[`\x02\x81`\x04\x81\x11\x15a\x1E\xA4Wa\x1E\xA4a1\x9EV[\x03a\x1E\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x05\xBAV[`\x03\x81`\x04\x81\x11\x15a\x1F\x05Wa\x1F\x05a1\x9EV[\x03a\t\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x05\xBAV[``a\x05\x13`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x1F~\x83`\x02a1\xB4V[a\x1F\x89\x90`\x02a1\xCBV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xA0Wa\x1F\xA0a&'V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1F\xCAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x1F\xE5Wa\x1F\xE5a/\x08V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a \x14Wa \x14a/\x08V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a 8\x84`\x02a1\xB4V[a C\x90`\x01a1\xCBV[\x90P[`\x01\x81\x11\x15a \xBBWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a wWa wa/\x08V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a \x8DWa \x8Da/\x08V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a \xB4\x81a1\xDEV[\x90Pa FV[P\x83\x15a\x0B\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x05\xBAV[a!\x14\x82\x82a\x0B\xF4V[a\t\x0BW`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua!L3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x0B\xED\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a#JV[a!\xAF\x82\x82a\x0B\xF4V[\x15a\t\x0BW`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x0B\xED\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a#\x99V[a\"*\x81a\x1C\xE3V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a\x0B\xED\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a2[`'\x919a$\x8CV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\"\xBDWP`\0\x90P`\x03a#AV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a#\x11W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a#:W`\0`\x01\x92P\x92PPa#AV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta#\x91WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x05\x13V[P`\0a\x05\x13V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a$\x82W`\0a#\xBD`\x01\x83a1\xF5V[\x85T\x90\x91P`\0\x90a#\xD1\x90`\x01\x90a1\xF5V[\x90P\x81\x81\x14a$6W`\0\x86`\0\x01\x82\x81T\x81\x10a#\xF1Wa#\xF1a/\x08V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a$\x14Wa$\x14a/\x08V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a$GWa$Ga2\x08V[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x05\x13V[`\0\x91PPa\x05\x13V[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa$\xA9\x91\x90a2\x1EV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a$\xE4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a$\xE9V[``\x91P[P\x91P\x91Pa$\xFA\x86\x83\x83\x87a%\x04V[\x96\x95PPPPPPV[``\x83\x15a%sW\x82Q`\0\x03a%lW`\x01`\x01`\xA0\x1B\x03\x85\x16;a%lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05\xBAV[P\x81a%}V[a%}\x83\x83a%\x85V[\x94\x93PPPPV[\x81Q\x15a%\x95W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xBA\x91\x90a,\xCFV[P\x80Ta%\xBB\x90a.6V[`\0\x82U\x80`\x1F\x10a%\xCBWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\t\xEB\x91\x90[\x80\x82\x11\x15a%\xF9W`\0\x81U`\x01\x01a%\xE5V[P\x90V[`\0` \x82\x84\x03\x12\x15a&\x0FW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0B\xEDW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a&_Wa&_a&'V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a&\x8DWa&\x8Da&'V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a&\xAEWa&\xAEa&'V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a&\xCDW`\0\x80\xFD[\x815a&\xE0a&\xDB\x82a&\x95V[a&eV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a&\xF5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a'$W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a':W`\0\x80\xFD[a%}\x84\x82\x85\x01a&\xBCV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xEBW`\0\x80\xFD[\x805a'f\x81a'FV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a'}W`\0\x80\xFD[\x815a\x0B\xED\x81a'FV[`\0` \x82\x84\x03\x12\x15a'\x9AW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a'\xB4W`\0\x80\xFD[\x825\x91P` \x83\x015a'\xC6\x81a'FV[\x80\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a'\xECW\x81\x81\x01Q\x83\x82\x01R` \x01a'\xD4V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra(\r\x81` \x86\x01` \x86\x01a'\xD1V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x81R`\0a(4``\x83\x01\x86a'\xF5V[\x82\x81\x03` \x84\x01Ra(F\x81\x86a'\xF5V[\x90P\x82\x81\x03`@\x84\x01Ra$\xFA\x81\x85a'\xF5V[`\0\x80`@\x83\x85\x03\x12\x15a(mW`\0\x80\xFD[\x825a(x\x81a'FV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\x93W`\0\x80\xFD[a(\x9F\x85\x82\x86\x01a&\xBCV[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a(\xBCW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a(\xE8W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a(\xFFW`\0\x80\xFD[a)\x0B\x8C\x83\x8D\x01a&\xBCV[\x99Pa)\x19` \x8C\x01a'[V[\x98Pa)'`@\x8C\x01a'[V[\x97P``\x8B\x015\x91P\x80\x82\x11\x15a)=W`\0\x80\xFD[a)I\x8C\x83\x8D\x01a&\xBCV[\x96P`\x80\x8B\x015\x91P\x80\x82\x11\x15a)_W`\0\x80\xFD[a)k\x8C\x83\x8D\x01a&\xBCV[\x95P`\xA0\x8B\x015\x91P\x80\x82\x11\x15a)\x81W`\0\x80\xFD[Pa)\x8E\x8B\x82\x8C\x01a&\xBCV[\x93PP`\xC0\x89\x015\x91P`\xE0\x89\x015\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x80`\0``\x84\x86\x03\x12\x15a)\xC1W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a)\xD8W`\0\x80\xFD[a)\xE4\x87\x83\x88\x01a&\xBCV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a)\xFAW`\0\x80\xFD[a*\x06\x87\x83\x88\x01a&\xBCV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a*\x1CW`\0\x80\xFD[Pa*)\x86\x82\x87\x01a&\xBCV[\x91PP\x92P\x92P\x92V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a*LWa*La&'V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a*gW`\0\x80\xFD[\x815` a*wa&\xDB\x83a*3V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a*\x96W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a*\xBAW\x805a*\xAD\x81a'FV[\x83R\x91\x83\x01\x91\x83\x01a*\x9AV[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a*\xDAW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a*\xF1W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a+\x05W`\0\x80\xFD[\x815` a+\x15a&\xDB\x83a*3V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a+4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a+\xEAW\x805\x86\x81\x11\x15a+OW`\0\x80\xFD[\x87\x01``\x81\x8E\x03`\x1F\x19\x01\x12\x15a+eW`\0\x80\xFD[a+ma&=V[\x85\x82\x015\x88\x81\x11\x15a+~W`\0\x80\xFD[a+\x8C\x8F\x88\x83\x86\x01\x01a&\xBCV[\x82RP`@\x82\x015\x88\x81\x11\x15a+\xA2W`\0\x80\x81\xFD[a+\xB0\x8F\x88\x83\x86\x01\x01a&\xBCV[\x87\x83\x01RP``\x82\x015\x88\x81\x11\x15a+\xC8W`\0\x80\x81\xFD[a+\xD6\x8F\x88\x83\x86\x01\x01a&\xBCV[`@\x83\x01RP\x84RP\x91\x83\x01\x91\x83\x01a+8V[P\x97PP\x87\x015\x92PP\x80\x82\x11\x15a,\x01W`\0\x80\xFD[Pa,\x0E\x86\x82\x87\x01a*VV[\x92PPa,\x1D`@\x85\x01a'[V[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a,9W`\0\x80\xFD[\x825a,D\x81a'FV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a,kW`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15a,\x81W`\0\x80\xFD[a,\x8D\x89\x82\x8A\x01a&\xBCV[\x96PP` \x87\x015a,\x9E\x81a'FV[\x94P`@\x87\x015a,\xAE\x81a'FV[\x95\x98\x94\x97P\x94\x95``\x81\x015\x95P`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[` \x81R`\0a\x0B\xED` \x83\x01\x84a'\xF5V[`\0\x82`\x1F\x83\x01\x12a,\xF3W`\0\x80\xFD[\x81Qa-\x01a&\xDB\x82a&\x95V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a-\x16W`\0\x80\xFD[a%}\x82` \x83\x01` \x87\x01a'\xD1V[\x80Qa'f\x81a'FV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a-OW`\0\x80\xFD[\x88Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a-fW`\0\x80\xFD[a-r\x8C\x83\x8D\x01a,\xE2V[\x99Pa-\x80` \x8C\x01a-'V[\x98Pa-\x8E`@\x8C\x01a-'V[\x97P``\x8B\x01Q\x91P\x80\x82\x11\x15a-\xA4W`\0\x80\xFD[a-\xB0\x8C\x83\x8D\x01a,\xE2V[\x96P`\x80\x8B\x01Q\x91P\x80\x82\x11\x15a-\xC6W`\0\x80\xFD[a-\xD2\x8C\x83\x8D\x01a,\xE2V[\x95P`\xA0\x8B\x01Q\x91P\x80\x82\x11\x15a-\xE8W`\0\x80\xFD[Pa-\xF5\x8B\x82\x8C\x01a,\xE2V[`\xC0\x8B\x01Q`\xE0\x90\x9B\x01Q\x99\x9C\x98\x9BP\x96\x99\x95\x98\x94\x97\x93PPPPV[` \x80\x82R`\n\x90\x82\x01Ri7\xB76<\x900\xB26\xB4\xB7`\xB1\x1B`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a.JW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a.jWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rk\x19\x19[\x19Y\xD8]\x19X\xD8[\x1B`\xA2\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rkactive proxy`\xA0\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a/FWa/Fa/\x1EV[P`\x01\x01\x90V[`\xE0\x81R`\0a/``\xE0\x83\x01\x8Aa'\xF5V[`\x01`\x01`\xA0\x1B\x03\x89\x16` \x84\x01R\x82\x81\x03`@\x84\x01Ra/\x81\x81\x89a'\xF5V[\x90P\x82\x81\x03``\x84\x01Ra/\x95\x81\x88a'\xF5V[\x90P\x82\x81\x03`\x80\x84\x01Ra/\xA9\x81\x87a'\xF5V[`\xA0\x84\x01\x95\x90\x95RPP`\xC0\x01R\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a/\xD2W`\0\x80\xFD[PQ\x91\x90PV[`\0\x84Qa/\xEB\x81\x84` \x89\x01a'\xD1V[\x84Q\x90\x83\x01\x90a/\xFF\x81\x83` \x89\x01a'\xD1V[\x84Q\x91\x01\x90a0\x12\x81\x83` \x88\x01a'\xD1V[\x01\x95\x94PPPPPV[`\x1F\x82\x11\x15a\x06\xD1W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a0CWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a0bW\x82\x81U`\x01\x01a0OV[PPPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x83Wa0\x83a&'V[a0\x97\x81a0\x91\x84Ta.6V[\x84a0\x1CV[` \x80`\x1F\x83\x11`\x01\x81\x14a0\xCCW`\0\x84\x15a0\xB4WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua0bV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a0\xFBW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a0\xDCV[P\x85\x82\x10\x15a1\x19W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa1a\x81`\x17\x85\x01` \x88\x01a'\xD1V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa1\x92\x81`(\x84\x01` \x88\x01a'\xD1V[\x01`(\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x13Wa\x05\x13a/\x1EV[\x80\x82\x01\x80\x82\x11\x15a\x05\x13Wa\x05\x13a/\x1EV[`\0\x81a1\xEDWa1\xEDa/\x1EV[P`\0\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x05\x13Wa\x05\x13a/\x1EV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0\x82Qa20\x81\x84` \x87\x01a'\xD1V[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCAddress: low-level delegate call failed\xA2dipfsX\"\x12 Q\xCB\xF6\xCD\xA3\xF6\xCB\xD9\x1E\xDA\x98\xB7)Y}\x83\x12\n><\xA3i\\!p\xD7\xE58\x13\t\xD6\xE5dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static ATTESTATIONVERIFIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AttestationVerifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AttestationVerifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AttestationVerifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AttestationVerifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AttestationVerifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AttestationVerifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AttestationVerifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ATTESTATIONVERIFIER_ABI.clone(),
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
                ATTESTATIONVERIFIER_ABI.clone(),
                ATTESTATIONVERIFIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ATTESTATION_PREFIX` (0xf5e64f92) function
        pub fn attestation_prefix(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([245, 230, 79, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
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
        ///Calls the contract's `getRoleMember` (0x9010d07c) function
        pub fn get_role_member(
            &self,
            role: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([144, 16, 208, 124], (role, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMemberCount` (0xca15c873) function
        pub fn get_role_member_count(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 21, 200, 115], role)
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
        ///Calls the contract's `initialize` (0xda350e72) function
        pub fn initialize(
            &self,
            images: ::std::vec::Vec<EnclaveImage>,
            enclave_keys: ::std::vec::Vec<::ethers::core::types::Address>,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([218, 53, 14, 114], (images, enclave_keys, admin))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isVerified` (0xb9209e33) function
        pub fn is_verified(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([185, 32, 158, 51], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
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
        ///Calls the contract's `revokeWhitelistedEnclave` (0x12554a8d) function
        pub fn revoke_whitelisted_enclave(
            &self,
            enclave_key: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 85, 74, 141], enclave_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeWhitelistedImage` (0xef98be3f) function
        pub fn revoke_whitelisted_image(
            &self,
            image_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 152, 190, 63], image_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeVerify` (0x06d5a2ed) function
        pub fn safe_verify(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([6, 213, 162, 237], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeVerify` (0x948b4459) function
        pub fn safe_verify_with_attestation(
            &self,
            attestation: ::ethers::core::types::Bytes,
            source_enclave_key: ::ethers::core::types::Address,
            enclave_key: ::ethers::core::types::Address,
            pcr0: ::ethers::core::types::Bytes,
            pcr1: ::ethers::core::types::Bytes,
            pcr2: ::ethers::core::types::Bytes,
            enclave_cp_us: ::ethers::core::types::U256,
            enclave_memory: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [148, 139, 68, 89],
                    (
                        attestation,
                        source_enclave_key,
                        enclave_key,
                        pcr0,
                        pcr1,
                        pcr2,
                        enclave_cp_us,
                        enclave_memory,
                    ),
                )
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
        ///Calls the contract's `upgradeTo` (0x3659cfe6) function
        pub fn upgrade_to(
            &self,
            new_implementation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 89, 207, 230], new_implementation)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeToAndCall` (0x4f1ef286) function
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify` (0x8e760afe) function
        pub fn verify(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([142, 118, 10, 254], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify` (0xb247499b) function
        pub fn verify_with_attestation(
            &self,
            attestation: ::ethers::core::types::Bytes,
            source_enclave_key: ::ethers::core::types::Address,
            enclave_key: ::ethers::core::types::Address,
            pcr0: ::ethers::core::types::Bytes,
            pcr1: ::ethers::core::types::Bytes,
            pcr2: ::ethers::core::types::Bytes,
            enclave_cp_us: ::ethers::core::types::U256,
            enclave_memory: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [178, 71, 73, 155],
                    (
                        attestation,
                        source_enclave_key,
                        enclave_key,
                        pcr0,
                        pcr1,
                        pcr2,
                        enclave_cp_us,
                        enclave_memory,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyEnclaveKey` (0xf48f5ab6) function
        pub fn verify_enclave_key(
            &self,
            attestation: ::ethers::core::types::Bytes,
            source_enclave_key: ::ethers::core::types::Address,
            enclave_key: ::ethers::core::types::Address,
            image_id: [u8; 32],
            enclave_cp_us: ::ethers::core::types::U256,
            enclave_memory: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [244, 143, 90, 182],
                    (
                        attestation,
                        source_enclave_key,
                        enclave_key,
                        image_id,
                        enclave_cp_us,
                        enclave_memory,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistEnclaveKey` (0xe3a12aa3) function
        pub fn whitelist_enclave_key(
            &self,
            enclave_key: ::ethers::core::types::Address,
            image_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 161, 42, 163], (enclave_key, image_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistImage` (0xcd455bd9) function
        pub fn whitelist_image(
            &self,
            pcr0: ::ethers::core::types::Bytes,
            pcr1: ::ethers::core::types::Bytes,
            pcr2: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 69, 91, 217], (pcr0, pcr1, pcr2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistedImages` (0x2f9b0ad7) function
        pub fn whitelisted_images(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Bytes,
                ::ethers::core::types::Bytes,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([47, 155, 10, 215], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AdminChanged` event
        pub fn admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BeaconUpgraded` event
        pub fn beacon_upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BeaconUpgradedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnclaveImageWhitelisted` event
        pub fn enclave_image_whitelisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnclaveImageWhitelistedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnclaveKeyVerified` event
        pub fn enclave_key_verified_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnclaveKeyVerifiedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnclaveKeyWhitelisted` event
        pub fn enclave_key_whitelisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnclaveKeyWhitelistedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
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
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WhitelistedEnclaveKeyRevoked` event
        pub fn whitelisted_enclave_key_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WhitelistedEnclaveKeyRevokedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WhitelistedImageRevoked` event
        pub fn whitelisted_image_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WhitelistedImageRevokedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AttestationVerifierEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AttestationVerifier<M> {
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
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ::ethers::core::types::Address,
        pub new_admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "BeaconUpgraded", abi = "BeaconUpgraded(address)")]
    pub struct BeaconUpgradedFilter {
        #[ethevent(indexed)]
        pub beacon: ::ethers::core::types::Address,
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
        name = "EnclaveImageWhitelisted",
        abi = "EnclaveImageWhitelisted(bytes32,bytes,bytes,bytes)"
    )]
    pub struct EnclaveImageWhitelistedFilter {
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "EnclaveKeyVerified", abi = "EnclaveKeyVerified(address,bytes32)")]
    pub struct EnclaveKeyVerifiedFilter {
        #[ethevent(indexed)]
        pub enclave_key: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
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
        name = "EnclaveKeyWhitelisted",
        abi = "EnclaveKeyWhitelisted(address,bytes32)"
    )]
    pub struct EnclaveKeyWhitelistedFilter {
        #[ethevent(indexed)]
        pub enclave_key: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
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
        name = "WhitelistedEnclaveKeyRevoked",
        abi = "WhitelistedEnclaveKeyRevoked(address,bytes32)"
    )]
    pub struct WhitelistedEnclaveKeyRevokedFilter {
        #[ethevent(indexed)]
        pub enclave_key: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
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
        name = "WhitelistedImageRevoked",
        abi = "WhitelistedImageRevoked(bytes32)"
    )]
    pub struct WhitelistedImageRevokedFilter {
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
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
    pub enum AttestationVerifierEvents {
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        EnclaveImageWhitelistedFilter(EnclaveImageWhitelistedFilter),
        EnclaveKeyVerifiedFilter(EnclaveKeyVerifiedFilter),
        EnclaveKeyWhitelistedFilter(EnclaveKeyWhitelistedFilter),
        InitializedFilter(InitializedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        UpgradedFilter(UpgradedFilter),
        WhitelistedEnclaveKeyRevokedFilter(WhitelistedEnclaveKeyRevokedFilter),
        WhitelistedImageRevokedFilter(WhitelistedImageRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AttestationVerifierEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveImageWhitelistedFilter::decode_log(log) {
                return Ok(
                    AttestationVerifierEvents::EnclaveImageWhitelistedFilter(decoded),
                );
            }
            if let Ok(decoded) = EnclaveKeyVerifiedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::EnclaveKeyVerifiedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveKeyWhitelistedFilter::decode_log(log) {
                return Ok(
                    AttestationVerifierEvents::EnclaveKeyWhitelistedFilter(decoded),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::UpgradedFilter(decoded));
            }
            if let Ok(decoded) = WhitelistedEnclaveKeyRevokedFilter::decode_log(log) {
                return Ok(
                    AttestationVerifierEvents::WhitelistedEnclaveKeyRevokedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = WhitelistedImageRevokedFilter::decode_log(log) {
                return Ok(
                    AttestationVerifierEvents::WhitelistedImageRevokedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AttestationVerifierEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeaconUpgradedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnclaveImageWhitelistedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnclaveKeyVerifiedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnclaveKeyWhitelistedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhitelistedEnclaveKeyRevokedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WhitelistedImageRevokedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AdminChangedFilter> for AttestationVerifierEvents {
        fn from(value: AdminChangedFilter) -> Self {
            Self::AdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<BeaconUpgradedFilter> for AttestationVerifierEvents {
        fn from(value: BeaconUpgradedFilter) -> Self {
            Self::BeaconUpgradedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageWhitelistedFilter>
    for AttestationVerifierEvents {
        fn from(value: EnclaveImageWhitelistedFilter) -> Self {
            Self::EnclaveImageWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyVerifiedFilter> for AttestationVerifierEvents {
        fn from(value: EnclaveKeyVerifiedFilter) -> Self {
            Self::EnclaveKeyVerifiedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyWhitelistedFilter>
    for AttestationVerifierEvents {
        fn from(value: EnclaveKeyWhitelistedFilter) -> Self {
            Self::EnclaveKeyWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for AttestationVerifierEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for AttestationVerifierEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for AttestationVerifierEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for AttestationVerifierEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for AttestationVerifierEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    impl ::core::convert::From<WhitelistedEnclaveKeyRevokedFilter>
    for AttestationVerifierEvents {
        fn from(value: WhitelistedEnclaveKeyRevokedFilter) -> Self {
            Self::WhitelistedEnclaveKeyRevokedFilter(value)
        }
    }
    impl ::core::convert::From<WhitelistedImageRevokedFilter>
    for AttestationVerifierEvents {
        fn from(value: WhitelistedImageRevokedFilter) -> Self {
            Self::WhitelistedImageRevokedFilter(value)
        }
    }
    ///Container type for all input parameters for the `ATTESTATION_PREFIX` function with signature `ATTESTATION_PREFIX()` and selector `0xf5e64f92`
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
    #[ethcall(name = "ATTESTATION_PREFIX", abi = "ATTESTATION_PREFIX()")]
    pub struct AttestationPrefixCall;
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
    ///Container type for all input parameters for the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`
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
    #[ethcall(name = "getRoleMember", abi = "getRoleMember(bytes32,uint256)")]
    pub struct GetRoleMemberCall {
        pub role: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
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
    #[ethcall(name = "getRoleMemberCount", abi = "getRoleMemberCount(bytes32)")]
    pub struct GetRoleMemberCountCall {
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize((bytes,bytes,bytes)[],address[],address)` and selector `0xda350e72`
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
        name = "initialize",
        abi = "initialize((bytes,bytes,bytes)[],address[],address)"
    )]
    pub struct InitializeCall {
        pub images: ::std::vec::Vec<EnclaveImage>,
        pub enclave_keys: ::std::vec::Vec<::ethers::core::types::Address>,
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isVerified` function with signature `isVerified(address)` and selector `0xb9209e33`
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
    #[ethcall(name = "isVerified", abi = "isVerified(address)")]
    pub struct IsVerifiedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
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
    ///Container type for all input parameters for the `revokeWhitelistedEnclave` function with signature `revokeWhitelistedEnclave(address)` and selector `0x12554a8d`
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
        name = "revokeWhitelistedEnclave",
        abi = "revokeWhitelistedEnclave(address)"
    )]
    pub struct RevokeWhitelistedEnclaveCall {
        pub enclave_key: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeWhitelistedImage` function with signature `revokeWhitelistedImage(bytes32)` and selector `0xef98be3f`
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
    #[ethcall(name = "revokeWhitelistedImage", abi = "revokeWhitelistedImage(bytes32)")]
    pub struct RevokeWhitelistedImageCall {
        pub image_id: [u8; 32],
    }
    ///Container type for all input parameters for the `safeVerify` function with signature `safeVerify(bytes)` and selector `0x06d5a2ed`
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
    #[ethcall(name = "safeVerify", abi = "safeVerify(bytes)")]
    pub struct SafeVerifyCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `safeVerify` function with signature `safeVerify(bytes,address,address,bytes,bytes,bytes,uint256,uint256)` and selector `0x948b4459`
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
        name = "safeVerify",
        abi = "safeVerify(bytes,address,address,bytes,bytes,bytes,uint256,uint256)"
    )]
    pub struct SafeVerifyWithAttestationCall {
        pub attestation: ::ethers::core::types::Bytes,
        pub source_enclave_key: ::ethers::core::types::Address,
        pub enclave_key: ::ethers::core::types::Address,
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
        pub enclave_cp_us: ::ethers::core::types::U256,
        pub enclave_memory: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `upgradeTo` function with signature `upgradeTo(address)` and selector `0x3659cfe6`
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
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address)")]
    pub struct UpgradeToCall {
        pub new_implementation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`
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
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
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
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `verify` function with signature `verify(bytes,address,address,bytes,bytes,bytes,uint256,uint256)` and selector `0xb247499b`
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
        name = "verify",
        abi = "verify(bytes,address,address,bytes,bytes,bytes,uint256,uint256)"
    )]
    pub struct VerifyWithAttestationCall {
        pub attestation: ::ethers::core::types::Bytes,
        pub source_enclave_key: ::ethers::core::types::Address,
        pub enclave_key: ::ethers::core::types::Address,
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
        pub enclave_cp_us: ::ethers::core::types::U256,
        pub enclave_memory: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verifyEnclaveKey` function with signature `verifyEnclaveKey(bytes,address,address,bytes32,uint256,uint256)` and selector `0xf48f5ab6`
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
        name = "verifyEnclaveKey",
        abi = "verifyEnclaveKey(bytes,address,address,bytes32,uint256,uint256)"
    )]
    pub struct VerifyEnclaveKeyCall {
        pub attestation: ::ethers::core::types::Bytes,
        pub source_enclave_key: ::ethers::core::types::Address,
        pub enclave_key: ::ethers::core::types::Address,
        pub image_id: [u8; 32],
        pub enclave_cp_us: ::ethers::core::types::U256,
        pub enclave_memory: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `whitelistEnclaveKey` function with signature `whitelistEnclaveKey(address,bytes32)` and selector `0xe3a12aa3`
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
        name = "whitelistEnclaveKey",
        abi = "whitelistEnclaveKey(address,bytes32)"
    )]
    pub struct WhitelistEnclaveKeyCall {
        pub enclave_key: ::ethers::core::types::Address,
        pub image_id: [u8; 32],
    }
    ///Container type for all input parameters for the `whitelistImage` function with signature `whitelistImage(bytes,bytes,bytes)` and selector `0xcd455bd9`
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
    #[ethcall(name = "whitelistImage", abi = "whitelistImage(bytes,bytes,bytes)")]
    pub struct WhitelistImageCall {
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `whitelistedImages` function with signature `whitelistedImages(bytes32)` and selector `0x2f9b0ad7`
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
    #[ethcall(name = "whitelistedImages", abi = "whitelistedImages(bytes32)")]
    pub struct WhitelistedImagesCall(pub [u8; 32]);
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
    pub enum AttestationVerifierCalls {
        AttestationPrefix(AttestationPrefixCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        Initialize(InitializeCall),
        IsVerified(IsVerifiedCall),
        ProxiableUUID(ProxiableUUIDCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        RevokeWhitelistedEnclave(RevokeWhitelistedEnclaveCall),
        RevokeWhitelistedImage(RevokeWhitelistedImageCall),
        SafeVerify(SafeVerifyCall),
        SafeVerifyWithAttestation(SafeVerifyWithAttestationCall),
        SupportsInterface(SupportsInterfaceCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        Verify(VerifyCall),
        VerifyWithAttestation(VerifyWithAttestationCall),
        VerifyEnclaveKey(VerifyEnclaveKeyCall),
        WhitelistEnclaveKey(WhitelistEnclaveKeyCall),
        WhitelistImage(WhitelistImageCall),
        WhitelistedImages(WhitelistedImagesCall),
    }
    impl ::ethers::core::abi::AbiDecode for AttestationVerifierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AttestationPrefixCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AttestationPrefix(decoded));
            }
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) = <GetRoleMemberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleMember(decoded));
            }
            if let Ok(decoded) = <GetRoleMemberCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleMemberCount(decoded));
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
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IsVerifiedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsVerified(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxiableUUID(decoded));
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
            if let Ok(decoded) = <RevokeWhitelistedEnclaveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeWhitelistedEnclave(decoded));
            }
            if let Ok(decoded) = <RevokeWhitelistedImageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeWhitelistedImage(decoded));
            }
            if let Ok(decoded) = <SafeVerifyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeVerify(decoded));
            }
            if let Ok(decoded) = <SafeVerifyWithAttestationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeVerifyWithAttestation(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <UpgradeToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeTo(decoded));
            }
            if let Ok(decoded) = <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) = <VerifyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Verify(decoded));
            }
            if let Ok(decoded) = <VerifyWithAttestationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifyWithAttestation(decoded));
            }
            if let Ok(decoded) = <VerifyEnclaveKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifyEnclaveKey(decoded));
            }
            if let Ok(decoded) = <WhitelistEnclaveKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WhitelistEnclaveKey(decoded));
            }
            if let Ok(decoded) = <WhitelistImageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WhitelistImage(decoded));
            }
            if let Ok(decoded) = <WhitelistedImagesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WhitelistedImages(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AttestationVerifierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AttestationPrefix(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMember(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMemberCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsVerified(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeWhitelistedEnclave(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeWhitelistedImage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeVerify(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeVerifyWithAttestation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyWithAttestation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyEnclaveKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WhitelistEnclaveKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WhitelistImage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WhitelistedImages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AttestationVerifierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AttestationPrefix(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMember(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsVerified(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeWhitelistedEnclave(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokeWhitelistedImage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeVerify(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeVerifyWithAttestation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyWithAttestation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyEnclaveKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhitelistEnclaveKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WhitelistImage(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhitelistedImages(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AttestationPrefixCall> for AttestationVerifierCalls {
        fn from(value: AttestationPrefixCall) -> Self {
            Self::AttestationPrefix(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for AttestationVerifierCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for AttestationVerifierCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCall> for AttestationVerifierCalls {
        fn from(value: GetRoleMemberCall) -> Self {
            Self::GetRoleMember(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCountCall> for AttestationVerifierCalls {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for AttestationVerifierCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for AttestationVerifierCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for AttestationVerifierCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsVerifiedCall> for AttestationVerifierCalls {
        fn from(value: IsVerifiedCall) -> Self {
            Self::IsVerified(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for AttestationVerifierCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for AttestationVerifierCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for AttestationVerifierCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<RevokeWhitelistedEnclaveCall>
    for AttestationVerifierCalls {
        fn from(value: RevokeWhitelistedEnclaveCall) -> Self {
            Self::RevokeWhitelistedEnclave(value)
        }
    }
    impl ::core::convert::From<RevokeWhitelistedImageCall> for AttestationVerifierCalls {
        fn from(value: RevokeWhitelistedImageCall) -> Self {
            Self::RevokeWhitelistedImage(value)
        }
    }
    impl ::core::convert::From<SafeVerifyCall> for AttestationVerifierCalls {
        fn from(value: SafeVerifyCall) -> Self {
            Self::SafeVerify(value)
        }
    }
    impl ::core::convert::From<SafeVerifyWithAttestationCall>
    for AttestationVerifierCalls {
        fn from(value: SafeVerifyWithAttestationCall) -> Self {
            Self::SafeVerifyWithAttestation(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for AttestationVerifierCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall> for AttestationVerifierCalls {
        fn from(value: UpgradeToCall) -> Self {
            Self::UpgradeTo(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for AttestationVerifierCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for AttestationVerifierCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    impl ::core::convert::From<VerifyWithAttestationCall> for AttestationVerifierCalls {
        fn from(value: VerifyWithAttestationCall) -> Self {
            Self::VerifyWithAttestation(value)
        }
    }
    impl ::core::convert::From<VerifyEnclaveKeyCall> for AttestationVerifierCalls {
        fn from(value: VerifyEnclaveKeyCall) -> Self {
            Self::VerifyEnclaveKey(value)
        }
    }
    impl ::core::convert::From<WhitelistEnclaveKeyCall> for AttestationVerifierCalls {
        fn from(value: WhitelistEnclaveKeyCall) -> Self {
            Self::WhitelistEnclaveKey(value)
        }
    }
    impl ::core::convert::From<WhitelistImageCall> for AttestationVerifierCalls {
        fn from(value: WhitelistImageCall) -> Self {
            Self::WhitelistImage(value)
        }
    }
    impl ::core::convert::From<WhitelistedImagesCall> for AttestationVerifierCalls {
        fn from(value: WhitelistedImagesCall) -> Self {
            Self::WhitelistedImages(value)
        }
    }
    ///Container type for all return fields from the `ATTESTATION_PREFIX` function with signature `ATTESTATION_PREFIX()` and selector `0xf5e64f92`
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
    pub struct AttestationPrefixReturn(pub ::std::string::String);
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
    ///Container type for all return fields from the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`
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
    pub struct GetRoleMemberReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
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
    pub struct GetRoleMemberCountReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `isVerified` function with signature `isVerified(address)` and selector `0xb9209e33`
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
    pub struct IsVerifiedReturn(pub [u8; 32]);
    ///Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `verify` function with signature `verify(bytes,address,address,bytes,bytes,bytes,uint256,uint256)` and selector `0xb247499b`
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
    pub struct VerifyWithAttestationReturn(pub bool);
    ///Container type for all return fields from the `whitelistedImages` function with signature `whitelistedImages(bytes32)` and selector `0x2f9b0ad7`
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
    pub struct WhitelistedImagesReturn {
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
    }
    ///`EnclaveImage(bytes,bytes,bytes)`
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
    pub struct EnclaveImage {
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
    }
}
