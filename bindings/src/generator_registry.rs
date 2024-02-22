pub use generator_registry::*;
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
pub mod generator_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_stakingToken"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IERC20Upgradeable",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_entityRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract EntityKeyRegistry",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("ENTITY_KEY_REGISTRY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ENTITY_KEY_REGISTRY",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract EntityKeyRegistry",
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
                    ::std::borrow::ToOwned::to_owned("PARALLEL_REQUESTS_UPPER_LIMIT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PARALLEL_REQUESTS_UPPER_LIMIT",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("SLASHER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("SLASHER_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("STAKING_TOKEN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("STAKING_TOKEN"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IERC20Upgradeable",
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
                    ::std::borrow::ToOwned::to_owned("UNLOCK_WAIT_BLOCKS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("UNLOCK_WAIT_BLOCKS"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("assignGeneratorTask"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "assignGeneratorTask",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakeToLock"),
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
                    ::std::borrow::ToOwned::to_owned("changeRewardAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "changeRewardAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newRewardAddress"),
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
                    ::std::borrow::ToOwned::to_owned("completeGeneratorTask"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "completeGeneratorTask",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakeToRelease"),
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
                    ::std::borrow::ToOwned::to_owned("decreaseDeclaredCompute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "decreaseDeclaredCompute",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deregister"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deregister"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundAddress"),
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
                    ::std::borrow::ToOwned::to_owned("generatorInfoPerMarket"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "generatorInfoPerMarket",
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
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum GeneratorRegistry.GeneratorState",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "computePerRequestRequired",
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
                                        "proofGenerationCost",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposedTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("activeRequests"),
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
                    ::std::borrow::ToOwned::to_owned("generatorRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("generatorRegistry"),
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
                                    name: ::std::borrow::ToOwned::to_owned("rewardAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalStake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "sumOfComputeAllocations",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("computeConsumed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakeLocked"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "activeMarketPlaces",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("declaredCompute"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "intendedStakeUtilization",
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
                                        "intendedComputeUtilization",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorData"),
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
                    ::std::borrow::ToOwned::to_owned("getGeneratorAssignmentDetails"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getGeneratorAssignmentDetails",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getGeneratorRewardDetails"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getGeneratorRewardDetails",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("getGeneratorState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getGeneratorState"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum GeneratorRegistry.GeneratorState",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("increaseDeclaredCompute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "increaseDeclaredCompute",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("computeToIncrease"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_admin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proofMarketPlace"),
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
                    ::std::borrow::ToOwned::to_owned("intendToReduceCompute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "intendToReduceCompute",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newUtilization"),
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
                    ::std::borrow::ToOwned::to_owned("intendToReduceStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "intendToReduceStake",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newUtilization"),
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
                    ::std::borrow::ToOwned::to_owned("joinMarketPlace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("joinMarketPlace"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "computePerRequestRequired",
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
                                        "proofGenerationCost",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposedTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "updateMarketDedicatedKey",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                                    name: ::std::borrow::ToOwned::to_owned("enclaveSignature"),
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
                    ::std::borrow::ToOwned::to_owned("leaveMarketPlace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("leaveMarketPlace"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("leaveMarketPlaces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("leaveMarketPlaces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketIds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                    ::std::borrow::ToOwned::to_owned("register"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("register"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rewardAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("declaredCompute"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialStake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorData"),
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
                    ::std::borrow::ToOwned::to_owned("removeEncryptionKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeEncryptionKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("requestForExitMarketPlace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "requestForExitMarketPlace",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("requestForExitMarketPlaces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "requestForExitMarketPlaces",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketIds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                    ::std::borrow::ToOwned::to_owned("slashGenerator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("slashGenerator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("slashingAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rewardAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("unstake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unstake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                    ::std::borrow::ToOwned::to_owned("updateEncryptionKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateEncryptionKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                    name: ::std::borrow::ToOwned::to_owned("enclaveSignature"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddedStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AddedStake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("ChangedGeneratorRewardAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ChangedGeneratorRewardAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newRewardAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DecreaseCompute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DecreaseCompute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("compute"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DeregisteredGenerator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeregisteredGenerator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncreasedCompute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("IncreasedCompute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("compute"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("JoinedMarketPlace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("JoinedMarketPlace"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("computeAllocation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LeftMarketplace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LeftMarketplace"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("RegisteredGenerator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RegisteredGenerator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("initialCompute"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("initialStake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RemovedStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RemovedStake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RequestComputeDecrease"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RequestComputeDecrease",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "intendedUtilization",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RequestExitMarketPlace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RequestExitMarketPlace",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("RequestStakeDecrease"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RequestStakeDecrease",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "intendedUtilization",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GENERATORREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R0`\x80R4\x80\x15b\0\0\x15W`\0\x80\xFD[P`@Qb\0[\n8\x03\x80b\0[\n\x839\x81\x01`@\x81\x90Rb\0\08\x91b\0\0iV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\xA0R\x16`\xC0Rb\0\0\xA8V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0fW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0}W`\0\x80\xFD[\x82Qb\0\0\x8A\x81b\0\0PV[` \x84\x01Q\x90\x92Pb\0\0\x9D\x81b\0\0PV[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0QaY\xDEb\0\x01,`\09`\0\x81\x81a\x05H\x01R\x81\x81a\x17\x98\x01R\x81\x81a\"\x94\x01Ra.\xFA\x01R`\0\x81\x81a\x02\xB3\x01R\x81\x81a\x0C\x01\x01R\x81\x81a\x1D'\x01R\x81\x81a'T\x01R\x81\x81a2\xBD\x01Ra4\xB4\x01R`\0\x81\x81a\x12\x89\x01R\x81\x81a\x12\xC9\x01R\x81\x81a\x15\xDB\x01R\x81\x81a\x16\x1B\x01Ra\x16\xC6\x01RaY\xDE`\0\xF3\xFE`\x80`@R`\x046\x10a\x02gW`\x005`\xE0\x1C\x80cz\x14\xC4c\x11a\x01DW\x80c\xAD\xC9w.\x11a\0\xB6W\x80c\xD5a9\xB6\x11a\0zW\x80c\xD5a9\xB6\x14a\x07\xD6W\x80c\xE8R\x10\xBD\x14a\x07\xF6W\x80c\xE9\xE94\xA0\x14a\x08\x16W\x80c\xEA\xAC\xAE\x94\x14a\x086W\x80c\xEE\x93(c\x14a\x08VW\x80c\xF2\x88\x8D\xBB\x14a\x08vW`\0\x80\xFD[\x80c\xAD\xC9w.\x14a\x07AW\x80c\xB8\n\xAA\x89\x14a\x07aW\x80c\xC4\x92\xEE9\x14a\x07vW\x80c\xCA\x15\xC8s\x14a\x07\x96W\x80c\xD5Gt\x1F\x14a\x07\xB6W`\0\x80\xFD[\x80c\x92\xEB\x91\xE2\x11a\x01\x08W\x80c\x92\xEB\x91\xE2\x14a\x06AW\x80c\x96\xDE\x0E\xEF\x14a\x06aW\x80c\x98*A]\x14a\x06\x81W\x80c\x9A\x7F\xCA\x8E\x14a\x06\xA1W\x80c\xA2\x17\xFD\xDF\x14a\x07\x0CW\x80c\xA6KR\x13\x14a\x07!W`\0\x80\xFD[\x80cz\x14\xC4c\x14a\x046W\x80c\x84\xAC3\xEC\x14a\x05\xABW\x80c\x8C\xFCV\xD8\x14a\x05\xCBW\x80c\x90\x10\xD0|\x14a\x06\x01W\x80c\x91\xD1HT\x14a\x06!W`\0\x80\xFD[\x80cF\x15\xE8\xBC\x11a\x01\xDDW\x80cR\xD1\x90-\x11a\x01\xA1W\x80cR\xD1\x90-\x14a\x04\xD3W\x80cT\x1A\x8C\x18\x14a\x04\xE8W\x80cdmQ\xB4\x14a\x05\x08W\x80cf\x1D\xE5\xAC\x14a\x056W\x80cm@Xw\x14a\x05jW\x80cs*\x9Dc\x14a\x05\x8AW`\0\x80\xFD[\x80cF\x15\xE8\xBC\x14a\x04KW\x80cH\\\xC9U\x14a\x04kW\x80cM*\xAB\x9A\x14a\x04\x8BW\x80cO\x1E\xF2\x86\x14a\x04\xABW\x80cP\x95\xAFd\x14a\x04\xBEW`\0\x80\xFD[\x80c+a\x0C-\x11a\x02/W\x80c+a\x0C-\x14a\x03\x82W\x80c//\xF1]\x14a\x03\xC1W\x80c/\x8FJ;\x14a\x03\xE1W\x80c6V\x8A\xBE\x14a\x03\xF6W\x80c6Y\xCF\xE6\x14a\x04\x16W\x80c<^\xB5|\x14a\x046W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02lW\x80c\x04y\xD6D\x14a\x02\xA1W\x80c\x13m\xFB\xF5\x14a\x02\xEDW\x80c\x1C~\xAEe\x14a\x03\x0FW\x80c$\x8A\x9C\xA3\x14a\x03DW[`\0\x80\xFD[4\x80\x15a\x02xW`\0\x80\xFD[Pa\x02\x8Ca\x02\x876`\x04aM\xB3V[a\x08\x96V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xADW`\0\x80\xFD[Pa\x02\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x98V[4\x80\x15a\x02\xF9W`\0\x80\xFD[Pa\x03\ra\x03\x086`\x04aN\xB7V[a\x08\xA7V[\0[4\x80\x15a\x03\x1BW`\0\x80\xFD[Pa\x03/a\x03*6`\x04aO\x1AV[a\x0C\x80V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x98V[4\x80\x15a\x03PW`\0\x80\xFD[Pa\x03ta\x03_6`\x04aOFV[`\0\x90\x81R`e` R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x02\x98V[4\x80\x15a\x03\x8EW`\0\x80\xFD[Pa\x03\xA2a\x03\x9D6`\x04aO\x1AV[a\r\"V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x98V[4\x80\x15a\x03\xCDW`\0\x80\xFD[Pa\x03\ra\x03\xDC6`\x04aO_V[a\x0F\tV[4\x80\x15a\x03\xEDW`\0\x80\xFD[Pa\x03\ra\x0F3V[4\x80\x15a\x04\x02W`\0\x80\xFD[Pa\x03\ra\x04\x116`\x04aO_V[a\x12\x01V[4\x80\x15a\x04\"W`\0\x80\xFD[Pa\x03\ra\x0416`\x04aO\x8FV[a\x12\x7FV[4\x80\x15a\x04BW`\0\x80\xFD[Pa\x03t`d\x81V[4\x80\x15a\x04WW`\0\x80\xFD[Pa\x03\ra\x04f6`\x04aO\xACV[a\x13^V[4\x80\x15a\x04wW`\0\x80\xFD[Pa\x03\ra\x04\x866`\x04aP!V[a\x13\x9EV[4\x80\x15a\x04\x97W`\0\x80\xFD[Pa\x03\ra\x04\xA66`\x04aO\x8FV[a\x15!V[a\x03\ra\x04\xB96`\x04aPOV[a\x15\xD1V[4\x80\x15a\x04\xCAW`\0\x80\xFD[Pa\x03ta\x16\x9DV[4\x80\x15a\x04\xDFW`\0\x80\xFD[Pa\x03ta\x16\xB9V[4\x80\x15a\x04\xF4W`\0\x80\xFD[Pa\x03\ra\x05\x036`\x04aOFV[a\x17lV[4\x80\x15a\x05\x14W`\0\x80\xFD[Pa\x05(a\x05#6`\x04aO\x1AV[a\x17\xFAV[`@Qa\x02\x98\x92\x91\x90aP\xD7V[4\x80\x15a\x05BW`\0\x80\xFD[Pa\x02\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05vW`\0\x80\xFD[Pa\x03\ra\x05\x856`\x04aOFV[a\x1A\xBEV[4\x80\x15a\x05\x96W`\0\x80\xFD[Pa\x01cTa\x02\xD5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05\xB7W`\0\x80\xFD[Pa\x03\ra\x05\xC66`\x04aO\x8FV[a\x1B\xB2V[4\x80\x15a\x05\xD7W`\0\x80\xFD[Pa\x05\xEBa\x05\xE66`\x04aO\x8FV[a\x1D\xFFV[`@Qa\x02\x98\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aQBV[4\x80\x15a\x06\rW`\0\x80\xFD[Pa\x02\xD5a\x06\x1C6`\x04aQ\xA7V[a\x1E\xE7V[4\x80\x15a\x06-W`\0\x80\xFD[Pa\x02\x8Ca\x06<6`\x04aO_V[a\x1F\x06V[4\x80\x15a\x06MW`\0\x80\xFD[Pa\x03\ra\x06\\6`\x04aR\x0BV[a\x1F1V[4\x80\x15a\x06mW`\0\x80\xFD[Pa\x03\ra\x06|6`\x04aOFV[a#$V[4\x80\x15a\x06\x8DW`\0\x80\xFD[Pa\x03\ra\x06\x9C6`\x04aR{V[a%\x1BV[4\x80\x15a\x06\xADW`\0\x80\xFD[Pa\x06\xFBa\x06\xBC6`\x04aO\x1AV[a\x01`` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T`\xFF\x90\x93\x16\x93\x91\x92\x90\x91\x90\x85V[`@Qa\x02\x98\x95\x94\x93\x92\x91\x90aR\xB0V[4\x80\x15a\x07\x18W`\0\x80\xFD[Pa\x03t`\0\x81V[4\x80\x15a\x07-W`\0\x80\xFD[Pa\x03\ra\x07<6`\x04aOFV[a&^V[4\x80\x15a\x07MW`\0\x80\xFD[Pa\x03ta\x07\\6`\x04aO\x1AV[a&iV[4\x80\x15a\x07mW`\0\x80\xFD[Pa\x03ta'\xECV[4\x80\x15a\x07\x82W`\0\x80\xFD[Pa\x03\ra\x07\x916`\x04aR{V[a(\x17V[4\x80\x15a\x07\xA2W`\0\x80\xFD[Pa\x03ta\x07\xB16`\x04aOFV[a*\x19V[4\x80\x15a\x07\xC2W`\0\x80\xFD[Pa\x03\ra\x07\xD16`\x04aO_V[a*0V[4\x80\x15a\x07\xE2W`\0\x80\xFD[Pa\x03\ra\x07\xF16`\x04aO\xACV[a*UV[4\x80\x15a\x08\x02W`\0\x80\xFD[Pa\x03\ra\x08\x116`\x04aR\xEEV[a*\x95V[4\x80\x15a\x08\"W`\0\x80\xFD[Pa\x03\ra\x0816`\x04aOFV[a/\xD0V[4\x80\x15a\x08BW`\0\x80\xFD[Pa\x03ta\x08Q6`\x04aS\x91V[a1^V[4\x80\x15a\x08bW`\0\x80\xFD[Pa\x03\ra\x08q6`\x04aOFV[a2\xF4V[4\x80\x15a\x08\x82W`\0\x80\xFD[Pa\x03\ra\x08\x916`\x04aO\x8FV[a2\xFFV[`\0a\x08\xA1\x82a5\xD1V[\x92\x91PPV[a\x08\xAFa5\xF6V[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01T`\xC0\x83\x01R`\x07\x81\x01T`\xE0\x83\x01R`\x08\x81\x01Ta\x01\0\x83\x01R`\t\x81\x01\x80Ta\x01 \x84\x01\x91\x90a\tC\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\to\x90aS\xDBV[\x80\x15a\t\xBCW\x80`\x1F\x10a\t\x91Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xBCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x9FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x82Q`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA3`\xF0\x1B\x81RP\x90a\n\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x87\x16a\nPW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x85a\n\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80Q`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaG1`\xF0\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a\n\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@Q\x80a\x01@\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01\x86\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0\x81R` \x01\x84\x81RPa\x01_`\0\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U`\xA0\x82\x01Q\x81`\x05\x01U`\xC0\x82\x01Q\x81`\x06\x01U`\xE0\x82\x01Q\x81`\x07\x01Ua\x01\0\x82\x01Q\x81`\x08\x01Ua\x01 \x82\x01Q\x81`\t\x01\x90\x81a\x0B\xEA\x91\x90aTnV[PP\x84\x15\x90Pa\x0C)Wa\x0C)`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x830\x87a6QV[`@\x80Q\x86\x81R` \x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x7FI\x1B\x0FF\x1D\x05\xB7\x9B\xFCi\x04\xA9\xBA\x9D\xF5#\xD7fP?\x11\x04k+^n\xDD\xAA\xE3\xA7#\x1C\x91\x01`@Q\x80\x91\x03\x90\xA2PPa\x0Cz`\x01a\x01-UV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01`` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\x0C\xCCWa\x0C\xCCaP\x9FV[`\x04\x81\x11\x15a\x0C\xDDWa\x0C\xDDaP\x9FV[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P\x80`@\x01Q\x81``\x01Q\x92P\x92PP[\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01`` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\rnWa\rnaP\x9FV[`\x04\x81\x11\x15a\r\x7FWa\r\x7FaP\x9FV[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P`\0a\x01_`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80a\x01@\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01T\x81R` \x01`\x07\x82\x01T\x81R` \x01`\x08\x82\x01T\x81R` \x01`\t\x82\x01\x80Ta\x0Ep\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E\x9C\x90aS\xDBV[\x80\x15a\x0E\xE9W\x80`\x1F\x10a\x0E\xBEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\xE9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\xCCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP\x90Q`@\x90\x93\x01Q\x92\x94P\x91\x92PPP\x92P\x92\x90PV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\x0F$\x81a6\xBCV[a\x0F.\x83\x83a6\xC6V[PPPV[3`\0\x81\x81Ra\x01_` R`@\x90 `\t\x81\x01\x80Ta\x0FR\x90aS\xDBV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x91Pa\x0F\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a\x0F\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x08\x01T\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG11`\xE8\x1B\x81RP\x90a\x10\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\x06\x01T\x83`\x08\x01Ta\x108\x91\x90aUDV[a\x10B\x91\x90aU[V[\x90P`\0\x81\x83`\x06\x01Ta\x10V\x91\x90aU}V[\x90P\x82`\x03\x01T\x82\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG5`\xF0\x1B\x81RP\x90a\x10\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x82`\x02\x01T\x82\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG5`\xF0\x1B\x81RP\x90a\x10\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x81a\x11\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x06\x83\x01\x82\x90Ug\r\xE0\xB6\xB3\xA7d\0\0`\x08\x84\x01U`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x01b` R`@\x90 TC\x10\x80\x15\x90a\x11jWP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x01b` R`@\x90 T\x15\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x11\xCCM`\xEA\x1B\x81RP\x90a\x11\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x01b` R`@\x80\x82 \x91\x90\x91UQ\x7F\x11\xDE\xAE(\x9Epx\xFC\xE2\x88\xF4\xE9sN\x9C=\xF6{U\xEA\xC4\xFFA\xB8\x90\xB6\xD68\x1A\xCE\xE7b\x90a\x11\xF3\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x12qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\n\x06V[a\x12{\x82\x82a6\xD0V[PPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x12\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x90aU\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x13\x10`\0\x80Q` aYB\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x136W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x90aU\xDCV[a\x13?\x81a7)V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Ra\x13[\x91\x83\x91\x90a74V[PV[3`\0[\x82\x81\x10\x15a\x0CzWa\x13\x8C\x82\x85\x85\x84\x81\x81\x10a\x13\x80Wa\x13\x80aV(V[\x90P` \x02\x015a8\x9FV[\x80a\x13\x96\x81aV>V[\x91PPa\x13bV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x13\xBEWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x13\xD8WP0;\x15\x80\x15a\x13\xD8WP`\0T`\xFF\x16`\x01\x14[a\x14;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\n\x06V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x14^W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x14fa;?V[a\x14na;?V[a\x14va;?V[a\x14~a;?V[a\x14\x86a;?V[a\x14\x8Ea;?V[a\x14\x99`\0\x84a;\xACV[a\x14\xBBa\x14\xB5`\x01`\0\x80Q` aYb\x839\x81Q\x91RaU}V[\x83a6\xC6V[a\x01c\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\x0F.W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a\x15xW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x83U`@Q\x91\x82R\x83\x16\x90\x7F\xB1\x95\xA9K\xEC\xD3\x88\xC9\xA0\x7F\xE8\x18qh3\xBD\xF9\x8Bu\\x\xC9\xB48\xF4\xC8\xF0o5O\xA3h\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x16\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x90aU\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x16b`\0\x80Q` aYB\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x90aU\xDCV[a\x16\x91\x82a7)V[a\x12{\x82\x82`\x01a74V[a\x16\xB6`\x01`\0\x80Q` aYb\x839\x81Q\x91RaU}V[\x81V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x17YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\x06V[P`\0\x80Q` aYB\x839\x81Q\x91R\x90V[`\x003`@Qc\xED8\r\x03`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xED8\r\x03\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\xDEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\xF2W=`\0\x80>=`\0\xFD[PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01`` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\x18FWa\x18FaP\x9FV[`\x04\x81\x11\x15a\x18WWa\x18WaP\x9FV[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P`\0a\x01_`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80a\x01@\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01T\x81R` \x01`\x07\x82\x01T\x81R` \x01`\x08\x82\x01T\x81R` \x01`\t\x82\x01\x80Ta\x19H\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19t\x90aS\xDBV[\x80\x15a\x19\xC1W\x80`\x1F\x10a\x19\x96Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x19\xC1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x19\xA4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0`\x04\x81\x11\x15a\x19\xDFWa\x19\xDFaP\x9FV[\x82Q`\x04\x81\x11\x15a\x19\xF2Wa\x19\xF2aP\x9FV[\x03a\x1A\x05W`\0\x80\x93P\x93PPPa\r\x1BV[`\x04\x82Q`\x04\x81\x11\x15a\x1A\x1AWa\x1A\x1AaP\x9FV[\x03a\x1A.W`\x04`\0\x93P\x93PPPa\r\x1BV[`\0a\x1A9\x87a;\xB6V[\x90P`\0\x83Q`\x04\x81\x11\x15a\x1APWa\x1APaP\x9FV[\x14\x15\x80\x15a\x1A\\WP\x80\x15[\x15a\x1AqW`\x02`\0\x94P\x94PPPPa\r\x1BV[\x81`\xC0\x01Q\x81\x03a\x1A\x8AW`\x01\x94P\x92Pa\r\x1B\x91PPV[\x80\x15\x80\x15\x90a\x1A\x9CWP\x81`\xC0\x01Q\x81\x10[\x15a\x1A\xAFW`\x03\x94P\x92Pa\r\x1B\x91PPV[P`\0\x96\x87\x96P\x94PPPPPV[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a\x1B\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80`\t\x01\x80Ta\x1B%\x90aS\xDBV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x91Pa\x1B]W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x82\x81`\x06\x01`\0\x82\x82Ta\x1Br\x91\x90aVWV[\x90\x91UPP`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F \x98X\xCE\xC2\x8CQ\x8A\xEAD\xAA?\xA7\n\x93\"S\xF8\xBD\xF7j\x1C\x9A\x0B\x08\"j\x13\xE6\x975\xA3\x90` \x01a\x15\xC4V[a\x1B\xBAa5\xF6V[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01T`\xC0\x83\x01R`\x07\x81\x01T`\xE0\x83\x01R`\x08\x81\x01Ta\x01\0\x83\x01R`\t\x81\x01\x80Ta\x01 \x84\x01\x91\x90a\x1CN\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Cz\x90aS\xDBV[\x80\x15a\x1C\xC7W\x80`\x1F\x10a\x1C\x9CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C\xC7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\xAAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80`@\x01Q`\0\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG3`\xF0\x1B\x81RP\x90a\x1D\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P` \x81\x01Qa\x1DO\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x85\x90a=.V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01_` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x81\x01\x82\x90U`\x02\x81\x01\x82\x90U`\x03\x81\x01\x82\x90U`\x04\x81\x01\x82\x90U`\x05\x81\x01\x82\x90U`\x06\x81\x01\x82\x90U`\x07\x81\x01\x82\x90U`\x08\x81\x01\x82\x90U\x90a\x1D\xBC`\t\x83\x01\x82aMeV[PP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\x9F70\xAD\xE9K\xE5\xCE?\xAD\x97;\x88\x8At\x86j:\x91]\0\x8E\x8C\xBD\xE2\x13\x82\xB91\xB6|c\x90`\0\x90\xA2PPa\x13[`\x01a\x01-UV[a\x01_` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01T`\x07\x88\x01T`\x08\x89\x01T`\t\x8A\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x9A\x16\x9A\x98\x99\x97\x98\x96\x97\x95\x96\x94\x95\x93\x94\x92\x93\x91\x92\x91a\x1Ed\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\x90\x90aS\xDBV[\x80\x15a\x1E\xDDW\x80`\x1F\x10a\x1E\xB2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\xDDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\xC0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x8AV[`\0\x82\x81R`\x97` R`@\x81 a\x1E\xFF\x90\x83a=^V[\x93\x92PPPV[`\0\x91\x82R`e` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01T`\xC0\x83\x01R`\x07\x81\x01T`\xE0\x83\x01R`\x08\x81\x01Ta\x01\0\x83\x01R`\t\x81\x01\x80Ta\x01 \x84\x01\x91\x90a\x1F\xC5\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F\xF1\x90aS\xDBV[\x80\x15a >W\x80`\x1F\x10a \x13Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a >V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a !W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPa\x01cT`@Qcb\x82R?`\xE1\x1B\x81R`\x04\x81\x01\x8A\x90R\x92\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xC5\x04\xA4~\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xBC\x91\x90aVjV[\x90P\x80\x15\x15\x80a \xECWP\x7F\x99\xFF\r\x91%\xE1\xFC\x951\xA1\x12b\xE1Z\xEB,`P\x9A\x07\x8CL\xC4\xC6L\xEF\xDF\xB0o\xF6\x86G\x81\x14\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG15`\xE8\x1B\x81RP\x90a!&W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pa!0\x86a=jV[\x81\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA11`\xE8\x1B\x81RP\x90a!lW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x81Q`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a!\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\0\x80a!\xBC\x88a=\xA3V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x89\x16` \x82\x01R\x92\x94P\x90\x92P`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a!\xFC\x82a=\xD9V[\x90P`\0a\"@\x82\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa>,\x92PPPV[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04\x13\x13`\xEC\x1B\x81RP\x90a\"\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ci\xFD\xBC\xCA\x89\x8E\x88\x8F`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\"\xE4\x94\x93\x92\x91\x90aV\x83V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\"\xFEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a#\x12W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPPPV[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a#{W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80`\t\x01\x80Ta#\x8B\x90aS\xDBV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x91Pa#\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x08\x01T\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bTR5`\xE8\x1B\x81RP\x90a$\rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA7`\xF0\x1B` \x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0\x84\x10a$OW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\x06\x01T\x85a$k\x91\x90aUDV[a$u\x91\x90aU[V[\x90P\x81`\x02\x01T\x81\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA7`\xF0\x1B\x81RP\x90a$\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x08\x82\x01\x84\x90Ua$\xCB`\x01CaVWV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x01b` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x7Fv\x12,\xFD_<h\0\xA2#\xE6\x0C\xC6$G\xC6I6\x03}x\xD0-\x91\x92\x93\xF6U\xBA}T\xCB\x90a\x11\xF3\x90\x87\x81R` \x01\x90V[a%4`\x01`\0\x80Q` aYb\x839\x81Q\x91RaU}V[a%=\x81a6\xBCV[`\0a%I\x85\x85a\x17\xFAV[P\x90P`\x03\x81`\x04\x81\x11\x15a%`Wa%`aP\x9FV[\x14\x80a%}WP`\x04\x81`\x04\x81\x11\x15a%{Wa%{aP\x9FV[\x14[\x80a%\x99WP`\x02\x81`\x04\x81\x11\x15a%\x97Wa%\x97aP\x9FV[\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a#\x9B`\xF1\x1B\x81RP\x90a%\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81Ra\x01_` \x90\x81R`@\x80\x83 a\x01`\x83R\x81\x84 \x88\x85R\x90\x92R\x82 `\x01\x81\x01T`\x03\x83\x01\x80T\x93\x94\x92\x93\x91\x92\x83\x92a&\x1D\x90\x84\x90aU}V[\x92PP\x81\x90UP\x85\x83`\x04\x01`\0\x82\x82Ta&8\x91\x90aU}V[\x90\x91UPP`\x04\x82\x01\x80T\x90`\0a&O\x83aV\xBCV[\x91\x90PUPPPPPPPPPV[3a\x12{\x81\x83a8\x9FV[`\0a&sa5\xF6V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x01_` R`@\x90 `\t\x81\x01\x80Ta&\x9B\x90aS\xDBV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x91Pa&\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a'\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x83a'NW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pa'\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x1630\x86a6QV[\x82\x81`\x01\x01`\0\x82\x82Ta'\x98\x91\x90aVWV[\x90\x91UPP`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xB6\xBE#\x16\x85\x06\xA1\xCEb\xCCo\x933\x9Fg\x10\xE0\x98De\x17\n\xB5-\xF8\xEC\xF7\xDA8\xB3E\x84\x90` \x01`@Q\x80\x91\x03\x90\xA2`\x01\x01T\x90Pa\x08\xA1`\x01a\x01-UV[a\x16\xB6`\x01\x7F\xDCi|\xE6\xC0\xC3\xF8\x86\xFB\xBC\x8E\r#:\xBF\xACZ\xC56S\xAC\x08\xB3c[\xB0\xAE\xC7\x0C9\\BaU}V[a(\x1Fa5\xF6V[a(8`\x01`\0\x80Q` aYb\x839\x81Q\x91RaU}V[a(A\x81a6\xBCV[`\0\x80a(N\x86\x86a\x17\xFAV[\x90\x92P\x90P`\x01\x82`\x04\x81\x11\x15a(gWa(gaP\x9FV[\x14\x80a(\x84WP`\x03\x82`\x04\x81\x11\x15a(\x82Wa(\x82aP\x9FV[\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x11\xCD`\xF2\x1B\x81RP\x90a(\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81Ra\x01_` \x90\x81R`@\x80\x83 a\x01`\x83R\x81\x84 \x89\x85R\x83R\x92\x81\x90 `\x01\x81\x01T\x82Q\x80\x84\x01\x90\x93R`\x02\x83RaG5`\xF0\x1B\x93\x83\x01\x93\x90\x93R\x91\x84\x10\x15a)*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`d\x81`\x04\x01T\x11\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b#\x98\x99`\xE9\x1B\x81RP\x90a)nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\0a)z\x89a>PV[\x90P\x86\x81\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA5`\xF0\x1B\x81RP\x90a)\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x86\x83`\x04\x01`\0\x82\x82Ta)\xCE\x91\x90aVWV[\x90\x91UPP`\x01\x82\x01T`\x03\x84\x01\x80T`\0\x90a)\xEC\x90\x84\x90aVWV[\x90\x91UPP`\x04\x82\x01\x80T\x90`\0a*\x03\x83aV>V[\x91\x90PUPPPPPPPa\x0F.`\x01a\x01-UV[`\0\x81\x81R`\x97` R`@\x81 a\x08\xA1\x90a?\xBFV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta*K\x81a6\xBCV[a\x0F.\x83\x83a6\xD0V[3`\0[\x82\x81\x10\x15a\x0CzWa*\x83\x82\x85\x85\x84\x81\x81\x10a*wWa*waV(V[\x90P` \x02\x015a?\xC9V[\x80a*\x8D\x81aV>V[\x91PPa*YV[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x80\x83 a\x01`\x83R\x81\x84 \x8D\x85R\x90\x92R\x80\x83 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x92\x93\x92\x82\x90`\xFF\x16`\x04\x81\x11\x15a*\xDDWa*\xDDaP\x9FV[`\x04\x81\x11\x15a*\xEEWa*\xEEaP\x9FV[\x81R`\x01\x82\x01T` \x80\x83\x01\x91\x90\x91R`\x02\x80\x84\x01T`@\x80\x85\x01\x91\x90\x91R`\x03\x85\x01T``\x85\x01R`\x04\x90\x94\x01T`\x80\x90\x93\x01\x92\x90\x92R\x85T\x83Q\x80\x85\x01\x90\x94R\x91\x83Ra#\x99`\xF1\x1B\x90\x83\x01R\x91\x92P\x90`\x01`\x01`\xA0\x1B\x03\x16a+gW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pa\x01cT`@Qc\x11\xA2R/`\xE3\x1B\x81R`\x04\x81\x01\x8D\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x8D\x12\x91x\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xD7\x91\x90aV\xD3V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aM1`\xF0\x1B\x81RP\x90a,\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\0\x81Q`\x04\x81\x11\x15a,1Wa,1aP\x9FV[\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a&\x99`\xF1\x1B\x81RP\x90a,kW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x88a,\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x8Aa,\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x89\x82`\x02\x01`\0\x82\x82Ta,\xF0\x91\x90aVWV[\x92PP\x81\x90UP\x81`\x06\x01T\x82`\x02\x01T\x11\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aM3`\xF0\x1B\x81RP\x90a-<W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x05\x82\x01\x80T\x90`\0a-O\x83aV>V[\x90\x91UPP`@\x80Q`\xA0\x81\x01\x90\x91R\x80`\x01\x81R` \x80\x82\x01\x8D\x90R`@\x80\x83\x01\x8D\x90R``\x83\x01\x8C\x90R`\0`\x80\x90\x93\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16\x83Ra\x01`\x82R\x80\x83 \x8F\x84R\x90\x91R\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a-\xBFWa-\xBFaP\x9FV[\x02\x17\x90UP` \x82\x01Q`\x01\x82\x01U`@\x80\x83\x01Q`\x02\x83\x01U``\x83\x01Q`\x03\x83\x01U`\x80\x90\x92\x01Q`\x04\x91\x82\x01Ua\x01cT\x91Qcb\x82R?`\xE1\x1B\x81R\x90\x81\x01\x8D\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC5\x04\xA4~\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\\\x91\x90aVjV[\x90P\x80\x15\x80\x15\x90a.\x8DWP\x7F\x99\xFF\r\x91%\xE1\xFC\x951\xA1\x12b\xE1Z\xEB,`P\x9A\x07\x8CL\xC4\xC6L\xEF\xDF\xB0o\xF6\x86G\x81\x14\x15[\x15a/~Wa.\x9B\x87a=jV[\x81\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA11`\xE8\x1B\x81RP\x90a.\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x87\x15a/~Wa.\xEA\x84\x88\x88\x88a@\xA8V[`\0a.\xF5\x88a=\xA3V[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ci\xFD\xBC\xCA\x86\x8F\x84\x8C`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/J\x94\x93\x92\x91\x90aV\x83V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/dW`\0\x80\xFD[PZ\xF1\x15\x80\x15a/xW=`\0\x80>=`\0\xFD[PPPPP[\x8B\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\x11\xF4P\xA0k\xAF\xFB\xE3T\xCA\xF0\xB61\xEAs\x85\xF51\xDB|\x81\xD1F+}\xF8c\x87$\xCD}a\x8D`@Qa/\xBA\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPPV[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a0'W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80`\t\x01\x80Ta07\x90aS\xDBV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x91Pa0oW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x07\x01T\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bTR5`\xE8\x1B\x81RP\x90a0\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA7`\xF0\x1B` \x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0\x84\x10a0\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x07\x81\x01\x83\x90Ua1\x0E`\x01CaVWV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x01a` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x7F\x13\x9A\xD7\xA0\xC3\xF6\xC0\xAD\x0F\x0F\xC5+vx`jq\xFF\x08\x1B\x99&\xA5do\xE5:\x8Eg\xC6\x8Ar\x90a\x15\xC4\x90\x86\x81R` \x01\x90V[`\0a1y`\x01`\0\x80Q` aYb\x839\x81Q\x91RaU}V[a1\x82\x81a6\xBCV[`\0a1\x8E\x87\x87a\x17\xFAV[P\x90P`\x03\x81`\x04\x81\x11\x15a1\xA5Wa1\xA5aP\x9FV[\x14\x80a1\xC2WP`\x04\x81`\x04\x81\x11\x15a1\xC0Wa1\xC0aP\x9FV[\x14[\x80a1\xDEWP`\x02\x81`\x04\x81\x11\x15a1\xDCWa1\xDCaP\x9FV[\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x10M`\xF2\x1B\x81RP\x90a2\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81Ra\x01_` \x90\x81R`@\x80\x83 a\x01`\x83R\x81\x84 \x8A\x85R\x90\x92R\x82 `\x04\x81\x01\x80T\x92\x93\x91\x92\x91a2X\x83aV\xBCV[\x91\x90PUP\x86\x82`\x01\x01`\0\x82\x82Ta2q\x91\x90aU}V[\x92PP\x81\x90UP\x86\x82`\x04\x01`\0\x82\x82Ta2\x8C\x91\x90aU}V[\x90\x91UPP`\x01\x81\x01T`\x03\x83\x01\x80T`\0\x90a2\xAA\x90\x84\x90aU}V[\x90\x91UPa2\xE4\x90P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x89a=.V[P`\x01\x01T\x97\x96PPPPPPPV[3a\x12{\x81\x83a?\xC9V[a3\x07a5\xF6V[3`\0\x81\x81Ra\x01_` R`@\x90 `\t\x81\x01\x80Ta3&\x90aS\xDBV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x91Pa3^W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a3\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x07\x01T\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04s\x13`\xEC\x1B\x81RP\x90a3\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\x01\x01T\x83`\x07\x01Ta4\x0C\x91\x90aUDV[a4\x16\x91\x90aU[V[\x90P`\0\x81\x83`\x01\x01Ta4*\x91\x90aU}V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x90\x91P\x81a4dW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x82`\x04\x01T\x82\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA5`\xF0\x1B\x81RP\x90a4\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pa4\xDB`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86\x83a=.V[`\x01\x83\x01\x82\x90Ug\r\xE0\xB6\xB3\xA7d\0\0`\x07\x84\x01U`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x01a` R`@\x90 TC\x10\x80\x15\x90a51WP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x01a` R`@\x90 T\x15\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x11\xCCM`\xEA\x1B\x81RP\x90a5kW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x01a` R`@\x80\x82 \x91\x90\x91UQ\x7F\xE5+<X\xA1\x16\xC1\xF0\x12\xC9\x9D\x11 \xC0T\xE7?Q\xB7\xA2\x9CEq\x92E_\xD2\xC2\x0E\x03\xA53\x90a5\xBA\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPa\x13[`\x01a\x01-UV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x08\xA1WPa\x08\xA1\x82aA\x91V[`\x02a\x01-T\x03a6IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\n\x06V[`\x02a\x01-UV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x0Cz\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91RaA\xC6V[a\x13[\x813aB\x9BV[a\x12{\x82\x82aB\xF4V[a6\xDA\x82\x82aC\x16V[a6\xE4`\0a*\x19V[`\0\x03a\x12{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrCannot be adminless`h\x1B`D\x82\x01R`d\x01a\n\x06V[`\0a\x12{\x81a6\xBCV[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a7gWa\x0F.\x83aC8V[\x82`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a7\xC1WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra7\xBE\x91\x81\x01\x90aVjV[`\x01[a8$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01Rmon is not UUPS`\x90\x1B`d\x82\x01R`\x84\x01a\n\x06V[`\0\x80Q` aYB\x839\x81Q\x91R\x81\x14a8\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01Rh\x1AXX\x9B\x19UURQ`\xBA\x1B`d\x82\x01R`\x84\x01a\n\x06V[Pa\x0F.\x83\x83\x83aC\xD4V[a\x01cT`@Qc\x11\xA2R/`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x8D\x12\x91x\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x0E\x91\x90aV\xD3V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aM1`\xF0\x1B\x81RP\x90a9RW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01`` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x82\x90`\xFF\x16`\x04\x81\x11\x15a9\x99Wa9\x99aP\x9FV[`\x04\x81\x11\x15a9\xAAWa9\xAAaP\x9FV[\x81R`\x01\x82\x01T` \x82\x01R`\x02\x82\x01T`@\x82\x01R`\x03\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x90\x91\x01R\x90P`\0\x81Q`\x04\x81\x11\x15a9\xEDWa9\xEDaP\x9FV[\x14\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG9`\xF0\x1B\x81RP\x90a:(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x80\x81\x01Q`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x13M`\xF2\x1B` \x82\x01R\x90\x15a:fW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\0a\x01_`\0\x85`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x90P\x81` \x01Q\x81`\x02\x01`\0\x82\x82Ta:\xA9\x91\x90aU}V[\x92PP\x81\x90UP`\x01\x81`\x05\x01`\0\x82\x82Ta:\xC5\x91\x90aU}V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x01`` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x80\x82 \x80T`\xFF\x19\x16\x81U`\x01\x81\x01\x83\x90U`\x02\x81\x01\x83\x90U`\x03\x81\x01\x83\x90U`\x04\x01\x82\x90UQ\x85\x92\x91\x7F\x83\x1C\xD5\xB7S\x83\x80\xB0\xA2\xA3\x14\x14\xD64\xECBq\x16\x07V\x84\xA2v\x82\x8A\xB4\xD2ut\xA2\xDF\xDF\x91\xA3PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a;\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\n\x06V[V[a\x12{\x82\x82a6\xC6V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81Ra\x01_` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T\x90\x95\x16\x85R`\x01\x81\x01T\x92\x85\x01\x92\x90\x92R`\x02\x82\x01T\x90\x84\x01R`\x03\x81\x01T``\x84\x01R`\x04\x81\x01T`\x80\x84\x01R`\x05\x81\x01T`\xA0\x84\x01R`\x06\x81\x01T`\xC0\x84\x01R`\x07\x81\x01T`\xE0\x84\x01R`\x08\x81\x01Ta\x01\0\x84\x01R`\t\x81\x01\x80T\x92\x93\x84\x93\x90\x92\x91a\x01 \x84\x01\x91a<Q\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta<}\x90aS\xDBV[\x80\x15a<\xCAW\x80`\x1F\x10a<\x9FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a<\xCAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a<\xADW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82a\x01\0\x01Q\x83`\xC0\x01Qa<\xF5\x91\x90aUDV[a<\xFF\x91\x90aU[V[\x90P\x81``\x01Q\x81\x10\x15a=\x17WP`\0\x93\x92PPPV[``\x82\x01Qa=&\x90\x82aU}V[\x94\x93PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x0F.\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a6\x85V[`\0a\x1E\xFF\x83\x83aC\xF9V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a=\x84\x91\x90aWEV[PP\x95P\x95P\x95PPPPa=\x9A\x83\x83\x83aD#V[\x95\x94PPPPPV[```\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a=\xBC\x91\x90aWEV[PPPPP\x92PPP\x80a=\xCF\x82aD\\V[\x92P\x92PP\x91P\x91V[`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0a>;\x85\x85aD\xA9V[\x91P\x91Pa>H\x81aD\xEBV[P\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81Ra\x01_` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T\x90\x95\x16\x85R`\x01\x81\x01T\x92\x85\x01\x92\x90\x92R`\x02\x82\x01T\x90\x84\x01R`\x03\x81\x01T``\x84\x01R`\x04\x81\x01T`\x80\x84\x01R`\x05\x81\x01T`\xA0\x84\x01R`\x06\x81\x01T`\xC0\x84\x01R`\x07\x81\x01T`\xE0\x84\x01R`\x08\x81\x01Ta\x01\0\x84\x01R`\t\x81\x01\x80T\x92\x93\x84\x93\x90\x92\x91a\x01 \x84\x01\x91a>\xEB\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta?\x17\x90aS\xDBV[\x80\x15a?dW\x80`\x1F\x10a?9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a?dV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a?GW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\xE0\x01Q\x83` \x01Qa?\x8E\x91\x90aUDV[a?\x98\x91\x90aU[V[\x90P\x81`\x80\x01Q\x81\x10\x15a?\xB0WP`\0\x93\x92PPPV[`\x80\x82\x01Qa=&\x90\x82aU}V[`\0a\x08\xA1\x82T\x90V[`\0a?\xD5\x83\x83a\x17\xFAV[P\x90P`\0\x81`\x04\x81\x11\x15a?\xECWa?\xECaP\x9FV[\x14\x15\x80\x15a@\x0CWP`\x04\x81`\x04\x81\x11\x15a@\tWa@\taP\x9FV[\x14\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x08\xE7`\xF3\x1B\x81RP\x90a@EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x01`` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x80\x82 \x80T`\xFF\x19\x16`\x04\x17\x81U\x90Q\x90\x92\x85\x92\x90\x91\x7F\x98o\xCB\xE6\xF7\xD2\xB4Q\xF9\x11\\{\xE2.\x18oC\xEB1I\x8C\x85B\xF7\x98!H\xF3\xB5e\xCF\xD3\x91\x90\xA3PPPPV[`\0a@\xB3\x84a=\xA3V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x89\x16` \x82\x01R\x91\x93P`\0\x92P\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a@\xF1\x82a=\xD9V[\x90P`\0aA5\x82\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa>,\x92PPPV[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04\x13\x13`\xEC\x1B\x81RP\x90aA\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[PPPPPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x08\xA1WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x08\xA1V[`\0aB\x1B\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16aF5\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80aB<WP\x80\x80` \x01\x90Q\x81\x01\x90aB<\x91\x90aX:V[a\x0F.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\n\x06V[aB\xA5\x82\x82a\x1F\x06V[a\x12{WaB\xB2\x81aFDV[aB\xBD\x83` aFVV[`@Q` \x01aB\xCE\x92\x91\x90aXWV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\n\x06\x91`\x04\x01aT\x15V[aB\xFE\x82\x82aG\xF2V[`\0\x82\x81R`\x97` R`@\x90 a\x0F.\x90\x82aHxV[aC \x82\x82aH\x8DV[`\0\x82\x81R`\x97` R`@\x90 a\x0F.\x90\x82aH\xF4V[`\x01`\x01`\xA0\x1B\x03\x81\x16;aC\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01a\n\x06V[`\0\x80Q` aYB\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[aC\xDD\x83aI\tV[`\0\x82Q\x11\x80aC\xEAWP\x80[\x15a\x0F.Wa\x0Cz\x83\x83aIIV[`\0\x82`\0\x01\x82\x81T\x81\x10aD\x10WaD\x10aV(V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0\x80\x84\x84\x84`@Q` \x01aD;\x93\x92\x91\x90aX\xCCV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`\0\x81Q`@\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG7`\xF0\x1B\x81RP\x90aD\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[PP\x80Q` \x90\x91\x01 \x90V[`\0\x80\x82Q`A\x03aD\xDFW` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaD\xD3\x87\x82\x85\x85aInV[\x94P\x94PPPPa\r\x1BV[P`\0\x90P`\x02a\r\x1BV[`\0\x81`\x04\x81\x11\x15aD\xFFWaD\xFFaP\x9FV[\x03aE\x07WPV[`\x01\x81`\x04\x81\x11\x15aE\x1BWaE\x1BaP\x9FV[\x03aEhW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\x06V[`\x02\x81`\x04\x81\x11\x15aE|WaE|aP\x9FV[\x03aE\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\n\x06V[`\x03\x81`\x04\x81\x11\x15aE\xDDWaE\xDDaP\x9FV[\x03a\x13[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n\x06V[``a=&\x84\x84`\0\x85aJ2V[``a\x08\xA1`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0aFe\x83`\x02aUDV[aFp\x90`\x02aVWV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF\x88WaF\x88aM\xF2V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aF\xB2W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10aF\xCDWaF\xCDaV(V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10aF\xFCWaF\xFCaV(V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0aG \x84`\x02aUDV[aG+\x90`\x01aVWV[\x90P[`\x01\x81\x11\x15aG\xA3Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10aG_WaG_aV(V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10aGuWaGuaV(V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93aG\x9C\x81aV\xBCV[\x90PaG.V[P\x83\x15a\x1E\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\n\x06V[aG\xFC\x82\x82a\x1F\x06V[a\x12{W`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UaH43\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x1E\xFF\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aK\rV[aH\x97\x82\x82a\x1F\x06V[\x15a\x12{W`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x1E\xFF\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aK\\V[aI\x12\x81aC8V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a\x1E\xFF\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01aY\x82`'\x919aLOV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aI\xA5WP`\0\x90P`\x03aJ)V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aI\xF9W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aJ\"W`\0`\x01\x92P\x92PPaJ)V[\x91P`\0\x90P[\x94P\x94\x92PPPV[``\x82G\x10\x15aJ\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\n\x06V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@QaJ\xAF\x91\x90aY\x0FV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14aJ\xECW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aJ\xF1V[``\x91P[P\x91P\x91PaK\x02\x87\x83\x83\x87aL\xC7V[\x97\x96PPPPPPPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 TaKTWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x08\xA1V[P`\0a\x08\xA1V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aLEW`\0aK\x80`\x01\x83aU}V[\x85T\x90\x91P`\0\x90aK\x94\x90`\x01\x90aU}V[\x90P\x81\x81\x14aK\xF9W`\0\x86`\0\x01\x82\x81T\x81\x10aK\xB4WaK\xB4aV(V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10aK\xD7WaK\xD7aV(V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80aL\nWaL\naY+V[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x08\xA1V[`\0\x91PPa\x08\xA1V[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@QaLl\x91\x90aY\x0FV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14aL\xA7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aL\xACV[``\x91P[P\x91P\x91PaL\xBD\x86\x83\x83\x87aL\xC7V[\x96\x95PPPPPPV[``\x83\x15aM6W\x82Q`\0\x03aM/W`\x01`\x01`\xA0\x1B\x03\x85\x16;aM/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\n\x06V[P\x81a=&V[a=&\x83\x83\x81Q\x15aMKW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80TaMq\x90aS\xDBV[`\0\x82U\x80`\x1F\x10aM\x81WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x13[\x91\x90[\x80\x82\x11\x15aM\xAFW`\0\x81U`\x01\x01aM\x9BV[P\x90V[`\0` \x82\x84\x03\x12\x15aM\xC5W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1E\xFFW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13[W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aN1WaN1aM\xF2V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aNSWaNSaM\xF2V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aNrW`\0\x80\xFD[\x815aN\x85aN\x80\x82aN9V[aN\x08V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aN\x9AW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aN\xCDW`\0\x80\xFD[\x845aN\xD8\x81aM\xDDV[\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aO\x02W`\0\x80\xFD[aO\x0E\x87\x82\x88\x01aNaV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15aO-W`\0\x80\xFD[\x825aO8\x81aM\xDDV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15aOXW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aOrW`\0\x80\xFD[\x825\x91P` \x83\x015aO\x84\x81aM\xDDV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15aO\xA1W`\0\x80\xFD[\x815a\x1E\xFF\x81aM\xDDV[`\0\x80` \x83\x85\x03\x12\x15aO\xBFW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aO\xD7W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aO\xEBW`\0\x80\xFD[\x815\x81\x81\x11\x15aO\xFAW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15aP\x0FW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15aP4W`\0\x80\xFD[\x825aP?\x81aM\xDDV[\x91P` \x83\x015aO\x84\x81aM\xDDV[`\0\x80`@\x83\x85\x03\x12\x15aPbW`\0\x80\xFD[\x825aPm\x81aM\xDDV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aP\x89W`\0\x80\xFD[aP\x95\x85\x82\x86\x01aNaV[\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x05\x81\x10aP\xD3WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`@\x81\x01aP\xE5\x82\x85aP\xB5V[\x82` \x83\x01R\x93\x92PPPV[`\0[\x83\x81\x10\x15aQ\rW\x81\x81\x01Q\x83\x82\x01R` \x01aP\xF5V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84RaQ.\x81` \x86\x01` \x86\x01aP\xF2V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0a\x01@`\x01\x80`\xA0\x1B\x03\x8D\x16\x83R\x8B` \x84\x01R\x8A`@\x84\x01R\x89``\x84\x01R\x88`\x80\x84\x01R\x87`\xA0\x84\x01R\x86`\xC0\x84\x01R\x85`\xE0\x84\x01R\x84a\x01\0\x84\x01R\x80a\x01 \x84\x01RaQ\x96\x81\x84\x01\x85aQ\x16V[\x9D\x9CPPPPPPPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aQ\xBAW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80\x83`\x1F\x84\x01\x12aQ\xDBW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aQ\xF3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\r\x1BW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15aR!W`\0\x80\xFD[\x845\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aR@W`\0\x80\xFD[aRL\x88\x83\x89\x01aNaV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15aRbW`\0\x80\xFD[PaRo\x87\x82\x88\x01aQ\xC9V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aR\x90W`\0\x80\xFD[\x835aR\x9B\x81aM\xDDV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\xA0\x81\x01aR\xBE\x82\x88aP\xB5V[\x85` \x83\x01R\x84`@\x83\x01R\x83``\x83\x01R\x82`\x80\x83\x01R\x96\x95PPPPPPV[\x80\x15\x15\x81\x14a\x13[W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x89\x8B\x03\x12\x15aS\nW`\0\x80\xFD[\x885\x97P` \x89\x015\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015aS1\x81aR\xE0V[\x93P`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aSNW`\0\x80\xFD[aSZ\x8C\x83\x8D\x01aNaV[\x94P`\xC0\x8B\x015\x91P\x80\x82\x11\x15aSpW`\0\x80\xFD[PaS}\x8B\x82\x8C\x01aQ\xC9V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aS\xA7W`\0\x80\xFD[\x845aS\xB2\x81aM\xDDV[\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015aS\xD0\x81aM\xDDV[\x93\x96\x92\x95P\x90\x93PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80aS\xEFW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aT\x0FWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x81R`\0a\x1E\xFF` \x83\x01\x84aQ\x16V[`\x1F\x82\x11\x15a\x0F.W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15aTOWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x17\xF2W\x82\x81U`\x01\x01aT[V[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aT\x88WaT\x88aM\xF2V[aT\x9C\x81aT\x96\x84TaS\xDBV[\x84aT(V[` \x80`\x1F\x83\x11`\x01\x81\x14aT\xD1W`\0\x84\x15aT\xB9WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x17\xF2V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aU\0W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aT\xE1V[P\x85\x82\x10\x15aU\x1EW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x08\xA1Wa\x08\xA1aU.V[`\0\x82aUxWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x08\xA1Wa\x08\xA1aU.V[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rk\x19\x19[\x19Y\xD8]\x19X\xD8[\x1B`\xA2\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rkactive proxy`\xA0\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01aVPWaVPaU.V[P`\x01\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x08\xA1Wa\x08\xA1aU.V[`\0` \x82\x84\x03\x12\x15aV|W`\0\x80\xFD[PQ\x91\x90PV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0aV\xAA`\x80\x83\x01\x85aQ\x16V[\x82\x81\x03``\x84\x01RaK\x02\x81\x85aQ\x16V[`\0\x81aV\xCBWaV\xCBaU.V[P`\0\x19\x01\x90V[`\0` \x82\x84\x03\x12\x15aV\xE5W`\0\x80\xFD[\x81Qa\x1E\xFF\x81aM\xDDV[`\0\x82`\x1F\x83\x01\x12aW\x01W`\0\x80\xFD[\x81QaW\x0FaN\x80\x82aN9V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aW$W`\0\x80\xFD[a=&\x82` \x83\x01` \x87\x01aP\xF2V[\x80QaW@\x81aM\xDDV[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15aWbW`\0\x80\xFD[\x88Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aWzW`\0\x80\xFD[aW\x86\x8C\x83\x8D\x01aV\xF0V[\x99PaW\x94` \x8C\x01aW5V[\x98P`@\x8B\x01Q\x91P\x80\x82\x11\x15aW\xAAW`\0\x80\xFD[aW\xB6\x8C\x83\x8D\x01aV\xF0V[\x97P``\x8B\x01Q\x91P\x80\x82\x11\x15aW\xCCW`\0\x80\xFD[aW\xD8\x8C\x83\x8D\x01aV\xF0V[\x96P`\x80\x8B\x01Q\x91P\x80\x82\x11\x15aW\xEEW`\0\x80\xFD[aW\xFA\x8C\x83\x8D\x01aV\xF0V[\x95P`\xA0\x8B\x01Q\x91P\x80\x82\x11\x15aX\x10W`\0\x80\xFD[PaX\x1D\x8B\x82\x8C\x01aV\xF0V[`\xC0\x8B\x01Q`\xE0\x90\x9B\x01Q\x99\x9C\x98\x9BP\x96\x99\x95\x98\x94\x97\x93PPPPV[`\0` \x82\x84\x03\x12\x15aXLW`\0\x80\xFD[\x81Qa\x1E\xFF\x81aR\xE0V[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83QaX\x8F\x81`\x17\x85\x01` \x88\x01aP\xF2V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83QaX\xC0\x81`(\x84\x01` \x88\x01aP\xF2V[\x01`(\x01\x94\x93PPPPV[`\0\x84QaX\xDE\x81\x84` \x89\x01aP\xF2V[\x84Q\x90\x83\x01\x90aX\xF2\x81\x83` \x89\x01aP\xF2V[\x84Q\x91\x01\x90aY\x05\x81\x83` \x88\x01aP\xF2V[\x01\x95\x94PPPPPV[`\0\x82QaY!\x81\x84` \x87\x01aP\xF2V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x12\xB4.\x8A\x16\x0F`d\xDC\x95\x9Co%\x1E:\xF0u\n\xD2\x13\xDB\xEC\xF5s\xB4q\rg\xD6\xC2\x8E9Address: low-level delegate call failed\xA2dipfsX\"\x12 ^p\xB9\x11f\xB1U[\xCE\xDA\xC1\xE4D\x1D\x88\x94\xDF\xD70\x81:/|M\x96\xD4\x1AJ3\x9E\xED\x82dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static GENERATORREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x02gW`\x005`\xE0\x1C\x80cz\x14\xC4c\x11a\x01DW\x80c\xAD\xC9w.\x11a\0\xB6W\x80c\xD5a9\xB6\x11a\0zW\x80c\xD5a9\xB6\x14a\x07\xD6W\x80c\xE8R\x10\xBD\x14a\x07\xF6W\x80c\xE9\xE94\xA0\x14a\x08\x16W\x80c\xEA\xAC\xAE\x94\x14a\x086W\x80c\xEE\x93(c\x14a\x08VW\x80c\xF2\x88\x8D\xBB\x14a\x08vW`\0\x80\xFD[\x80c\xAD\xC9w.\x14a\x07AW\x80c\xB8\n\xAA\x89\x14a\x07aW\x80c\xC4\x92\xEE9\x14a\x07vW\x80c\xCA\x15\xC8s\x14a\x07\x96W\x80c\xD5Gt\x1F\x14a\x07\xB6W`\0\x80\xFD[\x80c\x92\xEB\x91\xE2\x11a\x01\x08W\x80c\x92\xEB\x91\xE2\x14a\x06AW\x80c\x96\xDE\x0E\xEF\x14a\x06aW\x80c\x98*A]\x14a\x06\x81W\x80c\x9A\x7F\xCA\x8E\x14a\x06\xA1W\x80c\xA2\x17\xFD\xDF\x14a\x07\x0CW\x80c\xA6KR\x13\x14a\x07!W`\0\x80\xFD[\x80cz\x14\xC4c\x14a\x046W\x80c\x84\xAC3\xEC\x14a\x05\xABW\x80c\x8C\xFCV\xD8\x14a\x05\xCBW\x80c\x90\x10\xD0|\x14a\x06\x01W\x80c\x91\xD1HT\x14a\x06!W`\0\x80\xFD[\x80cF\x15\xE8\xBC\x11a\x01\xDDW\x80cR\xD1\x90-\x11a\x01\xA1W\x80cR\xD1\x90-\x14a\x04\xD3W\x80cT\x1A\x8C\x18\x14a\x04\xE8W\x80cdmQ\xB4\x14a\x05\x08W\x80cf\x1D\xE5\xAC\x14a\x056W\x80cm@Xw\x14a\x05jW\x80cs*\x9Dc\x14a\x05\x8AW`\0\x80\xFD[\x80cF\x15\xE8\xBC\x14a\x04KW\x80cH\\\xC9U\x14a\x04kW\x80cM*\xAB\x9A\x14a\x04\x8BW\x80cO\x1E\xF2\x86\x14a\x04\xABW\x80cP\x95\xAFd\x14a\x04\xBEW`\0\x80\xFD[\x80c+a\x0C-\x11a\x02/W\x80c+a\x0C-\x14a\x03\x82W\x80c//\xF1]\x14a\x03\xC1W\x80c/\x8FJ;\x14a\x03\xE1W\x80c6V\x8A\xBE\x14a\x03\xF6W\x80c6Y\xCF\xE6\x14a\x04\x16W\x80c<^\xB5|\x14a\x046W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02lW\x80c\x04y\xD6D\x14a\x02\xA1W\x80c\x13m\xFB\xF5\x14a\x02\xEDW\x80c\x1C~\xAEe\x14a\x03\x0FW\x80c$\x8A\x9C\xA3\x14a\x03DW[`\0\x80\xFD[4\x80\x15a\x02xW`\0\x80\xFD[Pa\x02\x8Ca\x02\x876`\x04aM\xB3V[a\x08\x96V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xADW`\0\x80\xFD[Pa\x02\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x98V[4\x80\x15a\x02\xF9W`\0\x80\xFD[Pa\x03\ra\x03\x086`\x04aN\xB7V[a\x08\xA7V[\0[4\x80\x15a\x03\x1BW`\0\x80\xFD[Pa\x03/a\x03*6`\x04aO\x1AV[a\x0C\x80V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x98V[4\x80\x15a\x03PW`\0\x80\xFD[Pa\x03ta\x03_6`\x04aOFV[`\0\x90\x81R`e` R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x02\x98V[4\x80\x15a\x03\x8EW`\0\x80\xFD[Pa\x03\xA2a\x03\x9D6`\x04aO\x1AV[a\r\"V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x98V[4\x80\x15a\x03\xCDW`\0\x80\xFD[Pa\x03\ra\x03\xDC6`\x04aO_V[a\x0F\tV[4\x80\x15a\x03\xEDW`\0\x80\xFD[Pa\x03\ra\x0F3V[4\x80\x15a\x04\x02W`\0\x80\xFD[Pa\x03\ra\x04\x116`\x04aO_V[a\x12\x01V[4\x80\x15a\x04\"W`\0\x80\xFD[Pa\x03\ra\x0416`\x04aO\x8FV[a\x12\x7FV[4\x80\x15a\x04BW`\0\x80\xFD[Pa\x03t`d\x81V[4\x80\x15a\x04WW`\0\x80\xFD[Pa\x03\ra\x04f6`\x04aO\xACV[a\x13^V[4\x80\x15a\x04wW`\0\x80\xFD[Pa\x03\ra\x04\x866`\x04aP!V[a\x13\x9EV[4\x80\x15a\x04\x97W`\0\x80\xFD[Pa\x03\ra\x04\xA66`\x04aO\x8FV[a\x15!V[a\x03\ra\x04\xB96`\x04aPOV[a\x15\xD1V[4\x80\x15a\x04\xCAW`\0\x80\xFD[Pa\x03ta\x16\x9DV[4\x80\x15a\x04\xDFW`\0\x80\xFD[Pa\x03ta\x16\xB9V[4\x80\x15a\x04\xF4W`\0\x80\xFD[Pa\x03\ra\x05\x036`\x04aOFV[a\x17lV[4\x80\x15a\x05\x14W`\0\x80\xFD[Pa\x05(a\x05#6`\x04aO\x1AV[a\x17\xFAV[`@Qa\x02\x98\x92\x91\x90aP\xD7V[4\x80\x15a\x05BW`\0\x80\xFD[Pa\x02\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05vW`\0\x80\xFD[Pa\x03\ra\x05\x856`\x04aOFV[a\x1A\xBEV[4\x80\x15a\x05\x96W`\0\x80\xFD[Pa\x01cTa\x02\xD5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05\xB7W`\0\x80\xFD[Pa\x03\ra\x05\xC66`\x04aO\x8FV[a\x1B\xB2V[4\x80\x15a\x05\xD7W`\0\x80\xFD[Pa\x05\xEBa\x05\xE66`\x04aO\x8FV[a\x1D\xFFV[`@Qa\x02\x98\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aQBV[4\x80\x15a\x06\rW`\0\x80\xFD[Pa\x02\xD5a\x06\x1C6`\x04aQ\xA7V[a\x1E\xE7V[4\x80\x15a\x06-W`\0\x80\xFD[Pa\x02\x8Ca\x06<6`\x04aO_V[a\x1F\x06V[4\x80\x15a\x06MW`\0\x80\xFD[Pa\x03\ra\x06\\6`\x04aR\x0BV[a\x1F1V[4\x80\x15a\x06mW`\0\x80\xFD[Pa\x03\ra\x06|6`\x04aOFV[a#$V[4\x80\x15a\x06\x8DW`\0\x80\xFD[Pa\x03\ra\x06\x9C6`\x04aR{V[a%\x1BV[4\x80\x15a\x06\xADW`\0\x80\xFD[Pa\x06\xFBa\x06\xBC6`\x04aO\x1AV[a\x01`` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T`\xFF\x90\x93\x16\x93\x91\x92\x90\x91\x90\x85V[`@Qa\x02\x98\x95\x94\x93\x92\x91\x90aR\xB0V[4\x80\x15a\x07\x18W`\0\x80\xFD[Pa\x03t`\0\x81V[4\x80\x15a\x07-W`\0\x80\xFD[Pa\x03\ra\x07<6`\x04aOFV[a&^V[4\x80\x15a\x07MW`\0\x80\xFD[Pa\x03ta\x07\\6`\x04aO\x1AV[a&iV[4\x80\x15a\x07mW`\0\x80\xFD[Pa\x03ta'\xECV[4\x80\x15a\x07\x82W`\0\x80\xFD[Pa\x03\ra\x07\x916`\x04aR{V[a(\x17V[4\x80\x15a\x07\xA2W`\0\x80\xFD[Pa\x03ta\x07\xB16`\x04aOFV[a*\x19V[4\x80\x15a\x07\xC2W`\0\x80\xFD[Pa\x03\ra\x07\xD16`\x04aO_V[a*0V[4\x80\x15a\x07\xE2W`\0\x80\xFD[Pa\x03\ra\x07\xF16`\x04aO\xACV[a*UV[4\x80\x15a\x08\x02W`\0\x80\xFD[Pa\x03\ra\x08\x116`\x04aR\xEEV[a*\x95V[4\x80\x15a\x08\"W`\0\x80\xFD[Pa\x03\ra\x0816`\x04aOFV[a/\xD0V[4\x80\x15a\x08BW`\0\x80\xFD[Pa\x03ta\x08Q6`\x04aS\x91V[a1^V[4\x80\x15a\x08bW`\0\x80\xFD[Pa\x03\ra\x08q6`\x04aOFV[a2\xF4V[4\x80\x15a\x08\x82W`\0\x80\xFD[Pa\x03\ra\x08\x916`\x04aO\x8FV[a2\xFFV[`\0a\x08\xA1\x82a5\xD1V[\x92\x91PPV[a\x08\xAFa5\xF6V[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01T`\xC0\x83\x01R`\x07\x81\x01T`\xE0\x83\x01R`\x08\x81\x01Ta\x01\0\x83\x01R`\t\x81\x01\x80Ta\x01 \x84\x01\x91\x90a\tC\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\to\x90aS\xDBV[\x80\x15a\t\xBCW\x80`\x1F\x10a\t\x91Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xBCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x9FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x82Q`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA3`\xF0\x1B\x81RP\x90a\n\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x87\x16a\nPW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x85a\n\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80Q`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaG1`\xF0\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a\n\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@Q\x80a\x01@\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01\x86\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0\x81R` \x01\x84\x81RPa\x01_`\0\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U`\xA0\x82\x01Q\x81`\x05\x01U`\xC0\x82\x01Q\x81`\x06\x01U`\xE0\x82\x01Q\x81`\x07\x01Ua\x01\0\x82\x01Q\x81`\x08\x01Ua\x01 \x82\x01Q\x81`\t\x01\x90\x81a\x0B\xEA\x91\x90aTnV[PP\x84\x15\x90Pa\x0C)Wa\x0C)`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x830\x87a6QV[`@\x80Q\x86\x81R` \x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x7FI\x1B\x0FF\x1D\x05\xB7\x9B\xFCi\x04\xA9\xBA\x9D\xF5#\xD7fP?\x11\x04k+^n\xDD\xAA\xE3\xA7#\x1C\x91\x01`@Q\x80\x91\x03\x90\xA2PPa\x0Cz`\x01a\x01-UV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01`` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\x0C\xCCWa\x0C\xCCaP\x9FV[`\x04\x81\x11\x15a\x0C\xDDWa\x0C\xDDaP\x9FV[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P\x80`@\x01Q\x81``\x01Q\x92P\x92PP[\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01`` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\rnWa\rnaP\x9FV[`\x04\x81\x11\x15a\r\x7FWa\r\x7FaP\x9FV[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P`\0a\x01_`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80a\x01@\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01T\x81R` \x01`\x07\x82\x01T\x81R` \x01`\x08\x82\x01T\x81R` \x01`\t\x82\x01\x80Ta\x0Ep\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E\x9C\x90aS\xDBV[\x80\x15a\x0E\xE9W\x80`\x1F\x10a\x0E\xBEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\xE9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\xCCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP\x90Q`@\x90\x93\x01Q\x92\x94P\x91\x92PPP\x92P\x92\x90PV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\x0F$\x81a6\xBCV[a\x0F.\x83\x83a6\xC6V[PPPV[3`\0\x81\x81Ra\x01_` R`@\x90 `\t\x81\x01\x80Ta\x0FR\x90aS\xDBV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x91Pa\x0F\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a\x0F\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x08\x01T\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG11`\xE8\x1B\x81RP\x90a\x10\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\x06\x01T\x83`\x08\x01Ta\x108\x91\x90aUDV[a\x10B\x91\x90aU[V[\x90P`\0\x81\x83`\x06\x01Ta\x10V\x91\x90aU}V[\x90P\x82`\x03\x01T\x82\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG5`\xF0\x1B\x81RP\x90a\x10\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x82`\x02\x01T\x82\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG5`\xF0\x1B\x81RP\x90a\x10\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x81a\x11\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x06\x83\x01\x82\x90Ug\r\xE0\xB6\xB3\xA7d\0\0`\x08\x84\x01U`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x01b` R`@\x90 TC\x10\x80\x15\x90a\x11jWP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x01b` R`@\x90 T\x15\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x11\xCCM`\xEA\x1B\x81RP\x90a\x11\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x01b` R`@\x80\x82 \x91\x90\x91UQ\x7F\x11\xDE\xAE(\x9Epx\xFC\xE2\x88\xF4\xE9sN\x9C=\xF6{U\xEA\xC4\xFFA\xB8\x90\xB6\xD68\x1A\xCE\xE7b\x90a\x11\xF3\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x12qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\n\x06V[a\x12{\x82\x82a6\xD0V[PPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x12\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x90aU\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x13\x10`\0\x80Q` aYB\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x136W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x90aU\xDCV[a\x13?\x81a7)V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Ra\x13[\x91\x83\x91\x90a74V[PV[3`\0[\x82\x81\x10\x15a\x0CzWa\x13\x8C\x82\x85\x85\x84\x81\x81\x10a\x13\x80Wa\x13\x80aV(V[\x90P` \x02\x015a8\x9FV[\x80a\x13\x96\x81aV>V[\x91PPa\x13bV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x13\xBEWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x13\xD8WP0;\x15\x80\x15a\x13\xD8WP`\0T`\xFF\x16`\x01\x14[a\x14;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\n\x06V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x14^W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x14fa;?V[a\x14na;?V[a\x14va;?V[a\x14~a;?V[a\x14\x86a;?V[a\x14\x8Ea;?V[a\x14\x99`\0\x84a;\xACV[a\x14\xBBa\x14\xB5`\x01`\0\x80Q` aYb\x839\x81Q\x91RaU}V[\x83a6\xC6V[a\x01c\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\x0F.W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a\x15xW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x83U`@Q\x91\x82R\x83\x16\x90\x7F\xB1\x95\xA9K\xEC\xD3\x88\xC9\xA0\x7F\xE8\x18qh3\xBD\xF9\x8Bu\\x\xC9\xB48\xF4\xC8\xF0o5O\xA3h\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x16\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x90aU\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x16b`\0\x80Q` aYB\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x90aU\xDCV[a\x16\x91\x82a7)V[a\x12{\x82\x82`\x01a74V[a\x16\xB6`\x01`\0\x80Q` aYb\x839\x81Q\x91RaU}V[\x81V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x17YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\x06V[P`\0\x80Q` aYB\x839\x81Q\x91R\x90V[`\x003`@Qc\xED8\r\x03`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xED8\r\x03\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\xDEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\xF2W=`\0\x80>=`\0\xFD[PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01`` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\x18FWa\x18FaP\x9FV[`\x04\x81\x11\x15a\x18WWa\x18WaP\x9FV[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P`\0a\x01_`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80a\x01@\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01T\x81R` \x01`\x07\x82\x01T\x81R` \x01`\x08\x82\x01T\x81R` \x01`\t\x82\x01\x80Ta\x19H\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19t\x90aS\xDBV[\x80\x15a\x19\xC1W\x80`\x1F\x10a\x19\x96Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x19\xC1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x19\xA4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0`\x04\x81\x11\x15a\x19\xDFWa\x19\xDFaP\x9FV[\x82Q`\x04\x81\x11\x15a\x19\xF2Wa\x19\xF2aP\x9FV[\x03a\x1A\x05W`\0\x80\x93P\x93PPPa\r\x1BV[`\x04\x82Q`\x04\x81\x11\x15a\x1A\x1AWa\x1A\x1AaP\x9FV[\x03a\x1A.W`\x04`\0\x93P\x93PPPa\r\x1BV[`\0a\x1A9\x87a;\xB6V[\x90P`\0\x83Q`\x04\x81\x11\x15a\x1APWa\x1APaP\x9FV[\x14\x15\x80\x15a\x1A\\WP\x80\x15[\x15a\x1AqW`\x02`\0\x94P\x94PPPPa\r\x1BV[\x81`\xC0\x01Q\x81\x03a\x1A\x8AW`\x01\x94P\x92Pa\r\x1B\x91PPV[\x80\x15\x80\x15\x90a\x1A\x9CWP\x81`\xC0\x01Q\x81\x10[\x15a\x1A\xAFW`\x03\x94P\x92Pa\r\x1B\x91PPV[P`\0\x96\x87\x96P\x94PPPPPV[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a\x1B\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80`\t\x01\x80Ta\x1B%\x90aS\xDBV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x91Pa\x1B]W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x82\x81`\x06\x01`\0\x82\x82Ta\x1Br\x91\x90aVWV[\x90\x91UPP`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F \x98X\xCE\xC2\x8CQ\x8A\xEAD\xAA?\xA7\n\x93\"S\xF8\xBD\xF7j\x1C\x9A\x0B\x08\"j\x13\xE6\x975\xA3\x90` \x01a\x15\xC4V[a\x1B\xBAa5\xF6V[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01T`\xC0\x83\x01R`\x07\x81\x01T`\xE0\x83\x01R`\x08\x81\x01Ta\x01\0\x83\x01R`\t\x81\x01\x80Ta\x01 \x84\x01\x91\x90a\x1CN\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Cz\x90aS\xDBV[\x80\x15a\x1C\xC7W\x80`\x1F\x10a\x1C\x9CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C\xC7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\xAAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80`@\x01Q`\0\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG3`\xF0\x1B\x81RP\x90a\x1D\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P` \x81\x01Qa\x1DO\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x85\x90a=.V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01_` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x81\x01\x82\x90U`\x02\x81\x01\x82\x90U`\x03\x81\x01\x82\x90U`\x04\x81\x01\x82\x90U`\x05\x81\x01\x82\x90U`\x06\x81\x01\x82\x90U`\x07\x81\x01\x82\x90U`\x08\x81\x01\x82\x90U\x90a\x1D\xBC`\t\x83\x01\x82aMeV[PP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\x9F70\xAD\xE9K\xE5\xCE?\xAD\x97;\x88\x8At\x86j:\x91]\0\x8E\x8C\xBD\xE2\x13\x82\xB91\xB6|c\x90`\0\x90\xA2PPa\x13[`\x01a\x01-UV[a\x01_` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01T`\x07\x88\x01T`\x08\x89\x01T`\t\x8A\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x9A\x16\x9A\x98\x99\x97\x98\x96\x97\x95\x96\x94\x95\x93\x94\x92\x93\x91\x92\x91a\x1Ed\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\x90\x90aS\xDBV[\x80\x15a\x1E\xDDW\x80`\x1F\x10a\x1E\xB2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\xDDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\xC0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x8AV[`\0\x82\x81R`\x97` R`@\x81 a\x1E\xFF\x90\x83a=^V[\x93\x92PPPV[`\0\x91\x82R`e` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01T`\xC0\x83\x01R`\x07\x81\x01T`\xE0\x83\x01R`\x08\x81\x01Ta\x01\0\x83\x01R`\t\x81\x01\x80Ta\x01 \x84\x01\x91\x90a\x1F\xC5\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F\xF1\x90aS\xDBV[\x80\x15a >W\x80`\x1F\x10a \x13Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a >V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a !W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPa\x01cT`@Qcb\x82R?`\xE1\x1B\x81R`\x04\x81\x01\x8A\x90R\x92\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xC5\x04\xA4~\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xBC\x91\x90aVjV[\x90P\x80\x15\x15\x80a \xECWP\x7F\x99\xFF\r\x91%\xE1\xFC\x951\xA1\x12b\xE1Z\xEB,`P\x9A\x07\x8CL\xC4\xC6L\xEF\xDF\xB0o\xF6\x86G\x81\x14\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG15`\xE8\x1B\x81RP\x90a!&W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pa!0\x86a=jV[\x81\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA11`\xE8\x1B\x81RP\x90a!lW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x81Q`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a!\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\0\x80a!\xBC\x88a=\xA3V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x89\x16` \x82\x01R\x92\x94P\x90\x92P`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a!\xFC\x82a=\xD9V[\x90P`\0a\"@\x82\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa>,\x92PPPV[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04\x13\x13`\xEC\x1B\x81RP\x90a\"\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ci\xFD\xBC\xCA\x89\x8E\x88\x8F`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\"\xE4\x94\x93\x92\x91\x90aV\x83V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\"\xFEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a#\x12W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPPPV[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a#{W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80`\t\x01\x80Ta#\x8B\x90aS\xDBV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x91Pa#\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x08\x01T\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bTR5`\xE8\x1B\x81RP\x90a$\rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA7`\xF0\x1B` \x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0\x84\x10a$OW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\x06\x01T\x85a$k\x91\x90aUDV[a$u\x91\x90aU[V[\x90P\x81`\x02\x01T\x81\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA7`\xF0\x1B\x81RP\x90a$\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x08\x82\x01\x84\x90Ua$\xCB`\x01CaVWV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x01b` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x7Fv\x12,\xFD_<h\0\xA2#\xE6\x0C\xC6$G\xC6I6\x03}x\xD0-\x91\x92\x93\xF6U\xBA}T\xCB\x90a\x11\xF3\x90\x87\x81R` \x01\x90V[a%4`\x01`\0\x80Q` aYb\x839\x81Q\x91RaU}V[a%=\x81a6\xBCV[`\0a%I\x85\x85a\x17\xFAV[P\x90P`\x03\x81`\x04\x81\x11\x15a%`Wa%`aP\x9FV[\x14\x80a%}WP`\x04\x81`\x04\x81\x11\x15a%{Wa%{aP\x9FV[\x14[\x80a%\x99WP`\x02\x81`\x04\x81\x11\x15a%\x97Wa%\x97aP\x9FV[\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a#\x9B`\xF1\x1B\x81RP\x90a%\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81Ra\x01_` \x90\x81R`@\x80\x83 a\x01`\x83R\x81\x84 \x88\x85R\x90\x92R\x82 `\x01\x81\x01T`\x03\x83\x01\x80T\x93\x94\x92\x93\x91\x92\x83\x92a&\x1D\x90\x84\x90aU}V[\x92PP\x81\x90UP\x85\x83`\x04\x01`\0\x82\x82Ta&8\x91\x90aU}V[\x90\x91UPP`\x04\x82\x01\x80T\x90`\0a&O\x83aV\xBCV[\x91\x90PUPPPPPPPPPV[3a\x12{\x81\x83a8\x9FV[`\0a&sa5\xF6V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x01_` R`@\x90 `\t\x81\x01\x80Ta&\x9B\x90aS\xDBV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x91Pa&\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a'\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x83a'NW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pa'\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x1630\x86a6QV[\x82\x81`\x01\x01`\0\x82\x82Ta'\x98\x91\x90aVWV[\x90\x91UPP`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xB6\xBE#\x16\x85\x06\xA1\xCEb\xCCo\x933\x9Fg\x10\xE0\x98De\x17\n\xB5-\xF8\xEC\xF7\xDA8\xB3E\x84\x90` \x01`@Q\x80\x91\x03\x90\xA2`\x01\x01T\x90Pa\x08\xA1`\x01a\x01-UV[a\x16\xB6`\x01\x7F\xDCi|\xE6\xC0\xC3\xF8\x86\xFB\xBC\x8E\r#:\xBF\xACZ\xC56S\xAC\x08\xB3c[\xB0\xAE\xC7\x0C9\\BaU}V[a(\x1Fa5\xF6V[a(8`\x01`\0\x80Q` aYb\x839\x81Q\x91RaU}V[a(A\x81a6\xBCV[`\0\x80a(N\x86\x86a\x17\xFAV[\x90\x92P\x90P`\x01\x82`\x04\x81\x11\x15a(gWa(gaP\x9FV[\x14\x80a(\x84WP`\x03\x82`\x04\x81\x11\x15a(\x82Wa(\x82aP\x9FV[\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x11\xCD`\xF2\x1B\x81RP\x90a(\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81Ra\x01_` \x90\x81R`@\x80\x83 a\x01`\x83R\x81\x84 \x89\x85R\x83R\x92\x81\x90 `\x01\x81\x01T\x82Q\x80\x84\x01\x90\x93R`\x02\x83RaG5`\xF0\x1B\x93\x83\x01\x93\x90\x93R\x91\x84\x10\x15a)*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`d\x81`\x04\x01T\x11\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b#\x98\x99`\xE9\x1B\x81RP\x90a)nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\0a)z\x89a>PV[\x90P\x86\x81\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA5`\xF0\x1B\x81RP\x90a)\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x86\x83`\x04\x01`\0\x82\x82Ta)\xCE\x91\x90aVWV[\x90\x91UPP`\x01\x82\x01T`\x03\x84\x01\x80T`\0\x90a)\xEC\x90\x84\x90aVWV[\x90\x91UPP`\x04\x82\x01\x80T\x90`\0a*\x03\x83aV>V[\x91\x90PUPPPPPPPa\x0F.`\x01a\x01-UV[`\0\x81\x81R`\x97` R`@\x81 a\x08\xA1\x90a?\xBFV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta*K\x81a6\xBCV[a\x0F.\x83\x83a6\xD0V[3`\0[\x82\x81\x10\x15a\x0CzWa*\x83\x82\x85\x85\x84\x81\x81\x10a*wWa*waV(V[\x90P` \x02\x015a?\xC9V[\x80a*\x8D\x81aV>V[\x91PPa*YV[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x80\x83 a\x01`\x83R\x81\x84 \x8D\x85R\x90\x92R\x80\x83 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x92\x93\x92\x82\x90`\xFF\x16`\x04\x81\x11\x15a*\xDDWa*\xDDaP\x9FV[`\x04\x81\x11\x15a*\xEEWa*\xEEaP\x9FV[\x81R`\x01\x82\x01T` \x80\x83\x01\x91\x90\x91R`\x02\x80\x84\x01T`@\x80\x85\x01\x91\x90\x91R`\x03\x85\x01T``\x85\x01R`\x04\x90\x94\x01T`\x80\x90\x93\x01\x92\x90\x92R\x85T\x83Q\x80\x85\x01\x90\x94R\x91\x83Ra#\x99`\xF1\x1B\x90\x83\x01R\x91\x92P\x90`\x01`\x01`\xA0\x1B\x03\x16a+gW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pa\x01cT`@Qc\x11\xA2R/`\xE3\x1B\x81R`\x04\x81\x01\x8D\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x8D\x12\x91x\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xD7\x91\x90aV\xD3V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aM1`\xF0\x1B\x81RP\x90a,\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\0\x81Q`\x04\x81\x11\x15a,1Wa,1aP\x9FV[\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a&\x99`\xF1\x1B\x81RP\x90a,kW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x88a,\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x8Aa,\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x89\x82`\x02\x01`\0\x82\x82Ta,\xF0\x91\x90aVWV[\x92PP\x81\x90UP\x81`\x06\x01T\x82`\x02\x01T\x11\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aM3`\xF0\x1B\x81RP\x90a-<W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x05\x82\x01\x80T\x90`\0a-O\x83aV>V[\x90\x91UPP`@\x80Q`\xA0\x81\x01\x90\x91R\x80`\x01\x81R` \x80\x82\x01\x8D\x90R`@\x80\x83\x01\x8D\x90R``\x83\x01\x8C\x90R`\0`\x80\x90\x93\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16\x83Ra\x01`\x82R\x80\x83 \x8F\x84R\x90\x91R\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a-\xBFWa-\xBFaP\x9FV[\x02\x17\x90UP` \x82\x01Q`\x01\x82\x01U`@\x80\x83\x01Q`\x02\x83\x01U``\x83\x01Q`\x03\x83\x01U`\x80\x90\x92\x01Q`\x04\x91\x82\x01Ua\x01cT\x91Qcb\x82R?`\xE1\x1B\x81R\x90\x81\x01\x8D\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC5\x04\xA4~\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\\\x91\x90aVjV[\x90P\x80\x15\x80\x15\x90a.\x8DWP\x7F\x99\xFF\r\x91%\xE1\xFC\x951\xA1\x12b\xE1Z\xEB,`P\x9A\x07\x8CL\xC4\xC6L\xEF\xDF\xB0o\xF6\x86G\x81\x14\x15[\x15a/~Wa.\x9B\x87a=jV[\x81\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA11`\xE8\x1B\x81RP\x90a.\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x87\x15a/~Wa.\xEA\x84\x88\x88\x88a@\xA8V[`\0a.\xF5\x88a=\xA3V[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ci\xFD\xBC\xCA\x86\x8F\x84\x8C`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/J\x94\x93\x92\x91\x90aV\x83V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/dW`\0\x80\xFD[PZ\xF1\x15\x80\x15a/xW=`\0\x80>=`\0\xFD[PPPPP[\x8B\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\x11\xF4P\xA0k\xAF\xFB\xE3T\xCA\xF0\xB61\xEAs\x85\xF51\xDB|\x81\xD1F+}\xF8c\x87$\xCD}a\x8D`@Qa/\xBA\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPPV[3`\0\x81\x81Ra\x01_` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a0'W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80`\t\x01\x80Ta07\x90aS\xDBV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x91Pa0oW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x07\x01T\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bTR5`\xE8\x1B\x81RP\x90a0\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA7`\xF0\x1B` \x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0\x84\x10a0\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x07\x81\x01\x83\x90Ua1\x0E`\x01CaVWV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x01a` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x7F\x13\x9A\xD7\xA0\xC3\xF6\xC0\xAD\x0F\x0F\xC5+vx`jq\xFF\x08\x1B\x99&\xA5do\xE5:\x8Eg\xC6\x8Ar\x90a\x15\xC4\x90\x86\x81R` \x01\x90V[`\0a1y`\x01`\0\x80Q` aYb\x839\x81Q\x91RaU}V[a1\x82\x81a6\xBCV[`\0a1\x8E\x87\x87a\x17\xFAV[P\x90P`\x03\x81`\x04\x81\x11\x15a1\xA5Wa1\xA5aP\x9FV[\x14\x80a1\xC2WP`\x04\x81`\x04\x81\x11\x15a1\xC0Wa1\xC0aP\x9FV[\x14[\x80a1\xDEWP`\x02\x81`\x04\x81\x11\x15a1\xDCWa1\xDCaP\x9FV[\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x10M`\xF2\x1B\x81RP\x90a2\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81Ra\x01_` \x90\x81R`@\x80\x83 a\x01`\x83R\x81\x84 \x8A\x85R\x90\x92R\x82 `\x04\x81\x01\x80T\x92\x93\x91\x92\x91a2X\x83aV\xBCV[\x91\x90PUP\x86\x82`\x01\x01`\0\x82\x82Ta2q\x91\x90aU}V[\x92PP\x81\x90UP\x86\x82`\x04\x01`\0\x82\x82Ta2\x8C\x91\x90aU}V[\x90\x91UPP`\x01\x81\x01T`\x03\x83\x01\x80T`\0\x90a2\xAA\x90\x84\x90aU}V[\x90\x91UPa2\xE4\x90P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x89a=.V[P`\x01\x01T\x97\x96PPPPPPPV[3a\x12{\x81\x83a?\xC9V[a3\x07a5\xF6V[3`\0\x81\x81Ra\x01_` R`@\x90 `\t\x81\x01\x80Ta3&\x90aS\xDBV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x91Pa3^W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a3\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x07\x01T\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04s\x13`\xEC\x1B\x81RP\x90a3\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\x01\x01T\x83`\x07\x01Ta4\x0C\x91\x90aUDV[a4\x16\x91\x90aU[V[\x90P`\0\x81\x83`\x01\x01Ta4*\x91\x90aU}V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x90\x91P\x81a4dW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x82`\x04\x01T\x82\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA5`\xF0\x1B\x81RP\x90a4\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[Pa4\xDB`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86\x83a=.V[`\x01\x83\x01\x82\x90Ug\r\xE0\xB6\xB3\xA7d\0\0`\x07\x84\x01U`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x01a` R`@\x90 TC\x10\x80\x15\x90a51WP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x01a` R`@\x90 T\x15\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x11\xCCM`\xEA\x1B\x81RP\x90a5kW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x01a` R`@\x80\x82 \x91\x90\x91UQ\x7F\xE5+<X\xA1\x16\xC1\xF0\x12\xC9\x9D\x11 \xC0T\xE7?Q\xB7\xA2\x9CEq\x92E_\xD2\xC2\x0E\x03\xA53\x90a5\xBA\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPa\x13[`\x01a\x01-UV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x08\xA1WPa\x08\xA1\x82aA\x91V[`\x02a\x01-T\x03a6IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\n\x06V[`\x02a\x01-UV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x0Cz\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91RaA\xC6V[a\x13[\x813aB\x9BV[a\x12{\x82\x82aB\xF4V[a6\xDA\x82\x82aC\x16V[a6\xE4`\0a*\x19V[`\0\x03a\x12{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrCannot be adminless`h\x1B`D\x82\x01R`d\x01a\n\x06V[`\0a\x12{\x81a6\xBCV[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a7gWa\x0F.\x83aC8V[\x82`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a7\xC1WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra7\xBE\x91\x81\x01\x90aVjV[`\x01[a8$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01Rmon is not UUPS`\x90\x1B`d\x82\x01R`\x84\x01a\n\x06V[`\0\x80Q` aYB\x839\x81Q\x91R\x81\x14a8\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01Rh\x1AXX\x9B\x19UURQ`\xBA\x1B`d\x82\x01R`\x84\x01a\n\x06V[Pa\x0F.\x83\x83\x83aC\xD4V[a\x01cT`@Qc\x11\xA2R/`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x8D\x12\x91x\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x0E\x91\x90aV\xD3V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aM1`\xF0\x1B\x81RP\x90a9RW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01`` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x82\x90`\xFF\x16`\x04\x81\x11\x15a9\x99Wa9\x99aP\x9FV[`\x04\x81\x11\x15a9\xAAWa9\xAAaP\x9FV[\x81R`\x01\x82\x01T` \x82\x01R`\x02\x82\x01T`@\x82\x01R`\x03\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x90\x91\x01R\x90P`\0\x81Q`\x04\x81\x11\x15a9\xEDWa9\xEDaP\x9FV[\x14\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG9`\xF0\x1B\x81RP\x90a:(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x80\x81\x01Q`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x13M`\xF2\x1B` \x82\x01R\x90\x15a:fW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\0a\x01_`\0\x85`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x90P\x81` \x01Q\x81`\x02\x01`\0\x82\x82Ta:\xA9\x91\x90aU}V[\x92PP\x81\x90UP`\x01\x81`\x05\x01`\0\x82\x82Ta:\xC5\x91\x90aU}V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x01`` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x80\x82 \x80T`\xFF\x19\x16\x81U`\x01\x81\x01\x83\x90U`\x02\x81\x01\x83\x90U`\x03\x81\x01\x83\x90U`\x04\x01\x82\x90UQ\x85\x92\x91\x7F\x83\x1C\xD5\xB7S\x83\x80\xB0\xA2\xA3\x14\x14\xD64\xECBq\x16\x07V\x84\xA2v\x82\x8A\xB4\xD2ut\xA2\xDF\xDF\x91\xA3PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a;\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\n\x06V[V[a\x12{\x82\x82a6\xC6V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81Ra\x01_` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T\x90\x95\x16\x85R`\x01\x81\x01T\x92\x85\x01\x92\x90\x92R`\x02\x82\x01T\x90\x84\x01R`\x03\x81\x01T``\x84\x01R`\x04\x81\x01T`\x80\x84\x01R`\x05\x81\x01T`\xA0\x84\x01R`\x06\x81\x01T`\xC0\x84\x01R`\x07\x81\x01T`\xE0\x84\x01R`\x08\x81\x01Ta\x01\0\x84\x01R`\t\x81\x01\x80T\x92\x93\x84\x93\x90\x92\x91a\x01 \x84\x01\x91a<Q\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta<}\x90aS\xDBV[\x80\x15a<\xCAW\x80`\x1F\x10a<\x9FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a<\xCAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a<\xADW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82a\x01\0\x01Q\x83`\xC0\x01Qa<\xF5\x91\x90aUDV[a<\xFF\x91\x90aU[V[\x90P\x81``\x01Q\x81\x10\x15a=\x17WP`\0\x93\x92PPPV[``\x82\x01Qa=&\x90\x82aU}V[\x94\x93PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x0F.\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a6\x85V[`\0a\x1E\xFF\x83\x83aC\xF9V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a=\x84\x91\x90aWEV[PP\x95P\x95P\x95PPPPa=\x9A\x83\x83\x83aD#V[\x95\x94PPPPPV[```\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a=\xBC\x91\x90aWEV[PPPPP\x92PPP\x80a=\xCF\x82aD\\V[\x92P\x92PP\x91P\x91V[`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0a>;\x85\x85aD\xA9V[\x91P\x91Pa>H\x81aD\xEBV[P\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81Ra\x01_` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T\x90\x95\x16\x85R`\x01\x81\x01T\x92\x85\x01\x92\x90\x92R`\x02\x82\x01T\x90\x84\x01R`\x03\x81\x01T``\x84\x01R`\x04\x81\x01T`\x80\x84\x01R`\x05\x81\x01T`\xA0\x84\x01R`\x06\x81\x01T`\xC0\x84\x01R`\x07\x81\x01T`\xE0\x84\x01R`\x08\x81\x01Ta\x01\0\x84\x01R`\t\x81\x01\x80T\x92\x93\x84\x93\x90\x92\x91a\x01 \x84\x01\x91a>\xEB\x90aS\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta?\x17\x90aS\xDBV[\x80\x15a?dW\x80`\x1F\x10a?9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a?dV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a?GW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\xE0\x01Q\x83` \x01Qa?\x8E\x91\x90aUDV[a?\x98\x91\x90aU[V[\x90P\x81`\x80\x01Q\x81\x10\x15a?\xB0WP`\0\x93\x92PPPV[`\x80\x82\x01Qa=&\x90\x82aU}V[`\0a\x08\xA1\x82T\x90V[`\0a?\xD5\x83\x83a\x17\xFAV[P\x90P`\0\x81`\x04\x81\x11\x15a?\xECWa?\xECaP\x9FV[\x14\x15\x80\x15a@\x0CWP`\x04\x81`\x04\x81\x11\x15a@\tWa@\taP\x9FV[\x14\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x08\xE7`\xF3\x1B\x81RP\x90a@EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x01`` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x80\x82 \x80T`\xFF\x19\x16`\x04\x17\x81U\x90Q\x90\x92\x85\x92\x90\x91\x7F\x98o\xCB\xE6\xF7\xD2\xB4Q\xF9\x11\\{\xE2.\x18oC\xEB1I\x8C\x85B\xF7\x98!H\xF3\xB5e\xCF\xD3\x91\x90\xA3PPPPV[`\0a@\xB3\x84a=\xA3V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x89\x16` \x82\x01R\x91\x93P`\0\x92P\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a@\xF1\x82a=\xD9V[\x90P`\0aA5\x82\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa>,\x92PPPV[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04\x13\x13`\xEC\x1B\x81RP\x90aA\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[PPPPPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x08\xA1WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x08\xA1V[`\0aB\x1B\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16aF5\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80aB<WP\x80\x80` \x01\x90Q\x81\x01\x90aB<\x91\x90aX:V[a\x0F.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\n\x06V[aB\xA5\x82\x82a\x1F\x06V[a\x12{WaB\xB2\x81aFDV[aB\xBD\x83` aFVV[`@Q` \x01aB\xCE\x92\x91\x90aXWV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\n\x06\x91`\x04\x01aT\x15V[aB\xFE\x82\x82aG\xF2V[`\0\x82\x81R`\x97` R`@\x90 a\x0F.\x90\x82aHxV[aC \x82\x82aH\x8DV[`\0\x82\x81R`\x97` R`@\x90 a\x0F.\x90\x82aH\xF4V[`\x01`\x01`\xA0\x1B\x03\x81\x16;aC\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01a\n\x06V[`\0\x80Q` aYB\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[aC\xDD\x83aI\tV[`\0\x82Q\x11\x80aC\xEAWP\x80[\x15a\x0F.Wa\x0Cz\x83\x83aIIV[`\0\x82`\0\x01\x82\x81T\x81\x10aD\x10WaD\x10aV(V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0\x80\x84\x84\x84`@Q` \x01aD;\x93\x92\x91\x90aX\xCCV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`\0\x81Q`@\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG7`\xF0\x1B\x81RP\x90aD\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[PP\x80Q` \x90\x91\x01 \x90V[`\0\x80\x82Q`A\x03aD\xDFW` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaD\xD3\x87\x82\x85\x85aInV[\x94P\x94PPPPa\r\x1BV[P`\0\x90P`\x02a\r\x1BV[`\0\x81`\x04\x81\x11\x15aD\xFFWaD\xFFaP\x9FV[\x03aE\x07WPV[`\x01\x81`\x04\x81\x11\x15aE\x1BWaE\x1BaP\x9FV[\x03aEhW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\x06V[`\x02\x81`\x04\x81\x11\x15aE|WaE|aP\x9FV[\x03aE\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\n\x06V[`\x03\x81`\x04\x81\x11\x15aE\xDDWaE\xDDaP\x9FV[\x03a\x13[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n\x06V[``a=&\x84\x84`\0\x85aJ2V[``a\x08\xA1`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0aFe\x83`\x02aUDV[aFp\x90`\x02aVWV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF\x88WaF\x88aM\xF2V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aF\xB2W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10aF\xCDWaF\xCDaV(V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10aF\xFCWaF\xFCaV(V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0aG \x84`\x02aUDV[aG+\x90`\x01aVWV[\x90P[`\x01\x81\x11\x15aG\xA3Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10aG_WaG_aV(V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10aGuWaGuaV(V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93aG\x9C\x81aV\xBCV[\x90PaG.V[P\x83\x15a\x1E\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\n\x06V[aG\xFC\x82\x82a\x1F\x06V[a\x12{W`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UaH43\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x1E\xFF\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aK\rV[aH\x97\x82\x82a\x1F\x06V[\x15a\x12{W`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x1E\xFF\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aK\\V[aI\x12\x81aC8V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a\x1E\xFF\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01aY\x82`'\x919aLOV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aI\xA5WP`\0\x90P`\x03aJ)V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aI\xF9W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aJ\"W`\0`\x01\x92P\x92PPaJ)V[\x91P`\0\x90P[\x94P\x94\x92PPPV[``\x82G\x10\x15aJ\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\n\x06V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@QaJ\xAF\x91\x90aY\x0FV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14aJ\xECW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aJ\xF1V[``\x91P[P\x91P\x91PaK\x02\x87\x83\x83\x87aL\xC7V[\x97\x96PPPPPPPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 TaKTWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x08\xA1V[P`\0a\x08\xA1V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aLEW`\0aK\x80`\x01\x83aU}V[\x85T\x90\x91P`\0\x90aK\x94\x90`\x01\x90aU}V[\x90P\x81\x81\x14aK\xF9W`\0\x86`\0\x01\x82\x81T\x81\x10aK\xB4WaK\xB4aV(V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10aK\xD7WaK\xD7aV(V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80aL\nWaL\naY+V[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x08\xA1V[`\0\x91PPa\x08\xA1V[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@QaLl\x91\x90aY\x0FV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14aL\xA7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aL\xACV[``\x91P[P\x91P\x91PaL\xBD\x86\x83\x83\x87aL\xC7V[\x96\x95PPPPPPV[``\x83\x15aM6W\x82Q`\0\x03aM/W`\x01`\x01`\xA0\x1B\x03\x85\x16;aM/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\n\x06V[P\x81a=&V[a=&\x83\x83\x81Q\x15aMKW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x06\x91\x90aT\x15V[P\x80TaMq\x90aS\xDBV[`\0\x82U\x80`\x1F\x10aM\x81WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x13[\x91\x90[\x80\x82\x11\x15aM\xAFW`\0\x81U`\x01\x01aM\x9BV[P\x90V[`\0` \x82\x84\x03\x12\x15aM\xC5W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1E\xFFW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13[W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aN1WaN1aM\xF2V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aNSWaNSaM\xF2V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aNrW`\0\x80\xFD[\x815aN\x85aN\x80\x82aN9V[aN\x08V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aN\x9AW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aN\xCDW`\0\x80\xFD[\x845aN\xD8\x81aM\xDDV[\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aO\x02W`\0\x80\xFD[aO\x0E\x87\x82\x88\x01aNaV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15aO-W`\0\x80\xFD[\x825aO8\x81aM\xDDV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15aOXW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aOrW`\0\x80\xFD[\x825\x91P` \x83\x015aO\x84\x81aM\xDDV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15aO\xA1W`\0\x80\xFD[\x815a\x1E\xFF\x81aM\xDDV[`\0\x80` \x83\x85\x03\x12\x15aO\xBFW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aO\xD7W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aO\xEBW`\0\x80\xFD[\x815\x81\x81\x11\x15aO\xFAW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15aP\x0FW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15aP4W`\0\x80\xFD[\x825aP?\x81aM\xDDV[\x91P` \x83\x015aO\x84\x81aM\xDDV[`\0\x80`@\x83\x85\x03\x12\x15aPbW`\0\x80\xFD[\x825aPm\x81aM\xDDV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aP\x89W`\0\x80\xFD[aP\x95\x85\x82\x86\x01aNaV[\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x05\x81\x10aP\xD3WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`@\x81\x01aP\xE5\x82\x85aP\xB5V[\x82` \x83\x01R\x93\x92PPPV[`\0[\x83\x81\x10\x15aQ\rW\x81\x81\x01Q\x83\x82\x01R` \x01aP\xF5V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84RaQ.\x81` \x86\x01` \x86\x01aP\xF2V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0a\x01@`\x01\x80`\xA0\x1B\x03\x8D\x16\x83R\x8B` \x84\x01R\x8A`@\x84\x01R\x89``\x84\x01R\x88`\x80\x84\x01R\x87`\xA0\x84\x01R\x86`\xC0\x84\x01R\x85`\xE0\x84\x01R\x84a\x01\0\x84\x01R\x80a\x01 \x84\x01RaQ\x96\x81\x84\x01\x85aQ\x16V[\x9D\x9CPPPPPPPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aQ\xBAW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80\x83`\x1F\x84\x01\x12aQ\xDBW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aQ\xF3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\r\x1BW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15aR!W`\0\x80\xFD[\x845\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aR@W`\0\x80\xFD[aRL\x88\x83\x89\x01aNaV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15aRbW`\0\x80\xFD[PaRo\x87\x82\x88\x01aQ\xC9V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aR\x90W`\0\x80\xFD[\x835aR\x9B\x81aM\xDDV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\xA0\x81\x01aR\xBE\x82\x88aP\xB5V[\x85` \x83\x01R\x84`@\x83\x01R\x83``\x83\x01R\x82`\x80\x83\x01R\x96\x95PPPPPPV[\x80\x15\x15\x81\x14a\x13[W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x89\x8B\x03\x12\x15aS\nW`\0\x80\xFD[\x885\x97P` \x89\x015\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015aS1\x81aR\xE0V[\x93P`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aSNW`\0\x80\xFD[aSZ\x8C\x83\x8D\x01aNaV[\x94P`\xC0\x8B\x015\x91P\x80\x82\x11\x15aSpW`\0\x80\xFD[PaS}\x8B\x82\x8C\x01aQ\xC9V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aS\xA7W`\0\x80\xFD[\x845aS\xB2\x81aM\xDDV[\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015aS\xD0\x81aM\xDDV[\x93\x96\x92\x95P\x90\x93PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80aS\xEFW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aT\x0FWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x81R`\0a\x1E\xFF` \x83\x01\x84aQ\x16V[`\x1F\x82\x11\x15a\x0F.W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15aTOWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x17\xF2W\x82\x81U`\x01\x01aT[V[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aT\x88WaT\x88aM\xF2V[aT\x9C\x81aT\x96\x84TaS\xDBV[\x84aT(V[` \x80`\x1F\x83\x11`\x01\x81\x14aT\xD1W`\0\x84\x15aT\xB9WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x17\xF2V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aU\0W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aT\xE1V[P\x85\x82\x10\x15aU\x1EW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x08\xA1Wa\x08\xA1aU.V[`\0\x82aUxWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x08\xA1Wa\x08\xA1aU.V[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rk\x19\x19[\x19Y\xD8]\x19X\xD8[\x1B`\xA2\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rkactive proxy`\xA0\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01aVPWaVPaU.V[P`\x01\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x08\xA1Wa\x08\xA1aU.V[`\0` \x82\x84\x03\x12\x15aV|W`\0\x80\xFD[PQ\x91\x90PV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0aV\xAA`\x80\x83\x01\x85aQ\x16V[\x82\x81\x03``\x84\x01RaK\x02\x81\x85aQ\x16V[`\0\x81aV\xCBWaV\xCBaU.V[P`\0\x19\x01\x90V[`\0` \x82\x84\x03\x12\x15aV\xE5W`\0\x80\xFD[\x81Qa\x1E\xFF\x81aM\xDDV[`\0\x82`\x1F\x83\x01\x12aW\x01W`\0\x80\xFD[\x81QaW\x0FaN\x80\x82aN9V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aW$W`\0\x80\xFD[a=&\x82` \x83\x01` \x87\x01aP\xF2V[\x80QaW@\x81aM\xDDV[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15aWbW`\0\x80\xFD[\x88Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aWzW`\0\x80\xFD[aW\x86\x8C\x83\x8D\x01aV\xF0V[\x99PaW\x94` \x8C\x01aW5V[\x98P`@\x8B\x01Q\x91P\x80\x82\x11\x15aW\xAAW`\0\x80\xFD[aW\xB6\x8C\x83\x8D\x01aV\xF0V[\x97P``\x8B\x01Q\x91P\x80\x82\x11\x15aW\xCCW`\0\x80\xFD[aW\xD8\x8C\x83\x8D\x01aV\xF0V[\x96P`\x80\x8B\x01Q\x91P\x80\x82\x11\x15aW\xEEW`\0\x80\xFD[aW\xFA\x8C\x83\x8D\x01aV\xF0V[\x95P`\xA0\x8B\x01Q\x91P\x80\x82\x11\x15aX\x10W`\0\x80\xFD[PaX\x1D\x8B\x82\x8C\x01aV\xF0V[`\xC0\x8B\x01Q`\xE0\x90\x9B\x01Q\x99\x9C\x98\x9BP\x96\x99\x95\x98\x94\x97\x93PPPPV[`\0` \x82\x84\x03\x12\x15aXLW`\0\x80\xFD[\x81Qa\x1E\xFF\x81aR\xE0V[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83QaX\x8F\x81`\x17\x85\x01` \x88\x01aP\xF2V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83QaX\xC0\x81`(\x84\x01` \x88\x01aP\xF2V[\x01`(\x01\x94\x93PPPPV[`\0\x84QaX\xDE\x81\x84` \x89\x01aP\xF2V[\x84Q\x90\x83\x01\x90aX\xF2\x81\x83` \x89\x01aP\xF2V[\x84Q\x91\x01\x90aY\x05\x81\x83` \x88\x01aP\xF2V[\x01\x95\x94PPPPPV[`\0\x82QaY!\x81\x84` \x87\x01aP\xF2V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x12\xB4.\x8A\x16\x0F`d\xDC\x95\x9Co%\x1E:\xF0u\n\xD2\x13\xDB\xEC\xF5s\xB4q\rg\xD6\xC2\x8E9Address: low-level delegate call failed\xA2dipfsX\"\x12 ^p\xB9\x11f\xB1U[\xCE\xDA\xC1\xE4D\x1D\x88\x94\xDF\xD70\x81:/|M\x96\xD4\x1AJ3\x9E\xED\x82dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static GENERATORREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GeneratorRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GeneratorRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GeneratorRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GeneratorRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GeneratorRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GeneratorRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GeneratorRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GENERATORREGISTRY_ABI.clone(),
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
                GENERATORREGISTRY_ABI.clone(),
                GENERATORREGISTRY_BYTECODE.clone().into(),
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
        ///Calls the contract's `ENTITY_KEY_REGISTRY` (0x661de5ac) function
        pub fn entity_key_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([102, 29, 229, 172], ())
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
        ///Calls the contract's `PARALLEL_REQUESTS_UPPER_LIMIT` (0x7a14c463) function
        pub fn parallel_requests_upper_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([122, 20, 196, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SLASHER_ROLE` (0x5095af64) function
        pub fn slasher_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([80, 149, 175, 100], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STAKING_TOKEN` (0x0479d644) function
        pub fn staking_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([4, 121, 214, 68], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UNLOCK_WAIT_BLOCKS` (0x3c5eb57c) function
        pub fn unlock_wait_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([60, 94, 181, 124], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assignGeneratorTask` (0xc492ee39) function
        pub fn assign_generator_task(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
            stake_to_lock: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [196, 146, 238, 57],
                    (generator_address, market_id, stake_to_lock),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeRewardAddress` (0x4d2aab9a) function
        pub fn change_reward_address(
            &self,
            new_reward_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 42, 171, 154], new_reward_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `completeGeneratorTask` (0x982a415d) function
        pub fn complete_generator_task(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
            stake_to_release: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [152, 42, 65, 93],
                    (generator_address, market_id, stake_to_release),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseDeclaredCompute` (0x2f8f4a3b) function
        pub fn decrease_declared_compute(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 143, 74, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregister` (0x84ac33ec) function
        pub fn deregister(
            &self,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 172, 51, 236], refund_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `generatorInfoPerMarket` (0x9a7fca8e) function
        pub fn generator_info_per_market(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u8,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([154, 127, 202, 142], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `generatorRegistry` (0x8cfc56d8) function
        pub fn generator_registry(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([140, 252, 86, 216], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGeneratorAssignmentDetails` (0x1c7eae65) function
        pub fn get_generator_assignment_details(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([28, 126, 174, 101], (generator_address, market_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGeneratorRewardDetails` (0x2b610c2d) function
        pub fn get_generator_reward_details(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([43, 97, 12, 45], (generator_address, market_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGeneratorState` (0x646d51b4) function
        pub fn get_generator_state(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u8, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([100, 109, 81, 180], (generator_address, market_id))
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
        ///Calls the contract's `increaseDeclaredCompute` (0x6d405877) function
        pub fn increase_declared_compute(
            &self,
            compute_to_increase: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 64, 88, 119], compute_to_increase)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x485cc955) function
        pub fn initialize(
            &self,
            admin: ::ethers::core::types::Address,
            proof_market_place: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (admin, proof_market_place))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intendToReduceCompute` (0x96de0eef) function
        pub fn intend_to_reduce_compute(
            &self,
            new_utilization: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 222, 14, 239], new_utilization)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intendToReduceStake` (0xe9e934a0) function
        pub fn intend_to_reduce_stake(
            &self,
            new_utilization: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 233, 52, 160], new_utilization)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `joinMarketPlace` (0xe85210bd) function
        pub fn join_market_place(
            &self,
            market_id: ::ethers::core::types::U256,
            compute_per_request_required: ::ethers::core::types::U256,
            proof_generation_cost: ::ethers::core::types::U256,
            proposed_time: ::ethers::core::types::U256,
            update_market_dedicated_key: bool,
            attestation_data: ::ethers::core::types::Bytes,
            enclave_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [232, 82, 16, 189],
                    (
                        market_id,
                        compute_per_request_required,
                        proof_generation_cost,
                        proposed_time,
                        update_market_dedicated_key,
                        attestation_data,
                        enclave_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `leaveMarketPlace` (0xa64b5213) function
        pub fn leave_market_place(
            &self,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 75, 82, 19], market_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `leaveMarketPlaces` (0x4615e8bc) function
        pub fn leave_market_places(
            &self,
            market_ids: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 21, 232, 188], market_ids)
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
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `register` (0x136dfbf5) function
        pub fn register(
            &self,
            reward_address: ::ethers::core::types::Address,
            declared_compute: ::ethers::core::types::U256,
            initial_stake: ::ethers::core::types::U256,
            generator_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [19, 109, 251, 245],
                    (reward_address, declared_compute, initial_stake, generator_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeEncryptionKey` (0x541a8c18) function
        pub fn remove_encryption_key(
            &self,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([84, 26, 140, 24], market_id)
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
        ///Calls the contract's `requestForExitMarketPlace` (0xee932863) function
        pub fn request_for_exit_market_place(
            &self,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 147, 40, 99], market_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestForExitMarketPlaces` (0xd56139b6) function
        pub fn request_for_exit_market_places(
            &self,
            market_ids: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 97, 57, 182], market_ids)
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
        ///Calls the contract's `slashGenerator` (0xeaacae94) function
        pub fn slash_generator(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
            slashing_amount: ::ethers::core::types::U256,
            reward_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [234, 172, 174, 148],
                    (generator_address, market_id, slashing_amount, reward_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stake` (0xadc9772e) function
        pub fn stake(
            &self,
            generator_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 201, 119, 46], (generator_address, amount))
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
        ///Calls the contract's `unstake` (0xf2888dbb) function
        pub fn unstake(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 136, 141, 187], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateEncryptionKey` (0x92eb91e2) function
        pub fn update_encryption_key(
            &self,
            market_id: ::ethers::core::types::U256,
            attestation_data: ::ethers::core::types::Bytes,
            enclave_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [146, 235, 145, 226],
                    (market_id, attestation_data, enclave_signature),
                )
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
        ///Gets the contract's `AddedStake` event
        pub fn added_stake_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddedStakeFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `ChangedGeneratorRewardAddress` event
        pub fn changed_generator_reward_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedGeneratorRewardAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DecreaseCompute` event
        pub fn decrease_compute_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DecreaseComputeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DeregisteredGenerator` event
        pub fn deregistered_generator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeregisteredGeneratorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `IncreasedCompute` event
        pub fn increased_compute_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IncreasedComputeFilter,
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
        ///Gets the contract's `JoinedMarketPlace` event
        pub fn joined_market_place_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            JoinedMarketPlaceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LeftMarketplace` event
        pub fn left_marketplace_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LeftMarketplaceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RegisteredGenerator` event
        pub fn registered_generator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RegisteredGeneratorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RemovedStake` event
        pub fn removed_stake_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemovedStakeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RequestComputeDecrease` event
        pub fn request_compute_decrease_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RequestComputeDecreaseFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RequestExitMarketPlace` event
        pub fn request_exit_market_place_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RequestExitMarketPlaceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RequestStakeDecrease` event
        pub fn request_stake_decrease_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RequestStakeDecreaseFilter,
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GeneratorRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GeneratorRegistry<M> {
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
    #[ethevent(name = "AddedStake", abi = "AddedStake(address,uint256)")]
    pub struct AddedStakeFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        name = "ChangedGeneratorRewardAddress",
        abi = "ChangedGeneratorRewardAddress(address,address)"
    )]
    pub struct ChangedGeneratorRewardAddressFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub new_reward_address: ::ethers::core::types::Address,
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
    #[ethevent(name = "DecreaseCompute", abi = "DecreaseCompute(address,uint256)")]
    pub struct DecreaseComputeFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub compute: ::ethers::core::types::U256,
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
    #[ethevent(name = "DeregisteredGenerator", abi = "DeregisteredGenerator(address)")]
    pub struct DeregisteredGeneratorFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
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
    #[ethevent(name = "IncreasedCompute", abi = "IncreasedCompute(address,uint256)")]
    pub struct IncreasedComputeFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub compute: ::ethers::core::types::U256,
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
        name = "JoinedMarketPlace",
        abi = "JoinedMarketPlace(address,uint256,uint256)"
    )]
    pub struct JoinedMarketPlaceFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
        pub compute_allocation: ::ethers::core::types::U256,
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
    #[ethevent(name = "LeftMarketplace", abi = "LeftMarketplace(address,uint256)")]
    pub struct LeftMarketplaceFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
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
        name = "RegisteredGenerator",
        abi = "RegisteredGenerator(address,uint256,uint256)"
    )]
    pub struct RegisteredGeneratorFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub initial_compute: ::ethers::core::types::U256,
        pub initial_stake: ::ethers::core::types::U256,
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
    #[ethevent(name = "RemovedStake", abi = "RemovedStake(address,uint256)")]
    pub struct RemovedStakeFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        name = "RequestComputeDecrease",
        abi = "RequestComputeDecrease(address,uint256)"
    )]
    pub struct RequestComputeDecreaseFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub intended_utilization: ::ethers::core::types::U256,
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
        name = "RequestExitMarketPlace",
        abi = "RequestExitMarketPlace(address,uint256)"
    )]
    pub struct RequestExitMarketPlaceFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
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
        name = "RequestStakeDecrease",
        abi = "RequestStakeDecrease(address,uint256)"
    )]
    pub struct RequestStakeDecreaseFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub intended_utilization: ::ethers::core::types::U256,
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
    pub enum GeneratorRegistryEvents {
        AddedStakeFilter(AddedStakeFilter),
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        ChangedGeneratorRewardAddressFilter(ChangedGeneratorRewardAddressFilter),
        DecreaseComputeFilter(DecreaseComputeFilter),
        DeregisteredGeneratorFilter(DeregisteredGeneratorFilter),
        IncreasedComputeFilter(IncreasedComputeFilter),
        InitializedFilter(InitializedFilter),
        JoinedMarketPlaceFilter(JoinedMarketPlaceFilter),
        LeftMarketplaceFilter(LeftMarketplaceFilter),
        RegisteredGeneratorFilter(RegisteredGeneratorFilter),
        RemovedStakeFilter(RemovedStakeFilter),
        RequestComputeDecreaseFilter(RequestComputeDecreaseFilter),
        RequestExitMarketPlaceFilter(RequestExitMarketPlaceFilter),
        RequestStakeDecreaseFilter(RequestStakeDecreaseFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for GeneratorRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddedStakeFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::AddedStakeFilter(decoded));
            }
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = ChangedGeneratorRewardAddressFilter::decode_log(log) {
                return Ok(
                    GeneratorRegistryEvents::ChangedGeneratorRewardAddressFilter(decoded),
                );
            }
            if let Ok(decoded) = DecreaseComputeFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::DecreaseComputeFilter(decoded));
            }
            if let Ok(decoded) = DeregisteredGeneratorFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::DeregisteredGeneratorFilter(decoded));
            }
            if let Ok(decoded) = IncreasedComputeFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::IncreasedComputeFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = JoinedMarketPlaceFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::JoinedMarketPlaceFilter(decoded));
            }
            if let Ok(decoded) = LeftMarketplaceFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::LeftMarketplaceFilter(decoded));
            }
            if let Ok(decoded) = RegisteredGeneratorFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RegisteredGeneratorFilter(decoded));
            }
            if let Ok(decoded) = RemovedStakeFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RemovedStakeFilter(decoded));
            }
            if let Ok(decoded) = RequestComputeDecreaseFilter::decode_log(log) {
                return Ok(
                    GeneratorRegistryEvents::RequestComputeDecreaseFilter(decoded),
                );
            }
            if let Ok(decoded) = RequestExitMarketPlaceFilter::decode_log(log) {
                return Ok(
                    GeneratorRegistryEvents::RequestExitMarketPlaceFilter(decoded),
                );
            }
            if let Ok(decoded) = RequestStakeDecreaseFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RequestStakeDecreaseFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GeneratorRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddedStakeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeaconUpgradedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangedGeneratorRewardAddressFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DecreaseComputeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeregisteredGeneratorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncreasedComputeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::JoinedMarketPlaceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LeftMarketplaceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisteredGeneratorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemovedStakeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestComputeDecreaseFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestExitMarketPlaceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestStakeDecreaseFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddedStakeFilter> for GeneratorRegistryEvents {
        fn from(value: AddedStakeFilter) -> Self {
            Self::AddedStakeFilter(value)
        }
    }
    impl ::core::convert::From<AdminChangedFilter> for GeneratorRegistryEvents {
        fn from(value: AdminChangedFilter) -> Self {
            Self::AdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<BeaconUpgradedFilter> for GeneratorRegistryEvents {
        fn from(value: BeaconUpgradedFilter) -> Self {
            Self::BeaconUpgradedFilter(value)
        }
    }
    impl ::core::convert::From<ChangedGeneratorRewardAddressFilter>
    for GeneratorRegistryEvents {
        fn from(value: ChangedGeneratorRewardAddressFilter) -> Self {
            Self::ChangedGeneratorRewardAddressFilter(value)
        }
    }
    impl ::core::convert::From<DecreaseComputeFilter> for GeneratorRegistryEvents {
        fn from(value: DecreaseComputeFilter) -> Self {
            Self::DecreaseComputeFilter(value)
        }
    }
    impl ::core::convert::From<DeregisteredGeneratorFilter> for GeneratorRegistryEvents {
        fn from(value: DeregisteredGeneratorFilter) -> Self {
            Self::DeregisteredGeneratorFilter(value)
        }
    }
    impl ::core::convert::From<IncreasedComputeFilter> for GeneratorRegistryEvents {
        fn from(value: IncreasedComputeFilter) -> Self {
            Self::IncreasedComputeFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for GeneratorRegistryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<JoinedMarketPlaceFilter> for GeneratorRegistryEvents {
        fn from(value: JoinedMarketPlaceFilter) -> Self {
            Self::JoinedMarketPlaceFilter(value)
        }
    }
    impl ::core::convert::From<LeftMarketplaceFilter> for GeneratorRegistryEvents {
        fn from(value: LeftMarketplaceFilter) -> Self {
            Self::LeftMarketplaceFilter(value)
        }
    }
    impl ::core::convert::From<RegisteredGeneratorFilter> for GeneratorRegistryEvents {
        fn from(value: RegisteredGeneratorFilter) -> Self {
            Self::RegisteredGeneratorFilter(value)
        }
    }
    impl ::core::convert::From<RemovedStakeFilter> for GeneratorRegistryEvents {
        fn from(value: RemovedStakeFilter) -> Self {
            Self::RemovedStakeFilter(value)
        }
    }
    impl ::core::convert::From<RequestComputeDecreaseFilter>
    for GeneratorRegistryEvents {
        fn from(value: RequestComputeDecreaseFilter) -> Self {
            Self::RequestComputeDecreaseFilter(value)
        }
    }
    impl ::core::convert::From<RequestExitMarketPlaceFilter>
    for GeneratorRegistryEvents {
        fn from(value: RequestExitMarketPlaceFilter) -> Self {
            Self::RequestExitMarketPlaceFilter(value)
        }
    }
    impl ::core::convert::From<RequestStakeDecreaseFilter> for GeneratorRegistryEvents {
        fn from(value: RequestStakeDecreaseFilter) -> Self {
            Self::RequestStakeDecreaseFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for GeneratorRegistryEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for GeneratorRegistryEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for GeneratorRegistryEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for GeneratorRegistryEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
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
    ///Container type for all input parameters for the `ENTITY_KEY_REGISTRY` function with signature `ENTITY_KEY_REGISTRY()` and selector `0x661de5ac`
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
    #[ethcall(name = "ENTITY_KEY_REGISTRY", abi = "ENTITY_KEY_REGISTRY()")]
    pub struct EntityKeyRegistryCall;
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
    ///Container type for all input parameters for the `PARALLEL_REQUESTS_UPPER_LIMIT` function with signature `PARALLEL_REQUESTS_UPPER_LIMIT()` and selector `0x7a14c463`
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
        name = "PARALLEL_REQUESTS_UPPER_LIMIT",
        abi = "PARALLEL_REQUESTS_UPPER_LIMIT()"
    )]
    pub struct ParallelRequestsUpperLimitCall;
    ///Container type for all input parameters for the `SLASHER_ROLE` function with signature `SLASHER_ROLE()` and selector `0x5095af64`
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
    #[ethcall(name = "SLASHER_ROLE", abi = "SLASHER_ROLE()")]
    pub struct SlasherRoleCall;
    ///Container type for all input parameters for the `STAKING_TOKEN` function with signature `STAKING_TOKEN()` and selector `0x0479d644`
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
    #[ethcall(name = "STAKING_TOKEN", abi = "STAKING_TOKEN()")]
    pub struct StakingTokenCall;
    ///Container type for all input parameters for the `UNLOCK_WAIT_BLOCKS` function with signature `UNLOCK_WAIT_BLOCKS()` and selector `0x3c5eb57c`
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
    #[ethcall(name = "UNLOCK_WAIT_BLOCKS", abi = "UNLOCK_WAIT_BLOCKS()")]
    pub struct UnlockWaitBlocksCall;
    ///Container type for all input parameters for the `assignGeneratorTask` function with signature `assignGeneratorTask(address,uint256,uint256)` and selector `0xc492ee39`
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
        name = "assignGeneratorTask",
        abi = "assignGeneratorTask(address,uint256,uint256)"
    )]
    pub struct AssignGeneratorTaskCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
        pub stake_to_lock: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `changeRewardAddress` function with signature `changeRewardAddress(address)` and selector `0x4d2aab9a`
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
    #[ethcall(name = "changeRewardAddress", abi = "changeRewardAddress(address)")]
    pub struct ChangeRewardAddressCall {
        pub new_reward_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `completeGeneratorTask` function with signature `completeGeneratorTask(address,uint256,uint256)` and selector `0x982a415d`
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
        name = "completeGeneratorTask",
        abi = "completeGeneratorTask(address,uint256,uint256)"
    )]
    pub struct CompleteGeneratorTaskCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
        pub stake_to_release: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decreaseDeclaredCompute` function with signature `decreaseDeclaredCompute()` and selector `0x2f8f4a3b`
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
    #[ethcall(name = "decreaseDeclaredCompute", abi = "decreaseDeclaredCompute()")]
    pub struct DecreaseDeclaredComputeCall;
    ///Container type for all input parameters for the `deregister` function with signature `deregister(address)` and selector `0x84ac33ec`
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
    #[ethcall(name = "deregister", abi = "deregister(address)")]
    pub struct DeregisterCall {
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `generatorInfoPerMarket` function with signature `generatorInfoPerMarket(address,uint256)` and selector `0x9a7fca8e`
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
        name = "generatorInfoPerMarket",
        abi = "generatorInfoPerMarket(address,uint256)"
    )]
    pub struct GeneratorInfoPerMarketCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `generatorRegistry` function with signature `generatorRegistry(address)` and selector `0x8cfc56d8`
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
    #[ethcall(name = "generatorRegistry", abi = "generatorRegistry(address)")]
    pub struct GeneratorRegistryCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getGeneratorAssignmentDetails` function with signature `getGeneratorAssignmentDetails(address,uint256)` and selector `0x1c7eae65`
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
        name = "getGeneratorAssignmentDetails",
        abi = "getGeneratorAssignmentDetails(address,uint256)"
    )]
    pub struct GetGeneratorAssignmentDetailsCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getGeneratorRewardDetails` function with signature `getGeneratorRewardDetails(address,uint256)` and selector `0x2b610c2d`
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
        name = "getGeneratorRewardDetails",
        abi = "getGeneratorRewardDetails(address,uint256)"
    )]
    pub struct GetGeneratorRewardDetailsCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getGeneratorState` function with signature `getGeneratorState(address,uint256)` and selector `0x646d51b4`
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
    #[ethcall(name = "getGeneratorState", abi = "getGeneratorState(address,uint256)")]
    pub struct GetGeneratorStateCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `increaseDeclaredCompute` function with signature `increaseDeclaredCompute(uint256)` and selector `0x6d405877`
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
        name = "increaseDeclaredCompute",
        abi = "increaseDeclaredCompute(uint256)"
    )]
    pub struct IncreaseDeclaredComputeCall {
        pub compute_to_increase: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `0x485cc955`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub admin: ::ethers::core::types::Address,
        pub proof_market_place: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `intendToReduceCompute` function with signature `intendToReduceCompute(uint256)` and selector `0x96de0eef`
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
    #[ethcall(name = "intendToReduceCompute", abi = "intendToReduceCompute(uint256)")]
    pub struct IntendToReduceComputeCall {
        pub new_utilization: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `intendToReduceStake` function with signature `intendToReduceStake(uint256)` and selector `0xe9e934a0`
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
    #[ethcall(name = "intendToReduceStake", abi = "intendToReduceStake(uint256)")]
    pub struct IntendToReduceStakeCall {
        pub new_utilization: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `joinMarketPlace` function with signature `joinMarketPlace(uint256,uint256,uint256,uint256,bool,bytes,bytes)` and selector `0xe85210bd`
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
        name = "joinMarketPlace",
        abi = "joinMarketPlace(uint256,uint256,uint256,uint256,bool,bytes,bytes)"
    )]
    pub struct JoinMarketPlaceCall {
        pub market_id: ::ethers::core::types::U256,
        pub compute_per_request_required: ::ethers::core::types::U256,
        pub proof_generation_cost: ::ethers::core::types::U256,
        pub proposed_time: ::ethers::core::types::U256,
        pub update_market_dedicated_key: bool,
        pub attestation_data: ::ethers::core::types::Bytes,
        pub enclave_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `leaveMarketPlace` function with signature `leaveMarketPlace(uint256)` and selector `0xa64b5213`
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
    #[ethcall(name = "leaveMarketPlace", abi = "leaveMarketPlace(uint256)")]
    pub struct LeaveMarketPlaceCall {
        pub market_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `leaveMarketPlaces` function with signature `leaveMarketPlaces(uint256[])` and selector `0x4615e8bc`
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
    #[ethcall(name = "leaveMarketPlaces", abi = "leaveMarketPlaces(uint256[])")]
    pub struct LeaveMarketPlacesCall {
        pub market_ids: ::std::vec::Vec<::ethers::core::types::U256>,
    }
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
    ///Container type for all input parameters for the `register` function with signature `register(address,uint256,uint256,bytes)` and selector `0x136dfbf5`
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
    #[ethcall(name = "register", abi = "register(address,uint256,uint256,bytes)")]
    pub struct RegisterCall {
        pub reward_address: ::ethers::core::types::Address,
        pub declared_compute: ::ethers::core::types::U256,
        pub initial_stake: ::ethers::core::types::U256,
        pub generator_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `removeEncryptionKey` function with signature `removeEncryptionKey(uint256)` and selector `0x541a8c18`
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
    #[ethcall(name = "removeEncryptionKey", abi = "removeEncryptionKey(uint256)")]
    pub struct RemoveEncryptionKeyCall {
        pub market_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `requestForExitMarketPlace` function with signature `requestForExitMarketPlace(uint256)` and selector `0xee932863`
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
        name = "requestForExitMarketPlace",
        abi = "requestForExitMarketPlace(uint256)"
    )]
    pub struct RequestForExitMarketPlaceCall {
        pub market_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `requestForExitMarketPlaces` function with signature `requestForExitMarketPlaces(uint256[])` and selector `0xd56139b6`
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
        name = "requestForExitMarketPlaces",
        abi = "requestForExitMarketPlaces(uint256[])"
    )]
    pub struct RequestForExitMarketPlacesCall {
        pub market_ids: ::std::vec::Vec<::ethers::core::types::U256>,
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
    ///Container type for all input parameters for the `slashGenerator` function with signature `slashGenerator(address,uint256,uint256,address)` and selector `0xeaacae94`
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
        name = "slashGenerator",
        abi = "slashGenerator(address,uint256,uint256,address)"
    )]
    pub struct SlashGeneratorCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
        pub slashing_amount: ::ethers::core::types::U256,
        pub reward_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `stake` function with signature `stake(address,uint256)` and selector `0xadc9772e`
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
    #[ethcall(name = "stake", abi = "stake(address,uint256)")]
    pub struct StakeCall {
        pub generator_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `unstake` function with signature `unstake(address)` and selector `0xf2888dbb`
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
    #[ethcall(name = "unstake", abi = "unstake(address)")]
    pub struct UnstakeCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateEncryptionKey` function with signature `updateEncryptionKey(uint256,bytes,bytes)` and selector `0x92eb91e2`
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
        name = "updateEncryptionKey",
        abi = "updateEncryptionKey(uint256,bytes,bytes)"
    )]
    pub struct UpdateEncryptionKeyCall {
        pub market_id: ::ethers::core::types::U256,
        pub attestation_data: ::ethers::core::types::Bytes,
        pub enclave_signature: ::ethers::core::types::Bytes,
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
    pub enum GeneratorRegistryCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        EntityKeyRegistry(EntityKeyRegistryCall),
        KeyRegisterRole(KeyRegisterRoleCall),
        ParallelRequestsUpperLimit(ParallelRequestsUpperLimitCall),
        SlasherRole(SlasherRoleCall),
        StakingToken(StakingTokenCall),
        UnlockWaitBlocks(UnlockWaitBlocksCall),
        AssignGeneratorTask(AssignGeneratorTaskCall),
        ChangeRewardAddress(ChangeRewardAddressCall),
        CompleteGeneratorTask(CompleteGeneratorTaskCall),
        DecreaseDeclaredCompute(DecreaseDeclaredComputeCall),
        Deregister(DeregisterCall),
        GeneratorInfoPerMarket(GeneratorInfoPerMarketCall),
        GeneratorRegistry(GeneratorRegistryCall),
        GetGeneratorAssignmentDetails(GetGeneratorAssignmentDetailsCall),
        GetGeneratorRewardDetails(GetGeneratorRewardDetailsCall),
        GetGeneratorState(GetGeneratorStateCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        IncreaseDeclaredCompute(IncreaseDeclaredComputeCall),
        Initialize(InitializeCall),
        IntendToReduceCompute(IntendToReduceComputeCall),
        IntendToReduceStake(IntendToReduceStakeCall),
        JoinMarketPlace(JoinMarketPlaceCall),
        LeaveMarketPlace(LeaveMarketPlaceCall),
        LeaveMarketPlaces(LeaveMarketPlacesCall),
        ProofMarketPlace(ProofMarketPlaceCall),
        ProxiableUUID(ProxiableUUIDCall),
        Register(RegisterCall),
        RemoveEncryptionKey(RemoveEncryptionKeyCall),
        RenounceRole(RenounceRoleCall),
        RequestForExitMarketPlace(RequestForExitMarketPlaceCall),
        RequestForExitMarketPlaces(RequestForExitMarketPlacesCall),
        RevokeRole(RevokeRoleCall),
        SlashGenerator(SlashGeneratorCall),
        Stake(StakeCall),
        SupportsInterface(SupportsInterfaceCall),
        Unstake(UnstakeCall),
        UpdateEncryptionKey(UpdateEncryptionKeyCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ::ethers::core::abi::AbiDecode for GeneratorRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <EntityKeyRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EntityKeyRegistry(decoded));
            }
            if let Ok(decoded) = <KeyRegisterRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::KeyRegisterRole(decoded));
            }
            if let Ok(decoded) = <ParallelRequestsUpperLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParallelRequestsUpperLimit(decoded));
            }
            if let Ok(decoded) = <SlasherRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SlasherRole(decoded));
            }
            if let Ok(decoded) = <StakingTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakingToken(decoded));
            }
            if let Ok(decoded) = <UnlockWaitBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnlockWaitBlocks(decoded));
            }
            if let Ok(decoded) = <AssignGeneratorTaskCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssignGeneratorTask(decoded));
            }
            if let Ok(decoded) = <ChangeRewardAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeRewardAddress(decoded));
            }
            if let Ok(decoded) = <CompleteGeneratorTaskCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CompleteGeneratorTask(decoded));
            }
            if let Ok(decoded) = <DecreaseDeclaredComputeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DecreaseDeclaredCompute(decoded));
            }
            if let Ok(decoded) = <DeregisterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deregister(decoded));
            }
            if let Ok(decoded) = <GeneratorInfoPerMarketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GeneratorInfoPerMarket(decoded));
            }
            if let Ok(decoded) = <GeneratorRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GeneratorRegistry(decoded));
            }
            if let Ok(decoded) = <GetGeneratorAssignmentDetailsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetGeneratorAssignmentDetails(decoded));
            }
            if let Ok(decoded) = <GetGeneratorRewardDetailsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetGeneratorRewardDetails(decoded));
            }
            if let Ok(decoded) = <GetGeneratorStateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetGeneratorState(decoded));
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
            if let Ok(decoded) = <IncreaseDeclaredComputeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncreaseDeclaredCompute(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IntendToReduceComputeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntendToReduceCompute(decoded));
            }
            if let Ok(decoded) = <IntendToReduceStakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntendToReduceStake(decoded));
            }
            if let Ok(decoded) = <JoinMarketPlaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::JoinMarketPlace(decoded));
            }
            if let Ok(decoded) = <LeaveMarketPlaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LeaveMarketPlace(decoded));
            }
            if let Ok(decoded) = <LeaveMarketPlacesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LeaveMarketPlaces(decoded));
            }
            if let Ok(decoded) = <ProofMarketPlaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProofMarketPlace(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) = <RegisterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Register(decoded));
            }
            if let Ok(decoded) = <RemoveEncryptionKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveEncryptionKey(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RequestForExitMarketPlaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RequestForExitMarketPlace(decoded));
            }
            if let Ok(decoded) = <RequestForExitMarketPlacesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RequestForExitMarketPlaces(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) = <SlashGeneratorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SlashGenerator(decoded));
            }
            if let Ok(decoded) = <StakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Stake(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <UnstakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unstake(decoded));
            }
            if let Ok(decoded) = <UpdateEncryptionKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateEncryptionKey(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GeneratorRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EntityKeyRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KeyRegisterRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParallelRequestsUpperLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SlasherRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakingToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnlockWaitBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssignGeneratorTask(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeRewardAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompleteGeneratorTask(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseDeclaredCompute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deregister(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GeneratorInfoPerMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GeneratorRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGeneratorAssignmentDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGeneratorRewardDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGeneratorState(element) => {
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
                Self::IncreaseDeclaredCompute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntendToReduceCompute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntendToReduceStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::JoinMarketPlace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LeaveMarketPlace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LeaveMarketPlaces(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProofMarketPlace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Register(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveEncryptionKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestForExitMarketPlace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestForExitMarketPlaces(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SlashGenerator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Stake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unstake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateEncryptionKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for GeneratorRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::EntityKeyRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::KeyRegisterRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParallelRequestsUpperLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SlasherRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakingToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnlockWaitBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssignGeneratorTask(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeRewardAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompleteGeneratorTask(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DecreaseDeclaredCompute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Deregister(element) => ::core::fmt::Display::fmt(element, f),
                Self::GeneratorInfoPerMarket(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GeneratorRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetGeneratorAssignmentDetails(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetGeneratorRewardDetails(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetGeneratorState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMember(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseDeclaredCompute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntendToReduceCompute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntendToReduceStake(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::JoinMarketPlace(element) => ::core::fmt::Display::fmt(element, f),
                Self::LeaveMarketPlace(element) => ::core::fmt::Display::fmt(element, f),
                Self::LeaveMarketPlaces(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofMarketPlace(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::Register(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveEncryptionKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestForExitMarketPlace(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestForExitMarketPlaces(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SlashGenerator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Stake(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unstake(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateEncryptionKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for GeneratorRegistryCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<EntityKeyRegistryCall> for GeneratorRegistryCalls {
        fn from(value: EntityKeyRegistryCall) -> Self {
            Self::EntityKeyRegistry(value)
        }
    }
    impl ::core::convert::From<KeyRegisterRoleCall> for GeneratorRegistryCalls {
        fn from(value: KeyRegisterRoleCall) -> Self {
            Self::KeyRegisterRole(value)
        }
    }
    impl ::core::convert::From<ParallelRequestsUpperLimitCall>
    for GeneratorRegistryCalls {
        fn from(value: ParallelRequestsUpperLimitCall) -> Self {
            Self::ParallelRequestsUpperLimit(value)
        }
    }
    impl ::core::convert::From<SlasherRoleCall> for GeneratorRegistryCalls {
        fn from(value: SlasherRoleCall) -> Self {
            Self::SlasherRole(value)
        }
    }
    impl ::core::convert::From<StakingTokenCall> for GeneratorRegistryCalls {
        fn from(value: StakingTokenCall) -> Self {
            Self::StakingToken(value)
        }
    }
    impl ::core::convert::From<UnlockWaitBlocksCall> for GeneratorRegistryCalls {
        fn from(value: UnlockWaitBlocksCall) -> Self {
            Self::UnlockWaitBlocks(value)
        }
    }
    impl ::core::convert::From<AssignGeneratorTaskCall> for GeneratorRegistryCalls {
        fn from(value: AssignGeneratorTaskCall) -> Self {
            Self::AssignGeneratorTask(value)
        }
    }
    impl ::core::convert::From<ChangeRewardAddressCall> for GeneratorRegistryCalls {
        fn from(value: ChangeRewardAddressCall) -> Self {
            Self::ChangeRewardAddress(value)
        }
    }
    impl ::core::convert::From<CompleteGeneratorTaskCall> for GeneratorRegistryCalls {
        fn from(value: CompleteGeneratorTaskCall) -> Self {
            Self::CompleteGeneratorTask(value)
        }
    }
    impl ::core::convert::From<DecreaseDeclaredComputeCall> for GeneratorRegistryCalls {
        fn from(value: DecreaseDeclaredComputeCall) -> Self {
            Self::DecreaseDeclaredCompute(value)
        }
    }
    impl ::core::convert::From<DeregisterCall> for GeneratorRegistryCalls {
        fn from(value: DeregisterCall) -> Self {
            Self::Deregister(value)
        }
    }
    impl ::core::convert::From<GeneratorInfoPerMarketCall> for GeneratorRegistryCalls {
        fn from(value: GeneratorInfoPerMarketCall) -> Self {
            Self::GeneratorInfoPerMarket(value)
        }
    }
    impl ::core::convert::From<GeneratorRegistryCall> for GeneratorRegistryCalls {
        fn from(value: GeneratorRegistryCall) -> Self {
            Self::GeneratorRegistry(value)
        }
    }
    impl ::core::convert::From<GetGeneratorAssignmentDetailsCall>
    for GeneratorRegistryCalls {
        fn from(value: GetGeneratorAssignmentDetailsCall) -> Self {
            Self::GetGeneratorAssignmentDetails(value)
        }
    }
    impl ::core::convert::From<GetGeneratorRewardDetailsCall>
    for GeneratorRegistryCalls {
        fn from(value: GetGeneratorRewardDetailsCall) -> Self {
            Self::GetGeneratorRewardDetails(value)
        }
    }
    impl ::core::convert::From<GetGeneratorStateCall> for GeneratorRegistryCalls {
        fn from(value: GetGeneratorStateCall) -> Self {
            Self::GetGeneratorState(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for GeneratorRegistryCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCall> for GeneratorRegistryCalls {
        fn from(value: GetRoleMemberCall) -> Self {
            Self::GetRoleMember(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCountCall> for GeneratorRegistryCalls {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for GeneratorRegistryCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for GeneratorRegistryCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<IncreaseDeclaredComputeCall> for GeneratorRegistryCalls {
        fn from(value: IncreaseDeclaredComputeCall) -> Self {
            Self::IncreaseDeclaredCompute(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for GeneratorRegistryCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IntendToReduceComputeCall> for GeneratorRegistryCalls {
        fn from(value: IntendToReduceComputeCall) -> Self {
            Self::IntendToReduceCompute(value)
        }
    }
    impl ::core::convert::From<IntendToReduceStakeCall> for GeneratorRegistryCalls {
        fn from(value: IntendToReduceStakeCall) -> Self {
            Self::IntendToReduceStake(value)
        }
    }
    impl ::core::convert::From<JoinMarketPlaceCall> for GeneratorRegistryCalls {
        fn from(value: JoinMarketPlaceCall) -> Self {
            Self::JoinMarketPlace(value)
        }
    }
    impl ::core::convert::From<LeaveMarketPlaceCall> for GeneratorRegistryCalls {
        fn from(value: LeaveMarketPlaceCall) -> Self {
            Self::LeaveMarketPlace(value)
        }
    }
    impl ::core::convert::From<LeaveMarketPlacesCall> for GeneratorRegistryCalls {
        fn from(value: LeaveMarketPlacesCall) -> Self {
            Self::LeaveMarketPlaces(value)
        }
    }
    impl ::core::convert::From<ProofMarketPlaceCall> for GeneratorRegistryCalls {
        fn from(value: ProofMarketPlaceCall) -> Self {
            Self::ProofMarketPlace(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for GeneratorRegistryCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RegisterCall> for GeneratorRegistryCalls {
        fn from(value: RegisterCall) -> Self {
            Self::Register(value)
        }
    }
    impl ::core::convert::From<RemoveEncryptionKeyCall> for GeneratorRegistryCalls {
        fn from(value: RemoveEncryptionKeyCall) -> Self {
            Self::RemoveEncryptionKey(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for GeneratorRegistryCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RequestForExitMarketPlaceCall>
    for GeneratorRegistryCalls {
        fn from(value: RequestForExitMarketPlaceCall) -> Self {
            Self::RequestForExitMarketPlace(value)
        }
    }
    impl ::core::convert::From<RequestForExitMarketPlacesCall>
    for GeneratorRegistryCalls {
        fn from(value: RequestForExitMarketPlacesCall) -> Self {
            Self::RequestForExitMarketPlaces(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for GeneratorRegistryCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SlashGeneratorCall> for GeneratorRegistryCalls {
        fn from(value: SlashGeneratorCall) -> Self {
            Self::SlashGenerator(value)
        }
    }
    impl ::core::convert::From<StakeCall> for GeneratorRegistryCalls {
        fn from(value: StakeCall) -> Self {
            Self::Stake(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for GeneratorRegistryCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UnstakeCall> for GeneratorRegistryCalls {
        fn from(value: UnstakeCall) -> Self {
            Self::Unstake(value)
        }
    }
    impl ::core::convert::From<UpdateEncryptionKeyCall> for GeneratorRegistryCalls {
        fn from(value: UpdateEncryptionKeyCall) -> Self {
            Self::UpdateEncryptionKey(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall> for GeneratorRegistryCalls {
        fn from(value: UpgradeToCall) -> Self {
            Self::UpgradeTo(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for GeneratorRegistryCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
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
    ///Container type for all return fields from the `ENTITY_KEY_REGISTRY` function with signature `ENTITY_KEY_REGISTRY()` and selector `0x661de5ac`
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
    pub struct EntityKeyRegistryReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `PARALLEL_REQUESTS_UPPER_LIMIT` function with signature `PARALLEL_REQUESTS_UPPER_LIMIT()` and selector `0x7a14c463`
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
    pub struct ParallelRequestsUpperLimitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `SLASHER_ROLE` function with signature `SLASHER_ROLE()` and selector `0x5095af64`
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
    pub struct SlasherRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `STAKING_TOKEN` function with signature `STAKING_TOKEN()` and selector `0x0479d644`
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
    pub struct StakingTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `UNLOCK_WAIT_BLOCKS` function with signature `UNLOCK_WAIT_BLOCKS()` and selector `0x3c5eb57c`
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
    pub struct UnlockWaitBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `generatorInfoPerMarket` function with signature `generatorInfoPerMarket(address,uint256)` and selector `0x9a7fca8e`
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
    pub struct GeneratorInfoPerMarketReturn {
        pub state: u8,
        pub compute_per_request_required: ::ethers::core::types::U256,
        pub proof_generation_cost: ::ethers::core::types::U256,
        pub proposed_time: ::ethers::core::types::U256,
        pub active_requests: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `generatorRegistry` function with signature `generatorRegistry(address)` and selector `0x8cfc56d8`
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
    pub struct GeneratorRegistryReturn {
        pub reward_address: ::ethers::core::types::Address,
        pub total_stake: ::ethers::core::types::U256,
        pub sum_of_compute_allocations: ::ethers::core::types::U256,
        pub compute_consumed: ::ethers::core::types::U256,
        pub stake_locked: ::ethers::core::types::U256,
        pub active_market_places: ::ethers::core::types::U256,
        pub declared_compute: ::ethers::core::types::U256,
        pub intended_stake_utilization: ::ethers::core::types::U256,
        pub intended_compute_utilization: ::ethers::core::types::U256,
        pub generator_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getGeneratorAssignmentDetails` function with signature `getGeneratorAssignmentDetails(address,uint256)` and selector `0x1c7eae65`
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
    pub struct GetGeneratorAssignmentDetailsReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getGeneratorRewardDetails` function with signature `getGeneratorRewardDetails(address,uint256)` and selector `0x2b610c2d`
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
    pub struct GetGeneratorRewardDetailsReturn(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getGeneratorState` function with signature `getGeneratorState(address,uint256)` and selector `0x646d51b4`
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
    pub struct GetGeneratorStateReturn(pub u8, pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `slashGenerator` function with signature `slashGenerator(address,uint256,uint256,address)` and selector `0xeaacae94`
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
    pub struct SlashGeneratorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stake` function with signature `stake(address,uint256)` and selector `0xadc9772e`
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
    pub struct StakeReturn(pub ::ethers::core::types::U256);
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
}
