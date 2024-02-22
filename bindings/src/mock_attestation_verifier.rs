pub use mock_attestation_verifier::*;
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
pub mod mock_attestation_verifier {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                                    name: ::std::string::String::new(),
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
    pub static MOCKATTESTATIONVERIFIER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03$\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c\x06\xD5\xA2\xED\x14a\0\\W\x80c\x8Ev\n\xFE\x14a\0oW\x80c\x94\x8BDY\x14a\0\x98W\x80c\xB2GI\x9B\x14a\0\xB0W\x80c\xB9 \x9E3\x14a\0\xCCW[`\0\x80\xFD[a\0ma\0j6`\x04a\x01\x91V[PV[\0[a\0\x83a\0}6`\x04a\x01\x91V[P`\x01\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0ma\0\xA66`\x04a\x01\xEAV[PPPPPPPPV[a\0\x83a\0\xBE6`\x04a\x01\xEAV[`\x01\x98\x97PPPPPPPPV[a\0\xE0a\0\xDA6`\x04a\x02\xCCV[P`\0\x90V[`@Q\x90\x81R` \x01a\0\x8FV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x01\x15W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x010Wa\x010a\0\xEEV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x01XWa\x01Xa\0\xEEV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x01qW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x01\xA3W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xBAW`\0\x80\xFD[a\x01\xC6\x84\x82\x85\x01a\x01\x04V[\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xE5W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x02\x07W`\0\x80\xFD[\x885g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\x1FW`\0\x80\xFD[a\x02+\x8C\x83\x8D\x01a\x01\x04V[\x99Pa\x029` \x8C\x01a\x01\xCEV[\x98Pa\x02G`@\x8C\x01a\x01\xCEV[\x97P``\x8B\x015\x91P\x80\x82\x11\x15a\x02]W`\0\x80\xFD[a\x02i\x8C\x83\x8D\x01a\x01\x04V[\x96P`\x80\x8B\x015\x91P\x80\x82\x11\x15a\x02\x7FW`\0\x80\xFD[a\x02\x8B\x8C\x83\x8D\x01a\x01\x04V[\x95P`\xA0\x8B\x015\x91P\x80\x82\x11\x15a\x02\xA1W`\0\x80\xFD[Pa\x02\xAE\x8B\x82\x8C\x01a\x01\x04V[\x93PP`\xC0\x89\x015\x91P`\xE0\x89\x015\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0` \x82\x84\x03\x12\x15a\x02\xDEW`\0\x80\xFD[a\x02\xE7\x82a\x01\xCEV[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xD9E\r\x7F\xA0#\xA5\xEA1\xBD\xA8\xE3D\xED\xA9\xDE7\xC5\x98\xB4m\xFC\xEFx\xE6p\xC5\t\xE7\xFFK\xECdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static MOCKATTESTATIONVERIFIER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c\x06\xD5\xA2\xED\x14a\0\\W\x80c\x8Ev\n\xFE\x14a\0oW\x80c\x94\x8BDY\x14a\0\x98W\x80c\xB2GI\x9B\x14a\0\xB0W\x80c\xB9 \x9E3\x14a\0\xCCW[`\0\x80\xFD[a\0ma\0j6`\x04a\x01\x91V[PV[\0[a\0\x83a\0}6`\x04a\x01\x91V[P`\x01\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0ma\0\xA66`\x04a\x01\xEAV[PPPPPPPPV[a\0\x83a\0\xBE6`\x04a\x01\xEAV[`\x01\x98\x97PPPPPPPPV[a\0\xE0a\0\xDA6`\x04a\x02\xCCV[P`\0\x90V[`@Q\x90\x81R` \x01a\0\x8FV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x01\x15W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x010Wa\x010a\0\xEEV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x01XWa\x01Xa\0\xEEV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x01qW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x01\xA3W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xBAW`\0\x80\xFD[a\x01\xC6\x84\x82\x85\x01a\x01\x04V[\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xE5W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x02\x07W`\0\x80\xFD[\x885g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\x1FW`\0\x80\xFD[a\x02+\x8C\x83\x8D\x01a\x01\x04V[\x99Pa\x029` \x8C\x01a\x01\xCEV[\x98Pa\x02G`@\x8C\x01a\x01\xCEV[\x97P``\x8B\x015\x91P\x80\x82\x11\x15a\x02]W`\0\x80\xFD[a\x02i\x8C\x83\x8D\x01a\x01\x04V[\x96P`\x80\x8B\x015\x91P\x80\x82\x11\x15a\x02\x7FW`\0\x80\xFD[a\x02\x8B\x8C\x83\x8D\x01a\x01\x04V[\x95P`\xA0\x8B\x015\x91P\x80\x82\x11\x15a\x02\xA1W`\0\x80\xFD[Pa\x02\xAE\x8B\x82\x8C\x01a\x01\x04V[\x93PP`\xC0\x89\x015\x91P`\xE0\x89\x015\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0` \x82\x84\x03\x12\x15a\x02\xDEW`\0\x80\xFD[a\x02\xE7\x82a\x01\xCEV[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xD9E\r\x7F\xA0#\xA5\xEA1\xBD\xA8\xE3D\xED\xA9\xDE7\xC5\x98\xB4m\xFC\xEFx\xE6p\xC5\t\xE7\xFFK\xECdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKATTESTATIONVERIFIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockAttestationVerifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockAttestationVerifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockAttestationVerifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockAttestationVerifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockAttestationVerifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockAttestationVerifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockAttestationVerifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKATTESTATIONVERIFIER_ABI.clone(),
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
                MOCKATTESTATIONVERIFIER_ABI.clone(),
                MOCKATTESTATIONVERIFIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `verify` (0x8e760afe) function
        pub fn verify(
            &self,
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([142, 118, 10, 254], p0)
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockAttestationVerifier<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    pub struct VerifyCall(pub ::ethers::core::types::Bytes);
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
    pub enum MockAttestationVerifierCalls {
        IsVerified(IsVerifiedCall),
        SafeVerify(SafeVerifyCall),
        SafeVerifyWithAttestation(SafeVerifyWithAttestationCall),
        Verify(VerifyCall),
        VerifyWithAttestation(VerifyWithAttestationCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockAttestationVerifierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsVerifiedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsVerified(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockAttestationVerifierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsVerified(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeVerify(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeVerifyWithAttestation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyWithAttestation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockAttestationVerifierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsVerified(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeVerify(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeVerifyWithAttestation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyWithAttestation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<IsVerifiedCall> for MockAttestationVerifierCalls {
        fn from(value: IsVerifiedCall) -> Self {
            Self::IsVerified(value)
        }
    }
    impl ::core::convert::From<SafeVerifyCall> for MockAttestationVerifierCalls {
        fn from(value: SafeVerifyCall) -> Self {
            Self::SafeVerify(value)
        }
    }
    impl ::core::convert::From<SafeVerifyWithAttestationCall>
    for MockAttestationVerifierCalls {
        fn from(value: SafeVerifyWithAttestationCall) -> Self {
            Self::SafeVerifyWithAttestation(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for MockAttestationVerifierCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    impl ::core::convert::From<VerifyWithAttestationCall>
    for MockAttestationVerifierCalls {
        fn from(value: VerifyWithAttestationCall) -> Self {
            Self::VerifyWithAttestation(value)
        }
    }
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
}
