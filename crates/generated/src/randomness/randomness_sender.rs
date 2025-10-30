///Module containing a contract's types and functions.
/**

```solidity
library TypesLib {
    struct RandomnessRequest { uint256 subId; uint256 directFundingFeePaid; uint32 callbackGasLimit; uint256 requestId; bytes message; bytes condition; bytes signature; uint256 nonce; address callback; }
    struct RandomnessRequestCreationParams { uint256 nonce; address callback; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod TypesLib {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct RandomnessRequest { uint256 subId; uint256 directFundingFeePaid; uint32 callbackGasLimit; uint256 requestId; bytes message; bytes condition; bytes signature; uint256 nonce; address callback; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RandomnessRequest {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub directFundingFeePaid: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub callbackGasLimit: u32,
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub message: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub condition: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub callback: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            u32,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<RandomnessRequest> for UnderlyingRustTuple<'_> {
            fn from(value: RandomnessRequest) -> Self {
                (
                    value.subId,
                    value.directFundingFeePaid,
                    value.callbackGasLimit,
                    value.requestId,
                    value.message,
                    value.condition,
                    value.signature,
                    value.nonce,
                    value.callback,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RandomnessRequest {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    subId: tuple.0,
                    directFundingFeePaid: tuple.1,
                    callbackGasLimit: tuple.2,
                    requestId: tuple.3,
                    message: tuple.4,
                    condition: tuple.5,
                    signature: tuple.6,
                    nonce: tuple.7,
                    callback: tuple.8,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RandomnessRequest {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RandomnessRequest {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.subId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.directFundingFeePaid),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.callbackGasLimit),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.message,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.condition,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.callback,
                    ),
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
        impl alloy_sol_types::SolType for RandomnessRequest {
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
        impl alloy_sol_types::SolStruct for RandomnessRequest {
            const NAME: &'static str = "RandomnessRequest";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RandomnessRequest(uint256 subId,uint256 directFundingFeePaid,uint32 callbackGasLimit,uint256 requestId,bytes message,bytes condition,bytes signature,uint256 nonce,address callback)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.subId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.directFundingFeePaid,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.callbackGasLimit,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.requestId)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.message,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.condition,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.callback,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RandomnessRequest {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.subId)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.directFundingFeePaid,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.callbackGasLimit,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestId,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.message,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.condition,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.callback,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.subId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.directFundingFeePaid,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.callbackGasLimit,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requestId,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.message,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.condition,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.callback,
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
struct RandomnessRequestCreationParams { uint256 nonce; address callback; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RandomnessRequestCreationParams {
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub callback: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<RandomnessRequestCreationParams>
        for UnderlyingRustTuple<'_> {
            fn from(value: RandomnessRequestCreationParams) -> Self {
                (value.nonce, value.callback)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for RandomnessRequestCreationParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    nonce: tuple.0,
                    callback: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RandomnessRequestCreationParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for RandomnessRequestCreationParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.callback,
                    ),
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
        impl alloy_sol_types::SolType for RandomnessRequestCreationParams {
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
        impl alloy_sol_types::SolStruct for RandomnessRequestCreationParams {
            const NAME: &'static str = "RandomnessRequestCreationParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RandomnessRequestCreationParams(uint256 nonce,address callback)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.callback,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RandomnessRequestCreationParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.callback,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.callback,
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
    /**Creates a new wrapper around an on-chain [`TypesLib`](self) contract instance.

See the [wrapper's documentation](`TypesLibInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> TypesLibInstance<P, N> {
        TypesLibInstance::<P, N>::new(address, __provider)
    }
    /**A [`TypesLib`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`TypesLib`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct TypesLibInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for TypesLibInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("TypesLibInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > TypesLibInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`TypesLib`](self) contract instance.

See the [wrapper's documentation](`TypesLibInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> TypesLibInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> TypesLibInstance<P, N> {
            TypesLibInstance {
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
    > TypesLibInstance<P, N> {
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
    > TypesLibInstance<P, N> {
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
library TypesLib {
    struct RandomnessRequest {
        uint256 subId;
        uint256 directFundingFeePaid;
        uint32 callbackGasLimit;
        uint256 requestId;
        bytes message;
        bytes condition;
        bytes signature;
        uint256 nonce;
        address callback;
    }
    struct RandomnessRequestCreationParams {
        uint256 nonce;
        address callback;
    }
}

interface RandomnessSender {
    error AccessControlBadConfirmation();
    error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
    error AddressEmptyCode(address target);
    error BalanceInvariantViolated(uint256 internalBalance, uint256 externalBalance);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error FailedCall();
    error FailedToSendNative();
    error IndexOutOfRange();
    error InsufficientBalance();
    error InvalidCalldata();
    error InvalidConsumer(uint256 subId, address consumer);
    error InvalidInitialization();
    error InvalidSubscription();
    error MustBeRequestedOwner(address proposedOwner);
    error MustBeSubOwner(address owner);
    error NotInitializing();
    error PendingRequestExists();
    error ReentrancyGuardReentrantCall();
    error TooManyConsumers();
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);

    event ConfigSet(uint32 maxGasLimit, uint32 gasAfterPaymentCalculation, uint32 fulfillmentFlatFeeNativePPM, uint32 weiPerUnitGas, uint32 blsPairingCheckOverhead, uint8 nativePremiumPercentage, uint32 gasForCallExactCheck);
    event Disabled();
    event Enabled();
    event Initialized(uint64 version);
    event L1GasFee(uint256 fee);
    event RandomnessCallbackFailed(uint256 indexed requestID);
    event RandomnessCallbackSuccess(uint256 indexed requestID, bytes32 randomness, bytes signature);
    event RandomnessRequested(uint256 indexed requestID, uint256 indexed nonce, address indexed requester, uint256 requestedAt);
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event SignatureSenderUpdated(address indexed signatureSender);
    event SubscriptionCanceled(uint256 indexed subId, address to, uint256 amountNative);
    event SubscriptionConsumerAdded(uint256 indexed subId, address consumer);
    event SubscriptionConsumerRemoved(uint256 indexed subId, address consumer);
    event SubscriptionCreated(uint256 indexed subId, address owner);
    event SubscriptionFundedWithNative(uint256 indexed subId, uint256 oldNativeBalance, uint256 newNativeBalance);
    event SubscriptionOwnerTransferRequested(uint256 indexed subId, address from, address to);
    event SubscriptionOwnerTransferred(uint256 indexed subId, address from, address to);
    event Upgraded(address indexed implementation);

    constructor();

    function ADMIN_ROLE() external view returns (bytes32);
    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function MAX_CONSUMERS() external view returns (uint16);
    function SCHEME_ID() external view returns (string memory);
    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function acceptSubscriptionOwnerTransfer(uint256 subId) external;
    function addConsumer(uint256 subId, address consumer) external;
    function calculateRequestPriceNative(uint32 _callbackGasLimit) external view returns (uint256);
    function cancelSubscription(uint256 subId, address to) external;
    function createSubscription() external returns (uint256 subId);
    function disable() external;
    function enable() external;
    function estimateRequestPriceNative(uint32 _callbackGasLimit, uint256 _requestGasPriceWei) external view returns (uint256);
    function fundSubscriptionWithNative(uint256 subId) external payable;
    function getActiveSubscriptionIds(uint256 startIndex, uint256 maxCount) external view returns (uint256[] memory ids);
    function getAllRequests() external view returns (TypesLib.RandomnessRequest[] memory);
    function getConfig() external view returns (uint32 maxGasLimit, uint32 gasAfterPaymentCalculation, uint32 fulfillmentFlatFeeNativePPM, uint32 weiPerUnitGas, uint32 blsPairingCheckOverhead, uint8 nativePremiumPercentage, uint32 gasForCallExactCheck);
    function getRequest(uint256 requestId) external view returns (TypesLib.RandomnessRequest memory);
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    function getRoleMember(bytes32 role, uint256 index) external view returns (address);
    function getRoleMemberCount(bytes32 role) external view returns (uint256);
    function getRoleMembers(bytes32 role) external view returns (address[] memory);
    function getSubscription(uint256 subId) external view returns (uint96 nativeBalance, uint64 reqCount, address subOwner, address[] memory consumers);
    function grantRole(bytes32 role, address account) external;
    function hasRole(bytes32 role, address account) external view returns (bool);
    function initialize(address _signatureSender, address owner) external;
    function isInFlight(uint256 requestID) external view returns (bool);
    function messageFrom(TypesLib.RandomnessRequestCreationParams memory r) external pure returns (bytes memory);
    function nonce() external view returns (uint256);
    function ownerCancelSubscription(uint256 subId) external;
    function pendingRequestExists(uint256 subId) external view returns (bool);
    function proxiableUUID() external view returns (bytes32);
    function receiveSignature(uint256 requestID, bytes memory signature) external;
    function removeConsumer(uint256 subId, address consumer) external;
    function renounceRole(bytes32 role, address callerConfirmation) external;
    function requestRandomness(uint32 callbackGasLimit) external payable returns (uint256 requestID);
    function requestRandomnessWithSubscription(uint32 callbackGasLimit, uint256 subId) external payable returns (uint256 requestID);
    function requestSubscriptionOwnerTransfer(uint256 subId, address newOwner) external;
    function revokeRole(bytes32 role, address account) external;
    function s_config() external view returns (uint32 maxGasLimit, uint32 gasAfterPaymentCalculation, uint32 fulfillmentFlatFeeNativePPM, uint32 weiPerUnitGas, uint32 blsPairingCheckOverhead, uint8 nativePremiumPercentage, uint32 gasForCallExactCheck);
    function s_configured() external view returns (bool);
    function s_currentSubNonce() external view returns (uint64);
    function s_disabled() external view returns (bool);
    function s_totalNativeBalance() external view returns (uint96);
    function s_withdrawableDirectFundingFeeNative() external view returns (uint96);
    function s_withdrawableSubscriptionFeeNative() external view returns (uint96);
    function setConfig(uint32 maxGasLimit, uint32 gasAfterPaymentCalculation, uint32 fulfillmentFlatFeeNativePPM, uint32 weiPerUnitGas, uint32 blsPairingCheckOverhead, uint8 nativePremiumPercentage, uint32 gasForCallExactCheck) external;
    function setSignatureSender(address newSignatureSender) external;
    function signatureSender() external view returns (address);
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
    function version() external pure returns (string memory);
    function withdrawDirectFundingFeesNative(address payable recipient) external;
    function withdrawSubscriptionFeesNative(address payable recipient) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "ADMIN_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "DEFAULT_ADMIN_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "MAX_CONSUMERS",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "SCHEME_ID",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "UPGRADE_INTERFACE_VERSION",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "acceptSubscriptionOwnerTransfer",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addConsumer",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "consumer",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "calculateRequestPriceNative",
    "inputs": [
      {
        "name": "_callbackGasLimit",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "cancelSubscription",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createSubscription",
    "inputs": [],
    "outputs": [
      {
        "name": "subId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "disable",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "enable",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "estimateRequestPriceNative",
    "inputs": [
      {
        "name": "_callbackGasLimit",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_requestGasPriceWei",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "fundSubscriptionWithNative",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "getActiveSubscriptionIds",
    "inputs": [
      {
        "name": "startIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "maxCount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "ids",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAllRequests",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct TypesLib.RandomnessRequest[]",
        "components": [
          {
            "name": "subId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "directFundingFeePaid",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "callbackGasLimit",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "requestId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "message",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "condition",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "callback",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getConfig",
    "inputs": [],
    "outputs": [
      {
        "name": "maxGasLimit",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "gasAfterPaymentCalculation",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "fulfillmentFlatFeeNativePPM",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "weiPerUnitGas",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "blsPairingCheckOverhead",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "nativePremiumPercentage",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "gasForCallExactCheck",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRequest",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct TypesLib.RandomnessRequest",
        "components": [
          {
            "name": "subId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "directFundingFeePaid",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "callbackGasLimit",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "requestId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "message",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "condition",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "callback",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleAdmin",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleMember",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleMemberCount",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleMembers",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getSubscription",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "nativeBalance",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "reqCount",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "subOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "consumers",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "grantRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "hasRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
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
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_signatureSender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isInFlight",
    "inputs": [
      {
        "name": "requestID",
        "type": "uint256",
        "internalType": "uint256"
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
    "type": "function",
    "name": "messageFrom",
    "inputs": [
      {
        "name": "r",
        "type": "tuple",
        "internalType": "struct TypesLib.RandomnessRequestCreationParams",
        "components": [
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "callback",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "nonce",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ownerCancelSubscription",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "pendingRequestExists",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "internalType": "uint256"
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
    "type": "function",
    "name": "proxiableUUID",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "receiveSignature",
    "inputs": [
      {
        "name": "requestID",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "removeConsumer",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "consumer",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "callerConfirmation",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "requestRandomness",
    "inputs": [
      {
        "name": "callbackGasLimit",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "requestID",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "requestRandomnessWithSubscription",
    "inputs": [
      {
        "name": "callbackGasLimit",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "subId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "requestID",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "requestSubscriptionOwnerTransfer",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "revokeRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "s_config",
    "inputs": [],
    "outputs": [
      {
        "name": "maxGasLimit",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "gasAfterPaymentCalculation",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "fulfillmentFlatFeeNativePPM",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "weiPerUnitGas",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "blsPairingCheckOverhead",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "nativePremiumPercentage",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "gasForCallExactCheck",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "s_configured",
    "inputs": [],
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
    "type": "function",
    "name": "s_currentSubNonce",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "s_disabled",
    "inputs": [],
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
    "type": "function",
    "name": "s_totalNativeBalance",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "s_withdrawableDirectFundingFeeNative",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "s_withdrawableSubscriptionFeeNative",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setConfig",
    "inputs": [
      {
        "name": "maxGasLimit",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "gasAfterPaymentCalculation",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "fulfillmentFlatFeeNativePPM",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "weiPerUnitGas",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "blsPairingCheckOverhead",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "nativePremiumPercentage",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "gasForCallExactCheck",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setSignatureSender",
    "inputs": [
      {
        "name": "newSignatureSender",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "signatureSender",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISignatureSender"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "supportsInterface",
    "inputs": [
      {
        "name": "interfaceId",
        "type": "bytes4",
        "internalType": "bytes4"
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
    "type": "function",
    "name": "upgradeToAndCall",
    "inputs": [
      {
        "name": "newImplementation",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "version",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "withdrawDirectFundingFeesNative",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address payable"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawSubscriptionFeesNative",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address payable"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "ConfigSet",
    "inputs": [
      {
        "name": "maxGasLimit",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "gasAfterPaymentCalculation",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "fulfillmentFlatFeeNativePPM",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "weiPerUnitGas",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "blsPairingCheckOverhead",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "nativePremiumPercentage",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      },
      {
        "name": "gasForCallExactCheck",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Disabled",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Enabled",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "L1GasFee",
    "inputs": [
      {
        "name": "fee",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RandomnessCallbackFailed",
    "inputs": [
      {
        "name": "requestID",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RandomnessCallbackSuccess",
    "inputs": [
      {
        "name": "requestID",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "randomness",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "signature",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RandomnessRequested",
    "inputs": [
      {
        "name": "requestID",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "nonce",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "requester",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "requestedAt",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleAdminChanged",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "previousAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "newAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleGranted",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleRevoked",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SignatureSenderUpdated",
    "inputs": [
      {
        "name": "signatureSender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SubscriptionCanceled",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "amountNative",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SubscriptionConsumerAdded",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "consumer",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SubscriptionConsumerRemoved",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "consumer",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SubscriptionCreated",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "owner",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SubscriptionFundedWithNative",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "oldNativeBalance",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newNativeBalance",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SubscriptionOwnerTransferRequested",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "from",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SubscriptionOwnerTransferred",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "from",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Upgraded",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AccessControlBadConfirmation",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AccessControlUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "neededRole",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "AddressEmptyCode",
    "inputs": [
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "BalanceInvariantViolated",
    "inputs": [
      {
        "name": "internalBalance",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "externalBalance",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC1967InvalidImplementation",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC1967NonPayable",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FailedCall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FailedToSendNative",
    "inputs": []
  },
  {
    "type": "error",
    "name": "IndexOutOfRange",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientBalance",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidCalldata",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidConsumer",
    "inputs": [
      {
        "name": "subId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "consumer",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidInitialization",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSubscription",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MustBeRequestedOwner",
    "inputs": [
      {
        "name": "proposedOwner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "MustBeSubOwner",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotInitializing",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PendingRequestExists",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ReentrancyGuardReentrantCall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TooManyConsumers",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UUPSUnauthorizedCallContext",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UUPSUnsupportedProxiableUUID",
    "inputs": [
      {
        "name": "slot",
        "type": "bytes32",
        "internalType": "bytes32"
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
pub mod RandomnessSender {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a0604052306080525f600c55348015610017575f5ffd5b5060015f55610024610029565b6100db565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805468010000000000000000900460ff16156100795760405163f92ee8a960e01b815260040160405180910390fd5b80546001600160401b03908116146100d85780546001600160401b0319166001600160401b0390811782556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50565b6080516159386101015f395f8181613a1d01528181613a460152613c8301526159385ff3fe608060405260043610610345575f3560e01c806391d14854116101b2578063b2a7cac5116100f2578063cb63179711610092578063dac83d291161006d578063dac83d2914610c09578063dc311dd314610c28578063f8fa0d6614610c57578063fb1a002a14610c76575f5ffd5b8063cb63179714610bac578063cd802c9114610bcb578063d547741f14610bea575f5ffd5b8063c3f909d4116100cd578063c3f909d414610aaa578063c58343ef14610b42578063c8db658214610b6e578063ca15c87314610b8d575f5ffd5b8063b2a7cac514610a4d578063bd18636b14610a6c578063bec4c08c14610a8b575f5ffd5b8063a3246ad31161015d578063aa433aff11610138578063aa433aff146109a5578063ad3cb1cc146109c4578063aefb212f14610a0c578063affed0e014610a38575f5ffd5b8063a3246ad314610947578063a3907d7114610973578063a608a1e114610987575f5ffd5b80639d40a6fd1161018d5780639d40a6fd146108e7578063a217fddf14610920578063a21a23e414610933575f5ffd5b806391d148541461084057806395b55cfc146108b0578063995cb36e146108c3575f5ffd5b8063485cc9551161028857806364d51a2a116102285780637d468106116102035780637d46810614610775578063811ee32a146107c65780638a1f165a146107d95780639010d07c14610821575f5ffd5b806364d51a2a146106fc57806375b238fc14610723578063775b839c14610756575f5ffd5b806352d1902d1161026357806352d1902d1461066257806354236fb31461067657806354fd4d501461069557806357a8070a146106e3575f5ffd5b8063485cc955146106115780634b160935146106305780634f1ef2861461064f575f5ffd5b80632f2770db116102f357806336568abe116102ce57806336568abe146105805780633bc32c751461059f57806341af6c87146105d357806345fa4354146105f2575f5ffd5b80632f2770db1461052e5780632f2ff15d146105425780633255c45614610561575f5ffd5b806318e3dd271161032357806318e3dd271461047f5780631da53c9f146104c0578063248a9ca3146104e1575f5ffd5b806301ffc9a714610349578063088070f51461037d5780630ae095401461045e575b5f5ffd5b348015610354575f5ffd5b50610368610363366004614cc9565b610c97565b60405190151581526020015b60405180910390f35b348015610388575f5ffd5b50600a546104179063ffffffff8082169164010000000081048216916801000000000000000082048116916c010000000000000000000000008104821691700100000000000000000000000000000000820481169160ff740100000000000000000000000000000000000000008204169175010000000000000000000000000000000000000000009091041687565b6040805163ffffffff988916815296881660208801529487169486019490945291851660608501528416608084015260ff1660a083015290911660c082015260e001610374565b348015610469575f5ffd5b5061047d610478366004614d29565b610cf2565b005b34801561048a575f5ffd5b506008546104a3906bffffffffffffffffffffffff1681565b6040516bffffffffffffffffffffffff9091168152602001610374565b6104d36104ce366004614d6f565b610d5c565b604051908152602001610374565b3480156104ec575f5ffd5b506104d36104fb366004614d97565b5f9081527f02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b626800602052604090206001015490565b348015610539575f5ffd5b5061047d6112c0565b34801561054d575f5ffd5b5061047d61055c366004614d29565b61133f565b34801561056c575f5ffd5b506104d361057b366004614d6f565b611388565b34801561058b575f5ffd5b5061047d61059a366004614d29565b6113a0565b3480156105aa575f5ffd5b506008546104a3906c0100000000000000000000000090046bffffffffffffffffffffffff1681565b3480156105de575f5ffd5b506103686105ed366004614d97565b6113f9565b3480156105fd575f5ffd5b5061047d61060c366004614dae565b6114a2565b34801561061c575f5ffd5b5061047d61062b366004614e36565b61175d565b34801561063b575f5ffd5b506104d361064a366004614e62565b611aad565b61047d61065d366004614ef7565b611abe565b34801561066d575f5ffd5b506104d3611add565b348015610681575f5ffd5b5061047d610690366004614fbc565b611b0c565b3480156106a0575f5ffd5b5060408051808201909152600581527f302e302e3100000000000000000000000000000000000000000000000000000060208201525b6040516103749190615023565b3480156106ee575f5ffd5b50600b546103689060ff1681565b348015610707575f5ffd5b50610710606481565b60405161ffff9091168152602001610374565b34801561072e575f5ffd5b506104d37fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c2177581565b348015610761575f5ffd5b506106d6610770366004615035565b611bb6565b348015610780575f5ffd5b506001546107a19073ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff9091168152602001610374565b6104d36107d4366004614e62565b611c1f565b3480156107e4575f5ffd5b506106d66040518060400160405280600581526020017f424e32353400000000000000000000000000000000000000000000000000000081525081565b34801561082c575f5ffd5b506107a161083b36600461508b565b611d09565b34801561084b575f5ffd5b5061036861085a366004614d29565b5f9182527f02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b6268006020908152604080842073ffffffffffffffffffffffffffffffffffffffff93909316845291905290205460ff1690565b61047d6108be366004614d97565b611d49565b3480156108ce575f5ffd5b506009546104a3906bffffffffffffffffffffffff1681565b3480156108f2575f5ffd5b506005546109079067ffffffffffffffff1681565b60405167ffffffffffffffff9091168152602001610374565b34801561092b575f5ffd5b506104d35f81565b34801561093e575f5ffd5b506104d3611e7f565b348015610952575f5ffd5b50610966610961366004614d97565b6120c3565b60405161037491906150fb565b34801561097e575f5ffd5b5061047d6120ff565b348015610992575f5ffd5b50600b5461036890610100900460ff1681565b3480156109b0575f5ffd5b5061047d6109bf366004614d97565b61217a565b3480156109cf575f5ffd5b506106d66040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b348015610a17575f5ffd5b50610a2b610a2636600461508b565b6121da565b604051610374919061510d565b348015610a43575f5ffd5b506104d3600c5481565b348015610a58575f5ffd5b5061047d610a67366004614d97565b6122e9565b348015610a77575f5ffd5b5061047d610a86366004614fbc565b612439565b348015610a96575f5ffd5b5061047d610aa5366004614d29565b612516565b348015610ab5575f5ffd5b50600a5463ffffffff8082169164010000000081048216916801000000000000000082048116916c010000000000000000000000008104821691700100000000000000000000000000000000820481169160ff7401000000000000000000000000000000000000000082041691750100000000000000000000000000000000000000000090910416610417565b348015610b4d575f5ffd5b50610b61610b5c366004614d97565b612691565b604051610374919061520c565b348015610b79575f5ffd5b5061047d610b8836600461521e565b612925565b348015610b98575f5ffd5b506104d3610ba7366004614d97565b6129b1565b348015610bb7575f5ffd5b5061047d610bc6366004614d29565b6129e8565b348015610bd6575f5ffd5b50610368610be5366004614d97565b612cc3565b348015610bf5575f5ffd5b5061047d610c04366004614d29565b612d54565b348015610c14575f5ffd5b5061047d610c23366004614d29565b612d97565b348015610c33575f5ffd5b50610c47610c42366004614d97565b612e66565b6040516103749493929190615295565b348015610c62575f5ffd5b5061047d610c71366004614fbc565b612f4e565b348015610c81575f5ffd5b50610c8a612fe5565b60405161037491906152f1565b5f7fffffffff0000000000000000000000000000000000000000000000000000000082167f5a05180f000000000000000000000000000000000000000000000000000000001480610cec5750610cec82613255565b92915050565b81610cfc816132eb565b610d0461337f565b610d0d836113f9565b15610d44576040517fb42f66e800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d4e83836133c0565b610d5760015f55565b505050565b600b545f9060ff16610dcf576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601a60248201527f436f6e7472616374206973206e6f7420636f6e6669677572656400000000000060448201526064015b60405180910390fd5b600b54610100900460ff1615610e41576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601460248201527f436f6e74726163742069732064697361626c65640000000000000000000000006044820152606401610dc6565b81151580610e4e57505f34115b610eda576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603860248201527f4469726563742066756e64696e6720726571756972656420666f72207265717560448201527f6573742066756c66696c6c6d656e742063616c6c6261636b00000000000000006064820152608401610dc6565b610ee48383613447565b6001600c5f828254610ef6919061539f565b909155505060408051610120810182528381523460208083019190915263ffffffff8616828401525f606083018190528351808301855281815260808401528351808301855281815260a08401528351808301855281815260c0840152600c5460e084018190523361010085018190528551808701909652908552918401919091529091610f8390611bb6565b60408051602080820183525f82528251808401909352600583527f424e32353400000000000000000000000000000000000000000000000000000090830152919250610fd0908383613743565b606084018181526080850184815260a086018490525f838152600d602090815260409182902088518155908801516001820155908701516002820180547fffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000001663ffffffff909216919091179055915160038301555191955084916004820190611059908261544e565b5060a0820151600582019061106e908261544e565b5060c08201516006820190611083908261544e565b5060e0820151600782015561010090910151600890910180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff909216919091179055600e80546001810182555f9190915283517fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c3fd600990920291820190815560208501517fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c3fe83015560408501517fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c3ff830180547fffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000001663ffffffff90921691909117905560608501517fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c400830155608085015185927fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c40101906111f9908261544e565b5060a0820151600582019061120e908261544e565b5060c08201516006820190611223908261544e565b5060e0820151600782015561010090910151600890910180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff909216919091179055600c5460405142815233919086907feee7195b6cee0fa7044c3af0b86fe2febb1d2703d71191f44052ba0d60ffda649060200160405180910390a450505092915050565b6112e97fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b600b80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff166101001790556040517f75884cdadc4a89e8b545db800057f06ec7f5338a08183c7ba515f2bfdd9fe1e1905f90a1565b5f8281527f02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b6268006020526040902060010154611378816137dd565b61138283836137e7565b50505050565b5f6113998363ffffffff168361383c565b9392505050565b73ffffffffffffffffffffffffffffffffffffffff811633146113ef576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d578282613987565b5f8181526003602052604081206002018054825b81811015611498575f60025f85848154811061142b5761142b615565565b5f91825260208083209091015473ffffffffffffffffffffffffffffffffffffffff168352828101939093526040918201812089825290925290205467ffffffffffffffff690100000000000000000090910416111561149057506001949350505050565b60010161140d565b505f949350505050565b6114cb7fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b60ff8216609b11611538576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601a60248201527f496e76616c6964205072656d69756d2050657263656e746167650000000000006044820152606401610dc6565b6040805160e0808201835263ffffffff8a81168084528a821660208086018290528b84168688018190528b851660608089018290528c87166080808b0182905260ff8e1660a0808d01829052998e1660c09c8d01819052600a80547fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000168b176401000000008b02177fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff166801000000000000000089027fffffffffffffffffffffffffffffffff00000000ffffffffffffffffffffffff16176c010000000000000000000000008802177fffffffffffffffffffffff0000000000ffffffffffffffffffffffffffffffff1670010000000000000000000000000000000086027fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff1617740100000000000000000000000000000000000000008402177fffffffffffffff00000000ffffffffffffffffffffffffffffffffffffffffff1675010000000000000000000000000000000000000000008302179055600b80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660011790558d51998a52968901979097529a8701939093528501529683019690965291810191909152918201929092527f55a28fde295f482c9f32d670c116103bca15724bcef4f18b35542e0553c35ad591015b60405180910390a150505050505050565b5f6117666139d3565b805490915060ff68010000000000000000820416159067ffffffffffffffff165f811580156117925750825b90505f8267ffffffffffffffff1660011480156117ae5750303b155b9050811580156117bc575080155b156117f3576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b84547fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000016600117855583156118545784547fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff16680100000000000000001785555b61185c6139fb565b6118646139fb565b61188e7fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c21775876137e7565b6118f4576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f4772616e7420726f6c65206661696c65640000000000000000000000000000006044820152606401610dc6565b6118fe5f876137e7565b611964576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f4772616e7420726f6c65206661696c65640000000000000000000000000000006044820152606401610dc6565b73ffffffffffffffffffffffffffffffffffffffff8716611a07576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602b60248201527f43616e6e6f7420736574207a65726f2061646472657373206173207369676e6160448201527f747572652073656e6465720000000000000000000000000000000000000000006064820152608401610dc6565b600180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff89161790558315611aa45784547fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff168555604051600181527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200161174c565b50505050505050565b5f610cec8263ffffffff163a61383c565b611ac6613a05565b611acf82613b09565b611ad98282613b32565b5050565b5f611ae6613c6b565b507f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5b90565b611b1461337f565b611b3d7fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b6008546c0100000000000000000000000090046bffffffffffffffffffffffff16611b69811515613cda565b600880547fffffffffffffffff000000000000000000000000ffffffffffffffffffffffff169055611ba9826bffffffffffffffffffffffff8316613d11565b50611bb360015f55565b50565b8051604051606091611bce9160200190815260200190565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152828252805160209182012090830152016040516020818303038152906040529050919050565b600b545f9060ff16611c8d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601a60248201527f436f6e7472616374206973206e6f7420636f6e666967757265640000000000006044820152606401610dc6565b600b54610100900460ff1615611cff576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601460248201527f436f6e74726163742069732064697361626c65640000000000000000000000006044820152606401610dc6565b610cec825f610d5c565b5f8281527fc1f6fe24621ce81ec5827caf0253cadb74709b061630e6b55e82371705932000602081905260408220611d419084613da7565b949350505050565b611d5161337f565b5f81815260036020526040902054611d7e9073ffffffffffffffffffffffffffffffffffffffff16613db2565b5f81815260046020526040812080546bffffffffffffffffffffffff1691349190611da98385615592565b92506101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055503460085f8282829054906101000a90046bffffffffffffffffffffffff16611dff9190615592565b92506101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550817f7603b205d03651ee812f803fccde89f1012e545a9c99f0abfea9cedd0fd8e902823484611e5c919061539f565b604080519283526020830191909152015b60405180910390a250611bb360015f55565b5f611e8861337f565b60055467ffffffffffffffff1633611ea16001436155b6565b6040517fffffffffffffffffffffffffffffffffffffffff000000000000000000000000606093841b81166020830152914060348201523090921b1660548201527fffffffffffffffff00000000000000000000000000000000000000000000000060c083901b166068820152607001604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081840301815291905280516020909101209150611f548160016155c9565b600580547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff928316179055604080515f808252606080830184526020808401838152848601848152898552600483528685209151825491516bffffffffffffffffffffffff9091167fffffffffffffffffffffffff0000000000000000000000000000000000000000928316176c01000000000000000000000000919099160297909717905584519182018552338252818101838152828601858152898552600383529590932082518154881673ffffffffffffffffffffffffffffffffffffffff918216178255935160018201805490981694169390931790955592518051929491926120729260028501920190614c16565b5061208291506006905084613dff565b5060405133815283907f1d3015d7ba850fa198dc7b1a3f5d42779313a681035f77c8c03764c61005518d9060200160405180910390a25050611b0960015f55565b5f8181527fc1f6fe24621ce81ec5827caf0253cadb74709b061630e6b55e82371705932000602081905260409091206060919061139990613e0a565b6121287fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b600b80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff1690556040517fc0f961051f97b04c496472d11cb6170d844e4b2c9dfd3b602a4fa0139712d484905f90a1565b6121a37fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b5f8181526003602052604090205473ffffffffffffffffffffffffffffffffffffffff166121d081613db2565b611ad982826133c0565b60605f6121e76006613e16565b9050808410612222576040517f1390f2a100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61222d848661539f565b90508181118061223b575083155b6122455780612247565b815b90505f61225486836155b6565b90508067ffffffffffffffff81111561226f5761226f614e7b565b604051908082528060200260200182016040528015612298578160200160208202803683370190505b5093505f5b818110156122df576122ba6122b2888361539f565b600690613da7565b8582815181106122cc576122cc615565565b602090810291909101015260010161229d565b5050505092915050565b6122f161337f565b5f8181526003602052604090205473ffffffffffffffffffffffffffffffffffffffff1661231e81613db2565b5f8281526003602052604090206001015473ffffffffffffffffffffffffffffffffffffffff1633146123a8575f82815260036020526040908190206001015490517fd084e97500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff9091166004820152602401610dc6565b5f828152600360209081526040918290208054337fffffffffffffffffffffffff000000000000000000000000000000000000000091821681178355600190920180549091169055825173ffffffffffffffffffffffffffffffffffffffff851681529182015283917fd4114ab6e9af9f597c52041f32d62dc57c5c4e4c0d4427006069635e216c93869101611e6d565b61244161337f565b61246a7fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b6009546bffffffffffffffffffffffff16612486811515613cda565b600980547fffffffffffffffffffffffffffffffffffffffff000000000000000000000000169055600880548291905f906124d09084906bffffffffffffffffffffffff166155e9565b92506101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550611ba982826bffffffffffffffffffffffff16613d11565b81612520816132eb565b61252861337f565b73ffffffffffffffffffffffffffffffffffffffff82165f9081526002602090815260408083208684529091529020805460ff16156125675750610d4e565b5f84815260036020526040902060020180547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9c016125d1576040517f05a48e0f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b815460017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0090911681178355815490810182555f82815260209081902090910180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff871690811790915560405190815286917f1e980d04aa7648e205713e5e8ea3808672ac163d10936d36f91b2c88ac1575e191015b60405180910390a25050610d5760015f55565b6126f66040518061012001604052805f81526020015f81526020015f63ffffffff1681526020015f81526020016060815260200160608152602001606081526020015f81526020015f73ffffffffffffffffffffffffffffffffffffffff1681525090565b5f828152600d602090815260409182902082516101208101845281548152600182015492810192909252600281015463ffffffff169282019290925260038201546060820152600482018054919291608084019190612754906153b2565b80601f0160208091040260200160405190810160405280929190818152602001828054612780906153b2565b80156127cb5780601f106127a2576101008083540402835291602001916127cb565b820191905f5260205f20905b8154815290600101906020018083116127ae57829003601f168201915b505050505081526020016005820180546127e4906153b2565b80601f0160208091040260200160405190810160405280929190818152602001828054612810906153b2565b801561285b5780601f106128325761010080835404028352916020019161285b565b820191905f5260205f20905b81548152906001019060200180831161283e57829003601f168201915b50505050508152602001600682018054612874906153b2565b80601f01602080910402602001604051908101604052809291908181526020018280546128a0906153b2565b80156128eb5780601f106128c2576101008083540402835291602001916128eb565b820191905f5260205f20905b8154815290600101906020018083116128ce57829003601f168201915b50505091835250506007820154602082015260089091015473ffffffffffffffffffffffffffffffffffffffff1660409091015292915050565b60015473ffffffffffffffffffffffffffffffffffffffff1633146129a6576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601d60248201527f4f6e6c79207369676e617475726553656e6465722063616e2063616c6c0000006044820152606401610dc6565b610d57838383613e1f565b5f8181527fc1f6fe24621ce81ec5827caf0253cadb74709b061630e6b55e8237170593200060208190526040822061139990613e16565b816129f2816132eb565b6129fa61337f565b612a03836113f9565b15612a3a576040517fb42f66e800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff82165f90815260026020908152604080832086845290915290205460ff16612ac2576040517f79bfd4010000000000000000000000000000000000000000000000000000000081526004810184905273ffffffffffffffffffffffffffffffffffffffff83166024820152604401610dc6565b5f838152600360205260408120600201805490915b81811015612c3d578473ffffffffffffffffffffffffffffffffffffffff16838281548110612b0857612b08615565565b5f9182526020909120015473ffffffffffffffffffffffffffffffffffffffff1603612c355782612b3a6001846155b6565b81548110612b4a57612b4a615565565b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16838281548110612b8457612b84615565565b905f5260205f20015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555082805480612bd957612bd961560d565b5f8281526020902081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff90810180547fffffffffffffffffffffffff0000000000000000000000000000000000000000169055019055612c3d565b600101612ad7565b5073ffffffffffffffffffffffffffffffffffffffff84165f81815260026020908152604080832089845282529182902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00169055905191825286917f32158c6058347c1601b2d12bc696ac6901d8a9a9aa3ba10c27ab0a983e8425a7910161267e565b6001546040517fcd802c91000000000000000000000000000000000000000000000000000000008152600481018390525f9173ffffffffffffffffffffffffffffffffffffffff169063cd802c9190602401602060405180830381865afa158015612d30573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cec919061563a565b5f8281527f02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b6268006020526040902060010154612d8d816137dd565b6113828383613987565b81612da1816132eb565b612da961337f565b5f838152600360205260409020600181015473ffffffffffffffffffffffffffffffffffffffff848116911614612e5c576001810180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff851690811790915560408051338152602081019290925285917f21a4dad170a6bf476c31bbcf4a16628295b0e450672eec25d7c93308e05344a1910160405180910390a25b50610d5760015f55565b5f81815260036020526040812054819073ffffffffffffffffffffffffffffffffffffffff166060612e9782613db2565b5f85815260046020908152604080832054600383529281902060020180548251818502810185019093528083526bffffffffffffffffffffffff8516946c01000000000000000000000000900467ffffffffffffffff16938793918391830182828015612f3857602002820191905f5260205f20905b815473ffffffffffffffffffffffffffffffffffffffff168152600190910190602001808311612f0d575b5050505050905093509350935093509193509193565b612f777fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b600180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff83169081179091556040517f229f6c3b095d683755a99ab458956747a8b7066c3dd42927d850631c34c238f1905f90a250565b6060600e805480602002602001604051908101604052809291908181526020015f905b8282101561324c575f84815260209081902060408051610120810182526009860290920180548352600181015493830193909352600283015463ffffffff16908201526003820154606082015260048201805491929160808401919061306d906153b2565b80601f0160208091040260200160405190810160405280929190818152602001828054613099906153b2565b80156130e45780601f106130bb576101008083540402835291602001916130e4565b820191905f5260205f20905b8154815290600101906020018083116130c757829003601f168201915b505050505081526020016005820180546130fd906153b2565b80601f0160208091040260200160405190810160405280929190818152602001828054613129906153b2565b80156131745780601f1061314b57610100808354040283529160200191613174565b820191905f5260205f20905b81548152906001019060200180831161315757829003601f168201915b5050505050815260200160068201805461318d906153b2565b80601f01602080910402602001604051908101604052809291908181526020018280546131b9906153b2565b80156132045780601f106131db57610100808354040283529160200191613204565b820191905f5260205f20905b8154815290600101906020018083116131e757829003601f168201915b5050509183525050600782015460208083019190915260089092015473ffffffffffffffffffffffffffffffffffffffff166040909101529082526001929092019101613008565b50505050905090565b5f7fffffffff0000000000000000000000000000000000000000000000000000000082167f7965db0b000000000000000000000000000000000000000000000000000000001480610cec57507f01ffc9a7000000000000000000000000000000000000000000000000000000007fffffffff00000000000000000000000000000000000000000000000000000000831614610cec565b5f8181526003602052604090205473ffffffffffffffffffffffffffffffffffffffff1661331881613db2565b3373ffffffffffffffffffffffffffffffffffffffff821614611ad9576040517fd8a3fb5200000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff82166004820152602401610dc6565b60025f54036133ba576040517f3ee5aeb500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60025f55565b5f6133ca83614017565b90506133e482826bffffffffffffffffffffffff16613d11565b6040805173ffffffffffffffffffffffffffffffffffffffff841681526bffffffffffffffffffffffff8316602082015284917f3784f77e8e883de95b5d47cd713ced01229fa74d118c0a462224bcb0516d43f1910160405180910390a2505050565b600a5463ffffffff90811690831611156134bd576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601a60248201527f43616c6c6261636b206761734c696d697420746f6f20686967680000000000006044820152606401610dc6565b80156136c6575f8181526003602052604090205473ffffffffffffffffffffffffffffffffffffffff166134f081613db2565b335f908152600260209081526040808320858452808352928190208151606081018352905460ff8116151580835267ffffffffffffffff6101008304811695840195909552690100000000000000000090910490931691810191909152906135da576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602160248201527f4e6f2061637469766520737562736372697074696f6e20666f722063616c6c6560448201527f72000000000000000000000000000000000000000000000000000000000000006064820152608401610dc6565b8060200180516135e990615659565b67ffffffffffffffff16905260408101805161360490615659565b67ffffffffffffffff9081169091525f85815260209384526040908190208351815495850151929094015183166901000000000000000000027fffffffffffffffffffffffffffffff0000000000000000ffffffffffffffffff92909316610100027fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000ff941515949094167fffffffffffffffffffffffffffffffffffffffffffffff000000000000000000909516949094179290921791909116179055505050565b5f6136d78363ffffffff163a61383c565b905080341015610d57576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600b60248201527f46656520746f6f206c6f770000000000000000000000000000000000000000006044820152606401610dc6565b6001546040517f95b8d0730000000000000000000000000000000000000000000000000000000081525f9173ffffffffffffffffffffffffffffffffffffffff16906395b8d0739061379d90879087908790600401615685565b6020604051808303815f875af11580156137b9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d4191906156bd565b611bb381336141cd565b5f7fc1f6fe24621ce81ec5827caf0253cadb74709b061630e6b55e82371705932000816138148585614273565b90508015611d41575f8581526020839052604090206138339085614391565b50949350505050565b6040805160e081018252600a5463ffffffff80821683526401000000008204811660208401526801000000000000000082048116938301939093526c0100000000000000000000000081048316606083015270010000000000000000000000000000000081048316608083015260ff7401000000000000000000000000000000000000000082041660a08301527501000000000000000000000000000000000000000000900490911660c08201525f90818361390257816060015163ffffffff16613904565b835b90505f613910866143b2565b63ffffffff16836080015163ffffffff1687856020015163ffffffff16010101820290505f61393c5f90565b90505f8460a0015160640160ff1690505f856040015163ffffffff1664e8d4a510000290505f816064848787010281613977576139776156d4565b04019a9950505050505050505050565b5f7fc1f6fe24621ce81ec5827caf0253cadb74709b061630e6b55e82371705932000816139b485856143c9565b90508015611d41575f85815260208390526040902061383390856144a5565b5f807ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00610cec565b613a036144c6565b565b3073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000161480613ad257507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16613ab97f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5473ffffffffffffffffffffffffffffffffffffffff1690565b73ffffffffffffffffffffffffffffffffffffffff1614155b15613a03576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611bb37fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015613bb7575060408051601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201909252613bb4918101906156bd565b60015b613c05576040517f4c9c8ce300000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff83166004820152602401610dc6565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8114613c61576040517faa1d49a400000000000000000000000000000000000000000000000000000000815260048101829052602401610dc6565b610d578383614504565b3073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614613a03576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80611bb3576040517ff4d678b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8273ffffffffffffffffffffffffffffffffffffffff16826040515f6040518083038185875af1925050503d805f8114613d67576040519150601f19603f3d011682016040523d82523d5f602084013e613d6c565b606091505b5050905080610d57576040517f950b247900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6113998383614566565b73ffffffffffffffffffffffffffffffffffffffff8116611bb3576040517f1f6a65b600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f611399838361458c565b60605f611399836145d8565b5f610cec825490565b5f5a5f858152600d60205260409020600781015491925090613e9d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601960248201527f4e6f207265717565737420666f722072657175657374206964000000000000006044820152606401610dc6565b5f8484604051613eae929190615701565b604080519182900382206024830189905260448084018290528251808503909101815260649093019091526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f431ac6570000000000000000000000000000000000000000000000000000000017905260088401546002850154600a549294505f92613f7e92859273ffffffffffffffffffffffffffffffffffffffff9091169163ffffffff91821691750100000000000000000000000000000000000000000090910481169061463116565b5090508015613fd85760068401613f96878983615710565b50877fb74b3204a538cd8021662d42e794681ddc339924ef675b8fd11e9eaf6aa19eb5848989604051613fcb93929190615826565b60405180910390a2614003565b60405188907f8f67472dde2126ccd0315b75dc482a5a73acb228a395553f8ae6edde5a0ca4fa905f90a25b61400d8886614668565b5050505050505050565b5f8181526003602090815260408083206004909252822054600290910180546bffffffffffffffffffffffff909216929091905b818110156140d15760025f84838154811061406857614068615565565b5f91825260208083209091015473ffffffffffffffffffffffffffffffffffffffff1683528281019390935260409182018120888252909252902080547fffffffffffffffffffffffffffffff000000000000000000000000000000000016905560010161404b565b505f84815260036020526040812080547fffffffffffffffffffffffff000000000000000000000000000000000000000090811682556001820180549091169055906141206002830182614c9e565b50505f84815260046020526040902080547fffffffffffffffffffffffff0000000000000000000000000000000000000000169055614160600685614777565b506bffffffffffffffffffffffff8316156141c657600880548491905f906141979084906bffffffffffffffffffffffff166155e9565b92506101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055505b5050919050565b5f8281527f02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b6268006020908152604080832073ffffffffffffffffffffffffffffffffffffffff8516845290915290205460ff16611ad9576040517fe2517d3f00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8216600482015260248101839052604401610dc6565b5f8281527f02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b6268006020818152604080842073ffffffffffffffffffffffffffffffffffffffff8616855290915282205460ff16614388575f8481526020828152604080832073ffffffffffffffffffffffffffffffffffffffff87168452909152902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660011790556143243390565b73ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff16857f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a46001915050610cec565b5f915050610cec565b5f6113998373ffffffffffffffffffffffffffffffffffffffff841661458c565b5f6143be603f83615879565b610cec9060016158c5565b5f8281527f02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b6268006020818152604080842073ffffffffffffffffffffffffffffffffffffffff8616855290915282205460ff1615614388575f8481526020828152604080832073ffffffffffffffffffffffffffffffffffffffff8716808552925280832080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016905551339287917ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9190a46001915050610cec565b5f6113998373ffffffffffffffffffffffffffffffffffffffff841661477e565b6144ce614858565b613a03576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61450d82614876565b60405173ffffffffffffffffffffffffffffffffffffffff8316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a280511561455e57610d578282614944565b611ad96149c3565b5f825f01828154811061457b5761457b615565565b905f5260205f200154905092915050565b5f8181526001830160205260408120546145d157508154600181810184555f848152602080822090930184905584548482528286019093526040902091909155610cec565b505f610cec565b6060815f0180548060200260200160405190810160405280929190818152602001828054801561462557602002820191905f5260205f20905b815481526020019060010190808311614611575b50505050509050919050565b5f5f5a83811061465e5783900360408104810385101561465e575f5f885160208a015f8a8af19250600191505b5094509492505050565b5f61467283612691565b8051909150156147665780515f9081526004602052604090208054600c906146b3906c01000000000000000000000000900467ffffffffffffffff16615659565b825467ffffffffffffffff91821661010093840a908102908302199091161790925582015173ffffffffffffffffffffffffffffffffffffffff165f90815260026020908152604080832085518452909152902080549091600991614726916901000000000000000000909104166158e1565b91906101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055505f614757833a6149fb565b905061138281835f0151614a69565b610d578160200151825f0151614a69565b5f61139983835b5f8181526001830160205260408120548015614388575f6147a06001836155b6565b85549091505f906147b3906001906155b6565b9050808214614812575f865f0182815481106147d1576147d1615565565b905f5260205f200154905080875f0184815481106147f1576147f1615565565b5f918252602080832090910192909255918252600188019052604090208390555b85548690806148235761482361560d565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050610cec565b5f6148616139d3565b5468010000000000000000900460ff16919050565b8073ffffffffffffffffffffffffffffffffffffffff163b5f036148de576040517f4c9c8ce300000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff82166004820152602401610dc6565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc80547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff92909216919091179055565b60605f5f8473ffffffffffffffffffffffffffffffffffffffff168460405161496d9190615922565b5f60405180830381855af49150503d805f81146149a5576040519150601f19603f3d011682016040523d82523d5f602084013e6149aa565b606091505b50915091506149ba858383614b46565b95945050505050565b3415613a03576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5f5a600a54640100000000900463ffffffff1685010390508281025f600a5460649190920160ff740100000000000000000000000000000000000000008404168201020464e8d4a5100063ffffffff68010000000000000000909304929092169190910201949350505050565b8015614af1575f81815260046020526040902080546bffffffffffffffffffffffff90811690614a9d908516821015613cda565b81546bffffffffffffffffffffffff9185900382167fffffffffffffffffffffffffffffffffffffffff00000000000000000000000091821617909255600980548083168601909216919092161790555050565b600880546bffffffffffffffffffffffff6c0100000000000000000000000080830482168601909116027fffffffffffffffff000000000000000000000000ffffffffffffffffffffffff9091161790555050565b606082614b5b57614b5682614bd5565b611399565b8151158015614b7f575073ffffffffffffffffffffffffffffffffffffffff84163b155b15614bce576040517f9996b31500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff85166004820152602401610dc6565b5080611399565b805115614be457805160208201fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b828054828255905f5260205f20908101928215614c8e579160200282015b82811115614c8e57825182547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff909116178255602090920191600190910190614c34565b50614c9a929150614cb5565b5090565b5080545f8255905f5260205f2090810190611bb391905b5b80821115614c9a575f8155600101614cb6565b5f60208284031215614cd9575f5ffd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114611399575f5ffd5b73ffffffffffffffffffffffffffffffffffffffff81168114611bb3575f5ffd5b5f5f60408385031215614d3a575f5ffd5b823591506020830135614d4c81614d08565b809150509250929050565b803563ffffffff81168114614d6a575f5ffd5b919050565b5f5f60408385031215614d80575f5ffd5b614d8983614d57565b946020939093013593505050565b5f60208284031215614da7575f5ffd5b5035919050565b5f5f5f5f5f5f5f60e0888a031215614dc4575f5ffd5b614dcd88614d57565b9650614ddb60208901614d57565b9550614de960408901614d57565b9450614df760608901614d57565b9350614e0560808901614d57565b925060a088013560ff81168114614e1a575f5ffd5b9150614e2860c08901614d57565b905092959891949750929550565b5f5f60408385031215614e47575f5ffd5b8235614e5281614d08565b91506020830135614d4c81614d08565b5f60208284031215614e72575f5ffd5b61139982614d57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff81118282101715614eef57614eef614e7b565b604052919050565b5f5f60408385031215614f08575f5ffd5b8235614f1381614d08565b9150602083013567ffffffffffffffff811115614f2e575f5ffd5b8301601f81018513614f3e575f5ffd5b803567ffffffffffffffff811115614f5857614f58614e7b565b614f8960207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601614ea8565b818152866020838501011115614f9d575f5ffd5b816020840160208301375f602083830101528093505050509250929050565b5f60208284031215614fcc575f5ffd5b813561139981614d08565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b602081525f6113996020830184614fd7565b5f6040828403128015615046575f5ffd5b506040805190810167ffffffffffffffff8111828210171561506a5761506a614e7b565b60405282358152602083013561507f81614d08565b60208201529392505050565b5f5f6040838503121561509c575f5ffd5b50508035926020909101359150565b5f8151808452602084019350602083015f5b828110156150f157815173ffffffffffffffffffffffffffffffffffffffff168652602095860195909101906001016150bd565b5093949350505050565b602081525f61139960208301846150ab565b602080825282518282018190525f918401906040840190835b81811015615144578351835260209384019390920191600101615126565b509095945050505050565b80518252602081015160208301525f6040820151615175604085018263ffffffff169052565b50606082015160608401526080820151610120608085015261519b610120850182614fd7565b905060a083015184820360a08601526151b48282614fd7565b91505060c083015184820360c08601526151ce8282614fd7565b91505060e083015160e085015261010083015161520461010086018273ffffffffffffffffffffffffffffffffffffffff169052565b509392505050565b602081525f611399602083018461514f565b5f5f5f60408486031215615230575f5ffd5b83359250602084013567ffffffffffffffff81111561524d575f5ffd5b8401601f8101861361525d575f5ffd5b803567ffffffffffffffff811115615273575f5ffd5b866020828401011115615284575f5ffd5b939660209190910195509293505050565b6bffffffffffffffffffffffff8516815267ffffffffffffffff8416602082015273ffffffffffffffffffffffffffffffffffffffff83166040820152608060608201525f6152e760808301846150ab565b9695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015615366577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc087860301845261535185835161514f565b94506020938401939190910190600101615317565b50929695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b80820180821115610cec57610cec615372565b600181811c908216806153c657607f821691505b6020821081036153fd577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b50919050565b601f821115610d5757805f5260205f20601f840160051c810160208510156154285750805b601f840160051c820191505b81811015615447575f8155600101615434565b5050505050565b815167ffffffffffffffff81111561546857615468614e7b565b61547c8161547684546153b2565b84615403565b6020601f8211600181146154cd575f83156154975750848201515b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600385901b1c1916600184901b178455615447565b5f848152602081207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08516915b8281101561551a57878501518255602094850194600190920191016154fa565b508482101561555657868401517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600387901b60f8161c191681555b50505050600190811b01905550565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b6bffffffffffffffffffffffff8181168382160190811115610cec57610cec615372565b81810381811115610cec57610cec615372565b67ffffffffffffffff8181168382160190811115610cec57610cec615372565b6bffffffffffffffffffffffff8281168282160390811115610cec57610cec615372565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b5f6020828403121561564a575f5ffd5b81518015158114611399575f5ffd5b5f67ffffffffffffffff821667ffffffffffffffff810361567c5761567c615372565b60010192915050565b606081525f6156976060830186614fd7565b82810360208401526156a98186614fd7565b905082810360408401526152e78185614fd7565b5f602082840312156156cd575f5ffd5b5051919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b818382375f9101908152919050565b67ffffffffffffffff83111561572857615728614e7b565b61573c8361573683546153b2565b83615403565b5f601f84116001811461578c575f85156157565750838201355b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600387901b1c1916600186901b178355615447565b5f838152602081207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08716915b828110156157d957868501358255602094850194600190920191016157b9565b5086821015615814577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60f88860031b161c19848701351681555b505060018560011b0183555050505050565b83815260406020820152816040820152818360608301375f818301606090810191909152601f9092017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016010192915050565b5f63ffffffff8316806158b3577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b8063ffffffff84160491505092915050565b63ffffffff8181168382160190811115610cec57610cec615372565b5f67ffffffffffffffff8216806158fa576158fa615372565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0192915050565b5f82518060208501845e5f92019182525091905056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0`\x80R_`\x0CU4\x80\x15a\0\x17W__\xFD[P`\x01_Ua\0$a\0)V[a\0\xDBV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\0yW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a\0\xD8W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80QaY8a\x01\x01_9_\x81\x81a:\x1D\x01R\x81\x81a:F\x01Ra<\x83\x01RaY8_\xF3\xFE`\x80`@R`\x046\x10a\x03EW_5`\xE0\x1C\x80c\x91\xD1HT\x11a\x01\xB2W\x80c\xB2\xA7\xCA\xC5\x11a\0\xF2W\x80c\xCBc\x17\x97\x11a\0\x92W\x80c\xDA\xC8=)\x11a\0mW\x80c\xDA\xC8=)\x14a\x0C\tW\x80c\xDC1\x1D\xD3\x14a\x0C(W\x80c\xF8\xFA\rf\x14a\x0CWW\x80c\xFB\x1A\0*\x14a\x0CvW__\xFD[\x80c\xCBc\x17\x97\x14a\x0B\xACW\x80c\xCD\x80,\x91\x14a\x0B\xCBW\x80c\xD5Gt\x1F\x14a\x0B\xEAW__\xFD[\x80c\xC3\xF9\t\xD4\x11a\0\xCDW\x80c\xC3\xF9\t\xD4\x14a\n\xAAW\x80c\xC5\x83C\xEF\x14a\x0BBW\x80c\xC8\xDBe\x82\x14a\x0BnW\x80c\xCA\x15\xC8s\x14a\x0B\x8DW__\xFD[\x80c\xB2\xA7\xCA\xC5\x14a\nMW\x80c\xBD\x18ck\x14a\nlW\x80c\xBE\xC4\xC0\x8C\x14a\n\x8BW__\xFD[\x80c\xA3$j\xD3\x11a\x01]W\x80c\xAAC:\xFF\x11a\x018W\x80c\xAAC:\xFF\x14a\t\xA5W\x80c\xAD<\xB1\xCC\x14a\t\xC4W\x80c\xAE\xFB!/\x14a\n\x0CW\x80c\xAF\xFE\xD0\xE0\x14a\n8W__\xFD[\x80c\xA3$j\xD3\x14a\tGW\x80c\xA3\x90}q\x14a\tsW\x80c\xA6\x08\xA1\xE1\x14a\t\x87W__\xFD[\x80c\x9D@\xA6\xFD\x11a\x01\x8DW\x80c\x9D@\xA6\xFD\x14a\x08\xE7W\x80c\xA2\x17\xFD\xDF\x14a\t W\x80c\xA2\x1A#\xE4\x14a\t3W__\xFD[\x80c\x91\xD1HT\x14a\x08@W\x80c\x95\xB5\\\xFC\x14a\x08\xB0W\x80c\x99\\\xB3n\x14a\x08\xC3W__\xFD[\x80cH\\\xC9U\x11a\x02\x88W\x80cd\xD5\x1A*\x11a\x02(W\x80c}F\x81\x06\x11a\x02\x03W\x80c}F\x81\x06\x14a\x07uW\x80c\x81\x1E\xE3*\x14a\x07\xC6W\x80c\x8A\x1F\x16Z\x14a\x07\xD9W\x80c\x90\x10\xD0|\x14a\x08!W__\xFD[\x80cd\xD5\x1A*\x14a\x06\xFCW\x80cu\xB28\xFC\x14a\x07#W\x80cw[\x83\x9C\x14a\x07VW__\xFD[\x80cR\xD1\x90-\x11a\x02cW\x80cR\xD1\x90-\x14a\x06bW\x80cT#o\xB3\x14a\x06vW\x80cT\xFDMP\x14a\x06\x95W\x80cW\xA8\x07\n\x14a\x06\xE3W__\xFD[\x80cH\\\xC9U\x14a\x06\x11W\x80cK\x16\t5\x14a\x060W\x80cO\x1E\xF2\x86\x14a\x06OW__\xFD[\x80c/'p\xDB\x11a\x02\xF3W\x80c6V\x8A\xBE\x11a\x02\xCEW\x80c6V\x8A\xBE\x14a\x05\x80W\x80c;\xC3,u\x14a\x05\x9FW\x80cA\xAFl\x87\x14a\x05\xD3W\x80cE\xFACT\x14a\x05\xF2W__\xFD[\x80c/'p\xDB\x14a\x05.W\x80c//\xF1]\x14a\x05BW\x80c2U\xC4V\x14a\x05aW__\xFD[\x80c\x18\xE3\xDD'\x11a\x03#W\x80c\x18\xE3\xDD'\x14a\x04\x7FW\x80c\x1D\xA5<\x9F\x14a\x04\xC0W\x80c$\x8A\x9C\xA3\x14a\x04\xE1W__\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x03IW\x80c\x08\x80p\xF5\x14a\x03}W\x80c\n\xE0\x95@\x14a\x04^W[__\xFD[4\x80\x15a\x03TW__\xFD[Pa\x03ha\x03c6`\x04aL\xC9V[a\x0C\x97V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x88W__\xFD[P`\nTa\x04\x17\x90c\xFF\xFF\xFF\xFF\x80\x82\x16\x91d\x01\0\0\0\0\x81\x04\x82\x16\x91h\x01\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04\x82\x16\x91p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91`\xFFt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x16\x91u\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16\x87V[`@\x80Qc\xFF\xFF\xFF\xFF\x98\x89\x16\x81R\x96\x88\x16` \x88\x01R\x94\x87\x16\x94\x86\x01\x94\x90\x94R\x91\x85\x16``\x85\x01R\x84\x16`\x80\x84\x01R`\xFF\x16`\xA0\x83\x01R\x90\x91\x16`\xC0\x82\x01R`\xE0\x01a\x03tV[4\x80\x15a\x04iW__\xFD[Pa\x04}a\x04x6`\x04aM)V[a\x0C\xF2V[\0[4\x80\x15a\x04\x8AW__\xFD[P`\x08Ta\x04\xA3\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03tV[a\x04\xD3a\x04\xCE6`\x04aMoV[a\r\\V[`@Q\x90\x81R` \x01a\x03tV[4\x80\x15a\x04\xECW__\xFD[Pa\x04\xD3a\x04\xFB6`\x04aM\x97V[_\x90\x81R\x7F\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0` R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x059W__\xFD[Pa\x04}a\x12\xC0V[4\x80\x15a\x05MW__\xFD[Pa\x04}a\x05\\6`\x04aM)V[a\x13?V[4\x80\x15a\x05lW__\xFD[Pa\x04\xD3a\x05{6`\x04aMoV[a\x13\x88V[4\x80\x15a\x05\x8BW__\xFD[Pa\x04}a\x05\x9A6`\x04aM)V[a\x13\xA0V[4\x80\x15a\x05\xAAW__\xFD[P`\x08Ta\x04\xA3\x90l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x05\xDEW__\xFD[Pa\x03ha\x05\xED6`\x04aM\x97V[a\x13\xF9V[4\x80\x15a\x05\xFDW__\xFD[Pa\x04}a\x06\x0C6`\x04aM\xAEV[a\x14\xA2V[4\x80\x15a\x06\x1CW__\xFD[Pa\x04}a\x06+6`\x04aN6V[a\x17]V[4\x80\x15a\x06;W__\xFD[Pa\x04\xD3a\x06J6`\x04aNbV[a\x1A\xADV[a\x04}a\x06]6`\x04aN\xF7V[a\x1A\xBEV[4\x80\x15a\x06mW__\xFD[Pa\x04\xD3a\x1A\xDDV[4\x80\x15a\x06\x81W__\xFD[Pa\x04}a\x06\x906`\x04aO\xBCV[a\x1B\x0CV[4\x80\x15a\x06\xA0W__\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81R\x7F0.0.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R[`@Qa\x03t\x91\x90aP#V[4\x80\x15a\x06\xEEW__\xFD[P`\x0BTa\x03h\x90`\xFF\x16\x81V[4\x80\x15a\x07\x07W__\xFD[Pa\x07\x10`d\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x03tV[4\x80\x15a\x07.W__\xFD[Pa\x04\xD3\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17u\x81V[4\x80\x15a\x07aW__\xFD[Pa\x06\xD6a\x07p6`\x04aP5V[a\x1B\xB6V[4\x80\x15a\x07\x80W__\xFD[P`\x01Ta\x07\xA1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03tV[a\x04\xD3a\x07\xD46`\x04aNbV[a\x1C\x1FV[4\x80\x15a\x07\xE4W__\xFD[Pa\x06\xD6`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7FBN254\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[4\x80\x15a\x08,W__\xFD[Pa\x07\xA1a\x08;6`\x04aP\x8BV[a\x1D\tV[4\x80\x15a\x08KW__\xFD[Pa\x03ha\x08Z6`\x04aM)V[_\x91\x82R\x7F\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0` \x90\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[a\x04}a\x08\xBE6`\x04aM\x97V[a\x1DIV[4\x80\x15a\x08\xCEW__\xFD[P`\tTa\x04\xA3\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x08\xF2W__\xFD[P`\x05Ta\t\x07\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03tV[4\x80\x15a\t+W__\xFD[Pa\x04\xD3_\x81V[4\x80\x15a\t>W__\xFD[Pa\x04\xD3a\x1E\x7FV[4\x80\x15a\tRW__\xFD[Pa\tfa\ta6`\x04aM\x97V[a \xC3V[`@Qa\x03t\x91\x90aP\xFBV[4\x80\x15a\t~W__\xFD[Pa\x04}a \xFFV[4\x80\x15a\t\x92W__\xFD[P`\x0BTa\x03h\x90a\x01\0\x90\x04`\xFF\x16\x81V[4\x80\x15a\t\xB0W__\xFD[Pa\x04}a\t\xBF6`\x04aM\x97V[a!zV[4\x80\x15a\t\xCFW__\xFD[Pa\x06\xD6`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[4\x80\x15a\n\x17W__\xFD[Pa\n+a\n&6`\x04aP\x8BV[a!\xDAV[`@Qa\x03t\x91\x90aQ\rV[4\x80\x15a\nCW__\xFD[Pa\x04\xD3`\x0CT\x81V[4\x80\x15a\nXW__\xFD[Pa\x04}a\ng6`\x04aM\x97V[a\"\xE9V[4\x80\x15a\nwW__\xFD[Pa\x04}a\n\x866`\x04aO\xBCV[a$9V[4\x80\x15a\n\x96W__\xFD[Pa\x04}a\n\xA56`\x04aM)V[a%\x16V[4\x80\x15a\n\xB5W__\xFD[P`\nTc\xFF\xFF\xFF\xFF\x80\x82\x16\x91d\x01\0\0\0\0\x81\x04\x82\x16\x91h\x01\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04\x82\x16\x91p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91`\xFFt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x16\x91u\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16a\x04\x17V[4\x80\x15a\x0BMW__\xFD[Pa\x0Baa\x0B\\6`\x04aM\x97V[a&\x91V[`@Qa\x03t\x91\x90aR\x0CV[4\x80\x15a\x0ByW__\xFD[Pa\x04}a\x0B\x886`\x04aR\x1EV[a)%V[4\x80\x15a\x0B\x98W__\xFD[Pa\x04\xD3a\x0B\xA76`\x04aM\x97V[a)\xB1V[4\x80\x15a\x0B\xB7W__\xFD[Pa\x04}a\x0B\xC66`\x04aM)V[a)\xE8V[4\x80\x15a\x0B\xD6W__\xFD[Pa\x03ha\x0B\xE56`\x04aM\x97V[a,\xC3V[4\x80\x15a\x0B\xF5W__\xFD[Pa\x04}a\x0C\x046`\x04aM)V[a-TV[4\x80\x15a\x0C\x14W__\xFD[Pa\x04}a\x0C#6`\x04aM)V[a-\x97V[4\x80\x15a\x0C3W__\xFD[Pa\x0CGa\x0CB6`\x04aM\x97V[a.fV[`@Qa\x03t\x94\x93\x92\x91\x90aR\x95V[4\x80\x15a\x0CbW__\xFD[Pa\x04}a\x0Cq6`\x04aO\xBCV[a/NV[4\x80\x15a\x0C\x81W__\xFD[Pa\x0C\x8Aa/\xE5V[`@Qa\x03t\x91\x90aR\xF1V[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x0C\xECWPa\x0C\xEC\x82a2UV[\x92\x91PPV[\x81a\x0C\xFC\x81a2\xEBV[a\r\x04a3\x7FV[a\r\r\x83a\x13\xF9V[\x15a\rDW`@Q\x7F\xB4/f\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\rN\x83\x83a3\xC0V[a\rW`\x01_UV[PPPV[`\x0BT_\x90`\xFF\x16a\r\xCFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FContract is not configured\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x0BTa\x01\0\x90\x04`\xFF\x16\x15a\x0EAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FContract is disabled\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[\x81\x15\x15\x80a\x0ENWP_4\x11[a\x0E\xDAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FDirect funding required for requ`D\x82\x01R\x7Fest fulfillment callback\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\r\xC6V[a\x0E\xE4\x83\x83a4GV[`\x01`\x0C_\x82\x82Ta\x0E\xF6\x91\x90aS\x9FV[\x90\x91UPP`@\x80Qa\x01 \x81\x01\x82R\x83\x81R4` \x80\x83\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x86\x16\x82\x84\x01R_``\x83\x01\x81\x90R\x83Q\x80\x83\x01\x85R\x81\x81R`\x80\x84\x01R\x83Q\x80\x83\x01\x85R\x81\x81R`\xA0\x84\x01R\x83Q\x80\x83\x01\x85R\x81\x81R`\xC0\x84\x01R`\x0CT`\xE0\x84\x01\x81\x90R3a\x01\0\x85\x01\x81\x90R\x85Q\x80\x87\x01\x90\x96R\x90\x85R\x91\x84\x01\x91\x90\x91R\x90\x91a\x0F\x83\x90a\x1B\xB6V[`@\x80Q` \x80\x82\x01\x83R_\x82R\x82Q\x80\x84\x01\x90\x93R`\x05\x83R\x7FBN254\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01R\x91\x92Pa\x0F\xD0\x90\x83\x83a7CV[``\x84\x01\x81\x81R`\x80\x85\x01\x84\x81R`\xA0\x86\x01\x84\x90R_\x83\x81R`\r` \x90\x81R`@\x91\x82\x90 \x88Q\x81U\x90\x88\x01Q`\x01\x82\x01U\x90\x87\x01Q`\x02\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U\x91Q`\x03\x83\x01UQ\x91\x95P\x84\x91`\x04\x82\x01\x90a\x10Y\x90\x82aTNV[P`\xA0\x82\x01Q`\x05\x82\x01\x90a\x10n\x90\x82aTNV[P`\xC0\x82\x01Q`\x06\x82\x01\x90a\x10\x83\x90\x82aTNV[P`\xE0\x82\x01Q`\x07\x82\x01Ua\x01\0\x90\x91\x01Q`\x08\x90\x91\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`\x0E\x80T`\x01\x81\x01\x82U_\x91\x90\x91R\x83Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD`\t\x90\x92\x02\x91\x82\x01\x90\x81U` \x85\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFE\x83\x01U`@\x85\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFF\x83\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U``\x85\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC4\0\x83\x01U`\x80\x85\x01Q\x85\x92\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC4\x01\x01\x90a\x11\xF9\x90\x82aTNV[P`\xA0\x82\x01Q`\x05\x82\x01\x90a\x12\x0E\x90\x82aTNV[P`\xC0\x82\x01Q`\x06\x82\x01\x90a\x12#\x90\x82aTNV[P`\xE0\x82\x01Q`\x07\x82\x01Ua\x01\0\x90\x91\x01Q`\x08\x90\x91\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`\x0CT`@QB\x81R3\x91\x90\x86\x90\x7F\xEE\xE7\x19[l\xEE\x0F\xA7\x04L:\xF0\xB8o\xE2\xFE\xBB\x1D'\x03\xD7\x11\x91\xF4@R\xBA\r`\xFF\xDAd\x90` \x01`@Q\x80\x91\x03\x90\xA4PPP\x92\x91PPV[a\x12\xE9\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[`\x0B\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U`@Q\x7Fu\x88L\xDA\xDCJ\x89\xE8\xB5E\xDB\x80\0W\xF0n\xC7\xF53\x8A\x08\x18<{\xA5\x15\xF2\xBF\xDD\x9F\xE1\xE1\x90_\x90\xA1V[_\x82\x81R\x7F\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0` R`@\x90 `\x01\x01Ta\x13x\x81a7\xDDV[a\x13\x82\x83\x83a7\xE7V[PPPPV[_a\x13\x99\x83c\xFF\xFF\xFF\xFF\x16\x83a8<V[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14a\x13\xEFW`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\rW\x82\x82a9\x87V[_\x81\x81R`\x03` R`@\x81 `\x02\x01\x80T\x82[\x81\x81\x10\x15a\x14\x98W_`\x02_\x85\x84\x81T\x81\x10a\x14+Wa\x14+aUeV[_\x91\x82R` \x80\x83 \x90\x91\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x82\x81\x01\x93\x90\x93R`@\x91\x82\x01\x81 \x89\x82R\x90\x92R\x90 Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFi\x01\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16\x11\x15a\x14\x90WP`\x01\x94\x93PPPPV[`\x01\x01a\x14\rV[P_\x94\x93PPPPV[a\x14\xCB\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[`\xFF\x82\x16`\x9B\x11a\x158W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid Premium Percentage\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[`@\x80Q`\xE0\x80\x82\x01\x83Rc\xFF\xFF\xFF\xFF\x8A\x81\x16\x80\x84R\x8A\x82\x16` \x80\x86\x01\x82\x90R\x8B\x84\x16\x86\x88\x01\x81\x90R\x8B\x85\x16``\x80\x89\x01\x82\x90R\x8C\x87\x16`\x80\x80\x8B\x01\x82\x90R`\xFF\x8E\x16`\xA0\x80\x8D\x01\x82\x90R\x99\x8E\x16`\xC0\x9C\x8D\x01\x81\x90R`\n\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16\x8B\x17d\x01\0\0\0\0\x8B\x02\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16h\x01\0\0\0\0\0\0\0\0\x89\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x88\x02\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x02\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16u\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x02\x17\x90U`\x0B\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x8DQ\x99\x8AR\x96\x89\x01\x97\x90\x97R\x9A\x87\x01\x93\x90\x93R\x85\x01R\x96\x83\x01\x96\x90\x96R\x91\x81\x01\x91\x90\x91R\x91\x82\x01\x92\x90\x92R\x7FU\xA2\x8F\xDE)_H,\x9F2\xD6p\xC1\x16\x10;\xCA\x15rK\xCE\xF4\xF1\x8B5T.\x05S\xC3Z\xD5\x91\x01[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_a\x17fa9\xD3V[\x80T\x90\x91P`\xFFh\x01\0\0\0\0\0\0\0\0\x82\x04\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x81\x15\x80\x15a\x17\x92WP\x82[\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x17\xAEWP0;\x15[\x90P\x81\x15\x80\x15a\x17\xBCWP\x80\x15[\x15a\x17\xF3W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16`\x01\x17\x85U\x83\x15a\x18TW\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16h\x01\0\0\0\0\0\0\0\0\x17\x85U[a\x18\\a9\xFBV[a\x18da9\xFBV[a\x18\x8E\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17u\x87a7\xE7V[a\x18\xF4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FGrant role failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[a\x18\xFE_\x87a7\xE7V[a\x19dW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FGrant role failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16a\x1A\x07W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FCannot set zero address as signa`D\x82\x01R\x7Fture sender\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\r\xC6V[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x17\x90U\x83\x15a\x1A\xA4W\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01a\x17LV[PPPPPPPV[_a\x0C\xEC\x82c\xFF\xFF\xFF\xFF\x16:a8<V[a\x1A\xC6a:\x05V[a\x1A\xCF\x82a;\tV[a\x1A\xD9\x82\x82a;2V[PPV[_a\x1A\xE6a<kV[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC[\x90V[a\x1B\x14a3\x7FV[a\x1B=\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[`\x08Tl\x01\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1Bi\x81\x15\x15a<\xDAV[`\x08\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Ua\x1B\xA9\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a=\x11V[Pa\x1B\xB3`\x01_UV[PV[\x80Q`@Q``\x91a\x1B\xCE\x91` \x01\x90\x81R` \x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\x0BT_\x90`\xFF\x16a\x1C\x8DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FContract is not configured\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[`\x0BTa\x01\0\x90\x04`\xFF\x16\x15a\x1C\xFFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FContract is disabled\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[a\x0C\xEC\x82_a\r\\V[_\x82\x81R\x7F\xC1\xF6\xFE$b\x1C\xE8\x1E\xC5\x82|\xAF\x02S\xCA\xDBtp\x9B\x06\x160\xE6\xB5^\x827\x17\x05\x93 \0` \x81\x90R`@\x82 a\x1DA\x90\x84a=\xA7V[\x94\x93PPPPV[a\x1DQa3\x7FV[_\x81\x81R`\x03` R`@\x90 Ta\x1D~\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a=\xB2V[_\x81\x81R`\x04` R`@\x81 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x914\x91\x90a\x1D\xA9\x83\x85aU\x92V[\x92Pa\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP4`\x08_\x82\x82\x82\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1D\xFF\x91\x90aU\x92V[\x92Pa\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81\x7Fv\x03\xB2\x05\xD06Q\xEE\x81/\x80?\xCC\xDE\x89\xF1\x01.TZ\x9C\x99\xF0\xAB\xFE\xA9\xCE\xDD\x0F\xD8\xE9\x02\x824\x84a\x1E\\\x91\x90aS\x9FV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xA2Pa\x1B\xB3`\x01_UV[_a\x1E\x88a3\x7FV[`\x05Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163a\x1E\xA1`\x01CaU\xB6V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x93\x84\x1B\x81\x16` \x83\x01R\x91@`4\x82\x01R0\x90\x92\x1B\x16`T\x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0\x83\x90\x1B\x16`h\x82\x01R`p\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x91Pa\x1FT\x81`\x01aU\xC9V[`\x05\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x17\x90U`@\x80Q_\x80\x82R``\x80\x83\x01\x84R` \x80\x84\x01\x83\x81R\x84\x86\x01\x84\x81R\x89\x85R`\x04\x83R\x86\x85 \x91Q\x82T\x91Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x17l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90\x99\x16\x02\x97\x90\x97\x17\x90U\x84Q\x91\x82\x01\x85R3\x82R\x81\x81\x01\x83\x81R\x82\x86\x01\x85\x81R\x89\x85R`\x03\x83R\x95\x90\x93 \x82Q\x81T\x88\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x17\x82U\x93Q`\x01\x82\x01\x80T\x90\x98\x16\x94\x16\x93\x90\x93\x17\x90\x95U\x92Q\x80Q\x92\x94\x91\x92a r\x92`\x02\x85\x01\x92\x01\x90aL\x16V[Pa \x82\x91P`\x06\x90P\x84a=\xFFV[P`@Q3\x81R\x83\x90\x7F\x1D0\x15\xD7\xBA\x85\x0F\xA1\x98\xDC{\x1A?]Bw\x93\x13\xA6\x81\x03_w\xC8\xC07d\xC6\x10\x05Q\x8D\x90` \x01`@Q\x80\x91\x03\x90\xA2PPa\x1B\t`\x01_UV[_\x81\x81R\x7F\xC1\xF6\xFE$b\x1C\xE8\x1E\xC5\x82|\xAF\x02S\xCA\xDBtp\x9B\x06\x160\xE6\xB5^\x827\x17\x05\x93 \0` \x81\x90R`@\x90\x91 ``\x91\x90a\x13\x99\x90a>\nV[a!(\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[`\x0B\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q\x7F\xC0\xF9a\x05\x1F\x97\xB0LIdr\xD1\x1C\xB6\x17\r\x84NK,\x9D\xFD;`*O\xA0\x13\x97\x12\xD4\x84\x90_\x90\xA1V[a!\xA3\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[_\x81\x81R`\x03` R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a!\xD0\x81a=\xB2V[a\x1A\xD9\x82\x82a3\xC0V[``_a!\xE7`\x06a>\x16V[\x90P\x80\x84\x10a\"\"W`@Q\x7F\x13\x90\xF2\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\"-\x84\x86aS\x9FV[\x90P\x81\x81\x11\x80a\";WP\x83\x15[a\"EW\x80a\"GV[\x81[\x90P_a\"T\x86\x83aU\xB6V[\x90P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"oWa\"oaN{V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\x98W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x93P_[\x81\x81\x10\x15a\"\xDFWa\"\xBAa\"\xB2\x88\x83aS\x9FV[`\x06\x90a=\xA7V[\x85\x82\x81Q\x81\x10a\"\xCCWa\"\xCCaUeV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\"\x9DV[PPPP\x92\x91PPV[a\"\xF1a3\x7FV[_\x81\x81R`\x03` R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a#\x1E\x81a=\xB2V[_\x82\x81R`\x03` R`@\x90 `\x01\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a#\xA8W_\x82\x81R`\x03` R`@\x90\x81\x90 `\x01\x01T\x90Q\x7F\xD0\x84\xE9u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\r\xC6V[_\x82\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x80T3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x82\x16\x81\x17\x83U`\x01\x90\x92\x01\x80T\x90\x91\x16\x90U\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x91\x82\x01R\x83\x91\x7F\xD4\x11J\xB6\xE9\xAF\x9FY|R\x04\x1F2\xD6-\xC5|\\NL\rD'\0`ic^!l\x93\x86\x91\x01a\x1EmV[a$Aa3\x7FV[a$j\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[`\tTk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a$\x86\x81\x15\x15a<\xDAV[`\t\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U`\x08\x80T\x82\x91\x90_\x90a$\xD0\x90\x84\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aU\xE9V[\x92Pa\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\x1B\xA9\x82\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a=\x11V[\x81a% \x81a2\xEBV[a%(a3\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 \x80T`\xFF\x16\x15a%gWPa\rNV[_\x84\x81R`\x03` R`@\x90 `\x02\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x9C\x01a%\xD1W`@Q\x7F\x05\xA4\x8E\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81T`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x90\x91\x16\x81\x17\x83U\x81T\x90\x81\x01\x82U_\x82\x81R` \x90\x81\x90 \x90\x91\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x86\x91\x7F\x1E\x98\r\x04\xAAvH\xE2\x05q>^\x8E\xA3\x80\x86r\xAC\x16=\x10\x93m6\xF9\x1B,\x88\xAC\x15u\xE1\x91\x01[`@Q\x80\x91\x03\x90\xA2PPa\rW`\x01_UV[a&\xF6`@Q\x80a\x01 \x01`@R\x80_\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_\x82\x81R`\r` \x90\x81R`@\x91\x82\x90 \x82Qa\x01 \x81\x01\x84R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01Tc\xFF\xFF\xFF\xFF\x16\x92\x82\x01\x92\x90\x92R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01\x80T\x91\x92\x91`\x80\x84\x01\x91\x90a'T\x90aS\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta'\x80\x90aS\xB2V[\x80\x15a'\xCBW\x80`\x1F\x10a'\xA2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a'\xCBV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a'\xAEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x05\x82\x01\x80Ta'\xE4\x90aS\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(\x10\x90aS\xB2V[\x80\x15a([W\x80`\x1F\x10a(2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a([V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(>W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta(t\x90aS\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(\xA0\x90aS\xB2V[\x80\x15a(\xEBW\x80`\x1F\x10a(\xC2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(\xEBV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(\xCEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x07\x82\x01T` \x82\x01R`\x08\x90\x91\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x90\x91\x01R\x92\x91PPV[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a)\xA6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FOnly signatureSender can call\0\0\0`D\x82\x01R`d\x01a\r\xC6V[a\rW\x83\x83\x83a>\x1FV[_\x81\x81R\x7F\xC1\xF6\xFE$b\x1C\xE8\x1E\xC5\x82|\xAF\x02S\xCA\xDBtp\x9B\x06\x160\xE6\xB5^\x827\x17\x05\x93 \0` \x81\x90R`@\x82 a\x13\x99\x90a>\x16V[\x81a)\xF2\x81a2\xEBV[a)\xFAa3\x7FV[a*\x03\x83a\x13\xF9V[\x15a*:W`@Q\x7F\xB4/f\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T`\xFF\x16a*\xC2W`@Q\x7Fy\xBF\xD4\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`$\x82\x01R`D\x01a\r\xC6V[_\x83\x81R`\x03` R`@\x81 `\x02\x01\x80T\x90\x91[\x81\x81\x10\x15a,=W\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x82\x81T\x81\x10a+\x08Wa+\x08aUeV[_\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a,5W\x82a+:`\x01\x84aU\xB6V[\x81T\x81\x10a+JWa+JaUeV[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x82\x81T\x81\x10a+\x84Wa+\x84aUeV[\x90_R` _ \x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82\x80T\x80a+\xD9Wa+\xD9aV\rV[_\x82\x81R` \x90 \x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U\x01\x90Ua,=V[`\x01\x01a*\xD7V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x89\x84R\x82R\x91\x82\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90U\x90Q\x91\x82R\x86\x91\x7F2\x15\x8C`X4|\x16\x01\xB2\xD1+\xC6\x96\xACi\x01\xD8\xA9\xA9\xAA;\xA1\x0C'\xAB\n\x98>\x84%\xA7\x91\x01a&~V[`\x01T`@Q\x7F\xCD\x80,\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R_\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xCD\x80,\x91\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xEC\x91\x90aV:V[_\x82\x81R\x7F\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0` R`@\x90 `\x01\x01Ta-\x8D\x81a7\xDDV[a\x13\x82\x83\x83a9\x87V[\x81a-\xA1\x81a2\xEBV[a-\xA9a3\x7FV[_\x83\x81R`\x03` R`@\x90 `\x01\x81\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16\x91\x16\x14a.\\W`\x01\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90\x81\x17\x90\x91U`@\x80Q3\x81R` \x81\x01\x92\x90\x92R\x85\x91\x7F!\xA4\xDA\xD1p\xA6\xBFGl1\xBB\xCFJ\x16b\x82\x95\xB0\xE4Pg.\xEC%\xD7\xC93\x08\xE0SD\xA1\x91\x01`@Q\x80\x91\x03\x90\xA2[Pa\rW`\x01_UV[_\x81\x81R`\x03` R`@\x81 T\x81\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``a.\x97\x82a=\xB2V[_\x85\x81R`\x04` \x90\x81R`@\x80\x83 T`\x03\x83R\x92\x81\x90 `\x02\x01\x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x94l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93\x87\x93\x91\x83\x91\x83\x01\x82\x82\x80\x15a/8W` \x02\x82\x01\x91\x90_R` _ \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a/\rW[PPPPP\x90P\x93P\x93P\x93P\x93P\x91\x93P\x91\x93V[a/w\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\"\x9Fl;\t]h7U\xA9\x9A\xB4X\x95gG\xA8\xB7\x06l=\xD4)'\xD8Pc\x1C4\xC28\xF1\x90_\x90\xA2PV[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a2LW_\x84\x81R` \x90\x81\x90 `@\x80Qa\x01 \x81\x01\x82R`\t\x86\x02\x90\x92\x01\x80T\x83R`\x01\x81\x01T\x93\x83\x01\x93\x90\x93R`\x02\x83\x01Tc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01\x80T\x91\x92\x91`\x80\x84\x01\x91\x90a0m\x90aS\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta0\x99\x90aS\xB2V[\x80\x15a0\xE4W\x80`\x1F\x10a0\xBBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0\xE4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a0\xC7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x05\x82\x01\x80Ta0\xFD\x90aS\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta1)\x90aS\xB2V[\x80\x15a1tW\x80`\x1F\x10a1KWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a1tV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a1WW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta1\x8D\x90aS\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta1\xB9\x90aS\xB2V[\x80\x15a2\x04W\x80`\x1F\x10a1\xDBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a2\x04V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a1\xE7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x07\x82\x01T` \x80\x83\x01\x91\x90\x91R`\x08\x90\x92\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x90\x91\x01R\x90\x82R`\x01\x92\x90\x92\x01\x91\x01a0\x08V[PPPP\x90P\x90V[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x0C\xECWP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14a\x0C\xECV[_\x81\x81R`\x03` R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a3\x18\x81a=\xB2V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a\x1A\xD9W`@Q\x7F\xD8\xA3\xFBR\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\r\xC6V[`\x02_T\x03a3\xBAW`@Q\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_UV[_a3\xCA\x83a@\x17V[\x90Pa3\xE4\x82\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a=\x11V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R\x84\x91\x7F7\x84\xF7~\x8E\x88=\xE9[]G\xCDq<\xED\x01\"\x9F\xA7M\x11\x8C\nF\"$\xBC\xB0QmC\xF1\x91\x01`@Q\x80\x91\x03\x90\xA2PPPV[`\nTc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x83\x16\x11\x15a4\xBDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FCallback gasLimit too high\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[\x80\x15a6\xC6W_\x81\x81R`\x03` R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a4\xF0\x81a=\xB2V[3_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x85\x84R\x80\x83R\x92\x81\x90 \x81Q``\x81\x01\x83R\x90T`\xFF\x81\x16\x15\x15\x80\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\0\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95Ri\x01\0\0\0\0\0\0\0\0\0\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x90a5\xDAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FNo active subscription for calle`D\x82\x01R\x7Fr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\r\xC6V[\x80` \x01\x80Qa5\xE9\x90aVYV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90R`@\x81\x01\x80Qa6\x04\x90aVYV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x91R_\x85\x81R` \x93\x84R`@\x90\x81\x90 \x83Q\x81T\x95\x85\x01Q\x92\x90\x94\x01Q\x83\x16i\x01\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x93\x16a\x01\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\x94\x15\x15\x94\x90\x94\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x90\x95\x16\x94\x90\x94\x17\x92\x90\x92\x17\x91\x90\x91\x16\x17\x90UPPPV[_a6\xD7\x83c\xFF\xFF\xFF\xFF\x16:a8<V[\x90P\x804\x10\x15a\rWW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7FFee too low\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[`\x01T`@Q\x7F\x95\xB8\xD0s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\x95\xB8\xD0s\x90a7\x9D\x90\x87\x90\x87\x90\x87\x90`\x04\x01aV\x85V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a7\xB9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DA\x91\x90aV\xBDV[a\x1B\xB3\x813aA\xCDV[_\x7F\xC1\xF6\xFE$b\x1C\xE8\x1E\xC5\x82|\xAF\x02S\xCA\xDBtp\x9B\x06\x160\xE6\xB5^\x827\x17\x05\x93 \0\x81a8\x14\x85\x85aBsV[\x90P\x80\x15a\x1DAW_\x85\x81R` \x83\x90R`@\x90 a83\x90\x85aC\x91V[P\x94\x93PPPPV[`@\x80Q`\xE0\x81\x01\x82R`\nTc\xFF\xFF\xFF\xFF\x80\x82\x16\x83Rd\x01\0\0\0\0\x82\x04\x81\x16` \x84\x01Rh\x01\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x93\x83\x01\x93\x90\x93Rl\x01\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04\x83\x16``\x83\x01Rp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04\x83\x16`\x80\x83\x01R`\xFFt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x16`\xA0\x83\x01Ru\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x90\x91\x16`\xC0\x82\x01R_\x90\x81\x83a9\x02W\x81``\x01Qc\xFF\xFF\xFF\xFF\x16a9\x04V[\x83[\x90P_a9\x10\x86aC\xB2V[c\xFF\xFF\xFF\xFF\x16\x83`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x87\x85` \x01Qc\xFF\xFF\xFF\xFF\x16\x01\x01\x01\x82\x02\x90P_a9<_\x90V[\x90P_\x84`\xA0\x01Q`d\x01`\xFF\x16\x90P_\x85`@\x01Qc\xFF\xFF\xFF\xFF\x16d\xE8\xD4\xA5\x10\0\x02\x90P_\x81`d\x84\x87\x87\x01\x02\x81a9wWa9waV\xD4V[\x04\x01\x9A\x99PPPPPPPPPPV[_\x7F\xC1\xF6\xFE$b\x1C\xE8\x1E\xC5\x82|\xAF\x02S\xCA\xDBtp\x9B\x06\x160\xE6\xB5^\x827\x17\x05\x93 \0\x81a9\xB4\x85\x85aC\xC9V[\x90P\x80\x15a\x1DAW_\x85\x81R` \x83\x90R`@\x90 a83\x90\x85aD\xA5V[_\x80\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0a\x0C\xECV[a:\x03aD\xC6V[V[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a:\xD2WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a:\xB9\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a:\x03W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1B\xB3\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a;\xB7WP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra;\xB4\x91\x81\x01\x90aV\xBDV[`\x01[a<\x05W`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x01a\r\xC6V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a<aW`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\r\xC6V[a\rW\x83\x83aE\x04V[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a:\x03W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x1B\xB3W`@Q\x7F\xF4\xD6x\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a=gW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a=lV[``\x91P[PP\x90P\x80a\rWW`@Q\x7F\x95\x0B$y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13\x99\x83\x83aEfV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x1B\xB3W`@Q\x7F\x1Fje\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13\x99\x83\x83aE\x8CV[``_a\x13\x99\x83aE\xD8V[_a\x0C\xEC\x82T\x90V[_Z_\x85\x81R`\r` R`@\x90 `\x07\x81\x01T\x91\x92P\x90a>\x9DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FNo request for request id\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[_\x84\x84`@Qa>\xAE\x92\x91\x90aW\x01V[`@\x80Q\x91\x82\x90\x03\x82 `$\x83\x01\x89\x90R`D\x80\x84\x01\x82\x90R\x82Q\x80\x85\x03\x90\x91\x01\x81R`d\x90\x93\x01\x90\x91R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FC\x1A\xC6W\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x08\x84\x01T`\x02\x85\x01T`\nT\x92\x94P_\x92a?~\x92\x85\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91c\xFF\xFF\xFF\xFF\x91\x82\x16\x91u\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04\x81\x16\x90aF1\x16V[P\x90P\x80\x15a?\xD8W`\x06\x84\x01a?\x96\x87\x89\x83aW\x10V[P\x87\x7F\xB7K2\x04\xA58\xCD\x80!f-B\xE7\x94h\x1D\xDC3\x99$\xEFg[\x8F\xD1\x1E\x9E\xAFj\xA1\x9E\xB5\x84\x89\x89`@Qa?\xCB\x93\x92\x91\x90aX&V[`@Q\x80\x91\x03\x90\xA2a@\x03V[`@Q\x88\x90\x7F\x8FgG-\xDE!&\xCC\xD01[u\xDCH*Zs\xAC\xB2(\xA3\x95U?\x8A\xE6\xED\xDEZ\x0C\xA4\xFA\x90_\x90\xA2[a@\r\x88\x86aFhV[PPPPPPPPV[_\x81\x81R`\x03` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 T`\x02\x90\x91\x01\x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x92\x90\x91\x90[\x81\x81\x10\x15a@\xD1W`\x02_\x84\x83\x81T\x81\x10a@hWa@haUeV[_\x91\x82R` \x80\x83 \x90\x91\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x82\x81\x01\x93\x90\x93R`@\x91\x82\x01\x81 \x88\x82R\x90\x92R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U`\x01\x01a@KV[P_\x84\x81R`\x03` R`@\x81 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x82U`\x01\x82\x01\x80T\x90\x91\x16\x90U\x90aA `\x02\x83\x01\x82aL\x9EV[PP_\x84\x81R`\x04` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90UaA``\x06\x85aGwV[Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x15aA\xC6W`\x08\x80T\x84\x91\x90_\x90aA\x97\x90\x84\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aU\xE9V[\x92Pa\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP[PP\x91\x90PV[_\x82\x81R\x7F\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x1A\xD9W`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\r\xC6V[_\x82\x81R\x7F\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0` \x81\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x85R\x90\x91R\x82 T`\xFF\x16aC\x88W_\x84\x81R` \x82\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UaC$3\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\x0C\xECV[_\x91PPa\x0C\xECV[_a\x13\x99\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16aE\x8CV[_aC\xBE`?\x83aXyV[a\x0C\xEC\x90`\x01aX\xC5V[_\x82\x81R\x7F\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0` \x81\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x85R\x90\x91R\x82 T`\xFF\x16\x15aC\x88W_\x84\x81R` \x82\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\x0C\xECV[_a\x13\x99\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16aG~V[aD\xCEaHXV[a:\x03W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aE\r\x82aHvV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15aE^Wa\rW\x82\x82aIDV[a\x1A\xD9aI\xC3V[_\x82_\x01\x82\x81T\x81\x10aE{WaE{aUeV[\x90_R` _ \x01T\x90P\x92\x91PPV[_\x81\x81R`\x01\x83\x01` R`@\x81 TaE\xD1WP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x0C\xECV[P_a\x0C\xECV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15aF%W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11aF\x11W[PPPPP\x90P\x91\x90PV[__Z\x83\x81\x10aF^W\x83\x90\x03`@\x81\x04\x81\x03\x85\x10\x15aF^W__\x88Q` \x8A\x01_\x8A\x8A\xF1\x92P`\x01\x91P[P\x94P\x94\x92PPPV[_aFr\x83a&\x91V[\x80Q\x90\x91P\x15aGfW\x80Q_\x90\x81R`\x04` R`@\x90 \x80T`\x0C\x90aF\xB3\x90l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aVYV[\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16a\x01\0\x93\x84\n\x90\x81\x02\x90\x83\x02\x19\x90\x91\x16\x17\x90\x92U\x82\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x85Q\x84R\x90\x91R\x90 \x80T\x90\x91`\t\x91aG&\x91i\x01\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16aX\xE1V[\x91\x90a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_aGW\x83:aI\xFBV[\x90Pa\x13\x82\x81\x83_\x01QaJiV[a\rW\x81` \x01Q\x82_\x01QaJiV[_a\x13\x99\x83\x83[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aC\x88W_aG\xA0`\x01\x83aU\xB6V[\x85T\x90\x91P_\x90aG\xB3\x90`\x01\x90aU\xB6V[\x90P\x80\x82\x14aH\x12W_\x86_\x01\x82\x81T\x81\x10aG\xD1WaG\xD1aUeV[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10aG\xF1WaG\xF1aUeV[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80aH#WaH#aV\rV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x0C\xECV[_aHaa9\xD3V[Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x91\x90PV[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;_\x03aH\xDEW`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\r\xC6V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``__\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@QaIm\x91\x90aY\"V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aI\xA5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aI\xAAV[``\x91P[P\x91P\x91PaI\xBA\x85\x83\x83aKFV[\x95\x94PPPPPV[4\x15a:\x03W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__Z`\nTd\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x85\x01\x03\x90P\x82\x81\x02_`\nT`d\x91\x90\x92\x01`\xFFt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x04\x16\x82\x01\x02\x04d\xE8\xD4\xA5\x10\0c\xFF\xFF\xFF\xFFh\x01\0\0\0\0\0\0\0\0\x90\x93\x04\x92\x90\x92\x16\x91\x90\x91\x02\x01\x94\x93PPPPV[\x80\x15aJ\xF1W_\x81\x81R`\x04` R`@\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90aJ\x9D\x90\x85\x16\x82\x10\x15a<\xDAV[\x81Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x85\x90\x03\x82\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x91\x82\x16\x17\x90\x92U`\t\x80T\x80\x83\x16\x86\x01\x90\x92\x16\x91\x90\x92\x16\x17\x90UPPV[`\x08\x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFl\x01\0\0\0\0\0\0\0\0\0\0\0\0\x80\x83\x04\x82\x16\x86\x01\x90\x91\x16\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x90UPPV[``\x82aK[WaKV\x82aK\xD5V[a\x13\x99V[\x81Q\x15\x80\x15aK\x7FWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;\x15[\x15aK\xCEW`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`$\x01a\r\xC6V[P\x80a\x13\x99V[\x80Q\x15aK\xE4W\x80Q` \x82\x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aL\x8EW\x91` \x02\x82\x01[\x82\x81\x11\x15aL\x8EW\x82Q\x82T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90aL4V[PaL\x9A\x92\x91PaL\xB5V[P\x90V[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a\x1B\xB3\x91\x90[[\x80\x82\x11\x15aL\x9AW_\x81U`\x01\x01aL\xB6V[_` \x82\x84\x03\x12\x15aL\xD9W__\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x13\x99W__\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1B\xB3W__\xFD[__`@\x83\x85\x03\x12\x15aM:W__\xFD[\x825\x91P` \x83\x015aML\x81aM\x08V[\x80\x91PP\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14aMjW__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15aM\x80W__\xFD[aM\x89\x83aMWV[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15aM\xA7W__\xFD[P5\x91\x90PV[_______`\xE0\x88\x8A\x03\x12\x15aM\xC4W__\xFD[aM\xCD\x88aMWV[\x96PaM\xDB` \x89\x01aMWV[\x95PaM\xE9`@\x89\x01aMWV[\x94PaM\xF7``\x89\x01aMWV[\x93PaN\x05`\x80\x89\x01aMWV[\x92P`\xA0\x88\x015`\xFF\x81\x16\x81\x14aN\x1AW__\xFD[\x91PaN(`\xC0\x89\x01aMWV[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[__`@\x83\x85\x03\x12\x15aNGW__\xFD[\x825aNR\x81aM\x08V[\x91P` \x83\x015aML\x81aM\x08V[_` \x82\x84\x03\x12\x15aNrW__\xFD[a\x13\x99\x82aMWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aN\xEFWaN\xEFaN{V[`@R\x91\x90PV[__`@\x83\x85\x03\x12\x15aO\x08W__\xFD[\x825aO\x13\x81aM\x08V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aO.W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aO>W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aOXWaOXaN{V[aO\x89` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01aN\xA8V[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15aO\x9DW__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15aO\xCCW__\xFD[\x815a\x13\x99\x81aM\x08V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x13\x99` \x83\x01\x84aO\xD7V[_`@\x82\x84\x03\x12\x80\x15aPFW__\xFD[P`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aPjWaPjaN{V[`@R\x825\x81R` \x83\x015aP\x7F\x81aM\x08V[` \x82\x01R\x93\x92PPPV[__`@\x83\x85\x03\x12\x15aP\x9CW__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aP\xF1W\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aP\xBDV[P\x93\x94\x93PPPPV[` \x81R_a\x13\x99` \x83\x01\x84aP\xABV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aQDW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aQ&V[P\x90\x95\x94PPPPPV[\x80Q\x82R` \x81\x01Q` \x83\x01R_`@\x82\x01QaQu`@\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P``\x82\x01Q``\x84\x01R`\x80\x82\x01Qa\x01 `\x80\x85\x01RaQ\x9Ba\x01 \x85\x01\x82aO\xD7V[\x90P`\xA0\x83\x01Q\x84\x82\x03`\xA0\x86\x01RaQ\xB4\x82\x82aO\xD7V[\x91PP`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01RaQ\xCE\x82\x82aO\xD7V[\x91PP`\xE0\x83\x01Q`\xE0\x85\x01Ra\x01\0\x83\x01QaR\x04a\x01\0\x86\x01\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[P\x93\x92PPPV[` \x81R_a\x13\x99` \x83\x01\x84aQOV[___`@\x84\x86\x03\x12\x15aR0W__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aRMW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13aR]W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aRsW__\xFD[\x86` \x82\x84\x01\x01\x11\x15aR\x84W__\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16` \x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`@\x82\x01R`\x80``\x82\x01R_aR\xE7`\x80\x83\x01\x84aP\xABV[\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aSfW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x87\x86\x03\x01\x84RaSQ\x85\x83QaQOV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aS\x17V[P\x92\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0C\xECWa\x0C\xECaSrV[`\x01\x81\x81\x1C\x90\x82\x16\x80aS\xC6W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aS\xFDW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\rWW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aT(WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15aTGW_\x81U`\x01\x01aT4V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aThWaThaN{V[aT|\x81aTv\x84TaS\xB2V[\x84aT\x03V[` `\x1F\x82\x11`\x01\x81\x14aT\xCDW_\x83\x15aT\x97WP\x84\x82\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84UaTGV[_\x84\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x85\x16\x91[\x82\x81\x10\x15aU\x1AW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aT\xFAV[P\x84\x82\x10\x15aUVW\x86\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0C\xECWa\x0C\xECaSrV[\x81\x81\x03\x81\x81\x11\x15a\x0C\xECWa\x0C\xECaSrV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0C\xECWa\x0C\xECaSrV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0C\xECWa\x0C\xECaSrV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aVJW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x13\x99W__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x03aV|WaV|aSrV[`\x01\x01\x92\x91PPV[``\x81R_aV\x97``\x83\x01\x86aO\xD7V[\x82\x81\x03` \x84\x01RaV\xA9\x81\x86aO\xD7V[\x90P\x82\x81\x03`@\x84\x01RaR\xE7\x81\x85aO\xD7V[_` \x82\x84\x03\x12\x15aV\xCDW__\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15aW(WaW(aN{V[aW<\x83aW6\x83TaS\xB2V[\x83aT\x03V[_`\x1F\x84\x11`\x01\x81\x14aW\x8CW_\x85\x15aWVWP\x83\x82\x015[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83UaTGV[_\x83\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x87\x16\x91[\x82\x81\x10\x15aW\xD9W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aW\xB9V[P\x86\x82\x10\x15aX\x14W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[\x83\x81R`@` \x82\x01R\x81`@\x82\x01R\x81\x83``\x83\x017_\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x01\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x83\x16\x80aX\xB3W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x80c\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0C\xECWa\x0C\xECaSrV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80aX\xFAWaX\xFAaSrV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x92\x91PPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610345575f3560e01c806391d14854116101b2578063b2a7cac5116100f2578063cb63179711610092578063dac83d291161006d578063dac83d2914610c09578063dc311dd314610c28578063f8fa0d6614610c57578063fb1a002a14610c76575f5ffd5b8063cb63179714610bac578063cd802c9114610bcb578063d547741f14610bea575f5ffd5b8063c3f909d4116100cd578063c3f909d414610aaa578063c58343ef14610b42578063c8db658214610b6e578063ca15c87314610b8d575f5ffd5b8063b2a7cac514610a4d578063bd18636b14610a6c578063bec4c08c14610a8b575f5ffd5b8063a3246ad31161015d578063aa433aff11610138578063aa433aff146109a5578063ad3cb1cc146109c4578063aefb212f14610a0c578063affed0e014610a38575f5ffd5b8063a3246ad314610947578063a3907d7114610973578063a608a1e114610987575f5ffd5b80639d40a6fd1161018d5780639d40a6fd146108e7578063a217fddf14610920578063a21a23e414610933575f5ffd5b806391d148541461084057806395b55cfc146108b0578063995cb36e146108c3575f5ffd5b8063485cc9551161028857806364d51a2a116102285780637d468106116102035780637d46810614610775578063811ee32a146107c65780638a1f165a146107d95780639010d07c14610821575f5ffd5b806364d51a2a146106fc57806375b238fc14610723578063775b839c14610756575f5ffd5b806352d1902d1161026357806352d1902d1461066257806354236fb31461067657806354fd4d501461069557806357a8070a146106e3575f5ffd5b8063485cc955146106115780634b160935146106305780634f1ef2861461064f575f5ffd5b80632f2770db116102f357806336568abe116102ce57806336568abe146105805780633bc32c751461059f57806341af6c87146105d357806345fa4354146105f2575f5ffd5b80632f2770db1461052e5780632f2ff15d146105425780633255c45614610561575f5ffd5b806318e3dd271161032357806318e3dd271461047f5780631da53c9f146104c0578063248a9ca3146104e1575f5ffd5b806301ffc9a714610349578063088070f51461037d5780630ae095401461045e575b5f5ffd5b348015610354575f5ffd5b50610368610363366004614cc9565b610c97565b60405190151581526020015b60405180910390f35b348015610388575f5ffd5b50600a546104179063ffffffff8082169164010000000081048216916801000000000000000082048116916c010000000000000000000000008104821691700100000000000000000000000000000000820481169160ff740100000000000000000000000000000000000000008204169175010000000000000000000000000000000000000000009091041687565b6040805163ffffffff988916815296881660208801529487169486019490945291851660608501528416608084015260ff1660a083015290911660c082015260e001610374565b348015610469575f5ffd5b5061047d610478366004614d29565b610cf2565b005b34801561048a575f5ffd5b506008546104a3906bffffffffffffffffffffffff1681565b6040516bffffffffffffffffffffffff9091168152602001610374565b6104d36104ce366004614d6f565b610d5c565b604051908152602001610374565b3480156104ec575f5ffd5b506104d36104fb366004614d97565b5f9081527f02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b626800602052604090206001015490565b348015610539575f5ffd5b5061047d6112c0565b34801561054d575f5ffd5b5061047d61055c366004614d29565b61133f565b34801561056c575f5ffd5b506104d361057b366004614d6f565b611388565b34801561058b575f5ffd5b5061047d61059a366004614d29565b6113a0565b3480156105aa575f5ffd5b506008546104a3906c0100000000000000000000000090046bffffffffffffffffffffffff1681565b3480156105de575f5ffd5b506103686105ed366004614d97565b6113f9565b3480156105fd575f5ffd5b5061047d61060c366004614dae565b6114a2565b34801561061c575f5ffd5b5061047d61062b366004614e36565b61175d565b34801561063b575f5ffd5b506104d361064a366004614e62565b611aad565b61047d61065d366004614ef7565b611abe565b34801561066d575f5ffd5b506104d3611add565b348015610681575f5ffd5b5061047d610690366004614fbc565b611b0c565b3480156106a0575f5ffd5b5060408051808201909152600581527f302e302e3100000000000000000000000000000000000000000000000000000060208201525b6040516103749190615023565b3480156106ee575f5ffd5b50600b546103689060ff1681565b348015610707575f5ffd5b50610710606481565b60405161ffff9091168152602001610374565b34801561072e575f5ffd5b506104d37fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c2177581565b348015610761575f5ffd5b506106d6610770366004615035565b611bb6565b348015610780575f5ffd5b506001546107a19073ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff9091168152602001610374565b6104d36107d4366004614e62565b611c1f565b3480156107e4575f5ffd5b506106d66040518060400160405280600581526020017f424e32353400000000000000000000000000000000000000000000000000000081525081565b34801561082c575f5ffd5b506107a161083b36600461508b565b611d09565b34801561084b575f5ffd5b5061036861085a366004614d29565b5f9182527f02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b6268006020908152604080842073ffffffffffffffffffffffffffffffffffffffff93909316845291905290205460ff1690565b61047d6108be366004614d97565b611d49565b3480156108ce575f5ffd5b506009546104a3906bffffffffffffffffffffffff1681565b3480156108f2575f5ffd5b506005546109079067ffffffffffffffff1681565b60405167ffffffffffffffff9091168152602001610374565b34801561092b575f5ffd5b506104d35f81565b34801561093e575f5ffd5b506104d3611e7f565b348015610952575f5ffd5b50610966610961366004614d97565b6120c3565b60405161037491906150fb565b34801561097e575f5ffd5b5061047d6120ff565b348015610992575f5ffd5b50600b5461036890610100900460ff1681565b3480156109b0575f5ffd5b5061047d6109bf366004614d97565b61217a565b3480156109cf575f5ffd5b506106d66040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b348015610a17575f5ffd5b50610a2b610a2636600461508b565b6121da565b604051610374919061510d565b348015610a43575f5ffd5b506104d3600c5481565b348015610a58575f5ffd5b5061047d610a67366004614d97565b6122e9565b348015610a77575f5ffd5b5061047d610a86366004614fbc565b612439565b348015610a96575f5ffd5b5061047d610aa5366004614d29565b612516565b348015610ab5575f5ffd5b50600a5463ffffffff8082169164010000000081048216916801000000000000000082048116916c010000000000000000000000008104821691700100000000000000000000000000000000820481169160ff7401000000000000000000000000000000000000000082041691750100000000000000000000000000000000000000000090910416610417565b348015610b4d575f5ffd5b50610b61610b5c366004614d97565b612691565b604051610374919061520c565b348015610b79575f5ffd5b5061047d610b8836600461521e565b612925565b348015610b98575f5ffd5b506104d3610ba7366004614d97565b6129b1565b348015610bb7575f5ffd5b5061047d610bc6366004614d29565b6129e8565b348015610bd6575f5ffd5b50610368610be5366004614d97565b612cc3565b348015610bf5575f5ffd5b5061047d610c04366004614d29565b612d54565b348015610c14575f5ffd5b5061047d610c23366004614d29565b612d97565b348015610c33575f5ffd5b50610c47610c42366004614d97565b612e66565b6040516103749493929190615295565b348015610c62575f5ffd5b5061047d610c71366004614fbc565b612f4e565b348015610c81575f5ffd5b50610c8a612fe5565b60405161037491906152f1565b5f7fffffffff0000000000000000000000000000000000000000000000000000000082167f5a05180f000000000000000000000000000000000000000000000000000000001480610cec5750610cec82613255565b92915050565b81610cfc816132eb565b610d0461337f565b610d0d836113f9565b15610d44576040517fb42f66e800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d4e83836133c0565b610d5760015f55565b505050565b600b545f9060ff16610dcf576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601a60248201527f436f6e7472616374206973206e6f7420636f6e6669677572656400000000000060448201526064015b60405180910390fd5b600b54610100900460ff1615610e41576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601460248201527f436f6e74726163742069732064697361626c65640000000000000000000000006044820152606401610dc6565b81151580610e4e57505f34115b610eda576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603860248201527f4469726563742066756e64696e6720726571756972656420666f72207265717560448201527f6573742066756c66696c6c6d656e742063616c6c6261636b00000000000000006064820152608401610dc6565b610ee48383613447565b6001600c5f828254610ef6919061539f565b909155505060408051610120810182528381523460208083019190915263ffffffff8616828401525f606083018190528351808301855281815260808401528351808301855281815260a08401528351808301855281815260c0840152600c5460e084018190523361010085018190528551808701909652908552918401919091529091610f8390611bb6565b60408051602080820183525f82528251808401909352600583527f424e32353400000000000000000000000000000000000000000000000000000090830152919250610fd0908383613743565b606084018181526080850184815260a086018490525f838152600d602090815260409182902088518155908801516001820155908701516002820180547fffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000001663ffffffff909216919091179055915160038301555191955084916004820190611059908261544e565b5060a0820151600582019061106e908261544e565b5060c08201516006820190611083908261544e565b5060e0820151600782015561010090910151600890910180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff909216919091179055600e80546001810182555f9190915283517fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c3fd600990920291820190815560208501517fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c3fe83015560408501517fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c3ff830180547fffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000001663ffffffff90921691909117905560608501517fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c400830155608085015185927fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c40101906111f9908261544e565b5060a0820151600582019061120e908261544e565b5060c08201516006820190611223908261544e565b5060e0820151600782015561010090910151600890910180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff909216919091179055600c5460405142815233919086907feee7195b6cee0fa7044c3af0b86fe2febb1d2703d71191f44052ba0d60ffda649060200160405180910390a450505092915050565b6112e97fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b600b80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff166101001790556040517f75884cdadc4a89e8b545db800057f06ec7f5338a08183c7ba515f2bfdd9fe1e1905f90a1565b5f8281527f02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b6268006020526040902060010154611378816137dd565b61138283836137e7565b50505050565b5f6113998363ffffffff168361383c565b9392505050565b73ffffffffffffffffffffffffffffffffffffffff811633146113ef576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d578282613987565b5f8181526003602052604081206002018054825b81811015611498575f60025f85848154811061142b5761142b615565565b5f91825260208083209091015473ffffffffffffffffffffffffffffffffffffffff168352828101939093526040918201812089825290925290205467ffffffffffffffff690100000000000000000090910416111561149057506001949350505050565b60010161140d565b505f949350505050565b6114cb7fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b60ff8216609b11611538576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601a60248201527f496e76616c6964205072656d69756d2050657263656e746167650000000000006044820152606401610dc6565b6040805160e0808201835263ffffffff8a81168084528a821660208086018290528b84168688018190528b851660608089018290528c87166080808b0182905260ff8e1660a0808d01829052998e1660c09c8d01819052600a80547fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000168b176401000000008b02177fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff166801000000000000000089027fffffffffffffffffffffffffffffffff00000000ffffffffffffffffffffffff16176c010000000000000000000000008802177fffffffffffffffffffffff0000000000ffffffffffffffffffffffffffffffff1670010000000000000000000000000000000086027fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff1617740100000000000000000000000000000000000000008402177fffffffffffffff00000000ffffffffffffffffffffffffffffffffffffffffff1675010000000000000000000000000000000000000000008302179055600b80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660011790558d51998a52968901979097529a8701939093528501529683019690965291810191909152918201929092527f55a28fde295f482c9f32d670c116103bca15724bcef4f18b35542e0553c35ad591015b60405180910390a150505050505050565b5f6117666139d3565b805490915060ff68010000000000000000820416159067ffffffffffffffff165f811580156117925750825b90505f8267ffffffffffffffff1660011480156117ae5750303b155b9050811580156117bc575080155b156117f3576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b84547fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000016600117855583156118545784547fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff16680100000000000000001785555b61185c6139fb565b6118646139fb565b61188e7fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c21775876137e7565b6118f4576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f4772616e7420726f6c65206661696c65640000000000000000000000000000006044820152606401610dc6565b6118fe5f876137e7565b611964576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f4772616e7420726f6c65206661696c65640000000000000000000000000000006044820152606401610dc6565b73ffffffffffffffffffffffffffffffffffffffff8716611a07576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602b60248201527f43616e6e6f7420736574207a65726f2061646472657373206173207369676e6160448201527f747572652073656e6465720000000000000000000000000000000000000000006064820152608401610dc6565b600180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff89161790558315611aa45784547fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff168555604051600181527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200161174c565b50505050505050565b5f610cec8263ffffffff163a61383c565b611ac6613a05565b611acf82613b09565b611ad98282613b32565b5050565b5f611ae6613c6b565b507f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5b90565b611b1461337f565b611b3d7fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b6008546c0100000000000000000000000090046bffffffffffffffffffffffff16611b69811515613cda565b600880547fffffffffffffffff000000000000000000000000ffffffffffffffffffffffff169055611ba9826bffffffffffffffffffffffff8316613d11565b50611bb360015f55565b50565b8051604051606091611bce9160200190815260200190565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152828252805160209182012090830152016040516020818303038152906040529050919050565b600b545f9060ff16611c8d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601a60248201527f436f6e7472616374206973206e6f7420636f6e666967757265640000000000006044820152606401610dc6565b600b54610100900460ff1615611cff576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601460248201527f436f6e74726163742069732064697361626c65640000000000000000000000006044820152606401610dc6565b610cec825f610d5c565b5f8281527fc1f6fe24621ce81ec5827caf0253cadb74709b061630e6b55e82371705932000602081905260408220611d419084613da7565b949350505050565b611d5161337f565b5f81815260036020526040902054611d7e9073ffffffffffffffffffffffffffffffffffffffff16613db2565b5f81815260046020526040812080546bffffffffffffffffffffffff1691349190611da98385615592565b92506101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055503460085f8282829054906101000a90046bffffffffffffffffffffffff16611dff9190615592565b92506101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550817f7603b205d03651ee812f803fccde89f1012e545a9c99f0abfea9cedd0fd8e902823484611e5c919061539f565b604080519283526020830191909152015b60405180910390a250611bb360015f55565b5f611e8861337f565b60055467ffffffffffffffff1633611ea16001436155b6565b6040517fffffffffffffffffffffffffffffffffffffffff000000000000000000000000606093841b81166020830152914060348201523090921b1660548201527fffffffffffffffff00000000000000000000000000000000000000000000000060c083901b166068820152607001604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081840301815291905280516020909101209150611f548160016155c9565b600580547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff928316179055604080515f808252606080830184526020808401838152848601848152898552600483528685209151825491516bffffffffffffffffffffffff9091167fffffffffffffffffffffffff0000000000000000000000000000000000000000928316176c01000000000000000000000000919099160297909717905584519182018552338252818101838152828601858152898552600383529590932082518154881673ffffffffffffffffffffffffffffffffffffffff918216178255935160018201805490981694169390931790955592518051929491926120729260028501920190614c16565b5061208291506006905084613dff565b5060405133815283907f1d3015d7ba850fa198dc7b1a3f5d42779313a681035f77c8c03764c61005518d9060200160405180910390a25050611b0960015f55565b5f8181527fc1f6fe24621ce81ec5827caf0253cadb74709b061630e6b55e82371705932000602081905260409091206060919061139990613e0a565b6121287fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b600b80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff1690556040517fc0f961051f97b04c496472d11cb6170d844e4b2c9dfd3b602a4fa0139712d484905f90a1565b6121a37fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b5f8181526003602052604090205473ffffffffffffffffffffffffffffffffffffffff166121d081613db2565b611ad982826133c0565b60605f6121e76006613e16565b9050808410612222576040517f1390f2a100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61222d848661539f565b90508181118061223b575083155b6122455780612247565b815b90505f61225486836155b6565b90508067ffffffffffffffff81111561226f5761226f614e7b565b604051908082528060200260200182016040528015612298578160200160208202803683370190505b5093505f5b818110156122df576122ba6122b2888361539f565b600690613da7565b8582815181106122cc576122cc615565565b602090810291909101015260010161229d565b5050505092915050565b6122f161337f565b5f8181526003602052604090205473ffffffffffffffffffffffffffffffffffffffff1661231e81613db2565b5f8281526003602052604090206001015473ffffffffffffffffffffffffffffffffffffffff1633146123a8575f82815260036020526040908190206001015490517fd084e97500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff9091166004820152602401610dc6565b5f828152600360209081526040918290208054337fffffffffffffffffffffffff000000000000000000000000000000000000000091821681178355600190920180549091169055825173ffffffffffffffffffffffffffffffffffffffff851681529182015283917fd4114ab6e9af9f597c52041f32d62dc57c5c4e4c0d4427006069635e216c93869101611e6d565b61244161337f565b61246a7fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b6009546bffffffffffffffffffffffff16612486811515613cda565b600980547fffffffffffffffffffffffffffffffffffffffff000000000000000000000000169055600880548291905f906124d09084906bffffffffffffffffffffffff166155e9565b92506101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550611ba982826bffffffffffffffffffffffff16613d11565b81612520816132eb565b61252861337f565b73ffffffffffffffffffffffffffffffffffffffff82165f9081526002602090815260408083208684529091529020805460ff16156125675750610d4e565b5f84815260036020526040902060020180547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9c016125d1576040517f05a48e0f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b815460017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0090911681178355815490810182555f82815260209081902090910180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff871690811790915560405190815286917f1e980d04aa7648e205713e5e8ea3808672ac163d10936d36f91b2c88ac1575e191015b60405180910390a25050610d5760015f55565b6126f66040518061012001604052805f81526020015f81526020015f63ffffffff1681526020015f81526020016060815260200160608152602001606081526020015f81526020015f73ffffffffffffffffffffffffffffffffffffffff1681525090565b5f828152600d602090815260409182902082516101208101845281548152600182015492810192909252600281015463ffffffff169282019290925260038201546060820152600482018054919291608084019190612754906153b2565b80601f0160208091040260200160405190810160405280929190818152602001828054612780906153b2565b80156127cb5780601f106127a2576101008083540402835291602001916127cb565b820191905f5260205f20905b8154815290600101906020018083116127ae57829003601f168201915b505050505081526020016005820180546127e4906153b2565b80601f0160208091040260200160405190810160405280929190818152602001828054612810906153b2565b801561285b5780601f106128325761010080835404028352916020019161285b565b820191905f5260205f20905b81548152906001019060200180831161283e57829003601f168201915b50505050508152602001600682018054612874906153b2565b80601f01602080910402602001604051908101604052809291908181526020018280546128a0906153b2565b80156128eb5780601f106128c2576101008083540402835291602001916128eb565b820191905f5260205f20905b8154815290600101906020018083116128ce57829003601f168201915b50505091835250506007820154602082015260089091015473ffffffffffffffffffffffffffffffffffffffff1660409091015292915050565b60015473ffffffffffffffffffffffffffffffffffffffff1633146129a6576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601d60248201527f4f6e6c79207369676e617475726553656e6465722063616e2063616c6c0000006044820152606401610dc6565b610d57838383613e1f565b5f8181527fc1f6fe24621ce81ec5827caf0253cadb74709b061630e6b55e8237170593200060208190526040822061139990613e16565b816129f2816132eb565b6129fa61337f565b612a03836113f9565b15612a3a576040517fb42f66e800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff82165f90815260026020908152604080832086845290915290205460ff16612ac2576040517f79bfd4010000000000000000000000000000000000000000000000000000000081526004810184905273ffffffffffffffffffffffffffffffffffffffff83166024820152604401610dc6565b5f838152600360205260408120600201805490915b81811015612c3d578473ffffffffffffffffffffffffffffffffffffffff16838281548110612b0857612b08615565565b5f9182526020909120015473ffffffffffffffffffffffffffffffffffffffff1603612c355782612b3a6001846155b6565b81548110612b4a57612b4a615565565b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16838281548110612b8457612b84615565565b905f5260205f20015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555082805480612bd957612bd961560d565b5f8281526020902081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff90810180547fffffffffffffffffffffffff0000000000000000000000000000000000000000169055019055612c3d565b600101612ad7565b5073ffffffffffffffffffffffffffffffffffffffff84165f81815260026020908152604080832089845282529182902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00169055905191825286917f32158c6058347c1601b2d12bc696ac6901d8a9a9aa3ba10c27ab0a983e8425a7910161267e565b6001546040517fcd802c91000000000000000000000000000000000000000000000000000000008152600481018390525f9173ffffffffffffffffffffffffffffffffffffffff169063cd802c9190602401602060405180830381865afa158015612d30573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cec919061563a565b5f8281527f02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b6268006020526040902060010154612d8d816137dd565b6113828383613987565b81612da1816132eb565b612da961337f565b5f838152600360205260409020600181015473ffffffffffffffffffffffffffffffffffffffff848116911614612e5c576001810180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff851690811790915560408051338152602081019290925285917f21a4dad170a6bf476c31bbcf4a16628295b0e450672eec25d7c93308e05344a1910160405180910390a25b50610d5760015f55565b5f81815260036020526040812054819073ffffffffffffffffffffffffffffffffffffffff166060612e9782613db2565b5f85815260046020908152604080832054600383529281902060020180548251818502810185019093528083526bffffffffffffffffffffffff8516946c01000000000000000000000000900467ffffffffffffffff16938793918391830182828015612f3857602002820191905f5260205f20905b815473ffffffffffffffffffffffffffffffffffffffff168152600190910190602001808311612f0d575b5050505050905093509350935093509193509193565b612f777fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b600180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff83169081179091556040517f229f6c3b095d683755a99ab458956747a8b7066c3dd42927d850631c34c238f1905f90a250565b6060600e805480602002602001604051908101604052809291908181526020015f905b8282101561324c575f84815260209081902060408051610120810182526009860290920180548352600181015493830193909352600283015463ffffffff16908201526003820154606082015260048201805491929160808401919061306d906153b2565b80601f0160208091040260200160405190810160405280929190818152602001828054613099906153b2565b80156130e45780601f106130bb576101008083540402835291602001916130e4565b820191905f5260205f20905b8154815290600101906020018083116130c757829003601f168201915b505050505081526020016005820180546130fd906153b2565b80601f0160208091040260200160405190810160405280929190818152602001828054613129906153b2565b80156131745780601f1061314b57610100808354040283529160200191613174565b820191905f5260205f20905b81548152906001019060200180831161315757829003601f168201915b5050505050815260200160068201805461318d906153b2565b80601f01602080910402602001604051908101604052809291908181526020018280546131b9906153b2565b80156132045780601f106131db57610100808354040283529160200191613204565b820191905f5260205f20905b8154815290600101906020018083116131e757829003601f168201915b5050509183525050600782015460208083019190915260089092015473ffffffffffffffffffffffffffffffffffffffff166040909101529082526001929092019101613008565b50505050905090565b5f7fffffffff0000000000000000000000000000000000000000000000000000000082167f7965db0b000000000000000000000000000000000000000000000000000000001480610cec57507f01ffc9a7000000000000000000000000000000000000000000000000000000007fffffffff00000000000000000000000000000000000000000000000000000000831614610cec565b5f8181526003602052604090205473ffffffffffffffffffffffffffffffffffffffff1661331881613db2565b3373ffffffffffffffffffffffffffffffffffffffff821614611ad9576040517fd8a3fb5200000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff82166004820152602401610dc6565b60025f54036133ba576040517f3ee5aeb500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60025f55565b5f6133ca83614017565b90506133e482826bffffffffffffffffffffffff16613d11565b6040805173ffffffffffffffffffffffffffffffffffffffff841681526bffffffffffffffffffffffff8316602082015284917f3784f77e8e883de95b5d47cd713ced01229fa74d118c0a462224bcb0516d43f1910160405180910390a2505050565b600a5463ffffffff90811690831611156134bd576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601a60248201527f43616c6c6261636b206761734c696d697420746f6f20686967680000000000006044820152606401610dc6565b80156136c6575f8181526003602052604090205473ffffffffffffffffffffffffffffffffffffffff166134f081613db2565b335f908152600260209081526040808320858452808352928190208151606081018352905460ff8116151580835267ffffffffffffffff6101008304811695840195909552690100000000000000000090910490931691810191909152906135da576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602160248201527f4e6f2061637469766520737562736372697074696f6e20666f722063616c6c6560448201527f72000000000000000000000000000000000000000000000000000000000000006064820152608401610dc6565b8060200180516135e990615659565b67ffffffffffffffff16905260408101805161360490615659565b67ffffffffffffffff9081169091525f85815260209384526040908190208351815495850151929094015183166901000000000000000000027fffffffffffffffffffffffffffffff0000000000000000ffffffffffffffffff92909316610100027fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000ff941515949094167fffffffffffffffffffffffffffffffffffffffffffffff000000000000000000909516949094179290921791909116179055505050565b5f6136d78363ffffffff163a61383c565b905080341015610d57576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600b60248201527f46656520746f6f206c6f770000000000000000000000000000000000000000006044820152606401610dc6565b6001546040517f95b8d0730000000000000000000000000000000000000000000000000000000081525f9173ffffffffffffffffffffffffffffffffffffffff16906395b8d0739061379d90879087908790600401615685565b6020604051808303815f875af11580156137b9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d4191906156bd565b611bb381336141cd565b5f7fc1f6fe24621ce81ec5827caf0253cadb74709b061630e6b55e82371705932000816138148585614273565b90508015611d41575f8581526020839052604090206138339085614391565b50949350505050565b6040805160e081018252600a5463ffffffff80821683526401000000008204811660208401526801000000000000000082048116938301939093526c0100000000000000000000000081048316606083015270010000000000000000000000000000000081048316608083015260ff7401000000000000000000000000000000000000000082041660a08301527501000000000000000000000000000000000000000000900490911660c08201525f90818361390257816060015163ffffffff16613904565b835b90505f613910866143b2565b63ffffffff16836080015163ffffffff1687856020015163ffffffff16010101820290505f61393c5f90565b90505f8460a0015160640160ff1690505f856040015163ffffffff1664e8d4a510000290505f816064848787010281613977576139776156d4565b04019a9950505050505050505050565b5f7fc1f6fe24621ce81ec5827caf0253cadb74709b061630e6b55e82371705932000816139b485856143c9565b90508015611d41575f85815260208390526040902061383390856144a5565b5f807ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00610cec565b613a036144c6565b565b3073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000161480613ad257507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16613ab97f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5473ffffffffffffffffffffffffffffffffffffffff1690565b73ffffffffffffffffffffffffffffffffffffffff1614155b15613a03576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611bb37fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217756137dd565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015613bb7575060408051601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201909252613bb4918101906156bd565b60015b613c05576040517f4c9c8ce300000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff83166004820152602401610dc6565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8114613c61576040517faa1d49a400000000000000000000000000000000000000000000000000000000815260048101829052602401610dc6565b610d578383614504565b3073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614613a03576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80611bb3576040517ff4d678b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8273ffffffffffffffffffffffffffffffffffffffff16826040515f6040518083038185875af1925050503d805f8114613d67576040519150601f19603f3d011682016040523d82523d5f602084013e613d6c565b606091505b5050905080610d57576040517f950b247900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6113998383614566565b73ffffffffffffffffffffffffffffffffffffffff8116611bb3576040517f1f6a65b600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f611399838361458c565b60605f611399836145d8565b5f610cec825490565b5f5a5f858152600d60205260409020600781015491925090613e9d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601960248201527f4e6f207265717565737420666f722072657175657374206964000000000000006044820152606401610dc6565b5f8484604051613eae929190615701565b604080519182900382206024830189905260448084018290528251808503909101815260649093019091526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f431ac6570000000000000000000000000000000000000000000000000000000017905260088401546002850154600a549294505f92613f7e92859273ffffffffffffffffffffffffffffffffffffffff9091169163ffffffff91821691750100000000000000000000000000000000000000000090910481169061463116565b5090508015613fd85760068401613f96878983615710565b50877fb74b3204a538cd8021662d42e794681ddc339924ef675b8fd11e9eaf6aa19eb5848989604051613fcb93929190615826565b60405180910390a2614003565b60405188907f8f67472dde2126ccd0315b75dc482a5a73acb228a395553f8ae6edde5a0ca4fa905f90a25b61400d8886614668565b5050505050505050565b5f8181526003602090815260408083206004909252822054600290910180546bffffffffffffffffffffffff909216929091905b818110156140d15760025f84838154811061406857614068615565565b5f91825260208083209091015473ffffffffffffffffffffffffffffffffffffffff1683528281019390935260409182018120888252909252902080547fffffffffffffffffffffffffffffff000000000000000000000000000000000016905560010161404b565b505f84815260036020526040812080547fffffffffffffffffffffffff000000000000000000000000000000000000000090811682556001820180549091169055906141206002830182614c9e565b50505f84815260046020526040902080547fffffffffffffffffffffffff0000000000000000000000000000000000000000169055614160600685614777565b506bffffffffffffffffffffffff8316156141c657600880548491905f906141979084906bffffffffffffffffffffffff166155e9565b92506101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055505b5050919050565b5f8281527f02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b6268006020908152604080832073ffffffffffffffffffffffffffffffffffffffff8516845290915290205460ff16611ad9576040517fe2517d3f00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8216600482015260248101839052604401610dc6565b5f8281527f02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b6268006020818152604080842073ffffffffffffffffffffffffffffffffffffffff8616855290915282205460ff16614388575f8481526020828152604080832073ffffffffffffffffffffffffffffffffffffffff87168452909152902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660011790556143243390565b73ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff16857f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a46001915050610cec565b5f915050610cec565b5f6113998373ffffffffffffffffffffffffffffffffffffffff841661458c565b5f6143be603f83615879565b610cec9060016158c5565b5f8281527f02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b6268006020818152604080842073ffffffffffffffffffffffffffffffffffffffff8616855290915282205460ff1615614388575f8481526020828152604080832073ffffffffffffffffffffffffffffffffffffffff8716808552925280832080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016905551339287917ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9190a46001915050610cec565b5f6113998373ffffffffffffffffffffffffffffffffffffffff841661477e565b6144ce614858565b613a03576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61450d82614876565b60405173ffffffffffffffffffffffffffffffffffffffff8316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a280511561455e57610d578282614944565b611ad96149c3565b5f825f01828154811061457b5761457b615565565b905f5260205f200154905092915050565b5f8181526001830160205260408120546145d157508154600181810184555f848152602080822090930184905584548482528286019093526040902091909155610cec565b505f610cec565b6060815f0180548060200260200160405190810160405280929190818152602001828054801561462557602002820191905f5260205f20905b815481526020019060010190808311614611575b50505050509050919050565b5f5f5a83811061465e5783900360408104810385101561465e575f5f885160208a015f8a8af19250600191505b5094509492505050565b5f61467283612691565b8051909150156147665780515f9081526004602052604090208054600c906146b3906c01000000000000000000000000900467ffffffffffffffff16615659565b825467ffffffffffffffff91821661010093840a908102908302199091161790925582015173ffffffffffffffffffffffffffffffffffffffff165f90815260026020908152604080832085518452909152902080549091600991614726916901000000000000000000909104166158e1565b91906101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055505f614757833a6149fb565b905061138281835f0151614a69565b610d578160200151825f0151614a69565b5f61139983835b5f8181526001830160205260408120548015614388575f6147a06001836155b6565b85549091505f906147b3906001906155b6565b9050808214614812575f865f0182815481106147d1576147d1615565565b905f5260205f200154905080875f0184815481106147f1576147f1615565565b5f918252602080832090910192909255918252600188019052604090208390555b85548690806148235761482361560d565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050610cec565b5f6148616139d3565b5468010000000000000000900460ff16919050565b8073ffffffffffffffffffffffffffffffffffffffff163b5f036148de576040517f4c9c8ce300000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff82166004820152602401610dc6565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc80547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff92909216919091179055565b60605f5f8473ffffffffffffffffffffffffffffffffffffffff168460405161496d9190615922565b5f60405180830381855af49150503d805f81146149a5576040519150601f19603f3d011682016040523d82523d5f602084013e6149aa565b606091505b50915091506149ba858383614b46565b95945050505050565b3415613a03576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5f5a600a54640100000000900463ffffffff1685010390508281025f600a5460649190920160ff740100000000000000000000000000000000000000008404168201020464e8d4a5100063ffffffff68010000000000000000909304929092169190910201949350505050565b8015614af1575f81815260046020526040902080546bffffffffffffffffffffffff90811690614a9d908516821015613cda565b81546bffffffffffffffffffffffff9185900382167fffffffffffffffffffffffffffffffffffffffff00000000000000000000000091821617909255600980548083168601909216919092161790555050565b600880546bffffffffffffffffffffffff6c0100000000000000000000000080830482168601909116027fffffffffffffffff000000000000000000000000ffffffffffffffffffffffff9091161790555050565b606082614b5b57614b5682614bd5565b611399565b8151158015614b7f575073ffffffffffffffffffffffffffffffffffffffff84163b155b15614bce576040517f9996b31500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff85166004820152602401610dc6565b5080611399565b805115614be457805160208201fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b828054828255905f5260205f20908101928215614c8e579160200282015b82811115614c8e57825182547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff909116178255602090920191600190910190614c34565b50614c9a929150614cb5565b5090565b5080545f8255905f5260205f2090810190611bb391905b5b80821115614c9a575f8155600101614cb6565b5f60208284031215614cd9575f5ffd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114611399575f5ffd5b73ffffffffffffffffffffffffffffffffffffffff81168114611bb3575f5ffd5b5f5f60408385031215614d3a575f5ffd5b823591506020830135614d4c81614d08565b809150509250929050565b803563ffffffff81168114614d6a575f5ffd5b919050565b5f5f60408385031215614d80575f5ffd5b614d8983614d57565b946020939093013593505050565b5f60208284031215614da7575f5ffd5b5035919050565b5f5f5f5f5f5f5f60e0888a031215614dc4575f5ffd5b614dcd88614d57565b9650614ddb60208901614d57565b9550614de960408901614d57565b9450614df760608901614d57565b9350614e0560808901614d57565b925060a088013560ff81168114614e1a575f5ffd5b9150614e2860c08901614d57565b905092959891949750929550565b5f5f60408385031215614e47575f5ffd5b8235614e5281614d08565b91506020830135614d4c81614d08565b5f60208284031215614e72575f5ffd5b61139982614d57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff81118282101715614eef57614eef614e7b565b604052919050565b5f5f60408385031215614f08575f5ffd5b8235614f1381614d08565b9150602083013567ffffffffffffffff811115614f2e575f5ffd5b8301601f81018513614f3e575f5ffd5b803567ffffffffffffffff811115614f5857614f58614e7b565b614f8960207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601614ea8565b818152866020838501011115614f9d575f5ffd5b816020840160208301375f602083830101528093505050509250929050565b5f60208284031215614fcc575f5ffd5b813561139981614d08565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b602081525f6113996020830184614fd7565b5f6040828403128015615046575f5ffd5b506040805190810167ffffffffffffffff8111828210171561506a5761506a614e7b565b60405282358152602083013561507f81614d08565b60208201529392505050565b5f5f6040838503121561509c575f5ffd5b50508035926020909101359150565b5f8151808452602084019350602083015f5b828110156150f157815173ffffffffffffffffffffffffffffffffffffffff168652602095860195909101906001016150bd565b5093949350505050565b602081525f61139960208301846150ab565b602080825282518282018190525f918401906040840190835b81811015615144578351835260209384019390920191600101615126565b509095945050505050565b80518252602081015160208301525f6040820151615175604085018263ffffffff169052565b50606082015160608401526080820151610120608085015261519b610120850182614fd7565b905060a083015184820360a08601526151b48282614fd7565b91505060c083015184820360c08601526151ce8282614fd7565b91505060e083015160e085015261010083015161520461010086018273ffffffffffffffffffffffffffffffffffffffff169052565b509392505050565b602081525f611399602083018461514f565b5f5f5f60408486031215615230575f5ffd5b83359250602084013567ffffffffffffffff81111561524d575f5ffd5b8401601f8101861361525d575f5ffd5b803567ffffffffffffffff811115615273575f5ffd5b866020828401011115615284575f5ffd5b939660209190910195509293505050565b6bffffffffffffffffffffffff8516815267ffffffffffffffff8416602082015273ffffffffffffffffffffffffffffffffffffffff83166040820152608060608201525f6152e760808301846150ab565b9695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015615366577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc087860301845261535185835161514f565b94506020938401939190910190600101615317565b50929695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b80820180821115610cec57610cec615372565b600181811c908216806153c657607f821691505b6020821081036153fd577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b50919050565b601f821115610d5757805f5260205f20601f840160051c810160208510156154285750805b601f840160051c820191505b81811015615447575f8155600101615434565b5050505050565b815167ffffffffffffffff81111561546857615468614e7b565b61547c8161547684546153b2565b84615403565b6020601f8211600181146154cd575f83156154975750848201515b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600385901b1c1916600184901b178455615447565b5f848152602081207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08516915b8281101561551a57878501518255602094850194600190920191016154fa565b508482101561555657868401517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600387901b60f8161c191681555b50505050600190811b01905550565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b6bffffffffffffffffffffffff8181168382160190811115610cec57610cec615372565b81810381811115610cec57610cec615372565b67ffffffffffffffff8181168382160190811115610cec57610cec615372565b6bffffffffffffffffffffffff8281168282160390811115610cec57610cec615372565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b5f6020828403121561564a575f5ffd5b81518015158114611399575f5ffd5b5f67ffffffffffffffff821667ffffffffffffffff810361567c5761567c615372565b60010192915050565b606081525f6156976060830186614fd7565b82810360208401526156a98186614fd7565b905082810360408401526152e78185614fd7565b5f602082840312156156cd575f5ffd5b5051919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b818382375f9101908152919050565b67ffffffffffffffff83111561572857615728614e7b565b61573c8361573683546153b2565b83615403565b5f601f84116001811461578c575f85156157565750838201355b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600387901b1c1916600186901b178355615447565b5f838152602081207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08716915b828110156157d957868501358255602094850194600190920191016157b9565b5086821015615814577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60f88860031b161c19848701351681555b505060018560011b0183555050505050565b83815260406020820152816040820152818360608301375f818301606090810191909152601f9092017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016010192915050565b5f63ffffffff8316806158b3577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b8063ffffffff84160491505092915050565b63ffffffff8181168382160190811115610cec57610cec615372565b5f67ffffffffffffffff8216806158fa576158fa615372565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0192915050565b5f82518060208501845e5f92019182525091905056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x03EW_5`\xE0\x1C\x80c\x91\xD1HT\x11a\x01\xB2W\x80c\xB2\xA7\xCA\xC5\x11a\0\xF2W\x80c\xCBc\x17\x97\x11a\0\x92W\x80c\xDA\xC8=)\x11a\0mW\x80c\xDA\xC8=)\x14a\x0C\tW\x80c\xDC1\x1D\xD3\x14a\x0C(W\x80c\xF8\xFA\rf\x14a\x0CWW\x80c\xFB\x1A\0*\x14a\x0CvW__\xFD[\x80c\xCBc\x17\x97\x14a\x0B\xACW\x80c\xCD\x80,\x91\x14a\x0B\xCBW\x80c\xD5Gt\x1F\x14a\x0B\xEAW__\xFD[\x80c\xC3\xF9\t\xD4\x11a\0\xCDW\x80c\xC3\xF9\t\xD4\x14a\n\xAAW\x80c\xC5\x83C\xEF\x14a\x0BBW\x80c\xC8\xDBe\x82\x14a\x0BnW\x80c\xCA\x15\xC8s\x14a\x0B\x8DW__\xFD[\x80c\xB2\xA7\xCA\xC5\x14a\nMW\x80c\xBD\x18ck\x14a\nlW\x80c\xBE\xC4\xC0\x8C\x14a\n\x8BW__\xFD[\x80c\xA3$j\xD3\x11a\x01]W\x80c\xAAC:\xFF\x11a\x018W\x80c\xAAC:\xFF\x14a\t\xA5W\x80c\xAD<\xB1\xCC\x14a\t\xC4W\x80c\xAE\xFB!/\x14a\n\x0CW\x80c\xAF\xFE\xD0\xE0\x14a\n8W__\xFD[\x80c\xA3$j\xD3\x14a\tGW\x80c\xA3\x90}q\x14a\tsW\x80c\xA6\x08\xA1\xE1\x14a\t\x87W__\xFD[\x80c\x9D@\xA6\xFD\x11a\x01\x8DW\x80c\x9D@\xA6\xFD\x14a\x08\xE7W\x80c\xA2\x17\xFD\xDF\x14a\t W\x80c\xA2\x1A#\xE4\x14a\t3W__\xFD[\x80c\x91\xD1HT\x14a\x08@W\x80c\x95\xB5\\\xFC\x14a\x08\xB0W\x80c\x99\\\xB3n\x14a\x08\xC3W__\xFD[\x80cH\\\xC9U\x11a\x02\x88W\x80cd\xD5\x1A*\x11a\x02(W\x80c}F\x81\x06\x11a\x02\x03W\x80c}F\x81\x06\x14a\x07uW\x80c\x81\x1E\xE3*\x14a\x07\xC6W\x80c\x8A\x1F\x16Z\x14a\x07\xD9W\x80c\x90\x10\xD0|\x14a\x08!W__\xFD[\x80cd\xD5\x1A*\x14a\x06\xFCW\x80cu\xB28\xFC\x14a\x07#W\x80cw[\x83\x9C\x14a\x07VW__\xFD[\x80cR\xD1\x90-\x11a\x02cW\x80cR\xD1\x90-\x14a\x06bW\x80cT#o\xB3\x14a\x06vW\x80cT\xFDMP\x14a\x06\x95W\x80cW\xA8\x07\n\x14a\x06\xE3W__\xFD[\x80cH\\\xC9U\x14a\x06\x11W\x80cK\x16\t5\x14a\x060W\x80cO\x1E\xF2\x86\x14a\x06OW__\xFD[\x80c/'p\xDB\x11a\x02\xF3W\x80c6V\x8A\xBE\x11a\x02\xCEW\x80c6V\x8A\xBE\x14a\x05\x80W\x80c;\xC3,u\x14a\x05\x9FW\x80cA\xAFl\x87\x14a\x05\xD3W\x80cE\xFACT\x14a\x05\xF2W__\xFD[\x80c/'p\xDB\x14a\x05.W\x80c//\xF1]\x14a\x05BW\x80c2U\xC4V\x14a\x05aW__\xFD[\x80c\x18\xE3\xDD'\x11a\x03#W\x80c\x18\xE3\xDD'\x14a\x04\x7FW\x80c\x1D\xA5<\x9F\x14a\x04\xC0W\x80c$\x8A\x9C\xA3\x14a\x04\xE1W__\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x03IW\x80c\x08\x80p\xF5\x14a\x03}W\x80c\n\xE0\x95@\x14a\x04^W[__\xFD[4\x80\x15a\x03TW__\xFD[Pa\x03ha\x03c6`\x04aL\xC9V[a\x0C\x97V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x88W__\xFD[P`\nTa\x04\x17\x90c\xFF\xFF\xFF\xFF\x80\x82\x16\x91d\x01\0\0\0\0\x81\x04\x82\x16\x91h\x01\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04\x82\x16\x91p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91`\xFFt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x16\x91u\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16\x87V[`@\x80Qc\xFF\xFF\xFF\xFF\x98\x89\x16\x81R\x96\x88\x16` \x88\x01R\x94\x87\x16\x94\x86\x01\x94\x90\x94R\x91\x85\x16``\x85\x01R\x84\x16`\x80\x84\x01R`\xFF\x16`\xA0\x83\x01R\x90\x91\x16`\xC0\x82\x01R`\xE0\x01a\x03tV[4\x80\x15a\x04iW__\xFD[Pa\x04}a\x04x6`\x04aM)V[a\x0C\xF2V[\0[4\x80\x15a\x04\x8AW__\xFD[P`\x08Ta\x04\xA3\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03tV[a\x04\xD3a\x04\xCE6`\x04aMoV[a\r\\V[`@Q\x90\x81R` \x01a\x03tV[4\x80\x15a\x04\xECW__\xFD[Pa\x04\xD3a\x04\xFB6`\x04aM\x97V[_\x90\x81R\x7F\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0` R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x059W__\xFD[Pa\x04}a\x12\xC0V[4\x80\x15a\x05MW__\xFD[Pa\x04}a\x05\\6`\x04aM)V[a\x13?V[4\x80\x15a\x05lW__\xFD[Pa\x04\xD3a\x05{6`\x04aMoV[a\x13\x88V[4\x80\x15a\x05\x8BW__\xFD[Pa\x04}a\x05\x9A6`\x04aM)V[a\x13\xA0V[4\x80\x15a\x05\xAAW__\xFD[P`\x08Ta\x04\xA3\x90l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x05\xDEW__\xFD[Pa\x03ha\x05\xED6`\x04aM\x97V[a\x13\xF9V[4\x80\x15a\x05\xFDW__\xFD[Pa\x04}a\x06\x0C6`\x04aM\xAEV[a\x14\xA2V[4\x80\x15a\x06\x1CW__\xFD[Pa\x04}a\x06+6`\x04aN6V[a\x17]V[4\x80\x15a\x06;W__\xFD[Pa\x04\xD3a\x06J6`\x04aNbV[a\x1A\xADV[a\x04}a\x06]6`\x04aN\xF7V[a\x1A\xBEV[4\x80\x15a\x06mW__\xFD[Pa\x04\xD3a\x1A\xDDV[4\x80\x15a\x06\x81W__\xFD[Pa\x04}a\x06\x906`\x04aO\xBCV[a\x1B\x0CV[4\x80\x15a\x06\xA0W__\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81R\x7F0.0.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R[`@Qa\x03t\x91\x90aP#V[4\x80\x15a\x06\xEEW__\xFD[P`\x0BTa\x03h\x90`\xFF\x16\x81V[4\x80\x15a\x07\x07W__\xFD[Pa\x07\x10`d\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x03tV[4\x80\x15a\x07.W__\xFD[Pa\x04\xD3\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17u\x81V[4\x80\x15a\x07aW__\xFD[Pa\x06\xD6a\x07p6`\x04aP5V[a\x1B\xB6V[4\x80\x15a\x07\x80W__\xFD[P`\x01Ta\x07\xA1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03tV[a\x04\xD3a\x07\xD46`\x04aNbV[a\x1C\x1FV[4\x80\x15a\x07\xE4W__\xFD[Pa\x06\xD6`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7FBN254\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[4\x80\x15a\x08,W__\xFD[Pa\x07\xA1a\x08;6`\x04aP\x8BV[a\x1D\tV[4\x80\x15a\x08KW__\xFD[Pa\x03ha\x08Z6`\x04aM)V[_\x91\x82R\x7F\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0` \x90\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[a\x04}a\x08\xBE6`\x04aM\x97V[a\x1DIV[4\x80\x15a\x08\xCEW__\xFD[P`\tTa\x04\xA3\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x08\xF2W__\xFD[P`\x05Ta\t\x07\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03tV[4\x80\x15a\t+W__\xFD[Pa\x04\xD3_\x81V[4\x80\x15a\t>W__\xFD[Pa\x04\xD3a\x1E\x7FV[4\x80\x15a\tRW__\xFD[Pa\tfa\ta6`\x04aM\x97V[a \xC3V[`@Qa\x03t\x91\x90aP\xFBV[4\x80\x15a\t~W__\xFD[Pa\x04}a \xFFV[4\x80\x15a\t\x92W__\xFD[P`\x0BTa\x03h\x90a\x01\0\x90\x04`\xFF\x16\x81V[4\x80\x15a\t\xB0W__\xFD[Pa\x04}a\t\xBF6`\x04aM\x97V[a!zV[4\x80\x15a\t\xCFW__\xFD[Pa\x06\xD6`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[4\x80\x15a\n\x17W__\xFD[Pa\n+a\n&6`\x04aP\x8BV[a!\xDAV[`@Qa\x03t\x91\x90aQ\rV[4\x80\x15a\nCW__\xFD[Pa\x04\xD3`\x0CT\x81V[4\x80\x15a\nXW__\xFD[Pa\x04}a\ng6`\x04aM\x97V[a\"\xE9V[4\x80\x15a\nwW__\xFD[Pa\x04}a\n\x866`\x04aO\xBCV[a$9V[4\x80\x15a\n\x96W__\xFD[Pa\x04}a\n\xA56`\x04aM)V[a%\x16V[4\x80\x15a\n\xB5W__\xFD[P`\nTc\xFF\xFF\xFF\xFF\x80\x82\x16\x91d\x01\0\0\0\0\x81\x04\x82\x16\x91h\x01\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04\x82\x16\x91p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91`\xFFt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x16\x91u\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16a\x04\x17V[4\x80\x15a\x0BMW__\xFD[Pa\x0Baa\x0B\\6`\x04aM\x97V[a&\x91V[`@Qa\x03t\x91\x90aR\x0CV[4\x80\x15a\x0ByW__\xFD[Pa\x04}a\x0B\x886`\x04aR\x1EV[a)%V[4\x80\x15a\x0B\x98W__\xFD[Pa\x04\xD3a\x0B\xA76`\x04aM\x97V[a)\xB1V[4\x80\x15a\x0B\xB7W__\xFD[Pa\x04}a\x0B\xC66`\x04aM)V[a)\xE8V[4\x80\x15a\x0B\xD6W__\xFD[Pa\x03ha\x0B\xE56`\x04aM\x97V[a,\xC3V[4\x80\x15a\x0B\xF5W__\xFD[Pa\x04}a\x0C\x046`\x04aM)V[a-TV[4\x80\x15a\x0C\x14W__\xFD[Pa\x04}a\x0C#6`\x04aM)V[a-\x97V[4\x80\x15a\x0C3W__\xFD[Pa\x0CGa\x0CB6`\x04aM\x97V[a.fV[`@Qa\x03t\x94\x93\x92\x91\x90aR\x95V[4\x80\x15a\x0CbW__\xFD[Pa\x04}a\x0Cq6`\x04aO\xBCV[a/NV[4\x80\x15a\x0C\x81W__\xFD[Pa\x0C\x8Aa/\xE5V[`@Qa\x03t\x91\x90aR\xF1V[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x0C\xECWPa\x0C\xEC\x82a2UV[\x92\x91PPV[\x81a\x0C\xFC\x81a2\xEBV[a\r\x04a3\x7FV[a\r\r\x83a\x13\xF9V[\x15a\rDW`@Q\x7F\xB4/f\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\rN\x83\x83a3\xC0V[a\rW`\x01_UV[PPPV[`\x0BT_\x90`\xFF\x16a\r\xCFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FContract is not configured\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x0BTa\x01\0\x90\x04`\xFF\x16\x15a\x0EAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FContract is disabled\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[\x81\x15\x15\x80a\x0ENWP_4\x11[a\x0E\xDAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FDirect funding required for requ`D\x82\x01R\x7Fest fulfillment callback\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\r\xC6V[a\x0E\xE4\x83\x83a4GV[`\x01`\x0C_\x82\x82Ta\x0E\xF6\x91\x90aS\x9FV[\x90\x91UPP`@\x80Qa\x01 \x81\x01\x82R\x83\x81R4` \x80\x83\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x86\x16\x82\x84\x01R_``\x83\x01\x81\x90R\x83Q\x80\x83\x01\x85R\x81\x81R`\x80\x84\x01R\x83Q\x80\x83\x01\x85R\x81\x81R`\xA0\x84\x01R\x83Q\x80\x83\x01\x85R\x81\x81R`\xC0\x84\x01R`\x0CT`\xE0\x84\x01\x81\x90R3a\x01\0\x85\x01\x81\x90R\x85Q\x80\x87\x01\x90\x96R\x90\x85R\x91\x84\x01\x91\x90\x91R\x90\x91a\x0F\x83\x90a\x1B\xB6V[`@\x80Q` \x80\x82\x01\x83R_\x82R\x82Q\x80\x84\x01\x90\x93R`\x05\x83R\x7FBN254\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01R\x91\x92Pa\x0F\xD0\x90\x83\x83a7CV[``\x84\x01\x81\x81R`\x80\x85\x01\x84\x81R`\xA0\x86\x01\x84\x90R_\x83\x81R`\r` \x90\x81R`@\x91\x82\x90 \x88Q\x81U\x90\x88\x01Q`\x01\x82\x01U\x90\x87\x01Q`\x02\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U\x91Q`\x03\x83\x01UQ\x91\x95P\x84\x91`\x04\x82\x01\x90a\x10Y\x90\x82aTNV[P`\xA0\x82\x01Q`\x05\x82\x01\x90a\x10n\x90\x82aTNV[P`\xC0\x82\x01Q`\x06\x82\x01\x90a\x10\x83\x90\x82aTNV[P`\xE0\x82\x01Q`\x07\x82\x01Ua\x01\0\x90\x91\x01Q`\x08\x90\x91\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`\x0E\x80T`\x01\x81\x01\x82U_\x91\x90\x91R\x83Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD`\t\x90\x92\x02\x91\x82\x01\x90\x81U` \x85\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFE\x83\x01U`@\x85\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFF\x83\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U``\x85\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC4\0\x83\x01U`\x80\x85\x01Q\x85\x92\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC4\x01\x01\x90a\x11\xF9\x90\x82aTNV[P`\xA0\x82\x01Q`\x05\x82\x01\x90a\x12\x0E\x90\x82aTNV[P`\xC0\x82\x01Q`\x06\x82\x01\x90a\x12#\x90\x82aTNV[P`\xE0\x82\x01Q`\x07\x82\x01Ua\x01\0\x90\x91\x01Q`\x08\x90\x91\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`\x0CT`@QB\x81R3\x91\x90\x86\x90\x7F\xEE\xE7\x19[l\xEE\x0F\xA7\x04L:\xF0\xB8o\xE2\xFE\xBB\x1D'\x03\xD7\x11\x91\xF4@R\xBA\r`\xFF\xDAd\x90` \x01`@Q\x80\x91\x03\x90\xA4PPP\x92\x91PPV[a\x12\xE9\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[`\x0B\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U`@Q\x7Fu\x88L\xDA\xDCJ\x89\xE8\xB5E\xDB\x80\0W\xF0n\xC7\xF53\x8A\x08\x18<{\xA5\x15\xF2\xBF\xDD\x9F\xE1\xE1\x90_\x90\xA1V[_\x82\x81R\x7F\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0` R`@\x90 `\x01\x01Ta\x13x\x81a7\xDDV[a\x13\x82\x83\x83a7\xE7V[PPPPV[_a\x13\x99\x83c\xFF\xFF\xFF\xFF\x16\x83a8<V[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14a\x13\xEFW`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\rW\x82\x82a9\x87V[_\x81\x81R`\x03` R`@\x81 `\x02\x01\x80T\x82[\x81\x81\x10\x15a\x14\x98W_`\x02_\x85\x84\x81T\x81\x10a\x14+Wa\x14+aUeV[_\x91\x82R` \x80\x83 \x90\x91\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x82\x81\x01\x93\x90\x93R`@\x91\x82\x01\x81 \x89\x82R\x90\x92R\x90 Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFi\x01\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16\x11\x15a\x14\x90WP`\x01\x94\x93PPPPV[`\x01\x01a\x14\rV[P_\x94\x93PPPPV[a\x14\xCB\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[`\xFF\x82\x16`\x9B\x11a\x158W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid Premium Percentage\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[`@\x80Q`\xE0\x80\x82\x01\x83Rc\xFF\xFF\xFF\xFF\x8A\x81\x16\x80\x84R\x8A\x82\x16` \x80\x86\x01\x82\x90R\x8B\x84\x16\x86\x88\x01\x81\x90R\x8B\x85\x16``\x80\x89\x01\x82\x90R\x8C\x87\x16`\x80\x80\x8B\x01\x82\x90R`\xFF\x8E\x16`\xA0\x80\x8D\x01\x82\x90R\x99\x8E\x16`\xC0\x9C\x8D\x01\x81\x90R`\n\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16\x8B\x17d\x01\0\0\0\0\x8B\x02\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16h\x01\0\0\0\0\0\0\0\0\x89\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x88\x02\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x02\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16u\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x02\x17\x90U`\x0B\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x8DQ\x99\x8AR\x96\x89\x01\x97\x90\x97R\x9A\x87\x01\x93\x90\x93R\x85\x01R\x96\x83\x01\x96\x90\x96R\x91\x81\x01\x91\x90\x91R\x91\x82\x01\x92\x90\x92R\x7FU\xA2\x8F\xDE)_H,\x9F2\xD6p\xC1\x16\x10;\xCA\x15rK\xCE\xF4\xF1\x8B5T.\x05S\xC3Z\xD5\x91\x01[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_a\x17fa9\xD3V[\x80T\x90\x91P`\xFFh\x01\0\0\0\0\0\0\0\0\x82\x04\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x81\x15\x80\x15a\x17\x92WP\x82[\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x17\xAEWP0;\x15[\x90P\x81\x15\x80\x15a\x17\xBCWP\x80\x15[\x15a\x17\xF3W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16`\x01\x17\x85U\x83\x15a\x18TW\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16h\x01\0\0\0\0\0\0\0\0\x17\x85U[a\x18\\a9\xFBV[a\x18da9\xFBV[a\x18\x8E\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17u\x87a7\xE7V[a\x18\xF4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FGrant role failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[a\x18\xFE_\x87a7\xE7V[a\x19dW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FGrant role failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16a\x1A\x07W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FCannot set zero address as signa`D\x82\x01R\x7Fture sender\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\r\xC6V[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x17\x90U\x83\x15a\x1A\xA4W\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01a\x17LV[PPPPPPPV[_a\x0C\xEC\x82c\xFF\xFF\xFF\xFF\x16:a8<V[a\x1A\xC6a:\x05V[a\x1A\xCF\x82a;\tV[a\x1A\xD9\x82\x82a;2V[PPV[_a\x1A\xE6a<kV[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC[\x90V[a\x1B\x14a3\x7FV[a\x1B=\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[`\x08Tl\x01\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1Bi\x81\x15\x15a<\xDAV[`\x08\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Ua\x1B\xA9\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a=\x11V[Pa\x1B\xB3`\x01_UV[PV[\x80Q`@Q``\x91a\x1B\xCE\x91` \x01\x90\x81R` \x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\x0BT_\x90`\xFF\x16a\x1C\x8DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FContract is not configured\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[`\x0BTa\x01\0\x90\x04`\xFF\x16\x15a\x1C\xFFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FContract is disabled\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[a\x0C\xEC\x82_a\r\\V[_\x82\x81R\x7F\xC1\xF6\xFE$b\x1C\xE8\x1E\xC5\x82|\xAF\x02S\xCA\xDBtp\x9B\x06\x160\xE6\xB5^\x827\x17\x05\x93 \0` \x81\x90R`@\x82 a\x1DA\x90\x84a=\xA7V[\x94\x93PPPPV[a\x1DQa3\x7FV[_\x81\x81R`\x03` R`@\x90 Ta\x1D~\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a=\xB2V[_\x81\x81R`\x04` R`@\x81 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x914\x91\x90a\x1D\xA9\x83\x85aU\x92V[\x92Pa\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP4`\x08_\x82\x82\x82\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1D\xFF\x91\x90aU\x92V[\x92Pa\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81\x7Fv\x03\xB2\x05\xD06Q\xEE\x81/\x80?\xCC\xDE\x89\xF1\x01.TZ\x9C\x99\xF0\xAB\xFE\xA9\xCE\xDD\x0F\xD8\xE9\x02\x824\x84a\x1E\\\x91\x90aS\x9FV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xA2Pa\x1B\xB3`\x01_UV[_a\x1E\x88a3\x7FV[`\x05Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163a\x1E\xA1`\x01CaU\xB6V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x93\x84\x1B\x81\x16` \x83\x01R\x91@`4\x82\x01R0\x90\x92\x1B\x16`T\x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0\x83\x90\x1B\x16`h\x82\x01R`p\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x91Pa\x1FT\x81`\x01aU\xC9V[`\x05\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x17\x90U`@\x80Q_\x80\x82R``\x80\x83\x01\x84R` \x80\x84\x01\x83\x81R\x84\x86\x01\x84\x81R\x89\x85R`\x04\x83R\x86\x85 \x91Q\x82T\x91Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x17l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90\x99\x16\x02\x97\x90\x97\x17\x90U\x84Q\x91\x82\x01\x85R3\x82R\x81\x81\x01\x83\x81R\x82\x86\x01\x85\x81R\x89\x85R`\x03\x83R\x95\x90\x93 \x82Q\x81T\x88\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x17\x82U\x93Q`\x01\x82\x01\x80T\x90\x98\x16\x94\x16\x93\x90\x93\x17\x90\x95U\x92Q\x80Q\x92\x94\x91\x92a r\x92`\x02\x85\x01\x92\x01\x90aL\x16V[Pa \x82\x91P`\x06\x90P\x84a=\xFFV[P`@Q3\x81R\x83\x90\x7F\x1D0\x15\xD7\xBA\x85\x0F\xA1\x98\xDC{\x1A?]Bw\x93\x13\xA6\x81\x03_w\xC8\xC07d\xC6\x10\x05Q\x8D\x90` \x01`@Q\x80\x91\x03\x90\xA2PPa\x1B\t`\x01_UV[_\x81\x81R\x7F\xC1\xF6\xFE$b\x1C\xE8\x1E\xC5\x82|\xAF\x02S\xCA\xDBtp\x9B\x06\x160\xE6\xB5^\x827\x17\x05\x93 \0` \x81\x90R`@\x90\x91 ``\x91\x90a\x13\x99\x90a>\nV[a!(\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[`\x0B\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q\x7F\xC0\xF9a\x05\x1F\x97\xB0LIdr\xD1\x1C\xB6\x17\r\x84NK,\x9D\xFD;`*O\xA0\x13\x97\x12\xD4\x84\x90_\x90\xA1V[a!\xA3\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[_\x81\x81R`\x03` R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a!\xD0\x81a=\xB2V[a\x1A\xD9\x82\x82a3\xC0V[``_a!\xE7`\x06a>\x16V[\x90P\x80\x84\x10a\"\"W`@Q\x7F\x13\x90\xF2\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\"-\x84\x86aS\x9FV[\x90P\x81\x81\x11\x80a\";WP\x83\x15[a\"EW\x80a\"GV[\x81[\x90P_a\"T\x86\x83aU\xB6V[\x90P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"oWa\"oaN{V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\x98W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x93P_[\x81\x81\x10\x15a\"\xDFWa\"\xBAa\"\xB2\x88\x83aS\x9FV[`\x06\x90a=\xA7V[\x85\x82\x81Q\x81\x10a\"\xCCWa\"\xCCaUeV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\"\x9DV[PPPP\x92\x91PPV[a\"\xF1a3\x7FV[_\x81\x81R`\x03` R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a#\x1E\x81a=\xB2V[_\x82\x81R`\x03` R`@\x90 `\x01\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a#\xA8W_\x82\x81R`\x03` R`@\x90\x81\x90 `\x01\x01T\x90Q\x7F\xD0\x84\xE9u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\r\xC6V[_\x82\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x80T3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x82\x16\x81\x17\x83U`\x01\x90\x92\x01\x80T\x90\x91\x16\x90U\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x91\x82\x01R\x83\x91\x7F\xD4\x11J\xB6\xE9\xAF\x9FY|R\x04\x1F2\xD6-\xC5|\\NL\rD'\0`ic^!l\x93\x86\x91\x01a\x1EmV[a$Aa3\x7FV[a$j\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[`\tTk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a$\x86\x81\x15\x15a<\xDAV[`\t\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U`\x08\x80T\x82\x91\x90_\x90a$\xD0\x90\x84\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aU\xE9V[\x92Pa\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\x1B\xA9\x82\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a=\x11V[\x81a% \x81a2\xEBV[a%(a3\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 \x80T`\xFF\x16\x15a%gWPa\rNV[_\x84\x81R`\x03` R`@\x90 `\x02\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x9C\x01a%\xD1W`@Q\x7F\x05\xA4\x8E\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81T`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x90\x91\x16\x81\x17\x83U\x81T\x90\x81\x01\x82U_\x82\x81R` \x90\x81\x90 \x90\x91\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x86\x91\x7F\x1E\x98\r\x04\xAAvH\xE2\x05q>^\x8E\xA3\x80\x86r\xAC\x16=\x10\x93m6\xF9\x1B,\x88\xAC\x15u\xE1\x91\x01[`@Q\x80\x91\x03\x90\xA2PPa\rW`\x01_UV[a&\xF6`@Q\x80a\x01 \x01`@R\x80_\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_\x82\x81R`\r` \x90\x81R`@\x91\x82\x90 \x82Qa\x01 \x81\x01\x84R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01Tc\xFF\xFF\xFF\xFF\x16\x92\x82\x01\x92\x90\x92R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01\x80T\x91\x92\x91`\x80\x84\x01\x91\x90a'T\x90aS\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta'\x80\x90aS\xB2V[\x80\x15a'\xCBW\x80`\x1F\x10a'\xA2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a'\xCBV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a'\xAEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x05\x82\x01\x80Ta'\xE4\x90aS\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(\x10\x90aS\xB2V[\x80\x15a([W\x80`\x1F\x10a(2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a([V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(>W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta(t\x90aS\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(\xA0\x90aS\xB2V[\x80\x15a(\xEBW\x80`\x1F\x10a(\xC2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(\xEBV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(\xCEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x07\x82\x01T` \x82\x01R`\x08\x90\x91\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x90\x91\x01R\x92\x91PPV[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a)\xA6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FOnly signatureSender can call\0\0\0`D\x82\x01R`d\x01a\r\xC6V[a\rW\x83\x83\x83a>\x1FV[_\x81\x81R\x7F\xC1\xF6\xFE$b\x1C\xE8\x1E\xC5\x82|\xAF\x02S\xCA\xDBtp\x9B\x06\x160\xE6\xB5^\x827\x17\x05\x93 \0` \x81\x90R`@\x82 a\x13\x99\x90a>\x16V[\x81a)\xF2\x81a2\xEBV[a)\xFAa3\x7FV[a*\x03\x83a\x13\xF9V[\x15a*:W`@Q\x7F\xB4/f\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T`\xFF\x16a*\xC2W`@Q\x7Fy\xBF\xD4\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`$\x82\x01R`D\x01a\r\xC6V[_\x83\x81R`\x03` R`@\x81 `\x02\x01\x80T\x90\x91[\x81\x81\x10\x15a,=W\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x82\x81T\x81\x10a+\x08Wa+\x08aUeV[_\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a,5W\x82a+:`\x01\x84aU\xB6V[\x81T\x81\x10a+JWa+JaUeV[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x82\x81T\x81\x10a+\x84Wa+\x84aUeV[\x90_R` _ \x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82\x80T\x80a+\xD9Wa+\xD9aV\rV[_\x82\x81R` \x90 \x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U\x01\x90Ua,=V[`\x01\x01a*\xD7V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x89\x84R\x82R\x91\x82\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90U\x90Q\x91\x82R\x86\x91\x7F2\x15\x8C`X4|\x16\x01\xB2\xD1+\xC6\x96\xACi\x01\xD8\xA9\xA9\xAA;\xA1\x0C'\xAB\n\x98>\x84%\xA7\x91\x01a&~V[`\x01T`@Q\x7F\xCD\x80,\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R_\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xCD\x80,\x91\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xEC\x91\x90aV:V[_\x82\x81R\x7F\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0` R`@\x90 `\x01\x01Ta-\x8D\x81a7\xDDV[a\x13\x82\x83\x83a9\x87V[\x81a-\xA1\x81a2\xEBV[a-\xA9a3\x7FV[_\x83\x81R`\x03` R`@\x90 `\x01\x81\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16\x91\x16\x14a.\\W`\x01\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90\x81\x17\x90\x91U`@\x80Q3\x81R` \x81\x01\x92\x90\x92R\x85\x91\x7F!\xA4\xDA\xD1p\xA6\xBFGl1\xBB\xCFJ\x16b\x82\x95\xB0\xE4Pg.\xEC%\xD7\xC93\x08\xE0SD\xA1\x91\x01`@Q\x80\x91\x03\x90\xA2[Pa\rW`\x01_UV[_\x81\x81R`\x03` R`@\x81 T\x81\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``a.\x97\x82a=\xB2V[_\x85\x81R`\x04` \x90\x81R`@\x80\x83 T`\x03\x83R\x92\x81\x90 `\x02\x01\x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x94l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93\x87\x93\x91\x83\x91\x83\x01\x82\x82\x80\x15a/8W` \x02\x82\x01\x91\x90_R` _ \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a/\rW[PPPPP\x90P\x93P\x93P\x93P\x93P\x91\x93P\x91\x93V[a/w\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\"\x9Fl;\t]h7U\xA9\x9A\xB4X\x95gG\xA8\xB7\x06l=\xD4)'\xD8Pc\x1C4\xC28\xF1\x90_\x90\xA2PV[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a2LW_\x84\x81R` \x90\x81\x90 `@\x80Qa\x01 \x81\x01\x82R`\t\x86\x02\x90\x92\x01\x80T\x83R`\x01\x81\x01T\x93\x83\x01\x93\x90\x93R`\x02\x83\x01Tc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01\x80T\x91\x92\x91`\x80\x84\x01\x91\x90a0m\x90aS\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta0\x99\x90aS\xB2V[\x80\x15a0\xE4W\x80`\x1F\x10a0\xBBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0\xE4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a0\xC7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x05\x82\x01\x80Ta0\xFD\x90aS\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta1)\x90aS\xB2V[\x80\x15a1tW\x80`\x1F\x10a1KWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a1tV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a1WW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta1\x8D\x90aS\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta1\xB9\x90aS\xB2V[\x80\x15a2\x04W\x80`\x1F\x10a1\xDBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a2\x04V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a1\xE7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x07\x82\x01T` \x80\x83\x01\x91\x90\x91R`\x08\x90\x92\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x90\x91\x01R\x90\x82R`\x01\x92\x90\x92\x01\x91\x01a0\x08V[PPPP\x90P\x90V[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x0C\xECWP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14a\x0C\xECV[_\x81\x81R`\x03` R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a3\x18\x81a=\xB2V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a\x1A\xD9W`@Q\x7F\xD8\xA3\xFBR\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\r\xC6V[`\x02_T\x03a3\xBAW`@Q\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_UV[_a3\xCA\x83a@\x17V[\x90Pa3\xE4\x82\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a=\x11V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R\x84\x91\x7F7\x84\xF7~\x8E\x88=\xE9[]G\xCDq<\xED\x01\"\x9F\xA7M\x11\x8C\nF\"$\xBC\xB0QmC\xF1\x91\x01`@Q\x80\x91\x03\x90\xA2PPPV[`\nTc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x83\x16\x11\x15a4\xBDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FCallback gasLimit too high\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[\x80\x15a6\xC6W_\x81\x81R`\x03` R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a4\xF0\x81a=\xB2V[3_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x85\x84R\x80\x83R\x92\x81\x90 \x81Q``\x81\x01\x83R\x90T`\xFF\x81\x16\x15\x15\x80\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\0\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95Ri\x01\0\0\0\0\0\0\0\0\0\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x90a5\xDAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FNo active subscription for calle`D\x82\x01R\x7Fr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\r\xC6V[\x80` \x01\x80Qa5\xE9\x90aVYV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90R`@\x81\x01\x80Qa6\x04\x90aVYV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x91R_\x85\x81R` \x93\x84R`@\x90\x81\x90 \x83Q\x81T\x95\x85\x01Q\x92\x90\x94\x01Q\x83\x16i\x01\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x93\x16a\x01\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\x94\x15\x15\x94\x90\x94\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x90\x95\x16\x94\x90\x94\x17\x92\x90\x92\x17\x91\x90\x91\x16\x17\x90UPPPV[_a6\xD7\x83c\xFF\xFF\xFF\xFF\x16:a8<V[\x90P\x804\x10\x15a\rWW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7FFee too low\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[`\x01T`@Q\x7F\x95\xB8\xD0s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\x95\xB8\xD0s\x90a7\x9D\x90\x87\x90\x87\x90\x87\x90`\x04\x01aV\x85V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a7\xB9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DA\x91\x90aV\xBDV[a\x1B\xB3\x813aA\xCDV[_\x7F\xC1\xF6\xFE$b\x1C\xE8\x1E\xC5\x82|\xAF\x02S\xCA\xDBtp\x9B\x06\x160\xE6\xB5^\x827\x17\x05\x93 \0\x81a8\x14\x85\x85aBsV[\x90P\x80\x15a\x1DAW_\x85\x81R` \x83\x90R`@\x90 a83\x90\x85aC\x91V[P\x94\x93PPPPV[`@\x80Q`\xE0\x81\x01\x82R`\nTc\xFF\xFF\xFF\xFF\x80\x82\x16\x83Rd\x01\0\0\0\0\x82\x04\x81\x16` \x84\x01Rh\x01\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x93\x83\x01\x93\x90\x93Rl\x01\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04\x83\x16``\x83\x01Rp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04\x83\x16`\x80\x83\x01R`\xFFt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x16`\xA0\x83\x01Ru\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x90\x91\x16`\xC0\x82\x01R_\x90\x81\x83a9\x02W\x81``\x01Qc\xFF\xFF\xFF\xFF\x16a9\x04V[\x83[\x90P_a9\x10\x86aC\xB2V[c\xFF\xFF\xFF\xFF\x16\x83`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x87\x85` \x01Qc\xFF\xFF\xFF\xFF\x16\x01\x01\x01\x82\x02\x90P_a9<_\x90V[\x90P_\x84`\xA0\x01Q`d\x01`\xFF\x16\x90P_\x85`@\x01Qc\xFF\xFF\xFF\xFF\x16d\xE8\xD4\xA5\x10\0\x02\x90P_\x81`d\x84\x87\x87\x01\x02\x81a9wWa9waV\xD4V[\x04\x01\x9A\x99PPPPPPPPPPV[_\x7F\xC1\xF6\xFE$b\x1C\xE8\x1E\xC5\x82|\xAF\x02S\xCA\xDBtp\x9B\x06\x160\xE6\xB5^\x827\x17\x05\x93 \0\x81a9\xB4\x85\x85aC\xC9V[\x90P\x80\x15a\x1DAW_\x85\x81R` \x83\x90R`@\x90 a83\x90\x85aD\xA5V[_\x80\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0a\x0C\xECV[a:\x03aD\xC6V[V[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a:\xD2WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a:\xB9\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a:\x03W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1B\xB3\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17ua7\xDDV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a;\xB7WP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra;\xB4\x91\x81\x01\x90aV\xBDV[`\x01[a<\x05W`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x01a\r\xC6V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a<aW`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\r\xC6V[a\rW\x83\x83aE\x04V[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a:\x03W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x1B\xB3W`@Q\x7F\xF4\xD6x\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a=gW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a=lV[``\x91P[PP\x90P\x80a\rWW`@Q\x7F\x95\x0B$y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13\x99\x83\x83aEfV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x1B\xB3W`@Q\x7F\x1Fje\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13\x99\x83\x83aE\x8CV[``_a\x13\x99\x83aE\xD8V[_a\x0C\xEC\x82T\x90V[_Z_\x85\x81R`\r` R`@\x90 `\x07\x81\x01T\x91\x92P\x90a>\x9DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FNo request for request id\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xC6V[_\x84\x84`@Qa>\xAE\x92\x91\x90aW\x01V[`@\x80Q\x91\x82\x90\x03\x82 `$\x83\x01\x89\x90R`D\x80\x84\x01\x82\x90R\x82Q\x80\x85\x03\x90\x91\x01\x81R`d\x90\x93\x01\x90\x91R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FC\x1A\xC6W\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x08\x84\x01T`\x02\x85\x01T`\nT\x92\x94P_\x92a?~\x92\x85\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91c\xFF\xFF\xFF\xFF\x91\x82\x16\x91u\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04\x81\x16\x90aF1\x16V[P\x90P\x80\x15a?\xD8W`\x06\x84\x01a?\x96\x87\x89\x83aW\x10V[P\x87\x7F\xB7K2\x04\xA58\xCD\x80!f-B\xE7\x94h\x1D\xDC3\x99$\xEFg[\x8F\xD1\x1E\x9E\xAFj\xA1\x9E\xB5\x84\x89\x89`@Qa?\xCB\x93\x92\x91\x90aX&V[`@Q\x80\x91\x03\x90\xA2a@\x03V[`@Q\x88\x90\x7F\x8FgG-\xDE!&\xCC\xD01[u\xDCH*Zs\xAC\xB2(\xA3\x95U?\x8A\xE6\xED\xDEZ\x0C\xA4\xFA\x90_\x90\xA2[a@\r\x88\x86aFhV[PPPPPPPPV[_\x81\x81R`\x03` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 T`\x02\x90\x91\x01\x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x92\x90\x91\x90[\x81\x81\x10\x15a@\xD1W`\x02_\x84\x83\x81T\x81\x10a@hWa@haUeV[_\x91\x82R` \x80\x83 \x90\x91\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x82\x81\x01\x93\x90\x93R`@\x91\x82\x01\x81 \x88\x82R\x90\x92R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U`\x01\x01a@KV[P_\x84\x81R`\x03` R`@\x81 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x82U`\x01\x82\x01\x80T\x90\x91\x16\x90U\x90aA `\x02\x83\x01\x82aL\x9EV[PP_\x84\x81R`\x04` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90UaA``\x06\x85aGwV[Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x15aA\xC6W`\x08\x80T\x84\x91\x90_\x90aA\x97\x90\x84\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aU\xE9V[\x92Pa\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP[PP\x91\x90PV[_\x82\x81R\x7F\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x1A\xD9W`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\r\xC6V[_\x82\x81R\x7F\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0` \x81\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x85R\x90\x91R\x82 T`\xFF\x16aC\x88W_\x84\x81R` \x82\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UaC$3\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\x0C\xECV[_\x91PPa\x0C\xECV[_a\x13\x99\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16aE\x8CV[_aC\xBE`?\x83aXyV[a\x0C\xEC\x90`\x01aX\xC5V[_\x82\x81R\x7F\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0` \x81\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x85R\x90\x91R\x82 T`\xFF\x16\x15aC\x88W_\x84\x81R` \x82\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\x0C\xECV[_a\x13\x99\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16aG~V[aD\xCEaHXV[a:\x03W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aE\r\x82aHvV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15aE^Wa\rW\x82\x82aIDV[a\x1A\xD9aI\xC3V[_\x82_\x01\x82\x81T\x81\x10aE{WaE{aUeV[\x90_R` _ \x01T\x90P\x92\x91PPV[_\x81\x81R`\x01\x83\x01` R`@\x81 TaE\xD1WP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x0C\xECV[P_a\x0C\xECV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15aF%W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11aF\x11W[PPPPP\x90P\x91\x90PV[__Z\x83\x81\x10aF^W\x83\x90\x03`@\x81\x04\x81\x03\x85\x10\x15aF^W__\x88Q` \x8A\x01_\x8A\x8A\xF1\x92P`\x01\x91P[P\x94P\x94\x92PPPV[_aFr\x83a&\x91V[\x80Q\x90\x91P\x15aGfW\x80Q_\x90\x81R`\x04` R`@\x90 \x80T`\x0C\x90aF\xB3\x90l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aVYV[\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16a\x01\0\x93\x84\n\x90\x81\x02\x90\x83\x02\x19\x90\x91\x16\x17\x90\x92U\x82\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x85Q\x84R\x90\x91R\x90 \x80T\x90\x91`\t\x91aG&\x91i\x01\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16aX\xE1V[\x91\x90a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_aGW\x83:aI\xFBV[\x90Pa\x13\x82\x81\x83_\x01QaJiV[a\rW\x81` \x01Q\x82_\x01QaJiV[_a\x13\x99\x83\x83[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aC\x88W_aG\xA0`\x01\x83aU\xB6V[\x85T\x90\x91P_\x90aG\xB3\x90`\x01\x90aU\xB6V[\x90P\x80\x82\x14aH\x12W_\x86_\x01\x82\x81T\x81\x10aG\xD1WaG\xD1aUeV[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10aG\xF1WaG\xF1aUeV[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80aH#WaH#aV\rV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x0C\xECV[_aHaa9\xD3V[Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x91\x90PV[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;_\x03aH\xDEW`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\r\xC6V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``__\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@QaIm\x91\x90aY\"V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aI\xA5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aI\xAAV[``\x91P[P\x91P\x91PaI\xBA\x85\x83\x83aKFV[\x95\x94PPPPPV[4\x15a:\x03W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__Z`\nTd\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x85\x01\x03\x90P\x82\x81\x02_`\nT`d\x91\x90\x92\x01`\xFFt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x04\x16\x82\x01\x02\x04d\xE8\xD4\xA5\x10\0c\xFF\xFF\xFF\xFFh\x01\0\0\0\0\0\0\0\0\x90\x93\x04\x92\x90\x92\x16\x91\x90\x91\x02\x01\x94\x93PPPPV[\x80\x15aJ\xF1W_\x81\x81R`\x04` R`@\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90aJ\x9D\x90\x85\x16\x82\x10\x15a<\xDAV[\x81Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x85\x90\x03\x82\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x91\x82\x16\x17\x90\x92U`\t\x80T\x80\x83\x16\x86\x01\x90\x92\x16\x91\x90\x92\x16\x17\x90UPPV[`\x08\x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFl\x01\0\0\0\0\0\0\0\0\0\0\0\0\x80\x83\x04\x82\x16\x86\x01\x90\x91\x16\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x90UPPV[``\x82aK[WaKV\x82aK\xD5V[a\x13\x99V[\x81Q\x15\x80\x15aK\x7FWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;\x15[\x15aK\xCEW`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`$\x01a\r\xC6V[P\x80a\x13\x99V[\x80Q\x15aK\xE4W\x80Q` \x82\x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aL\x8EW\x91` \x02\x82\x01[\x82\x81\x11\x15aL\x8EW\x82Q\x82T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90aL4V[PaL\x9A\x92\x91PaL\xB5V[P\x90V[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a\x1B\xB3\x91\x90[[\x80\x82\x11\x15aL\x9AW_\x81U`\x01\x01aL\xB6V[_` \x82\x84\x03\x12\x15aL\xD9W__\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x13\x99W__\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1B\xB3W__\xFD[__`@\x83\x85\x03\x12\x15aM:W__\xFD[\x825\x91P` \x83\x015aML\x81aM\x08V[\x80\x91PP\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14aMjW__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15aM\x80W__\xFD[aM\x89\x83aMWV[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15aM\xA7W__\xFD[P5\x91\x90PV[_______`\xE0\x88\x8A\x03\x12\x15aM\xC4W__\xFD[aM\xCD\x88aMWV[\x96PaM\xDB` \x89\x01aMWV[\x95PaM\xE9`@\x89\x01aMWV[\x94PaM\xF7``\x89\x01aMWV[\x93PaN\x05`\x80\x89\x01aMWV[\x92P`\xA0\x88\x015`\xFF\x81\x16\x81\x14aN\x1AW__\xFD[\x91PaN(`\xC0\x89\x01aMWV[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[__`@\x83\x85\x03\x12\x15aNGW__\xFD[\x825aNR\x81aM\x08V[\x91P` \x83\x015aML\x81aM\x08V[_` \x82\x84\x03\x12\x15aNrW__\xFD[a\x13\x99\x82aMWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aN\xEFWaN\xEFaN{V[`@R\x91\x90PV[__`@\x83\x85\x03\x12\x15aO\x08W__\xFD[\x825aO\x13\x81aM\x08V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aO.W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aO>W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aOXWaOXaN{V[aO\x89` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01aN\xA8V[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15aO\x9DW__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15aO\xCCW__\xFD[\x815a\x13\x99\x81aM\x08V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x13\x99` \x83\x01\x84aO\xD7V[_`@\x82\x84\x03\x12\x80\x15aPFW__\xFD[P`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aPjWaPjaN{V[`@R\x825\x81R` \x83\x015aP\x7F\x81aM\x08V[` \x82\x01R\x93\x92PPPV[__`@\x83\x85\x03\x12\x15aP\x9CW__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aP\xF1W\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aP\xBDV[P\x93\x94\x93PPPPV[` \x81R_a\x13\x99` \x83\x01\x84aP\xABV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aQDW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aQ&V[P\x90\x95\x94PPPPPV[\x80Q\x82R` \x81\x01Q` \x83\x01R_`@\x82\x01QaQu`@\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P``\x82\x01Q``\x84\x01R`\x80\x82\x01Qa\x01 `\x80\x85\x01RaQ\x9Ba\x01 \x85\x01\x82aO\xD7V[\x90P`\xA0\x83\x01Q\x84\x82\x03`\xA0\x86\x01RaQ\xB4\x82\x82aO\xD7V[\x91PP`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01RaQ\xCE\x82\x82aO\xD7V[\x91PP`\xE0\x83\x01Q`\xE0\x85\x01Ra\x01\0\x83\x01QaR\x04a\x01\0\x86\x01\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[P\x93\x92PPPV[` \x81R_a\x13\x99` \x83\x01\x84aQOV[___`@\x84\x86\x03\x12\x15aR0W__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aRMW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13aR]W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aRsW__\xFD[\x86` \x82\x84\x01\x01\x11\x15aR\x84W__\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16` \x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`@\x82\x01R`\x80``\x82\x01R_aR\xE7`\x80\x83\x01\x84aP\xABV[\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aSfW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x87\x86\x03\x01\x84RaSQ\x85\x83QaQOV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aS\x17V[P\x92\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0C\xECWa\x0C\xECaSrV[`\x01\x81\x81\x1C\x90\x82\x16\x80aS\xC6W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aS\xFDW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\rWW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aT(WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15aTGW_\x81U`\x01\x01aT4V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aThWaThaN{V[aT|\x81aTv\x84TaS\xB2V[\x84aT\x03V[` `\x1F\x82\x11`\x01\x81\x14aT\xCDW_\x83\x15aT\x97WP\x84\x82\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84UaTGV[_\x84\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x85\x16\x91[\x82\x81\x10\x15aU\x1AW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aT\xFAV[P\x84\x82\x10\x15aUVW\x86\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0C\xECWa\x0C\xECaSrV[\x81\x81\x03\x81\x81\x11\x15a\x0C\xECWa\x0C\xECaSrV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0C\xECWa\x0C\xECaSrV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0C\xECWa\x0C\xECaSrV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aVJW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x13\x99W__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x03aV|WaV|aSrV[`\x01\x01\x92\x91PPV[``\x81R_aV\x97``\x83\x01\x86aO\xD7V[\x82\x81\x03` \x84\x01RaV\xA9\x81\x86aO\xD7V[\x90P\x82\x81\x03`@\x84\x01RaR\xE7\x81\x85aO\xD7V[_` \x82\x84\x03\x12\x15aV\xCDW__\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15aW(WaW(aN{V[aW<\x83aW6\x83TaS\xB2V[\x83aT\x03V[_`\x1F\x84\x11`\x01\x81\x14aW\x8CW_\x85\x15aWVWP\x83\x82\x015[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83UaTGV[_\x83\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x87\x16\x91[\x82\x81\x10\x15aW\xD9W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aW\xB9V[P\x86\x82\x10\x15aX\x14W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[\x83\x81R`@` \x82\x01R\x81`@\x82\x01R\x81\x83``\x83\x017_\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x01\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x83\x16\x80aX\xB3W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x80c\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0C\xECWa\x0C\xECaSrV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80aX\xFAWaX\xFAaSrV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x92\x91PPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AccessControlBadConfirmation()` and selector `0x6697b232`.
```solidity
error AccessControlBadConfirmation();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlBadConfirmation;
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
        impl ::core::convert::From<AccessControlBadConfirmation>
        for UnderlyingRustTuple<'_> {
            fn from(value: AccessControlBadConfirmation) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AccessControlBadConfirmation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccessControlBadConfirmation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccessControlBadConfirmation()";
            const SELECTOR: [u8; 4] = [102u8, 151u8, 178u8, 50u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AccessControlUnauthorizedAccount(address,bytes32)` and selector `0xe2517d3f`.
```solidity
error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlUnauthorizedAccount {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub neededRole: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<AccessControlUnauthorizedAccount>
        for UnderlyingRustTuple<'_> {
            fn from(value: AccessControlUnauthorizedAccount) -> Self {
                (value.account, value.neededRole)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AccessControlUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    account: tuple.0,
                    neededRole: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccessControlUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccessControlUnauthorizedAccount(address,bytes32)";
            const SELECTOR: [u8; 4] = [226u8, 81u8, 125u8, 63u8];
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
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.neededRole),
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AddressEmptyCode(address)` and selector `0x9996b315`.
```solidity
error AddressEmptyCode(address target);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AddressEmptyCode {
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<AddressEmptyCode> for UnderlyingRustTuple<'_> {
            fn from(value: AddressEmptyCode) -> Self {
                (value.target,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AddressEmptyCode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { target: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AddressEmptyCode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AddressEmptyCode(address)";
            const SELECTOR: [u8; 4] = [153u8, 150u8, 179u8, 21u8];
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
                        &self.target,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `BalanceInvariantViolated(uint256,uint256)` and selector `0xa99da302`.
```solidity
error BalanceInvariantViolated(uint256 internalBalance, uint256 externalBalance);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BalanceInvariantViolated {
        #[allow(missing_docs)]
        pub internalBalance: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub externalBalance: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<BalanceInvariantViolated>
        for UnderlyingRustTuple<'_> {
            fn from(value: BalanceInvariantViolated) -> Self {
                (value.internalBalance, value.externalBalance)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for BalanceInvariantViolated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    internalBalance: tuple.0,
                    externalBalance: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BalanceInvariantViolated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BalanceInvariantViolated(uint256,uint256)";
            const SELECTOR: [u8; 4] = [169u8, 157u8, 163u8, 2u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.internalBalance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.externalBalance),
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC1967InvalidImplementation(address)` and selector `0x4c9c8ce3`.
```solidity
error ERC1967InvalidImplementation(address implementation);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC1967InvalidImplementation {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<ERC1967InvalidImplementation>
        for UnderlyingRustTuple<'_> {
            fn from(value: ERC1967InvalidImplementation) -> Self {
                (value.implementation,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ERC1967InvalidImplementation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { implementation: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC1967InvalidImplementation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC1967InvalidImplementation(address)";
            const SELECTOR: [u8; 4] = [76u8, 156u8, 140u8, 227u8];
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
                        &self.implementation,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC1967NonPayable()` and selector `0xb398979f`.
```solidity
error ERC1967NonPayable();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC1967NonPayable;
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
        impl ::core::convert::From<ERC1967NonPayable> for UnderlyingRustTuple<'_> {
            fn from(value: ERC1967NonPayable) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC1967NonPayable {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC1967NonPayable {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC1967NonPayable()";
            const SELECTOR: [u8; 4] = [179u8, 152u8, 151u8, 159u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `FailedCall()` and selector `0xd6bda275`.
```solidity
error FailedCall();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FailedCall;
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
        impl ::core::convert::From<FailedCall> for UnderlyingRustTuple<'_> {
            fn from(value: FailedCall) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FailedCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FailedCall {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FailedCall()";
            const SELECTOR: [u8; 4] = [214u8, 189u8, 162u8, 117u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `FailedToSendNative()` and selector `0x950b2479`.
```solidity
error FailedToSendNative();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FailedToSendNative;
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
        impl ::core::convert::From<FailedToSendNative> for UnderlyingRustTuple<'_> {
            fn from(value: FailedToSendNative) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FailedToSendNative {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FailedToSendNative {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FailedToSendNative()";
            const SELECTOR: [u8; 4] = [149u8, 11u8, 36u8, 121u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `IndexOutOfRange()` and selector `0x1390f2a1`.
```solidity
error IndexOutOfRange();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IndexOutOfRange;
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
        impl ::core::convert::From<IndexOutOfRange> for UnderlyingRustTuple<'_> {
            fn from(value: IndexOutOfRange) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for IndexOutOfRange {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IndexOutOfRange {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IndexOutOfRange()";
            const SELECTOR: [u8; 4] = [19u8, 144u8, 242u8, 161u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InsufficientBalance()` and selector `0xf4d678b8`.
```solidity
error InsufficientBalance();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientBalance;
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
        impl ::core::convert::From<InsufficientBalance> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientBalance) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientBalance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientBalance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientBalance()";
            const SELECTOR: [u8; 4] = [244u8, 214u8, 120u8, 184u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidCalldata()` and selector `0x8129bbcd`.
```solidity
error InvalidCalldata();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidCalldata;
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
        impl ::core::convert::From<InvalidCalldata> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCalldata) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidCalldata {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCalldata {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCalldata()";
            const SELECTOR: [u8; 4] = [129u8, 41u8, 187u8, 205u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidConsumer(uint256,address)` and selector `0x79bfd401`.
```solidity
error InvalidConsumer(uint256 subId, address consumer);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidConsumer {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub consumer: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<InvalidConsumer> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidConsumer) -> Self {
                (value.subId, value.consumer)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidConsumer {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    subId: tuple.0,
                    consumer: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidConsumer {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidConsumer(uint256,address)";
            const SELECTOR: [u8; 4] = [121u8, 191u8, 212u8, 1u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.subId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.consumer,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidInitialization()` and selector `0xf92ee8a9`.
```solidity
error InvalidInitialization();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidInitialization;
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
        impl ::core::convert::From<InvalidInitialization> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidInitialization) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidInitialization {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidInitialization {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidInitialization()";
            const SELECTOR: [u8; 4] = [249u8, 46u8, 232u8, 169u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidSubscription()` and selector `0x1f6a65b6`.
```solidity
error InvalidSubscription();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSubscription;
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
        impl ::core::convert::From<InvalidSubscription> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSubscription) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSubscription {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSubscription {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSubscription()";
            const SELECTOR: [u8; 4] = [31u8, 106u8, 101u8, 182u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `MustBeRequestedOwner(address)` and selector `0xd084e975`.
```solidity
error MustBeRequestedOwner(address proposedOwner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MustBeRequestedOwner {
        #[allow(missing_docs)]
        pub proposedOwner: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<MustBeRequestedOwner> for UnderlyingRustTuple<'_> {
            fn from(value: MustBeRequestedOwner) -> Self {
                (value.proposedOwner,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MustBeRequestedOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { proposedOwner: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MustBeRequestedOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MustBeRequestedOwner(address)";
            const SELECTOR: [u8; 4] = [208u8, 132u8, 233u8, 117u8];
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
                        &self.proposedOwner,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `MustBeSubOwner(address)` and selector `0xd8a3fb52`.
```solidity
error MustBeSubOwner(address owner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MustBeSubOwner {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<MustBeSubOwner> for UnderlyingRustTuple<'_> {
            fn from(value: MustBeSubOwner) -> Self {
                (value.owner,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MustBeSubOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { owner: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MustBeSubOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MustBeSubOwner(address)";
            const SELECTOR: [u8; 4] = [216u8, 163u8, 251u8, 82u8];
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
                        &self.owner,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotInitializing()` and selector `0xd7e6bcf8`.
```solidity
error NotInitializing();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotInitializing;
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
        impl ::core::convert::From<NotInitializing> for UnderlyingRustTuple<'_> {
            fn from(value: NotInitializing) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotInitializing {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotInitializing {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotInitializing()";
            const SELECTOR: [u8; 4] = [215u8, 230u8, 188u8, 248u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `PendingRequestExists()` and selector `0xb42f66e8`.
```solidity
error PendingRequestExists();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PendingRequestExists;
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
        impl ::core::convert::From<PendingRequestExists> for UnderlyingRustTuple<'_> {
            fn from(value: PendingRequestExists) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PendingRequestExists {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PendingRequestExists {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PendingRequestExists()";
            const SELECTOR: [u8; 4] = [180u8, 47u8, 102u8, 232u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ReentrancyGuardReentrantCall()` and selector `0x3ee5aeb5`.
```solidity
error ReentrancyGuardReentrantCall();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ReentrancyGuardReentrantCall;
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
        impl ::core::convert::From<ReentrancyGuardReentrantCall>
        for UnderlyingRustTuple<'_> {
            fn from(value: ReentrancyGuardReentrantCall) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ReentrancyGuardReentrantCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ReentrancyGuardReentrantCall {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ReentrancyGuardReentrantCall()";
            const SELECTOR: [u8; 4] = [62u8, 229u8, 174u8, 181u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `TooManyConsumers()` and selector `0x05a48e0f`.
```solidity
error TooManyConsumers();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TooManyConsumers;
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
        impl ::core::convert::From<TooManyConsumers> for UnderlyingRustTuple<'_> {
            fn from(value: TooManyConsumers) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TooManyConsumers {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TooManyConsumers {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TooManyConsumers()";
            const SELECTOR: [u8; 4] = [5u8, 164u8, 142u8, 15u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UUPSUnauthorizedCallContext()` and selector `0xe07c8dba`.
```solidity
error UUPSUnauthorizedCallContext();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UUPSUnauthorizedCallContext;
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
        impl ::core::convert::From<UUPSUnauthorizedCallContext>
        for UnderlyingRustTuple<'_> {
            fn from(value: UUPSUnauthorizedCallContext) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UUPSUnauthorizedCallContext {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UUPSUnauthorizedCallContext {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UUPSUnauthorizedCallContext()";
            const SELECTOR: [u8; 4] = [224u8, 124u8, 141u8, 186u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UUPSUnsupportedProxiableUUID(bytes32)` and selector `0xaa1d49a4`.
```solidity
error UUPSUnsupportedProxiableUUID(bytes32 slot);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UUPSUnsupportedProxiableUUID {
        #[allow(missing_docs)]
        pub slot: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<UUPSUnsupportedProxiableUUID>
        for UnderlyingRustTuple<'_> {
            fn from(value: UUPSUnsupportedProxiableUUID) -> Self {
                (value.slot,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UUPSUnsupportedProxiableUUID {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { slot: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UUPSUnsupportedProxiableUUID {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UUPSUnsupportedProxiableUUID(bytes32)";
            const SELECTOR: [u8; 4] = [170u8, 29u8, 73u8, 164u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ConfigSet(uint32,uint32,uint32,uint32,uint32,uint8,uint32)` and selector `0x55a28fde295f482c9f32d670c116103bca15724bcef4f18b35542e0553c35ad5`.
```solidity
event ConfigSet(uint32 maxGasLimit, uint32 gasAfterPaymentCalculation, uint32 fulfillmentFlatFeeNativePPM, uint32 weiPerUnitGas, uint32 blsPairingCheckOverhead, uint8 nativePremiumPercentage, uint32 gasForCallExactCheck);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ConfigSet {
        #[allow(missing_docs)]
        pub maxGasLimit: u32,
        #[allow(missing_docs)]
        pub gasAfterPaymentCalculation: u32,
        #[allow(missing_docs)]
        pub fulfillmentFlatFeeNativePPM: u32,
        #[allow(missing_docs)]
        pub weiPerUnitGas: u32,
        #[allow(missing_docs)]
        pub blsPairingCheckOverhead: u32,
        #[allow(missing_docs)]
        pub nativePremiumPercentage: u8,
        #[allow(missing_docs)]
        pub gasForCallExactCheck: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ConfigSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ConfigSet(uint32,uint32,uint32,uint32,uint32,uint8,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                85u8, 162u8, 143u8, 222u8, 41u8, 95u8, 72u8, 44u8, 159u8, 50u8, 214u8,
                112u8, 193u8, 22u8, 16u8, 59u8, 202u8, 21u8, 114u8, 75u8, 206u8, 244u8,
                241u8, 139u8, 53u8, 84u8, 46u8, 5u8, 83u8, 195u8, 90u8, 213u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    maxGasLimit: data.0,
                    gasAfterPaymentCalculation: data.1,
                    fulfillmentFlatFeeNativePPM: data.2,
                    weiPerUnitGas: data.3,
                    blsPairingCheckOverhead: data.4,
                    nativePremiumPercentage: data.5,
                    gasForCallExactCheck: data.6,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxGasLimit),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.gasAfterPaymentCalculation,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.fulfillmentFlatFeeNativePPM,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.weiPerUnitGas),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.blsPairingCheckOverhead,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nativePremiumPercentage,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasForCallExactCheck),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ConfigSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ConfigSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ConfigSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Disabled()` and selector `0x75884cdadc4a89e8b545db800057f06ec7f5338a08183c7ba515f2bfdd9fe1e1`.
```solidity
event Disabled();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Disabled;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Disabled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Disabled()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                117u8, 136u8, 76u8, 218u8, 220u8, 74u8, 137u8, 232u8, 181u8, 69u8, 219u8,
                128u8, 0u8, 87u8, 240u8, 110u8, 199u8, 245u8, 51u8, 138u8, 8u8, 24u8,
                60u8, 123u8, 165u8, 21u8, 242u8, 191u8, 221u8, 159u8, 225u8, 225u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Disabled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Disabled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Disabled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Enabled()` and selector `0xc0f961051f97b04c496472d11cb6170d844e4b2c9dfd3b602a4fa0139712d484`.
```solidity
event Enabled();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Enabled;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Enabled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Enabled()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                192u8, 249u8, 97u8, 5u8, 31u8, 151u8, 176u8, 76u8, 73u8, 100u8, 114u8,
                209u8, 28u8, 182u8, 23u8, 13u8, 132u8, 78u8, 75u8, 44u8, 157u8, 253u8,
                59u8, 96u8, 42u8, 79u8, 160u8, 19u8, 151u8, 18u8, 212u8, 132u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Enabled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Enabled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Enabled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Initialized(uint64)` and selector `0xc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2`.
```solidity
event Initialized(uint64 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8,
                19u8, 244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8,
                33u8, 238u8, 209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `L1GasFee(uint256)` and selector `0x56296f7beae05a0db815737fdb4cd298897b1e517614d62468081531ae14d099`.
```solidity
event L1GasFee(uint256 fee);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct L1GasFee {
        #[allow(missing_docs)]
        pub fee: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for L1GasFee {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "L1GasFee(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                86u8, 41u8, 111u8, 123u8, 234u8, 224u8, 90u8, 13u8, 184u8, 21u8, 115u8,
                127u8, 219u8, 76u8, 210u8, 152u8, 137u8, 123u8, 30u8, 81u8, 118u8, 20u8,
                214u8, 36u8, 104u8, 8u8, 21u8, 49u8, 174u8, 20u8, 208u8, 153u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { fee: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for L1GasFee {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&L1GasFee> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &L1GasFee) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RandomnessCallbackFailed(uint256)` and selector `0x8f67472dde2126ccd0315b75dc482a5a73acb228a395553f8ae6edde5a0ca4fa`.
```solidity
event RandomnessCallbackFailed(uint256 indexed requestID);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RandomnessCallbackFailed {
        #[allow(missing_docs)]
        pub requestID: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RandomnessCallbackFailed {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "RandomnessCallbackFailed(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                143u8, 103u8, 71u8, 45u8, 222u8, 33u8, 38u8, 204u8, 208u8, 49u8, 91u8,
                117u8, 220u8, 72u8, 42u8, 90u8, 115u8, 172u8, 178u8, 40u8, 163u8, 149u8,
                85u8, 63u8, 138u8, 230u8, 237u8, 222u8, 90u8, 12u8, 164u8, 250u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { requestID: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.requestID.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestID);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RandomnessCallbackFailed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RandomnessCallbackFailed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RandomnessCallbackFailed,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RandomnessCallbackSuccess(uint256,bytes32,bytes)` and selector `0xb74b3204a538cd8021662d42e794681ddc339924ef675b8fd11e9eaf6aa19eb5`.
```solidity
event RandomnessCallbackSuccess(uint256 indexed requestID, bytes32 randomness, bytes signature);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RandomnessCallbackSuccess {
        #[allow(missing_docs)]
        pub requestID: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub randomness: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RandomnessCallbackSuccess {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "RandomnessCallbackSuccess(uint256,bytes32,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                183u8, 75u8, 50u8, 4u8, 165u8, 56u8, 205u8, 128u8, 33u8, 102u8, 45u8,
                66u8, 231u8, 148u8, 104u8, 29u8, 220u8, 51u8, 153u8, 36u8, 239u8, 103u8,
                91u8, 143u8, 209u8, 30u8, 158u8, 175u8, 106u8, 161u8, 158u8, 181u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestID: topics.1,
                    randomness: data.0,
                    signature: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.randomness),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.requestID.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestID);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RandomnessCallbackSuccess {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RandomnessCallbackSuccess> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RandomnessCallbackSuccess,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RandomnessRequested(uint256,uint256,address,uint256)` and selector `0xeee7195b6cee0fa7044c3af0b86fe2febb1d2703d71191f44052ba0d60ffda64`.
```solidity
event RandomnessRequested(uint256 indexed requestID, uint256 indexed nonce, address indexed requester, uint256 requestedAt);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RandomnessRequested {
        #[allow(missing_docs)]
        pub requestID: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub requester: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub requestedAt: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RandomnessRequested {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RandomnessRequested(uint256,uint256,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                238u8, 231u8, 25u8, 91u8, 108u8, 238u8, 15u8, 167u8, 4u8, 76u8, 58u8,
                240u8, 184u8, 111u8, 226u8, 254u8, 187u8, 29u8, 39u8, 3u8, 215u8, 17u8,
                145u8, 244u8, 64u8, 82u8, 186u8, 13u8, 96u8, 255u8, 218u8, 100u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestID: topics.1,
                    nonce: topics.2,
                    requester: topics.3,
                    requestedAt: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestedAt),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.requestID.clone(),
                    self.nonce.clone(),
                    self.requester.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestID);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.nonce);
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.requester,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RandomnessRequested {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RandomnessRequested> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RandomnessRequested) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleAdminChanged(bytes32,bytes32,bytes32)` and selector `0xbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff`.
```solidity
event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleAdminChanged {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub previousAdminRole: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub newAdminRole: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleAdminChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "RoleAdminChanged(bytes32,bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8,
                81u8, 66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8,
                71u8, 92u8, 58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    previousAdminRole: topics.2,
                    newAdminRole: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.previousAdminRole.clone(),
                    self.newAdminRole.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.previousAdminRole);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.newAdminRole);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleAdminChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleAdminChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleAdminChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleGranted(bytes32,address,address)` and selector `0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d`.
```solidity
event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleGranted {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleGranted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleGranted(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleGranted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleGranted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleGranted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleRevoked(bytes32,address,address)` and selector `0xf6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b`.
```solidity
event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleRevoked {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleRevoked {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleRevoked(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleRevoked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleRevoked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleRevoked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `SignatureSenderUpdated(address)` and selector `0x229f6c3b095d683755a99ab458956747a8b7066c3dd42927d850631c34c238f1`.
```solidity
event SignatureSenderUpdated(address indexed signatureSender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SignatureSenderUpdated {
        #[allow(missing_docs)]
        pub signatureSender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SignatureSenderUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SignatureSenderUpdated(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                34u8, 159u8, 108u8, 59u8, 9u8, 93u8, 104u8, 55u8, 85u8, 169u8, 154u8,
                180u8, 88u8, 149u8, 103u8, 71u8, 168u8, 183u8, 6u8, 108u8, 61u8, 212u8,
                41u8, 39u8, 216u8, 80u8, 99u8, 28u8, 52u8, 194u8, 56u8, 241u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { signatureSender: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.signatureSender.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.signatureSender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SignatureSenderUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SignatureSenderUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SignatureSenderUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `SubscriptionCanceled(uint256,address,uint256)` and selector `0x3784f77e8e883de95b5d47cd713ced01229fa74d118c0a462224bcb0516d43f1`.
```solidity
event SubscriptionCanceled(uint256 indexed subId, address to, uint256 amountNative);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SubscriptionCanceled {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amountNative: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SubscriptionCanceled {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "SubscriptionCanceled(uint256,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                55u8, 132u8, 247u8, 126u8, 142u8, 136u8, 61u8, 233u8, 91u8, 93u8, 71u8,
                205u8, 113u8, 60u8, 237u8, 1u8, 34u8, 159u8, 167u8, 77u8, 17u8, 140u8,
                10u8, 70u8, 34u8, 36u8, 188u8, 176u8, 81u8, 109u8, 67u8, 241u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    subId: topics.1,
                    to: data.0,
                    amountNative: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountNative),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.subId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.subId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SubscriptionCanceled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SubscriptionCanceled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SubscriptionCanceled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `SubscriptionConsumerAdded(uint256,address)` and selector `0x1e980d04aa7648e205713e5e8ea3808672ac163d10936d36f91b2c88ac1575e1`.
```solidity
event SubscriptionConsumerAdded(uint256 indexed subId, address consumer);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SubscriptionConsumerAdded {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub consumer: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SubscriptionConsumerAdded {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "SubscriptionConsumerAdded(uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                30u8, 152u8, 13u8, 4u8, 170u8, 118u8, 72u8, 226u8, 5u8, 113u8, 62u8,
                94u8, 142u8, 163u8, 128u8, 134u8, 114u8, 172u8, 22u8, 61u8, 16u8, 147u8,
                109u8, 54u8, 249u8, 27u8, 44u8, 136u8, 172u8, 21u8, 117u8, 225u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    subId: topics.1,
                    consumer: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.consumer,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.subId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.subId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SubscriptionConsumerAdded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SubscriptionConsumerAdded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SubscriptionConsumerAdded,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `SubscriptionConsumerRemoved(uint256,address)` and selector `0x32158c6058347c1601b2d12bc696ac6901d8a9a9aa3ba10c27ab0a983e8425a7`.
```solidity
event SubscriptionConsumerRemoved(uint256 indexed subId, address consumer);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SubscriptionConsumerRemoved {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub consumer: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SubscriptionConsumerRemoved {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "SubscriptionConsumerRemoved(uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                50u8, 21u8, 140u8, 96u8, 88u8, 52u8, 124u8, 22u8, 1u8, 178u8, 209u8,
                43u8, 198u8, 150u8, 172u8, 105u8, 1u8, 216u8, 169u8, 169u8, 170u8, 59u8,
                161u8, 12u8, 39u8, 171u8, 10u8, 152u8, 62u8, 132u8, 37u8, 167u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    subId: topics.1,
                    consumer: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.consumer,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.subId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.subId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SubscriptionConsumerRemoved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SubscriptionConsumerRemoved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SubscriptionConsumerRemoved,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `SubscriptionCreated(uint256,address)` and selector `0x1d3015d7ba850fa198dc7b1a3f5d42779313a681035f77c8c03764c61005518d`.
```solidity
event SubscriptionCreated(uint256 indexed subId, address owner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SubscriptionCreated {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SubscriptionCreated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "SubscriptionCreated(uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                29u8, 48u8, 21u8, 215u8, 186u8, 133u8, 15u8, 161u8, 152u8, 220u8, 123u8,
                26u8, 63u8, 93u8, 66u8, 119u8, 147u8, 19u8, 166u8, 129u8, 3u8, 95u8,
                119u8, 200u8, 192u8, 55u8, 100u8, 198u8, 16u8, 5u8, 81u8, 141u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    subId: topics.1,
                    owner: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.subId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.subId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SubscriptionCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SubscriptionCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SubscriptionCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `SubscriptionFundedWithNative(uint256,uint256,uint256)` and selector `0x7603b205d03651ee812f803fccde89f1012e545a9c99f0abfea9cedd0fd8e902`.
```solidity
event SubscriptionFundedWithNative(uint256 indexed subId, uint256 oldNativeBalance, uint256 newNativeBalance);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SubscriptionFundedWithNative {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub oldNativeBalance: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newNativeBalance: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SubscriptionFundedWithNative {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "SubscriptionFundedWithNative(uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                118u8, 3u8, 178u8, 5u8, 208u8, 54u8, 81u8, 238u8, 129u8, 47u8, 128u8,
                63u8, 204u8, 222u8, 137u8, 241u8, 1u8, 46u8, 84u8, 90u8, 156u8, 153u8,
                240u8, 171u8, 254u8, 169u8, 206u8, 221u8, 15u8, 216u8, 233u8, 2u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    subId: topics.1,
                    oldNativeBalance: data.0,
                    newNativeBalance: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldNativeBalance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newNativeBalance),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.subId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.subId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SubscriptionFundedWithNative {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SubscriptionFundedWithNative> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SubscriptionFundedWithNative,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `SubscriptionOwnerTransferRequested(uint256,address,address)` and selector `0x21a4dad170a6bf476c31bbcf4a16628295b0e450672eec25d7c93308e05344a1`.
```solidity
event SubscriptionOwnerTransferRequested(uint256 indexed subId, address from, address to);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SubscriptionOwnerTransferRequested {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SubscriptionOwnerTransferRequested {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "SubscriptionOwnerTransferRequested(uint256,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                33u8, 164u8, 218u8, 209u8, 112u8, 166u8, 191u8, 71u8, 108u8, 49u8, 187u8,
                207u8, 74u8, 22u8, 98u8, 130u8, 149u8, 176u8, 228u8, 80u8, 103u8, 46u8,
                236u8, 37u8, 215u8, 201u8, 51u8, 8u8, 224u8, 83u8, 68u8, 161u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    subId: topics.1,
                    from: data.0,
                    to: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.subId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.subId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData
        for SubscriptionOwnerTransferRequested {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SubscriptionOwnerTransferRequested>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SubscriptionOwnerTransferRequested,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `SubscriptionOwnerTransferred(uint256,address,address)` and selector `0xd4114ab6e9af9f597c52041f32d62dc57c5c4e4c0d4427006069635e216c9386`.
```solidity
event SubscriptionOwnerTransferred(uint256 indexed subId, address from, address to);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SubscriptionOwnerTransferred {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SubscriptionOwnerTransferred {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "SubscriptionOwnerTransferred(uint256,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                212u8, 17u8, 74u8, 182u8, 233u8, 175u8, 159u8, 89u8, 124u8, 82u8, 4u8,
                31u8, 50u8, 214u8, 45u8, 197u8, 124u8, 92u8, 78u8, 76u8, 13u8, 68u8,
                39u8, 0u8, 96u8, 105u8, 99u8, 94u8, 33u8, 108u8, 147u8, 134u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    subId: topics.1,
                    from: data.0,
                    to: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.subId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.subId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SubscriptionOwnerTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SubscriptionOwnerTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SubscriptionOwnerTransferred,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Upgraded(address)` and selector `0xbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b`.
```solidity
event Upgraded(address indexed implementation);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Upgraded {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Upgraded {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Upgraded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                188u8, 124u8, 215u8, 90u8, 32u8, 238u8, 39u8, 253u8, 154u8, 222u8, 186u8,
                179u8, 32u8, 65u8, 247u8, 85u8, 33u8, 77u8, 188u8, 107u8, 255u8, 169u8,
                12u8, 192u8, 34u8, 91u8, 57u8, 218u8, 46u8, 92u8, 45u8, 59u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { implementation: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.implementation.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.implementation,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Upgraded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Upgraded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Upgraded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {}
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = ();
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
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ADMIN_ROLE()` and selector `0x75b238fc`.
```solidity
function ADMIN_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ADMIN_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ADMIN_ROLE()`](ADMIN_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ADMIN_ROLEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<ADMIN_ROLECall> for UnderlyingRustTuple<'_> {
                fn from(value: ADMIN_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ADMIN_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
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
            impl ::core::convert::From<ADMIN_ROLEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ADMIN_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ADMIN_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ADMIN_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ADMIN_ROLE()";
            const SELECTOR: [u8; 4] = [117u8, 178u8, 56u8, 252u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: ADMIN_ROLEReturn = r.into();
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
                        let r: ADMIN_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`.
```solidity
function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`DEFAULT_ADMIN_ROLE()`](DEFAULT_ADMIN_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<DEFAULT_ADMIN_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_ADMIN_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
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
            impl ::core::convert::From<DEFAULT_ADMIN_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_ADMIN_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEFAULT_ADMIN_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEFAULT_ADMIN_ROLE()";
            const SELECTOR: [u8; 4] = [162u8, 23u8, 253u8, 223u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: DEFAULT_ADMIN_ROLEReturn = r.into();
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
                        let r: DEFAULT_ADMIN_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `MAX_CONSUMERS()` and selector `0x64d51a2a`.
```solidity
function MAX_CONSUMERS() external view returns (uint16);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_CONSUMERSCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`MAX_CONSUMERS()`](MAX_CONSUMERSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_CONSUMERSReturn {
        #[allow(missing_docs)]
        pub _0: u16,
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
            impl ::core::convert::From<MAX_CONSUMERSCall> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_CONSUMERSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_CONSUMERSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u16,);
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
            impl ::core::convert::From<MAX_CONSUMERSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_CONSUMERSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_CONSUMERSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_CONSUMERSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u16;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_CONSUMERS()";
            const SELECTOR: [u8; 4] = [100u8, 213u8, 26u8, 42u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: MAX_CONSUMERSReturn = r.into();
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
                        let r: MAX_CONSUMERSReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `SCHEME_ID()` and selector `0x8a1f165a`.
```solidity
function SCHEME_ID() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SCHEME_IDCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`SCHEME_ID()`](SCHEME_IDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SCHEME_IDReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
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
            impl ::core::convert::From<SCHEME_IDCall> for UnderlyingRustTuple<'_> {
                fn from(value: SCHEME_IDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SCHEME_IDCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
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
            impl ::core::convert::From<SCHEME_IDReturn> for UnderlyingRustTuple<'_> {
                fn from(value: SCHEME_IDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SCHEME_IDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SCHEME_IDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SCHEME_ID()";
            const SELECTOR: [u8; 4] = [138u8, 31u8, 22u8, 90u8];
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
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
                        let r: SCHEME_IDReturn = r.into();
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
                        let r: SCHEME_IDReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`.
```solidity
function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPGRADE_INTERFACE_VERSIONCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`UPGRADE_INTERFACE_VERSION()`](UPGRADE_INTERFACE_VERSIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPGRADE_INTERFACE_VERSIONReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
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
            impl ::core::convert::From<UPGRADE_INTERFACE_VERSIONCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: UPGRADE_INTERFACE_VERSIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UPGRADE_INTERFACE_VERSIONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
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
            impl ::core::convert::From<UPGRADE_INTERFACE_VERSIONReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: UPGRADE_INTERFACE_VERSIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UPGRADE_INTERFACE_VERSIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for UPGRADE_INTERFACE_VERSIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UPGRADE_INTERFACE_VERSION()";
            const SELECTOR: [u8; 4] = [173u8, 60u8, 177u8, 204u8];
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
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
                        let r: UPGRADE_INTERFACE_VERSIONReturn = r.into();
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
                        let r: UPGRADE_INTERFACE_VERSIONReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `acceptSubscriptionOwnerTransfer(uint256)` and selector `0xb2a7cac5`.
```solidity
function acceptSubscriptionOwnerTransfer(uint256 subId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptSubscriptionOwnerTransferCall {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`acceptSubscriptionOwnerTransfer(uint256)`](acceptSubscriptionOwnerTransferCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptSubscriptionOwnerTransferReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<acceptSubscriptionOwnerTransferCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptSubscriptionOwnerTransferCall) -> Self {
                    (value.subId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptSubscriptionOwnerTransferCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { subId: tuple.0 }
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
            impl ::core::convert::From<acceptSubscriptionOwnerTransferReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptSubscriptionOwnerTransferReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptSubscriptionOwnerTransferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl acceptSubscriptionOwnerTransferReturn {
            fn _tokenize(
                &self,
            ) -> <acceptSubscriptionOwnerTransferCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptSubscriptionOwnerTransferCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = acceptSubscriptionOwnerTransferReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "acceptSubscriptionOwnerTransfer(uint256)";
            const SELECTOR: [u8; 4] = [178u8, 167u8, 202u8, 197u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.subId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                acceptSubscriptionOwnerTransferReturn::_tokenize(ret)
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
    /**Function with signature `addConsumer(uint256,address)` and selector `0xbec4c08c`.
```solidity
function addConsumer(uint256 subId, address consumer) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addConsumerCall {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub consumer: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`addConsumer(uint256,address)`](addConsumerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addConsumerReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<addConsumerCall> for UnderlyingRustTuple<'_> {
                fn from(value: addConsumerCall) -> Self {
                    (value.subId, value.consumer)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addConsumerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        subId: tuple.0,
                        consumer: tuple.1,
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
            impl ::core::convert::From<addConsumerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addConsumerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addConsumerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addConsumerReturn {
            fn _tokenize(
                &self,
            ) -> <addConsumerCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addConsumerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addConsumerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addConsumer(uint256,address)";
            const SELECTOR: [u8; 4] = [190u8, 196u8, 192u8, 140u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.subId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.consumer,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addConsumerReturn::_tokenize(ret)
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
    /**Function with signature `calculateRequestPriceNative(uint32)` and selector `0x4b160935`.
```solidity
function calculateRequestPriceNative(uint32 _callbackGasLimit) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateRequestPriceNativeCall {
        #[allow(missing_docs)]
        pub _callbackGasLimit: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`calculateRequestPriceNative(uint32)`](calculateRequestPriceNativeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateRequestPriceNativeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<calculateRequestPriceNativeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateRequestPriceNativeCall) -> Self {
                    (value._callbackGasLimit,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateRequestPriceNativeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _callbackGasLimit: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<calculateRequestPriceNativeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateRequestPriceNativeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateRequestPriceNativeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateRequestPriceNativeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateRequestPriceNative(uint32)";
            const SELECTOR: [u8; 4] = [75u8, 22u8, 9u8, 53u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._callbackGasLimit),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: calculateRequestPriceNativeReturn = r.into();
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
                        let r: calculateRequestPriceNativeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `cancelSubscription(uint256,address)` and selector `0x0ae09540`.
```solidity
function cancelSubscription(uint256 subId, address to) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelSubscriptionCall {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`cancelSubscription(uint256,address)`](cancelSubscriptionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelSubscriptionReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<cancelSubscriptionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelSubscriptionCall) -> Self {
                    (value.subId, value.to)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelSubscriptionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        subId: tuple.0,
                        to: tuple.1,
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
            impl ::core::convert::From<cancelSubscriptionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelSubscriptionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelSubscriptionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl cancelSubscriptionReturn {
            fn _tokenize(
                &self,
            ) -> <cancelSubscriptionCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelSubscriptionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelSubscriptionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancelSubscription(uint256,address)";
            const SELECTOR: [u8; 4] = [10u8, 224u8, 149u8, 64u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.subId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                cancelSubscriptionReturn::_tokenize(ret)
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
    /**Function with signature `createSubscription()` and selector `0xa21a23e4`.
```solidity
function createSubscription() external returns (uint256 subId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createSubscriptionCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`createSubscription()`](createSubscriptionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createSubscriptionReturn {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<createSubscriptionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createSubscriptionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createSubscriptionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<createSubscriptionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createSubscriptionReturn) -> Self {
                    (value.subId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createSubscriptionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { subId: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createSubscriptionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createSubscription()";
            const SELECTOR: [u8; 4] = [162u8, 26u8, 35u8, 228u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: createSubscriptionReturn = r.into();
                        r.subId
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
                        let r: createSubscriptionReturn = r.into();
                        r.subId
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `disable()` and selector `0x2f2770db`.
```solidity
function disable() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableCall;
    ///Container type for the return parameters of the [`disable()`](disableCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableReturn {}
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
            impl ::core::convert::From<disableCall> for UnderlyingRustTuple<'_> {
                fn from(value: disableCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disableCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<disableReturn> for UnderlyingRustTuple<'_> {
                fn from(value: disableReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disableReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl disableReturn {
            fn _tokenize(
                &self,
            ) -> <disableCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disableCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = disableReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disable()";
            const SELECTOR: [u8; 4] = [47u8, 39u8, 112u8, 219u8];
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
                disableReturn::_tokenize(ret)
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
    /**Function with signature `enable()` and selector `0xa3907d71`.
```solidity
function enable() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableCall;
    ///Container type for the return parameters of the [`enable()`](enableCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableReturn {}
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
            impl ::core::convert::From<enableCall> for UnderlyingRustTuple<'_> {
                fn from(value: enableCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for enableCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<enableReturn> for UnderlyingRustTuple<'_> {
                fn from(value: enableReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for enableReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl enableReturn {
            fn _tokenize(
                &self,
            ) -> <enableCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for enableCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = enableReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "enable()";
            const SELECTOR: [u8; 4] = [163u8, 144u8, 125u8, 113u8];
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
                enableReturn::_tokenize(ret)
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
    /**Function with signature `estimateRequestPriceNative(uint32,uint256)` and selector `0x3255c456`.
```solidity
function estimateRequestPriceNative(uint32 _callbackGasLimit, uint256 _requestGasPriceWei) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct estimateRequestPriceNativeCall {
        #[allow(missing_docs)]
        pub _callbackGasLimit: u32,
        #[allow(missing_docs)]
        pub _requestGasPriceWei: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`estimateRequestPriceNative(uint32,uint256)`](estimateRequestPriceNativeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct estimateRequestPriceNativeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
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
            impl ::core::convert::From<estimateRequestPriceNativeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: estimateRequestPriceNativeCall) -> Self {
                    (value._callbackGasLimit, value._requestGasPriceWei)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for estimateRequestPriceNativeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _callbackGasLimit: tuple.0,
                        _requestGasPriceWei: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<estimateRequestPriceNativeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: estimateRequestPriceNativeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for estimateRequestPriceNativeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for estimateRequestPriceNativeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "estimateRequestPriceNative(uint32,uint256)";
            const SELECTOR: [u8; 4] = [50u8, 85u8, 196u8, 86u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._callbackGasLimit),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._requestGasPriceWei),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: estimateRequestPriceNativeReturn = r.into();
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
                        let r: estimateRequestPriceNativeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `fundSubscriptionWithNative(uint256)` and selector `0x95b55cfc`.
```solidity
function fundSubscriptionWithNative(uint256 subId) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fundSubscriptionWithNativeCall {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`fundSubscriptionWithNative(uint256)`](fundSubscriptionWithNativeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fundSubscriptionWithNativeReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<fundSubscriptionWithNativeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: fundSubscriptionWithNativeCall) -> Self {
                    (value.subId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fundSubscriptionWithNativeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { subId: tuple.0 }
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
            impl ::core::convert::From<fundSubscriptionWithNativeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: fundSubscriptionWithNativeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fundSubscriptionWithNativeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl fundSubscriptionWithNativeReturn {
            fn _tokenize(
                &self,
            ) -> <fundSubscriptionWithNativeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for fundSubscriptionWithNativeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = fundSubscriptionWithNativeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "fundSubscriptionWithNative(uint256)";
            const SELECTOR: [u8; 4] = [149u8, 181u8, 92u8, 252u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.subId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                fundSubscriptionWithNativeReturn::_tokenize(ret)
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
    /**Function with signature `getActiveSubscriptionIds(uint256,uint256)` and selector `0xaefb212f`.
```solidity
function getActiveSubscriptionIds(uint256 startIndex, uint256 maxCount) external view returns (uint256[] memory ids);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveSubscriptionIdsCall {
        #[allow(missing_docs)]
        pub startIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxCount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getActiveSubscriptionIds(uint256,uint256)`](getActiveSubscriptionIdsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveSubscriptionIdsReturn {
        #[allow(missing_docs)]
        pub ids: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getActiveSubscriptionIdsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getActiveSubscriptionIdsCall) -> Self {
                    (value.startIndex, value.maxCount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getActiveSubscriptionIdsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        startIndex: tuple.0,
                        maxCount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<getActiveSubscriptionIdsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getActiveSubscriptionIdsReturn) -> Self {
                    (value.ids,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getActiveSubscriptionIdsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ids: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActiveSubscriptionIdsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getActiveSubscriptionIds(uint256,uint256)";
            const SELECTOR: [u8; 4] = [174u8, 251u8, 33u8, 47u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxCount),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getActiveSubscriptionIdsReturn = r.into();
                        r.ids
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
                        let r: getActiveSubscriptionIdsReturn = r.into();
                        r.ids
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getAllRequests()` and selector `0xfb1a002a`.
```solidity
function getAllRequests() external view returns (TypesLib.RandomnessRequest[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllRequestsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getAllRequests()`](getAllRequestsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllRequestsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <TypesLib::RandomnessRequest as alloy::sol_types::SolType>::RustType,
        >,
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
            impl ::core::convert::From<getAllRequestsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAllRequestsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllRequestsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<TypesLib::RandomnessRequest>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <TypesLib::RandomnessRequest as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<getAllRequestsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllRequestsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllRequestsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllRequestsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <TypesLib::RandomnessRequest as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<TypesLib::RandomnessRequest>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllRequests()";
            const SELECTOR: [u8; 4] = [251u8, 26u8, 0u8, 42u8];
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
                    <alloy::sol_types::sol_data::Array<
                        TypesLib::RandomnessRequest,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getAllRequestsReturn = r.into();
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
                        let r: getAllRequestsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getConfig()` and selector `0xc3f909d4`.
```solidity
function getConfig() external view returns (uint32 maxGasLimit, uint32 gasAfterPaymentCalculation, uint32 fulfillmentFlatFeeNativePPM, uint32 weiPerUnitGas, uint32 blsPairingCheckOverhead, uint8 nativePremiumPercentage, uint32 gasForCallExactCheck);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getConfigCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getConfig()`](getConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getConfigReturn {
        #[allow(missing_docs)]
        pub maxGasLimit: u32,
        #[allow(missing_docs)]
        pub gasAfterPaymentCalculation: u32,
        #[allow(missing_docs)]
        pub fulfillmentFlatFeeNativePPM: u32,
        #[allow(missing_docs)]
        pub weiPerUnitGas: u32,
        #[allow(missing_docs)]
        pub blsPairingCheckOverhead: u32,
        #[allow(missing_docs)]
        pub nativePremiumPercentage: u8,
        #[allow(missing_docs)]
        pub gasForCallExactCheck: u32,
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
            impl ::core::convert::From<getConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: getConfigCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, u32, u32, u32, u32, u8, u32);
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
            impl ::core::convert::From<getConfigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getConfigReturn) -> Self {
                    (
                        value.maxGasLimit,
                        value.gasAfterPaymentCalculation,
                        value.fulfillmentFlatFeeNativePPM,
                        value.weiPerUnitGas,
                        value.blsPairingCheckOverhead,
                        value.nativePremiumPercentage,
                        value.gasForCallExactCheck,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        maxGasLimit: tuple.0,
                        gasAfterPaymentCalculation: tuple.1,
                        fulfillmentFlatFeeNativePPM: tuple.2,
                        weiPerUnitGas: tuple.3,
                        blsPairingCheckOverhead: tuple.4,
                        nativePremiumPercentage: tuple.5,
                        gasForCallExactCheck: tuple.6,
                    }
                }
            }
        }
        impl getConfigReturn {
            fn _tokenize(
                &self,
            ) -> <getConfigCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxGasLimit),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.gasAfterPaymentCalculation,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.fulfillmentFlatFeeNativePPM,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.weiPerUnitGas),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.blsPairingCheckOverhead,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nativePremiumPercentage,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasForCallExactCheck),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getConfigCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getConfigReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getConfig()";
            const SELECTOR: [u8; 4] = [195u8, 249u8, 9u8, 212u8];
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
                getConfigReturn::_tokenize(ret)
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
    /**Function with signature `getRequest(uint256)` and selector `0xc58343ef`.
```solidity
function getRequest(uint256 requestId) external view returns (TypesLib.RandomnessRequest memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRequestCall {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRequest(uint256)`](getRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRequestReturn {
        #[allow(missing_docs)]
        pub _0: <TypesLib::RandomnessRequest as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getRequestCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRequestCall) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (TypesLib::RandomnessRequest,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <TypesLib::RandomnessRequest as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getRequestReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRequestReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRequestCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <TypesLib::RandomnessRequest as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (TypesLib::RandomnessRequest,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRequest(uint256)";
            const SELECTOR: [u8; 4] = [197u8, 131u8, 67u8, 239u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <TypesLib::RandomnessRequest as alloy_sol_types::SolType>::tokenize(
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
                        let r: getRequestReturn = r.into();
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
                        let r: getRequestReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`.
```solidity
function getRoleAdmin(bytes32 role) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRoleAdmin(bytes32)`](getRoleAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getRoleAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminCall) -> Self {
                    (value.role,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<getRoleAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleAdmin(bytes32)";
            const SELECTOR: [u8; 4] = [36u8, 138u8, 156u8, 163u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getRoleAdminReturn = r.into();
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
                        let r: getRoleAdminReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`.
```solidity
function getRoleMember(bytes32 role, uint256 index) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleMemberCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRoleMember(bytes32,uint256)`](getRoleMemberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleMemberReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getRoleMemberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleMemberCall) -> Self {
                    (value.role, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleMemberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        index: tuple.1,
                    }
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
            impl ::core::convert::From<getRoleMemberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleMemberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleMemberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleMemberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleMember(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [144u8, 16u8, 208u8, 124u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
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
                        let r: getRoleMemberReturn = r.into();
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
                        let r: getRoleMemberReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`.
```solidity
function getRoleMemberCount(bytes32 role) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleMemberCountCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRoleMemberCount(bytes32)`](getRoleMemberCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleMemberCountReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getRoleMemberCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRoleMemberCountCall) -> Self {
                    (value.role,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRoleMemberCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getRoleMemberCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRoleMemberCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRoleMemberCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleMemberCountCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleMemberCount(bytes32)";
            const SELECTOR: [u8; 4] = [202u8, 21u8, 200u8, 115u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getRoleMemberCountReturn = r.into();
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
                        let r: getRoleMemberCountReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRoleMembers(bytes32)` and selector `0xa3246ad3`.
```solidity
function getRoleMembers(bytes32 role) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleMembersCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRoleMembers(bytes32)`](getRoleMembersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleMembersReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getRoleMembersCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleMembersCall) -> Self {
                    (value.role,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleMembersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getRoleMembersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRoleMembersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRoleMembersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleMembersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleMembers(bytes32)";
            const SELECTOR: [u8; 4] = [163u8, 36u8, 106u8, 211u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getRoleMembersReturn = r.into();
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
                        let r: getRoleMembersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getSubscription(uint256)` and selector `0xdc311dd3`.
```solidity
function getSubscription(uint256 subId) external view returns (uint96 nativeBalance, uint64 reqCount, address subOwner, address[] memory consumers);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSubscriptionCall {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getSubscription(uint256)`](getSubscriptionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSubscriptionReturn {
        #[allow(missing_docs)]
        pub nativeBalance: alloy::sol_types::private::primitives::aliases::U96,
        #[allow(missing_docs)]
        pub reqCount: u64,
        #[allow(missing_docs)]
        pub subOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub consumers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getSubscriptionCall> for UnderlyingRustTuple<'_> {
                fn from(value: getSubscriptionCall) -> Self {
                    (value.subId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSubscriptionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { subId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
                u64,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getSubscriptionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSubscriptionReturn) -> Self {
                    (
                        value.nativeBalance,
                        value.reqCount,
                        value.subOwner,
                        value.consumers,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSubscriptionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        nativeBalance: tuple.0,
                        reqCount: tuple.1,
                        subOwner: tuple.2,
                        consumers: tuple.3,
                    }
                }
            }
        }
        impl getSubscriptionReturn {
            fn _tokenize(
                &self,
            ) -> <getSubscriptionCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.nativeBalance),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.reqCount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.subOwner,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.consumers),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSubscriptionCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSubscriptionReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSubscription(uint256)";
            const SELECTOR: [u8; 4] = [220u8, 49u8, 29u8, 211u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.subId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getSubscriptionReturn::_tokenize(ret)
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
    /**Function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`.
```solidity
function grantRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`grantRole(bytes32,address)`](grantRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleReturn {}
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<grantRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
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
            impl ::core::convert::From<grantRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl grantRoleReturn {
            fn _tokenize(
                &self,
            ) -> <grantRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for grantRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = grantRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "grantRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [47u8, 47u8, 241u8, 93u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                grantRoleReturn::_tokenize(ret)
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
    /**Function with signature `hasRole(bytes32,address)` and selector `0x91d14854`.
```solidity
function hasRole(bytes32 role, address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`hasRole(bytes32,address)`](hasRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<hasRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
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
            impl ::core::convert::From<hasRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [145u8, 209u8, 72u8, 84u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
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
                        let r: hasRoleReturn = r.into();
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
                        let r: hasRoleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initialize(address,address)` and selector `0x485cc955`.
```solidity
function initialize(address _signatureSender, address owner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub _signatureSender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._signatureSender, value.owner)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _signatureSender: tuple.0,
                        owner: tuple.1,
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
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initializeReturn {
            fn _tokenize(
                &self,
            ) -> <initializeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,address)";
            const SELECTOR: [u8; 4] = [72u8, 92u8, 201u8, 85u8];
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
                        &self._signatureSender,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initializeReturn::_tokenize(ret)
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
    /**Function with signature `isInFlight(uint256)` and selector `0xcd802c91`.
```solidity
function isInFlight(uint256 requestID) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isInFlightCall {
        #[allow(missing_docs)]
        pub requestID: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isInFlight(uint256)`](isInFlightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isInFlightReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<isInFlightCall> for UnderlyingRustTuple<'_> {
                fn from(value: isInFlightCall) -> Self {
                    (value.requestID,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isInFlightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestID: tuple.0 }
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
            impl ::core::convert::From<isInFlightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isInFlightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isInFlightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isInFlightCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isInFlight(uint256)";
            const SELECTOR: [u8; 4] = [205u8, 128u8, 44u8, 145u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestID),
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
                        let r: isInFlightReturn = r.into();
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
                        let r: isInFlightReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `messageFrom((uint256,address))` and selector `0x775b839c`.
```solidity
function messageFrom(TypesLib.RandomnessRequestCreationParams memory r) external pure returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct messageFromCall {
        #[allow(missing_docs)]
        pub r: <TypesLib::RandomnessRequestCreationParams as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`messageFrom((uint256,address))`](messageFromCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct messageFromReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
            type UnderlyingSolTuple<'a> = (TypesLib::RandomnessRequestCreationParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <TypesLib::RandomnessRequestCreationParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<messageFromCall> for UnderlyingRustTuple<'_> {
                fn from(value: messageFromCall) -> Self {
                    (value.r,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for messageFromCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { r: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
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
            impl ::core::convert::From<messageFromReturn> for UnderlyingRustTuple<'_> {
                fn from(value: messageFromReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for messageFromReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for messageFromCall {
            type Parameters<'a> = (TypesLib::RandomnessRequestCreationParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Bytes;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "messageFrom((uint256,address))";
            const SELECTOR: [u8; 4] = [119u8, 91u8, 131u8, 156u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <TypesLib::RandomnessRequestCreationParams as alloy_sol_types::SolType>::tokenize(
                        &self.r,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
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
                        let r: messageFromReturn = r.into();
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
                        let r: messageFromReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `nonce()` and selector `0xaffed0e0`.
```solidity
function nonce() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nonceCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`nonce()`](nonceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nonceReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<nonceCall> for UnderlyingRustTuple<'_> {
                fn from(value: nonceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nonceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<nonceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nonceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nonceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nonceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nonce()";
            const SELECTOR: [u8; 4] = [175u8, 254u8, 208u8, 224u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: nonceReturn = r.into();
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
                        let r: nonceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ownerCancelSubscription(uint256)` and selector `0xaa433aff`.
```solidity
function ownerCancelSubscription(uint256 subId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCancelSubscriptionCall {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`ownerCancelSubscription(uint256)`](ownerCancelSubscriptionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCancelSubscriptionReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<ownerCancelSubscriptionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ownerCancelSubscriptionCall) -> Self {
                    (value.subId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ownerCancelSubscriptionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { subId: tuple.0 }
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
            impl ::core::convert::From<ownerCancelSubscriptionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ownerCancelSubscriptionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ownerCancelSubscriptionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl ownerCancelSubscriptionReturn {
            fn _tokenize(
                &self,
            ) -> <ownerCancelSubscriptionCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCancelSubscriptionCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerCancelSubscriptionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ownerCancelSubscription(uint256)";
            const SELECTOR: [u8; 4] = [170u8, 67u8, 58u8, 255u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.subId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                ownerCancelSubscriptionReturn::_tokenize(ret)
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
    /**Function with signature `pendingRequestExists(uint256)` and selector `0x41af6c87`.
```solidity
function pendingRequestExists(uint256 subId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingRequestExistsCall {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`pendingRequestExists(uint256)`](pendingRequestExistsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingRequestExistsReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<pendingRequestExistsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: pendingRequestExistsCall) -> Self {
                    (value.subId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pendingRequestExistsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { subId: tuple.0 }
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
            impl ::core::convert::From<pendingRequestExistsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: pendingRequestExistsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pendingRequestExistsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pendingRequestExistsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pendingRequestExists(uint256)";
            const SELECTOR: [u8; 4] = [65u8, 175u8, 108u8, 135u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.subId),
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
                        let r: pendingRequestExistsReturn = r.into();
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
                        let r: pendingRequestExistsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proxiableUUID()` and selector `0x52d1902d`.
```solidity
function proxiableUUID() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proxiableUUID()`](proxiableUUIDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<proxiableUUIDCall> for UnderlyingRustTuple<'_> {
                fn from(value: proxiableUUIDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxiableUUIDCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
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
            impl ::core::convert::From<proxiableUUIDReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proxiableUUIDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxiableUUIDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proxiableUUIDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proxiableUUID()";
            const SELECTOR: [u8; 4] = [82u8, 209u8, 144u8, 45u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: proxiableUUIDReturn = r.into();
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
                        let r: proxiableUUIDReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `receiveSignature(uint256,bytes)` and selector `0xc8db6582`.
```solidity
function receiveSignature(uint256 requestID, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct receiveSignatureCall {
        #[allow(missing_docs)]
        pub requestID: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`receiveSignature(uint256,bytes)`](receiveSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct receiveSignatureReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<receiveSignatureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: receiveSignatureCall) -> Self {
                    (value.requestID, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for receiveSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        requestID: tuple.0,
                        signature: tuple.1,
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
            impl ::core::convert::From<receiveSignatureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: receiveSignatureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for receiveSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl receiveSignatureReturn {
            fn _tokenize(
                &self,
            ) -> <receiveSignatureCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for receiveSignatureCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = receiveSignatureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "receiveSignature(uint256,bytes)";
            const SELECTOR: [u8; 4] = [200u8, 219u8, 101u8, 130u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestID),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                receiveSignatureReturn::_tokenize(ret)
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
    /**Function with signature `removeConsumer(uint256,address)` and selector `0xcb631797`.
```solidity
function removeConsumer(uint256 subId, address consumer) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeConsumerCall {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub consumer: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`removeConsumer(uint256,address)`](removeConsumerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeConsumerReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<removeConsumerCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeConsumerCall) -> Self {
                    (value.subId, value.consumer)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeConsumerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        subId: tuple.0,
                        consumer: tuple.1,
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
            impl ::core::convert::From<removeConsumerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeConsumerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeConsumerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl removeConsumerReturn {
            fn _tokenize(
                &self,
            ) -> <removeConsumerCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeConsumerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeConsumerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeConsumer(uint256,address)";
            const SELECTOR: [u8; 4] = [203u8, 99u8, 23u8, 151u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.subId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.consumer,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                removeConsumerReturn::_tokenize(ret)
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
    /**Function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`.
```solidity
function renounceRole(bytes32 role, address callerConfirmation) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub callerConfirmation: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`renounceRole(bytes32,address)`](renounceRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleReturn {}
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<renounceRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleCall) -> Self {
                    (value.role, value.callerConfirmation)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        callerConfirmation: tuple.1,
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
            impl ::core::convert::From<renounceRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceRoleReturn {
            fn _tokenize(
                &self,
            ) -> <renounceRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [54u8, 86u8, 138u8, 190u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.callerConfirmation,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                renounceRoleReturn::_tokenize(ret)
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
    /**Function with signature `requestRandomness(uint32)` and selector `0x811ee32a`.
```solidity
function requestRandomness(uint32 callbackGasLimit) external payable returns (uint256 requestID);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestRandomnessCall {
        #[allow(missing_docs)]
        pub callbackGasLimit: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`requestRandomness(uint32)`](requestRandomnessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestRandomnessReturn {
        #[allow(missing_docs)]
        pub requestID: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<requestRandomnessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestRandomnessCall) -> Self {
                    (value.callbackGasLimit,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestRandomnessCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { callbackGasLimit: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<requestRandomnessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestRandomnessReturn) -> Self {
                    (value.requestID,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestRandomnessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestID: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestRandomnessCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "requestRandomness(uint32)";
            const SELECTOR: [u8; 4] = [129u8, 30u8, 227u8, 42u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.callbackGasLimit),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: requestRandomnessReturn = r.into();
                        r.requestID
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
                        let r: requestRandomnessReturn = r.into();
                        r.requestID
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `requestRandomnessWithSubscription(uint32,uint256)` and selector `0x1da53c9f`.
```solidity
function requestRandomnessWithSubscription(uint32 callbackGasLimit, uint256 subId) external payable returns (uint256 requestID);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestRandomnessWithSubscriptionCall {
        #[allow(missing_docs)]
        pub callbackGasLimit: u32,
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`requestRandomnessWithSubscription(uint32,uint256)`](requestRandomnessWithSubscriptionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestRandomnessWithSubscriptionReturn {
        #[allow(missing_docs)]
        pub requestID: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
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
            impl ::core::convert::From<requestRandomnessWithSubscriptionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestRandomnessWithSubscriptionCall) -> Self {
                    (value.callbackGasLimit, value.subId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestRandomnessWithSubscriptionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        callbackGasLimit: tuple.0,
                        subId: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<requestRandomnessWithSubscriptionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestRandomnessWithSubscriptionReturn) -> Self {
                    (value.requestID,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestRandomnessWithSubscriptionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestID: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestRandomnessWithSubscriptionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "requestRandomnessWithSubscription(uint32,uint256)";
            const SELECTOR: [u8; 4] = [29u8, 165u8, 60u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.callbackGasLimit),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.subId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: requestRandomnessWithSubscriptionReturn = r.into();
                        r.requestID
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
                        let r: requestRandomnessWithSubscriptionReturn = r.into();
                        r.requestID
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `requestSubscriptionOwnerTransfer(uint256,address)` and selector `0xdac83d29`.
```solidity
function requestSubscriptionOwnerTransfer(uint256 subId, address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestSubscriptionOwnerTransferCall {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`requestSubscriptionOwnerTransfer(uint256,address)`](requestSubscriptionOwnerTransferCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestSubscriptionOwnerTransferReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<requestSubscriptionOwnerTransferCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestSubscriptionOwnerTransferCall) -> Self {
                    (value.subId, value.newOwner)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestSubscriptionOwnerTransferCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        subId: tuple.0,
                        newOwner: tuple.1,
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
            impl ::core::convert::From<requestSubscriptionOwnerTransferReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestSubscriptionOwnerTransferReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestSubscriptionOwnerTransferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl requestSubscriptionOwnerTransferReturn {
            fn _tokenize(
                &self,
            ) -> <requestSubscriptionOwnerTransferCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestSubscriptionOwnerTransferCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = requestSubscriptionOwnerTransferReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "requestSubscriptionOwnerTransfer(uint256,address)";
            const SELECTOR: [u8; 4] = [218u8, 200u8, 61u8, 41u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.subId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                requestSubscriptionOwnerTransferReturn::_tokenize(ret)
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
    /**Function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`.
```solidity
function revokeRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`revokeRole(bytes32,address)`](revokeRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleReturn {}
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<revokeRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
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
            impl ::core::convert::From<revokeRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl revokeRoleReturn {
            fn _tokenize(
                &self,
            ) -> <revokeRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revokeRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = revokeRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revokeRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [213u8, 71u8, 116u8, 31u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                revokeRoleReturn::_tokenize(ret)
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
    /**Function with signature `s_config()` and selector `0x088070f5`.
```solidity
function s_config() external view returns (uint32 maxGasLimit, uint32 gasAfterPaymentCalculation, uint32 fulfillmentFlatFeeNativePPM, uint32 weiPerUnitGas, uint32 blsPairingCheckOverhead, uint8 nativePremiumPercentage, uint32 gasForCallExactCheck);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct s_configCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`s_config()`](s_configCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct s_configReturn {
        #[allow(missing_docs)]
        pub maxGasLimit: u32,
        #[allow(missing_docs)]
        pub gasAfterPaymentCalculation: u32,
        #[allow(missing_docs)]
        pub fulfillmentFlatFeeNativePPM: u32,
        #[allow(missing_docs)]
        pub weiPerUnitGas: u32,
        #[allow(missing_docs)]
        pub blsPairingCheckOverhead: u32,
        #[allow(missing_docs)]
        pub nativePremiumPercentage: u8,
        #[allow(missing_docs)]
        pub gasForCallExactCheck: u32,
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
            impl ::core::convert::From<s_configCall> for UnderlyingRustTuple<'_> {
                fn from(value: s_configCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for s_configCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, u32, u32, u32, u32, u8, u32);
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
            impl ::core::convert::From<s_configReturn> for UnderlyingRustTuple<'_> {
                fn from(value: s_configReturn) -> Self {
                    (
                        value.maxGasLimit,
                        value.gasAfterPaymentCalculation,
                        value.fulfillmentFlatFeeNativePPM,
                        value.weiPerUnitGas,
                        value.blsPairingCheckOverhead,
                        value.nativePremiumPercentage,
                        value.gasForCallExactCheck,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for s_configReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        maxGasLimit: tuple.0,
                        gasAfterPaymentCalculation: tuple.1,
                        fulfillmentFlatFeeNativePPM: tuple.2,
                        weiPerUnitGas: tuple.3,
                        blsPairingCheckOverhead: tuple.4,
                        nativePremiumPercentage: tuple.5,
                        gasForCallExactCheck: tuple.6,
                    }
                }
            }
        }
        impl s_configReturn {
            fn _tokenize(
                &self,
            ) -> <s_configCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxGasLimit),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.gasAfterPaymentCalculation,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.fulfillmentFlatFeeNativePPM,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.weiPerUnitGas),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.blsPairingCheckOverhead,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nativePremiumPercentage,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasForCallExactCheck),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for s_configCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = s_configReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "s_config()";
            const SELECTOR: [u8; 4] = [8u8, 128u8, 112u8, 245u8];
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
                s_configReturn::_tokenize(ret)
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
    /**Function with signature `s_configured()` and selector `0x57a8070a`.
```solidity
function s_configured() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct s_configuredCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`s_configured()`](s_configuredCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct s_configuredReturn {
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
            impl ::core::convert::From<s_configuredCall> for UnderlyingRustTuple<'_> {
                fn from(value: s_configuredCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for s_configuredCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<s_configuredReturn> for UnderlyingRustTuple<'_> {
                fn from(value: s_configuredReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for s_configuredReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for s_configuredCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "s_configured()";
            const SELECTOR: [u8; 4] = [87u8, 168u8, 7u8, 10u8];
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
                        let r: s_configuredReturn = r.into();
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
                        let r: s_configuredReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `s_currentSubNonce()` and selector `0x9d40a6fd`.
```solidity
function s_currentSubNonce() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct s_currentSubNonceCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`s_currentSubNonce()`](s_currentSubNonceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct s_currentSubNonceReturn {
        #[allow(missing_docs)]
        pub _0: u64,
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
            impl ::core::convert::From<s_currentSubNonceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: s_currentSubNonceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for s_currentSubNonceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<s_currentSubNonceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: s_currentSubNonceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for s_currentSubNonceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for s_currentSubNonceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "s_currentSubNonce()";
            const SELECTOR: [u8; 4] = [157u8, 64u8, 166u8, 253u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: s_currentSubNonceReturn = r.into();
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
                        let r: s_currentSubNonceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `s_disabled()` and selector `0xa608a1e1`.
```solidity
function s_disabled() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct s_disabledCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`s_disabled()`](s_disabledCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct s_disabledReturn {
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
            impl ::core::convert::From<s_disabledCall> for UnderlyingRustTuple<'_> {
                fn from(value: s_disabledCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for s_disabledCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<s_disabledReturn> for UnderlyingRustTuple<'_> {
                fn from(value: s_disabledReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for s_disabledReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for s_disabledCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "s_disabled()";
            const SELECTOR: [u8; 4] = [166u8, 8u8, 161u8, 225u8];
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
                        let r: s_disabledReturn = r.into();
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
                        let r: s_disabledReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `s_totalNativeBalance()` and selector `0x18e3dd27`.
```solidity
function s_totalNativeBalance() external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct s_totalNativeBalanceCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`s_totalNativeBalance()`](s_totalNativeBalanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct s_totalNativeBalanceReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<s_totalNativeBalanceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: s_totalNativeBalanceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for s_totalNativeBalanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<s_totalNativeBalanceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: s_totalNativeBalanceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for s_totalNativeBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for s_totalNativeBalanceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U96;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "s_totalNativeBalance()";
            const SELECTOR: [u8; 4] = [24u8, 227u8, 221u8, 39u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: s_totalNativeBalanceReturn = r.into();
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
                        let r: s_totalNativeBalanceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `s_withdrawableDirectFundingFeeNative()` and selector `0x3bc32c75`.
```solidity
function s_withdrawableDirectFundingFeeNative() external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct s_withdrawableDirectFundingFeeNativeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`s_withdrawableDirectFundingFeeNative()`](s_withdrawableDirectFundingFeeNativeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct s_withdrawableDirectFundingFeeNativeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<s_withdrawableDirectFundingFeeNativeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: s_withdrawableDirectFundingFeeNativeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for s_withdrawableDirectFundingFeeNativeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<s_withdrawableDirectFundingFeeNativeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: s_withdrawableDirectFundingFeeNativeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for s_withdrawableDirectFundingFeeNativeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for s_withdrawableDirectFundingFeeNativeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U96;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "s_withdrawableDirectFundingFeeNative()";
            const SELECTOR: [u8; 4] = [59u8, 195u8, 44u8, 117u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: s_withdrawableDirectFundingFeeNativeReturn = r.into();
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
                        let r: s_withdrawableDirectFundingFeeNativeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `s_withdrawableSubscriptionFeeNative()` and selector `0x995cb36e`.
```solidity
function s_withdrawableSubscriptionFeeNative() external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct s_withdrawableSubscriptionFeeNativeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`s_withdrawableSubscriptionFeeNative()`](s_withdrawableSubscriptionFeeNativeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct s_withdrawableSubscriptionFeeNativeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<s_withdrawableSubscriptionFeeNativeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: s_withdrawableSubscriptionFeeNativeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for s_withdrawableSubscriptionFeeNativeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<s_withdrawableSubscriptionFeeNativeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: s_withdrawableSubscriptionFeeNativeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for s_withdrawableSubscriptionFeeNativeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for s_withdrawableSubscriptionFeeNativeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U96;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "s_withdrawableSubscriptionFeeNative()";
            const SELECTOR: [u8; 4] = [153u8, 92u8, 179u8, 110u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: s_withdrawableSubscriptionFeeNativeReturn = r.into();
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
                        let r: s_withdrawableSubscriptionFeeNativeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setConfig(uint32,uint32,uint32,uint32,uint32,uint8,uint32)` and selector `0x45fa4354`.
```solidity
function setConfig(uint32 maxGasLimit, uint32 gasAfterPaymentCalculation, uint32 fulfillmentFlatFeeNativePPM, uint32 weiPerUnitGas, uint32 blsPairingCheckOverhead, uint8 nativePremiumPercentage, uint32 gasForCallExactCheck) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setConfigCall {
        #[allow(missing_docs)]
        pub maxGasLimit: u32,
        #[allow(missing_docs)]
        pub gasAfterPaymentCalculation: u32,
        #[allow(missing_docs)]
        pub fulfillmentFlatFeeNativePPM: u32,
        #[allow(missing_docs)]
        pub weiPerUnitGas: u32,
        #[allow(missing_docs)]
        pub blsPairingCheckOverhead: u32,
        #[allow(missing_docs)]
        pub nativePremiumPercentage: u8,
        #[allow(missing_docs)]
        pub gasForCallExactCheck: u32,
    }
    ///Container type for the return parameters of the [`setConfig(uint32,uint32,uint32,uint32,uint32,uint8,uint32)`](setConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setConfigReturn {}
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, u32, u32, u32, u32, u8, u32);
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
            impl ::core::convert::From<setConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: setConfigCall) -> Self {
                    (
                        value.maxGasLimit,
                        value.gasAfterPaymentCalculation,
                        value.fulfillmentFlatFeeNativePPM,
                        value.weiPerUnitGas,
                        value.blsPairingCheckOverhead,
                        value.nativePremiumPercentage,
                        value.gasForCallExactCheck,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        maxGasLimit: tuple.0,
                        gasAfterPaymentCalculation: tuple.1,
                        fulfillmentFlatFeeNativePPM: tuple.2,
                        weiPerUnitGas: tuple.3,
                        blsPairingCheckOverhead: tuple.4,
                        nativePremiumPercentage: tuple.5,
                        gasForCallExactCheck: tuple.6,
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
            impl ::core::convert::From<setConfigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setConfigReturn {
            fn _tokenize(
                &self,
            ) -> <setConfigCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setConfigCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setConfig(uint32,uint32,uint32,uint32,uint32,uint8,uint32)";
            const SELECTOR: [u8; 4] = [69u8, 250u8, 67u8, 84u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxGasLimit),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.gasAfterPaymentCalculation,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.fulfillmentFlatFeeNativePPM,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.weiPerUnitGas),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.blsPairingCheckOverhead,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nativePremiumPercentage,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasForCallExactCheck),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setConfigReturn::_tokenize(ret)
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
    /**Function with signature `setSignatureSender(address)` and selector `0xf8fa0d66`.
```solidity
function setSignatureSender(address newSignatureSender) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSignatureSenderCall {
        #[allow(missing_docs)]
        pub newSignatureSender: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setSignatureSender(address)`](setSignatureSenderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSignatureSenderReturn {}
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
            impl ::core::convert::From<setSignatureSenderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setSignatureSenderCall) -> Self {
                    (value.newSignatureSender,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setSignatureSenderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newSignatureSender: tuple.0,
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
            impl ::core::convert::From<setSignatureSenderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setSignatureSenderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setSignatureSenderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setSignatureSenderReturn {
            fn _tokenize(
                &self,
            ) -> <setSignatureSenderCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setSignatureSenderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setSignatureSenderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setSignatureSender(address)";
            const SELECTOR: [u8; 4] = [248u8, 250u8, 13u8, 102u8];
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
                        &self.newSignatureSender,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setSignatureSenderReturn::_tokenize(ret)
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
    /**Function with signature `signatureSender()` and selector `0x7d468106`.
```solidity
function signatureSender() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct signatureSenderCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`signatureSender()`](signatureSenderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct signatureSenderReturn {
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
            impl ::core::convert::From<signatureSenderCall> for UnderlyingRustTuple<'_> {
                fn from(value: signatureSenderCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for signatureSenderCall {
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
            impl ::core::convert::From<signatureSenderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: signatureSenderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for signatureSenderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for signatureSenderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "signatureSender()";
            const SELECTOR: [u8; 4] = [125u8, 70u8, 129u8, 6u8];
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
                        let r: signatureSenderReturn = r.into();
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
                        let r: signatureSenderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`.
```solidity
function supportsInterface(bytes4 interfaceId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceCall {
        #[allow(missing_docs)]
        pub interfaceId: alloy::sol_types::private::FixedBytes<4>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`supportsInterface(bytes4)`](supportsInterfaceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
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
            impl ::core::convert::From<supportsInterfaceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceCall) -> Self {
                    (value.interfaceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { interfaceId: tuple.0 }
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
            impl ::core::convert::From<supportsInterfaceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsInterfaceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsInterface(bytes4)";
            const SELECTOR: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
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
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.interfaceId),
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
                        let r: supportsInterfaceReturn = r.into();
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
                        let r: supportsInterfaceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`.
```solidity
function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeToAndCallCall {
        #[allow(missing_docs)]
        pub newImplementation: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`upgradeToAndCall(address,bytes)`](upgradeToAndCallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeToAndCallReturn {}
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
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<upgradeToAndCallCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: upgradeToAndCallCall) -> Self {
                    (value.newImplementation, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for upgradeToAndCallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newImplementation: tuple.0,
                        data: tuple.1,
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
            impl ::core::convert::From<upgradeToAndCallReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: upgradeToAndCallReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for upgradeToAndCallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl upgradeToAndCallReturn {
            fn _tokenize(
                &self,
            ) -> <upgradeToAndCallCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for upgradeToAndCallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = upgradeToAndCallReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "upgradeToAndCall(address,bytes)";
            const SELECTOR: [u8; 4] = [79u8, 30u8, 242u8, 134u8];
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
                        &self.newImplementation,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                upgradeToAndCallReturn::_tokenize(ret)
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
    /**Function with signature `version()` and selector `0x54fd4d50`.
```solidity
function version() external pure returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct versionCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`version()`](versionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct versionReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
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
            impl ::core::convert::From<versionCall> for UnderlyingRustTuple<'_> {
                fn from(value: versionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for versionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
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
            impl ::core::convert::From<versionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: versionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for versionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for versionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "version()";
            const SELECTOR: [u8; 4] = [84u8, 253u8, 77u8, 80u8];
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
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
                        let r: versionReturn = r.into();
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
                        let r: versionReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `withdrawDirectFundingFeesNative(address)` and selector `0x54236fb3`.
```solidity
function withdrawDirectFundingFeesNative(address recipient) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawDirectFundingFeesNativeCall {
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`withdrawDirectFundingFeesNative(address)`](withdrawDirectFundingFeesNativeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawDirectFundingFeesNativeReturn {}
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
            impl ::core::convert::From<withdrawDirectFundingFeesNativeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawDirectFundingFeesNativeCall) -> Self {
                    (value.recipient,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawDirectFundingFeesNativeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { recipient: tuple.0 }
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
            impl ::core::convert::From<withdrawDirectFundingFeesNativeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawDirectFundingFeesNativeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawDirectFundingFeesNativeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl withdrawDirectFundingFeesNativeReturn {
            fn _tokenize(
                &self,
            ) -> <withdrawDirectFundingFeesNativeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawDirectFundingFeesNativeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawDirectFundingFeesNativeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawDirectFundingFeesNative(address)";
            const SELECTOR: [u8; 4] = [84u8, 35u8, 111u8, 179u8];
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
                        &self.recipient,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                withdrawDirectFundingFeesNativeReturn::_tokenize(ret)
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
    /**Function with signature `withdrawSubscriptionFeesNative(address)` and selector `0xbd18636b`.
```solidity
function withdrawSubscriptionFeesNative(address recipient) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawSubscriptionFeesNativeCall {
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`withdrawSubscriptionFeesNative(address)`](withdrawSubscriptionFeesNativeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawSubscriptionFeesNativeReturn {}
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
            impl ::core::convert::From<withdrawSubscriptionFeesNativeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawSubscriptionFeesNativeCall) -> Self {
                    (value.recipient,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawSubscriptionFeesNativeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { recipient: tuple.0 }
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
            impl ::core::convert::From<withdrawSubscriptionFeesNativeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawSubscriptionFeesNativeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawSubscriptionFeesNativeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl withdrawSubscriptionFeesNativeReturn {
            fn _tokenize(
                &self,
            ) -> <withdrawSubscriptionFeesNativeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawSubscriptionFeesNativeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawSubscriptionFeesNativeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawSubscriptionFeesNative(address)";
            const SELECTOR: [u8; 4] = [189u8, 24u8, 99u8, 107u8];
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
                        &self.recipient,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                withdrawSubscriptionFeesNativeReturn::_tokenize(ret)
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
    ///Container for all the [`RandomnessSender`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum RandomnessSenderCalls {
        #[allow(missing_docs)]
        ADMIN_ROLE(ADMIN_ROLECall),
        #[allow(missing_docs)]
        DEFAULT_ADMIN_ROLE(DEFAULT_ADMIN_ROLECall),
        #[allow(missing_docs)]
        MAX_CONSUMERS(MAX_CONSUMERSCall),
        #[allow(missing_docs)]
        SCHEME_ID(SCHEME_IDCall),
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        acceptSubscriptionOwnerTransfer(acceptSubscriptionOwnerTransferCall),
        #[allow(missing_docs)]
        addConsumer(addConsumerCall),
        #[allow(missing_docs)]
        calculateRequestPriceNative(calculateRequestPriceNativeCall),
        #[allow(missing_docs)]
        cancelSubscription(cancelSubscriptionCall),
        #[allow(missing_docs)]
        createSubscription(createSubscriptionCall),
        #[allow(missing_docs)]
        disable(disableCall),
        #[allow(missing_docs)]
        enable(enableCall),
        #[allow(missing_docs)]
        estimateRequestPriceNative(estimateRequestPriceNativeCall),
        #[allow(missing_docs)]
        fundSubscriptionWithNative(fundSubscriptionWithNativeCall),
        #[allow(missing_docs)]
        getActiveSubscriptionIds(getActiveSubscriptionIdsCall),
        #[allow(missing_docs)]
        getAllRequests(getAllRequestsCall),
        #[allow(missing_docs)]
        getConfig(getConfigCall),
        #[allow(missing_docs)]
        getRequest(getRequestCall),
        #[allow(missing_docs)]
        getRoleAdmin(getRoleAdminCall),
        #[allow(missing_docs)]
        getRoleMember(getRoleMemberCall),
        #[allow(missing_docs)]
        getRoleMemberCount(getRoleMemberCountCall),
        #[allow(missing_docs)]
        getRoleMembers(getRoleMembersCall),
        #[allow(missing_docs)]
        getSubscription(getSubscriptionCall),
        #[allow(missing_docs)]
        grantRole(grantRoleCall),
        #[allow(missing_docs)]
        hasRole(hasRoleCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isInFlight(isInFlightCall),
        #[allow(missing_docs)]
        messageFrom(messageFromCall),
        #[allow(missing_docs)]
        nonce(nonceCall),
        #[allow(missing_docs)]
        ownerCancelSubscription(ownerCancelSubscriptionCall),
        #[allow(missing_docs)]
        pendingRequestExists(pendingRequestExistsCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        receiveSignature(receiveSignatureCall),
        #[allow(missing_docs)]
        removeConsumer(removeConsumerCall),
        #[allow(missing_docs)]
        renounceRole(renounceRoleCall),
        #[allow(missing_docs)]
        requestRandomness(requestRandomnessCall),
        #[allow(missing_docs)]
        requestRandomnessWithSubscription(requestRandomnessWithSubscriptionCall),
        #[allow(missing_docs)]
        requestSubscriptionOwnerTransfer(requestSubscriptionOwnerTransferCall),
        #[allow(missing_docs)]
        revokeRole(revokeRoleCall),
        #[allow(missing_docs)]
        s_config(s_configCall),
        #[allow(missing_docs)]
        s_configured(s_configuredCall),
        #[allow(missing_docs)]
        s_currentSubNonce(s_currentSubNonceCall),
        #[allow(missing_docs)]
        s_disabled(s_disabledCall),
        #[allow(missing_docs)]
        s_totalNativeBalance(s_totalNativeBalanceCall),
        #[allow(missing_docs)]
        s_withdrawableDirectFundingFeeNative(s_withdrawableDirectFundingFeeNativeCall),
        #[allow(missing_docs)]
        s_withdrawableSubscriptionFeeNative(s_withdrawableSubscriptionFeeNativeCall),
        #[allow(missing_docs)]
        setConfig(setConfigCall),
        #[allow(missing_docs)]
        setSignatureSender(setSignatureSenderCall),
        #[allow(missing_docs)]
        signatureSender(signatureSenderCall),
        #[allow(missing_docs)]
        supportsInterface(supportsInterfaceCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
        #[allow(missing_docs)]
        version(versionCall),
        #[allow(missing_docs)]
        withdrawDirectFundingFeesNative(withdrawDirectFundingFeesNativeCall),
        #[allow(missing_docs)]
        withdrawSubscriptionFeesNative(withdrawSubscriptionFeesNativeCall),
    }
    impl RandomnessSenderCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 255u8, 201u8, 167u8],
            [8u8, 128u8, 112u8, 245u8],
            [10u8, 224u8, 149u8, 64u8],
            [24u8, 227u8, 221u8, 39u8],
            [29u8, 165u8, 60u8, 159u8],
            [36u8, 138u8, 156u8, 163u8],
            [47u8, 39u8, 112u8, 219u8],
            [47u8, 47u8, 241u8, 93u8],
            [50u8, 85u8, 196u8, 86u8],
            [54u8, 86u8, 138u8, 190u8],
            [59u8, 195u8, 44u8, 117u8],
            [65u8, 175u8, 108u8, 135u8],
            [69u8, 250u8, 67u8, 84u8],
            [72u8, 92u8, 201u8, 85u8],
            [75u8, 22u8, 9u8, 53u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [84u8, 35u8, 111u8, 179u8],
            [84u8, 253u8, 77u8, 80u8],
            [87u8, 168u8, 7u8, 10u8],
            [100u8, 213u8, 26u8, 42u8],
            [117u8, 178u8, 56u8, 252u8],
            [119u8, 91u8, 131u8, 156u8],
            [125u8, 70u8, 129u8, 6u8],
            [129u8, 30u8, 227u8, 42u8],
            [138u8, 31u8, 22u8, 90u8],
            [144u8, 16u8, 208u8, 124u8],
            [145u8, 209u8, 72u8, 84u8],
            [149u8, 181u8, 92u8, 252u8],
            [153u8, 92u8, 179u8, 110u8],
            [157u8, 64u8, 166u8, 253u8],
            [162u8, 23u8, 253u8, 223u8],
            [162u8, 26u8, 35u8, 228u8],
            [163u8, 36u8, 106u8, 211u8],
            [163u8, 144u8, 125u8, 113u8],
            [166u8, 8u8, 161u8, 225u8],
            [170u8, 67u8, 58u8, 255u8],
            [173u8, 60u8, 177u8, 204u8],
            [174u8, 251u8, 33u8, 47u8],
            [175u8, 254u8, 208u8, 224u8],
            [178u8, 167u8, 202u8, 197u8],
            [189u8, 24u8, 99u8, 107u8],
            [190u8, 196u8, 192u8, 140u8],
            [195u8, 249u8, 9u8, 212u8],
            [197u8, 131u8, 67u8, 239u8],
            [200u8, 219u8, 101u8, 130u8],
            [202u8, 21u8, 200u8, 115u8],
            [203u8, 99u8, 23u8, 151u8],
            [205u8, 128u8, 44u8, 145u8],
            [213u8, 71u8, 116u8, 31u8],
            [218u8, 200u8, 61u8, 41u8],
            [220u8, 49u8, 29u8, 211u8],
            [248u8, 250u8, 13u8, 102u8],
            [251u8, 26u8, 0u8, 42u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(supportsInterface),
            ::core::stringify!(s_config),
            ::core::stringify!(cancelSubscription),
            ::core::stringify!(s_totalNativeBalance),
            ::core::stringify!(requestRandomnessWithSubscription),
            ::core::stringify!(getRoleAdmin),
            ::core::stringify!(disable),
            ::core::stringify!(grantRole),
            ::core::stringify!(estimateRequestPriceNative),
            ::core::stringify!(renounceRole),
            ::core::stringify!(s_withdrawableDirectFundingFeeNative),
            ::core::stringify!(pendingRequestExists),
            ::core::stringify!(setConfig),
            ::core::stringify!(initialize),
            ::core::stringify!(calculateRequestPriceNative),
            ::core::stringify!(upgradeToAndCall),
            ::core::stringify!(proxiableUUID),
            ::core::stringify!(withdrawDirectFundingFeesNative),
            ::core::stringify!(version),
            ::core::stringify!(s_configured),
            ::core::stringify!(MAX_CONSUMERS),
            ::core::stringify!(ADMIN_ROLE),
            ::core::stringify!(messageFrom),
            ::core::stringify!(signatureSender),
            ::core::stringify!(requestRandomness),
            ::core::stringify!(SCHEME_ID),
            ::core::stringify!(getRoleMember),
            ::core::stringify!(hasRole),
            ::core::stringify!(fundSubscriptionWithNative),
            ::core::stringify!(s_withdrawableSubscriptionFeeNative),
            ::core::stringify!(s_currentSubNonce),
            ::core::stringify!(DEFAULT_ADMIN_ROLE),
            ::core::stringify!(createSubscription),
            ::core::stringify!(getRoleMembers),
            ::core::stringify!(enable),
            ::core::stringify!(s_disabled),
            ::core::stringify!(ownerCancelSubscription),
            ::core::stringify!(UPGRADE_INTERFACE_VERSION),
            ::core::stringify!(getActiveSubscriptionIds),
            ::core::stringify!(nonce),
            ::core::stringify!(acceptSubscriptionOwnerTransfer),
            ::core::stringify!(withdrawSubscriptionFeesNative),
            ::core::stringify!(addConsumer),
            ::core::stringify!(getConfig),
            ::core::stringify!(getRequest),
            ::core::stringify!(receiveSignature),
            ::core::stringify!(getRoleMemberCount),
            ::core::stringify!(removeConsumer),
            ::core::stringify!(isInFlight),
            ::core::stringify!(revokeRole),
            ::core::stringify!(requestSubscriptionOwnerTransfer),
            ::core::stringify!(getSubscription),
            ::core::stringify!(setSignatureSender),
            ::core::stringify!(getAllRequests),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <supportsInterfaceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <s_configCall as alloy_sol_types::SolCall>::SIGNATURE,
            <cancelSubscriptionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <s_totalNativeBalanceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <requestRandomnessWithSubscriptionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <disableCall as alloy_sol_types::SolCall>::SIGNATURE,
            <grantRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <estimateRequestPriceNativeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <s_withdrawableDirectFundingFeeNativeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pendingRequestExistsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <calculateRequestPriceNativeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeToAndCallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxiableUUIDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <withdrawDirectFundingFeesNativeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <versionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <s_configuredCall as alloy_sol_types::SolCall>::SIGNATURE,
            <MAX_CONSUMERSCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ADMIN_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <messageFromCall as alloy_sol_types::SolCall>::SIGNATURE,
            <signatureSenderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <requestRandomnessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <SCHEME_IDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleMemberCall as alloy_sol_types::SolCall>::SIGNATURE,
            <hasRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <fundSubscriptionWithNativeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <s_withdrawableSubscriptionFeeNativeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <s_currentSubNonceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <createSubscriptionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleMembersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <enableCall as alloy_sol_types::SolCall>::SIGNATURE,
            <s_disabledCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCancelSubscriptionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getActiveSubscriptionIdsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <nonceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <acceptSubscriptionOwnerTransferCall as alloy_sol_types::SolCall>::SIGNATURE,
            <withdrawSubscriptionFeesNativeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <addConsumerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRequestCall as alloy_sol_types::SolCall>::SIGNATURE,
            <receiveSignatureCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleMemberCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <removeConsumerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isInFlightCall as alloy_sol_types::SolCall>::SIGNATURE,
            <revokeRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <requestSubscriptionOwnerTransferCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getSubscriptionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setSignatureSenderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getAllRequestsCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for RandomnessSenderCalls {
        const NAME: &'static str = "RandomnessSenderCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 54usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ADMIN_ROLE(_) => {
                    <ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DEFAULT_ADMIN_ROLE(_) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_CONSUMERS(_) => {
                    <MAX_CONSUMERSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::SCHEME_ID(_) => {
                    <SCHEME_IDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::acceptSubscriptionOwnerTransfer(_) => {
                    <acceptSubscriptionOwnerTransferCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addConsumer(_) => {
                    <addConsumerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateRequestPriceNative(_) => {
                    <calculateRequestPriceNativeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancelSubscription(_) => {
                    <cancelSubscriptionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createSubscription(_) => {
                    <createSubscriptionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disable(_) => <disableCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::enable(_) => <enableCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::estimateRequestPriceNative(_) => {
                    <estimateRequestPriceNativeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::fundSubscriptionWithNative(_) => {
                    <fundSubscriptionWithNativeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getActiveSubscriptionIds(_) => {
                    <getActiveSubscriptionIdsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllRequests(_) => {
                    <getAllRequestsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getConfig(_) => {
                    <getConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRequest(_) => {
                    <getRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRoleAdmin(_) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRoleMember(_) => {
                    <getRoleMemberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRoleMemberCount(_) => {
                    <getRoleMemberCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRoleMembers(_) => {
                    <getRoleMembersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getSubscription(_) => {
                    <getSubscriptionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::grantRole(_) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasRole(_) => <hasRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isInFlight(_) => {
                    <isInFlightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::messageFrom(_) => {
                    <messageFromCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nonce(_) => <nonceCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::ownerCancelSubscription(_) => {
                    <ownerCancelSubscriptionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pendingRequestExists(_) => {
                    <pendingRequestExistsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::receiveSignature(_) => {
                    <receiveSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeConsumer(_) => {
                    <removeConsumerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceRole(_) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::requestRandomness(_) => {
                    <requestRandomnessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::requestRandomnessWithSubscription(_) => {
                    <requestRandomnessWithSubscriptionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::requestSubscriptionOwnerTransfer(_) => {
                    <requestSubscriptionOwnerTransferCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::revokeRole(_) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::s_config(_) => <s_configCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::s_configured(_) => {
                    <s_configuredCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::s_currentSubNonce(_) => {
                    <s_currentSubNonceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::s_disabled(_) => {
                    <s_disabledCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::s_totalNativeBalance(_) => {
                    <s_totalNativeBalanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::s_withdrawableDirectFundingFeeNative(_) => {
                    <s_withdrawableDirectFundingFeeNativeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::s_withdrawableSubscriptionFeeNative(_) => {
                    <s_withdrawableSubscriptionFeeNativeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setConfig(_) => {
                    <setConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setSignatureSender(_) => {
                    <setSignatureSenderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::signatureSender(_) => {
                    <signatureSenderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::upgradeToAndCall(_) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::version(_) => <versionCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::withdrawDirectFundingFeesNative(_) => {
                    <withdrawDirectFundingFeesNativeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawSubscriptionFeesNative(_) => {
                    <withdrawSubscriptionFeesNativeCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<RandomnessSenderCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn s_config(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <s_configCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RandomnessSenderCalls::s_config)
                    }
                    s_config
                },
                {
                    fn cancelSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <cancelSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::cancelSubscription)
                    }
                    cancelSubscription
                },
                {
                    fn s_totalNativeBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <s_totalNativeBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::s_totalNativeBalance)
                    }
                    s_totalNativeBalance
                },
                {
                    fn requestRandomnessWithSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <requestRandomnessWithSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RandomnessSenderCalls::requestRandomnessWithSubscription,
                            )
                    }
                    requestRandomnessWithSubscription
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn disable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <disableCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RandomnessSenderCalls::disable)
                    }
                    disable
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RandomnessSenderCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn estimateRequestPriceNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <estimateRequestPriceNativeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::estimateRequestPriceNative)
                    }
                    estimateRequestPriceNative
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn s_withdrawableDirectFundingFeeNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <s_withdrawableDirectFundingFeeNativeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RandomnessSenderCalls::s_withdrawableDirectFundingFeeNative,
                            )
                    }
                    s_withdrawableDirectFundingFeeNative
                },
                {
                    fn pendingRequestExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <pendingRequestExistsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::pendingRequestExists)
                    }
                    pendingRequestExists
                },
                {
                    fn setConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <setConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RandomnessSenderCalls::setConfig)
                    }
                    setConfig
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::initialize)
                    }
                    initialize
                },
                {
                    fn calculateRequestPriceNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <calculateRequestPriceNativeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::calculateRequestPriceNative)
                    }
                    calculateRequestPriceNative
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn withdrawDirectFundingFeesNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <withdrawDirectFundingFeesNativeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::withdrawDirectFundingFeesNative)
                    }
                    withdrawDirectFundingFeesNative
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RandomnessSenderCalls::version)
                    }
                    version
                },
                {
                    fn s_configured(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <s_configuredCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::s_configured)
                    }
                    s_configured
                },
                {
                    fn MAX_CONSUMERS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <MAX_CONSUMERSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::MAX_CONSUMERS)
                    }
                    MAX_CONSUMERS
                },
                {
                    fn ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::ADMIN_ROLE)
                    }
                    ADMIN_ROLE
                },
                {
                    fn messageFrom(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <messageFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::messageFrom)
                    }
                    messageFrom
                },
                {
                    fn signatureSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <signatureSenderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::signatureSender)
                    }
                    signatureSender
                },
                {
                    fn requestRandomness(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <requestRandomnessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::requestRandomness)
                    }
                    requestRandomness
                },
                {
                    fn SCHEME_ID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <SCHEME_IDCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RandomnessSenderCalls::SCHEME_ID)
                    }
                    SCHEME_ID
                },
                {
                    fn getRoleMember(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getRoleMemberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::getRoleMember)
                    }
                    getRoleMember
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RandomnessSenderCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn fundSubscriptionWithNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <fundSubscriptionWithNativeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::fundSubscriptionWithNative)
                    }
                    fundSubscriptionWithNative
                },
                {
                    fn s_withdrawableSubscriptionFeeNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <s_withdrawableSubscriptionFeeNativeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RandomnessSenderCalls::s_withdrawableSubscriptionFeeNative,
                            )
                    }
                    s_withdrawableSubscriptionFeeNative
                },
                {
                    fn s_currentSubNonce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <s_currentSubNonceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::s_currentSubNonce)
                    }
                    s_currentSubNonce
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn createSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <createSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::createSubscription)
                    }
                    createSubscription
                },
                {
                    fn getRoleMembers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getRoleMembersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::getRoleMembers)
                    }
                    getRoleMembers
                },
                {
                    fn enable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <enableCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RandomnessSenderCalls::enable)
                    }
                    enable
                },
                {
                    fn s_disabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <s_disabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::s_disabled)
                    }
                    s_disabled
                },
                {
                    fn ownerCancelSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <ownerCancelSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::ownerCancelSubscription)
                    }
                    ownerCancelSubscription
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn getActiveSubscriptionIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getActiveSubscriptionIdsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::getActiveSubscriptionIds)
                    }
                    getActiveSubscriptionIds
                },
                {
                    fn nonce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <nonceCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RandomnessSenderCalls::nonce)
                    }
                    nonce
                },
                {
                    fn acceptSubscriptionOwnerTransfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <acceptSubscriptionOwnerTransferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::acceptSubscriptionOwnerTransfer)
                    }
                    acceptSubscriptionOwnerTransfer
                },
                {
                    fn withdrawSubscriptionFeesNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <withdrawSubscriptionFeesNativeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::withdrawSubscriptionFeesNative)
                    }
                    withdrawSubscriptionFeesNative
                },
                {
                    fn addConsumer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <addConsumerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::addConsumer)
                    }
                    addConsumer
                },
                {
                    fn getConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RandomnessSenderCalls::getConfig)
                    }
                    getConfig
                },
                {
                    fn getRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::getRequest)
                    }
                    getRequest
                },
                {
                    fn receiveSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <receiveSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::receiveSignature)
                    }
                    receiveSignature
                },
                {
                    fn getRoleMemberCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getRoleMemberCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::getRoleMemberCount)
                    }
                    getRoleMemberCount
                },
                {
                    fn removeConsumer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <removeConsumerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::removeConsumer)
                    }
                    removeConsumer
                },
                {
                    fn isInFlight(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <isInFlightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::isInFlight)
                    }
                    isInFlight
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn requestSubscriptionOwnerTransfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <requestSubscriptionOwnerTransferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::requestSubscriptionOwnerTransfer)
                    }
                    requestSubscriptionOwnerTransfer
                },
                {
                    fn getSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::getSubscription)
                    }
                    getSubscription
                },
                {
                    fn setSignatureSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <setSignatureSenderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::setSignatureSender)
                    }
                    setSignatureSender
                },
                {
                    fn getAllRequests(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getAllRequestsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderCalls::getAllRequests)
                    }
                    getAllRequests
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
            ) -> alloy_sol_types::Result<RandomnessSenderCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn s_config(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <s_configCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::s_config)
                    }
                    s_config
                },
                {
                    fn cancelSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <cancelSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::cancelSubscription)
                    }
                    cancelSubscription
                },
                {
                    fn s_totalNativeBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <s_totalNativeBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::s_totalNativeBalance)
                    }
                    s_totalNativeBalance
                },
                {
                    fn requestRandomnessWithSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <requestRandomnessWithSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RandomnessSenderCalls::requestRandomnessWithSubscription,
                            )
                    }
                    requestRandomnessWithSubscription
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn disable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <disableCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::disable)
                    }
                    disable
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn estimateRequestPriceNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <estimateRequestPriceNativeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::estimateRequestPriceNative)
                    }
                    estimateRequestPriceNative
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn s_withdrawableDirectFundingFeeNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <s_withdrawableDirectFundingFeeNativeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RandomnessSenderCalls::s_withdrawableDirectFundingFeeNative,
                            )
                    }
                    s_withdrawableDirectFundingFeeNative
                },
                {
                    fn pendingRequestExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <pendingRequestExistsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::pendingRequestExists)
                    }
                    pendingRequestExists
                },
                {
                    fn setConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <setConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::setConfig)
                    }
                    setConfig
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::initialize)
                    }
                    initialize
                },
                {
                    fn calculateRequestPriceNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <calculateRequestPriceNativeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::calculateRequestPriceNative)
                    }
                    calculateRequestPriceNative
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn withdrawDirectFundingFeesNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <withdrawDirectFundingFeesNativeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::withdrawDirectFundingFeesNative)
                    }
                    withdrawDirectFundingFeesNative
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::version)
                    }
                    version
                },
                {
                    fn s_configured(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <s_configuredCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::s_configured)
                    }
                    s_configured
                },
                {
                    fn MAX_CONSUMERS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <MAX_CONSUMERSCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::MAX_CONSUMERS)
                    }
                    MAX_CONSUMERS
                },
                {
                    fn ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::ADMIN_ROLE)
                    }
                    ADMIN_ROLE
                },
                {
                    fn messageFrom(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <messageFromCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::messageFrom)
                    }
                    messageFrom
                },
                {
                    fn signatureSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <signatureSenderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::signatureSender)
                    }
                    signatureSender
                },
                {
                    fn requestRandomness(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <requestRandomnessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::requestRandomness)
                    }
                    requestRandomness
                },
                {
                    fn SCHEME_ID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <SCHEME_IDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::SCHEME_ID)
                    }
                    SCHEME_ID
                },
                {
                    fn getRoleMember(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getRoleMemberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::getRoleMember)
                    }
                    getRoleMember
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn fundSubscriptionWithNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <fundSubscriptionWithNativeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::fundSubscriptionWithNative)
                    }
                    fundSubscriptionWithNative
                },
                {
                    fn s_withdrawableSubscriptionFeeNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <s_withdrawableSubscriptionFeeNativeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RandomnessSenderCalls::s_withdrawableSubscriptionFeeNative,
                            )
                    }
                    s_withdrawableSubscriptionFeeNative
                },
                {
                    fn s_currentSubNonce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <s_currentSubNonceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::s_currentSubNonce)
                    }
                    s_currentSubNonce
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn createSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <createSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::createSubscription)
                    }
                    createSubscription
                },
                {
                    fn getRoleMembers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getRoleMembersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::getRoleMembers)
                    }
                    getRoleMembers
                },
                {
                    fn enable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <enableCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::enable)
                    }
                    enable
                },
                {
                    fn s_disabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <s_disabledCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::s_disabled)
                    }
                    s_disabled
                },
                {
                    fn ownerCancelSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <ownerCancelSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::ownerCancelSubscription)
                    }
                    ownerCancelSubscription
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn getActiveSubscriptionIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getActiveSubscriptionIdsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::getActiveSubscriptionIds)
                    }
                    getActiveSubscriptionIds
                },
                {
                    fn nonce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <nonceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::nonce)
                    }
                    nonce
                },
                {
                    fn acceptSubscriptionOwnerTransfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <acceptSubscriptionOwnerTransferCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::acceptSubscriptionOwnerTransfer)
                    }
                    acceptSubscriptionOwnerTransfer
                },
                {
                    fn withdrawSubscriptionFeesNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <withdrawSubscriptionFeesNativeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::withdrawSubscriptionFeesNative)
                    }
                    withdrawSubscriptionFeesNative
                },
                {
                    fn addConsumer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <addConsumerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::addConsumer)
                    }
                    addConsumer
                },
                {
                    fn getConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::getConfig)
                    }
                    getConfig
                },
                {
                    fn getRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getRequestCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::getRequest)
                    }
                    getRequest
                },
                {
                    fn receiveSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <receiveSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::receiveSignature)
                    }
                    receiveSignature
                },
                {
                    fn getRoleMemberCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getRoleMemberCountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::getRoleMemberCount)
                    }
                    getRoleMemberCount
                },
                {
                    fn removeConsumer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <removeConsumerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::removeConsumer)
                    }
                    removeConsumer
                },
                {
                    fn isInFlight(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <isInFlightCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::isInFlight)
                    }
                    isInFlight
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn requestSubscriptionOwnerTransfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <requestSubscriptionOwnerTransferCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::requestSubscriptionOwnerTransfer)
                    }
                    requestSubscriptionOwnerTransfer
                },
                {
                    fn getSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::getSubscription)
                    }
                    getSubscription
                },
                {
                    fn setSignatureSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <setSignatureSenderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::setSignatureSender)
                    }
                    setSignatureSender
                },
                {
                    fn getAllRequests(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderCalls> {
                        <getAllRequestsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderCalls::getAllRequests)
                    }
                    getAllRequests
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
                Self::ADMIN_ROLE(inner) => {
                    <ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_CONSUMERS(inner) => {
                    <MAX_CONSUMERSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SCHEME_ID(inner) => {
                    <SCHEME_IDCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::acceptSubscriptionOwnerTransfer(inner) => {
                    <acceptSubscriptionOwnerTransferCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addConsumer(inner) => {
                    <addConsumerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateRequestPriceNative(inner) => {
                    <calculateRequestPriceNativeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cancelSubscription(inner) => {
                    <cancelSubscriptionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createSubscription(inner) => {
                    <createSubscriptionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disable(inner) => {
                    <disableCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::enable(inner) => {
                    <enableCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::estimateRequestPriceNative(inner) => {
                    <estimateRequestPriceNativeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::fundSubscriptionWithNative(inner) => {
                    <fundSubscriptionWithNativeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getActiveSubscriptionIds(inner) => {
                    <getActiveSubscriptionIdsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllRequests(inner) => {
                    <getAllRequestsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getConfig(inner) => {
                    <getConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getRequest(inner) => {
                    <getRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRoleMember(inner) => {
                    <getRoleMemberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRoleMemberCount(inner) => {
                    <getRoleMemberCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRoleMembers(inner) => {
                    <getRoleMembersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getSubscription(inner) => {
                    <getSubscriptionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isInFlight(inner) => {
                    <isInFlightCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::messageFrom(inner) => {
                    <messageFromCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::nonce(inner) => {
                    <nonceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::ownerCancelSubscription(inner) => {
                    <ownerCancelSubscriptionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pendingRequestExists(inner) => {
                    <pendingRequestExistsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::receiveSignature(inner) => {
                    <receiveSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeConsumer(inner) => {
                    <removeConsumerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::requestRandomness(inner) => {
                    <requestRandomnessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::requestRandomnessWithSubscription(inner) => {
                    <requestRandomnessWithSubscriptionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::requestSubscriptionOwnerTransfer(inner) => {
                    <requestSubscriptionOwnerTransferCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::s_config(inner) => {
                    <s_configCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::s_configured(inner) => {
                    <s_configuredCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::s_currentSubNonce(inner) => {
                    <s_currentSubNonceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::s_disabled(inner) => {
                    <s_disabledCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::s_totalNativeBalance(inner) => {
                    <s_totalNativeBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::s_withdrawableDirectFundingFeeNative(inner) => {
                    <s_withdrawableDirectFundingFeeNativeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::s_withdrawableSubscriptionFeeNative(inner) => {
                    <s_withdrawableSubscriptionFeeNativeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setConfig(inner) => {
                    <setConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setSignatureSender(inner) => {
                    <setSignatureSenderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::signatureSender(inner) => {
                    <signatureSenderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::withdrawDirectFundingFeesNative(inner) => {
                    <withdrawDirectFundingFeesNativeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawSubscriptionFeesNative(inner) => {
                    <withdrawSubscriptionFeesNativeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ADMIN_ROLE(inner) => {
                    <ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_CONSUMERS(inner) => {
                    <MAX_CONSUMERSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SCHEME_ID(inner) => {
                    <SCHEME_IDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::acceptSubscriptionOwnerTransfer(inner) => {
                    <acceptSubscriptionOwnerTransferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addConsumer(inner) => {
                    <addConsumerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateRequestPriceNative(inner) => {
                    <calculateRequestPriceNativeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cancelSubscription(inner) => {
                    <cancelSubscriptionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createSubscription(inner) => {
                    <createSubscriptionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::disable(inner) => {
                    <disableCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::enable(inner) => {
                    <enableCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::estimateRequestPriceNative(inner) => {
                    <estimateRequestPriceNativeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::fundSubscriptionWithNative(inner) => {
                    <fundSubscriptionWithNativeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getActiveSubscriptionIds(inner) => {
                    <getActiveSubscriptionIdsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllRequests(inner) => {
                    <getAllRequestsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getConfig(inner) => {
                    <getConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRequest(inner) => {
                    <getRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRoleMember(inner) => {
                    <getRoleMemberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRoleMemberCount(inner) => {
                    <getRoleMemberCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRoleMembers(inner) => {
                    <getRoleMembersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getSubscription(inner) => {
                    <getSubscriptionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isInFlight(inner) => {
                    <isInFlightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::messageFrom(inner) => {
                    <messageFromCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nonce(inner) => {
                    <nonceCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::ownerCancelSubscription(inner) => {
                    <ownerCancelSubscriptionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pendingRequestExists(inner) => {
                    <pendingRequestExistsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::receiveSignature(inner) => {
                    <receiveSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeConsumer(inner) => {
                    <removeConsumerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::requestRandomness(inner) => {
                    <requestRandomnessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::requestRandomnessWithSubscription(inner) => {
                    <requestRandomnessWithSubscriptionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::requestSubscriptionOwnerTransfer(inner) => {
                    <requestSubscriptionOwnerTransferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::s_config(inner) => {
                    <s_configCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::s_configured(inner) => {
                    <s_configuredCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::s_currentSubNonce(inner) => {
                    <s_currentSubNonceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::s_disabled(inner) => {
                    <s_disabledCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::s_totalNativeBalance(inner) => {
                    <s_totalNativeBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::s_withdrawableDirectFundingFeeNative(inner) => {
                    <s_withdrawableDirectFundingFeeNativeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::s_withdrawableSubscriptionFeeNative(inner) => {
                    <s_withdrawableSubscriptionFeeNativeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setConfig(inner) => {
                    <setConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setSignatureSender(inner) => {
                    <setSignatureSenderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::signatureSender(inner) => {
                    <signatureSenderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::withdrawDirectFundingFeesNative(inner) => {
                    <withdrawDirectFundingFeesNativeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawSubscriptionFeesNative(inner) => {
                    <withdrawSubscriptionFeesNativeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`RandomnessSender`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum RandomnessSenderErrors {
        #[allow(missing_docs)]
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        #[allow(missing_docs)]
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        BalanceInvariantViolated(BalanceInvariantViolated),
        #[allow(missing_docs)]
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        #[allow(missing_docs)]
        ERC1967NonPayable(ERC1967NonPayable),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        FailedToSendNative(FailedToSendNative),
        #[allow(missing_docs)]
        IndexOutOfRange(IndexOutOfRange),
        #[allow(missing_docs)]
        InsufficientBalance(InsufficientBalance),
        #[allow(missing_docs)]
        InvalidCalldata(InvalidCalldata),
        #[allow(missing_docs)]
        InvalidConsumer(InvalidConsumer),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        InvalidSubscription(InvalidSubscription),
        #[allow(missing_docs)]
        MustBeRequestedOwner(MustBeRequestedOwner),
        #[allow(missing_docs)]
        MustBeSubOwner(MustBeSubOwner),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        PendingRequestExists(PendingRequestExists),
        #[allow(missing_docs)]
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        #[allow(missing_docs)]
        TooManyConsumers(TooManyConsumers),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
    }
    impl RandomnessSenderErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [5u8, 164u8, 142u8, 15u8],
            [19u8, 144u8, 242u8, 161u8],
            [31u8, 106u8, 101u8, 182u8],
            [62u8, 229u8, 174u8, 181u8],
            [76u8, 156u8, 140u8, 227u8],
            [102u8, 151u8, 178u8, 50u8],
            [121u8, 191u8, 212u8, 1u8],
            [129u8, 41u8, 187u8, 205u8],
            [149u8, 11u8, 36u8, 121u8],
            [153u8, 150u8, 179u8, 21u8],
            [169u8, 157u8, 163u8, 2u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [180u8, 47u8, 102u8, 232u8],
            [208u8, 132u8, 233u8, 117u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 230u8, 188u8, 248u8],
            [216u8, 163u8, 251u8, 82u8],
            [224u8, 124u8, 141u8, 186u8],
            [226u8, 81u8, 125u8, 63u8],
            [244u8, 214u8, 120u8, 184u8],
            [249u8, 46u8, 232u8, 169u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(TooManyConsumers),
            ::core::stringify!(IndexOutOfRange),
            ::core::stringify!(InvalidSubscription),
            ::core::stringify!(ReentrancyGuardReentrantCall),
            ::core::stringify!(ERC1967InvalidImplementation),
            ::core::stringify!(AccessControlBadConfirmation),
            ::core::stringify!(InvalidConsumer),
            ::core::stringify!(InvalidCalldata),
            ::core::stringify!(FailedToSendNative),
            ::core::stringify!(AddressEmptyCode),
            ::core::stringify!(BalanceInvariantViolated),
            ::core::stringify!(UUPSUnsupportedProxiableUUID),
            ::core::stringify!(ERC1967NonPayable),
            ::core::stringify!(PendingRequestExists),
            ::core::stringify!(MustBeRequestedOwner),
            ::core::stringify!(FailedCall),
            ::core::stringify!(NotInitializing),
            ::core::stringify!(MustBeSubOwner),
            ::core::stringify!(UUPSUnauthorizedCallContext),
            ::core::stringify!(AccessControlUnauthorizedAccount),
            ::core::stringify!(InsufficientBalance),
            ::core::stringify!(InvalidInitialization),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <TooManyConsumers as alloy_sol_types::SolError>::SIGNATURE,
            <IndexOutOfRange as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidSubscription as alloy_sol_types::SolError>::SIGNATURE,
            <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SIGNATURE,
            <AccessControlBadConfirmation as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidConsumer as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidCalldata as alloy_sol_types::SolError>::SIGNATURE,
            <FailedToSendNative as alloy_sol_types::SolError>::SIGNATURE,
            <AddressEmptyCode as alloy_sol_types::SolError>::SIGNATURE,
            <BalanceInvariantViolated as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967NonPayable as alloy_sol_types::SolError>::SIGNATURE,
            <PendingRequestExists as alloy_sol_types::SolError>::SIGNATURE,
            <MustBeRequestedOwner as alloy_sol_types::SolError>::SIGNATURE,
            <FailedCall as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializing as alloy_sol_types::SolError>::SIGNATURE,
            <MustBeSubOwner as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SIGNATURE,
            <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SIGNATURE,
            <InsufficientBalance as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidInitialization as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for RandomnessSenderErrors {
        const NAME: &'static str = "RandomnessSenderErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 22usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccessControlBadConfirmation(_) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AccessControlUnauthorizedAccount(_) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BalanceInvariantViolated(_) => {
                    <BalanceInvariantViolated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967InvalidImplementation(_) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967NonPayable(_) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedToSendNative(_) => {
                    <FailedToSendNative as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IndexOutOfRange(_) => {
                    <IndexOutOfRange as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientBalance(_) => {
                    <InsufficientBalance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCalldata(_) => {
                    <InvalidCalldata as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidConsumer(_) => {
                    <InvalidConsumer as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSubscription(_) => {
                    <InvalidSubscription as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MustBeRequestedOwner(_) => {
                    <MustBeRequestedOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MustBeSubOwner(_) => {
                    <MustBeSubOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializing(_) => {
                    <NotInitializing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PendingRequestExists(_) => {
                    <PendingRequestExists as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ReentrancyGuardReentrantCall(_) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TooManyConsumers(_) => {
                    <TooManyConsumers as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnauthorizedCallContext(_) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnsupportedProxiableUUID(_) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<RandomnessSenderErrors>] = &[
                {
                    fn TooManyConsumers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <TooManyConsumers as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::TooManyConsumers)
                    }
                    TooManyConsumers
                },
                {
                    fn IndexOutOfRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <IndexOutOfRange as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::IndexOutOfRange)
                    }
                    IndexOutOfRange
                },
                {
                    fn InvalidSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <InvalidSubscription as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::InvalidSubscription)
                    }
                    InvalidSubscription
                },
                {
                    fn ReentrancyGuardReentrantCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::ReentrancyGuardReentrantCall)
                    }
                    ReentrancyGuardReentrantCall
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn InvalidConsumer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <InvalidConsumer as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::InvalidConsumer)
                    }
                    InvalidConsumer
                },
                {
                    fn InvalidCalldata(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <InvalidCalldata as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::InvalidCalldata)
                    }
                    InvalidCalldata
                },
                {
                    fn FailedToSendNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <FailedToSendNative as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::FailedToSendNative)
                    }
                    FailedToSendNative
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn BalanceInvariantViolated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <BalanceInvariantViolated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::BalanceInvariantViolated)
                    }
                    BalanceInvariantViolated
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn PendingRequestExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <PendingRequestExists as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::PendingRequestExists)
                    }
                    PendingRequestExists
                },
                {
                    fn MustBeRequestedOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <MustBeRequestedOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::MustBeRequestedOwner)
                    }
                    MustBeRequestedOwner
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(RandomnessSenderErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn MustBeSubOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <MustBeSubOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::MustBeSubOwner)
                    }
                    MustBeSubOwner
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RandomnessSenderErrors::AccessControlUnauthorizedAccount,
                            )
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn InsufficientBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <InsufficientBalance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::InsufficientBalance)
                    }
                    InsufficientBalance
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(RandomnessSenderErrors::InvalidInitialization)
                    }
                    InvalidInitialization
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
            ) -> alloy_sol_types::Result<RandomnessSenderErrors>] = &[
                {
                    fn TooManyConsumers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <TooManyConsumers as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::TooManyConsumers)
                    }
                    TooManyConsumers
                },
                {
                    fn IndexOutOfRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <IndexOutOfRange as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::IndexOutOfRange)
                    }
                    IndexOutOfRange
                },
                {
                    fn InvalidSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <InvalidSubscription as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::InvalidSubscription)
                    }
                    InvalidSubscription
                },
                {
                    fn ReentrancyGuardReentrantCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::ReentrancyGuardReentrantCall)
                    }
                    ReentrancyGuardReentrantCall
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn InvalidConsumer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <InvalidConsumer as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::InvalidConsumer)
                    }
                    InvalidConsumer
                },
                {
                    fn InvalidCalldata(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <InvalidCalldata as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::InvalidCalldata)
                    }
                    InvalidCalldata
                },
                {
                    fn FailedToSendNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <FailedToSendNative as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::FailedToSendNative)
                    }
                    FailedToSendNative
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn BalanceInvariantViolated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <BalanceInvariantViolated as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::BalanceInvariantViolated)
                    }
                    BalanceInvariantViolated
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn PendingRequestExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <PendingRequestExists as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::PendingRequestExists)
                    }
                    PendingRequestExists
                },
                {
                    fn MustBeRequestedOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <MustBeRequestedOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::MustBeRequestedOwner)
                    }
                    MustBeRequestedOwner
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn MustBeSubOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <MustBeSubOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::MustBeSubOwner)
                    }
                    MustBeSubOwner
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RandomnessSenderErrors::AccessControlUnauthorizedAccount,
                            )
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn InsufficientBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <InsufficientBalance as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::InsufficientBalance)
                    }
                    InsufficientBalance
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RandomnessSenderErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RandomnessSenderErrors::InvalidInitialization)
                    }
                    InvalidInitialization
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
                Self::AccessControlBadConfirmation(inner) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AccessControlUnauthorizedAccount(inner) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BalanceInvariantViolated(inner) => {
                    <BalanceInvariantViolated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC1967InvalidImplementation(inner) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC1967NonPayable(inner) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::FailedToSendNative(inner) => {
                    <FailedToSendNative as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::IndexOutOfRange(inner) => {
                    <IndexOutOfRange as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientBalance(inner) => {
                    <InsufficientBalance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidCalldata(inner) => {
                    <InvalidCalldata as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidConsumer(inner) => {
                    <InvalidConsumer as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSubscription(inner) => {
                    <InvalidSubscription as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MustBeRequestedOwner(inner) => {
                    <MustBeRequestedOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MustBeSubOwner(inner) => {
                    <MustBeSubOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotInitializing(inner) => {
                    <NotInitializing as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PendingRequestExists(inner) => {
                    <PendingRequestExists as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ReentrancyGuardReentrantCall(inner) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TooManyConsumers(inner) => {
                    <TooManyConsumers as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UUPSUnauthorizedCallContext(inner) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UUPSUnsupportedProxiableUUID(inner) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AccessControlBadConfirmation(inner) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AccessControlUnauthorizedAccount(inner) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BalanceInvariantViolated(inner) => {
                    <BalanceInvariantViolated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC1967InvalidImplementation(inner) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC1967NonPayable(inner) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::FailedToSendNative(inner) => {
                    <FailedToSendNative as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IndexOutOfRange(inner) => {
                    <IndexOutOfRange as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientBalance(inner) => {
                    <InsufficientBalance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidCalldata(inner) => {
                    <InvalidCalldata as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidConsumer(inner) => {
                    <InvalidConsumer as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSubscription(inner) => {
                    <InvalidSubscription as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MustBeRequestedOwner(inner) => {
                    <MustBeRequestedOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MustBeSubOwner(inner) => {
                    <MustBeSubOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotInitializing(inner) => {
                    <NotInitializing as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PendingRequestExists(inner) => {
                    <PendingRequestExists as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ReentrancyGuardReentrantCall(inner) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TooManyConsumers(inner) => {
                    <TooManyConsumers as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UUPSUnauthorizedCallContext(inner) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UUPSUnsupportedProxiableUUID(inner) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`RandomnessSender`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum RandomnessSenderEvents {
        #[allow(missing_docs)]
        ConfigSet(ConfigSet),
        #[allow(missing_docs)]
        Disabled(Disabled),
        #[allow(missing_docs)]
        Enabled(Enabled),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        L1GasFee(L1GasFee),
        #[allow(missing_docs)]
        RandomnessCallbackFailed(RandomnessCallbackFailed),
        #[allow(missing_docs)]
        RandomnessCallbackSuccess(RandomnessCallbackSuccess),
        #[allow(missing_docs)]
        RandomnessRequested(RandomnessRequested),
        #[allow(missing_docs)]
        RoleAdminChanged(RoleAdminChanged),
        #[allow(missing_docs)]
        RoleGranted(RoleGranted),
        #[allow(missing_docs)]
        RoleRevoked(RoleRevoked),
        #[allow(missing_docs)]
        SignatureSenderUpdated(SignatureSenderUpdated),
        #[allow(missing_docs)]
        SubscriptionCanceled(SubscriptionCanceled),
        #[allow(missing_docs)]
        SubscriptionConsumerAdded(SubscriptionConsumerAdded),
        #[allow(missing_docs)]
        SubscriptionConsumerRemoved(SubscriptionConsumerRemoved),
        #[allow(missing_docs)]
        SubscriptionCreated(SubscriptionCreated),
        #[allow(missing_docs)]
        SubscriptionFundedWithNative(SubscriptionFundedWithNative),
        #[allow(missing_docs)]
        SubscriptionOwnerTransferRequested(SubscriptionOwnerTransferRequested),
        #[allow(missing_docs)]
        SubscriptionOwnerTransferred(SubscriptionOwnerTransferred),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
    }
    impl RandomnessSenderEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                29u8, 48u8, 21u8, 215u8, 186u8, 133u8, 15u8, 161u8, 152u8, 220u8, 123u8,
                26u8, 63u8, 93u8, 66u8, 119u8, 147u8, 19u8, 166u8, 129u8, 3u8, 95u8,
                119u8, 200u8, 192u8, 55u8, 100u8, 198u8, 16u8, 5u8, 81u8, 141u8,
            ],
            [
                30u8, 152u8, 13u8, 4u8, 170u8, 118u8, 72u8, 226u8, 5u8, 113u8, 62u8,
                94u8, 142u8, 163u8, 128u8, 134u8, 114u8, 172u8, 22u8, 61u8, 16u8, 147u8,
                109u8, 54u8, 249u8, 27u8, 44u8, 136u8, 172u8, 21u8, 117u8, 225u8,
            ],
            [
                33u8, 164u8, 218u8, 209u8, 112u8, 166u8, 191u8, 71u8, 108u8, 49u8, 187u8,
                207u8, 74u8, 22u8, 98u8, 130u8, 149u8, 176u8, 228u8, 80u8, 103u8, 46u8,
                236u8, 37u8, 215u8, 201u8, 51u8, 8u8, 224u8, 83u8, 68u8, 161u8,
            ],
            [
                34u8, 159u8, 108u8, 59u8, 9u8, 93u8, 104u8, 55u8, 85u8, 169u8, 154u8,
                180u8, 88u8, 149u8, 103u8, 71u8, 168u8, 183u8, 6u8, 108u8, 61u8, 212u8,
                41u8, 39u8, 216u8, 80u8, 99u8, 28u8, 52u8, 194u8, 56u8, 241u8,
            ],
            [
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ],
            [
                50u8, 21u8, 140u8, 96u8, 88u8, 52u8, 124u8, 22u8, 1u8, 178u8, 209u8,
                43u8, 198u8, 150u8, 172u8, 105u8, 1u8, 216u8, 169u8, 169u8, 170u8, 59u8,
                161u8, 12u8, 39u8, 171u8, 10u8, 152u8, 62u8, 132u8, 37u8, 167u8,
            ],
            [
                55u8, 132u8, 247u8, 126u8, 142u8, 136u8, 61u8, 233u8, 91u8, 93u8, 71u8,
                205u8, 113u8, 60u8, 237u8, 1u8, 34u8, 159u8, 167u8, 77u8, 17u8, 140u8,
                10u8, 70u8, 34u8, 36u8, 188u8, 176u8, 81u8, 109u8, 67u8, 241u8,
            ],
            [
                85u8, 162u8, 143u8, 222u8, 41u8, 95u8, 72u8, 44u8, 159u8, 50u8, 214u8,
                112u8, 193u8, 22u8, 16u8, 59u8, 202u8, 21u8, 114u8, 75u8, 206u8, 244u8,
                241u8, 139u8, 53u8, 84u8, 46u8, 5u8, 83u8, 195u8, 90u8, 213u8,
            ],
            [
                86u8, 41u8, 111u8, 123u8, 234u8, 224u8, 90u8, 13u8, 184u8, 21u8, 115u8,
                127u8, 219u8, 76u8, 210u8, 152u8, 137u8, 123u8, 30u8, 81u8, 118u8, 20u8,
                214u8, 36u8, 104u8, 8u8, 21u8, 49u8, 174u8, 20u8, 208u8, 153u8,
            ],
            [
                117u8, 136u8, 76u8, 218u8, 220u8, 74u8, 137u8, 232u8, 181u8, 69u8, 219u8,
                128u8, 0u8, 87u8, 240u8, 110u8, 199u8, 245u8, 51u8, 138u8, 8u8, 24u8,
                60u8, 123u8, 165u8, 21u8, 242u8, 191u8, 221u8, 159u8, 225u8, 225u8,
            ],
            [
                118u8, 3u8, 178u8, 5u8, 208u8, 54u8, 81u8, 238u8, 129u8, 47u8, 128u8,
                63u8, 204u8, 222u8, 137u8, 241u8, 1u8, 46u8, 84u8, 90u8, 156u8, 153u8,
                240u8, 171u8, 254u8, 169u8, 206u8, 221u8, 15u8, 216u8, 233u8, 2u8,
            ],
            [
                143u8, 103u8, 71u8, 45u8, 222u8, 33u8, 38u8, 204u8, 208u8, 49u8, 91u8,
                117u8, 220u8, 72u8, 42u8, 90u8, 115u8, 172u8, 178u8, 40u8, 163u8, 149u8,
                85u8, 63u8, 138u8, 230u8, 237u8, 222u8, 90u8, 12u8, 164u8, 250u8,
            ],
            [
                183u8, 75u8, 50u8, 4u8, 165u8, 56u8, 205u8, 128u8, 33u8, 102u8, 45u8,
                66u8, 231u8, 148u8, 104u8, 29u8, 220u8, 51u8, 153u8, 36u8, 239u8, 103u8,
                91u8, 143u8, 209u8, 30u8, 158u8, 175u8, 106u8, 161u8, 158u8, 181u8,
            ],
            [
                188u8, 124u8, 215u8, 90u8, 32u8, 238u8, 39u8, 253u8, 154u8, 222u8, 186u8,
                179u8, 32u8, 65u8, 247u8, 85u8, 33u8, 77u8, 188u8, 107u8, 255u8, 169u8,
                12u8, 192u8, 34u8, 91u8, 57u8, 218u8, 46u8, 92u8, 45u8, 59u8,
            ],
            [
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8,
                81u8, 66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8,
                71u8, 92u8, 58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
            ],
            [
                192u8, 249u8, 97u8, 5u8, 31u8, 151u8, 176u8, 76u8, 73u8, 100u8, 114u8,
                209u8, 28u8, 182u8, 23u8, 13u8, 132u8, 78u8, 75u8, 44u8, 157u8, 253u8,
                59u8, 96u8, 42u8, 79u8, 160u8, 19u8, 151u8, 18u8, 212u8, 132u8,
            ],
            [
                199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8,
                19u8, 244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8,
                33u8, 238u8, 209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
            ],
            [
                212u8, 17u8, 74u8, 182u8, 233u8, 175u8, 159u8, 89u8, 124u8, 82u8, 4u8,
                31u8, 50u8, 214u8, 45u8, 197u8, 124u8, 92u8, 78u8, 76u8, 13u8, 68u8,
                39u8, 0u8, 96u8, 105u8, 99u8, 94u8, 33u8, 108u8, 147u8, 134u8,
            ],
            [
                238u8, 231u8, 25u8, 91u8, 108u8, 238u8, 15u8, 167u8, 4u8, 76u8, 58u8,
                240u8, 184u8, 111u8, 226u8, 254u8, 187u8, 29u8, 39u8, 3u8, 215u8, 17u8,
                145u8, 244u8, 64u8, 82u8, 186u8, 13u8, 96u8, 255u8, 218u8, 100u8,
            ],
            [
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(SubscriptionCreated),
            ::core::stringify!(SubscriptionConsumerAdded),
            ::core::stringify!(SubscriptionOwnerTransferRequested),
            ::core::stringify!(SignatureSenderUpdated),
            ::core::stringify!(RoleGranted),
            ::core::stringify!(SubscriptionConsumerRemoved),
            ::core::stringify!(SubscriptionCanceled),
            ::core::stringify!(ConfigSet),
            ::core::stringify!(L1GasFee),
            ::core::stringify!(Disabled),
            ::core::stringify!(SubscriptionFundedWithNative),
            ::core::stringify!(RandomnessCallbackFailed),
            ::core::stringify!(RandomnessCallbackSuccess),
            ::core::stringify!(Upgraded),
            ::core::stringify!(RoleAdminChanged),
            ::core::stringify!(Enabled),
            ::core::stringify!(Initialized),
            ::core::stringify!(SubscriptionOwnerTransferred),
            ::core::stringify!(RandomnessRequested),
            ::core::stringify!(RoleRevoked),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <SubscriptionCreated as alloy_sol_types::SolEvent>::SIGNATURE,
            <SubscriptionConsumerAdded as alloy_sol_types::SolEvent>::SIGNATURE,
            <SubscriptionOwnerTransferRequested as alloy_sol_types::SolEvent>::SIGNATURE,
            <SignatureSenderUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE,
            <SubscriptionConsumerRemoved as alloy_sol_types::SolEvent>::SIGNATURE,
            <SubscriptionCanceled as alloy_sol_types::SolEvent>::SIGNATURE,
            <ConfigSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <L1GasFee as alloy_sol_types::SolEvent>::SIGNATURE,
            <Disabled as alloy_sol_types::SolEvent>::SIGNATURE,
            <SubscriptionFundedWithNative as alloy_sol_types::SolEvent>::SIGNATURE,
            <RandomnessCallbackFailed as alloy_sol_types::SolEvent>::SIGNATURE,
            <RandomnessCallbackSuccess as alloy_sol_types::SolEvent>::SIGNATURE,
            <Upgraded as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE,
            <Enabled as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
            <SubscriptionOwnerTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <RandomnessRequested as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
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
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for RandomnessSenderEvents {
        const NAME: &'static str = "RandomnessSenderEvents";
        const COUNT: usize = 20usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ConfigSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ConfigSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ConfigSet)
                }
                Some(<Disabled as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Disabled as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Disabled)
                }
                Some(<Enabled as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Enabled as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Enabled)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(<L1GasFee as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <L1GasFee as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::L1GasFee)
                }
                Some(
                    <RandomnessCallbackFailed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RandomnessCallbackFailed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RandomnessCallbackFailed)
                }
                Some(
                    <RandomnessCallbackSuccess as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RandomnessCallbackSuccess as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RandomnessCallbackSuccess)
                }
                Some(
                    <RandomnessRequested as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RandomnessRequested as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RandomnessRequested)
                }
                Some(<RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleAdminChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleAdminChanged)
                }
                Some(<RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleGranted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleGranted)
                }
                Some(<RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleRevoked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleRevoked)
                }
                Some(
                    <SignatureSenderUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SignatureSenderUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SignatureSenderUpdated)
                }
                Some(
                    <SubscriptionCanceled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SubscriptionCanceled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SubscriptionCanceled)
                }
                Some(
                    <SubscriptionConsumerAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SubscriptionConsumerAdded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SubscriptionConsumerAdded)
                }
                Some(
                    <SubscriptionConsumerRemoved as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SubscriptionConsumerRemoved as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SubscriptionConsumerRemoved)
                }
                Some(
                    <SubscriptionCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SubscriptionCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SubscriptionCreated)
                }
                Some(
                    <SubscriptionFundedWithNative as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SubscriptionFundedWithNative as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SubscriptionFundedWithNative)
                }
                Some(
                    <SubscriptionOwnerTransferRequested as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SubscriptionOwnerTransferRequested as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SubscriptionOwnerTransferRequested)
                }
                Some(
                    <SubscriptionOwnerTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SubscriptionOwnerTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SubscriptionOwnerTransferred)
                }
                Some(<Upgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgraded as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Upgraded)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for RandomnessSenderEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ConfigSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Disabled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Enabled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::L1GasFee(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RandomnessCallbackFailed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RandomnessCallbackSuccess(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RandomnessRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SignatureSenderUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SubscriptionCanceled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SubscriptionConsumerAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SubscriptionConsumerRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SubscriptionCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SubscriptionFundedWithNative(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SubscriptionOwnerTransferRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SubscriptionOwnerTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ConfigSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Disabled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Enabled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::L1GasFee(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RandomnessCallbackFailed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RandomnessCallbackSuccess(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RandomnessRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SignatureSenderUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SubscriptionCanceled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SubscriptionConsumerAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SubscriptionConsumerRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SubscriptionCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SubscriptionFundedWithNative(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SubscriptionOwnerTransferRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SubscriptionOwnerTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`RandomnessSender`](self) contract instance.

See the [wrapper's documentation](`RandomnessSenderInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> RandomnessSenderInstance<P, N> {
        RandomnessSenderInstance::<P, N>::new(address, __provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<RandomnessSenderInstance<P, N>>,
    > {
        RandomnessSenderInstance::<P, N>::deploy(__provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        RandomnessSenderInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`RandomnessSender`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`RandomnessSender`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct RandomnessSenderInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for RandomnessSenderInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("RandomnessSenderInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > RandomnessSenderInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`RandomnessSender`](self) contract instance.

See the [wrapper's documentation](`RandomnessSenderInstance`) for more details.*/
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
        ) -> alloy_contract::Result<RandomnessSenderInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                ::core::clone::Clone::clone(&BYTECODE),
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
    impl<P: ::core::clone::Clone, N> RandomnessSenderInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> RandomnessSenderInstance<P, N> {
            RandomnessSenderInstance {
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
    > RandomnessSenderInstance<P, N> {
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
        ///Creates a new call builder for the [`ADMIN_ROLE`] function.
        pub fn ADMIN_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, ADMIN_ROLECall, N> {
            self.call_builder(&ADMIN_ROLECall)
        }
        ///Creates a new call builder for the [`DEFAULT_ADMIN_ROLE`] function.
        pub fn DEFAULT_ADMIN_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, DEFAULT_ADMIN_ROLECall, N> {
            self.call_builder(&DEFAULT_ADMIN_ROLECall)
        }
        ///Creates a new call builder for the [`MAX_CONSUMERS`] function.
        pub fn MAX_CONSUMERS(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, MAX_CONSUMERSCall, N> {
            self.call_builder(&MAX_CONSUMERSCall)
        }
        ///Creates a new call builder for the [`SCHEME_ID`] function.
        pub fn SCHEME_ID(&self) -> alloy_contract::SolCallBuilder<&P, SCHEME_IDCall, N> {
            self.call_builder(&SCHEME_IDCall)
        }
        ///Creates a new call builder for the [`UPGRADE_INTERFACE_VERSION`] function.
        pub fn UPGRADE_INTERFACE_VERSION(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, UPGRADE_INTERFACE_VERSIONCall, N> {
            self.call_builder(&UPGRADE_INTERFACE_VERSIONCall)
        }
        ///Creates a new call builder for the [`acceptSubscriptionOwnerTransfer`] function.
        pub fn acceptSubscriptionOwnerTransfer(
            &self,
            subId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, acceptSubscriptionOwnerTransferCall, N> {
            self.call_builder(
                &acceptSubscriptionOwnerTransferCall {
                    subId,
                },
            )
        }
        ///Creates a new call builder for the [`addConsumer`] function.
        pub fn addConsumer(
            &self,
            subId: alloy::sol_types::private::primitives::aliases::U256,
            consumer: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, addConsumerCall, N> {
            self.call_builder(&addConsumerCall { subId, consumer })
        }
        ///Creates a new call builder for the [`calculateRequestPriceNative`] function.
        pub fn calculateRequestPriceNative(
            &self,
            _callbackGasLimit: u32,
        ) -> alloy_contract::SolCallBuilder<&P, calculateRequestPriceNativeCall, N> {
            self.call_builder(
                &calculateRequestPriceNativeCall {
                    _callbackGasLimit,
                },
            )
        }
        ///Creates a new call builder for the [`cancelSubscription`] function.
        pub fn cancelSubscription(
            &self,
            subId: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, cancelSubscriptionCall, N> {
            self.call_builder(
                &cancelSubscriptionCall {
                    subId,
                    to,
                },
            )
        }
        ///Creates a new call builder for the [`createSubscription`] function.
        pub fn createSubscription(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, createSubscriptionCall, N> {
            self.call_builder(&createSubscriptionCall)
        }
        ///Creates a new call builder for the [`disable`] function.
        pub fn disable(&self) -> alloy_contract::SolCallBuilder<&P, disableCall, N> {
            self.call_builder(&disableCall)
        }
        ///Creates a new call builder for the [`enable`] function.
        pub fn enable(&self) -> alloy_contract::SolCallBuilder<&P, enableCall, N> {
            self.call_builder(&enableCall)
        }
        ///Creates a new call builder for the [`estimateRequestPriceNative`] function.
        pub fn estimateRequestPriceNative(
            &self,
            _callbackGasLimit: u32,
            _requestGasPriceWei: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, estimateRequestPriceNativeCall, N> {
            self.call_builder(
                &estimateRequestPriceNativeCall {
                    _callbackGasLimit,
                    _requestGasPriceWei,
                },
            )
        }
        ///Creates a new call builder for the [`fundSubscriptionWithNative`] function.
        pub fn fundSubscriptionWithNative(
            &self,
            subId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, fundSubscriptionWithNativeCall, N> {
            self.call_builder(
                &fundSubscriptionWithNativeCall {
                    subId,
                },
            )
        }
        ///Creates a new call builder for the [`getActiveSubscriptionIds`] function.
        pub fn getActiveSubscriptionIds(
            &self,
            startIndex: alloy::sol_types::private::primitives::aliases::U256,
            maxCount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getActiveSubscriptionIdsCall, N> {
            self.call_builder(
                &getActiveSubscriptionIdsCall {
                    startIndex,
                    maxCount,
                },
            )
        }
        ///Creates a new call builder for the [`getAllRequests`] function.
        pub fn getAllRequests(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getAllRequestsCall, N> {
            self.call_builder(&getAllRequestsCall)
        }
        ///Creates a new call builder for the [`getConfig`] function.
        pub fn getConfig(&self) -> alloy_contract::SolCallBuilder<&P, getConfigCall, N> {
            self.call_builder(&getConfigCall)
        }
        ///Creates a new call builder for the [`getRequest`] function.
        pub fn getRequest(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getRequestCall, N> {
            self.call_builder(&getRequestCall { requestId })
        }
        ///Creates a new call builder for the [`getRoleAdmin`] function.
        pub fn getRoleAdmin(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getRoleAdminCall, N> {
            self.call_builder(&getRoleAdminCall { role })
        }
        ///Creates a new call builder for the [`getRoleMember`] function.
        pub fn getRoleMember(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getRoleMemberCall, N> {
            self.call_builder(&getRoleMemberCall { role, index })
        }
        ///Creates a new call builder for the [`getRoleMemberCount`] function.
        pub fn getRoleMemberCount(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getRoleMemberCountCall, N> {
            self.call_builder(&getRoleMemberCountCall { role })
        }
        ///Creates a new call builder for the [`getRoleMembers`] function.
        pub fn getRoleMembers(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getRoleMembersCall, N> {
            self.call_builder(&getRoleMembersCall { role })
        }
        ///Creates a new call builder for the [`getSubscription`] function.
        pub fn getSubscription(
            &self,
            subId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getSubscriptionCall, N> {
            self.call_builder(&getSubscriptionCall { subId })
        }
        ///Creates a new call builder for the [`grantRole`] function.
        pub fn grantRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, grantRoleCall, N> {
            self.call_builder(&grantRoleCall { role, account })
        }
        ///Creates a new call builder for the [`hasRole`] function.
        pub fn hasRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, hasRoleCall, N> {
            self.call_builder(&hasRoleCall { role, account })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _signatureSender: alloy::sol_types::private::Address,
            owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    _signatureSender,
                    owner,
                },
            )
        }
        ///Creates a new call builder for the [`isInFlight`] function.
        pub fn isInFlight(
            &self,
            requestID: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isInFlightCall, N> {
            self.call_builder(&isInFlightCall { requestID })
        }
        ///Creates a new call builder for the [`messageFrom`] function.
        pub fn messageFrom(
            &self,
            r: <TypesLib::RandomnessRequestCreationParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, messageFromCall, N> {
            self.call_builder(&messageFromCall { r })
        }
        ///Creates a new call builder for the [`nonce`] function.
        pub fn nonce(&self) -> alloy_contract::SolCallBuilder<&P, nonceCall, N> {
            self.call_builder(&nonceCall)
        }
        ///Creates a new call builder for the [`ownerCancelSubscription`] function.
        pub fn ownerCancelSubscription(
            &self,
            subId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, ownerCancelSubscriptionCall, N> {
            self.call_builder(
                &ownerCancelSubscriptionCall {
                    subId,
                },
            )
        }
        ///Creates a new call builder for the [`pendingRequestExists`] function.
        pub fn pendingRequestExists(
            &self,
            subId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, pendingRequestExistsCall, N> {
            self.call_builder(&pendingRequestExistsCall { subId })
        }
        ///Creates a new call builder for the [`proxiableUUID`] function.
        pub fn proxiableUUID(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, proxiableUUIDCall, N> {
            self.call_builder(&proxiableUUIDCall)
        }
        ///Creates a new call builder for the [`receiveSignature`] function.
        pub fn receiveSignature(
            &self,
            requestID: alloy::sol_types::private::primitives::aliases::U256,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, receiveSignatureCall, N> {
            self.call_builder(
                &receiveSignatureCall {
                    requestID,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`removeConsumer`] function.
        pub fn removeConsumer(
            &self,
            subId: alloy::sol_types::private::primitives::aliases::U256,
            consumer: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, removeConsumerCall, N> {
            self.call_builder(
                &removeConsumerCall {
                    subId,
                    consumer,
                },
            )
        }
        ///Creates a new call builder for the [`renounceRole`] function.
        pub fn renounceRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            callerConfirmation: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, renounceRoleCall, N> {
            self.call_builder(
                &renounceRoleCall {
                    role,
                    callerConfirmation,
                },
            )
        }
        ///Creates a new call builder for the [`requestRandomness`] function.
        pub fn requestRandomness(
            &self,
            callbackGasLimit: u32,
        ) -> alloy_contract::SolCallBuilder<&P, requestRandomnessCall, N> {
            self.call_builder(
                &requestRandomnessCall {
                    callbackGasLimit,
                },
            )
        }
        ///Creates a new call builder for the [`requestRandomnessWithSubscription`] function.
        pub fn requestRandomnessWithSubscription(
            &self,
            callbackGasLimit: u32,
            subId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            requestRandomnessWithSubscriptionCall,
            N,
        > {
            self.call_builder(
                &requestRandomnessWithSubscriptionCall {
                    callbackGasLimit,
                    subId,
                },
            )
        }
        ///Creates a new call builder for the [`requestSubscriptionOwnerTransfer`] function.
        pub fn requestSubscriptionOwnerTransfer(
            &self,
            subId: alloy::sol_types::private::primitives::aliases::U256,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            requestSubscriptionOwnerTransferCall,
            N,
        > {
            self.call_builder(
                &requestSubscriptionOwnerTransferCall {
                    subId,
                    newOwner,
                },
            )
        }
        ///Creates a new call builder for the [`revokeRole`] function.
        pub fn revokeRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, revokeRoleCall, N> {
            self.call_builder(&revokeRoleCall { role, account })
        }
        ///Creates a new call builder for the [`s_config`] function.
        pub fn s_config(&self) -> alloy_contract::SolCallBuilder<&P, s_configCall, N> {
            self.call_builder(&s_configCall)
        }
        ///Creates a new call builder for the [`s_configured`] function.
        pub fn s_configured(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, s_configuredCall, N> {
            self.call_builder(&s_configuredCall)
        }
        ///Creates a new call builder for the [`s_currentSubNonce`] function.
        pub fn s_currentSubNonce(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, s_currentSubNonceCall, N> {
            self.call_builder(&s_currentSubNonceCall)
        }
        ///Creates a new call builder for the [`s_disabled`] function.
        pub fn s_disabled(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, s_disabledCall, N> {
            self.call_builder(&s_disabledCall)
        }
        ///Creates a new call builder for the [`s_totalNativeBalance`] function.
        pub fn s_totalNativeBalance(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, s_totalNativeBalanceCall, N> {
            self.call_builder(&s_totalNativeBalanceCall)
        }
        ///Creates a new call builder for the [`s_withdrawableDirectFundingFeeNative`] function.
        pub fn s_withdrawableDirectFundingFeeNative(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            s_withdrawableDirectFundingFeeNativeCall,
            N,
        > {
            self.call_builder(&s_withdrawableDirectFundingFeeNativeCall)
        }
        ///Creates a new call builder for the [`s_withdrawableSubscriptionFeeNative`] function.
        pub fn s_withdrawableSubscriptionFeeNative(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            s_withdrawableSubscriptionFeeNativeCall,
            N,
        > {
            self.call_builder(&s_withdrawableSubscriptionFeeNativeCall)
        }
        ///Creates a new call builder for the [`setConfig`] function.
        pub fn setConfig(
            &self,
            maxGasLimit: u32,
            gasAfterPaymentCalculation: u32,
            fulfillmentFlatFeeNativePPM: u32,
            weiPerUnitGas: u32,
            blsPairingCheckOverhead: u32,
            nativePremiumPercentage: u8,
            gasForCallExactCheck: u32,
        ) -> alloy_contract::SolCallBuilder<&P, setConfigCall, N> {
            self.call_builder(
                &setConfigCall {
                    maxGasLimit,
                    gasAfterPaymentCalculation,
                    fulfillmentFlatFeeNativePPM,
                    weiPerUnitGas,
                    blsPairingCheckOverhead,
                    nativePremiumPercentage,
                    gasForCallExactCheck,
                },
            )
        }
        ///Creates a new call builder for the [`setSignatureSender`] function.
        pub fn setSignatureSender(
            &self,
            newSignatureSender: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, setSignatureSenderCall, N> {
            self.call_builder(
                &setSignatureSenderCall {
                    newSignatureSender,
                },
            )
        }
        ///Creates a new call builder for the [`signatureSender`] function.
        pub fn signatureSender(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, signatureSenderCall, N> {
            self.call_builder(&signatureSenderCall)
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<&P, supportsInterfaceCall, N> {
            self.call_builder(
                &supportsInterfaceCall {
                    interfaceId,
                },
            )
        }
        ///Creates a new call builder for the [`upgradeToAndCall`] function.
        pub fn upgradeToAndCall(
            &self,
            newImplementation: alloy::sol_types::private::Address,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, upgradeToAndCallCall, N> {
            self.call_builder(
                &upgradeToAndCallCall {
                    newImplementation,
                    data,
                },
            )
        }
        ///Creates a new call builder for the [`version`] function.
        pub fn version(&self) -> alloy_contract::SolCallBuilder<&P, versionCall, N> {
            self.call_builder(&versionCall)
        }
        ///Creates a new call builder for the [`withdrawDirectFundingFeesNative`] function.
        pub fn withdrawDirectFundingFeesNative(
            &self,
            recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, withdrawDirectFundingFeesNativeCall, N> {
            self.call_builder(
                &withdrawDirectFundingFeesNativeCall {
                    recipient,
                },
            )
        }
        ///Creates a new call builder for the [`withdrawSubscriptionFeesNative`] function.
        pub fn withdrawSubscriptionFeesNative(
            &self,
            recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, withdrawSubscriptionFeesNativeCall, N> {
            self.call_builder(
                &withdrawSubscriptionFeesNativeCall {
                    recipient,
                },
            )
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > RandomnessSenderInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ConfigSet`] event.
        pub fn ConfigSet_filter(&self) -> alloy_contract::Event<&P, ConfigSet, N> {
            self.event_filter::<ConfigSet>()
        }
        ///Creates a new event filter for the [`Disabled`] event.
        pub fn Disabled_filter(&self) -> alloy_contract::Event<&P, Disabled, N> {
            self.event_filter::<Disabled>()
        }
        ///Creates a new event filter for the [`Enabled`] event.
        pub fn Enabled_filter(&self) -> alloy_contract::Event<&P, Enabled, N> {
            self.event_filter::<Enabled>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`L1GasFee`] event.
        pub fn L1GasFee_filter(&self) -> alloy_contract::Event<&P, L1GasFee, N> {
            self.event_filter::<L1GasFee>()
        }
        ///Creates a new event filter for the [`RandomnessCallbackFailed`] event.
        pub fn RandomnessCallbackFailed_filter(
            &self,
        ) -> alloy_contract::Event<&P, RandomnessCallbackFailed, N> {
            self.event_filter::<RandomnessCallbackFailed>()
        }
        ///Creates a new event filter for the [`RandomnessCallbackSuccess`] event.
        pub fn RandomnessCallbackSuccess_filter(
            &self,
        ) -> alloy_contract::Event<&P, RandomnessCallbackSuccess, N> {
            self.event_filter::<RandomnessCallbackSuccess>()
        }
        ///Creates a new event filter for the [`RandomnessRequested`] event.
        pub fn RandomnessRequested_filter(
            &self,
        ) -> alloy_contract::Event<&P, RandomnessRequested, N> {
            self.event_filter::<RandomnessRequested>()
        }
        ///Creates a new event filter for the [`RoleAdminChanged`] event.
        pub fn RoleAdminChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, RoleAdminChanged, N> {
            self.event_filter::<RoleAdminChanged>()
        }
        ///Creates a new event filter for the [`RoleGranted`] event.
        pub fn RoleGranted_filter(&self) -> alloy_contract::Event<&P, RoleGranted, N> {
            self.event_filter::<RoleGranted>()
        }
        ///Creates a new event filter for the [`RoleRevoked`] event.
        pub fn RoleRevoked_filter(&self) -> alloy_contract::Event<&P, RoleRevoked, N> {
            self.event_filter::<RoleRevoked>()
        }
        ///Creates a new event filter for the [`SignatureSenderUpdated`] event.
        pub fn SignatureSenderUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, SignatureSenderUpdated, N> {
            self.event_filter::<SignatureSenderUpdated>()
        }
        ///Creates a new event filter for the [`SubscriptionCanceled`] event.
        pub fn SubscriptionCanceled_filter(
            &self,
        ) -> alloy_contract::Event<&P, SubscriptionCanceled, N> {
            self.event_filter::<SubscriptionCanceled>()
        }
        ///Creates a new event filter for the [`SubscriptionConsumerAdded`] event.
        pub fn SubscriptionConsumerAdded_filter(
            &self,
        ) -> alloy_contract::Event<&P, SubscriptionConsumerAdded, N> {
            self.event_filter::<SubscriptionConsumerAdded>()
        }
        ///Creates a new event filter for the [`SubscriptionConsumerRemoved`] event.
        pub fn SubscriptionConsumerRemoved_filter(
            &self,
        ) -> alloy_contract::Event<&P, SubscriptionConsumerRemoved, N> {
            self.event_filter::<SubscriptionConsumerRemoved>()
        }
        ///Creates a new event filter for the [`SubscriptionCreated`] event.
        pub fn SubscriptionCreated_filter(
            &self,
        ) -> alloy_contract::Event<&P, SubscriptionCreated, N> {
            self.event_filter::<SubscriptionCreated>()
        }
        ///Creates a new event filter for the [`SubscriptionFundedWithNative`] event.
        pub fn SubscriptionFundedWithNative_filter(
            &self,
        ) -> alloy_contract::Event<&P, SubscriptionFundedWithNative, N> {
            self.event_filter::<SubscriptionFundedWithNative>()
        }
        ///Creates a new event filter for the [`SubscriptionOwnerTransferRequested`] event.
        pub fn SubscriptionOwnerTransferRequested_filter(
            &self,
        ) -> alloy_contract::Event<&P, SubscriptionOwnerTransferRequested, N> {
            self.event_filter::<SubscriptionOwnerTransferRequested>()
        }
        ///Creates a new event filter for the [`SubscriptionOwnerTransferred`] event.
        pub fn SubscriptionOwnerTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, SubscriptionOwnerTransferred, N> {
            self.event_filter::<SubscriptionOwnerTransferred>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
