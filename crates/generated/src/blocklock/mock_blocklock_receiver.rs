///Module containing a contract's types and functions.
/**

```solidity
library BLS {
    struct PointG2 { uint256[2] x; uint256[2] y; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BLS {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct PointG2 { uint256[2] x; uint256[2] y; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PointG2 {
        #[allow(missing_docs)]
        pub x: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
        #[allow(missing_docs)]
        pub y: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
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
            alloy::sol_types::sol_data::FixedArray<
                alloy::sol_types::sol_data::Uint<256>,
                2usize,
            >,
            alloy::sol_types::sol_data::FixedArray<
                alloy::sol_types::sol_data::Uint<256>,
                2usize,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
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
        impl ::core::convert::From<PointG2> for UnderlyingRustTuple<'_> {
            fn from(value: PointG2) -> Self {
                (value.x, value.y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PointG2 {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { x: tuple.0, y: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PointG2 {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PointG2 {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.x),
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.y),
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
        impl alloy_sol_types::SolType for PointG2 {
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
        impl alloy_sol_types::SolStruct for PointG2 {
            const NAME: &'static str = "PointG2";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PointG2(uint256[2] x,uint256[2] y)",
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
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.x)
                        .0,
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.y)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PointG2 {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.x)
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.y)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.x, out);
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.y, out);
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
    /**Creates a new wrapper around an on-chain [`BLS`](self) contract instance.

See the [wrapper's documentation](`BLSInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(address: alloy_sol_types::private::Address, __provider: P) -> BLSInstance<P, N> {
        BLSInstance::<P, N>::new(address, __provider)
    }
    /**A [`BLS`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BLS`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BLSInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for BLSInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BLSInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > BLSInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`BLS`](self) contract instance.

See the [wrapper's documentation](`BLSInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> BLSInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BLSInstance<P, N> {
            BLSInstance {
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
    > BLSInstance<P, N> {
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
    > BLSInstance<P, N> {
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
///Module containing a contract's types and functions.
/**

```solidity
library TypesLib {
    struct Ciphertext { BLS.PointG2 u; bytes v; bytes w; }
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
struct Ciphertext { BLS.PointG2 u; bytes v; bytes w; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Ciphertext {
        #[allow(missing_docs)]
        pub u: <BLS::PointG2 as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub v: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub w: alloy::sol_types::private::Bytes,
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
            BLS::PointG2,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <BLS::PointG2 as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<Ciphertext> for UnderlyingRustTuple<'_> {
            fn from(value: Ciphertext) -> Self {
                (value.u, value.v, value.w)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Ciphertext {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    u: tuple.0,
                    v: tuple.1,
                    w: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Ciphertext {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Ciphertext {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <BLS::PointG2 as alloy_sol_types::SolType>::tokenize(&self.u),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.v,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.w,
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
        impl alloy_sol_types::SolType for Ciphertext {
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
        impl alloy_sol_types::SolStruct for Ciphertext {
            const NAME: &'static str = "Ciphertext";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Ciphertext(PointG2 u,bytes v,bytes w)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <BLS::PointG2 as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BLS::PointG2 as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <BLS::PointG2 as alloy_sol_types::SolType>::eip712_data_word(&self.u)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.v,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.w,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Ciphertext {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <BLS::PointG2 as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.u,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.v,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.w,
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
                <BLS::PointG2 as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.u,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.v,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.w,
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
library BLS {
    struct PointG2 {
        uint256[2] x;
        uint256[2] y;
    }
}

library TypesLib {
    struct Ciphertext {
        BLS.PointG2 u;
        bytes v;
        bytes w;
    }
}

interface MockBlocklockReceiver {
    event Funded(address indexed sender, uint256 amount);
    event NewSubscriptionId(uint256 indexed subscriptionId);
    event OwnershipTransferRequested(address indexed from, address indexed to);
    event OwnershipTransferred(address indexed from, address indexed to);
    event Received(address, uint256);
    event Withdrawn(address indexed recipient, uint256 amount);

    constructor(address blocklockContract);

    receive() external payable;

    function acceptOwnership() external;
    function blocklock() external view returns (address);
    function cancelSubscription(address to) external;
    function createSubscriptionAndFundNative() external payable;
    function createTimelockRequestWithDirectFunding(uint32 callbackGasLimit, bytes memory condition, TypesLib.Ciphertext memory encryptedData) external payable returns (uint256, uint256);
    function createTimelockRequestWithSubscription(uint32 callbackGasLimit, bytes memory condition, TypesLib.Ciphertext memory encryptedData) external returns (uint256);
    function encryptedValue() external view returns (BLS.PointG2 memory u, bytes memory v, bytes memory w);
    function fundContractNative() external payable;
    function getBalance() external view returns (uint256);
    function isInFlight(uint256 requestId) external view returns (bool);
    function owner() external view returns (address);
    function pendingRequestExists(uint256 subId) external view returns (bool);
    function plainTextValue() external view returns (uint256);
    function receiveBlocklock(uint256 requestId, bytes memory decryptionKey) external;
    function requestId() external view returns (uint256);
    function setBlocklock(address _blocklock) external;
    function setSubId(uint256 subId) external;
    function subscriptionId() external view returns (uint256);
    function topUpSubscriptionNative() external payable;
    function transferOwnership(address to) external;
    function updateSubscription(address[] memory consumers) external;
    function withdrawNative(uint256 amount, address recipient) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "blocklockContract",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "receive",
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "acceptOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "blocklock",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBlocklockSender"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "cancelSubscription",
    "inputs": [
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
    "name": "createSubscriptionAndFundNative",
    "inputs": [],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "createTimelockRequestWithDirectFunding",
    "inputs": [
      {
        "name": "callbackGasLimit",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "condition",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "encryptedData",
        "type": "tuple",
        "internalType": "struct TypesLib.Ciphertext",
        "components": [
          {
            "name": "u",
            "type": "tuple",
            "internalType": "struct BLS.PointG2",
            "components": [
              {
                "name": "x",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              },
              {
                "name": "y",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              }
            ]
          },
          {
            "name": "v",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "w",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "createTimelockRequestWithSubscription",
    "inputs": [
      {
        "name": "callbackGasLimit",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "condition",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "encryptedData",
        "type": "tuple",
        "internalType": "struct TypesLib.Ciphertext",
        "components": [
          {
            "name": "u",
            "type": "tuple",
            "internalType": "struct BLS.PointG2",
            "components": [
              {
                "name": "x",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              },
              {
                "name": "y",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              }
            ]
          },
          {
            "name": "v",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "w",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "encryptedValue",
    "inputs": [],
    "outputs": [
      {
        "name": "u",
        "type": "tuple",
        "internalType": "struct BLS.PointG2",
        "components": [
          {
            "name": "x",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          },
          {
            "name": "y",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          }
        ]
      },
      {
        "name": "v",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "w",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "fundContractNative",
    "inputs": [],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "getBalance",
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
    "name": "isInFlight",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
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
    "name": "plainTextValue",
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
    "name": "receiveBlocklock",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "decryptionKey",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "requestId",
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
    "name": "setBlocklock",
    "inputs": [
      {
        "name": "_blocklock",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setSubId",
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
    "name": "subscriptionId",
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
    "name": "topUpSubscriptionNative",
    "inputs": [],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
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
    "name": "updateSubscription",
    "inputs": [
      {
        "name": "consumers",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawNative",
    "inputs": [
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "Funded",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NewSubscriptionId",
    "inputs": [
      {
        "name": "subscriptionId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferRequested",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Received",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Withdrawn",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
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
pub mod MockBlocklockReceiver {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50604051611c1e380380611c1e83398101604081905261002e91610186565b8033805f816100845760405162461bcd60e51b815260206004820152601860248201527f43616e6e6f7420736574206f776e657220746f207a65726f000000000000000060448201526064015b60405180910390fd5b5f80546001600160a01b0319166001600160a01b03848116919091179091558116156100b3576100b3816100de565b5050600280546001600160a01b0319166001600160a01b039390931692909217909155506101b39050565b336001600160a01b038216036101365760405162461bcd60e51b815260206004820152601760248201527f43616e6e6f74207472616e7366657220746f2073656c66000000000000000000604482015260640161007b565b600180546001600160a01b0319166001600160a01b038381169182179092555f8054604051929316917fed8889f560326eb138920d842192f0eb3dd22b4f139c87a2c57538e05bae12789190a350565b5f60208284031215610196575f5ffd5b81516001600160a01b03811681146101ac575f5ffd5b9392505050565b611a5e806101c05f395ff3fe608060405260043610610133575f3560e01c806380980043116100a857806397a9c2851161006d57806397a9c28514610366578063b8ca8dd81461037b578063b96dbba71461039a578063cd802c91146103a2578063e64a66ea146103c1578063f2fde38b146103c9575f5ffd5b806380980043146102b657806386e560be146102d55780638da5cb5b146102f857806393b9740a1461032857806393d81d5814610347575f5ffd5b806341af6c87116100f957806341af6c87146101ee5780634d3de3531461021d5780634fa26d401461023c57806359608fda1461025b5780635d9418021461028357806379ba5097146102a2575f5ffd5b80626d6cae1461017657806309c1ba2e1461019e57806312065fe0146101b35780631d2b2afd146101c557806336bfffed146101cf575f5ffd5b3661017257604080513381523460208201527f88a5966d370b9919b20f3e2c13ff65706f196a4e32cc2c12bf57088f88525874910160405180910390a1005b5f5ffd5b348015610181575f5ffd5b5061018b60045481565b6040519081526020015b60405180910390f35b3480156101a9575f5ffd5b5061018b60035481565b3480156101be575f5ffd5b504761018b565b6101cd6103e8565b005b3480156101da575f5ffd5b506101cd6101e9366004611313565b61048f565b3480156101f9575f5ffd5b5061020d610208366004611384565b61058c565b6040519015158152602001610195565b348015610228575f5ffd5b5061018b6102373660046113e0565b6105fe565b348015610247575f5ffd5b506101cd610256366004611483565b610664565b61026e6102693660046113e0565b6106ee565b60408051928352602083019190915201610195565b34801561028e575f5ffd5b506101cd61029d3660046114a3565b61075c565b3480156102ad575f5ffd5b506101cd6107c1565b3480156102c1575f5ffd5b506101cd6102d0366004611384565b61086a565b3480156102e0575f5ffd5b506102e96108a4565b6040516101959392919061155e565b348015610303575f5ffd5b505f546001600160a01b03165b6040516001600160a01b039091168152602001610195565b348015610333575f5ffd5b50600254610310906001600160a01b031681565b348015610352575f5ffd5b506101cd610361366004611483565b610a30565b348015610371575f5ffd5b5061018b600b5481565b348015610386575f5ffd5b506101cd610395366004611599565b610a44565b6101cd610b17565b3480156103ad575f5ffd5b5061020d6103bc366004611384565b610b5e565b6101cd610b8f565b3480156103d4575f5ffd5b506101cd6103e3366004611483565b610c0e565b6003545f0361042c5760405162461bcd60e51b815260206004820152600b60248201526a1cdd58881b9bdd081cd95d60aa1b60448201526064015b60405180910390fd5b60025460035460405163256d573f60e21b815260048101919091526001600160a01b03909116906395b55cfc9034906024015b5f604051808303818588803b158015610476575f5ffd5b505af1158015610488573d5f5f3e3d5ffd5b5050505050565b610497610c1f565b6003545f036104d85760405162461bcd60e51b815260206004820152600d60248201526c1cdd589251081b9bdd081cd95d609a1b6044820152606401610423565b5f5b81811015610587576002546003546001600160a01b039091169063bec4c08c9085858581811061050c5761050c6115c3565b90506020020160208101906105219190611483565b6040516001600160e01b031960e085901b16815260048101929092526001600160a01b031660248201526044015f604051808303815f87803b158015610565575f5ffd5b505af1158015610577573d5f5f3e3d5ffd5b5050600190920191506104da9050565b505050565b6002546040516341af6c8760e01b8152600481018390525f916001600160a01b0316906341af6c87906024015b602060405180830381865afa1580156105d4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105f891906115d7565b92915050565b5f5f6106418686868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250889250610c73915050565b600481905590508260056106558282611783565b50506004549695505050505050565b61066c610c1f565b6001600160a01b0381166106cc5760405162461bcd60e51b815260206004820152602160248201527f43616e6e6f7420736574207a65726f20616464726573732061732073656e64656044820152603960f91b6064820152608401610423565b600280546001600160a01b0319166001600160a01b0392909216919091179055565b5f5f5f5f6107338888888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508a9250610cf1915050565b6004829055909250905084600561074a8282611783565b50506004549890975095505050505050565b6002546001600160a01b031633146107b65760405162461bcd60e51b815260206004820181905260248201527f4f6e6c7920626c6f636b6c6f636b20636f6e74726163742063616e2063616c6c6044820152606401610423565b610587838383610e2b565b6001546001600160a01b031633146108145760405162461bcd60e51b815260206004820152601660248201527526bab9ba10313290383937b837b9b2b21037bbb732b960511b6044820152606401610423565b5f8054336001600160a01b0319808316821784556001805490911690556040516001600160a01b0390921692909183917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e091a350565b610872610c1f565b600381905560405181907f5f479ac993925efae23839aa1c813b6ecb2fe1bbd14dc78295ab267d2fca4368905f90a250565b604080516080810180835260059283918391820190839060029082845b8154815260200190600101908083116108c157505050918352505060408051808201918290526020909201919060028481019182845b8154815260200190600101908083116108f757505050505081525050908060040180546109239061164d565b80601f016020809104026020016040519081016040528092919081815260200182805461094f9061164d565b801561099a5780601f106109715761010080835404028352916020019161099a565b820191905f5260205f20905b81548152906001019060200180831161097d57829003601f168201915b5050505050908060050180546109af9061164d565b80601f01602080910402602001604051908101604052809291908181526020018280546109db9061164d565b8015610a265780601f106109fd57610100808354040283529160200191610a26565b820191905f5260205f20905b815481529060010190602001808311610a0957829003601f168201915b5050505050905083565b610a38610c1f565b610a4181611033565b50565b610a4c610c1f565b81471015610a9c5760405162461bcd60e51b815260206004820152601e60248201527f496e73756666696369656e742066756e647320696e20636f6e747261637400006044820152606401610423565b6040516001600160a01b0382169083156108fc029084905f818181858888f19350505050158015610acf573d5f5f3e3d5ffd5b50806001600160a01b03167f7084f5476618d8e60b11ef0d7d3f06914655adb8793e28ff7f018d4c76d505d583604051610b0b91815260200190565b60405180910390a25050565b610b1f610c1f565b610b276110cb565b600381905560025460405163256d573f60e21b815260048101929092526001600160a01b0316906395b55cfc90349060240161045f565b60025460405163cd802c9160e01b8152600481018390525f916001600160a01b03169063cd802c91906024016105b9565b5f3411610bd75760405162461bcd60e51b81526020600482015260166024820152750b2deea40daeae6e840e6cadcc840e6dedaca408aa8960531b6044820152606401610423565b60405134815233907f5af8184bef8e4b45eb9f6ed7734d04da38ced226495548f46e0c8ff8d7d9a5249060200160405180910390a2565b610c16610c1f565b610a41816111f5565b5f546001600160a01b03163314610c715760405162461bcd60e51b815260206004820152601660248201527527b7363c9031b0b63630b1363290313c9037bbb732b960511b6044820152606401610423565b565b6002546003546040516262b1d960e41b81525f926001600160a01b03169163062b1d9091610ca9918891889088906004016118c4565b6020604051808303815f875af1158015610cc5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ce99190611905565b949350505050565b600254604051634b16093560e01b815263ffffffff851660048201525f9182916001600160a01b0390911690634b16093590602401602060405180830381865afa158015610d41573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d659190611905565b905080341015610daa5760405162461bcd60e51b815260206004820152601060248201526f092dce6eaccccd2c6d2cadce8408aa8960831b6044820152606401610423565b600254604051637909dc7b60e01b81526001600160a01b0390911690637909dc7b903490610de09089908990899060040161191c565b60206040518083038185885af1158015610dfc573d5f5f3e3d5ffd5b50505050506040513d601f19601f82011682018060405250810190610e219190611905565b9150935093915050565b8260045414610e725760405162461bcd60e51b815260206004820152601360248201527224b73b30b634b2103932b8bab2b9ba1034b21760691b6044820152606401610423565b6040805160e081019091526110189060058160608101828160a084018260028282826020028201915b815481526020019060010190808311610e9b57505050918352505060408051808201918290526020909201919060028481019182845b815481526020019060010190808311610ed1575050505050815250508152602001600482018054610f019061164d565b80601f0160208091040260200160405190810160405280929190818152602001828054610f2d9061164d565b8015610f785780601f10610f4f57610100808354040283529160200191610f78565b820191905f5260205f20905b815481529060010190602001808311610f5b57829003601f168201915b50505050508152602001600582018054610f919061164d565b80601f0160208091040260200160405190810160405280929190818152602001828054610fbd9061164d565b80156110085780601f10610fdf57610100808354040283529160200191611008565b820191905f5260205f20905b815481529060010190602001808311610feb57829003601f168201915b505050505081525050838361129d565b80602001905181019061102b9190611905565b600b55505050565b6003545f0361107d5760405162461bcd60e51b8152602060048201526016602482015275537562736372697074696f6e4964206973207a65726f60501b6044820152606401610423565b600254600354604051622b825560e61b815260048101919091526001600160a01b03838116602483015290911690630ae09540906044015f604051808303815f87803b158015610476575f5ffd5b5f6003545f1461111d5760405162461bcd60e51b815260206004820152601a60248201527f537562736372697074696f6e4964206973206e6f74207a65726f0000000000006044820152606401610423565b60025f9054906101000a90046001600160a01b03166001600160a01b031663a21a23e46040518163ffffffff1660e01b81526004016020604051808303815f875af115801561116e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111929190611905565b600254604051632fb1302360e21b8152600481018390523060248201529192506001600160a01b03169063bec4c08c906044015f604051808303815f87803b1580156111dc575f5ffd5b505af11580156111ee573d5f5f3e3d5ffd5b5050505090565b336001600160a01b0382160361124d5760405162461bcd60e51b815260206004820152601760248201527f43616e6e6f74207472616e7366657220746f2073656c660000000000000000006044820152606401610423565b600180546001600160a01b0319166001600160a01b038381169182179092555f8054604051929316917fed8889f560326eb138920d842192f0eb3dd22b4f139c87a2c57538e05bae12789190a350565b60025460405163326f063160e21b81526060916001600160a01b03169063c9bc18c4906112d29087908790879060040161194c565b5f60405180830381865afa1580156112ec573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610ce991908101906119ab565b5f5f60208385031215611324575f5ffd5b823567ffffffffffffffff81111561133a575f5ffd5b8301601f8101851361134a575f5ffd5b803567ffffffffffffffff811115611360575f5ffd5b8560208260051b8401011115611374575f5ffd5b6020919091019590945092505050565b5f60208284031215611394575f5ffd5b5035919050565b5f5f83601f8401126113ab575f5ffd5b50813567ffffffffffffffff8111156113c2575f5ffd5b6020830191508360208285010111156113d9575f5ffd5b9250929050565b5f5f5f5f606085870312156113f3575f5ffd5b843563ffffffff81168114611406575f5ffd5b9350602085013567ffffffffffffffff811115611421575f5ffd5b61142d8782880161139b565b909450925050604085013567ffffffffffffffff81111561144c575f5ffd5b850160c0818803121561145d575f5ffd5b939692955090935050565b80356001600160a01b038116811461147e575f5ffd5b919050565b5f60208284031215611493575f5ffd5b61149c82611468565b9392505050565b5f5f5f604084860312156114b5575f5ffd5b83359250602084013567ffffffffffffffff8111156114d2575f5ffd5b6114de8682870161139b565b9497909650939450505050565b805f5b600281101561150d5781518452602093840193909101906001016114ee565b50505050565b61151e8282516114eb565b602081015161058760408401826114eb565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6115688185611513565b60c060808201525f61157d60c0830185611530565b82810360a084015261158f8185611530565b9695505050505050565b5f5f604083850312156115aa575f5ffd5b823591506115ba60208401611468565b90509250929050565b634e487b7160e01b5f52603260045260245ffd5b5f602082840312156115e7575f5ffd5b8151801515811461149c575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b5f5f8335601e1984360301811261161f575f5ffd5b83018035915067ffffffffffffffff821115611639575f5ffd5b6020019150368190038213156113d9575f5ffd5b600181811c9082168061166157607f821691505b60208210810361167f57634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561058757805f5260205f20601f840160051c810160208510156116aa5750805b601f840160051c820191505b81811015610488575f81556001016116b6565b67ffffffffffffffff8311156116e1576116e16115f6565b6116f5836116ef835461164d565b83611685565b5f601f841160018114611726575f851561170f5750838201355b5f19600387901b1c1916600186901b178355610488565b5f83815260208120601f198716915b828110156117555786850135825560209485019460019092019101611735565b5086821015611771575f1960f88860031b161c19848701351681555b505060018560011b0183555050505050565b815f5b60028110156117a357813583820155602090910190600101611786565b5050604082015f5b60028110156117cb578135838201600201556020909101906001016117ab565b50506117da608083018361160a565b6117e88183600486016116c9565b50506117f760a083018361160a565b61150d8183600586016116c9565b5f5f8335601e1984360301811261181a575f5ffd5b830160208101925035905067ffffffffffffffff811115611839575f5ffd5b8036038213156113d9575f5ffd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b6040818337604080820160408401375f61188c6080830183611805565b60c060808601526118a160c086018284611847565b9150506118b160a0840184611805565b85830360a087015261158f838284611847565b63ffffffff85168152836020820152608060408201525f6118e86080830185611530565b82810360608401526118fa818561186f565b979650505050505050565b5f60208284031215611915575f5ffd5b5051919050565b63ffffffff84168152606060208201525f61193a6060830185611530565b828103604084015261158f818561186f565b6040815261195e604082018551611513565b5f602085015160c080840152611978610100840182611530565b90506040860151603f198483030160e08501526119958282611530565b915050828103602084015261158f818587611847565b5f602082840312156119bb575f5ffd5b815167ffffffffffffffff8111156119d1575f5ffd5b8201601f810184136119e1575f5ffd5b805167ffffffffffffffff8111156119fb576119fb6115f6565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715611a2a57611a2a6115f6565b604052818152828201602001861015611a41575f5ffd5b8160208401602083015e5f9181016020019190915294935050505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x1C\x1E8\x03\x80a\x1C\x1E\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01\x86V[\x803\x80_\x81a\0\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FCannot set owner to zero\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x90\x91\x17\x90\x91U\x81\x16\x15a\0\xB3Wa\0\xB3\x81a\0\xDEV[PP`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91UPa\x01\xB3\x90PV[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\x016W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCannot transfer to self\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\0{V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x82\x17\x90\x92U_\x80T`@Q\x92\x93\x16\x91\x7F\xED\x88\x89\xF5`2n\xB18\x92\r\x84!\x92\xF0\xEB=\xD2+O\x13\x9C\x87\xA2\xC5u8\xE0[\xAE\x12x\x91\x90\xA3PV[_` \x82\x84\x03\x12\x15a\x01\x96W__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xACW__\xFD[\x93\x92PPPV[a\x1A^\x80a\x01\xC0_9_\xF3\xFE`\x80`@R`\x046\x10a\x013W_5`\xE0\x1C\x80c\x80\x98\0C\x11a\0\xA8W\x80c\x97\xA9\xC2\x85\x11a\0mW\x80c\x97\xA9\xC2\x85\x14a\x03fW\x80c\xB8\xCA\x8D\xD8\x14a\x03{W\x80c\xB9m\xBB\xA7\x14a\x03\x9AW\x80c\xCD\x80,\x91\x14a\x03\xA2W\x80c\xE6Jf\xEA\x14a\x03\xC1W\x80c\xF2\xFD\xE3\x8B\x14a\x03\xC9W__\xFD[\x80c\x80\x98\0C\x14a\x02\xB6W\x80c\x86\xE5`\xBE\x14a\x02\xD5W\x80c\x8D\xA5\xCB[\x14a\x02\xF8W\x80c\x93\xB9t\n\x14a\x03(W\x80c\x93\xD8\x1DX\x14a\x03GW__\xFD[\x80cA\xAFl\x87\x11a\0\xF9W\x80cA\xAFl\x87\x14a\x01\xEEW\x80cM=\xE3S\x14a\x02\x1DW\x80cO\xA2m@\x14a\x02<W\x80cY`\x8F\xDA\x14a\x02[W\x80c]\x94\x18\x02\x14a\x02\x83W\x80cy\xBAP\x97\x14a\x02\xA2W__\xFD[\x80bml\xAE\x14a\x01vW\x80c\t\xC1\xBA.\x14a\x01\x9EW\x80c\x12\x06_\xE0\x14a\x01\xB3W\x80c\x1D+*\xFD\x14a\x01\xC5W\x80c6\xBF\xFF\xED\x14a\x01\xCFW__\xFD[6a\x01rW`@\x80Q3\x81R4` \x82\x01R\x7F\x88\xA5\x96m7\x0B\x99\x19\xB2\x0F>,\x13\xFFepo\x19jN2\xCC,\x12\xBFW\x08\x8F\x88RXt\x91\x01`@Q\x80\x91\x03\x90\xA1\0[__\xFD[4\x80\x15a\x01\x81W__\xFD[Pa\x01\x8B`\x04T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xA9W__\xFD[Pa\x01\x8B`\x03T\x81V[4\x80\x15a\x01\xBEW__\xFD[PGa\x01\x8BV[a\x01\xCDa\x03\xE8V[\0[4\x80\x15a\x01\xDAW__\xFD[Pa\x01\xCDa\x01\xE96`\x04a\x13\x13V[a\x04\x8FV[4\x80\x15a\x01\xF9W__\xFD[Pa\x02\ra\x02\x086`\x04a\x13\x84V[a\x05\x8CV[`@Q\x90\x15\x15\x81R` \x01a\x01\x95V[4\x80\x15a\x02(W__\xFD[Pa\x01\x8Ba\x0276`\x04a\x13\xE0V[a\x05\xFEV[4\x80\x15a\x02GW__\xFD[Pa\x01\xCDa\x02V6`\x04a\x14\x83V[a\x06dV[a\x02na\x02i6`\x04a\x13\xE0V[a\x06\xEEV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x95V[4\x80\x15a\x02\x8EW__\xFD[Pa\x01\xCDa\x02\x9D6`\x04a\x14\xA3V[a\x07\\V[4\x80\x15a\x02\xADW__\xFD[Pa\x01\xCDa\x07\xC1V[4\x80\x15a\x02\xC1W__\xFD[Pa\x01\xCDa\x02\xD06`\x04a\x13\x84V[a\x08jV[4\x80\x15a\x02\xE0W__\xFD[Pa\x02\xE9a\x08\xA4V[`@Qa\x01\x95\x93\x92\x91\x90a\x15^V[4\x80\x15a\x03\x03W__\xFD[P_T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x95V[4\x80\x15a\x033W__\xFD[P`\x02Ta\x03\x10\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x03RW__\xFD[Pa\x01\xCDa\x03a6`\x04a\x14\x83V[a\n0V[4\x80\x15a\x03qW__\xFD[Pa\x01\x8B`\x0BT\x81V[4\x80\x15a\x03\x86W__\xFD[Pa\x01\xCDa\x03\x956`\x04a\x15\x99V[a\nDV[a\x01\xCDa\x0B\x17V[4\x80\x15a\x03\xADW__\xFD[Pa\x02\ra\x03\xBC6`\x04a\x13\x84V[a\x0B^V[a\x01\xCDa\x0B\x8FV[4\x80\x15a\x03\xD4W__\xFD[Pa\x01\xCDa\x03\xE36`\x04a\x14\x83V[a\x0C\x0EV[`\x03T_\x03a\x04,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x1C\xDDX\x88\x1B\x9B\xDD\x08\x1C\xD9]`\xAA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02T`\x03T`@Qc%mW?`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x95\xB5\\\xFC\x904\x90`$\x01[_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x04vW__\xFD[PZ\xF1\x15\x80\x15a\x04\x88W=__>=_\xFD[PPPPPV[a\x04\x97a\x0C\x1FV[`\x03T_\x03a\x04\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x1C\xDDX\x92Q\x08\x1B\x9B\xDD\x08\x1C\xD9]`\x9A\x1B`D\x82\x01R`d\x01a\x04#V[_[\x81\x81\x10\x15a\x05\x87W`\x02T`\x03T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBE\xC4\xC0\x8C\x90\x85\x85\x85\x81\x81\x10a\x05\x0CWa\x05\x0Ca\x15\xC3V[\x90P` \x02\x01` \x81\x01\x90a\x05!\x91\x90a\x14\x83V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05eW__\xFD[PZ\xF1\x15\x80\x15a\x05wW=__>=_\xFD[PP`\x01\x90\x92\x01\x91Pa\x04\xDA\x90PV[PPPV[`\x02T`@QcA\xAFl\x87`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cA\xAFl\x87\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xD4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xF8\x91\x90a\x15\xD7V[\x92\x91PPV[__a\x06A\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x0Cs\x91PPV[`\x04\x81\x90U\x90P\x82`\x05a\x06U\x82\x82a\x17\x83V[PP`\x04T\x96\x95PPPPPPV[a\x06la\x0C\x1FV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FCannot set zero address as sende`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x01a\x04#V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[____a\x073\x88\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8A\x92Pa\x0C\xF1\x91PPV[`\x04\x82\x90U\x90\x92P\x90P\x84`\x05a\x07J\x82\x82a\x17\x83V[PP`\x04T\x98\x90\x97P\x95PPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOnly blocklock contract can call`D\x82\x01R`d\x01a\x04#V[a\x05\x87\x83\x83\x83a\x0E+V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru&\xBA\xB9\xBA\x1012\x90897\xB87\xB9\xB2\xB2\x107\xBB\xB72\xB9`Q\x1B`D\x82\x01R`d\x01a\x04#V[_\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x80\x83\x16\x82\x17\x84U`\x01\x80T\x90\x91\x16\x90U`@Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x90\x91\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\xA3PV[a\x08ra\x0C\x1FV[`\x03\x81\x90U`@Q\x81\x90\x7F_G\x9A\xC9\x93\x92^\xFA\xE289\xAA\x1C\x81;n\xCB/\xE1\xBB\xD1M\xC7\x82\x95\xAB&}/\xCACh\x90_\x90\xA2PV[`@\x80Q`\x80\x81\x01\x80\x83R`\x05\x92\x83\x91\x83\x91\x82\x01\x90\x83\x90`\x02\x90\x82\x84[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xC1WPPP\x91\x83RPP`@\x80Q\x80\x82\x01\x91\x82\x90R` \x90\x92\x01\x91\x90`\x02\x84\x81\x01\x91\x82\x84[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xF7WPPPPP\x81RPP\x90\x80`\x04\x01\x80Ta\t#\x90a\x16MV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tO\x90a\x16MV[\x80\x15a\t\x9AW\x80`\x1F\x10a\tqWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x9AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t}W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x05\x01\x80Ta\t\xAF\x90a\x16MV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xDB\x90a\x16MV[\x80\x15a\n&W\x80`\x1F\x10a\t\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n&V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\tW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[a\n8a\x0C\x1FV[a\nA\x81a\x103V[PV[a\nLa\x0C\x1FV[\x81G\x10\x15a\n\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInsufficient funds in contract\0\0`D\x82\x01R`d\x01a\x04#V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x83\x15a\x08\xFC\x02\x90\x84\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\n\xCFW=__>=_\xFD[P\x80`\x01`\x01`\xA0\x1B\x03\x16\x7Fp\x84\xF5Gf\x18\xD8\xE6\x0B\x11\xEF\r}?\x06\x91FU\xAD\xB8y>(\xFF\x7F\x01\x8DLv\xD5\x05\xD5\x83`@Qa\x0B\x0B\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[a\x0B\x1Fa\x0C\x1FV[a\x0B'a\x10\xCBV[`\x03\x81\x90U`\x02T`@Qc%mW?`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90c\x95\xB5\\\xFC\x904\x90`$\x01a\x04_V[`\x02T`@Qc\xCD\x80,\x91`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCD\x80,\x91\x90`$\x01a\x05\xB9V[_4\x11a\x0B\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x0B-\xEE\xA4\r\xAE\xAEn\x84\x0El\xAD\xCC\x84\x0Em\xED\xAC\xA4\x08\xAA\x89`S\x1B`D\x82\x01R`d\x01a\x04#V[`@Q4\x81R3\x90\x7FZ\xF8\x18K\xEF\x8EKE\xEB\x9Fn\xD7sM\x04\xDA8\xCE\xD2&IUH\xF4n\x0C\x8F\xF8\xD7\xD9\xA5$\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x0C\x16a\x0C\x1FV[a\nA\x81a\x11\xF5V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0CqW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru'\xB76<\x901\xB0\xB660\xB162\x901<\x907\xBB\xB72\xB9`Q\x1B`D\x82\x01R`d\x01a\x04#V[V[`\x02T`\x03T`@Qbb\xB1\xD9`\xE4\x1B\x81R_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x06+\x1D\x90\x91a\x0C\xA9\x91\x88\x91\x88\x90\x88\x90`\x04\x01a\x18\xC4V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\xC5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE9\x91\x90a\x19\x05V[\x94\x93PPPPV[`\x02T`@QcK\x16\t5`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R_\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cK\x16\t5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\re\x91\x90a\x19\x05V[\x90P\x804\x10\x15a\r\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\t-\xCEn\xAC\xCC\xCD,m,\xAD\xCE\x84\x08\xAA\x89`\x83\x1B`D\x82\x01R`d\x01a\x04#V[`\x02T`@Qcy\t\xDC{`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cy\t\xDC{\x904\x90a\r\xE0\x90\x89\x90\x89\x90\x89\x90`\x04\x01a\x19\x1CV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\r\xFCW=__>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E!\x91\x90a\x19\x05V[\x91P\x93P\x93\x91PPV[\x82`\x04T\x14a\x0ErW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr$\xB7;0\xB64\xB2\x1092\xB8\xBA\xB2\xB9\xBA\x104\xB2\x17`i\x1B`D\x82\x01R`d\x01a\x04#V[`@\x80Q`\xE0\x81\x01\x90\x91Ra\x10\x18\x90`\x05\x81``\x81\x01\x82\x81`\xA0\x84\x01\x82`\x02\x82\x82\x82` \x02\x82\x01\x91[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0E\x9BWPPP\x91\x83RPP`@\x80Q\x80\x82\x01\x91\x82\x90R` \x90\x92\x01\x91\x90`\x02\x84\x81\x01\x91\x82\x84[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0E\xD1WPPPPP\x81RPP\x81R` \x01`\x04\x82\x01\x80Ta\x0F\x01\x90a\x16MV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F-\x90a\x16MV[\x80\x15a\x0FxW\x80`\x1F\x10a\x0FOWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0FxV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F[W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x05\x82\x01\x80Ta\x0F\x91\x90a\x16MV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\xBD\x90a\x16MV[\x80\x15a\x10\x08W\x80`\x1F\x10a\x0F\xDFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\x08V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F\xEBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x83\x83a\x12\x9DV[\x80` \x01\x90Q\x81\x01\x90a\x10+\x91\x90a\x19\x05V[`\x0BUPPPV[`\x03T_\x03a\x10}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuSubscriptionId is zero`P\x1B`D\x82\x01R`d\x01a\x04#V[`\x02T`\x03T`@Qb+\x82U`\xE6\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R\x90\x91\x16\x90c\n\xE0\x95@\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04vW__\xFD[_`\x03T_\x14a\x11\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FSubscriptionId is not zero\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04#V[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xA2\x1A#\xE4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x92\x91\x90a\x19\x05V[`\x02T`@Qc/\xB10#`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xBE\xC4\xC0\x8C\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11\xDCW__\xFD[PZ\xF1\x15\x80\x15a\x11\xEEW=__>=_\xFD[PPPP\x90V[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\x12MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCannot transfer to self\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04#V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x82\x17\x90\x92U_\x80T`@Q\x92\x93\x16\x91\x7F\xED\x88\x89\xF5`2n\xB18\x92\r\x84!\x92\xF0\xEB=\xD2+O\x13\x9C\x87\xA2\xC5u8\xE0[\xAE\x12x\x91\x90\xA3PV[`\x02T`@Qc2o\x061`\xE2\x1B\x81R``\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC9\xBC\x18\xC4\x90a\x12\xD2\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x19LV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xECW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C\xE9\x91\x90\x81\x01\x90a\x19\xABV[__` \x83\x85\x03\x12\x15a\x13$W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13:W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x13JW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13`W__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x13tW__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_` \x82\x84\x03\x12\x15a\x13\x94W__\xFD[P5\x91\x90PV[__\x83`\x1F\x84\x01\x12a\x13\xABW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xC2W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x13\xD9W__\xFD[\x92P\x92\x90PV[____``\x85\x87\x03\x12\x15a\x13\xF3W__\xFD[\x845c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x14\x06W__\xFD[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14!W__\xFD[a\x14-\x87\x82\x88\x01a\x13\x9BV[\x90\x94P\x92PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14LW__\xFD[\x85\x01`\xC0\x81\x88\x03\x12\x15a\x14]W__\xFD[\x93\x96\x92\x95P\x90\x93PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x14~W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x14\x93W__\xFD[a\x14\x9C\x82a\x14hV[\x93\x92PPPV[___`@\x84\x86\x03\x12\x15a\x14\xB5W__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xD2W__\xFD[a\x14\xDE\x86\x82\x87\x01a\x13\x9BV[\x94\x97\x90\x96P\x93\x94PPPPV[\x80_[`\x02\x81\x10\x15a\x15\rW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x14\xEEV[PPPPV[a\x15\x1E\x82\x82Qa\x14\xEBV[` \x81\x01Qa\x05\x87`@\x84\x01\x82a\x14\xEBV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[a\x15h\x81\x85a\x15\x13V[`\xC0`\x80\x82\x01R_a\x15}`\xC0\x83\x01\x85a\x150V[\x82\x81\x03`\xA0\x84\x01Ra\x15\x8F\x81\x85a\x150V[\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15a\x15\xAAW__\xFD[\x825\x91Pa\x15\xBA` \x84\x01a\x14hV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x15\xE7W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x14\x9CW__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x16\x1FW__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x169W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x13\xD9W__\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x16aW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x16\x7FWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x05\x87W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x16\xAAWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x04\x88W_\x81U`\x01\x01a\x16\xB6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x16\xE1Wa\x16\xE1a\x15\xF6V[a\x16\xF5\x83a\x16\xEF\x83Ta\x16MV[\x83a\x16\x85V[_`\x1F\x84\x11`\x01\x81\x14a\x17&W_\x85\x15a\x17\x0FWP\x83\x82\x015[_\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x04\x88V[_\x83\x81R` \x81 `\x1F\x19\x87\x16\x91[\x82\x81\x10\x15a\x17UW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x175V[P\x86\x82\x10\x15a\x17qW_\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[\x81_[`\x02\x81\x10\x15a\x17\xA3W\x815\x83\x82\x01U` \x90\x91\x01\x90`\x01\x01a\x17\x86V[PP`@\x82\x01_[`\x02\x81\x10\x15a\x17\xCBW\x815\x83\x82\x01`\x02\x01U` \x90\x91\x01\x90`\x01\x01a\x17\xABV[PPa\x17\xDA`\x80\x83\x01\x83a\x16\nV[a\x17\xE8\x81\x83`\x04\x86\x01a\x16\xC9V[PPa\x17\xF7`\xA0\x83\x01\x83a\x16\nV[a\x15\r\x81\x83`\x05\x86\x01a\x16\xC9V[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x18\x1AW__\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x189W__\xFD[\x806\x03\x82\x13\x15a\x13\xD9W__\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`@\x81\x837`@\x80\x82\x01`@\x84\x017_a\x18\x8C`\x80\x83\x01\x83a\x18\x05V[`\xC0`\x80\x86\x01Ra\x18\xA1`\xC0\x86\x01\x82\x84a\x18GV[\x91PPa\x18\xB1`\xA0\x84\x01\x84a\x18\x05V[\x85\x83\x03`\xA0\x87\x01Ra\x15\x8F\x83\x82\x84a\x18GV[c\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_a\x18\xE8`\x80\x83\x01\x85a\x150V[\x82\x81\x03``\x84\x01Ra\x18\xFA\x81\x85a\x18oV[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a\x19\x15W__\xFD[PQ\x91\x90PV[c\xFF\xFF\xFF\xFF\x84\x16\x81R``` \x82\x01R_a\x19:``\x83\x01\x85a\x150V[\x82\x81\x03`@\x84\x01Ra\x15\x8F\x81\x85a\x18oV[`@\x81Ra\x19^`@\x82\x01\x85Qa\x15\x13V[_` \x85\x01Q`\xC0\x80\x84\x01Ra\x19xa\x01\0\x84\x01\x82a\x150V[\x90P`@\x86\x01Q`?\x19\x84\x83\x03\x01`\xE0\x85\x01Ra\x19\x95\x82\x82a\x150V[\x91PP\x82\x81\x03` \x84\x01Ra\x15\x8F\x81\x85\x87a\x18GV[_` \x82\x84\x03\x12\x15a\x19\xBBW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xD1W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x19\xE1W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xFBWa\x19\xFBa\x15\xF6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A*Wa\x1A*a\x15\xF6V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x1AAW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610133575f3560e01c806380980043116100a857806397a9c2851161006d57806397a9c28514610366578063b8ca8dd81461037b578063b96dbba71461039a578063cd802c91146103a2578063e64a66ea146103c1578063f2fde38b146103c9575f5ffd5b806380980043146102b657806386e560be146102d55780638da5cb5b146102f857806393b9740a1461032857806393d81d5814610347575f5ffd5b806341af6c87116100f957806341af6c87146101ee5780634d3de3531461021d5780634fa26d401461023c57806359608fda1461025b5780635d9418021461028357806379ba5097146102a2575f5ffd5b80626d6cae1461017657806309c1ba2e1461019e57806312065fe0146101b35780631d2b2afd146101c557806336bfffed146101cf575f5ffd5b3661017257604080513381523460208201527f88a5966d370b9919b20f3e2c13ff65706f196a4e32cc2c12bf57088f88525874910160405180910390a1005b5f5ffd5b348015610181575f5ffd5b5061018b60045481565b6040519081526020015b60405180910390f35b3480156101a9575f5ffd5b5061018b60035481565b3480156101be575f5ffd5b504761018b565b6101cd6103e8565b005b3480156101da575f5ffd5b506101cd6101e9366004611313565b61048f565b3480156101f9575f5ffd5b5061020d610208366004611384565b61058c565b6040519015158152602001610195565b348015610228575f5ffd5b5061018b6102373660046113e0565b6105fe565b348015610247575f5ffd5b506101cd610256366004611483565b610664565b61026e6102693660046113e0565b6106ee565b60408051928352602083019190915201610195565b34801561028e575f5ffd5b506101cd61029d3660046114a3565b61075c565b3480156102ad575f5ffd5b506101cd6107c1565b3480156102c1575f5ffd5b506101cd6102d0366004611384565b61086a565b3480156102e0575f5ffd5b506102e96108a4565b6040516101959392919061155e565b348015610303575f5ffd5b505f546001600160a01b03165b6040516001600160a01b039091168152602001610195565b348015610333575f5ffd5b50600254610310906001600160a01b031681565b348015610352575f5ffd5b506101cd610361366004611483565b610a30565b348015610371575f5ffd5b5061018b600b5481565b348015610386575f5ffd5b506101cd610395366004611599565b610a44565b6101cd610b17565b3480156103ad575f5ffd5b5061020d6103bc366004611384565b610b5e565b6101cd610b8f565b3480156103d4575f5ffd5b506101cd6103e3366004611483565b610c0e565b6003545f0361042c5760405162461bcd60e51b815260206004820152600b60248201526a1cdd58881b9bdd081cd95d60aa1b60448201526064015b60405180910390fd5b60025460035460405163256d573f60e21b815260048101919091526001600160a01b03909116906395b55cfc9034906024015b5f604051808303818588803b158015610476575f5ffd5b505af1158015610488573d5f5f3e3d5ffd5b5050505050565b610497610c1f565b6003545f036104d85760405162461bcd60e51b815260206004820152600d60248201526c1cdd589251081b9bdd081cd95d609a1b6044820152606401610423565b5f5b81811015610587576002546003546001600160a01b039091169063bec4c08c9085858581811061050c5761050c6115c3565b90506020020160208101906105219190611483565b6040516001600160e01b031960e085901b16815260048101929092526001600160a01b031660248201526044015f604051808303815f87803b158015610565575f5ffd5b505af1158015610577573d5f5f3e3d5ffd5b5050600190920191506104da9050565b505050565b6002546040516341af6c8760e01b8152600481018390525f916001600160a01b0316906341af6c87906024015b602060405180830381865afa1580156105d4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105f891906115d7565b92915050565b5f5f6106418686868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250889250610c73915050565b600481905590508260056106558282611783565b50506004549695505050505050565b61066c610c1f565b6001600160a01b0381166106cc5760405162461bcd60e51b815260206004820152602160248201527f43616e6e6f7420736574207a65726f20616464726573732061732073656e64656044820152603960f91b6064820152608401610423565b600280546001600160a01b0319166001600160a01b0392909216919091179055565b5f5f5f5f6107338888888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508a9250610cf1915050565b6004829055909250905084600561074a8282611783565b50506004549890975095505050505050565b6002546001600160a01b031633146107b65760405162461bcd60e51b815260206004820181905260248201527f4f6e6c7920626c6f636b6c6f636b20636f6e74726163742063616e2063616c6c6044820152606401610423565b610587838383610e2b565b6001546001600160a01b031633146108145760405162461bcd60e51b815260206004820152601660248201527526bab9ba10313290383937b837b9b2b21037bbb732b960511b6044820152606401610423565b5f8054336001600160a01b0319808316821784556001805490911690556040516001600160a01b0390921692909183917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e091a350565b610872610c1f565b600381905560405181907f5f479ac993925efae23839aa1c813b6ecb2fe1bbd14dc78295ab267d2fca4368905f90a250565b604080516080810180835260059283918391820190839060029082845b8154815260200190600101908083116108c157505050918352505060408051808201918290526020909201919060028481019182845b8154815260200190600101908083116108f757505050505081525050908060040180546109239061164d565b80601f016020809104026020016040519081016040528092919081815260200182805461094f9061164d565b801561099a5780601f106109715761010080835404028352916020019161099a565b820191905f5260205f20905b81548152906001019060200180831161097d57829003601f168201915b5050505050908060050180546109af9061164d565b80601f01602080910402602001604051908101604052809291908181526020018280546109db9061164d565b8015610a265780601f106109fd57610100808354040283529160200191610a26565b820191905f5260205f20905b815481529060010190602001808311610a0957829003601f168201915b5050505050905083565b610a38610c1f565b610a4181611033565b50565b610a4c610c1f565b81471015610a9c5760405162461bcd60e51b815260206004820152601e60248201527f496e73756666696369656e742066756e647320696e20636f6e747261637400006044820152606401610423565b6040516001600160a01b0382169083156108fc029084905f818181858888f19350505050158015610acf573d5f5f3e3d5ffd5b50806001600160a01b03167f7084f5476618d8e60b11ef0d7d3f06914655adb8793e28ff7f018d4c76d505d583604051610b0b91815260200190565b60405180910390a25050565b610b1f610c1f565b610b276110cb565b600381905560025460405163256d573f60e21b815260048101929092526001600160a01b0316906395b55cfc90349060240161045f565b60025460405163cd802c9160e01b8152600481018390525f916001600160a01b03169063cd802c91906024016105b9565b5f3411610bd75760405162461bcd60e51b81526020600482015260166024820152750b2deea40daeae6e840e6cadcc840e6dedaca408aa8960531b6044820152606401610423565b60405134815233907f5af8184bef8e4b45eb9f6ed7734d04da38ced226495548f46e0c8ff8d7d9a5249060200160405180910390a2565b610c16610c1f565b610a41816111f5565b5f546001600160a01b03163314610c715760405162461bcd60e51b815260206004820152601660248201527527b7363c9031b0b63630b1363290313c9037bbb732b960511b6044820152606401610423565b565b6002546003546040516262b1d960e41b81525f926001600160a01b03169163062b1d9091610ca9918891889088906004016118c4565b6020604051808303815f875af1158015610cc5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ce99190611905565b949350505050565b600254604051634b16093560e01b815263ffffffff851660048201525f9182916001600160a01b0390911690634b16093590602401602060405180830381865afa158015610d41573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d659190611905565b905080341015610daa5760405162461bcd60e51b815260206004820152601060248201526f092dce6eaccccd2c6d2cadce8408aa8960831b6044820152606401610423565b600254604051637909dc7b60e01b81526001600160a01b0390911690637909dc7b903490610de09089908990899060040161191c565b60206040518083038185885af1158015610dfc573d5f5f3e3d5ffd5b50505050506040513d601f19601f82011682018060405250810190610e219190611905565b9150935093915050565b8260045414610e725760405162461bcd60e51b815260206004820152601360248201527224b73b30b634b2103932b8bab2b9ba1034b21760691b6044820152606401610423565b6040805160e081019091526110189060058160608101828160a084018260028282826020028201915b815481526020019060010190808311610e9b57505050918352505060408051808201918290526020909201919060028481019182845b815481526020019060010190808311610ed1575050505050815250508152602001600482018054610f019061164d565b80601f0160208091040260200160405190810160405280929190818152602001828054610f2d9061164d565b8015610f785780601f10610f4f57610100808354040283529160200191610f78565b820191905f5260205f20905b815481529060010190602001808311610f5b57829003601f168201915b50505050508152602001600582018054610f919061164d565b80601f0160208091040260200160405190810160405280929190818152602001828054610fbd9061164d565b80156110085780601f10610fdf57610100808354040283529160200191611008565b820191905f5260205f20905b815481529060010190602001808311610feb57829003601f168201915b505050505081525050838361129d565b80602001905181019061102b9190611905565b600b55505050565b6003545f0361107d5760405162461bcd60e51b8152602060048201526016602482015275537562736372697074696f6e4964206973207a65726f60501b6044820152606401610423565b600254600354604051622b825560e61b815260048101919091526001600160a01b03838116602483015290911690630ae09540906044015f604051808303815f87803b158015610476575f5ffd5b5f6003545f1461111d5760405162461bcd60e51b815260206004820152601a60248201527f537562736372697074696f6e4964206973206e6f74207a65726f0000000000006044820152606401610423565b60025f9054906101000a90046001600160a01b03166001600160a01b031663a21a23e46040518163ffffffff1660e01b81526004016020604051808303815f875af115801561116e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111929190611905565b600254604051632fb1302360e21b8152600481018390523060248201529192506001600160a01b03169063bec4c08c906044015f604051808303815f87803b1580156111dc575f5ffd5b505af11580156111ee573d5f5f3e3d5ffd5b5050505090565b336001600160a01b0382160361124d5760405162461bcd60e51b815260206004820152601760248201527f43616e6e6f74207472616e7366657220746f2073656c660000000000000000006044820152606401610423565b600180546001600160a01b0319166001600160a01b038381169182179092555f8054604051929316917fed8889f560326eb138920d842192f0eb3dd22b4f139c87a2c57538e05bae12789190a350565b60025460405163326f063160e21b81526060916001600160a01b03169063c9bc18c4906112d29087908790879060040161194c565b5f60405180830381865afa1580156112ec573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610ce991908101906119ab565b5f5f60208385031215611324575f5ffd5b823567ffffffffffffffff81111561133a575f5ffd5b8301601f8101851361134a575f5ffd5b803567ffffffffffffffff811115611360575f5ffd5b8560208260051b8401011115611374575f5ffd5b6020919091019590945092505050565b5f60208284031215611394575f5ffd5b5035919050565b5f5f83601f8401126113ab575f5ffd5b50813567ffffffffffffffff8111156113c2575f5ffd5b6020830191508360208285010111156113d9575f5ffd5b9250929050565b5f5f5f5f606085870312156113f3575f5ffd5b843563ffffffff81168114611406575f5ffd5b9350602085013567ffffffffffffffff811115611421575f5ffd5b61142d8782880161139b565b909450925050604085013567ffffffffffffffff81111561144c575f5ffd5b850160c0818803121561145d575f5ffd5b939692955090935050565b80356001600160a01b038116811461147e575f5ffd5b919050565b5f60208284031215611493575f5ffd5b61149c82611468565b9392505050565b5f5f5f604084860312156114b5575f5ffd5b83359250602084013567ffffffffffffffff8111156114d2575f5ffd5b6114de8682870161139b565b9497909650939450505050565b805f5b600281101561150d5781518452602093840193909101906001016114ee565b50505050565b61151e8282516114eb565b602081015161058760408401826114eb565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6115688185611513565b60c060808201525f61157d60c0830185611530565b82810360a084015261158f8185611530565b9695505050505050565b5f5f604083850312156115aa575f5ffd5b823591506115ba60208401611468565b90509250929050565b634e487b7160e01b5f52603260045260245ffd5b5f602082840312156115e7575f5ffd5b8151801515811461149c575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b5f5f8335601e1984360301811261161f575f5ffd5b83018035915067ffffffffffffffff821115611639575f5ffd5b6020019150368190038213156113d9575f5ffd5b600181811c9082168061166157607f821691505b60208210810361167f57634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561058757805f5260205f20601f840160051c810160208510156116aa5750805b601f840160051c820191505b81811015610488575f81556001016116b6565b67ffffffffffffffff8311156116e1576116e16115f6565b6116f5836116ef835461164d565b83611685565b5f601f841160018114611726575f851561170f5750838201355b5f19600387901b1c1916600186901b178355610488565b5f83815260208120601f198716915b828110156117555786850135825560209485019460019092019101611735565b5086821015611771575f1960f88860031b161c19848701351681555b505060018560011b0183555050505050565b815f5b60028110156117a357813583820155602090910190600101611786565b5050604082015f5b60028110156117cb578135838201600201556020909101906001016117ab565b50506117da608083018361160a565b6117e88183600486016116c9565b50506117f760a083018361160a565b61150d8183600586016116c9565b5f5f8335601e1984360301811261181a575f5ffd5b830160208101925035905067ffffffffffffffff811115611839575f5ffd5b8036038213156113d9575f5ffd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b6040818337604080820160408401375f61188c6080830183611805565b60c060808601526118a160c086018284611847565b9150506118b160a0840184611805565b85830360a087015261158f838284611847565b63ffffffff85168152836020820152608060408201525f6118e86080830185611530565b82810360608401526118fa818561186f565b979650505050505050565b5f60208284031215611915575f5ffd5b5051919050565b63ffffffff84168152606060208201525f61193a6060830185611530565b828103604084015261158f818561186f565b6040815261195e604082018551611513565b5f602085015160c080840152611978610100840182611530565b90506040860151603f198483030160e08501526119958282611530565b915050828103602084015261158f818587611847565b5f602082840312156119bb575f5ffd5b815167ffffffffffffffff8111156119d1575f5ffd5b8201601f810184136119e1575f5ffd5b805167ffffffffffffffff8111156119fb576119fb6115f6565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715611a2a57611a2a6115f6565b604052818152828201602001861015611a41575f5ffd5b8160208401602083015e5f9181016020019190915294935050505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x013W_5`\xE0\x1C\x80c\x80\x98\0C\x11a\0\xA8W\x80c\x97\xA9\xC2\x85\x11a\0mW\x80c\x97\xA9\xC2\x85\x14a\x03fW\x80c\xB8\xCA\x8D\xD8\x14a\x03{W\x80c\xB9m\xBB\xA7\x14a\x03\x9AW\x80c\xCD\x80,\x91\x14a\x03\xA2W\x80c\xE6Jf\xEA\x14a\x03\xC1W\x80c\xF2\xFD\xE3\x8B\x14a\x03\xC9W__\xFD[\x80c\x80\x98\0C\x14a\x02\xB6W\x80c\x86\xE5`\xBE\x14a\x02\xD5W\x80c\x8D\xA5\xCB[\x14a\x02\xF8W\x80c\x93\xB9t\n\x14a\x03(W\x80c\x93\xD8\x1DX\x14a\x03GW__\xFD[\x80cA\xAFl\x87\x11a\0\xF9W\x80cA\xAFl\x87\x14a\x01\xEEW\x80cM=\xE3S\x14a\x02\x1DW\x80cO\xA2m@\x14a\x02<W\x80cY`\x8F\xDA\x14a\x02[W\x80c]\x94\x18\x02\x14a\x02\x83W\x80cy\xBAP\x97\x14a\x02\xA2W__\xFD[\x80bml\xAE\x14a\x01vW\x80c\t\xC1\xBA.\x14a\x01\x9EW\x80c\x12\x06_\xE0\x14a\x01\xB3W\x80c\x1D+*\xFD\x14a\x01\xC5W\x80c6\xBF\xFF\xED\x14a\x01\xCFW__\xFD[6a\x01rW`@\x80Q3\x81R4` \x82\x01R\x7F\x88\xA5\x96m7\x0B\x99\x19\xB2\x0F>,\x13\xFFepo\x19jN2\xCC,\x12\xBFW\x08\x8F\x88RXt\x91\x01`@Q\x80\x91\x03\x90\xA1\0[__\xFD[4\x80\x15a\x01\x81W__\xFD[Pa\x01\x8B`\x04T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xA9W__\xFD[Pa\x01\x8B`\x03T\x81V[4\x80\x15a\x01\xBEW__\xFD[PGa\x01\x8BV[a\x01\xCDa\x03\xE8V[\0[4\x80\x15a\x01\xDAW__\xFD[Pa\x01\xCDa\x01\xE96`\x04a\x13\x13V[a\x04\x8FV[4\x80\x15a\x01\xF9W__\xFD[Pa\x02\ra\x02\x086`\x04a\x13\x84V[a\x05\x8CV[`@Q\x90\x15\x15\x81R` \x01a\x01\x95V[4\x80\x15a\x02(W__\xFD[Pa\x01\x8Ba\x0276`\x04a\x13\xE0V[a\x05\xFEV[4\x80\x15a\x02GW__\xFD[Pa\x01\xCDa\x02V6`\x04a\x14\x83V[a\x06dV[a\x02na\x02i6`\x04a\x13\xE0V[a\x06\xEEV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x95V[4\x80\x15a\x02\x8EW__\xFD[Pa\x01\xCDa\x02\x9D6`\x04a\x14\xA3V[a\x07\\V[4\x80\x15a\x02\xADW__\xFD[Pa\x01\xCDa\x07\xC1V[4\x80\x15a\x02\xC1W__\xFD[Pa\x01\xCDa\x02\xD06`\x04a\x13\x84V[a\x08jV[4\x80\x15a\x02\xE0W__\xFD[Pa\x02\xE9a\x08\xA4V[`@Qa\x01\x95\x93\x92\x91\x90a\x15^V[4\x80\x15a\x03\x03W__\xFD[P_T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x95V[4\x80\x15a\x033W__\xFD[P`\x02Ta\x03\x10\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x03RW__\xFD[Pa\x01\xCDa\x03a6`\x04a\x14\x83V[a\n0V[4\x80\x15a\x03qW__\xFD[Pa\x01\x8B`\x0BT\x81V[4\x80\x15a\x03\x86W__\xFD[Pa\x01\xCDa\x03\x956`\x04a\x15\x99V[a\nDV[a\x01\xCDa\x0B\x17V[4\x80\x15a\x03\xADW__\xFD[Pa\x02\ra\x03\xBC6`\x04a\x13\x84V[a\x0B^V[a\x01\xCDa\x0B\x8FV[4\x80\x15a\x03\xD4W__\xFD[Pa\x01\xCDa\x03\xE36`\x04a\x14\x83V[a\x0C\x0EV[`\x03T_\x03a\x04,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x1C\xDDX\x88\x1B\x9B\xDD\x08\x1C\xD9]`\xAA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02T`\x03T`@Qc%mW?`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x95\xB5\\\xFC\x904\x90`$\x01[_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x04vW__\xFD[PZ\xF1\x15\x80\x15a\x04\x88W=__>=_\xFD[PPPPPV[a\x04\x97a\x0C\x1FV[`\x03T_\x03a\x04\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x1C\xDDX\x92Q\x08\x1B\x9B\xDD\x08\x1C\xD9]`\x9A\x1B`D\x82\x01R`d\x01a\x04#V[_[\x81\x81\x10\x15a\x05\x87W`\x02T`\x03T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBE\xC4\xC0\x8C\x90\x85\x85\x85\x81\x81\x10a\x05\x0CWa\x05\x0Ca\x15\xC3V[\x90P` \x02\x01` \x81\x01\x90a\x05!\x91\x90a\x14\x83V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05eW__\xFD[PZ\xF1\x15\x80\x15a\x05wW=__>=_\xFD[PP`\x01\x90\x92\x01\x91Pa\x04\xDA\x90PV[PPPV[`\x02T`@QcA\xAFl\x87`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cA\xAFl\x87\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xD4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xF8\x91\x90a\x15\xD7V[\x92\x91PPV[__a\x06A\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x0Cs\x91PPV[`\x04\x81\x90U\x90P\x82`\x05a\x06U\x82\x82a\x17\x83V[PP`\x04T\x96\x95PPPPPPV[a\x06la\x0C\x1FV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FCannot set zero address as sende`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x01a\x04#V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[____a\x073\x88\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8A\x92Pa\x0C\xF1\x91PPV[`\x04\x82\x90U\x90\x92P\x90P\x84`\x05a\x07J\x82\x82a\x17\x83V[PP`\x04T\x98\x90\x97P\x95PPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOnly blocklock contract can call`D\x82\x01R`d\x01a\x04#V[a\x05\x87\x83\x83\x83a\x0E+V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru&\xBA\xB9\xBA\x1012\x90897\xB87\xB9\xB2\xB2\x107\xBB\xB72\xB9`Q\x1B`D\x82\x01R`d\x01a\x04#V[_\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x80\x83\x16\x82\x17\x84U`\x01\x80T\x90\x91\x16\x90U`@Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x90\x91\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\xA3PV[a\x08ra\x0C\x1FV[`\x03\x81\x90U`@Q\x81\x90\x7F_G\x9A\xC9\x93\x92^\xFA\xE289\xAA\x1C\x81;n\xCB/\xE1\xBB\xD1M\xC7\x82\x95\xAB&}/\xCACh\x90_\x90\xA2PV[`@\x80Q`\x80\x81\x01\x80\x83R`\x05\x92\x83\x91\x83\x91\x82\x01\x90\x83\x90`\x02\x90\x82\x84[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xC1WPPP\x91\x83RPP`@\x80Q\x80\x82\x01\x91\x82\x90R` \x90\x92\x01\x91\x90`\x02\x84\x81\x01\x91\x82\x84[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xF7WPPPPP\x81RPP\x90\x80`\x04\x01\x80Ta\t#\x90a\x16MV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tO\x90a\x16MV[\x80\x15a\t\x9AW\x80`\x1F\x10a\tqWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x9AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t}W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x05\x01\x80Ta\t\xAF\x90a\x16MV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xDB\x90a\x16MV[\x80\x15a\n&W\x80`\x1F\x10a\t\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n&V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\tW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[a\n8a\x0C\x1FV[a\nA\x81a\x103V[PV[a\nLa\x0C\x1FV[\x81G\x10\x15a\n\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInsufficient funds in contract\0\0`D\x82\x01R`d\x01a\x04#V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x83\x15a\x08\xFC\x02\x90\x84\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\n\xCFW=__>=_\xFD[P\x80`\x01`\x01`\xA0\x1B\x03\x16\x7Fp\x84\xF5Gf\x18\xD8\xE6\x0B\x11\xEF\r}?\x06\x91FU\xAD\xB8y>(\xFF\x7F\x01\x8DLv\xD5\x05\xD5\x83`@Qa\x0B\x0B\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[a\x0B\x1Fa\x0C\x1FV[a\x0B'a\x10\xCBV[`\x03\x81\x90U`\x02T`@Qc%mW?`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90c\x95\xB5\\\xFC\x904\x90`$\x01a\x04_V[`\x02T`@Qc\xCD\x80,\x91`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCD\x80,\x91\x90`$\x01a\x05\xB9V[_4\x11a\x0B\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x0B-\xEE\xA4\r\xAE\xAEn\x84\x0El\xAD\xCC\x84\x0Em\xED\xAC\xA4\x08\xAA\x89`S\x1B`D\x82\x01R`d\x01a\x04#V[`@Q4\x81R3\x90\x7FZ\xF8\x18K\xEF\x8EKE\xEB\x9Fn\xD7sM\x04\xDA8\xCE\xD2&IUH\xF4n\x0C\x8F\xF8\xD7\xD9\xA5$\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x0C\x16a\x0C\x1FV[a\nA\x81a\x11\xF5V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0CqW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru'\xB76<\x901\xB0\xB660\xB162\x901<\x907\xBB\xB72\xB9`Q\x1B`D\x82\x01R`d\x01a\x04#V[V[`\x02T`\x03T`@Qbb\xB1\xD9`\xE4\x1B\x81R_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x06+\x1D\x90\x91a\x0C\xA9\x91\x88\x91\x88\x90\x88\x90`\x04\x01a\x18\xC4V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\xC5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE9\x91\x90a\x19\x05V[\x94\x93PPPPV[`\x02T`@QcK\x16\t5`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R_\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cK\x16\t5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\re\x91\x90a\x19\x05V[\x90P\x804\x10\x15a\r\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\t-\xCEn\xAC\xCC\xCD,m,\xAD\xCE\x84\x08\xAA\x89`\x83\x1B`D\x82\x01R`d\x01a\x04#V[`\x02T`@Qcy\t\xDC{`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cy\t\xDC{\x904\x90a\r\xE0\x90\x89\x90\x89\x90\x89\x90`\x04\x01a\x19\x1CV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\r\xFCW=__>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E!\x91\x90a\x19\x05V[\x91P\x93P\x93\x91PPV[\x82`\x04T\x14a\x0ErW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr$\xB7;0\xB64\xB2\x1092\xB8\xBA\xB2\xB9\xBA\x104\xB2\x17`i\x1B`D\x82\x01R`d\x01a\x04#V[`@\x80Q`\xE0\x81\x01\x90\x91Ra\x10\x18\x90`\x05\x81``\x81\x01\x82\x81`\xA0\x84\x01\x82`\x02\x82\x82\x82` \x02\x82\x01\x91[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0E\x9BWPPP\x91\x83RPP`@\x80Q\x80\x82\x01\x91\x82\x90R` \x90\x92\x01\x91\x90`\x02\x84\x81\x01\x91\x82\x84[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0E\xD1WPPPPP\x81RPP\x81R` \x01`\x04\x82\x01\x80Ta\x0F\x01\x90a\x16MV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F-\x90a\x16MV[\x80\x15a\x0FxW\x80`\x1F\x10a\x0FOWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0FxV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F[W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x05\x82\x01\x80Ta\x0F\x91\x90a\x16MV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\xBD\x90a\x16MV[\x80\x15a\x10\x08W\x80`\x1F\x10a\x0F\xDFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\x08V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F\xEBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x83\x83a\x12\x9DV[\x80` \x01\x90Q\x81\x01\x90a\x10+\x91\x90a\x19\x05V[`\x0BUPPPV[`\x03T_\x03a\x10}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuSubscriptionId is zero`P\x1B`D\x82\x01R`d\x01a\x04#V[`\x02T`\x03T`@Qb+\x82U`\xE6\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R\x90\x91\x16\x90c\n\xE0\x95@\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04vW__\xFD[_`\x03T_\x14a\x11\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FSubscriptionId is not zero\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04#V[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xA2\x1A#\xE4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x92\x91\x90a\x19\x05V[`\x02T`@Qc/\xB10#`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xBE\xC4\xC0\x8C\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11\xDCW__\xFD[PZ\xF1\x15\x80\x15a\x11\xEEW=__>=_\xFD[PPPP\x90V[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\x12MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCannot transfer to self\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04#V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x82\x17\x90\x92U_\x80T`@Q\x92\x93\x16\x91\x7F\xED\x88\x89\xF5`2n\xB18\x92\r\x84!\x92\xF0\xEB=\xD2+O\x13\x9C\x87\xA2\xC5u8\xE0[\xAE\x12x\x91\x90\xA3PV[`\x02T`@Qc2o\x061`\xE2\x1B\x81R``\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC9\xBC\x18\xC4\x90a\x12\xD2\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x19LV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xECW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C\xE9\x91\x90\x81\x01\x90a\x19\xABV[__` \x83\x85\x03\x12\x15a\x13$W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13:W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x13JW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13`W__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x13tW__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_` \x82\x84\x03\x12\x15a\x13\x94W__\xFD[P5\x91\x90PV[__\x83`\x1F\x84\x01\x12a\x13\xABW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xC2W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x13\xD9W__\xFD[\x92P\x92\x90PV[____``\x85\x87\x03\x12\x15a\x13\xF3W__\xFD[\x845c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x14\x06W__\xFD[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14!W__\xFD[a\x14-\x87\x82\x88\x01a\x13\x9BV[\x90\x94P\x92PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14LW__\xFD[\x85\x01`\xC0\x81\x88\x03\x12\x15a\x14]W__\xFD[\x93\x96\x92\x95P\x90\x93PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x14~W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x14\x93W__\xFD[a\x14\x9C\x82a\x14hV[\x93\x92PPPV[___`@\x84\x86\x03\x12\x15a\x14\xB5W__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xD2W__\xFD[a\x14\xDE\x86\x82\x87\x01a\x13\x9BV[\x94\x97\x90\x96P\x93\x94PPPPV[\x80_[`\x02\x81\x10\x15a\x15\rW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x14\xEEV[PPPPV[a\x15\x1E\x82\x82Qa\x14\xEBV[` \x81\x01Qa\x05\x87`@\x84\x01\x82a\x14\xEBV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[a\x15h\x81\x85a\x15\x13V[`\xC0`\x80\x82\x01R_a\x15}`\xC0\x83\x01\x85a\x150V[\x82\x81\x03`\xA0\x84\x01Ra\x15\x8F\x81\x85a\x150V[\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15a\x15\xAAW__\xFD[\x825\x91Pa\x15\xBA` \x84\x01a\x14hV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x15\xE7W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x14\x9CW__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x16\x1FW__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x169W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x13\xD9W__\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x16aW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x16\x7FWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x05\x87W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x16\xAAWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x04\x88W_\x81U`\x01\x01a\x16\xB6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x16\xE1Wa\x16\xE1a\x15\xF6V[a\x16\xF5\x83a\x16\xEF\x83Ta\x16MV[\x83a\x16\x85V[_`\x1F\x84\x11`\x01\x81\x14a\x17&W_\x85\x15a\x17\x0FWP\x83\x82\x015[_\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x04\x88V[_\x83\x81R` \x81 `\x1F\x19\x87\x16\x91[\x82\x81\x10\x15a\x17UW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x175V[P\x86\x82\x10\x15a\x17qW_\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[\x81_[`\x02\x81\x10\x15a\x17\xA3W\x815\x83\x82\x01U` \x90\x91\x01\x90`\x01\x01a\x17\x86V[PP`@\x82\x01_[`\x02\x81\x10\x15a\x17\xCBW\x815\x83\x82\x01`\x02\x01U` \x90\x91\x01\x90`\x01\x01a\x17\xABV[PPa\x17\xDA`\x80\x83\x01\x83a\x16\nV[a\x17\xE8\x81\x83`\x04\x86\x01a\x16\xC9V[PPa\x17\xF7`\xA0\x83\x01\x83a\x16\nV[a\x15\r\x81\x83`\x05\x86\x01a\x16\xC9V[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x18\x1AW__\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x189W__\xFD[\x806\x03\x82\x13\x15a\x13\xD9W__\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`@\x81\x837`@\x80\x82\x01`@\x84\x017_a\x18\x8C`\x80\x83\x01\x83a\x18\x05V[`\xC0`\x80\x86\x01Ra\x18\xA1`\xC0\x86\x01\x82\x84a\x18GV[\x91PPa\x18\xB1`\xA0\x84\x01\x84a\x18\x05V[\x85\x83\x03`\xA0\x87\x01Ra\x15\x8F\x83\x82\x84a\x18GV[c\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_a\x18\xE8`\x80\x83\x01\x85a\x150V[\x82\x81\x03``\x84\x01Ra\x18\xFA\x81\x85a\x18oV[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a\x19\x15W__\xFD[PQ\x91\x90PV[c\xFF\xFF\xFF\xFF\x84\x16\x81R``` \x82\x01R_a\x19:``\x83\x01\x85a\x150V[\x82\x81\x03`@\x84\x01Ra\x15\x8F\x81\x85a\x18oV[`@\x81Ra\x19^`@\x82\x01\x85Qa\x15\x13V[_` \x85\x01Q`\xC0\x80\x84\x01Ra\x19xa\x01\0\x84\x01\x82a\x150V[\x90P`@\x86\x01Q`?\x19\x84\x83\x03\x01`\xE0\x85\x01Ra\x19\x95\x82\x82a\x150V[\x91PP\x82\x81\x03` \x84\x01Ra\x15\x8F\x81\x85\x87a\x18GV[_` \x82\x84\x03\x12\x15a\x19\xBBW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xD1W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x19\xE1W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xFBWa\x19\xFBa\x15\xF6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A*Wa\x1A*a\x15\xF6V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x1AAW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Funded(address,uint256)` and selector `0x5af8184bef8e4b45eb9f6ed7734d04da38ced226495548f46e0c8ff8d7d9a524`.
```solidity
event Funded(address indexed sender, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Funded {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Funded {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Funded(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                90u8, 248u8, 24u8, 75u8, 239u8, 142u8, 75u8, 69u8, 235u8, 159u8, 110u8,
                215u8, 115u8, 77u8, 4u8, 218u8, 56u8, 206u8, 210u8, 38u8, 73u8, 85u8,
                72u8, 244u8, 110u8, 12u8, 143u8, 248u8, 215u8, 217u8, 165u8, 36u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    sender: topics.1,
                    amount: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.sender.clone())
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
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Funded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Funded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Funded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `NewSubscriptionId(uint256)` and selector `0x5f479ac993925efae23839aa1c813b6ecb2fe1bbd14dc78295ab267d2fca4368`.
```solidity
event NewSubscriptionId(uint256 indexed subscriptionId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewSubscriptionId {
        #[allow(missing_docs)]
        pub subscriptionId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for NewSubscriptionId {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "NewSubscriptionId(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                95u8, 71u8, 154u8, 201u8, 147u8, 146u8, 94u8, 250u8, 226u8, 56u8, 57u8,
                170u8, 28u8, 129u8, 59u8, 110u8, 203u8, 47u8, 225u8, 187u8, 209u8, 77u8,
                199u8, 130u8, 149u8, 171u8, 38u8, 125u8, 47u8, 202u8, 67u8, 104u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { subscriptionId: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.subscriptionId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.subscriptionId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewSubscriptionId {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewSubscriptionId> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewSubscriptionId) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferRequested(address,address)` and selector `0xed8889f560326eb138920d842192f0eb3dd22b4f139c87a2c57538e05bae1278`.
```solidity
event OwnershipTransferRequested(address indexed from, address indexed to);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferRequested {
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
        impl alloy_sol_types::SolEvent for OwnershipTransferRequested {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferRequested(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                237u8, 136u8, 137u8, 245u8, 96u8, 50u8, 110u8, 177u8, 56u8, 146u8, 13u8,
                132u8, 33u8, 146u8, 240u8, 235u8, 61u8, 210u8, 43u8, 79u8, 19u8, 156u8,
                135u8, 162u8, 197u8, 117u8, 56u8, 224u8, 91u8, 174u8, 18u8, 120u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    from: topics.1,
                    to: topics.2,
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
                (Self::SIGNATURE_HASH.into(), self.from.clone(), self.to.clone())
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
                    &self.from,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferRequested {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferRequested> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OwnershipTransferRequested,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed from, address indexed to);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
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
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    from: topics.1,
                    to: topics.2,
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
                (Self::SIGNATURE_HASH.into(), self.from.clone(), self.to.clone())
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
                    &self.from,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Received(address,uint256)` and selector `0x88a5966d370b9919b20f3e2c13ff65706f196a4e32cc2c12bf57088f88525874`.
```solidity
event Received(address, uint256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Received {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Received {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Received(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                136u8, 165u8, 150u8, 109u8, 55u8, 11u8, 153u8, 25u8, 178u8, 15u8, 62u8,
                44u8, 19u8, 255u8, 101u8, 112u8, 111u8, 25u8, 106u8, 78u8, 50u8, 204u8,
                44u8, 18u8, 191u8, 87u8, 8u8, 143u8, 136u8, 82u8, 88u8, 116u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0, _1: data.1 }
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
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
        impl alloy_sol_types::private::IntoLogData for Received {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Received> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Received) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Withdrawn(address,uint256)` and selector `0x7084f5476618d8e60b11ef0d7d3f06914655adb8793e28ff7f018d4c76d505d5`.
```solidity
event Withdrawn(address indexed recipient, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Withdrawn {
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Withdrawn {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Withdrawn(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                112u8, 132u8, 245u8, 71u8, 102u8, 24u8, 216u8, 230u8, 11u8, 17u8, 239u8,
                13u8, 125u8, 63u8, 6u8, 145u8, 70u8, 85u8, 173u8, 184u8, 121u8, 62u8,
                40u8, 255u8, 127u8, 1u8, 141u8, 76u8, 118u8, 213u8, 5u8, 213u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    recipient: topics.1,
                    amount: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.recipient.clone())
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
                    &self.recipient,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Withdrawn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Withdrawn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Withdrawn) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address blocklockContract);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub blocklockContract: alloy::sol_types::private::Address,
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
                    (value.blocklockContract,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { blocklockContract: tuple.0 }
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
                        &self.blocklockContract,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `acceptOwnership()` and selector `0x79ba5097`.
```solidity
function acceptOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptOwnershipCall;
    ///Container type for the return parameters of the [`acceptOwnership()`](acceptOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptOwnershipReturn {}
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
            impl ::core::convert::From<acceptOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: acceptOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for acceptOwnershipCall {
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
            impl ::core::convert::From<acceptOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl acceptOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <acceptOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = acceptOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "acceptOwnership()";
            const SELECTOR: [u8; 4] = [121u8, 186u8, 80u8, 151u8];
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
                acceptOwnershipReturn::_tokenize(ret)
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
    /**Function with signature `blocklock()` and selector `0x93b9740a`.
```solidity
function blocklock() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blocklockCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`blocklock()`](blocklockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blocklockReturn {
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
            impl ::core::convert::From<blocklockCall> for UnderlyingRustTuple<'_> {
                fn from(value: blocklockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blocklockCall {
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
            impl ::core::convert::From<blocklockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: blocklockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blocklockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for blocklockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "blocklock()";
            const SELECTOR: [u8; 4] = [147u8, 185u8, 116u8, 10u8];
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
                        let r: blocklockReturn = r.into();
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
                        let r: blocklockReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `cancelSubscription(address)` and selector `0x93d81d58`.
```solidity
function cancelSubscription(address to) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelSubscriptionCall {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`cancelSubscription(address)`](cancelSubscriptionCall) function.
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
            impl ::core::convert::From<cancelSubscriptionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelSubscriptionCall) -> Self {
                    (value.to,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelSubscriptionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { to: tuple.0 }
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
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelSubscriptionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancelSubscription(address)";
            const SELECTOR: [u8; 4] = [147u8, 216u8, 29u8, 88u8];
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
    /**Function with signature `createSubscriptionAndFundNative()` and selector `0xb96dbba7`.
```solidity
function createSubscriptionAndFundNative() external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createSubscriptionAndFundNativeCall;
    ///Container type for the return parameters of the [`createSubscriptionAndFundNative()`](createSubscriptionAndFundNativeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createSubscriptionAndFundNativeReturn {}
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
            impl ::core::convert::From<createSubscriptionAndFundNativeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createSubscriptionAndFundNativeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createSubscriptionAndFundNativeCall {
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
            impl ::core::convert::From<createSubscriptionAndFundNativeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createSubscriptionAndFundNativeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createSubscriptionAndFundNativeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl createSubscriptionAndFundNativeReturn {
            fn _tokenize(
                &self,
            ) -> <createSubscriptionAndFundNativeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createSubscriptionAndFundNativeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createSubscriptionAndFundNativeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createSubscriptionAndFundNative()";
            const SELECTOR: [u8; 4] = [185u8, 109u8, 187u8, 167u8];
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
                createSubscriptionAndFundNativeReturn::_tokenize(ret)
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
    /**Function with signature `createTimelockRequestWithDirectFunding(uint32,bytes,((uint256[2],uint256[2]),bytes,bytes))` and selector `0x59608fda`.
```solidity
function createTimelockRequestWithDirectFunding(uint32 callbackGasLimit, bytes memory condition, TypesLib.Ciphertext memory encryptedData) external payable returns (uint256, uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTimelockRequestWithDirectFundingCall {
        #[allow(missing_docs)]
        pub callbackGasLimit: u32,
        #[allow(missing_docs)]
        pub condition: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub encryptedData: <TypesLib::Ciphertext as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`createTimelockRequestWithDirectFunding(uint32,bytes,((uint256[2],uint256[2]),bytes,bytes))`](createTimelockRequestWithDirectFundingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTimelockRequestWithDirectFundingReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
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
                alloy::sol_types::sol_data::Bytes,
                TypesLib::Ciphertext,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
                alloy::sol_types::private::Bytes,
                <TypesLib::Ciphertext as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createTimelockRequestWithDirectFundingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createTimelockRequestWithDirectFundingCall) -> Self {
                    (value.callbackGasLimit, value.condition, value.encryptedData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createTimelockRequestWithDirectFundingCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        callbackGasLimit: tuple.0,
                        condition: tuple.1,
                        encryptedData: tuple.2,
                    }
                }
            }
        }
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
            impl ::core::convert::From<createTimelockRequestWithDirectFundingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createTimelockRequestWithDirectFundingReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createTimelockRequestWithDirectFundingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        impl createTimelockRequestWithDirectFundingReturn {
            fn _tokenize(
                &self,
            ) -> <createTimelockRequestWithDirectFundingCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createTimelockRequestWithDirectFundingCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
                TypesLib::Ciphertext,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createTimelockRequestWithDirectFundingReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createTimelockRequestWithDirectFunding(uint32,bytes,((uint256[2],uint256[2]),bytes,bytes))";
            const SELECTOR: [u8; 4] = [89u8, 96u8, 143u8, 218u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.condition,
                    ),
                    <TypesLib::Ciphertext as alloy_sol_types::SolType>::tokenize(
                        &self.encryptedData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                createTimelockRequestWithDirectFundingReturn::_tokenize(ret)
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
    /**Function with signature `createTimelockRequestWithSubscription(uint32,bytes,((uint256[2],uint256[2]),bytes,bytes))` and selector `0x4d3de353`.
```solidity
function createTimelockRequestWithSubscription(uint32 callbackGasLimit, bytes memory condition, TypesLib.Ciphertext memory encryptedData) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTimelockRequestWithSubscriptionCall {
        #[allow(missing_docs)]
        pub callbackGasLimit: u32,
        #[allow(missing_docs)]
        pub condition: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub encryptedData: <TypesLib::Ciphertext as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`createTimelockRequestWithSubscription(uint32,bytes,((uint256[2],uint256[2]),bytes,bytes))`](createTimelockRequestWithSubscriptionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTimelockRequestWithSubscriptionReturn {
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
                alloy::sol_types::sol_data::Bytes,
                TypesLib::Ciphertext,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
                alloy::sol_types::private::Bytes,
                <TypesLib::Ciphertext as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createTimelockRequestWithSubscriptionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createTimelockRequestWithSubscriptionCall) -> Self {
                    (value.callbackGasLimit, value.condition, value.encryptedData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createTimelockRequestWithSubscriptionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        callbackGasLimit: tuple.0,
                        condition: tuple.1,
                        encryptedData: tuple.2,
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
            impl ::core::convert::From<createTimelockRequestWithSubscriptionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createTimelockRequestWithSubscriptionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createTimelockRequestWithSubscriptionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createTimelockRequestWithSubscriptionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
                TypesLib::Ciphertext,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createTimelockRequestWithSubscription(uint32,bytes,((uint256[2],uint256[2]),bytes,bytes))";
            const SELECTOR: [u8; 4] = [77u8, 61u8, 227u8, 83u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.condition,
                    ),
                    <TypesLib::Ciphertext as alloy_sol_types::SolType>::tokenize(
                        &self.encryptedData,
                    ),
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
                        let r: createTimelockRequestWithSubscriptionReturn = r.into();
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
                        let r: createTimelockRequestWithSubscriptionReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `encryptedValue()` and selector `0x86e560be`.
```solidity
function encryptedValue() external view returns (BLS.PointG2 memory u, bytes memory v, bytes memory w);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct encryptedValueCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`encryptedValue()`](encryptedValueCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct encryptedValueReturn {
        #[allow(missing_docs)]
        pub u: <BLS::PointG2 as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub v: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub w: alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<encryptedValueCall> for UnderlyingRustTuple<'_> {
                fn from(value: encryptedValueCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for encryptedValueCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                BLS::PointG2,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BLS::PointG2 as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<encryptedValueReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: encryptedValueReturn) -> Self {
                    (value.u, value.v, value.w)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for encryptedValueReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        u: tuple.0,
                        v: tuple.1,
                        w: tuple.2,
                    }
                }
            }
        }
        impl encryptedValueReturn {
            fn _tokenize(
                &self,
            ) -> <encryptedValueCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <BLS::PointG2 as alloy_sol_types::SolType>::tokenize(&self.u),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.v,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.w,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for encryptedValueCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = encryptedValueReturn;
            type ReturnTuple<'a> = (
                BLS::PointG2,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "encryptedValue()";
            const SELECTOR: [u8; 4] = [134u8, 229u8, 96u8, 190u8];
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
                encryptedValueReturn::_tokenize(ret)
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
    /**Function with signature `fundContractNative()` and selector `0xe64a66ea`.
```solidity
function fundContractNative() external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fundContractNativeCall;
    ///Container type for the return parameters of the [`fundContractNative()`](fundContractNativeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fundContractNativeReturn {}
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
            impl ::core::convert::From<fundContractNativeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: fundContractNativeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fundContractNativeCall {
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
            impl ::core::convert::From<fundContractNativeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: fundContractNativeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fundContractNativeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl fundContractNativeReturn {
            fn _tokenize(
                &self,
            ) -> <fundContractNativeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for fundContractNativeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = fundContractNativeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "fundContractNative()";
            const SELECTOR: [u8; 4] = [230u8, 74u8, 102u8, 234u8];
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
                fundContractNativeReturn::_tokenize(ret)
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
    /**Function with signature `getBalance()` and selector `0x12065fe0`.
```solidity
function getBalance() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBalanceCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getBalance()`](getBalanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBalanceReturn {
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
            impl ::core::convert::From<getBalanceCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBalanceCall {
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
            impl ::core::convert::From<getBalanceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBalanceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBalance()";
            const SELECTOR: [u8; 4] = [18u8, 6u8, 95u8, 224u8];
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
                        let r: getBalanceReturn = r.into();
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
                        let r: getBalanceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isInFlight(uint256)` and selector `0xcd802c91`.
```solidity
function isInFlight(uint256 requestId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isInFlightCall {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
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
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isInFlightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
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
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
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
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
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
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
                        let r: ownerReturn = r.into();
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
                        let r: ownerReturn = r.into();
                        r._0
                    })
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
    /**Function with signature `plainTextValue()` and selector `0x97a9c285`.
```solidity
function plainTextValue() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct plainTextValueCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`plainTextValue()`](plainTextValueCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct plainTextValueReturn {
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
            impl ::core::convert::From<plainTextValueCall> for UnderlyingRustTuple<'_> {
                fn from(value: plainTextValueCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for plainTextValueCall {
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
            impl ::core::convert::From<plainTextValueReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: plainTextValueReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for plainTextValueReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for plainTextValueCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "plainTextValue()";
            const SELECTOR: [u8; 4] = [151u8, 169u8, 194u8, 133u8];
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
                        let r: plainTextValueReturn = r.into();
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
                        let r: plainTextValueReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `receiveBlocklock(uint256,bytes)` and selector `0x5d941802`.
```solidity
function receiveBlocklock(uint256 requestId, bytes memory decryptionKey) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct receiveBlocklockCall {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decryptionKey: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`receiveBlocklock(uint256,bytes)`](receiveBlocklockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct receiveBlocklockReturn {}
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
            impl ::core::convert::From<receiveBlocklockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: receiveBlocklockCall) -> Self {
                    (value.requestId, value.decryptionKey)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for receiveBlocklockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        requestId: tuple.0,
                        decryptionKey: tuple.1,
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
            impl ::core::convert::From<receiveBlocklockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: receiveBlocklockReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for receiveBlocklockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl receiveBlocklockReturn {
            fn _tokenize(
                &self,
            ) -> <receiveBlocklockCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for receiveBlocklockCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = receiveBlocklockReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "receiveBlocklock(uint256,bytes)";
            const SELECTOR: [u8; 4] = [93u8, 148u8, 24u8, 2u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.decryptionKey,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                receiveBlocklockReturn::_tokenize(ret)
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
    /**Function with signature `requestId()` and selector `0x006d6cae`.
```solidity
function requestId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`requestId()`](requestIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestIdReturn {
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
            impl ::core::convert::From<requestIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: requestIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for requestIdCall {
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
            impl ::core::convert::From<requestIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: requestIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for requestIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "requestId()";
            const SELECTOR: [u8; 4] = [0u8, 109u8, 108u8, 174u8];
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
                        let r: requestIdReturn = r.into();
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
                        let r: requestIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setBlocklock(address)` and selector `0x4fa26d40`.
```solidity
function setBlocklock(address _blocklock) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBlocklockCall {
        #[allow(missing_docs)]
        pub _blocklock: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setBlocklock(address)`](setBlocklockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBlocklockReturn {}
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
            impl ::core::convert::From<setBlocklockCall> for UnderlyingRustTuple<'_> {
                fn from(value: setBlocklockCall) -> Self {
                    (value._blocklock,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setBlocklockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _blocklock: tuple.0 }
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
            impl ::core::convert::From<setBlocklockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setBlocklockReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setBlocklockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setBlocklockReturn {
            fn _tokenize(
                &self,
            ) -> <setBlocklockCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setBlocklockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setBlocklockReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setBlocklock(address)";
            const SELECTOR: [u8; 4] = [79u8, 162u8, 109u8, 64u8];
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
                        &self._blocklock,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setBlocklockReturn::_tokenize(ret)
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
    /**Function with signature `setSubId(uint256)` and selector `0x80980043`.
```solidity
function setSubId(uint256 subId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSubIdCall {
        #[allow(missing_docs)]
        pub subId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setSubId(uint256)`](setSubIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSubIdReturn {}
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
            impl ::core::convert::From<setSubIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: setSubIdCall) -> Self {
                    (value.subId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setSubIdCall {
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
            impl ::core::convert::From<setSubIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setSubIdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setSubIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setSubIdReturn {
            fn _tokenize(
                &self,
            ) -> <setSubIdCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setSubIdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setSubIdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setSubId(uint256)";
            const SELECTOR: [u8; 4] = [128u8, 152u8, 0u8, 67u8];
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
                setSubIdReturn::_tokenize(ret)
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
    /**Function with signature `subscriptionId()` and selector `0x09c1ba2e`.
```solidity
function subscriptionId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct subscriptionIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`subscriptionId()`](subscriptionIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct subscriptionIdReturn {
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
            impl ::core::convert::From<subscriptionIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: subscriptionIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for subscriptionIdCall {
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
            impl ::core::convert::From<subscriptionIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: subscriptionIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for subscriptionIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for subscriptionIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "subscriptionId()";
            const SELECTOR: [u8; 4] = [9u8, 193u8, 186u8, 46u8];
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
                        let r: subscriptionIdReturn = r.into();
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
                        let r: subscriptionIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `topUpSubscriptionNative()` and selector `0x1d2b2afd`.
```solidity
function topUpSubscriptionNative() external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct topUpSubscriptionNativeCall;
    ///Container type for the return parameters of the [`topUpSubscriptionNative()`](topUpSubscriptionNativeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct topUpSubscriptionNativeReturn {}
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
            impl ::core::convert::From<topUpSubscriptionNativeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: topUpSubscriptionNativeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for topUpSubscriptionNativeCall {
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
            impl ::core::convert::From<topUpSubscriptionNativeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: topUpSubscriptionNativeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for topUpSubscriptionNativeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl topUpSubscriptionNativeReturn {
            fn _tokenize(
                &self,
            ) -> <topUpSubscriptionNativeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for topUpSubscriptionNativeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = topUpSubscriptionNativeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "topUpSubscriptionNative()";
            const SELECTOR: [u8; 4] = [29u8, 43u8, 42u8, 253u8];
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
                topUpSubscriptionNativeReturn::_tokenize(ret)
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
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address to) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
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
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.to,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { to: tuple.0 }
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
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl transferOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <transferOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
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
                        &self.to,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                transferOwnershipReturn::_tokenize(ret)
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
    /**Function with signature `updateSubscription(address[])` and selector `0x36bfffed`.
```solidity
function updateSubscription(address[] memory consumers) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateSubscriptionCall {
        #[allow(missing_docs)]
        pub consumers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`updateSubscription(address[])`](updateSubscriptionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateSubscriptionReturn {}
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
            impl ::core::convert::From<updateSubscriptionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateSubscriptionCall) -> Self {
                    (value.consumers,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateSubscriptionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { consumers: tuple.0 }
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
            impl ::core::convert::From<updateSubscriptionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateSubscriptionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateSubscriptionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateSubscriptionReturn {
            fn _tokenize(
                &self,
            ) -> <updateSubscriptionCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateSubscriptionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateSubscriptionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateSubscription(address[])";
            const SELECTOR: [u8; 4] = [54u8, 191u8, 255u8, 237u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.consumers),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateSubscriptionReturn::_tokenize(ret)
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
    /**Function with signature `withdrawNative(uint256,address)` and selector `0xb8ca8dd8`.
```solidity
function withdrawNative(uint256 amount, address recipient) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawNativeCall {
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`withdrawNative(uint256,address)`](withdrawNativeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawNativeReturn {}
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
            impl ::core::convert::From<withdrawNativeCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawNativeCall) -> Self {
                    (value.amount, value.recipient)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawNativeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amount: tuple.0,
                        recipient: tuple.1,
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
            impl ::core::convert::From<withdrawNativeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawNativeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawNativeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl withdrawNativeReturn {
            fn _tokenize(
                &self,
            ) -> <withdrawNativeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawNativeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawNativeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawNative(uint256,address)";
            const SELECTOR: [u8; 4] = [184u8, 202u8, 141u8, 216u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                withdrawNativeReturn::_tokenize(ret)
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
    ///Container for all the [`MockBlocklockReceiver`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum MockBlocklockReceiverCalls {
        #[allow(missing_docs)]
        acceptOwnership(acceptOwnershipCall),
        #[allow(missing_docs)]
        blocklock(blocklockCall),
        #[allow(missing_docs)]
        cancelSubscription(cancelSubscriptionCall),
        #[allow(missing_docs)]
        createSubscriptionAndFundNative(createSubscriptionAndFundNativeCall),
        #[allow(missing_docs)]
        createTimelockRequestWithDirectFunding(
            createTimelockRequestWithDirectFundingCall,
        ),
        #[allow(missing_docs)]
        createTimelockRequestWithSubscription(createTimelockRequestWithSubscriptionCall),
        #[allow(missing_docs)]
        encryptedValue(encryptedValueCall),
        #[allow(missing_docs)]
        fundContractNative(fundContractNativeCall),
        #[allow(missing_docs)]
        getBalance(getBalanceCall),
        #[allow(missing_docs)]
        isInFlight(isInFlightCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        pendingRequestExists(pendingRequestExistsCall),
        #[allow(missing_docs)]
        plainTextValue(plainTextValueCall),
        #[allow(missing_docs)]
        receiveBlocklock(receiveBlocklockCall),
        #[allow(missing_docs)]
        requestId(requestIdCall),
        #[allow(missing_docs)]
        setBlocklock(setBlocklockCall),
        #[allow(missing_docs)]
        setSubId(setSubIdCall),
        #[allow(missing_docs)]
        subscriptionId(subscriptionIdCall),
        #[allow(missing_docs)]
        topUpSubscriptionNative(topUpSubscriptionNativeCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        updateSubscription(updateSubscriptionCall),
        #[allow(missing_docs)]
        withdrawNative(withdrawNativeCall),
    }
    impl MockBlocklockReceiverCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 109u8, 108u8, 174u8],
            [9u8, 193u8, 186u8, 46u8],
            [18u8, 6u8, 95u8, 224u8],
            [29u8, 43u8, 42u8, 253u8],
            [54u8, 191u8, 255u8, 237u8],
            [65u8, 175u8, 108u8, 135u8],
            [77u8, 61u8, 227u8, 83u8],
            [79u8, 162u8, 109u8, 64u8],
            [89u8, 96u8, 143u8, 218u8],
            [93u8, 148u8, 24u8, 2u8],
            [121u8, 186u8, 80u8, 151u8],
            [128u8, 152u8, 0u8, 67u8],
            [134u8, 229u8, 96u8, 190u8],
            [141u8, 165u8, 203u8, 91u8],
            [147u8, 185u8, 116u8, 10u8],
            [147u8, 216u8, 29u8, 88u8],
            [151u8, 169u8, 194u8, 133u8],
            [184u8, 202u8, 141u8, 216u8],
            [185u8, 109u8, 187u8, 167u8],
            [205u8, 128u8, 44u8, 145u8],
            [230u8, 74u8, 102u8, 234u8],
            [242u8, 253u8, 227u8, 139u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(requestId),
            ::core::stringify!(subscriptionId),
            ::core::stringify!(getBalance),
            ::core::stringify!(topUpSubscriptionNative),
            ::core::stringify!(updateSubscription),
            ::core::stringify!(pendingRequestExists),
            ::core::stringify!(createTimelockRequestWithSubscription),
            ::core::stringify!(setBlocklock),
            ::core::stringify!(createTimelockRequestWithDirectFunding),
            ::core::stringify!(receiveBlocklock),
            ::core::stringify!(acceptOwnership),
            ::core::stringify!(setSubId),
            ::core::stringify!(encryptedValue),
            ::core::stringify!(owner),
            ::core::stringify!(blocklock),
            ::core::stringify!(cancelSubscription),
            ::core::stringify!(plainTextValue),
            ::core::stringify!(withdrawNative),
            ::core::stringify!(createSubscriptionAndFundNative),
            ::core::stringify!(isInFlight),
            ::core::stringify!(fundContractNative),
            ::core::stringify!(transferOwnership),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <requestIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <subscriptionIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getBalanceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <topUpSubscriptionNativeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <updateSubscriptionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pendingRequestExistsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <createTimelockRequestWithSubscriptionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setBlocklockCall as alloy_sol_types::SolCall>::SIGNATURE,
            <createTimelockRequestWithDirectFundingCall as alloy_sol_types::SolCall>::SIGNATURE,
            <receiveBlocklockCall as alloy_sol_types::SolCall>::SIGNATURE,
            <acceptOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setSubIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <encryptedValueCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <blocklockCall as alloy_sol_types::SolCall>::SIGNATURE,
            <cancelSubscriptionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <plainTextValueCall as alloy_sol_types::SolCall>::SIGNATURE,
            <withdrawNativeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <createSubscriptionAndFundNativeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isInFlightCall as alloy_sol_types::SolCall>::SIGNATURE,
            <fundContractNativeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for MockBlocklockReceiverCalls {
        const NAME: &'static str = "MockBlocklockReceiverCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 22usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::acceptOwnership(_) => {
                    <acceptOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blocklock(_) => {
                    <blocklockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancelSubscription(_) => {
                    <cancelSubscriptionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createSubscriptionAndFundNative(_) => {
                    <createSubscriptionAndFundNativeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createTimelockRequestWithDirectFunding(_) => {
                    <createTimelockRequestWithDirectFundingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createTimelockRequestWithSubscription(_) => {
                    <createTimelockRequestWithSubscriptionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::encryptedValue(_) => {
                    <encryptedValueCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::fundContractNative(_) => {
                    <fundContractNativeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBalance(_) => {
                    <getBalanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isInFlight(_) => {
                    <isInFlightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pendingRequestExists(_) => {
                    <pendingRequestExistsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::plainTextValue(_) => {
                    <plainTextValueCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::receiveBlocklock(_) => {
                    <receiveBlocklockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::requestId(_) => {
                    <requestIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setBlocklock(_) => {
                    <setBlocklockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setSubId(_) => <setSubIdCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::subscriptionId(_) => {
                    <subscriptionIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::topUpSubscriptionNative(_) => {
                    <topUpSubscriptionNativeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateSubscription(_) => {
                    <updateSubscriptionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawNative(_) => {
                    <withdrawNativeCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls>] = &[
                {
                    fn requestId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <requestIdCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(MockBlocklockReceiverCalls::requestId)
                    }
                    requestId
                },
                {
                    fn subscriptionId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <subscriptionIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::subscriptionId)
                    }
                    subscriptionId
                },
                {
                    fn getBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <getBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::getBalance)
                    }
                    getBalance
                },
                {
                    fn topUpSubscriptionNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <topUpSubscriptionNativeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::topUpSubscriptionNative)
                    }
                    topUpSubscriptionNative
                },
                {
                    fn updateSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <updateSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::updateSubscription)
                    }
                    updateSubscription
                },
                {
                    fn pendingRequestExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <pendingRequestExistsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::pendingRequestExists)
                    }
                    pendingRequestExists
                },
                {
                    fn createTimelockRequestWithSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <createTimelockRequestWithSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                MockBlocklockReceiverCalls::createTimelockRequestWithSubscription,
                            )
                    }
                    createTimelockRequestWithSubscription
                },
                {
                    fn setBlocklock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <setBlocklockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::setBlocklock)
                    }
                    setBlocklock
                },
                {
                    fn createTimelockRequestWithDirectFunding(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <createTimelockRequestWithDirectFundingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                MockBlocklockReceiverCalls::createTimelockRequestWithDirectFunding,
                            )
                    }
                    createTimelockRequestWithDirectFunding
                },
                {
                    fn receiveBlocklock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <receiveBlocklockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::receiveBlocklock)
                    }
                    receiveBlocklock
                },
                {
                    fn acceptOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::acceptOwnership)
                    }
                    acceptOwnership
                },
                {
                    fn setSubId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <setSubIdCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(MockBlocklockReceiverCalls::setSubId)
                    }
                    setSubId
                },
                {
                    fn encryptedValue(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <encryptedValueCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::encryptedValue)
                    }
                    encryptedValue
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(MockBlocklockReceiverCalls::owner)
                    }
                    owner
                },
                {
                    fn blocklock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <blocklockCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(MockBlocklockReceiverCalls::blocklock)
                    }
                    blocklock
                },
                {
                    fn cancelSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <cancelSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::cancelSubscription)
                    }
                    cancelSubscription
                },
                {
                    fn plainTextValue(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <plainTextValueCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::plainTextValue)
                    }
                    plainTextValue
                },
                {
                    fn withdrawNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <withdrawNativeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::withdrawNative)
                    }
                    withdrawNative
                },
                {
                    fn createSubscriptionAndFundNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <createSubscriptionAndFundNativeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                MockBlocklockReceiverCalls::createSubscriptionAndFundNative,
                            )
                    }
                    createSubscriptionAndFundNative
                },
                {
                    fn isInFlight(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <isInFlightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::isInFlight)
                    }
                    isInFlight
                },
                {
                    fn fundContractNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <fundContractNativeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::fundContractNative)
                    }
                    fundContractNative
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::transferOwnership)
                    }
                    transferOwnership
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
            ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls>] = &[
                {
                    fn requestId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <requestIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::requestId)
                    }
                    requestId
                },
                {
                    fn subscriptionId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <subscriptionIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::subscriptionId)
                    }
                    subscriptionId
                },
                {
                    fn getBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <getBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::getBalance)
                    }
                    getBalance
                },
                {
                    fn topUpSubscriptionNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <topUpSubscriptionNativeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::topUpSubscriptionNative)
                    }
                    topUpSubscriptionNative
                },
                {
                    fn updateSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <updateSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::updateSubscription)
                    }
                    updateSubscription
                },
                {
                    fn pendingRequestExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <pendingRequestExistsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::pendingRequestExists)
                    }
                    pendingRequestExists
                },
                {
                    fn createTimelockRequestWithSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <createTimelockRequestWithSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                MockBlocklockReceiverCalls::createTimelockRequestWithSubscription,
                            )
                    }
                    createTimelockRequestWithSubscription
                },
                {
                    fn setBlocklock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <setBlocklockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::setBlocklock)
                    }
                    setBlocklock
                },
                {
                    fn createTimelockRequestWithDirectFunding(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <createTimelockRequestWithDirectFundingCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                MockBlocklockReceiverCalls::createTimelockRequestWithDirectFunding,
                            )
                    }
                    createTimelockRequestWithDirectFunding
                },
                {
                    fn receiveBlocklock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <receiveBlocklockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::receiveBlocklock)
                    }
                    receiveBlocklock
                },
                {
                    fn acceptOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::acceptOwnership)
                    }
                    acceptOwnership
                },
                {
                    fn setSubId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <setSubIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::setSubId)
                    }
                    setSubId
                },
                {
                    fn encryptedValue(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <encryptedValueCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::encryptedValue)
                    }
                    encryptedValue
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::owner)
                    }
                    owner
                },
                {
                    fn blocklock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <blocklockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::blocklock)
                    }
                    blocklock
                },
                {
                    fn cancelSubscription(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <cancelSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::cancelSubscription)
                    }
                    cancelSubscription
                },
                {
                    fn plainTextValue(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <plainTextValueCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::plainTextValue)
                    }
                    plainTextValue
                },
                {
                    fn withdrawNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <withdrawNativeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::withdrawNative)
                    }
                    withdrawNative
                },
                {
                    fn createSubscriptionAndFundNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <createSubscriptionAndFundNativeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                MockBlocklockReceiverCalls::createSubscriptionAndFundNative,
                            )
                    }
                    createSubscriptionAndFundNative
                },
                {
                    fn isInFlight(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <isInFlightCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::isInFlight)
                    }
                    isInFlight
                },
                {
                    fn fundContractNative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <fundContractNativeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::fundContractNative)
                    }
                    fundContractNative
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockBlocklockReceiverCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockBlocklockReceiverCalls::transferOwnership)
                    }
                    transferOwnership
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
                Self::acceptOwnership(inner) => {
                    <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::blocklock(inner) => {
                    <blocklockCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::cancelSubscription(inner) => {
                    <cancelSubscriptionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createSubscriptionAndFundNative(inner) => {
                    <createSubscriptionAndFundNativeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createTimelockRequestWithDirectFunding(inner) => {
                    <createTimelockRequestWithDirectFundingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createTimelockRequestWithSubscription(inner) => {
                    <createTimelockRequestWithSubscriptionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::encryptedValue(inner) => {
                    <encryptedValueCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::fundContractNative(inner) => {
                    <fundContractNativeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBalance(inner) => {
                    <getBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isInFlight(inner) => {
                    <isInFlightCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pendingRequestExists(inner) => {
                    <pendingRequestExistsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::plainTextValue(inner) => {
                    <plainTextValueCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::receiveBlocklock(inner) => {
                    <receiveBlocklockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::requestId(inner) => {
                    <requestIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setBlocklock(inner) => {
                    <setBlocklockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setSubId(inner) => {
                    <setSubIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::subscriptionId(inner) => {
                    <subscriptionIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::topUpSubscriptionNative(inner) => {
                    <topUpSubscriptionNativeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateSubscription(inner) => {
                    <updateSubscriptionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawNative(inner) => {
                    <withdrawNativeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::acceptOwnership(inner) => {
                    <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::blocklock(inner) => {
                    <blocklockCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::createSubscriptionAndFundNative(inner) => {
                    <createSubscriptionAndFundNativeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createTimelockRequestWithDirectFunding(inner) => {
                    <createTimelockRequestWithDirectFundingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createTimelockRequestWithSubscription(inner) => {
                    <createTimelockRequestWithSubscriptionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::encryptedValue(inner) => {
                    <encryptedValueCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::fundContractNative(inner) => {
                    <fundContractNativeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBalance(inner) => {
                    <getBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pendingRequestExists(inner) => {
                    <pendingRequestExistsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::plainTextValue(inner) => {
                    <plainTextValueCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::receiveBlocklock(inner) => {
                    <receiveBlocklockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::requestId(inner) => {
                    <requestIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setBlocklock(inner) => {
                    <setBlocklockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setSubId(inner) => {
                    <setSubIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::subscriptionId(inner) => {
                    <subscriptionIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::topUpSubscriptionNative(inner) => {
                    <topUpSubscriptionNativeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateSubscription(inner) => {
                    <updateSubscriptionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawNative(inner) => {
                    <withdrawNativeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`MockBlocklockReceiver`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum MockBlocklockReceiverEvents {
        #[allow(missing_docs)]
        Funded(Funded),
        #[allow(missing_docs)]
        NewSubscriptionId(NewSubscriptionId),
        #[allow(missing_docs)]
        OwnershipTransferRequested(OwnershipTransferRequested),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Received(Received),
        #[allow(missing_docs)]
        Withdrawn(Withdrawn),
    }
    impl MockBlocklockReceiverEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                90u8, 248u8, 24u8, 75u8, 239u8, 142u8, 75u8, 69u8, 235u8, 159u8, 110u8,
                215u8, 115u8, 77u8, 4u8, 218u8, 56u8, 206u8, 210u8, 38u8, 73u8, 85u8,
                72u8, 244u8, 110u8, 12u8, 143u8, 248u8, 215u8, 217u8, 165u8, 36u8,
            ],
            [
                95u8, 71u8, 154u8, 201u8, 147u8, 146u8, 94u8, 250u8, 226u8, 56u8, 57u8,
                170u8, 28u8, 129u8, 59u8, 110u8, 203u8, 47u8, 225u8, 187u8, 209u8, 77u8,
                199u8, 130u8, 149u8, 171u8, 38u8, 125u8, 47u8, 202u8, 67u8, 104u8,
            ],
            [
                112u8, 132u8, 245u8, 71u8, 102u8, 24u8, 216u8, 230u8, 11u8, 17u8, 239u8,
                13u8, 125u8, 63u8, 6u8, 145u8, 70u8, 85u8, 173u8, 184u8, 121u8, 62u8,
                40u8, 255u8, 127u8, 1u8, 141u8, 76u8, 118u8, 213u8, 5u8, 213u8,
            ],
            [
                136u8, 165u8, 150u8, 109u8, 55u8, 11u8, 153u8, 25u8, 178u8, 15u8, 62u8,
                44u8, 19u8, 255u8, 101u8, 112u8, 111u8, 25u8, 106u8, 78u8, 50u8, 204u8,
                44u8, 18u8, 191u8, 87u8, 8u8, 143u8, 136u8, 82u8, 88u8, 116u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                237u8, 136u8, 137u8, 245u8, 96u8, 50u8, 110u8, 177u8, 56u8, 146u8, 13u8,
                132u8, 33u8, 146u8, 240u8, 235u8, 61u8, 210u8, 43u8, 79u8, 19u8, 156u8,
                135u8, 162u8, 197u8, 117u8, 56u8, 224u8, 91u8, 174u8, 18u8, 120u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(Funded),
            ::core::stringify!(NewSubscriptionId),
            ::core::stringify!(Withdrawn),
            ::core::stringify!(Received),
            ::core::stringify!(OwnershipTransferred),
            ::core::stringify!(OwnershipTransferRequested),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <Funded as alloy_sol_types::SolEvent>::SIGNATURE,
            <NewSubscriptionId as alloy_sol_types::SolEvent>::SIGNATURE,
            <Withdrawn as alloy_sol_types::SolEvent>::SIGNATURE,
            <Received as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferRequested as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for MockBlocklockReceiverEvents {
        const NAME: &'static str = "MockBlocklockReceiverEvents";
        const COUNT: usize = 6usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Funded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Funded as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Funded)
                }
                Some(
                    <NewSubscriptionId as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <NewSubscriptionId as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::NewSubscriptionId)
                }
                Some(
                    <OwnershipTransferRequested as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferRequested as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferRequested)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<Received as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Received as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Received)
                }
                Some(<Withdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Withdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Withdrawn)
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
    impl alloy_sol_types::private::IntoLogData for MockBlocklockReceiverEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Funded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewSubscriptionId(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Received(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Withdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Funded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewSubscriptionId(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Received(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Withdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`MockBlocklockReceiver`](self) contract instance.

See the [wrapper's documentation](`MockBlocklockReceiverInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> MockBlocklockReceiverInstance<P, N> {
        MockBlocklockReceiverInstance::<P, N>::new(address, __provider)
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
        blocklockContract: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<MockBlocklockReceiverInstance<P, N>>,
    > {
        MockBlocklockReceiverInstance::<P, N>::deploy(__provider, blocklockContract)
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
        blocklockContract: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        MockBlocklockReceiverInstance::<
            P,
            N,
        >::deploy_builder(__provider, blocklockContract)
    }
    /**A [`MockBlocklockReceiver`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`MockBlocklockReceiver`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct MockBlocklockReceiverInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for MockBlocklockReceiverInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("MockBlocklockReceiverInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > MockBlocklockReceiverInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`MockBlocklockReceiver`](self) contract instance.

See the [wrapper's documentation](`MockBlocklockReceiverInstance`) for more details.*/
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
            blocklockContract: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<MockBlocklockReceiverInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider, blocklockContract);
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
            blocklockContract: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            blocklockContract,
                        },
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
    impl<P: ::core::clone::Clone, N> MockBlocklockReceiverInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> MockBlocklockReceiverInstance<P, N> {
            MockBlocklockReceiverInstance {
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
    > MockBlocklockReceiverInstance<P, N> {
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
        ///Creates a new call builder for the [`acceptOwnership`] function.
        pub fn acceptOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, acceptOwnershipCall, N> {
            self.call_builder(&acceptOwnershipCall)
        }
        ///Creates a new call builder for the [`blocklock`] function.
        pub fn blocklock(&self) -> alloy_contract::SolCallBuilder<&P, blocklockCall, N> {
            self.call_builder(&blocklockCall)
        }
        ///Creates a new call builder for the [`cancelSubscription`] function.
        pub fn cancelSubscription(
            &self,
            to: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, cancelSubscriptionCall, N> {
            self.call_builder(&cancelSubscriptionCall { to })
        }
        ///Creates a new call builder for the [`createSubscriptionAndFundNative`] function.
        pub fn createSubscriptionAndFundNative(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, createSubscriptionAndFundNativeCall, N> {
            self.call_builder(&createSubscriptionAndFundNativeCall)
        }
        ///Creates a new call builder for the [`createTimelockRequestWithDirectFunding`] function.
        pub fn createTimelockRequestWithDirectFunding(
            &self,
            callbackGasLimit: u32,
            condition: alloy::sol_types::private::Bytes,
            encryptedData: <TypesLib::Ciphertext as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            createTimelockRequestWithDirectFundingCall,
            N,
        > {
            self.call_builder(
                &createTimelockRequestWithDirectFundingCall {
                    callbackGasLimit,
                    condition,
                    encryptedData,
                },
            )
        }
        ///Creates a new call builder for the [`createTimelockRequestWithSubscription`] function.
        pub fn createTimelockRequestWithSubscription(
            &self,
            callbackGasLimit: u32,
            condition: alloy::sol_types::private::Bytes,
            encryptedData: <TypesLib::Ciphertext as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            createTimelockRequestWithSubscriptionCall,
            N,
        > {
            self.call_builder(
                &createTimelockRequestWithSubscriptionCall {
                    callbackGasLimit,
                    condition,
                    encryptedData,
                },
            )
        }
        ///Creates a new call builder for the [`encryptedValue`] function.
        pub fn encryptedValue(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, encryptedValueCall, N> {
            self.call_builder(&encryptedValueCall)
        }
        ///Creates a new call builder for the [`fundContractNative`] function.
        pub fn fundContractNative(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, fundContractNativeCall, N> {
            self.call_builder(&fundContractNativeCall)
        }
        ///Creates a new call builder for the [`getBalance`] function.
        pub fn getBalance(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getBalanceCall, N> {
            self.call_builder(&getBalanceCall)
        }
        ///Creates a new call builder for the [`isInFlight`] function.
        pub fn isInFlight(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isInFlightCall, N> {
            self.call_builder(&isInFlightCall { requestId })
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
        }
        ///Creates a new call builder for the [`pendingRequestExists`] function.
        pub fn pendingRequestExists(
            &self,
            subId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, pendingRequestExistsCall, N> {
            self.call_builder(&pendingRequestExistsCall { subId })
        }
        ///Creates a new call builder for the [`plainTextValue`] function.
        pub fn plainTextValue(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, plainTextValueCall, N> {
            self.call_builder(&plainTextValueCall)
        }
        ///Creates a new call builder for the [`receiveBlocklock`] function.
        pub fn receiveBlocklock(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
            decryptionKey: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, receiveBlocklockCall, N> {
            self.call_builder(
                &receiveBlocklockCall {
                    requestId,
                    decryptionKey,
                },
            )
        }
        ///Creates a new call builder for the [`requestId`] function.
        pub fn requestId(&self) -> alloy_contract::SolCallBuilder<&P, requestIdCall, N> {
            self.call_builder(&requestIdCall)
        }
        ///Creates a new call builder for the [`setBlocklock`] function.
        pub fn setBlocklock(
            &self,
            _blocklock: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, setBlocklockCall, N> {
            self.call_builder(&setBlocklockCall { _blocklock })
        }
        ///Creates a new call builder for the [`setSubId`] function.
        pub fn setSubId(
            &self,
            subId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, setSubIdCall, N> {
            self.call_builder(&setSubIdCall { subId })
        }
        ///Creates a new call builder for the [`subscriptionId`] function.
        pub fn subscriptionId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, subscriptionIdCall, N> {
            self.call_builder(&subscriptionIdCall)
        }
        ///Creates a new call builder for the [`topUpSubscriptionNative`] function.
        pub fn topUpSubscriptionNative(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, topUpSubscriptionNativeCall, N> {
            self.call_builder(&topUpSubscriptionNativeCall)
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            to: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { to })
        }
        ///Creates a new call builder for the [`updateSubscription`] function.
        pub fn updateSubscription(
            &self,
            consumers: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<&P, updateSubscriptionCall, N> {
            self.call_builder(
                &updateSubscriptionCall {
                    consumers,
                },
            )
        }
        ///Creates a new call builder for the [`withdrawNative`] function.
        pub fn withdrawNative(
            &self,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, withdrawNativeCall, N> {
            self.call_builder(
                &withdrawNativeCall {
                    amount,
                    recipient,
                },
            )
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > MockBlocklockReceiverInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Funded`] event.
        pub fn Funded_filter(&self) -> alloy_contract::Event<&P, Funded, N> {
            self.event_filter::<Funded>()
        }
        ///Creates a new event filter for the [`NewSubscriptionId`] event.
        pub fn NewSubscriptionId_filter(
            &self,
        ) -> alloy_contract::Event<&P, NewSubscriptionId, N> {
            self.event_filter::<NewSubscriptionId>()
        }
        ///Creates a new event filter for the [`OwnershipTransferRequested`] event.
        pub fn OwnershipTransferRequested_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferRequested, N> {
            self.event_filter::<OwnershipTransferRequested>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`Received`] event.
        pub fn Received_filter(&self) -> alloy_contract::Event<&P, Received, N> {
            self.event_filter::<Received>()
        }
        ///Creates a new event filter for the [`Withdrawn`] event.
        pub fn Withdrawn_filter(&self) -> alloy_contract::Event<&P, Withdrawn, N> {
            self.event_filter::<Withdrawn>()
        }
    }
}
