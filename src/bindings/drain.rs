pub use drain::*;
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
pub mod drain {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"drain\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"drainAndApprove\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static DRAIN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
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
        22,
        87,
        97,
        3,
        75,
        144,
        129,
        97,
        0,
        28,
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
        144,
        128,
        130,
        82,
        96,
        4,
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
        145,
        130,
        53,
        96,
        224,
        28,
        145,
        130,
        99,
        10,
        68,
        50,
        118,
        20,
        97,
        2,
        2,
        87,
        80,
        129,
        99,
        16,
        26,
        230,
        94,
        20,
        97,
        0,
        138,
        87,
        80,
        99,
        236,
        229,
        49,
        50,
        20,
        97,
        0,
        69,
        87,
        96,
        0,
        128,
        253,
        91,
        52,
        97,
        0,
        135,
        87,
        96,
        32,
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
        135,
        87,
        97,
        0,
        132,
        97,
        0,
        127,
        97,
        2,
        208,
        86,
        91,
        97,
        2,
        248,
        86,
        91,
        128,
        243,
        91,
        128,
        253,
        91,
        144,
        80,
        52,
        97,
        1,
        191,
        87,
        96,
        32,
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
        1,
        191,
        87,
        97,
        0,
        195,
        97,
        2,
        208,
        86,
        91,
        144,
        97,
        0,
        205,
        130,
        97,
        2,
        248,
        86,
        91,
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
        129,
        81,
        146,
        127,
        9,
        94,
        167,
        179,
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
        132,
        82,
        22,
        96,
        4,
        131,
        1,
        82,
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
        255,
        96,
        36,
        131,
        1,
        82,
        96,
        32,
        130,
        96,
        68,
        129,
        134,
        110,
        140,
        67,
        239,
        192,
        20,
        116,
        108,
        35,
        0,
        73,
        227,
        48,
        3,
        156,
        179,
        90,
        241,
        128,
        21,
        97,
        1,
        248,
        87,
        97,
        1,
        91,
        87,
        130,
        128,
        243,
        91,
        96,
        32,
        144,
        61,
        130,
        17,
        97,
        1,
        240,
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
        1,
        195,
        87,
        82,
        96,
        32,
        144,
        130,
        1,
        130,
        144,
        3,
        18,
        97,
        1,
        191,
        87,
        81,
        128,
        21,
        21,
        3,
        97,
        0,
        135,
        87,
        56,
        128,
        130,
        128,
        243,
        91,
        80,
        128,
        253,
        91,
        96,
        36,
        133,
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
        129,
        82,
        96,
        65,
        96,
        4,
        82,
        253,
        91,
        61,
        145,
        80,
        97,
        1,
        102,
        86,
        91,
        129,
        81,
        61,
        133,
        130,
        62,
        61,
        144,
        253,
        91,
        145,
        80,
        52,
        97,
        2,
        204,
        87,
        128,
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
        204,
        87,
        130,
        97,
        2,
        59,
        97,
        2,
        208,
        86,
        91,
        96,
        36,
        53,
        144,
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
        145,
        130,
        129,
        22,
        128,
        145,
        3,
        97,
        2,
        200,
        87,
        96,
        32,
        147,
        134,
        147,
        96,
        68,
        147,
        133,
        147,
        127,
        9,
        94,
        167,
        179,
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
        133,
        82,
        96,
        4,
        133,
        1,
        82,
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
        255,
        96,
        36,
        133,
        1,
        82,
        22,
        90,
        241,
        128,
        21,
        97,
        1,
        248,
        87,
        97,
        1,
        91,
        87,
        130,
        128,
        243,
        91,
        131,
        128,
        253,
        91,
        130,
        128,
        253,
        91,
        96,
        4,
        53,
        144,
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
        130,
        22,
        130,
        3,
        97,
        2,
        243,
        87,
        86,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        0,
        128,
        128,
        128,
        147,
        71,
        144,
        130,
        144,
        130,
        21,
        97,
        3,
        52,
        87,
        91,
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
        22,
        144,
        241,
        21,
        97,
        3,
        40,
        87,
        86,
        91,
        96,
        64,
        81,
        61,
        96,
        0,
        130,
        62,
        61,
        144,
        253,
        91,
        97,
        8,
        252,
        145,
        80,
        97,
        3,
        9,
        86,
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
    pub static DRAIN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        144,
        128,
        130,
        82,
        96,
        4,
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
        145,
        130,
        53,
        96,
        224,
        28,
        145,
        130,
        99,
        10,
        68,
        50,
        118,
        20,
        97,
        2,
        2,
        87,
        80,
        129,
        99,
        16,
        26,
        230,
        94,
        20,
        97,
        0,
        138,
        87,
        80,
        99,
        236,
        229,
        49,
        50,
        20,
        97,
        0,
        69,
        87,
        96,
        0,
        128,
        253,
        91,
        52,
        97,
        0,
        135,
        87,
        96,
        32,
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
        135,
        87,
        97,
        0,
        132,
        97,
        0,
        127,
        97,
        2,
        208,
        86,
        91,
        97,
        2,
        248,
        86,
        91,
        128,
        243,
        91,
        128,
        253,
        91,
        144,
        80,
        52,
        97,
        1,
        191,
        87,
        96,
        32,
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
        1,
        191,
        87,
        97,
        0,
        195,
        97,
        2,
        208,
        86,
        91,
        144,
        97,
        0,
        205,
        130,
        97,
        2,
        248,
        86,
        91,
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
        129,
        81,
        146,
        127,
        9,
        94,
        167,
        179,
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
        132,
        82,
        22,
        96,
        4,
        131,
        1,
        82,
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
        255,
        96,
        36,
        131,
        1,
        82,
        96,
        32,
        130,
        96,
        68,
        129,
        134,
        110,
        140,
        67,
        239,
        192,
        20,
        116,
        108,
        35,
        0,
        73,
        227,
        48,
        3,
        156,
        179,
        90,
        241,
        128,
        21,
        97,
        1,
        248,
        87,
        97,
        1,
        91,
        87,
        130,
        128,
        243,
        91,
        96,
        32,
        144,
        61,
        130,
        17,
        97,
        1,
        240,
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
        1,
        195,
        87,
        82,
        96,
        32,
        144,
        130,
        1,
        130,
        144,
        3,
        18,
        97,
        1,
        191,
        87,
        81,
        128,
        21,
        21,
        3,
        97,
        0,
        135,
        87,
        56,
        128,
        130,
        128,
        243,
        91,
        80,
        128,
        253,
        91,
        96,
        36,
        133,
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
        129,
        82,
        96,
        65,
        96,
        4,
        82,
        253,
        91,
        61,
        145,
        80,
        97,
        1,
        102,
        86,
        91,
        129,
        81,
        61,
        133,
        130,
        62,
        61,
        144,
        253,
        91,
        145,
        80,
        52,
        97,
        2,
        204,
        87,
        128,
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
        204,
        87,
        130,
        97,
        2,
        59,
        97,
        2,
        208,
        86,
        91,
        96,
        36,
        53,
        144,
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
        145,
        130,
        129,
        22,
        128,
        145,
        3,
        97,
        2,
        200,
        87,
        96,
        32,
        147,
        134,
        147,
        96,
        68,
        147,
        133,
        147,
        127,
        9,
        94,
        167,
        179,
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
        133,
        82,
        96,
        4,
        133,
        1,
        82,
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
        255,
        96,
        36,
        133,
        1,
        82,
        22,
        90,
        241,
        128,
        21,
        97,
        1,
        248,
        87,
        97,
        1,
        91,
        87,
        130,
        128,
        243,
        91,
        131,
        128,
        253,
        91,
        130,
        128,
        253,
        91,
        96,
        4,
        53,
        144,
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
        130,
        22,
        130,
        3,
        97,
        2,
        243,
        87,
        86,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        0,
        128,
        128,
        128,
        147,
        71,
        144,
        130,
        144,
        130,
        21,
        97,
        3,
        52,
        87,
        91,
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
        22,
        144,
        241,
        21,
        97,
        3,
        40,
        87,
        86,
        91,
        96,
        64,
        81,
        61,
        96,
        0,
        130,
        62,
        61,
        144,
        253,
        91,
        97,
        8,
        252,
        145,
        80,
        97,
        3,
        9,
        86,
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
    pub static DRAIN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Drain<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Drain<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Drain<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Drain<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Drain<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Drain)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Drain<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DRAIN_ABI.clone(),
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
                DRAIN_ABI.clone(),
                DRAIN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `approveAll` (0x0a443276) function
        pub fn approve_all(
            &self,
            asset: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 68, 50, 118], (asset, to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `drain` (0xece53132) function
        pub fn drain(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 229, 49, 50], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `drainAndApprove` (0x101ae65e) function
        pub fn drain_and_approve(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 26, 230, 94], to)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Drain<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `approveAll` function with signature `approveAll(address,address)` and selector `0x0a443276`
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
    #[ethcall(name = "approveAll", abi = "approveAll(address,address)")]
    pub struct ApproveAllCall {
        pub asset: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `drain` function with signature `drain(address)` and selector `0xece53132`
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
    #[ethcall(name = "drain", abi = "drain(address)")]
    pub struct DrainCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `drainAndApprove` function with signature `drainAndApprove(address)` and selector `0x101ae65e`
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
    #[ethcall(name = "drainAndApprove", abi = "drainAndApprove(address)")]
    pub struct DrainAndApproveCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DrainCalls {
        ApproveAll(ApproveAllCall),
        Drain(DrainCall),
        DrainAndApprove(DrainAndApproveCall),
    }
    impl ::ethers::core::abi::AbiDecode for DrainCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ApproveAllCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ApproveAll(decoded));
            }
            if let Ok(decoded)
                = <DrainCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Drain(decoded));
            }
            if let Ok(decoded)
                = <DrainAndApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DrainAndApprove(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DrainCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ApproveAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Drain(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DrainAndApprove(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DrainCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApproveAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Drain(element) => ::core::fmt::Display::fmt(element, f),
                Self::DrainAndApprove(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproveAllCall> for DrainCalls {
        fn from(value: ApproveAllCall) -> Self {
            Self::ApproveAll(value)
        }
    }
    impl ::core::convert::From<DrainCall> for DrainCalls {
        fn from(value: DrainCall) -> Self {
            Self::Drain(value)
        }
    }
    impl ::core::convert::From<DrainAndApproveCall> for DrainCalls {
        fn from(value: DrainAndApproveCall) -> Self {
            Self::DrainAndApprove(value)
        }
    }
}