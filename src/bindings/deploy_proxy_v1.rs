pub use deploy_proxy_v1::*;
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
pub mod deploy_proxy_v1 {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"proxy\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_SCRIPT\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"run\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static DEPLOYPROXYV1_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        128,
        96,
        64,
        82,
        52,
        97,
        0,
        61,
        87,
        96,
        12,
        128,
        84,
        96,
        1,
        96,
        1,
        96,
        168,
        27,
        3,
        25,
        22,
        116,
        146,
        72,
        181,
        230,
        114,
        225,
        136,
        10,
        243,
        64,
        104,
        192,
        250,
        225,
        141,
        48,
        194,
        109,
        5,
        251,
        1,
        23,
        144,
        85,
        97,
        2,
        198,
        144,
        129,
        97,
        0,
        67,
        130,
        57,
        243,
        91,
        96,
        0,
        128,
        253,
        254,
        96,
        128,
        96,
        64,
        129,
        129,
        82,
        96,
        4,
        128,
        54,
        16,
        21,
        97,
        0,
        21,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        0,
        146,
        131,
        53,
        96,
        224,
        28,
        145,
        130,
        99,
        239,
        106,
        192,
        240,
        20,
        97,
        0,
        126,
        87,
        80,
        80,
        99,
        248,
        204,
        191,
        71,
        20,
        97,
        0,
        58,
        87,
        96,
        0,
        128,
        253,
        91,
        52,
        97,
        0,
        122,
        87,
        129,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        252,
        54,
        1,
        18,
        97,
        0,
        122,
        87,
        96,
        32,
        144,
        96,
        255,
        96,
        12,
        84,
        22,
        144,
        81,
        144,
        21,
        21,
        129,
        82,
        243,
        91,
        80,
        128,
        253,
        91,
        144,
        145,
        80,
        52,
        97,
        2,
        181,
        87,
        96,
        32,
        145,
        130,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        252,
        54,
        1,
        18,
        97,
        2,
        32,
        87,
        132,
        115,
        113,
        9,
        112,
        158,
        207,
        169,
        26,
        128,
        98,
        111,
        243,
        152,
        157,
        104,
        246,
        127,
        91,
        29,
        209,
        45,
        128,
        59,
        21,
        97,
        0,
        122,
        87,
        131,
        131,
        131,
        129,
        147,
        127,
        175,
        201,
        128,
        64,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        131,
        82,
        90,
        241,
        128,
        21,
        97,
        2,
        171,
        87,
        97,
        2,
        100,
        87,
        91,
        80,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        146,
        132,
        131,
        133,
        96,
        12,
        84,
        96,
        8,
        28,
        22,
        96,
        36,
        132,
        81,
        128,
        148,
        129,
        147,
        127,
        29,
        100,
        118,
        5,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        131,
        82,
        136,
        53,
        137,
        132,
        1,
        82,
        90,
        241,
        146,
        131,
        21,
        97,
        2,
        90,
        87,
        134,
        147,
        97,
        1,
        147,
        87,
        91,
        80,
        80,
        81,
        146,
        22,
        130,
        82,
        127,
        6,
        113,
        60,
        62,
        37,
        215,
        124,
        252,
        129,
        157,
        235,
        128,
        156,
        25,
        59,
        160,
        90,
        223,
        209,
        128,
        248,
        100,
        112,
        45,
        125,
        153,
        35,
        113,
        231,
        142,
        62,
        127,
        145,
        161,
        128,
        243,
        91,
        144,
        145,
        146,
        80,
        131,
        144,
        61,
        133,
        17,
        97,
        2,
        82,
        87,
        91,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        224,
        96,
        31,
        131,
        1,
        22,
        131,
        1,
        144,
        131,
        130,
        16,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        131,
        17,
        23,
        97,
        2,
        36,
        87,
        80,
        132,
        145,
        131,
        145,
        133,
        82,
        129,
        1,
        3,
        18,
        97,
        2,
        32,
        87,
        81,
        146,
        128,
        132,
        22,
        132,
        3,
        97,
        2,
        32,
        87,
        146,
        144,
        127,
        6,
        113,
        60,
        62,
        37,
        215,
        124,
        252,
        129,
        157,
        235,
        128,
        156,
        25,
        59,
        160,
        90,
        223,
        209,
        128,
        248,
        100,
        112,
        45,
        125,
        153,
        35,
        113,
        231,
        142,
        62,
        127,
        56,
        97,
        1,
        102,
        86,
        91,
        132,
        128,
        253,
        91,
        96,
        65,
        144,
        127,
        78,
        72,
        123,
        113,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        0,
        82,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        61,
        145,
        80,
        97,
        1,
        161,
        86,
        91,
        130,
        81,
        61,
        136,
        130,
        62,
        61,
        144,
        253,
        91,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        149,
        146,
        149,
        17,
        97,
        2,
        127,
        87,
        131,
        82,
        146,
        56,
        97,
        1,
        4,
        86,
        91,
        96,
        36,
        130,
        96,
        65,
        135,
        127,
        78,
        72,
        123,
        113,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        131,
        82,
        82,
        253,
        91,
        132,
        81,
        61,
        135,
        130,
        62,
        61,
        144,
        253,
        91,
        131,
        128,
        253,
        254,
        161,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        17,
        0,
        10,
    ];
    ///The bytecode of the contract.
    pub static DEPLOYPROXYV1_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        129,
        129,
        82,
        96,
        4,
        128,
        54,
        16,
        21,
        97,
        0,
        21,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        0,
        146,
        131,
        53,
        96,
        224,
        28,
        145,
        130,
        99,
        239,
        106,
        192,
        240,
        20,
        97,
        0,
        126,
        87,
        80,
        80,
        99,
        248,
        204,
        191,
        71,
        20,
        97,
        0,
        58,
        87,
        96,
        0,
        128,
        253,
        91,
        52,
        97,
        0,
        122,
        87,
        129,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        252,
        54,
        1,
        18,
        97,
        0,
        122,
        87,
        96,
        32,
        144,
        96,
        255,
        96,
        12,
        84,
        22,
        144,
        81,
        144,
        21,
        21,
        129,
        82,
        243,
        91,
        80,
        128,
        253,
        91,
        144,
        145,
        80,
        52,
        97,
        2,
        181,
        87,
        96,
        32,
        145,
        130,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        252,
        54,
        1,
        18,
        97,
        2,
        32,
        87,
        132,
        115,
        113,
        9,
        112,
        158,
        207,
        169,
        26,
        128,
        98,
        111,
        243,
        152,
        157,
        104,
        246,
        127,
        91,
        29,
        209,
        45,
        128,
        59,
        21,
        97,
        0,
        122,
        87,
        131,
        131,
        131,
        129,
        147,
        127,
        175,
        201,
        128,
        64,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        131,
        82,
        90,
        241,
        128,
        21,
        97,
        2,
        171,
        87,
        97,
        2,
        100,
        87,
        91,
        80,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        146,
        132,
        131,
        133,
        96,
        12,
        84,
        96,
        8,
        28,
        22,
        96,
        36,
        132,
        81,
        128,
        148,
        129,
        147,
        127,
        29,
        100,
        118,
        5,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        131,
        82,
        136,
        53,
        137,
        132,
        1,
        82,
        90,
        241,
        146,
        131,
        21,
        97,
        2,
        90,
        87,
        134,
        147,
        97,
        1,
        147,
        87,
        91,
        80,
        80,
        81,
        146,
        22,
        130,
        82,
        127,
        6,
        113,
        60,
        62,
        37,
        215,
        124,
        252,
        129,
        157,
        235,
        128,
        156,
        25,
        59,
        160,
        90,
        223,
        209,
        128,
        248,
        100,
        112,
        45,
        125,
        153,
        35,
        113,
        231,
        142,
        62,
        127,
        145,
        161,
        128,
        243,
        91,
        144,
        145,
        146,
        80,
        131,
        144,
        61,
        133,
        17,
        97,
        2,
        82,
        87,
        91,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        224,
        96,
        31,
        131,
        1,
        22,
        131,
        1,
        144,
        131,
        130,
        16,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        131,
        17,
        23,
        97,
        2,
        36,
        87,
        80,
        132,
        145,
        131,
        145,
        133,
        82,
        129,
        1,
        3,
        18,
        97,
        2,
        32,
        87,
        81,
        146,
        128,
        132,
        22,
        132,
        3,
        97,
        2,
        32,
        87,
        146,
        144,
        127,
        6,
        113,
        60,
        62,
        37,
        215,
        124,
        252,
        129,
        157,
        235,
        128,
        156,
        25,
        59,
        160,
        90,
        223,
        209,
        128,
        248,
        100,
        112,
        45,
        125,
        153,
        35,
        113,
        231,
        142,
        62,
        127,
        56,
        97,
        1,
        102,
        86,
        91,
        132,
        128,
        253,
        91,
        96,
        65,
        144,
        127,
        78,
        72,
        123,
        113,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        0,
        82,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        61,
        145,
        80,
        97,
        1,
        161,
        86,
        91,
        130,
        81,
        61,
        136,
        130,
        62,
        61,
        144,
        253,
        91,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        149,
        146,
        149,
        17,
        97,
        2,
        127,
        87,
        131,
        82,
        146,
        56,
        97,
        1,
        4,
        86,
        91,
        96,
        36,
        130,
        96,
        65,
        135,
        127,
        78,
        72,
        123,
        113,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        131,
        82,
        82,
        253,
        91,
        132,
        81,
        61,
        135,
        130,
        62,
        61,
        144,
        253,
        91,
        131,
        128,
        253,
        254,
        161,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        17,
        0,
        10,
    ];
    ///The deployed bytecode of the contract.
    pub static DEPLOYPROXYV1_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DeployProxyV1<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DeployProxyV1<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DeployProxyV1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DeployProxyV1<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DeployProxyV1<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(DeployProxyV1)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DeployProxyV1<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEPLOYPROXYV1_ABI.clone(),
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
                DEPLOYPROXYV1_ABI.clone(),
                DEPLOYPROXYV1_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function
        pub fn is_script(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `run` (0xef6ac0f0) function
        pub fn run(
            &self,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 106, 192, 240], salt)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `proxy` event
        pub fn proxy_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProxyFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProxyFilter> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DeployProxyV1<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "proxy", abi = "proxy(address)")]
    pub struct ProxyFilter(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    ///Container type for all input parameters for the `run` function with signature `run(bytes32)` and selector `0xef6ac0f0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "run", abi = "run(bytes32)")]
    pub struct RunCall {
        pub salt: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DeployProxyV1Calls {
        IsScript(IsScriptCall),
        Run(RunCall),
    }
    impl ::ethers::core::abi::AbiDecode for DeployProxyV1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <IsScriptCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsScript(decoded));
            }
            if let Ok(decoded)
                = <RunCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Run(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DeployProxyV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsScript(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Run(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DeployProxyV1Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsScript(element) => ::core::fmt::Display::fmt(element, f),
                Self::Run(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsScriptCall> for DeployProxyV1Calls {
        fn from(value: IsScriptCall) -> Self {
            Self::IsScript(value)
        }
    }
    impl ::core::convert::From<RunCall> for DeployProxyV1Calls {
        fn from(value: RunCall) -> Self {
            Self::Run(value)
        }
    }
    ///Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsScriptReturn(pub bool);
}