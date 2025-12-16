///Module containing a contract's types and functions.
/**

```solidity
library ISignatureTransfer {
    struct PermitTransferFrom { TokenPermissions permitted; uint256 nonce; uint256 deadline; }
    struct TokenPermissions { address token; uint256 amount; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISignatureTransfer {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct PermitTransferFrom { TokenPermissions permitted; uint256 nonce; uint256 deadline; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PermitTransferFrom {
        #[allow(missing_docs)]
        pub permitted: <TokenPermissions as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            TokenPermissions,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <TokenPermissions as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PermitTransferFrom> for UnderlyingRustTuple<'_> {
            fn from(value: PermitTransferFrom) -> Self {
                (value.permitted, value.nonce, value.deadline)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PermitTransferFrom {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    permitted: tuple.0,
                    nonce: tuple.1,
                    deadline: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PermitTransferFrom {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PermitTransferFrom {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <TokenPermissions as alloy_sol_types::SolType>::tokenize(
                        &self.permitted,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for PermitTransferFrom {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for PermitTransferFrom {
            const NAME: &'static str = "PermitTransferFrom";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PermitTransferFrom(TokenPermissions permitted,uint256 nonce,uint256 deadline)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <TokenPermissions as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <TokenPermissions as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <TokenPermissions as alloy_sol_types::SolType>::eip712_data_word(
                            &self.permitted,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.deadline)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PermitTransferFrom {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <TokenPermissions as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.permitted,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.deadline,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <TokenPermissions as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.permitted,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.deadline,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct TokenPermissions { address token; uint256 amount; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TokenPermissions {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TokenPermissions> for UnderlyingRustTuple<'_> {
            fn from(value: TokenPermissions) -> Self {
                (value.token, value.amount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TokenPermissions {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token: tuple.0,
                    amount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TokenPermissions {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TokenPermissions {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for TokenPermissions {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for TokenPermissions {
            const NAME: &'static str = "TokenPermissions";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TokenPermissions(address token,uint256 amount)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TokenPermissions {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISignatureTransfer`](self) contract instance.

See the [wrapper's documentation](`ISignatureTransferInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> ISignatureTransferInstance<P, N> {
        ISignatureTransferInstance::<P, N>::new(address, __provider)
    }
    /**A [`ISignatureTransfer`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ISignatureTransfer`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISignatureTransferInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ISignatureTransferInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISignatureTransferInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ISignatureTransferInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ISignatureTransfer`](self) contract instance.

See the [wrapper's documentation](`ISignatureTransferInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> ISignatureTransferInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISignatureTransferInstance<P, N> {
            ISignatureTransferInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ISignatureTransferInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ISignatureTransferInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library ISignatureTransfer {
    struct PermitTransferFrom {
        TokenPermissions permitted;
        uint256 nonce;
        uint256 deadline;
    }
    struct TokenPermissions {
        address token;
        uint256 amount;
    }
}

interface Permit2Relayer {
    error SafeERC20FailedOperation(address token);

    constructor(address permit2Address);

    function PERMIT2() external view returns (address);
    function relayTokensPermit2(bytes32 requestId, address signer, address recipient, bytes memory additionalData, ISignatureTransfer.PermitTransferFrom memory permit, bytes memory signature) external;
    function requestCrossChainSwapPermit2(address router, address signer, address tokenIn, address tokenOut, uint256 amountIn, uint256 amountOut, uint256 solverFee, uint256 dstChainId, address recipient, ISignatureTransfer.PermitTransferFrom memory permit, bytes memory signature, bytes memory additionalData) external;
    function usedRelayTokensIdentifiers(bytes32) external view returns (bool);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "permit2Address",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "PERMIT2",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IPermit2"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "relayTokensPermit2",
    "inputs": [
      {
        "name": "requestId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "additionalData",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "permit",
        "type": "tuple",
        "internalType": "struct ISignatureTransfer.PermitTransferFrom",
        "components": [
          {
            "name": "permitted",
            "type": "tuple",
            "internalType": "struct ISignatureTransfer.TokenPermissions",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "deadline",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "requestCrossChainSwapPermit2",
    "inputs": [
      {
        "name": "router",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenIn",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenOut",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amountIn",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountOut",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "solverFee",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "dstChainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "permit",
        "type": "tuple",
        "internalType": "struct ISignatureTransfer.PermitTransferFrom",
        "components": [
          {
            "name": "permitted",
            "type": "tuple",
            "internalType": "struct ISignatureTransfer.TokenPermissions",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "deadline",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "additionalData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "usedRelayTokensIdentifiers",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "error",
    "name": "SafeERC20FailedOperation",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      }
    ]
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Permit2Relayer {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a034607657601f610ba338819003918201601f19168301916001600160401b03831184841017607a57808492602094604052833981010312607657516001600160a01b03811690819003607657608052604051610b14908161008f823960805181818161023e015281816103ed015261056a0152f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f5f3560e01c8063035fddf11461041c5780636afdd850146103d7578063a2b5263d146100765763e013add514610046575f80fd5b346100735760203660031901126100735760ff604060209260043581528084522054166040519015158152f35b80fd5b5034610073576101e0366003190112610073576004356001600160a01b0381169081810361033e576100a66106bc565b6100ae6106d2565b926064356001600160a01b038116908190036103d357610104356001600160a01b03811695608435949160c435908890036103cf5736610123190197608089126103cb5760408051996101008b610744565b126103cb5760405161011181610715565b610124356001600160a01b03811681036103c757815261014435602082015289526101643560208a01526101843560408a01526101a43560018060401b0381116103c3576101639036906004016106e8565b9690936101c4356001600160401b0381116103bf576101869036906004016106e8565b93909460208d510151818c0190818d116103aa5703610351578d9a6020996101d88f8c9051015198604051996101bb8b610715565b308b528d8b01526101ca610999565b8d815191012098369161082a565b8b815191012094604051968c8801988952604088015260018060a01b03166060870152608086015260a085015260a43560c085015260e084015260e43561010084015261012083015261014082015261014081526102386101608261075f565b519020947f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316926102c9610272610969565b6102bb60098980610281610999565b6040519c868e9751918291018489015e86019068207769746e6573732960b81b83830152805192839101602983015e0101018b815261086f565b03601f19810188528761075f565b833b1561034d576102f78896928b9288946040519a8b998a9889976309be14ff60e11b8952600489016108c7565b03925af1801561034257610329575b5050905180516020909101516103269290916001600160a01b0316610a86565b80f35b816103339161075f565b61033e57825f610306565b8280fd5b6040513d84823e3d90fd5b8780fd5b60405162461bcd60e51b815260206004820152602b60248201527f5065726d697420616d6f756e74206d75737420657175616c20616d6f756e742060448201526a2b20736f6c76657246656560a81b6064820152608490fd5b50634e487b7160e01b8f52601160045260248ffd5b8c80fd5b8a80fd5b8b80fd5b8980fd5b8880fd5b8580fd5b50346100735780600319360112610073576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b503461066557610120366003190112610665576004359061043b6106bc565b6104436106d2565b906064356001600160401b038111610665576104639036906004016106e8565b3660831901959192906080871261066557604080519761048289610744565b126106655760405161049381610715565b6084356001600160a01b038116810361066557815260a4356020820152875260c435602088015260e43560408801526101043560018060401b038111610665576104e19036906004016106e8565b949091845f525f60205260ff60405f2054166106695760209461052d868b510151936040519461051086610715565b3086528886015261051f6107ae565b88815191012093369161082a565b86815191012060405191878301938452604083015260018060a01b038916606083015260808201526080815261056460a08261075f565b519020947f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316926105e761059e610782565b6102bb600989806105ad6107ae565b6040519c868e9751918291018489015e86019068207769746e6573732960b81b83830152805192839101602983015e0101015f815261086f565b833b15610665576106155f96928b9288946040519a8b998a9889976309be14ff60e11b8952600489016108c7565b03925af1801561065a57610644575b5091518051602090910151919261032692916001600160a01b0316610a86565b61032692505f6106539161075f565b5f91610624565b6040513d5f823e3d90fd5b5f80fd5b60405162461bcd60e51b815260206004820152602560248201527f546f6b656e52656c617965723a204964656e74696669657220616c7265616479604482015264081d5cd95960da1b6064820152608490fd5b602435906001600160a01b038216820361066557565b604435906001600160a01b038216820361066557565b9181601f84011215610665578235916001600160401b038311610665576020838186019501011161066557565b604081019081106001600160401b0382111761073057604052565b634e487b7160e01b5f52604160045260245ffd5b606081019081106001600160401b0382111761073057604052565b601f909101601f19168101906001600160401b0382119082101761073057604052565b6040519061079160408361075f565b600e82526d52656c617965725769746e65737360901b6020830152565b6020610827603a826107be610782565b6040519481869251918291018484015e81017f2862797465733332207265717565737449642c61646472657373207265636970838201527969656e742c6279746573206164646974696f6e616c446174612960301b60408201520301601a81018452018261075f565b90565b9192916001600160401b0382116107305760405191610853601f8201601f19166020018461075f565b829481845281830111610665578281602093845f960137010152565b7f546f6b656e5065726d697373696f6e73286164647265737320746f6b656e2c7581526d696e7432353620616d6f756e742960901b6020820152602e0190565b80516001600160a01b03168252602090810151910152565b948794602098946108fe610160999560408c966108e58c82516108af565b87810151828d0152015160608b015260808a01906108af565b6001600160a01b031660c088015260e087015261014061010087018190528151908701819052918291018787015e5f868287010152601f8019910116840193610120868287030191015281858501526101808401375f8282018401850152601f01601f191601010190565b6040519061097860408361075f565b601282527153776170526571756573745769746e65737360701b6020830152565b602061082760a0826109a9610969565b6040519481869251918291018484015e81017f286164647265737320726f757465722c6164647265737320746f6b656e496e2c838201527f6164647265737320746f6b656e4f75742c75696e7432353620616d6f756e744960408201527f6e2c75696e7432353620616d6f756e744f75742c75696e7432353620736f6c7660608201527f65724665652c75696e7432353620647374436861696e49642c6164647265737360808201527f20726563697069656e742c6279746573206164646974696f6e616c4461746129848201520301608081018452018261075f565b60405163a9059cbb60e01b60208281019182526001600160a01b03909416602483015260448083019590955293815290925f91610ac460648261075f565b519082855af11561065a575f513d610b0b57506001600160a01b0381163b155b610aeb5750565b635274afe760e01b5f9081526001600160a01b0391909116600452602490fd5b60011415610ae456
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA04`vW`\x1Fa\x0B\xA38\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`zW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12`vWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03`vW`\x80R`@Qa\x0B\x14\x90\x81a\0\x8F\x829`\x80Q\x81\x81\x81a\x02>\x01R\x81\x81a\x03\xED\x01Ra\x05j\x01R\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x03_\xDD\xF1\x14a\x04\x1CW\x80cj\xFD\xD8P\x14a\x03\xD7W\x80c\xA2\xB5&=\x14a\0vWc\xE0\x13\xAD\xD5\x14a\0FW_\x80\xFD[4a\0sW` 6`\x03\x19\x01\x12a\0sW`\xFF`@` \x92`\x045\x81R\x80\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\0sWa\x01\xE06`\x03\x19\x01\x12a\0sW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x81\x03a\x03>Wa\0\xA6a\x06\xBCV[a\0\xAEa\x06\xD2V[\x92`d5`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x03\xD3Wa\x01\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x95`\x845\x94\x91`\xC45\x90\x88\x90\x03a\x03\xCFW6a\x01#\x19\x01\x97`\x80\x89\x12a\x03\xCBW`@\x80Q\x99a\x01\0\x8Ba\x07DV[\x12a\x03\xCBW`@Qa\x01\x11\x81a\x07\x15V[a\x01$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\xC7W\x81Ra\x01D5` \x82\x01R\x89Ra\x01d5` \x8A\x01Ra\x01\x845`@\x8A\x01Ra\x01\xA45`\x01\x80`@\x1B\x03\x81\x11a\x03\xC3Wa\x01c\x906\x90`\x04\x01a\x06\xE8V[\x96\x90\x93a\x01\xC45`\x01`\x01`@\x1B\x03\x81\x11a\x03\xBFWa\x01\x86\x906\x90`\x04\x01a\x06\xE8V[\x93\x90\x94` \x8DQ\x01Q\x81\x8C\x01\x90\x81\x8D\x11a\x03\xAAW\x03a\x03QW\x8D\x9A` \x99a\x01\xD8\x8F\x8C\x90Q\x01Q\x98`@Q\x99a\x01\xBB\x8Ba\x07\x15V[0\x8BR\x8D\x8B\x01Ra\x01\xCAa\t\x99V[\x8D\x81Q\x91\x01 \x986\x91a\x08*V[\x8B\x81Q\x91\x01 \x94`@Q\x96\x8C\x88\x01\x98\x89R`@\x88\x01R`\x01\x80`\xA0\x1B\x03\x16``\x87\x01R`\x80\x86\x01R`\xA0\x85\x01R`\xA45`\xC0\x85\x01R`\xE0\x84\x01R`\xE45a\x01\0\x84\x01Ra\x01 \x83\x01Ra\x01@\x82\x01Ra\x01@\x81Ra\x028a\x01`\x82a\x07_V[Q\x90 \x94\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92a\x02\xC9a\x02ra\tiV[a\x02\xBB`\t\x89\x80a\x02\x81a\t\x99V[`@Q\x9C\x86\x8E\x97Q\x91\x82\x91\x01\x84\x89\x01^\x86\x01\x90h witness)`\xB8\x1B\x83\x83\x01R\x80Q\x92\x83\x91\x01`)\x83\x01^\x01\x01\x01\x8B\x81Ra\x08oV[\x03`\x1F\x19\x81\x01\x88R\x87a\x07_V[\x83;\x15a\x03MWa\x02\xF7\x88\x96\x92\x8B\x92\x88\x94`@Q\x9A\x8B\x99\x8A\x98\x89\x97c\t\xBE\x14\xFF`\xE1\x1B\x89R`\x04\x89\x01a\x08\xC7V[\x03\x92Z\xF1\x80\x15a\x03BWa\x03)W[PP\x90Q\x80Q` \x90\x91\x01Qa\x03&\x92\x90\x91`\x01`\x01`\xA0\x1B\x03\x16a\n\x86V[\x80\xF3[\x81a\x033\x91a\x07_V[a\x03>W\x82_a\x03\x06V[\x82\x80\xFD[`@Q=\x84\x82>=\x90\xFD[\x87\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FPermit amount must equal amount `D\x82\x01Rj+ solverFee`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[PcNH{q`\xE0\x1B\x8FR`\x11`\x04R`$\x8F\xFD[\x8C\x80\xFD[\x8A\x80\xFD[\x8B\x80\xFD[\x89\x80\xFD[\x88\x80\xFD[\x85\x80\xFD[P4a\0sW\x80`\x03\x196\x01\x12a\0sW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x06eWa\x01 6`\x03\x19\x01\x12a\x06eW`\x045\x90a\x04;a\x06\xBCV[a\x04Ca\x06\xD2V[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x06eWa\x04c\x906\x90`\x04\x01a\x06\xE8V[6`\x83\x19\x01\x95\x91\x92\x90`\x80\x87\x12a\x06eW`@\x80Q\x97a\x04\x82\x89a\x07DV[\x12a\x06eW`@Qa\x04\x93\x81a\x07\x15V[`\x845`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x06eW\x81R`\xA45` \x82\x01R\x87R`\xC45` \x88\x01R`\xE45`@\x88\x01Ra\x01\x045`\x01\x80`@\x1B\x03\x81\x11a\x06eWa\x04\xE1\x906\x90`\x04\x01a\x06\xE8V[\x94\x90\x91\x84_R_` R`\xFF`@_ T\x16a\x06iW` \x94a\x05-\x86\x8BQ\x01Q\x93`@Q\x94a\x05\x10\x86a\x07\x15V[0\x86R\x88\x86\x01Ra\x05\x1Fa\x07\xAEV[\x88\x81Q\x91\x01 \x936\x91a\x08*V[\x86\x81Q\x91\x01 `@Q\x91\x87\x83\x01\x93\x84R`@\x83\x01R`\x01\x80`\xA0\x1B\x03\x89\x16``\x83\x01R`\x80\x82\x01R`\x80\x81Ra\x05d`\xA0\x82a\x07_V[Q\x90 \x94\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92a\x05\xE7a\x05\x9Ea\x07\x82V[a\x02\xBB`\t\x89\x80a\x05\xADa\x07\xAEV[`@Q\x9C\x86\x8E\x97Q\x91\x82\x91\x01\x84\x89\x01^\x86\x01\x90h witness)`\xB8\x1B\x83\x83\x01R\x80Q\x92\x83\x91\x01`)\x83\x01^\x01\x01\x01_\x81Ra\x08oV[\x83;\x15a\x06eWa\x06\x15_\x96\x92\x8B\x92\x88\x94`@Q\x9A\x8B\x99\x8A\x98\x89\x97c\t\xBE\x14\xFF`\xE1\x1B\x89R`\x04\x89\x01a\x08\xC7V[\x03\x92Z\xF1\x80\x15a\x06ZWa\x06DW[P\x91Q\x80Q` \x90\x91\x01Q\x91\x92a\x03&\x92\x91`\x01`\x01`\xA0\x1B\x03\x16a\n\x86V[a\x03&\x92P_a\x06S\x91a\x07_V[_\x91a\x06$V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTokenRelayer: Identifier already`D\x82\x01Rd\x08\x1D\\\xD9Y`\xDA\x1B`d\x82\x01R`\x84\x90\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x06eWV[`D5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x06eWV[\x91\x81`\x1F\x84\x01\x12\x15a\x06eW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x06eW` \x83\x81\x86\x01\x95\x01\x01\x11a\x06eWV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x070W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x070W`@RV[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x070W`@RV[`@Q\x90a\x07\x91`@\x83a\x07_V[`\x0E\x82RmRelayerWitness`\x90\x1B` \x83\x01RV[` a\x08'`:\x82a\x07\xBEa\x07\x82V[`@Q\x94\x81\x86\x92Q\x91\x82\x91\x01\x84\x84\x01^\x81\x01\x7F(bytes32 requestId,address recip\x83\x82\x01Ryient,bytes additionalData)`0\x1B`@\x82\x01R\x03\x01`\x1A\x81\x01\x84R\x01\x82a\x07_V[\x90V[\x91\x92\x91`\x01`\x01`@\x1B\x03\x82\x11a\x070W`@Q\x91a\x08S`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x07_V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x06eW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x7FTokenPermissions(address token,u\x81Rmint256 amount)`\x90\x1B` \x82\x01R`.\x01\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[\x94\x87\x94` \x98\x94a\x08\xFEa\x01`\x99\x95`@\x8C\x96a\x08\xE5\x8C\x82Qa\x08\xAFV[\x87\x81\x01Q\x82\x8D\x01R\x01Q``\x8B\x01R`\x80\x8A\x01\x90a\x08\xAFV[`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x88\x01R`\xE0\x87\x01Ra\x01@a\x01\0\x87\x01\x81\x90R\x81Q\x90\x87\x01\x81\x90R\x91\x82\x91\x01\x87\x87\x01^_\x86\x82\x87\x01\x01R`\x1F\x80\x19\x91\x01\x16\x84\x01\x93a\x01 \x86\x82\x87\x03\x01\x91\x01R\x81\x85\x85\x01Ra\x01\x80\x84\x017_\x82\x82\x01\x84\x01\x85\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x01\x90V[`@Q\x90a\tx`@\x83a\x07_V[`\x12\x82RqSwapRequestWitness`p\x1B` \x83\x01RV[` a\x08'`\xA0\x82a\t\xA9a\tiV[`@Q\x94\x81\x86\x92Q\x91\x82\x91\x01\x84\x84\x01^\x81\x01\x7F(address router,address tokenIn,\x83\x82\x01R\x7Faddress tokenOut,uint256 amountI`@\x82\x01R\x7Fn,uint256 amountOut,uint256 solv``\x82\x01R\x7FerFee,uint256 dstChainId,address`\x80\x82\x01R\x7F recipient,bytes additionalData)\x84\x82\x01R\x03\x01`\x80\x81\x01\x84R\x01\x82a\x07_V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B` \x82\x81\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`$\x83\x01R`D\x80\x83\x01\x95\x90\x95R\x93\x81R\x90\x92_\x91a\n\xC4`d\x82a\x07_V[Q\x90\x82\x85Z\xF1\x15a\x06ZW_Q=a\x0B\x0BWP`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15[a\n\xEBWPV[cRt\xAF\xE7`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R`$\x90\xFD[`\x01\x14\x15a\n\xE4V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f5f3560e01c8063035fddf11461041c5780636afdd850146103d7578063a2b5263d146100765763e013add514610046575f80fd5b346100735760203660031901126100735760ff604060209260043581528084522054166040519015158152f35b80fd5b5034610073576101e0366003190112610073576004356001600160a01b0381169081810361033e576100a66106bc565b6100ae6106d2565b926064356001600160a01b038116908190036103d357610104356001600160a01b03811695608435949160c435908890036103cf5736610123190197608089126103cb5760408051996101008b610744565b126103cb5760405161011181610715565b610124356001600160a01b03811681036103c757815261014435602082015289526101643560208a01526101843560408a01526101a43560018060401b0381116103c3576101639036906004016106e8565b9690936101c4356001600160401b0381116103bf576101869036906004016106e8565b93909460208d510151818c0190818d116103aa5703610351578d9a6020996101d88f8c9051015198604051996101bb8b610715565b308b528d8b01526101ca610999565b8d815191012098369161082a565b8b815191012094604051968c8801988952604088015260018060a01b03166060870152608086015260a085015260a43560c085015260e084015260e43561010084015261012083015261014082015261014081526102386101608261075f565b519020947f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316926102c9610272610969565b6102bb60098980610281610999565b6040519c868e9751918291018489015e86019068207769746e6573732960b81b83830152805192839101602983015e0101018b815261086f565b03601f19810188528761075f565b833b1561034d576102f78896928b9288946040519a8b998a9889976309be14ff60e11b8952600489016108c7565b03925af1801561034257610329575b5050905180516020909101516103269290916001600160a01b0316610a86565b80f35b816103339161075f565b61033e57825f610306565b8280fd5b6040513d84823e3d90fd5b8780fd5b60405162461bcd60e51b815260206004820152602b60248201527f5065726d697420616d6f756e74206d75737420657175616c20616d6f756e742060448201526a2b20736f6c76657246656560a81b6064820152608490fd5b50634e487b7160e01b8f52601160045260248ffd5b8c80fd5b8a80fd5b8b80fd5b8980fd5b8880fd5b8580fd5b50346100735780600319360112610073576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b503461066557610120366003190112610665576004359061043b6106bc565b6104436106d2565b906064356001600160401b038111610665576104639036906004016106e8565b3660831901959192906080871261066557604080519761048289610744565b126106655760405161049381610715565b6084356001600160a01b038116810361066557815260a4356020820152875260c435602088015260e43560408801526101043560018060401b038111610665576104e19036906004016106e8565b949091845f525f60205260ff60405f2054166106695760209461052d868b510151936040519461051086610715565b3086528886015261051f6107ae565b88815191012093369161082a565b86815191012060405191878301938452604083015260018060a01b038916606083015260808201526080815261056460a08261075f565b519020947f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316926105e761059e610782565b6102bb600989806105ad6107ae565b6040519c868e9751918291018489015e86019068207769746e6573732960b81b83830152805192839101602983015e0101015f815261086f565b833b15610665576106155f96928b9288946040519a8b998a9889976309be14ff60e11b8952600489016108c7565b03925af1801561065a57610644575b5091518051602090910151919261032692916001600160a01b0316610a86565b61032692505f6106539161075f565b5f91610624565b6040513d5f823e3d90fd5b5f80fd5b60405162461bcd60e51b815260206004820152602560248201527f546f6b656e52656c617965723a204964656e74696669657220616c7265616479604482015264081d5cd95960da1b6064820152608490fd5b602435906001600160a01b038216820361066557565b604435906001600160a01b038216820361066557565b9181601f84011215610665578235916001600160401b038311610665576020838186019501011161066557565b604081019081106001600160401b0382111761073057604052565b634e487b7160e01b5f52604160045260245ffd5b606081019081106001600160401b0382111761073057604052565b601f909101601f19168101906001600160401b0382119082101761073057604052565b6040519061079160408361075f565b600e82526d52656c617965725769746e65737360901b6020830152565b6020610827603a826107be610782565b6040519481869251918291018484015e81017f2862797465733332207265717565737449642c61646472657373207265636970838201527969656e742c6279746573206164646974696f6e616c446174612960301b60408201520301601a81018452018261075f565b90565b9192916001600160401b0382116107305760405191610853601f8201601f19166020018461075f565b829481845281830111610665578281602093845f960137010152565b7f546f6b656e5065726d697373696f6e73286164647265737320746f6b656e2c7581526d696e7432353620616d6f756e742960901b6020820152602e0190565b80516001600160a01b03168252602090810151910152565b948794602098946108fe610160999560408c966108e58c82516108af565b87810151828d0152015160608b015260808a01906108af565b6001600160a01b031660c088015260e087015261014061010087018190528151908701819052918291018787015e5f868287010152601f8019910116840193610120868287030191015281858501526101808401375f8282018401850152601f01601f191601010190565b6040519061097860408361075f565b601282527153776170526571756573745769746e65737360701b6020830152565b602061082760a0826109a9610969565b6040519481869251918291018484015e81017f286164647265737320726f757465722c6164647265737320746f6b656e496e2c838201527f6164647265737320746f6b656e4f75742c75696e7432353620616d6f756e744960408201527f6e2c75696e7432353620616d6f756e744f75742c75696e7432353620736f6c7660608201527f65724665652c75696e7432353620647374436861696e49642c6164647265737360808201527f20726563697069656e742c6279746573206164646974696f6e616c4461746129848201520301608081018452018261075f565b60405163a9059cbb60e01b60208281019182526001600160a01b03909416602483015260448083019590955293815290925f91610ac460648261075f565b519082855af11561065a575f513d610b0b57506001600160a01b0381163b155b610aeb5750565b635274afe760e01b5f9081526001600160a01b0391909116600452602490fd5b60011415610ae456
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x03_\xDD\xF1\x14a\x04\x1CW\x80cj\xFD\xD8P\x14a\x03\xD7W\x80c\xA2\xB5&=\x14a\0vWc\xE0\x13\xAD\xD5\x14a\0FW_\x80\xFD[4a\0sW` 6`\x03\x19\x01\x12a\0sW`\xFF`@` \x92`\x045\x81R\x80\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\0sWa\x01\xE06`\x03\x19\x01\x12a\0sW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x81\x03a\x03>Wa\0\xA6a\x06\xBCV[a\0\xAEa\x06\xD2V[\x92`d5`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x03\xD3Wa\x01\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x95`\x845\x94\x91`\xC45\x90\x88\x90\x03a\x03\xCFW6a\x01#\x19\x01\x97`\x80\x89\x12a\x03\xCBW`@\x80Q\x99a\x01\0\x8Ba\x07DV[\x12a\x03\xCBW`@Qa\x01\x11\x81a\x07\x15V[a\x01$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\xC7W\x81Ra\x01D5` \x82\x01R\x89Ra\x01d5` \x8A\x01Ra\x01\x845`@\x8A\x01Ra\x01\xA45`\x01\x80`@\x1B\x03\x81\x11a\x03\xC3Wa\x01c\x906\x90`\x04\x01a\x06\xE8V[\x96\x90\x93a\x01\xC45`\x01`\x01`@\x1B\x03\x81\x11a\x03\xBFWa\x01\x86\x906\x90`\x04\x01a\x06\xE8V[\x93\x90\x94` \x8DQ\x01Q\x81\x8C\x01\x90\x81\x8D\x11a\x03\xAAW\x03a\x03QW\x8D\x9A` \x99a\x01\xD8\x8F\x8C\x90Q\x01Q\x98`@Q\x99a\x01\xBB\x8Ba\x07\x15V[0\x8BR\x8D\x8B\x01Ra\x01\xCAa\t\x99V[\x8D\x81Q\x91\x01 \x986\x91a\x08*V[\x8B\x81Q\x91\x01 \x94`@Q\x96\x8C\x88\x01\x98\x89R`@\x88\x01R`\x01\x80`\xA0\x1B\x03\x16``\x87\x01R`\x80\x86\x01R`\xA0\x85\x01R`\xA45`\xC0\x85\x01R`\xE0\x84\x01R`\xE45a\x01\0\x84\x01Ra\x01 \x83\x01Ra\x01@\x82\x01Ra\x01@\x81Ra\x028a\x01`\x82a\x07_V[Q\x90 \x94\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92a\x02\xC9a\x02ra\tiV[a\x02\xBB`\t\x89\x80a\x02\x81a\t\x99V[`@Q\x9C\x86\x8E\x97Q\x91\x82\x91\x01\x84\x89\x01^\x86\x01\x90h witness)`\xB8\x1B\x83\x83\x01R\x80Q\x92\x83\x91\x01`)\x83\x01^\x01\x01\x01\x8B\x81Ra\x08oV[\x03`\x1F\x19\x81\x01\x88R\x87a\x07_V[\x83;\x15a\x03MWa\x02\xF7\x88\x96\x92\x8B\x92\x88\x94`@Q\x9A\x8B\x99\x8A\x98\x89\x97c\t\xBE\x14\xFF`\xE1\x1B\x89R`\x04\x89\x01a\x08\xC7V[\x03\x92Z\xF1\x80\x15a\x03BWa\x03)W[PP\x90Q\x80Q` \x90\x91\x01Qa\x03&\x92\x90\x91`\x01`\x01`\xA0\x1B\x03\x16a\n\x86V[\x80\xF3[\x81a\x033\x91a\x07_V[a\x03>W\x82_a\x03\x06V[\x82\x80\xFD[`@Q=\x84\x82>=\x90\xFD[\x87\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FPermit amount must equal amount `D\x82\x01Rj+ solverFee`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[PcNH{q`\xE0\x1B\x8FR`\x11`\x04R`$\x8F\xFD[\x8C\x80\xFD[\x8A\x80\xFD[\x8B\x80\xFD[\x89\x80\xFD[\x88\x80\xFD[\x85\x80\xFD[P4a\0sW\x80`\x03\x196\x01\x12a\0sW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x06eWa\x01 6`\x03\x19\x01\x12a\x06eW`\x045\x90a\x04;a\x06\xBCV[a\x04Ca\x06\xD2V[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x06eWa\x04c\x906\x90`\x04\x01a\x06\xE8V[6`\x83\x19\x01\x95\x91\x92\x90`\x80\x87\x12a\x06eW`@\x80Q\x97a\x04\x82\x89a\x07DV[\x12a\x06eW`@Qa\x04\x93\x81a\x07\x15V[`\x845`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x06eW\x81R`\xA45` \x82\x01R\x87R`\xC45` \x88\x01R`\xE45`@\x88\x01Ra\x01\x045`\x01\x80`@\x1B\x03\x81\x11a\x06eWa\x04\xE1\x906\x90`\x04\x01a\x06\xE8V[\x94\x90\x91\x84_R_` R`\xFF`@_ T\x16a\x06iW` \x94a\x05-\x86\x8BQ\x01Q\x93`@Q\x94a\x05\x10\x86a\x07\x15V[0\x86R\x88\x86\x01Ra\x05\x1Fa\x07\xAEV[\x88\x81Q\x91\x01 \x936\x91a\x08*V[\x86\x81Q\x91\x01 `@Q\x91\x87\x83\x01\x93\x84R`@\x83\x01R`\x01\x80`\xA0\x1B\x03\x89\x16``\x83\x01R`\x80\x82\x01R`\x80\x81Ra\x05d`\xA0\x82a\x07_V[Q\x90 \x94\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92a\x05\xE7a\x05\x9Ea\x07\x82V[a\x02\xBB`\t\x89\x80a\x05\xADa\x07\xAEV[`@Q\x9C\x86\x8E\x97Q\x91\x82\x91\x01\x84\x89\x01^\x86\x01\x90h witness)`\xB8\x1B\x83\x83\x01R\x80Q\x92\x83\x91\x01`)\x83\x01^\x01\x01\x01_\x81Ra\x08oV[\x83;\x15a\x06eWa\x06\x15_\x96\x92\x8B\x92\x88\x94`@Q\x9A\x8B\x99\x8A\x98\x89\x97c\t\xBE\x14\xFF`\xE1\x1B\x89R`\x04\x89\x01a\x08\xC7V[\x03\x92Z\xF1\x80\x15a\x06ZWa\x06DW[P\x91Q\x80Q` \x90\x91\x01Q\x91\x92a\x03&\x92\x91`\x01`\x01`\xA0\x1B\x03\x16a\n\x86V[a\x03&\x92P_a\x06S\x91a\x07_V[_\x91a\x06$V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTokenRelayer: Identifier already`D\x82\x01Rd\x08\x1D\\\xD9Y`\xDA\x1B`d\x82\x01R`\x84\x90\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x06eWV[`D5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x06eWV[\x91\x81`\x1F\x84\x01\x12\x15a\x06eW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x06eW` \x83\x81\x86\x01\x95\x01\x01\x11a\x06eWV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x070W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x070W`@RV[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x070W`@RV[`@Q\x90a\x07\x91`@\x83a\x07_V[`\x0E\x82RmRelayerWitness`\x90\x1B` \x83\x01RV[` a\x08'`:\x82a\x07\xBEa\x07\x82V[`@Q\x94\x81\x86\x92Q\x91\x82\x91\x01\x84\x84\x01^\x81\x01\x7F(bytes32 requestId,address recip\x83\x82\x01Ryient,bytes additionalData)`0\x1B`@\x82\x01R\x03\x01`\x1A\x81\x01\x84R\x01\x82a\x07_V[\x90V[\x91\x92\x91`\x01`\x01`@\x1B\x03\x82\x11a\x070W`@Q\x91a\x08S`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x07_V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x06eW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x7FTokenPermissions(address token,u\x81Rmint256 amount)`\x90\x1B` \x82\x01R`.\x01\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[\x94\x87\x94` \x98\x94a\x08\xFEa\x01`\x99\x95`@\x8C\x96a\x08\xE5\x8C\x82Qa\x08\xAFV[\x87\x81\x01Q\x82\x8D\x01R\x01Q``\x8B\x01R`\x80\x8A\x01\x90a\x08\xAFV[`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x88\x01R`\xE0\x87\x01Ra\x01@a\x01\0\x87\x01\x81\x90R\x81Q\x90\x87\x01\x81\x90R\x91\x82\x91\x01\x87\x87\x01^_\x86\x82\x87\x01\x01R`\x1F\x80\x19\x91\x01\x16\x84\x01\x93a\x01 \x86\x82\x87\x03\x01\x91\x01R\x81\x85\x85\x01Ra\x01\x80\x84\x017_\x82\x82\x01\x84\x01\x85\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x01\x90V[`@Q\x90a\tx`@\x83a\x07_V[`\x12\x82RqSwapRequestWitness`p\x1B` \x83\x01RV[` a\x08'`\xA0\x82a\t\xA9a\tiV[`@Q\x94\x81\x86\x92Q\x91\x82\x91\x01\x84\x84\x01^\x81\x01\x7F(address router,address tokenIn,\x83\x82\x01R\x7Faddress tokenOut,uint256 amountI`@\x82\x01R\x7Fn,uint256 amountOut,uint256 solv``\x82\x01R\x7FerFee,uint256 dstChainId,address`\x80\x82\x01R\x7F recipient,bytes additionalData)\x84\x82\x01R\x03\x01`\x80\x81\x01\x84R\x01\x82a\x07_V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B` \x82\x81\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`$\x83\x01R`D\x80\x83\x01\x95\x90\x95R\x93\x81R\x90\x92_\x91a\n\xC4`d\x82a\x07_V[Q\x90\x82\x85Z\xF1\x15a\x06ZW_Q=a\x0B\x0BWP`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15[a\n\xEBWPV[cRt\xAF\xE7`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R`$\x90\xFD[`\x01\x14\x15a\n\xE4V",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`.
```solidity
error SafeERC20FailedOperation(address token);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SafeERC20FailedOperation {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SafeERC20FailedOperation>
        for UnderlyingRustTuple<'_> {
            fn from(value: SafeERC20FailedOperation) -> Self {
                (value.token,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SafeERC20FailedOperation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { token: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SafeERC20FailedOperation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SafeERC20FailedOperation(address)";
            const SELECTOR: [u8; 4] = [82u8, 116u8, 175u8, 231u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address permit2Address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub permit2Address: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.permit2Address,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { permit2Address: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.permit2Address,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `PERMIT2()` and selector `0x6afdd850`.
```solidity
function PERMIT2() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PERMIT2Call;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`PERMIT2()`](PERMIT2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PERMIT2Return {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<PERMIT2Call> for UnderlyingRustTuple<'_> {
                fn from(value: PERMIT2Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PERMIT2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<PERMIT2Return> for UnderlyingRustTuple<'_> {
                fn from(value: PERMIT2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PERMIT2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PERMIT2Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PERMIT2()";
            const SELECTOR: [u8; 4] = [106u8, 253u8, 216u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: PERMIT2Return = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: PERMIT2Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**Function with signature `relayTokensPermit2(bytes32,address,address,bytes,((address,uint256),uint256,uint256),bytes)` and selector `0x035fddf1`.
```solidity
function relayTokensPermit2(bytes32 requestId, address signer, address recipient, bytes memory additionalData, ISignatureTransfer.PermitTransferFrom memory permit, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct relayTokensPermit2Call {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub additionalData: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub permit: <ISignatureTransfer::PermitTransferFrom as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`relayTokensPermit2(bytes32,address,address,bytes,((address,uint256),uint256,uint256),bytes)`](relayTokensPermit2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct relayTokensPermit2Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                ISignatureTransfer::PermitTransferFrom,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
                <ISignatureTransfer::PermitTransferFrom as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<relayTokensPermit2Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: relayTokensPermit2Call) -> Self {
                    (
                        value.requestId,
                        value.signer,
                        value.recipient,
                        value.additionalData,
                        value.permit,
                        value.signature,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for relayTokensPermit2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        requestId: tuple.0,
                        signer: tuple.1,
                        recipient: tuple.2,
                        additionalData: tuple.3,
                        permit: tuple.4,
                        signature: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<relayTokensPermit2Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: relayTokensPermit2Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for relayTokensPermit2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl relayTokensPermit2Return {
            fn _tokenize(
                &self,
            ) -> <relayTokensPermit2Call as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for relayTokensPermit2Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                ISignatureTransfer::PermitTransferFrom,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = relayTokensPermit2Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "relayTokensPermit2(bytes32,address,address,bytes,((address,uint256),uint256,uint256),bytes)";
            const SELECTOR: [u8; 4] = [3u8, 95u8, 221u8, 241u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.additionalData,
                    ),
                    <ISignatureTransfer::PermitTransferFrom as alloy_sol_types::SolType>::tokenize(
                        &self.permit,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                relayTokensPermit2Return::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**Function with signature `requestCrossChainSwapPermit2(address,address,address,address,uint256,uint256,uint256,uint256,address,((address,uint256),uint256,uint256),bytes,bytes)` and selector `0xa2b5263d`.
```solidity
function requestCrossChainSwapPermit2(address router, address signer, address tokenIn, address tokenOut, uint256 amountIn, uint256 amountOut, uint256 solverFee, uint256 dstChainId, address recipient, ISignatureTransfer.PermitTransferFrom memory permit, bytes memory signature, bytes memory additionalData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestCrossChainSwapPermit2Call {
        #[allow(missing_docs)]
        pub router: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenIn: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenOut: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amountIn: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountOut: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub solverFee: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub dstChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub permit: <ISignatureTransfer::PermitTransferFrom as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub additionalData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`requestCrossChainSwapPermit2(address,address,address,address,uint256,uint256,uint256,uint256,address,((address,uint256),uint256,uint256),bytes,bytes)`](requestCrossChainSwapPermit2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestCrossChainSwapPermit2Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                ISignatureTransfer::PermitTransferFrom,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                <ISignatureTransfer::PermitTransferFrom as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<requestCrossChainSwapPermit2Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestCrossChainSwapPermit2Call) -> Self {
                    (
                        value.router,
                        value.signer,
                        value.tokenIn,
                        value.tokenOut,
                        value.amountIn,
                        value.amountOut,
                        value.solverFee,
                        value.dstChainId,
                        value.recipient,
                        value.permit,
                        value.signature,
                        value.additionalData,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestCrossChainSwapPermit2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        router: tuple.0,
                        signer: tuple.1,
                        tokenIn: tuple.2,
                        tokenOut: tuple.3,
                        amountIn: tuple.4,
                        amountOut: tuple.5,
                        solverFee: tuple.6,
                        dstChainId: tuple.7,
                        recipient: tuple.8,
                        permit: tuple.9,
                        signature: tuple.10,
                        additionalData: tuple.11,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<requestCrossChainSwapPermit2Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestCrossChainSwapPermit2Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestCrossChainSwapPermit2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl requestCrossChainSwapPermit2Return {
            fn _tokenize(
                &self,
            ) -> <requestCrossChainSwapPermit2Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestCrossChainSwapPermit2Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                ISignatureTransfer::PermitTransferFrom,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = requestCrossChainSwapPermit2Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "requestCrossChainSwapPermit2(address,address,address,address,uint256,uint256,uint256,uint256,address,((address,uint256),uint256,uint256),bytes,bytes)";
            const SELECTOR: [u8; 4] = [162u8, 181u8, 38u8, 61u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.router,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenIn,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenOut,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountIn),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountOut),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.solverFee),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.dstChainId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <ISignatureTransfer::PermitTransferFrom as alloy_sol_types::SolType>::tokenize(
                        &self.permit,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.additionalData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                requestCrossChainSwapPermit2Return::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `usedRelayTokensIdentifiers(bytes32)` and selector `0xe013add5`.
```solidity
function usedRelayTokensIdentifiers(bytes32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct usedRelayTokensIdentifiersCall(
        pub alloy::sol_types::private::FixedBytes<32>,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`usedRelayTokensIdentifiers(bytes32)`](usedRelayTokensIdentifiersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct usedRelayTokensIdentifiersReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<usedRelayTokensIdentifiersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: usedRelayTokensIdentifiersCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for usedRelayTokensIdentifiersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<usedRelayTokensIdentifiersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: usedRelayTokensIdentifiersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for usedRelayTokensIdentifiersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for usedRelayTokensIdentifiersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "usedRelayTokensIdentifiers(bytes32)";
            const SELECTOR: [u8; 4] = [224u8, 19u8, 173u8, 213u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: usedRelayTokensIdentifiersReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: usedRelayTokensIdentifiersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`Permit2Relayer`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum Permit2RelayerCalls {
        #[allow(missing_docs)]
        PERMIT2(PERMIT2Call),
        #[allow(missing_docs)]
        relayTokensPermit2(relayTokensPermit2Call),
        #[allow(missing_docs)]
        requestCrossChainSwapPermit2(requestCrossChainSwapPermit2Call),
        #[allow(missing_docs)]
        usedRelayTokensIdentifiers(usedRelayTokensIdentifiersCall),
    }
    impl Permit2RelayerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [3u8, 95u8, 221u8, 241u8],
            [106u8, 253u8, 216u8, 80u8],
            [162u8, 181u8, 38u8, 61u8],
            [224u8, 19u8, 173u8, 213u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(relayTokensPermit2),
            ::core::stringify!(PERMIT2),
            ::core::stringify!(requestCrossChainSwapPermit2),
            ::core::stringify!(usedRelayTokensIdentifiers),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <relayTokensPermit2Call as alloy_sol_types::SolCall>::SIGNATURE,
            <PERMIT2Call as alloy_sol_types::SolCall>::SIGNATURE,
            <requestCrossChainSwapPermit2Call as alloy_sol_types::SolCall>::SIGNATURE,
            <usedRelayTokensIdentifiersCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for Permit2RelayerCalls {
        const NAME: &'static str = "Permit2RelayerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 4usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::PERMIT2(_) => <PERMIT2Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::relayTokensPermit2(_) => {
                    <relayTokensPermit2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::requestCrossChainSwapPermit2(_) => {
                    <requestCrossChainSwapPermit2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::usedRelayTokensIdentifiers(_) => {
                    <usedRelayTokensIdentifiersCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<Permit2RelayerCalls>] = &[
                {
                    fn relayTokensPermit2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<Permit2RelayerCalls> {
                        <relayTokensPermit2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(Permit2RelayerCalls::relayTokensPermit2)
                    }
                    relayTokensPermit2
                },
                {
                    fn PERMIT2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<Permit2RelayerCalls> {
                        <PERMIT2Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(Permit2RelayerCalls::PERMIT2)
                    }
                    PERMIT2
                },
                {
                    fn requestCrossChainSwapPermit2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<Permit2RelayerCalls> {
                        <requestCrossChainSwapPermit2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(Permit2RelayerCalls::requestCrossChainSwapPermit2)
                    }
                    requestCrossChainSwapPermit2
                },
                {
                    fn usedRelayTokensIdentifiers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<Permit2RelayerCalls> {
                        <usedRelayTokensIdentifiersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(Permit2RelayerCalls::usedRelayTokensIdentifiers)
                    }
                    usedRelayTokensIdentifiers
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<Permit2RelayerCalls>] = &[
                {
                    fn relayTokensPermit2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<Permit2RelayerCalls> {
                        <relayTokensPermit2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(Permit2RelayerCalls::relayTokensPermit2)
                    }
                    relayTokensPermit2
                },
                {
                    fn PERMIT2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<Permit2RelayerCalls> {
                        <PERMIT2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(Permit2RelayerCalls::PERMIT2)
                    }
                    PERMIT2
                },
                {
                    fn requestCrossChainSwapPermit2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<Permit2RelayerCalls> {
                        <requestCrossChainSwapPermit2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(Permit2RelayerCalls::requestCrossChainSwapPermit2)
                    }
                    requestCrossChainSwapPermit2
                },
                {
                    fn usedRelayTokensIdentifiers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<Permit2RelayerCalls> {
                        <usedRelayTokensIdentifiersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(Permit2RelayerCalls::usedRelayTokensIdentifiers)
                    }
                    usedRelayTokensIdentifiers
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::PERMIT2(inner) => {
                    <PERMIT2Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::relayTokensPermit2(inner) => {
                    <relayTokensPermit2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::requestCrossChainSwapPermit2(inner) => {
                    <requestCrossChainSwapPermit2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::usedRelayTokensIdentifiers(inner) => {
                    <usedRelayTokensIdentifiersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::PERMIT2(inner) => {
                    <PERMIT2Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::relayTokensPermit2(inner) => {
                    <relayTokensPermit2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::requestCrossChainSwapPermit2(inner) => {
                    <requestCrossChainSwapPermit2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::usedRelayTokensIdentifiers(inner) => {
                    <usedRelayTokensIdentifiersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`Permit2Relayer`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum Permit2RelayerErrors {
        #[allow(missing_docs)]
        SafeERC20FailedOperation(SafeERC20FailedOperation),
    }
    impl Permit2RelayerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[82u8, 116u8, 175u8, 231u8]];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(SafeERC20FailedOperation),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <SafeERC20FailedOperation as alloy_sol_types::SolError>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for Permit2RelayerErrors {
        const NAME: &'static str = "Permit2RelayerErrors";
        const MIN_DATA_LENGTH: usize = 32usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::SafeERC20FailedOperation(_) => {
                    <SafeERC20FailedOperation as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<Permit2RelayerErrors>] = &[
                {
                    fn SafeERC20FailedOperation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<Permit2RelayerErrors> {
                        <SafeERC20FailedOperation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(Permit2RelayerErrors::SafeERC20FailedOperation)
                    }
                    SafeERC20FailedOperation
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<Permit2RelayerErrors>] = &[
                {
                    fn SafeERC20FailedOperation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<Permit2RelayerErrors> {
                        <SafeERC20FailedOperation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(Permit2RelayerErrors::SafeERC20FailedOperation)
                    }
                    SafeERC20FailedOperation
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::SafeERC20FailedOperation(inner) => {
                    <SafeERC20FailedOperation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::SafeERC20FailedOperation(inner) => {
                    <SafeERC20FailedOperation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Permit2Relayer`](self) contract instance.

See the [wrapper's documentation](`Permit2RelayerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> Permit2RelayerInstance<P, N> {
        Permit2RelayerInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        permit2Address: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<Permit2RelayerInstance<P, N>>,
    > {
        Permit2RelayerInstance::<P, N>::deploy(__provider, permit2Address)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        permit2Address: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        Permit2RelayerInstance::<P, N>::deploy_builder(__provider, permit2Address)
    }
    /**A [`Permit2Relayer`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Permit2Relayer`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct Permit2RelayerInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for Permit2RelayerInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("Permit2RelayerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > Permit2RelayerInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`Permit2Relayer`](self) contract instance.

See the [wrapper's documentation](`Permit2RelayerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
            permit2Address: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<Permit2RelayerInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider, permit2Address);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            __provider: P,
            permit2Address: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { permit2Address },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> Permit2RelayerInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> Permit2RelayerInstance<P, N> {
            Permit2RelayerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > Permit2RelayerInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`PERMIT2`] function.
        pub fn PERMIT2(&self) -> alloy_contract::SolCallBuilder<&P, PERMIT2Call, N> {
            self.call_builder(&PERMIT2Call)
        }
        ///Creates a new call builder for the [`relayTokensPermit2`] function.
        pub fn relayTokensPermit2(
            &self,
            requestId: alloy::sol_types::private::FixedBytes<32>,
            signer: alloy::sol_types::private::Address,
            recipient: alloy::sol_types::private::Address,
            additionalData: alloy::sol_types::private::Bytes,
            permit: <ISignatureTransfer::PermitTransferFrom as alloy::sol_types::SolType>::RustType,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, relayTokensPermit2Call, N> {
            self.call_builder(
                &relayTokensPermit2Call {
                    requestId,
                    signer,
                    recipient,
                    additionalData,
                    permit,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`requestCrossChainSwapPermit2`] function.
        pub fn requestCrossChainSwapPermit2(
            &self,
            router: alloy::sol_types::private::Address,
            signer: alloy::sol_types::private::Address,
            tokenIn: alloy::sol_types::private::Address,
            tokenOut: alloy::sol_types::private::Address,
            amountIn: alloy::sol_types::private::primitives::aliases::U256,
            amountOut: alloy::sol_types::private::primitives::aliases::U256,
            solverFee: alloy::sol_types::private::primitives::aliases::U256,
            dstChainId: alloy::sol_types::private::primitives::aliases::U256,
            recipient: alloy::sol_types::private::Address,
            permit: <ISignatureTransfer::PermitTransferFrom as alloy::sol_types::SolType>::RustType,
            signature: alloy::sol_types::private::Bytes,
            additionalData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, requestCrossChainSwapPermit2Call, N> {
            self.call_builder(
                &requestCrossChainSwapPermit2Call {
                    router,
                    signer,
                    tokenIn,
                    tokenOut,
                    amountIn,
                    amountOut,
                    solverFee,
                    dstChainId,
                    recipient,
                    permit,
                    signature,
                    additionalData,
                },
            )
        }
        ///Creates a new call builder for the [`usedRelayTokensIdentifiers`] function.
        pub fn usedRelayTokensIdentifiers(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, usedRelayTokensIdentifiersCall, N> {
            self.call_builder(&usedRelayTokensIdentifiersCall(_0))
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > Permit2RelayerInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
