pub use i_uniswap_v2_factory::*;
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
pub mod i_uniswap_v2_factory {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"token0\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"token1\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"pair\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"PairCreated\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"allPairs\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pair\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"allPairsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenA\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenB\",\"type\":\"address\"}],\"name\":\"createPair\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pair\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"feeTo\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"feeToSetter\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenA\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenB\",\"type\":\"address\"}],\"name\":\"getPair\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pair\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"setFeeTo\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"setFeeToSetter\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]";
    ///The parsed JSON ABI of the contract.
    pub static IUNISWAPV2FACTORY_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> = ::ethers_contract::Lazy::new(||
    ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IUniswapV2Factory<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IUniswapV2Factory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IUniswapV2Factory<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IUniswapV2Factory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IUniswapV2Factory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV2Factory)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IUniswapV2Factory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IUNISWAPV2FACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `allPairs` (0x1e3dd18b) function
        pub fn all_pairs(
            &self,
            p0: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([30, 61, 209, 139], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allPairsLength` (0x574f2ba3) function
        pub fn all_pairs_length(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([87, 79, 43, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPair` (0xc9c65396) function
        pub fn create_pair(
            &self,
            token_a: ::ethers_core::types::Address,
            token_b: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([201, 198, 83, 150], (token_a, token_b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeTo` (0x017e7e58) function
        pub fn fee_to(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([1, 126, 126, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeToSetter` (0x094b7415) function
        pub fn fee_to_setter(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([9, 75, 116, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPair` (0xe6a43905) function
        pub fn get_pair(
            &self,
            token_a: ::ethers_core::types::Address,
            token_b: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([230, 164, 57, 5], (token_a, token_b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeTo` (0xf46901ed) function
        pub fn set_fee_to(
            &self,
            p0: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 105, 1, 237], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeToSetter` (0xa2e74af6) function
        pub fn set_fee_to_setter(
            &self,
            p0: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 231, 74, 246], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PairCreated` event
        pub fn pair_created_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PairCreatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PairCreatedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IUniswapV2Factory<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "PairCreated",
        abi = "PairCreated(address,address,address,uint256)"
    )]
    pub struct PairCreatedFilter {
        #[ethevent(indexed)]
        pub token_0: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub token_1: ::ethers_core::types::Address,
        pub pair: ::ethers_core::types::Address,
        pub p3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `allPairs` function with signature `allPairs(uint256)` and selector `0x1e3dd18b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "allPairs", abi = "allPairs(uint256)")]
    pub struct AllPairsCall(pub ::ethers_core::types::U256);
    ///Container type for all input parameters for the `allPairsLength` function with signature `allPairsLength()` and selector `0x574f2ba3`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "allPairsLength", abi = "allPairsLength()")]
    pub struct AllPairsLengthCall;
    ///Container type for all input parameters for the `createPair` function with signature `createPair(address,address)` and selector `0xc9c65396`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "createPair", abi = "createPair(address,address)")]
    pub struct CreatePairCall {
        pub token_a: ::ethers_core::types::Address,
        pub token_b: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `feeTo` function with signature `feeTo()` and selector `0x017e7e58`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "feeTo", abi = "feeTo()")]
    pub struct FeeToCall;
    ///Container type for all input parameters for the `feeToSetter` function with signature `feeToSetter()` and selector `0x094b7415`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "feeToSetter", abi = "feeToSetter()")]
    pub struct FeeToSetterCall;
    ///Container type for all input parameters for the `getPair` function with signature `getPair(address,address)` and selector `0xe6a43905`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPair", abi = "getPair(address,address)")]
    pub struct GetPairCall {
        pub token_a: ::ethers_core::types::Address,
        pub token_b: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `setFeeTo` function with signature `setFeeTo(address)` and selector `0xf46901ed`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setFeeTo", abi = "setFeeTo(address)")]
    pub struct SetFeeToCall(pub ::ethers_core::types::Address);
    ///Container type for all input parameters for the `setFeeToSetter` function with signature `setFeeToSetter(address)` and selector `0xa2e74af6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setFeeToSetter", abi = "setFeeToSetter(address)")]
    pub struct SetFeeToSetterCall(pub ::ethers_core::types::Address);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IUniswapV2FactoryCalls {
        AllPairs(AllPairsCall),
        AllPairsLength(AllPairsLengthCall),
        CreatePair(CreatePairCall),
        FeeTo(FeeToCall),
        FeeToSetter(FeeToSetterCall),
        GetPair(GetPairCall),
        SetFeeTo(SetFeeToCall),
        SetFeeToSetter(SetFeeToSetterCall),
    }
    impl ::ethers_core::abi::AbiDecode for IUniswapV2FactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AllPairsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AllPairs(decoded));
            }
            if let Ok(decoded)
                = <AllPairsLengthCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AllPairsLength(decoded));
            }
            if let Ok(decoded)
                = <CreatePairCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreatePair(decoded));
            }
            if let Ok(decoded)
                = <FeeToCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeTo(decoded));
            }
            if let Ok(decoded)
                = <FeeToSetterCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeToSetter(decoded));
            }
            if let Ok(decoded)
                = <GetPairCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPair(decoded));
            }
            if let Ok(decoded)
                = <SetFeeToCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFeeTo(decoded));
            }
            if let Ok(decoded)
                = <SetFeeToSetterCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFeeToSetter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for IUniswapV2FactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AllPairs(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AllPairsLength(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CreatePair(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::FeeTo(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::FeeToSetter(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetPair(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetFeeTo(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetFeeToSetter(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IUniswapV2FactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllPairs(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllPairsLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePair(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeToSetter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPair(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeToSetter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllPairsCall> for IUniswapV2FactoryCalls {
        fn from(value: AllPairsCall) -> Self {
            Self::AllPairs(value)
        }
    }
    impl ::core::convert::From<AllPairsLengthCall> for IUniswapV2FactoryCalls {
        fn from(value: AllPairsLengthCall) -> Self {
            Self::AllPairsLength(value)
        }
    }
    impl ::core::convert::From<CreatePairCall> for IUniswapV2FactoryCalls {
        fn from(value: CreatePairCall) -> Self {
            Self::CreatePair(value)
        }
    }
    impl ::core::convert::From<FeeToCall> for IUniswapV2FactoryCalls {
        fn from(value: FeeToCall) -> Self {
            Self::FeeTo(value)
        }
    }
    impl ::core::convert::From<FeeToSetterCall> for IUniswapV2FactoryCalls {
        fn from(value: FeeToSetterCall) -> Self {
            Self::FeeToSetter(value)
        }
    }
    impl ::core::convert::From<GetPairCall> for IUniswapV2FactoryCalls {
        fn from(value: GetPairCall) -> Self {
            Self::GetPair(value)
        }
    }
    impl ::core::convert::From<SetFeeToCall> for IUniswapV2FactoryCalls {
        fn from(value: SetFeeToCall) -> Self {
            Self::SetFeeTo(value)
        }
    }
    impl ::core::convert::From<SetFeeToSetterCall> for IUniswapV2FactoryCalls {
        fn from(value: SetFeeToSetterCall) -> Self {
            Self::SetFeeToSetter(value)
        }
    }
    ///Container type for all return fields from the `allPairs` function with signature `allPairs(uint256)` and selector `0x1e3dd18b`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AllPairsReturn {
        pub pair: ::ethers_core::types::Address,
    }
    ///Container type for all return fields from the `allPairsLength` function with signature `allPairsLength()` and selector `0x574f2ba3`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AllPairsLengthReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `createPair` function with signature `createPair(address,address)` and selector `0xc9c65396`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CreatePairReturn {
        pub pair: ::ethers_core::types::Address,
    }
    ///Container type for all return fields from the `feeTo` function with signature `feeTo()` and selector `0x017e7e58`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FeeToReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `feeToSetter` function with signature `feeToSetter()` and selector `0x094b7415`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FeeToSetterReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `getPair` function with signature `getPair(address,address)` and selector `0xe6a43905`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetPairReturn {
        pub pair: ::ethers_core::types::Address,
    }
}
