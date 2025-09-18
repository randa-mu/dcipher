///Module containing a contract's types and functions.
/**

```solidity
library TypesLib {
    struct DecryptionRequest { string schemeID; bytes ciphertext; bytes condition; bytes decryptionKey; bytes signature; address callback; bool isFulfilled; }
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
struct DecryptionRequest { string schemeID; bytes ciphertext; bytes condition; bytes decryptionKey; bytes signature; address callback; bool isFulfilled; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DecryptionRequest {
        #[allow(missing_docs)]
        pub schemeID: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub ciphertext: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub condition: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub decryptionKey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub callback: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub isFulfilled: bool,
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
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Address,
            bool,
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
        impl ::core::convert::From<DecryptionRequest> for UnderlyingRustTuple<'_> {
            fn from(value: DecryptionRequest) -> Self {
                (
                    value.schemeID,
                    value.ciphertext,
                    value.condition,
                    value.decryptionKey,
                    value.signature,
                    value.callback,
                    value.isFulfilled,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DecryptionRequest {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    schemeID: tuple.0,
                    ciphertext: tuple.1,
                    condition: tuple.2,
                    decryptionKey: tuple.3,
                    signature: tuple.4,
                    callback: tuple.5,
                    isFulfilled: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DecryptionRequest {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DecryptionRequest {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.schemeID,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.ciphertext,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.condition,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.decryptionKey,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.callback,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isFulfilled,
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
        impl alloy_sol_types::SolType for DecryptionRequest {
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
        impl alloy_sol_types::SolStruct for DecryptionRequest {
            const NAME: &'static str = "DecryptionRequest";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DecryptionRequest(string schemeID,bytes ciphertext,bytes condition,bytes decryptionKey,bytes signature,address callback,bool isFulfilled)",
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.schemeID,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ciphertext,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.condition,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.decryptionKey,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.callback,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.isFulfilled,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DecryptionRequest {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.schemeID,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ciphertext,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.condition,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.decryptionKey,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.callback,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.isFulfilled,
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
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.schemeID,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ciphertext,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.condition,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.decryptionKey,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.callback,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.isFulfilled,
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
        provider: P,
    ) -> TypesLibInstance<P, N> {
        TypesLibInstance::<P, N>::new(address, provider)
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
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > TypesLibInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`TypesLib`](self) contract instance.

See the [wrapper's documentation](`TypesLibInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
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
    #[automatically_derived]
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
    #[automatically_derived]
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
    struct DecryptionRequest {
        string schemeID;
        bytes ciphertext;
        bytes condition;
        bytes decryptionKey;
        bytes signature;
        address callback;
        bool isFulfilled;
    }
}

interface DecryptionSender {
    error AccessControlBadConfirmation();
    error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
    error AddressEmptyCode(address target);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error FailedCall();
    error InvalidInitialization();
    error NotInitializing();
    error ReentrancyGuardReentrantCall();
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);

    event DecryptionReceiverCallbackFailed(uint256 indexed requestId);
    event DecryptionReceiverCallbackSuccess(uint256 indexed requestId, bytes decryptionKey, bytes signature);
    event DecryptionRequested(uint256 indexed requestId, address indexed callback, string schemeID, bytes condition, bytes ciphertext, uint256 requestedAt);
    event Initialized(uint64 version);
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event SignatureSchemeAddressProviderUpdated(address indexed newSignatureSchemeAddressProvider);
    event Upgraded(address indexed implementation);

    constructor();

    function ADMIN_ROLE() external view returns (bytes32);
    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function fulfillDecryptionRequest(uint256 requestId, bytes memory decryptionKey, bytes memory signature) external;
    function getAllErroredRequestIds() external view returns (uint256[] memory);
    function getAllFulfilledRequestIds() external view returns (uint256[] memory);
    function getAllUnfulfilledRequestIds() external view returns (uint256[] memory);
    function getCountOfUnfulfilledRequestIds() external view returns (uint256);
    function getRequest(uint256 requestId) external view returns (TypesLib.DecryptionRequest memory);
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    function getRoleMember(bytes32 role, uint256 index) external view returns (address);
    function getRoleMemberCount(bytes32 role) external view returns (uint256);
    function getRoleMembers(bytes32 role) external view returns (address[] memory);
    function grantRole(bytes32 role, address account) external;
    function hasErrored(uint256 requestId) external view returns (bool);
    function hasRole(bytes32 role, address account) external view returns (bool);
    function initialize(address owner, address _signatureSchemeAddressProvider) external;
    function isInFlight(uint256 requestId) external view returns (bool);
    function lastRequestId() external view returns (uint256);
    function multicall(bytes[] memory data) external returns (bytes[] memory results);
    function proxiableUUID() external view returns (bytes32);
    function registerCiphertext(string memory schemeID, bytes memory ciphertext, bytes memory condition) external returns (uint256);
    function renounceRole(bytes32 role, address callerConfirmation) external;
    function requests(uint256) external view returns (string memory schemeID, bytes memory ciphertext, bytes memory condition, bytes memory decryptionKey, bytes memory signature, address callback, bool isFulfilled);
    function revokeRole(bytes32 role, address account) external;
    function setSignatureSchemeAddressProvider(address newSignatureSchemeAddressProvider) external;
    function signatureSchemeAddressProvider() external view returns (address);
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
    function version() external pure returns (string memory);
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
    "name": "fulfillDecryptionRequest",
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
    "name": "getAllErroredRequestIds",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAllFulfilledRequestIds",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAllUnfulfilledRequestIds",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCountOfUnfulfilledRequestIds",
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
        "internalType": "struct TypesLib.DecryptionRequest",
        "components": [
          {
            "name": "schemeID",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "ciphertext",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "condition",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "decryptionKey",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "callback",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "isFulfilled",
            "type": "bool",
            "internalType": "bool"
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
    "name": "hasErrored",
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
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_signatureSchemeAddressProvider",
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
    "name": "lastRequestId",
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
    "name": "multicall",
    "inputs": [
      {
        "name": "data",
        "type": "bytes[]",
        "internalType": "bytes[]"
      }
    ],
    "outputs": [
      {
        "name": "results",
        "type": "bytes[]",
        "internalType": "bytes[]"
      }
    ],
    "stateMutability": "nonpayable"
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
    "name": "registerCiphertext",
    "inputs": [
      {
        "name": "schemeID",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "ciphertext",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "condition",
        "type": "bytes",
        "internalType": "bytes"
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
    "name": "requests",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "schemeID",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "ciphertext",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "condition",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "decryptionKey",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "callback",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "isFulfilled",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
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
    "name": "setSignatureSchemeAddressProvider",
    "inputs": [
      {
        "name": "newSignatureSchemeAddressProvider",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "signatureSchemeAddressProvider",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISignatureSchemeAddressProvider"
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
    "type": "event",
    "name": "DecryptionReceiverCallbackFailed",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DecryptionReceiverCallbackSuccess",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "decryptionKey",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
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
    "name": "DecryptionRequested",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "callback",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "schemeID",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "condition",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "ciphertext",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
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
    "name": "SignatureSchemeAddressProviderUpdated",
    "inputs": [
      {
        "name": "newSignatureSchemeAddressProvider",
        "type": "address",
        "indexed": true,
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
    "name": "InvalidInitialization",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotInitializing",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ReentrancyGuardReentrantCall",
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
pub mod DecryptionSender {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a0604052306080525f600155348015610017575f5ffd5b5060015f55610024610029565b6100db565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805468010000000000000000900460ff16156100795760405163f92ee8a960e01b815260040160405180910390fd5b80546001600160401b03908116146100d85780546001600160401b0319166001600160401b0390811782556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50565b6080516131e96101015f395f8181611ef201528181611f1b015261206801526131e95ff3fe6080604052600436106101c5575f3560e01c806391d14854116100f2578063ca15c87311610092578063e6b3ca7111610062578063e6b3ca711461054d578063f7cef8001461056c578063f87f0e611461058b578063fc2a88c3146105aa575f5ffd5b8063ca15c873146104dc578063cd802c91146104fb578063d547741f1461051a578063e63b5d5814610539575f5ffd5b8063ac9650d8116100cd578063ac9650d814610435578063ad3cb1cc14610461578063b094728914610491578063c58343ef146104b0575f5ffd5b806391d14854146103d7578063a217fddf146103f6578063a3246ad314610409575f5ffd5b80634f1ef286116101685780636f421ea9116101385780636f421ea91461033a57806375b238fc1461034e57806381d12c581461036e5780639010d07c146103a0575f5ffd5b80634f1ef286146102c957806352d1902d146102dc57806354fd4d50146102f0578063571d708714610326575f5ffd5b80632f2ff15d116101a35780632f2ff15d1461024b57806336568abe1461026a578063485cc955146102895780634b96e166146102a8575f5ffd5b806301ffc9a7146101c957806316cc9a98146101fd578063248a9ca31461021e575b5f5ffd5b3480156101d4575f5ffd5b506101e86101e3366004612716565b6105bf565b60405190151581526020015b60405180910390f35b348015610208575f5ffd5b5061021c610217366004612751565b6105e9565b005b348015610229575f5ffd5b5061023d61023836600461276c565b610648565b6040519081526020016101f4565b348015610256575f5ffd5b5061021c610265366004612783565b610668565b348015610275575f5ffd5b5061021c610284366004612783565b61068a565b348015610294575f5ffd5b5061021c6102a33660046127b1565b6106c2565b3480156102b3575f5ffd5b506102bc610882565b6040516101f491906127dd565b61021c6102d7366004612889565b610893565b3480156102e7575f5ffd5b5061023d6108b2565b3480156102fb575f5ffd5b50604080518082019091526005815264302e302e3160d81b60208201525b6040516101f49190612943565b348015610331575f5ffd5b506102bc6108cd565b348015610345575f5ffd5b506102bc6108d9565b348015610359575f5ffd5b5061023d5f5160206131945f395f51905f5281565b348015610379575f5ffd5b5061038d61038836600461276c565b6108e5565b6040516101f49796959493929190612955565b3480156103ab575f5ffd5b506103bf6103ba3660046129da565b610bcc565b6040516001600160a01b0390911681526020016101f4565b3480156103e2575f5ffd5b506101e86103f1366004612783565b610bf9565b348015610401575f5ffd5b5061023d5f81565b348015610414575f5ffd5b5061042861042336600461276c565b610c2f565b6040516101f491906129fa565b348015610440575f5ffd5b5061045461044f366004612a3a565b610c5f565b6040516101f49190612aa9565b34801561046c575f5ffd5b50610319604051806040016040528060058152602001640352e302e360dc1b81525081565b34801561049c575f5ffd5b506101e86104ab36600461276c565b610d44565b3480156104bb575f5ffd5b506104cf6104ca36600461276c565b610d50565b6040516101f49190612b0c565b3480156104e7575f5ffd5b5061023d6104f636600461276c565b6110ad565b348015610506575f5ffd5b506101e861051536600461276c565b6110d1565b348015610525575f5ffd5b5061021c610534366004612783565b6110ee565b348015610544575f5ffd5b5061023d61110a565b348015610558575f5ffd5b506003546103bf906001600160a01b031681565b348015610577575f5ffd5b5061021c610586366004612c13565b611115565b348015610596575f5ffd5b5061023d6105a5366004612c8a565b61187a565b3480156105b5575f5ffd5b5061023d60015481565b5f6001600160e01b03198216635a05180f60e01b14806105e357506105e382611ded565b92915050565b6105ff5f5160206131945f395f51905f52611e21565b600380546001600160a01b0319166001600160a01b0383169081179091556040517f7724bcb43a09ae6582affdee2f0ace931e26f2ffa8b5c334baf0a39e9dc03426905f90a250565b5f9081525f5160206131745f395f51905f52602052604090206001015490565b61067182610648565b61067a81611e21565b6106848383611e2e565b50505050565b6001600160a01b03811633146106b35760405163334bd91960e11b815260040160405180910390fd5b6106bd8282611e70565b505050565b5f6106cb611ea9565b805490915060ff600160401b82041615906001600160401b03165f811580156106f15750825b90505f826001600160401b0316600114801561070c5750303b155b90508115801561071a575080155b156107385760405163f92ee8a960e01b815260040160405180910390fd5b845467ffffffffffffffff19166001178555831561076257845460ff60401b1916600160401b1785555b61076a611ed1565b610772611ed1565b6107895f5160206131945f395f51905f5288611e2e565b6107ce5760405162461bcd60e51b815260206004820152601160248201527011dc985b9d081c9bdb194819985a5b1959607a1b60448201526064015b60405180910390fd5b6107d85f88611e2e565b6108185760405162461bcd60e51b815260206004820152601160248201527011dc985b9d081c9bdb194819985a5b1959607a1b60448201526064016107c5565b600380546001600160a01b0319166001600160a01b038816179055831561087957845460ff60401b19168555604051600181527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50505050505050565b606061088e6006611edb565b905090565b61089b611ee7565b6108a482611f8b565b6108ae8282611fa1565b5050565b5f6108bb61205d565b505f5160206131545f395f51905f5290565b606061088e6004611edb565b606061088e6008611edb565b60026020525f90815260409020805481906108ff90612d26565b80601f016020809104026020016040519081016040528092919081815260200182805461092b90612d26565b80156109765780601f1061094d57610100808354040283529160200191610976565b820191905f5260205f20905b81548152906001019060200180831161095957829003601f168201915b50505050509080600101805461098b90612d26565b80601f01602080910402602001604051908101604052809291908181526020018280546109b790612d26565b8015610a025780601f106109d957610100808354040283529160200191610a02565b820191905f5260205f20905b8154815290600101906020018083116109e557829003601f168201915b505050505090806002018054610a1790612d26565b80601f0160208091040260200160405190810160405280929190818152602001828054610a4390612d26565b8015610a8e5780601f10610a6557610100808354040283529160200191610a8e565b820191905f5260205f20905b815481529060010190602001808311610a7157829003601f168201915b505050505090806003018054610aa390612d26565b80601f0160208091040260200160405190810160405280929190818152602001828054610acf90612d26565b8015610b1a5780601f10610af157610100808354040283529160200191610b1a565b820191905f5260205f20905b815481529060010190602001808311610afd57829003601f168201915b505050505090806004018054610b2f90612d26565b80601f0160208091040260200160405190810160405280929190818152602001828054610b5b90612d26565b8015610ba65780601f10610b7d57610100808354040283529160200191610ba6565b820191905f5260205f20905b815481529060010190602001808311610b8957829003601f168201915b505050600590930154919250506001600160a01b0381169060ff600160a01b9091041687565b5f8281525f5160206131345f395f51905f52602081905260408220610bf190846120a6565b949350505050565b5f9182525f5160206131745f395f51905f52602090815260408084206001600160a01b0393909316845291905290205460ff1690565b5f8181525f5160206131345f395f51905f526020819052604090912060609190610c5890611edb565b9392505050565b604080515f815260208101909152606090826001600160401b03811115610c8857610c8861281f565b604051908082528060200260200182016040528015610cbb57816020015b6060815260200190600190039081610ca65790505b5091505f5b83811015610d3c57610d1730868684818110610cde57610cde612d85565b9050602002810190610cf09190612d99565b85604051602001610d0393929190612df2565b6040516020818303038152906040526120b1565b838281518110610d2957610d29612d85565b6020908102919091010152600101610cc0565b505092915050565b5f6105e3600883612123565b610d996040518060e0016040528060608152602001606081526020016060815260200160608152602001606081526020015f6001600160a01b031681526020015f151581525090565b5f8281526002602052604090819020815160e08101909252805482908290610dc090612d26565b80601f0160208091040260200160405190810160405280929190818152602001828054610dec90612d26565b8015610e375780601f10610e0e57610100808354040283529160200191610e37565b820191905f5260205f20905b815481529060010190602001808311610e1a57829003601f168201915b50505050508152602001600182018054610e5090612d26565b80601f0160208091040260200160405190810160405280929190818152602001828054610e7c90612d26565b8015610ec75780601f10610e9e57610100808354040283529160200191610ec7565b820191905f5260205f20905b815481529060010190602001808311610eaa57829003601f168201915b50505050508152602001600282018054610ee090612d26565b80601f0160208091040260200160405190810160405280929190818152602001828054610f0c90612d26565b8015610f575780601f10610f2e57610100808354040283529160200191610f57565b820191905f5260205f20905b815481529060010190602001808311610f3a57829003601f168201915b50505050508152602001600382018054610f7090612d26565b80601f0160208091040260200160405190810160405280929190818152602001828054610f9c90612d26565b8015610fe75780601f10610fbe57610100808354040283529160200191610fe7565b820191905f5260205f20905b815481529060010190602001808311610fca57829003601f168201915b5050505050815260200160048201805461100090612d26565b80601f016020809104026020016040519081016040528092919081815260200182805461102c90612d26565b80156110775780601f1061104e57610100808354040283529160200191611077565b820191905f5260205f20905b81548152906001019060200180831161105a57829003601f168201915b5050509183525050600591909101546001600160a01b0381166020830152600160a01b900460ff16151560409091015292915050565b5f8181525f5160206131345f395f51905f52602081905260408220610c589061213a565b5f6110dd600683612123565b806105e357506105e3600883612123565b6110f782610648565b61110081611e21565b6106848383611e70565b5f61088e600661213a565b61111d612143565b611126856110d1565b6111865760405162461bcd60e51b815260206004820152602b60248201527f4e6f2070656e64696e672072657175657374207769746820737065636966696560448201526a19081c995c5d595cdd125960aa1b60648201526084016107c5565b5f85815260026020526040808220815160e081019092528054829082906111ac90612d26565b80601f01602080910402602001604051908101604052809291908181526020018280546111d890612d26565b80156112235780601f106111fa57610100808354040283529160200191611223565b820191905f5260205f20905b81548152906001019060200180831161120657829003601f168201915b5050505050815260200160018201805461123c90612d26565b80601f016020809104026020016040519081016040528092919081815260200182805461126890612d26565b80156112b35780601f1061128a576101008083540402835291602001916112b3565b820191905f5260205f20905b81548152906001019060200180831161129657829003601f168201915b505050505081526020016002820180546112cc90612d26565b80601f01602080910402602001604051908101604052809291908181526020018280546112f890612d26565b80156113435780601f1061131a57610100808354040283529160200191611343565b820191905f5260205f20905b81548152906001019060200180831161132657829003601f168201915b5050505050815260200160038201805461135c90612d26565b80601f016020809104026020016040519081016040528092919081815260200182805461138890612d26565b80156113d35780601f106113aa576101008083540402835291602001916113d3565b820191905f5260205f20905b8154815290600101906020018083116113b657829003601f168201915b505050505081526020016004820180546113ec90612d26565b80601f016020809104026020016040519081016040528092919081815260200182805461141890612d26565b80156114635780601f1061143a57610100808354040283529160200191611463565b820191905f5260205f20905b81548152906001019060200180831161144657829003601f168201915b5050509183525050600591909101546001600160a01b038082166020840152600160a01b90910460ff16151560409283015282516003549251630b76139f60e31b8152939450925f9290911690635bb09cf8906114c4908590600401612943565b602060405180830381865afa1580156114df573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115039190612e11565b90506001600160a01b03811661154c5760405162461bcd60e51b815260206004820152600e60248201526d696e76616c696420736368656d6560901b60448201526064016107c5565b604080840151905163eae1e15b60e01b815282915f916001600160a01b0384169163eae1e15b916115809190600401612943565b5f60405180830381865afa15801561159a573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526115c19190810190612e2c565b9050816001600160a01b031663f6e548e9828989866001600160a01b031663acae9fee6040518163ffffffff1660e01b81526004015f60405180830381865afa158015611610573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526116379190810190612e2c565b6040518563ffffffff1660e01b81526004016116569493929190612ec8565b602060405180830381865afa158015611671573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116959190612f0c565b6116e15760405162461bcd60e51b815260206004820152601d60248201527f5369676e617475726520766572696669636174696f6e206661696c656400000060448201526064016107c5565b5f8560a001516001600160a01b0316635d3be00160e01b8c8c8c8c8c604051602401611711959493929190612f2b565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905161174f9190612f63565b5f604051808303815f865af19150503d805f8114611788576040519150601f19603f3d011682016040523d82523d5f602084013e61178d565b606091505b50505f8c8152600260205260409020600501805460ff60a01b1916600160a01b17905590506117bd60068c61216b565b50806117fe576117ce60088c612176565b506040518b907f3fed05c5d3a79e0e2f23ffd594267f0e29799ad1b4b6a7756138d5247baa4a7b905f90a2611864565b6118078b610d44565b156118195761181760088c61216b565b505b61182460048c612176565b508a7f0a1f5b3782d8f604353407c980dbfe6b55499c5b92cb8c39f089c73f348407e68b8b8b8b60405161185b9493929190612f6e565b60405180910390a25b50505050505061187360015f55565b5050505050565b5f6001805f82825461188c9190612f94565b9091555050600354604051632fc9fa3360e01b81526001600160a01b0390911690632fc9fa33906118c3908a908a90600401612fa7565b602060405180830381865afa1580156118de573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119029190612f0c565b61194e5760405162461bcd60e51b815260206004820152601e60248201527f5369676e617475726520736368656d65206e6f7420737570706f72746564000060448201526064016107c5565b611995600161100087878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152509294939250506121819050565b6119ef5760405162461bcd60e51b815260206004820152602560248201527f43697068657274657874206661696c6564206c656e67746820626f756e647320604482015264636865636b60d81b60648201526084016107c5565b611a36600161100085858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152509294939250506121819050565b611a8e5760405162461bcd60e51b8152602060048201526024808201527f436f6e646974696f6e206661696c6564206c656e67746820626f756e647320636044820152636865636b60e01b60648201526084016107c5565b611acc83838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061221192505050565b15611b255760405162461bcd60e51b815260206004820152602360248201527f436f6e646974696f6e2062797465732063616e6e6f7420626520616c6c207a65604482015262726f7360e81b60648201526084016107c5565b600354604051630b76139f60e31b81525f916001600160a01b031690635bb09cf890611b57908b908b90600401612fa7565b602060405180830381865afa158015611b72573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b969190612e11565b90506001600160a01b038116611bee5760405162461bcd60e51b815260206004820152601860248201527f696e76616c6964207369676e617475726520736368656d65000000000000000060448201526064016107c5565b6040518060e0016040528089898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250505090825250604080516020601f8a01819004810282018101909252888152918101919089908990819084018382808284375f92019190915250505090825250604080516020601f8801819004810282018101909252868152918101919087908790819084018382808284375f92018290525093855250506040805160208181018352848252808601919091528151808201835284815282860152336060860152608090940183905260015483526002909352502081518190611cee9082612ffe565b5060208201516001820190611d039082612ffe565b5060408201516002820190611d189082612ffe565b5060608201516003820190611d2d9082612ffe565b5060808201516004820190611d429082612ffe565b5060a08201516005909101805460c0909301511515600160a01b026001600160a81b03199093166001600160a01b0390921691909117919091179055600154611d8d90600690612176565b50336001600160a01b03166001547f133b1acfa49920f88bc8c23faeb2ba43b75b3d6c453290a824f3c85e7733df138a8a88888c8c42604051611dd697969594939291906130b8565b60405180910390a350506001549695505050505050565b5f6001600160e01b03198216637965db0b60e01b14806105e357506301ffc9a760e01b6001600160e01b03198316146105e3565b611e2b813361225c565b50565b5f5f5160206131345f395f51905f5281611e488585612295565b90508015610bf1575f858152602083905260409020611e679085612336565b50949350505050565b5f5f5160206131345f395f51905f5281611e8a858561234a565b90508015610bf1575f858152602083905260409020611e6790856123c3565b5f807ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a006105e3565b611ed96123d7565b565b60605f610c58836123fc565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161480611f6d57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316611f615f5160206131545f395f51905f52546001600160a01b031690565b6001600160a01b031614155b15611ed95760405163703e46dd60e11b815260040160405180910390fd5b611e2b5f5160206131945f395f51905f52611e21565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015611ffb575060408051601f3d908101601f19168201909252611ff891810190613108565b60015b61202357604051634c9c8ce360e01b81526001600160a01b03831660048201526024016107c5565b5f5160206131545f395f51905f52811461205357604051632a87526960e21b8152600481018290526024016107c5565b6106bd8383612455565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611ed95760405163703e46dd60e11b815260040160405180910390fd5b5f610c5883836124aa565b60605f5f846001600160a01b0316846040516120cd9190612f63565b5f60405180830381855af49150503d805f8114612105576040519150601f19603f3d011682016040523d82523d5f602084013e61210a565b606091505b509150915061211a8583836124d0565b95945050505050565b5f8181526001830160205260408120541515610c58565b5f6105e3825490565b60025f540361216557604051633ee5aeb560e01b815260040160405180910390fd5b60025f55565b5f610c58838361252c565b5f610c588383612606565b5f818311156121f85760405162461bcd60e51b815260206004820152603a60248201527f496e76616c696420626f756e64733a206d696e4c656e6774682063616e6e6f7460448201527f2062652067726561746572207468616e206d61784c656e67746800000000000060648201526084016107c5565b835183811080159061211a575091909111159392505050565b5f805b82518110156122535782818151811061222f5761222f612d85565b01602001516001600160f81b0319161561224b57505f92915050565b600101612214565b50600192915050565b6122668282610bf9565b6108ae5760405163e2517d3f60e01b81526001600160a01b0382166004820152602481018390526044016107c5565b5f5f5160206131745f395f51905f526122ae8484610bf9565b61232d575f848152602082815260408083206001600160a01b03871684529091529020805460ff191660011790556122e33390565b6001600160a01b0316836001600160a01b0316857f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019150506105e3565b5f9150506105e3565b5f610c58836001600160a01b038416612606565b5f5f5160206131745f395f51905f526123638484610bf9565b1561232d575f848152602082815260408083206001600160a01b0387168085529252808320805460ff1916905551339287917ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9190a460019150506105e3565b5f610c58836001600160a01b03841661252c565b6123df612652565b611ed957604051631afcd79f60e31b815260040160405180910390fd5b6060815f0180548060200260200160405190810160405280929190818152602001828054801561244957602002820191905f5260205f20905b815481526020019060010190808311612435575b50505050509050919050565b61245e8261266b565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a28051156124a2576106bd82826120b1565b6108ae6126ce565b5f825f0182815481106124bf576124bf612d85565b905f5260205f200154905092915050565b6060826124e5576124e0826126ed565b610c58565b81511580156124fc57506001600160a01b0384163b155b1561252557604051639996b31560e01b81526001600160a01b03851660048201526024016107c5565b5080610c58565b5f818152600183016020526040812054801561232d575f61254e600183612d72565b85549091505f9061256190600190612d72565b90508082146125c0575f865f01828154811061257f5761257f612d85565b905f5260205f200154905080875f01848154811061259f5761259f612d85565b5f918252602080832090910192909255918252600188019052604090208390555b85548690806125d1576125d161311f565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f9055600193505050506105e3565b5f81815260018301602052604081205461264b57508154600181810184555f8481526020808220909301849055845484825282860190935260409020919091556105e3565b505f6105e3565b5f61265b611ea9565b54600160401b900460ff16919050565b806001600160a01b03163b5f036126a057604051634c9c8ce360e01b81526001600160a01b03821660048201526024016107c5565b5f5160206131545f395f51905f5280546001600160a01b0319166001600160a01b0392909216919091179055565b3415611ed95760405163b398979f60e01b815260040160405180910390fd5b8051156126fd5780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f60208284031215612726575f5ffd5b81356001600160e01b031981168114610c58575f5ffd5b6001600160a01b0381168114611e2b575f5ffd5b5f60208284031215612761575f5ffd5b8135610c588161273d565b5f6020828403121561277c575f5ffd5b5035919050565b5f5f60408385031215612794575f5ffd5b8235915060208301356127a68161273d565b809150509250929050565b5f5f604083850312156127c2575f5ffd5b82356127cd8161273d565b915060208301356127a68161273d565b602080825282518282018190525f918401906040840190835b818110156128145783518352602093840193909201916001016127f6565b509095945050505050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b038111828210171561285b5761285b61281f565b604052919050565b5f6001600160401b0382111561287b5761287b61281f565b50601f01601f191660200190565b5f5f6040838503121561289a575f5ffd5b82356128a58161273d565b915060208301356001600160401b038111156128bf575f5ffd5b8301601f810185136128cf575f5ffd5b80356128e26128dd82612863565b612833565b8181528660208385010111156128f6575f5ffd5b816020840160208301375f602083830101528093505050509250929050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f610c586020830184612915565b60e081525f61296760e083018a612915565b8281036020840152612979818a612915565b9050828103604084015261298d8189612915565b905082810360608401526129a18188612915565b905082810360808401526129b58187612915565b6001600160a01b039590951660a0840152505090151560c09091015295945050505050565b5f5f604083850312156129eb575f5ffd5b50508035926020909101359150565b602080825282518282018190525f918401906040840190835b818110156128145783516001600160a01b0316835260209384019390920191600101612a13565b5f5f60208385031215612a4b575f5ffd5b82356001600160401b03811115612a60575f5ffd5b8301601f81018513612a70575f5ffd5b80356001600160401b03811115612a85575f5ffd5b8560208260051b8401011115612a99575f5ffd5b6020919091019590945092505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b0057603f19878603018452612aeb858351612915565b94506020938401939190910190600101612acf565b50929695505050505050565b602081525f825160e06020840152612b28610100840182612915565b90506020840151601f19848303016040850152612b458282612915565b9150506040840151601f19848303016060850152612b638282612915565b9150506060840151601f19848303016080850152612b818282612915565b9150506080840151601f198483030160a0850152612b9f8282612915565b91505060018060a01b0360a08501511660c084015260c0840151612bc760e085018215159052565b509392505050565b5f5f83601f840112612bdf575f5ffd5b5081356001600160401b03811115612bf5575f5ffd5b602083019150836020828501011115612c0c575f5ffd5b9250929050565b5f5f5f5f5f60608688031215612c27575f5ffd5b8535945060208601356001600160401b03811115612c43575f5ffd5b612c4f88828901612bcf565b90955093505060408601356001600160401b03811115612c6d575f5ffd5b612c7988828901612bcf565b969995985093965092949392505050565b5f5f5f5f5f5f60608789031215612c9f575f5ffd5b86356001600160401b03811115612cb4575f5ffd5b612cc089828a01612bcf565b90975095505060208701356001600160401b03811115612cde575f5ffd5b612cea89828a01612bcf565b90955093505060408701356001600160401b03811115612d08575f5ffd5b612d1489828a01612bcf565b979a9699509497509295939492505050565b600181811c90821680612d3a57607f821691505b602082108103612d5857634e487b7160e01b5f52602260045260245ffd5b50919050565b634e487b7160e01b5f52601160045260245ffd5b818103818111156105e3576105e3612d5e565b634e487b7160e01b5f52603260045260245ffd5b5f5f8335601e19843603018112612dae575f5ffd5b8301803591506001600160401b03821115612dc7575f5ffd5b602001915036819003821315612c0c575f5ffd5b5f81518060208401855e5f93019283525090919050565b828482375f8382015f8152612e078185612ddb565b9695505050505050565b5f60208284031215612e21575f5ffd5b8151610c588161273d565b5f60208284031215612e3c575f5ffd5b81516001600160401b03811115612e51575f5ffd5b8201601f81018413612e61575f5ffd5b8051612e6f6128dd82612863565b818152856020838501011115612e83575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b606081525f612eda6060830187612915565b8281036020840152612eed818688612ea0565b90508281036040840152612f018185612915565b979650505050505050565b5f60208284031215612f1c575f5ffd5b81518015158114610c58575f5ffd5b858152606060208201525f612f44606083018688612ea0565b8281036040840152612f57818587612ea0565b98975050505050505050565b5f610c588284612ddb565b604081525f612f81604083018688612ea0565b8281036020840152612f01818587612ea0565b808201808211156105e3576105e3612d5e565b602081525f610bf1602083018486612ea0565b601f8211156106bd57805f5260205f20601f840160051c81016020851015612fdf5750805b601f840160051c820191505b81811015611873575f8155600101612feb565b81516001600160401b038111156130175761301761281f565b61302b816130258454612d26565b84612fba565b6020601f82116001811461305d575f83156130465750848201515b5f19600385901b1c1916600184901b178455611873565b5f84815260208120601f198516915b8281101561308c578785015182556020948501946001909201910161306c565b50848210156130a957868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b608081525f6130cb60808301898b612ea0565b82810360208401526130de81888a612ea0565b905082810360408401526130f3818688612ea0565b91505082606083015298975050505050505050565b5f60208284031215613118575f5ffd5b5051919050565b634e487b7160e01b5f52603160045260245ffdfec1f6fe24621ce81ec5827caf0253cadb74709b061630e6b55e82371705932000360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b626800a49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c21775a2646970667358221220adcac5656fbff7469096f3117c572315dc5ef26355b567e82e88299a7ed26afe64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0`\x80R_`\x01U4\x80\x15a\0\x17W__\xFD[P`\x01_Ua\0$a\0)V[a\0\xDBV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\0yW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a\0\xD8W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80Qa1\xE9a\x01\x01_9_\x81\x81a\x1E\xF2\x01R\x81\x81a\x1F\x1B\x01Ra h\x01Ra1\xE9_\xF3\xFE`\x80`@R`\x046\x10a\x01\xC5W_5`\xE0\x1C\x80c\x91\xD1HT\x11a\0\xF2W\x80c\xCA\x15\xC8s\x11a\0\x92W\x80c\xE6\xB3\xCAq\x11a\0bW\x80c\xE6\xB3\xCAq\x14a\x05MW\x80c\xF7\xCE\xF8\0\x14a\x05lW\x80c\xF8\x7F\x0Ea\x14a\x05\x8BW\x80c\xFC*\x88\xC3\x14a\x05\xAAW__\xFD[\x80c\xCA\x15\xC8s\x14a\x04\xDCW\x80c\xCD\x80,\x91\x14a\x04\xFBW\x80c\xD5Gt\x1F\x14a\x05\x1AW\x80c\xE6;]X\x14a\x059W__\xFD[\x80c\xAC\x96P\xD8\x11a\0\xCDW\x80c\xAC\x96P\xD8\x14a\x045W\x80c\xAD<\xB1\xCC\x14a\x04aW\x80c\xB0\x94r\x89\x14a\x04\x91W\x80c\xC5\x83C\xEF\x14a\x04\xB0W__\xFD[\x80c\x91\xD1HT\x14a\x03\xD7W\x80c\xA2\x17\xFD\xDF\x14a\x03\xF6W\x80c\xA3$j\xD3\x14a\x04\tW__\xFD[\x80cO\x1E\xF2\x86\x11a\x01hW\x80coB\x1E\xA9\x11a\x018W\x80coB\x1E\xA9\x14a\x03:W\x80cu\xB28\xFC\x14a\x03NW\x80c\x81\xD1,X\x14a\x03nW\x80c\x90\x10\xD0|\x14a\x03\xA0W__\xFD[\x80cO\x1E\xF2\x86\x14a\x02\xC9W\x80cR\xD1\x90-\x14a\x02\xDCW\x80cT\xFDMP\x14a\x02\xF0W\x80cW\x1Dp\x87\x14a\x03&W__\xFD[\x80c//\xF1]\x11a\x01\xA3W\x80c//\xF1]\x14a\x02KW\x80c6V\x8A\xBE\x14a\x02jW\x80cH\\\xC9U\x14a\x02\x89W\x80cK\x96\xE1f\x14a\x02\xA8W__\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xC9W\x80c\x16\xCC\x9A\x98\x14a\x01\xFDW\x80c$\x8A\x9C\xA3\x14a\x02\x1EW[__\xFD[4\x80\x15a\x01\xD4W__\xFD[Pa\x01\xE8a\x01\xE36`\x04a'\x16V[a\x05\xBFV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x08W__\xFD[Pa\x02\x1Ca\x02\x176`\x04a'QV[a\x05\xE9V[\0[4\x80\x15a\x02)W__\xFD[Pa\x02=a\x0286`\x04a'lV[a\x06HV[`@Q\x90\x81R` \x01a\x01\xF4V[4\x80\x15a\x02VW__\xFD[Pa\x02\x1Ca\x02e6`\x04a'\x83V[a\x06hV[4\x80\x15a\x02uW__\xFD[Pa\x02\x1Ca\x02\x846`\x04a'\x83V[a\x06\x8AV[4\x80\x15a\x02\x94W__\xFD[Pa\x02\x1Ca\x02\xA36`\x04a'\xB1V[a\x06\xC2V[4\x80\x15a\x02\xB3W__\xFD[Pa\x02\xBCa\x08\x82V[`@Qa\x01\xF4\x91\x90a'\xDDV[a\x02\x1Ca\x02\xD76`\x04a(\x89V[a\x08\x93V[4\x80\x15a\x02\xE7W__\xFD[Pa\x02=a\x08\xB2V[4\x80\x15a\x02\xFBW__\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rd0.0.1`\xD8\x1B` \x82\x01R[`@Qa\x01\xF4\x91\x90a)CV[4\x80\x15a\x031W__\xFD[Pa\x02\xBCa\x08\xCDV[4\x80\x15a\x03EW__\xFD[Pa\x02\xBCa\x08\xD9V[4\x80\x15a\x03YW__\xFD[Pa\x02=_Q` a1\x94_9_Q\x90_R\x81V[4\x80\x15a\x03yW__\xFD[Pa\x03\x8Da\x03\x886`\x04a'lV[a\x08\xE5V[`@Qa\x01\xF4\x97\x96\x95\x94\x93\x92\x91\x90a)UV[4\x80\x15a\x03\xABW__\xFD[Pa\x03\xBFa\x03\xBA6`\x04a)\xDAV[a\x0B\xCCV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xF4V[4\x80\x15a\x03\xE2W__\xFD[Pa\x01\xE8a\x03\xF16`\x04a'\x83V[a\x0B\xF9V[4\x80\x15a\x04\x01W__\xFD[Pa\x02=_\x81V[4\x80\x15a\x04\x14W__\xFD[Pa\x04(a\x04#6`\x04a'lV[a\x0C/V[`@Qa\x01\xF4\x91\x90a)\xFAV[4\x80\x15a\x04@W__\xFD[Pa\x04Ta\x04O6`\x04a*:V[a\x0C_V[`@Qa\x01\xF4\x91\x90a*\xA9V[4\x80\x15a\x04lW__\xFD[Pa\x03\x19`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x04\x9CW__\xFD[Pa\x01\xE8a\x04\xAB6`\x04a'lV[a\rDV[4\x80\x15a\x04\xBBW__\xFD[Pa\x04\xCFa\x04\xCA6`\x04a'lV[a\rPV[`@Qa\x01\xF4\x91\x90a+\x0CV[4\x80\x15a\x04\xE7W__\xFD[Pa\x02=a\x04\xF66`\x04a'lV[a\x10\xADV[4\x80\x15a\x05\x06W__\xFD[Pa\x01\xE8a\x05\x156`\x04a'lV[a\x10\xD1V[4\x80\x15a\x05%W__\xFD[Pa\x02\x1Ca\x0546`\x04a'\x83V[a\x10\xEEV[4\x80\x15a\x05DW__\xFD[Pa\x02=a\x11\nV[4\x80\x15a\x05XW__\xFD[P`\x03Ta\x03\xBF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05wW__\xFD[Pa\x02\x1Ca\x05\x866`\x04a,\x13V[a\x11\x15V[4\x80\x15a\x05\x96W__\xFD[Pa\x02=a\x05\xA56`\x04a,\x8AV[a\x18zV[4\x80\x15a\x05\xB5W__\xFD[Pa\x02=`\x01T\x81V[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x05\xE3WPa\x05\xE3\x82a\x1D\xEDV[\x92\x91PPV[a\x05\xFF_Q` a1\x94_9_Q\x90_Ra\x1E!V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7Fw$\xBC\xB4:\t\xAEe\x82\xAF\xFD\xEE/\n\xCE\x93\x1E&\xF2\xFF\xA8\xB5\xC34\xBA\xF0\xA3\x9E\x9D\xC04&\x90_\x90\xA2PV[_\x90\x81R_Q` a1t_9_Q\x90_R` R`@\x90 `\x01\x01T\x90V[a\x06q\x82a\x06HV[a\x06z\x81a\x1E!V[a\x06\x84\x83\x83a\x1E.V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x06\xB3W`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\xBD\x82\x82a\x1EpV[PPPV[_a\x06\xCBa\x1E\xA9V[\x80T\x90\x91P`\xFF`\x01`@\x1B\x82\x04\x16\x15\x90`\x01`\x01`@\x1B\x03\x16_\x81\x15\x80\x15a\x06\xF1WP\x82[\x90P_\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x07\x0CWP0;\x15[\x90P\x81\x15\x80\x15a\x07\x1AWP\x80\x15[\x15a\x078W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x07bW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x07ja\x1E\xD1V[a\x07ra\x1E\xD1V[a\x07\x89_Q` a1\x94_9_Q\x90_R\x88a\x1E.V[a\x07\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x11\xDC\x98[\x9D\x08\x1C\x9B\xDB\x19H\x19\x98Z[\x19Y`z\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x07\xD8_\x88a\x1E.V[a\x08\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x11\xDC\x98[\x9D\x08\x1C\x9B\xDB\x19H\x19\x98Z[\x19Y`z\x1B`D\x82\x01R`d\x01a\x07\xC5V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x88\x16\x17\x90U\x83\x15a\x08yW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[``a\x08\x8E`\x06a\x1E\xDBV[\x90P\x90V[a\x08\x9Ba\x1E\xE7V[a\x08\xA4\x82a\x1F\x8BV[a\x08\xAE\x82\x82a\x1F\xA1V[PPV[_a\x08\xBBa ]V[P_Q` a1T_9_Q\x90_R\x90V[``a\x08\x8E`\x04a\x1E\xDBV[``a\x08\x8E`\x08a\x1E\xDBV[`\x02` R_\x90\x81R`@\x90 \x80T\x81\x90a\x08\xFF\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t+\x90a-&V[\x80\x15a\tvW\x80`\x1F\x10a\tMWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\tvV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tYW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\t\x8B\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xB7\x90a-&V[\x80\x15a\n\x02W\x80`\x1F\x10a\t\xD9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x02V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xE5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta\n\x17\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\nC\x90a-&V[\x80\x15a\n\x8EW\x80`\x1F\x10a\neWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x8EV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\nqW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x03\x01\x80Ta\n\xA3\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xCF\x90a-&V[\x80\x15a\x0B\x1AW\x80`\x1F\x10a\n\xF1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\x1AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xFDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x04\x01\x80Ta\x0B/\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B[\x90a-&V[\x80\x15a\x0B\xA6W\x80`\x1F\x10a\x0B}Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xA6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x89W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP`\x05\x90\x93\x01T\x91\x92PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x90`\xFF`\x01`\xA0\x1B\x90\x91\x04\x16\x87V[_\x82\x81R_Q` a14_9_Q\x90_R` \x81\x90R`@\x82 a\x0B\xF1\x90\x84a \xA6V[\x94\x93PPPPV[_\x91\x82R_Q` a1t_9_Q\x90_R` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[_\x81\x81R_Q` a14_9_Q\x90_R` \x81\x90R`@\x90\x91 ``\x91\x90a\x0CX\x90a\x1E\xDBV[\x93\x92PPPV[`@\x80Q_\x81R` \x81\x01\x90\x91R``\x90\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\x88Wa\x0C\x88a(\x1FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xBBW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xA6W\x90P[P\x91P_[\x83\x81\x10\x15a\r<Wa\r\x170\x86\x86\x84\x81\x81\x10a\x0C\xDEWa\x0C\xDEa-\x85V[\x90P` \x02\x81\x01\x90a\x0C\xF0\x91\x90a-\x99V[\x85`@Q` \x01a\r\x03\x93\x92\x91\x90a-\xF2V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra \xB1V[\x83\x82\x81Q\x81\x10a\r)Wa\r)a-\x85V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0C\xC0V[PP\x92\x91PPV[_a\x05\xE3`\x08\x83a!#V[a\r\x99`@Q\x80`\xE0\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81RP\x90V[_\x82\x81R`\x02` R`@\x90\x81\x90 \x81Q`\xE0\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\r\xC0\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\xEC\x90a-&V[\x80\x15a\x0E7W\x80`\x1F\x10a\x0E\x0EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\x1AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x0EP\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E|\x90a-&V[\x80\x15a\x0E\xC7W\x80`\x1F\x10a\x0E\x9EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\xC7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\xAAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x0E\xE0\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\x0C\x90a-&V[\x80\x15a\x0FWW\x80`\x1F\x10a\x0F.Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0FWV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F:W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x0Fp\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\x9C\x90a-&V[\x80\x15a\x0F\xE7W\x80`\x1F\x10a\x0F\xBEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\xE7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F\xCAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x04\x82\x01\x80Ta\x10\0\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10,\x90a-&V[\x80\x15a\x10wW\x80`\x1F\x10a\x10NWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10wV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10ZW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x05\x91\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16` \x83\x01R`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15\x15`@\x90\x91\x01R\x92\x91PPV[_\x81\x81R_Q` a14_9_Q\x90_R` \x81\x90R`@\x82 a\x0CX\x90a!:V[_a\x10\xDD`\x06\x83a!#V[\x80a\x05\xE3WPa\x05\xE3`\x08\x83a!#V[a\x10\xF7\x82a\x06HV[a\x11\0\x81a\x1E!V[a\x06\x84\x83\x83a\x1EpV[_a\x08\x8E`\x06a!:V[a\x11\x1Da!CV[a\x11&\x85a\x10\xD1V[a\x11\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FNo pending request with specifie`D\x82\x01Rj\x19\x08\x1C\x99\\]Y\\\xDD\x12Y`\xAA\x1B`d\x82\x01R`\x84\x01a\x07\xC5V[_\x85\x81R`\x02` R`@\x80\x82 \x81Q`\xE0\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x11\xAC\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x11\xD8\x90a-&V[\x80\x15a\x12#W\x80`\x1F\x10a\x11\xFAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12#V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12\x06W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x12<\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12h\x90a-&V[\x80\x15a\x12\xB3W\x80`\x1F\x10a\x12\x8AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12\xB3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12\x96W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x12\xCC\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\xF8\x90a-&V[\x80\x15a\x13CW\x80`\x1F\x10a\x13\x1AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13CV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13&W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x13\\\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x88\x90a-&V[\x80\x15a\x13\xD3W\x80`\x1F\x10a\x13\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xD3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xB6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x04\x82\x01\x80Ta\x13\xEC\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\x18\x90a-&V[\x80\x15a\x14cW\x80`\x1F\x10a\x14:Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14cV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x05\x91\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16` \x84\x01R`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15`@\x92\x83\x01R\x82Q`\x03T\x92Qc\x0Bv\x13\x9F`\xE3\x1B\x81R\x93\x94P\x92_\x92\x90\x91\x16\x90c[\xB0\x9C\xF8\x90a\x14\xC4\x90\x85\x90`\x04\x01a)CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xDFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x03\x91\x90a.\x11V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x15LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid scheme`\x90\x1B`D\x82\x01R`d\x01a\x07\xC5V[`@\x80\x84\x01Q\x90Qc\xEA\xE1\xE1[`\xE0\x1B\x81R\x82\x91_\x91`\x01`\x01`\xA0\x1B\x03\x84\x16\x91c\xEA\xE1\xE1[\x91a\x15\x80\x91\x90`\x04\x01a)CV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x9AW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x15\xC1\x91\x90\x81\x01\x90a.,V[\x90P\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF6\xE5H\xE9\x82\x89\x89\x86`\x01`\x01`\xA0\x1B\x03\x16c\xAC\xAE\x9F\xEE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x10W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x167\x91\x90\x81\x01\x90a.,V[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16V\x94\x93\x92\x91\x90a.\xC8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16qW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x95\x91\x90a/\x0CV[a\x16\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FSignature verification failed\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[_\x85`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c];\xE0\x01`\xE0\x1B\x8C\x8C\x8C\x8C\x8C`@Q`$\x01a\x17\x11\x95\x94\x93\x92\x91\x90a/+V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x17O\x91\x90a/cV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x17\x88W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x17\x8DV[``\x91P[PP_\x8C\x81R`\x02` R`@\x90 `\x05\x01\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U\x90Pa\x17\xBD`\x06\x8Ca!kV[P\x80a\x17\xFEWa\x17\xCE`\x08\x8Ca!vV[P`@Q\x8B\x90\x7F?\xED\x05\xC5\xD3\xA7\x9E\x0E/#\xFF\xD5\x94&\x7F\x0E)y\x9A\xD1\xB4\xB6\xA7ua8\xD5${\xAAJ{\x90_\x90\xA2a\x18dV[a\x18\x07\x8Ba\rDV[\x15a\x18\x19Wa\x18\x17`\x08\x8Ca!kV[P[a\x18$`\x04\x8Ca!vV[P\x8A\x7F\n\x1F[7\x82\xD8\xF6\x0454\x07\xC9\x80\xDB\xFEkUI\x9C[\x92\xCB\x8C9\xF0\x89\xC7?4\x84\x07\xE6\x8B\x8B\x8B\x8B`@Qa\x18[\x94\x93\x92\x91\x90a/nV[`@Q\x80\x91\x03\x90\xA2[PPPPPPa\x18s`\x01_UV[PPPPPV[_`\x01\x80_\x82\x82Ta\x18\x8C\x91\x90a/\x94V[\x90\x91UPP`\x03T`@Qc/\xC9\xFA3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c/\xC9\xFA3\x90a\x18\xC3\x90\x8A\x90\x8A\x90`\x04\x01a/\xA7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xDEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x02\x91\x90a/\x0CV[a\x19NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FSignature scheme not supported\0\0`D\x82\x01R`d\x01a\x07\xC5V[a\x19\x95`\x01a\x10\0\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x92\x94\x93\x92PPa!\x81\x90PV[a\x19\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FCiphertext failed length bounds `D\x82\x01Rdcheck`\xD8\x1B`d\x82\x01R`\x84\x01a\x07\xC5V[a\x1A6`\x01a\x10\0\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x92\x94\x93\x92PPa!\x81\x90PV[a\x1A\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FCondition failed length bounds c`D\x82\x01Rcheck`\xE0\x1B`d\x82\x01R`\x84\x01a\x07\xC5V[a\x1A\xCC\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\"\x11\x92PPPV[\x15a\x1B%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FCondition bytes cannot be all ze`D\x82\x01Rbros`\xE8\x1B`d\x82\x01R`\x84\x01a\x07\xC5V[`\x03T`@Qc\x0Bv\x13\x9F`\xE3\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c[\xB0\x9C\xF8\x90a\x1BW\x90\x8B\x90\x8B\x90`\x04\x01a/\xA7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BrW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x96\x91\x90a.\x11V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1B\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Finvalid signature scheme\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[`@Q\x80`\xE0\x01`@R\x80\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP`@\x80Q` `\x1F\x8A\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x88\x81R\x91\x81\x01\x91\x90\x89\x90\x89\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP`@\x80Q` `\x1F\x88\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x86\x81R\x91\x81\x01\x91\x90\x87\x90\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x93\x85RPP`@\x80Q` \x81\x81\x01\x83R\x84\x82R\x80\x86\x01\x91\x90\x91R\x81Q\x80\x82\x01\x83R\x84\x81R\x82\x86\x01R3``\x86\x01R`\x80\x90\x94\x01\x83\x90R`\x01T\x83R`\x02\x90\x93RP \x81Q\x81\x90a\x1C\xEE\x90\x82a/\xFEV[P` \x82\x01Q`\x01\x82\x01\x90a\x1D\x03\x90\x82a/\xFEV[P`@\x82\x01Q`\x02\x82\x01\x90a\x1D\x18\x90\x82a/\xFEV[P``\x82\x01Q`\x03\x82\x01\x90a\x1D-\x90\x82a/\xFEV[P`\x80\x82\x01Q`\x04\x82\x01\x90a\x1DB\x90\x82a/\xFEV[P`\xA0\x82\x01Q`\x05\x90\x91\x01\x80T`\xC0\x90\x93\x01Q\x15\x15`\x01`\xA0\x1B\x02`\x01`\x01`\xA8\x1B\x03\x19\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90U`\x01Ta\x1D\x8D\x90`\x06\x90a!vV[P3`\x01`\x01`\xA0\x1B\x03\x16`\x01T\x7F\x13;\x1A\xCF\xA4\x99 \xF8\x8B\xC8\xC2?\xAE\xB2\xBAC\xB7[=lE2\x90\xA8$\xF3\xC8^w3\xDF\x13\x8A\x8A\x88\x88\x8C\x8CB`@Qa\x1D\xD6\x97\x96\x95\x94\x93\x92\x91\x90a0\xB8V[`@Q\x80\x91\x03\x90\xA3PP`\x01T\x96\x95PPPPPPV[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x05\xE3WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x05\xE3V[a\x1E+\x813a\"\\V[PV[__Q` a14_9_Q\x90_R\x81a\x1EH\x85\x85a\"\x95V[\x90P\x80\x15a\x0B\xF1W_\x85\x81R` \x83\x90R`@\x90 a\x1Eg\x90\x85a#6V[P\x94\x93PPPPV[__Q` a14_9_Q\x90_R\x81a\x1E\x8A\x85\x85a#JV[\x90P\x80\x15a\x0B\xF1W_\x85\x81R` \x83\x90R`@\x90 a\x1Eg\x90\x85a#\xC3V[_\x80\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0a\x05\xE3V[a\x1E\xD9a#\xD7V[V[``_a\x0CX\x83a#\xFCV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x1FmWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x1Fa_Q` a1T_9_Q\x90_RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x1E\xD9W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E+_Q` a1\x94_9_Q\x90_Ra\x1E!V[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1F\xFBWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1F\xF8\x91\x81\x01\x90a1\x08V[`\x01[a #W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x07\xC5V[_Q` a1T_9_Q\x90_R\x81\x14a SW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xC5V[a\x06\xBD\x83\x83a$UV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1E\xD9W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0CX\x83\x83a$\xAAV[``__\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa \xCD\x91\x90a/cV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a!\x05W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a!\nV[``\x91P[P\x91P\x91Pa!\x1A\x85\x83\x83a$\xD0V[\x95\x94PPPPPV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x0CXV[_a\x05\xE3\x82T\x90V[`\x02_T\x03a!eW`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_UV[_a\x0CX\x83\x83a%,V[_a\x0CX\x83\x83a&\x06V[_\x81\x83\x11\x15a!\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FInvalid bounds: minLength cannot`D\x82\x01R\x7F be greater than maxLength\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC5V[\x83Q\x83\x81\x10\x80\x15\x90a!\x1AWP\x91\x90\x91\x11\x15\x93\x92PPPV[_\x80[\x82Q\x81\x10\x15a\"SW\x82\x81\x81Q\x81\x10a\"/Wa\"/a-\x85V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x15a\"KWP_\x92\x91PPV[`\x01\x01a\"\x14V[P`\x01\x92\x91PPV[a\"f\x82\x82a\x0B\xF9V[a\x08\xAEW`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x07\xC5V[__Q` a1t_9_Q\x90_Ra\"\xAE\x84\x84a\x0B\xF9V[a#-W_\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\"\xE33\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\x05\xE3V[_\x91PPa\x05\xE3V[_a\x0CX\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a&\x06V[__Q` a1t_9_Q\x90_Ra#c\x84\x84a\x0B\xF9V[\x15a#-W_\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\x05\xE3V[_a\x0CX\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a%,V[a#\xDFa&RV[a\x1E\xD9W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a$IW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a$5W[PPPPP\x90P\x91\x90PV[a$^\x82a&kV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a$\xA2Wa\x06\xBD\x82\x82a \xB1V[a\x08\xAEa&\xCEV[_\x82_\x01\x82\x81T\x81\x10a$\xBFWa$\xBFa-\x85V[\x90_R` _ \x01T\x90P\x92\x91PPV[``\x82a$\xE5Wa$\xE0\x82a&\xEDV[a\x0CXV[\x81Q\x15\x80\x15a$\xFCWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a%%W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x07\xC5V[P\x80a\x0CXV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a#-W_a%N`\x01\x83a-rV[\x85T\x90\x91P_\x90a%a\x90`\x01\x90a-rV[\x90P\x80\x82\x14a%\xC0W_\x86_\x01\x82\x81T\x81\x10a%\x7FWa%\x7Fa-\x85V[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a%\x9FWa%\x9Fa-\x85V[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a%\xD1Wa%\xD1a1\x1FV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x05\xE3V[_\x81\x81R`\x01\x83\x01` R`@\x81 Ta&KWP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x05\xE3V[P_a\x05\xE3V[_a&[a\x1E\xA9V[T`\x01`@\x1B\x90\x04`\xFF\x16\x91\x90PV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a&\xA0W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x07\xC5V[_Q` a1T_9_Q\x90_R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[4\x15a\x1E\xD9W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q\x15a&\xFDW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_` \x82\x84\x03\x12\x15a'&W__\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0CXW__\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1E+W__\xFD[_` \x82\x84\x03\x12\x15a'aW__\xFD[\x815a\x0CX\x81a'=V[_` \x82\x84\x03\x12\x15a'|W__\xFD[P5\x91\x90PV[__`@\x83\x85\x03\x12\x15a'\x94W__\xFD[\x825\x91P` \x83\x015a'\xA6\x81a'=V[\x80\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a'\xC2W__\xFD[\x825a'\xCD\x81a'=V[\x91P` \x83\x015a'\xA6\x81a'=V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a(\x14W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a'\xF6V[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a([Wa([a(\x1FV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a({Wa({a(\x1FV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[__`@\x83\x85\x03\x12\x15a(\x9AW__\xFD[\x825a(\xA5\x81a'=V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xBFW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a(\xCFW__\xFD[\x805a(\xE2a(\xDD\x82a(cV[a(3V[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a(\xF6W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x0CX` \x83\x01\x84a)\x15V[`\xE0\x81R_a)g`\xE0\x83\x01\x8Aa)\x15V[\x82\x81\x03` \x84\x01Ra)y\x81\x8Aa)\x15V[\x90P\x82\x81\x03`@\x84\x01Ra)\x8D\x81\x89a)\x15V[\x90P\x82\x81\x03``\x84\x01Ra)\xA1\x81\x88a)\x15V[\x90P\x82\x81\x03`\x80\x84\x01Ra)\xB5\x81\x87a)\x15V[`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16`\xA0\x84\x01RPP\x90\x15\x15`\xC0\x90\x91\x01R\x95\x94PPPPPV[__`@\x83\x85\x03\x12\x15a)\xEBW__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a(\x14W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a*\x13V[__` \x83\x85\x03\x12\x15a*KW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a*`W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a*pW__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a*\x85W__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a*\x99W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+\0W`?\x19\x87\x86\x03\x01\x84Ra*\xEB\x85\x83Qa)\x15V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a*\xCFV[P\x92\x96\x95PPPPPPV[` \x81R_\x82Q`\xE0` \x84\x01Ra+(a\x01\0\x84\x01\x82a)\x15V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra+E\x82\x82a)\x15V[\x91PP`@\x84\x01Q`\x1F\x19\x84\x83\x03\x01``\x85\x01Ra+c\x82\x82a)\x15V[\x91PP``\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\x80\x85\x01Ra+\x81\x82\x82a)\x15V[\x91PP`\x80\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\xA0\x85\x01Ra+\x9F\x82\x82a)\x15V[\x91PP`\x01\x80`\xA0\x1B\x03`\xA0\x85\x01Q\x16`\xC0\x84\x01R`\xC0\x84\x01Qa+\xC7`\xE0\x85\x01\x82\x15\x15\x90RV[P\x93\x92PPPV[__\x83`\x1F\x84\x01\x12a+\xDFW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a+\xF5W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a,\x0CW__\xFD[\x92P\x92\x90PV[_____``\x86\x88\x03\x12\x15a,'W__\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,CW__\xFD[a,O\x88\x82\x89\x01a+\xCFV[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,mW__\xFD[a,y\x88\x82\x89\x01a+\xCFV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[______``\x87\x89\x03\x12\x15a,\x9FW__\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15a,\xB4W__\xFD[a,\xC0\x89\x82\x8A\x01a+\xCFV[\x90\x97P\x95PP` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,\xDEW__\xFD[a,\xEA\x89\x82\x8A\x01a+\xCFV[\x90\x95P\x93PP`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-\x08W__\xFD[a-\x14\x89\x82\x8A\x01a+\xCFV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a-:W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a-XWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05\xE3Wa\x05\xE3a-^V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a-\xAEW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a-\xC7W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a,\x0CW__\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x82\x84\x827_\x83\x82\x01_\x81Ra.\x07\x81\x85a-\xDBV[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a.!W__\xFD[\x81Qa\x0CX\x81a'=V[_` \x82\x84\x03\x12\x15a.<W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a.QW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a.aW__\xFD[\x80Qa.oa(\xDD\x82a(cV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a.\x83W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[``\x81R_a.\xDA``\x83\x01\x87a)\x15V[\x82\x81\x03` \x84\x01Ra.\xED\x81\x86\x88a.\xA0V[\x90P\x82\x81\x03`@\x84\x01Ra/\x01\x81\x85a)\x15V[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a/\x1CW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x0CXW__\xFD[\x85\x81R``` \x82\x01R_a/D``\x83\x01\x86\x88a.\xA0V[\x82\x81\x03`@\x84\x01Ra/W\x81\x85\x87a.\xA0V[\x98\x97PPPPPPPPV[_a\x0CX\x82\x84a-\xDBV[`@\x81R_a/\x81`@\x83\x01\x86\x88a.\xA0V[\x82\x81\x03` \x84\x01Ra/\x01\x81\x85\x87a.\xA0V[\x80\x82\x01\x80\x82\x11\x15a\x05\xE3Wa\x05\xE3a-^V[` \x81R_a\x0B\xF1` \x83\x01\x84\x86a.\xA0V[`\x1F\x82\x11\x15a\x06\xBDW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a/\xDFWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x18sW_\x81U`\x01\x01a/\xEBV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x17Wa0\x17a(\x1FV[a0+\x81a0%\x84Ta-&V[\x84a/\xBAV[` `\x1F\x82\x11`\x01\x81\x14a0]W_\x83\x15a0FWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x18sV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a0\x8CW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a0lV[P\x84\x82\x10\x15a0\xA9W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80\x81R_a0\xCB`\x80\x83\x01\x89\x8Ba.\xA0V[\x82\x81\x03` \x84\x01Ra0\xDE\x81\x88\x8Aa.\xA0V[\x90P\x82\x81\x03`@\x84\x01Ra0\xF3\x81\x86\x88a.\xA0V[\x91PP\x82``\x83\x01R\x98\x97PPPPPPPPV[_` \x82\x84\x03\x12\x15a1\x18W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xC1\xF6\xFE$b\x1C\xE8\x1E\xC5\x82|\xAF\x02S\xCA\xDBtp\x9B\x06\x160\xE6\xB5^\x827\x17\x05\x93 \x006\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17u\xA2dipfsX\"\x12 \xAD\xCA\xC5eo\xBF\xF7F\x90\x96\xF3\x11|W#\x15\xDC^\xF2cU\xB5g\xE8.\x88)\x9A~\xD2j\xFEdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101c5575f3560e01c806391d14854116100f2578063ca15c87311610092578063e6b3ca7111610062578063e6b3ca711461054d578063f7cef8001461056c578063f87f0e611461058b578063fc2a88c3146105aa575f5ffd5b8063ca15c873146104dc578063cd802c91146104fb578063d547741f1461051a578063e63b5d5814610539575f5ffd5b8063ac9650d8116100cd578063ac9650d814610435578063ad3cb1cc14610461578063b094728914610491578063c58343ef146104b0575f5ffd5b806391d14854146103d7578063a217fddf146103f6578063a3246ad314610409575f5ffd5b80634f1ef286116101685780636f421ea9116101385780636f421ea91461033a57806375b238fc1461034e57806381d12c581461036e5780639010d07c146103a0575f5ffd5b80634f1ef286146102c957806352d1902d146102dc57806354fd4d50146102f0578063571d708714610326575f5ffd5b80632f2ff15d116101a35780632f2ff15d1461024b57806336568abe1461026a578063485cc955146102895780634b96e166146102a8575f5ffd5b806301ffc9a7146101c957806316cc9a98146101fd578063248a9ca31461021e575b5f5ffd5b3480156101d4575f5ffd5b506101e86101e3366004612716565b6105bf565b60405190151581526020015b60405180910390f35b348015610208575f5ffd5b5061021c610217366004612751565b6105e9565b005b348015610229575f5ffd5b5061023d61023836600461276c565b610648565b6040519081526020016101f4565b348015610256575f5ffd5b5061021c610265366004612783565b610668565b348015610275575f5ffd5b5061021c610284366004612783565b61068a565b348015610294575f5ffd5b5061021c6102a33660046127b1565b6106c2565b3480156102b3575f5ffd5b506102bc610882565b6040516101f491906127dd565b61021c6102d7366004612889565b610893565b3480156102e7575f5ffd5b5061023d6108b2565b3480156102fb575f5ffd5b50604080518082019091526005815264302e302e3160d81b60208201525b6040516101f49190612943565b348015610331575f5ffd5b506102bc6108cd565b348015610345575f5ffd5b506102bc6108d9565b348015610359575f5ffd5b5061023d5f5160206131945f395f51905f5281565b348015610379575f5ffd5b5061038d61038836600461276c565b6108e5565b6040516101f49796959493929190612955565b3480156103ab575f5ffd5b506103bf6103ba3660046129da565b610bcc565b6040516001600160a01b0390911681526020016101f4565b3480156103e2575f5ffd5b506101e86103f1366004612783565b610bf9565b348015610401575f5ffd5b5061023d5f81565b348015610414575f5ffd5b5061042861042336600461276c565b610c2f565b6040516101f491906129fa565b348015610440575f5ffd5b5061045461044f366004612a3a565b610c5f565b6040516101f49190612aa9565b34801561046c575f5ffd5b50610319604051806040016040528060058152602001640352e302e360dc1b81525081565b34801561049c575f5ffd5b506101e86104ab36600461276c565b610d44565b3480156104bb575f5ffd5b506104cf6104ca36600461276c565b610d50565b6040516101f49190612b0c565b3480156104e7575f5ffd5b5061023d6104f636600461276c565b6110ad565b348015610506575f5ffd5b506101e861051536600461276c565b6110d1565b348015610525575f5ffd5b5061021c610534366004612783565b6110ee565b348015610544575f5ffd5b5061023d61110a565b348015610558575f5ffd5b506003546103bf906001600160a01b031681565b348015610577575f5ffd5b5061021c610586366004612c13565b611115565b348015610596575f5ffd5b5061023d6105a5366004612c8a565b61187a565b3480156105b5575f5ffd5b5061023d60015481565b5f6001600160e01b03198216635a05180f60e01b14806105e357506105e382611ded565b92915050565b6105ff5f5160206131945f395f51905f52611e21565b600380546001600160a01b0319166001600160a01b0383169081179091556040517f7724bcb43a09ae6582affdee2f0ace931e26f2ffa8b5c334baf0a39e9dc03426905f90a250565b5f9081525f5160206131745f395f51905f52602052604090206001015490565b61067182610648565b61067a81611e21565b6106848383611e2e565b50505050565b6001600160a01b03811633146106b35760405163334bd91960e11b815260040160405180910390fd5b6106bd8282611e70565b505050565b5f6106cb611ea9565b805490915060ff600160401b82041615906001600160401b03165f811580156106f15750825b90505f826001600160401b0316600114801561070c5750303b155b90508115801561071a575080155b156107385760405163f92ee8a960e01b815260040160405180910390fd5b845467ffffffffffffffff19166001178555831561076257845460ff60401b1916600160401b1785555b61076a611ed1565b610772611ed1565b6107895f5160206131945f395f51905f5288611e2e565b6107ce5760405162461bcd60e51b815260206004820152601160248201527011dc985b9d081c9bdb194819985a5b1959607a1b60448201526064015b60405180910390fd5b6107d85f88611e2e565b6108185760405162461bcd60e51b815260206004820152601160248201527011dc985b9d081c9bdb194819985a5b1959607a1b60448201526064016107c5565b600380546001600160a01b0319166001600160a01b038816179055831561087957845460ff60401b19168555604051600181527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50505050505050565b606061088e6006611edb565b905090565b61089b611ee7565b6108a482611f8b565b6108ae8282611fa1565b5050565b5f6108bb61205d565b505f5160206131545f395f51905f5290565b606061088e6004611edb565b606061088e6008611edb565b60026020525f90815260409020805481906108ff90612d26565b80601f016020809104026020016040519081016040528092919081815260200182805461092b90612d26565b80156109765780601f1061094d57610100808354040283529160200191610976565b820191905f5260205f20905b81548152906001019060200180831161095957829003601f168201915b50505050509080600101805461098b90612d26565b80601f01602080910402602001604051908101604052809291908181526020018280546109b790612d26565b8015610a025780601f106109d957610100808354040283529160200191610a02565b820191905f5260205f20905b8154815290600101906020018083116109e557829003601f168201915b505050505090806002018054610a1790612d26565b80601f0160208091040260200160405190810160405280929190818152602001828054610a4390612d26565b8015610a8e5780601f10610a6557610100808354040283529160200191610a8e565b820191905f5260205f20905b815481529060010190602001808311610a7157829003601f168201915b505050505090806003018054610aa390612d26565b80601f0160208091040260200160405190810160405280929190818152602001828054610acf90612d26565b8015610b1a5780601f10610af157610100808354040283529160200191610b1a565b820191905f5260205f20905b815481529060010190602001808311610afd57829003601f168201915b505050505090806004018054610b2f90612d26565b80601f0160208091040260200160405190810160405280929190818152602001828054610b5b90612d26565b8015610ba65780601f10610b7d57610100808354040283529160200191610ba6565b820191905f5260205f20905b815481529060010190602001808311610b8957829003601f168201915b505050600590930154919250506001600160a01b0381169060ff600160a01b9091041687565b5f8281525f5160206131345f395f51905f52602081905260408220610bf190846120a6565b949350505050565b5f9182525f5160206131745f395f51905f52602090815260408084206001600160a01b0393909316845291905290205460ff1690565b5f8181525f5160206131345f395f51905f526020819052604090912060609190610c5890611edb565b9392505050565b604080515f815260208101909152606090826001600160401b03811115610c8857610c8861281f565b604051908082528060200260200182016040528015610cbb57816020015b6060815260200190600190039081610ca65790505b5091505f5b83811015610d3c57610d1730868684818110610cde57610cde612d85565b9050602002810190610cf09190612d99565b85604051602001610d0393929190612df2565b6040516020818303038152906040526120b1565b838281518110610d2957610d29612d85565b6020908102919091010152600101610cc0565b505092915050565b5f6105e3600883612123565b610d996040518060e0016040528060608152602001606081526020016060815260200160608152602001606081526020015f6001600160a01b031681526020015f151581525090565b5f8281526002602052604090819020815160e08101909252805482908290610dc090612d26565b80601f0160208091040260200160405190810160405280929190818152602001828054610dec90612d26565b8015610e375780601f10610e0e57610100808354040283529160200191610e37565b820191905f5260205f20905b815481529060010190602001808311610e1a57829003601f168201915b50505050508152602001600182018054610e5090612d26565b80601f0160208091040260200160405190810160405280929190818152602001828054610e7c90612d26565b8015610ec75780601f10610e9e57610100808354040283529160200191610ec7565b820191905f5260205f20905b815481529060010190602001808311610eaa57829003601f168201915b50505050508152602001600282018054610ee090612d26565b80601f0160208091040260200160405190810160405280929190818152602001828054610f0c90612d26565b8015610f575780601f10610f2e57610100808354040283529160200191610f57565b820191905f5260205f20905b815481529060010190602001808311610f3a57829003601f168201915b50505050508152602001600382018054610f7090612d26565b80601f0160208091040260200160405190810160405280929190818152602001828054610f9c90612d26565b8015610fe75780601f10610fbe57610100808354040283529160200191610fe7565b820191905f5260205f20905b815481529060010190602001808311610fca57829003601f168201915b5050505050815260200160048201805461100090612d26565b80601f016020809104026020016040519081016040528092919081815260200182805461102c90612d26565b80156110775780601f1061104e57610100808354040283529160200191611077565b820191905f5260205f20905b81548152906001019060200180831161105a57829003601f168201915b5050509183525050600591909101546001600160a01b0381166020830152600160a01b900460ff16151560409091015292915050565b5f8181525f5160206131345f395f51905f52602081905260408220610c589061213a565b5f6110dd600683612123565b806105e357506105e3600883612123565b6110f782610648565b61110081611e21565b6106848383611e70565b5f61088e600661213a565b61111d612143565b611126856110d1565b6111865760405162461bcd60e51b815260206004820152602b60248201527f4e6f2070656e64696e672072657175657374207769746820737065636966696560448201526a19081c995c5d595cdd125960aa1b60648201526084016107c5565b5f85815260026020526040808220815160e081019092528054829082906111ac90612d26565b80601f01602080910402602001604051908101604052809291908181526020018280546111d890612d26565b80156112235780601f106111fa57610100808354040283529160200191611223565b820191905f5260205f20905b81548152906001019060200180831161120657829003601f168201915b5050505050815260200160018201805461123c90612d26565b80601f016020809104026020016040519081016040528092919081815260200182805461126890612d26565b80156112b35780601f1061128a576101008083540402835291602001916112b3565b820191905f5260205f20905b81548152906001019060200180831161129657829003601f168201915b505050505081526020016002820180546112cc90612d26565b80601f01602080910402602001604051908101604052809291908181526020018280546112f890612d26565b80156113435780601f1061131a57610100808354040283529160200191611343565b820191905f5260205f20905b81548152906001019060200180831161132657829003601f168201915b5050505050815260200160038201805461135c90612d26565b80601f016020809104026020016040519081016040528092919081815260200182805461138890612d26565b80156113d35780601f106113aa576101008083540402835291602001916113d3565b820191905f5260205f20905b8154815290600101906020018083116113b657829003601f168201915b505050505081526020016004820180546113ec90612d26565b80601f016020809104026020016040519081016040528092919081815260200182805461141890612d26565b80156114635780601f1061143a57610100808354040283529160200191611463565b820191905f5260205f20905b81548152906001019060200180831161144657829003601f168201915b5050509183525050600591909101546001600160a01b038082166020840152600160a01b90910460ff16151560409283015282516003549251630b76139f60e31b8152939450925f9290911690635bb09cf8906114c4908590600401612943565b602060405180830381865afa1580156114df573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115039190612e11565b90506001600160a01b03811661154c5760405162461bcd60e51b815260206004820152600e60248201526d696e76616c696420736368656d6560901b60448201526064016107c5565b604080840151905163eae1e15b60e01b815282915f916001600160a01b0384169163eae1e15b916115809190600401612943565b5f60405180830381865afa15801561159a573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526115c19190810190612e2c565b9050816001600160a01b031663f6e548e9828989866001600160a01b031663acae9fee6040518163ffffffff1660e01b81526004015f60405180830381865afa158015611610573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526116379190810190612e2c565b6040518563ffffffff1660e01b81526004016116569493929190612ec8565b602060405180830381865afa158015611671573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116959190612f0c565b6116e15760405162461bcd60e51b815260206004820152601d60248201527f5369676e617475726520766572696669636174696f6e206661696c656400000060448201526064016107c5565b5f8560a001516001600160a01b0316635d3be00160e01b8c8c8c8c8c604051602401611711959493929190612f2b565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905161174f9190612f63565b5f604051808303815f865af19150503d805f8114611788576040519150601f19603f3d011682016040523d82523d5f602084013e61178d565b606091505b50505f8c8152600260205260409020600501805460ff60a01b1916600160a01b17905590506117bd60068c61216b565b50806117fe576117ce60088c612176565b506040518b907f3fed05c5d3a79e0e2f23ffd594267f0e29799ad1b4b6a7756138d5247baa4a7b905f90a2611864565b6118078b610d44565b156118195761181760088c61216b565b505b61182460048c612176565b508a7f0a1f5b3782d8f604353407c980dbfe6b55499c5b92cb8c39f089c73f348407e68b8b8b8b60405161185b9493929190612f6e565b60405180910390a25b50505050505061187360015f55565b5050505050565b5f6001805f82825461188c9190612f94565b9091555050600354604051632fc9fa3360e01b81526001600160a01b0390911690632fc9fa33906118c3908a908a90600401612fa7565b602060405180830381865afa1580156118de573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119029190612f0c565b61194e5760405162461bcd60e51b815260206004820152601e60248201527f5369676e617475726520736368656d65206e6f7420737570706f72746564000060448201526064016107c5565b611995600161100087878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152509294939250506121819050565b6119ef5760405162461bcd60e51b815260206004820152602560248201527f43697068657274657874206661696c6564206c656e67746820626f756e647320604482015264636865636b60d81b60648201526084016107c5565b611a36600161100085858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152509294939250506121819050565b611a8e5760405162461bcd60e51b8152602060048201526024808201527f436f6e646974696f6e206661696c6564206c656e67746820626f756e647320636044820152636865636b60e01b60648201526084016107c5565b611acc83838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061221192505050565b15611b255760405162461bcd60e51b815260206004820152602360248201527f436f6e646974696f6e2062797465732063616e6e6f7420626520616c6c207a65604482015262726f7360e81b60648201526084016107c5565b600354604051630b76139f60e31b81525f916001600160a01b031690635bb09cf890611b57908b908b90600401612fa7565b602060405180830381865afa158015611b72573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b969190612e11565b90506001600160a01b038116611bee5760405162461bcd60e51b815260206004820152601860248201527f696e76616c6964207369676e617475726520736368656d65000000000000000060448201526064016107c5565b6040518060e0016040528089898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250505090825250604080516020601f8a01819004810282018101909252888152918101919089908990819084018382808284375f92019190915250505090825250604080516020601f8801819004810282018101909252868152918101919087908790819084018382808284375f92018290525093855250506040805160208181018352848252808601919091528151808201835284815282860152336060860152608090940183905260015483526002909352502081518190611cee9082612ffe565b5060208201516001820190611d039082612ffe565b5060408201516002820190611d189082612ffe565b5060608201516003820190611d2d9082612ffe565b5060808201516004820190611d429082612ffe565b5060a08201516005909101805460c0909301511515600160a01b026001600160a81b03199093166001600160a01b0390921691909117919091179055600154611d8d90600690612176565b50336001600160a01b03166001547f133b1acfa49920f88bc8c23faeb2ba43b75b3d6c453290a824f3c85e7733df138a8a88888c8c42604051611dd697969594939291906130b8565b60405180910390a350506001549695505050505050565b5f6001600160e01b03198216637965db0b60e01b14806105e357506301ffc9a760e01b6001600160e01b03198316146105e3565b611e2b813361225c565b50565b5f5f5160206131345f395f51905f5281611e488585612295565b90508015610bf1575f858152602083905260409020611e679085612336565b50949350505050565b5f5f5160206131345f395f51905f5281611e8a858561234a565b90508015610bf1575f858152602083905260409020611e6790856123c3565b5f807ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a006105e3565b611ed96123d7565b565b60605f610c58836123fc565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161480611f6d57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316611f615f5160206131545f395f51905f52546001600160a01b031690565b6001600160a01b031614155b15611ed95760405163703e46dd60e11b815260040160405180910390fd5b611e2b5f5160206131945f395f51905f52611e21565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015611ffb575060408051601f3d908101601f19168201909252611ff891810190613108565b60015b61202357604051634c9c8ce360e01b81526001600160a01b03831660048201526024016107c5565b5f5160206131545f395f51905f52811461205357604051632a87526960e21b8152600481018290526024016107c5565b6106bd8383612455565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611ed95760405163703e46dd60e11b815260040160405180910390fd5b5f610c5883836124aa565b60605f5f846001600160a01b0316846040516120cd9190612f63565b5f60405180830381855af49150503d805f8114612105576040519150601f19603f3d011682016040523d82523d5f602084013e61210a565b606091505b509150915061211a8583836124d0565b95945050505050565b5f8181526001830160205260408120541515610c58565b5f6105e3825490565b60025f540361216557604051633ee5aeb560e01b815260040160405180910390fd5b60025f55565b5f610c58838361252c565b5f610c588383612606565b5f818311156121f85760405162461bcd60e51b815260206004820152603a60248201527f496e76616c696420626f756e64733a206d696e4c656e6774682063616e6e6f7460448201527f2062652067726561746572207468616e206d61784c656e67746800000000000060648201526084016107c5565b835183811080159061211a575091909111159392505050565b5f805b82518110156122535782818151811061222f5761222f612d85565b01602001516001600160f81b0319161561224b57505f92915050565b600101612214565b50600192915050565b6122668282610bf9565b6108ae5760405163e2517d3f60e01b81526001600160a01b0382166004820152602481018390526044016107c5565b5f5f5160206131745f395f51905f526122ae8484610bf9565b61232d575f848152602082815260408083206001600160a01b03871684529091529020805460ff191660011790556122e33390565b6001600160a01b0316836001600160a01b0316857f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019150506105e3565b5f9150506105e3565b5f610c58836001600160a01b038416612606565b5f5f5160206131745f395f51905f526123638484610bf9565b1561232d575f848152602082815260408083206001600160a01b0387168085529252808320805460ff1916905551339287917ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9190a460019150506105e3565b5f610c58836001600160a01b03841661252c565b6123df612652565b611ed957604051631afcd79f60e31b815260040160405180910390fd5b6060815f0180548060200260200160405190810160405280929190818152602001828054801561244957602002820191905f5260205f20905b815481526020019060010190808311612435575b50505050509050919050565b61245e8261266b565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a28051156124a2576106bd82826120b1565b6108ae6126ce565b5f825f0182815481106124bf576124bf612d85565b905f5260205f200154905092915050565b6060826124e5576124e0826126ed565b610c58565b81511580156124fc57506001600160a01b0384163b155b1561252557604051639996b31560e01b81526001600160a01b03851660048201526024016107c5565b5080610c58565b5f818152600183016020526040812054801561232d575f61254e600183612d72565b85549091505f9061256190600190612d72565b90508082146125c0575f865f01828154811061257f5761257f612d85565b905f5260205f200154905080875f01848154811061259f5761259f612d85565b5f918252602080832090910192909255918252600188019052604090208390555b85548690806125d1576125d161311f565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f9055600193505050506105e3565b5f81815260018301602052604081205461264b57508154600181810184555f8481526020808220909301849055845484825282860190935260409020919091556105e3565b505f6105e3565b5f61265b611ea9565b54600160401b900460ff16919050565b806001600160a01b03163b5f036126a057604051634c9c8ce360e01b81526001600160a01b03821660048201526024016107c5565b5f5160206131545f395f51905f5280546001600160a01b0319166001600160a01b0392909216919091179055565b3415611ed95760405163b398979f60e01b815260040160405180910390fd5b8051156126fd5780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f60208284031215612726575f5ffd5b81356001600160e01b031981168114610c58575f5ffd5b6001600160a01b0381168114611e2b575f5ffd5b5f60208284031215612761575f5ffd5b8135610c588161273d565b5f6020828403121561277c575f5ffd5b5035919050565b5f5f60408385031215612794575f5ffd5b8235915060208301356127a68161273d565b809150509250929050565b5f5f604083850312156127c2575f5ffd5b82356127cd8161273d565b915060208301356127a68161273d565b602080825282518282018190525f918401906040840190835b818110156128145783518352602093840193909201916001016127f6565b509095945050505050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b038111828210171561285b5761285b61281f565b604052919050565b5f6001600160401b0382111561287b5761287b61281f565b50601f01601f191660200190565b5f5f6040838503121561289a575f5ffd5b82356128a58161273d565b915060208301356001600160401b038111156128bf575f5ffd5b8301601f810185136128cf575f5ffd5b80356128e26128dd82612863565b612833565b8181528660208385010111156128f6575f5ffd5b816020840160208301375f602083830101528093505050509250929050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f610c586020830184612915565b60e081525f61296760e083018a612915565b8281036020840152612979818a612915565b9050828103604084015261298d8189612915565b905082810360608401526129a18188612915565b905082810360808401526129b58187612915565b6001600160a01b039590951660a0840152505090151560c09091015295945050505050565b5f5f604083850312156129eb575f5ffd5b50508035926020909101359150565b602080825282518282018190525f918401906040840190835b818110156128145783516001600160a01b0316835260209384019390920191600101612a13565b5f5f60208385031215612a4b575f5ffd5b82356001600160401b03811115612a60575f5ffd5b8301601f81018513612a70575f5ffd5b80356001600160401b03811115612a85575f5ffd5b8560208260051b8401011115612a99575f5ffd5b6020919091019590945092505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b0057603f19878603018452612aeb858351612915565b94506020938401939190910190600101612acf565b50929695505050505050565b602081525f825160e06020840152612b28610100840182612915565b90506020840151601f19848303016040850152612b458282612915565b9150506040840151601f19848303016060850152612b638282612915565b9150506060840151601f19848303016080850152612b818282612915565b9150506080840151601f198483030160a0850152612b9f8282612915565b91505060018060a01b0360a08501511660c084015260c0840151612bc760e085018215159052565b509392505050565b5f5f83601f840112612bdf575f5ffd5b5081356001600160401b03811115612bf5575f5ffd5b602083019150836020828501011115612c0c575f5ffd5b9250929050565b5f5f5f5f5f60608688031215612c27575f5ffd5b8535945060208601356001600160401b03811115612c43575f5ffd5b612c4f88828901612bcf565b90955093505060408601356001600160401b03811115612c6d575f5ffd5b612c7988828901612bcf565b969995985093965092949392505050565b5f5f5f5f5f5f60608789031215612c9f575f5ffd5b86356001600160401b03811115612cb4575f5ffd5b612cc089828a01612bcf565b90975095505060208701356001600160401b03811115612cde575f5ffd5b612cea89828a01612bcf565b90955093505060408701356001600160401b03811115612d08575f5ffd5b612d1489828a01612bcf565b979a9699509497509295939492505050565b600181811c90821680612d3a57607f821691505b602082108103612d5857634e487b7160e01b5f52602260045260245ffd5b50919050565b634e487b7160e01b5f52601160045260245ffd5b818103818111156105e3576105e3612d5e565b634e487b7160e01b5f52603260045260245ffd5b5f5f8335601e19843603018112612dae575f5ffd5b8301803591506001600160401b03821115612dc7575f5ffd5b602001915036819003821315612c0c575f5ffd5b5f81518060208401855e5f93019283525090919050565b828482375f8382015f8152612e078185612ddb565b9695505050505050565b5f60208284031215612e21575f5ffd5b8151610c588161273d565b5f60208284031215612e3c575f5ffd5b81516001600160401b03811115612e51575f5ffd5b8201601f81018413612e61575f5ffd5b8051612e6f6128dd82612863565b818152856020838501011115612e83575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b606081525f612eda6060830187612915565b8281036020840152612eed818688612ea0565b90508281036040840152612f018185612915565b979650505050505050565b5f60208284031215612f1c575f5ffd5b81518015158114610c58575f5ffd5b858152606060208201525f612f44606083018688612ea0565b8281036040840152612f57818587612ea0565b98975050505050505050565b5f610c588284612ddb565b604081525f612f81604083018688612ea0565b8281036020840152612f01818587612ea0565b808201808211156105e3576105e3612d5e565b602081525f610bf1602083018486612ea0565b601f8211156106bd57805f5260205f20601f840160051c81016020851015612fdf5750805b601f840160051c820191505b81811015611873575f8155600101612feb565b81516001600160401b038111156130175761301761281f565b61302b816130258454612d26565b84612fba565b6020601f82116001811461305d575f83156130465750848201515b5f19600385901b1c1916600184901b178455611873565b5f84815260208120601f198516915b8281101561308c578785015182556020948501946001909201910161306c565b50848210156130a957868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b608081525f6130cb60808301898b612ea0565b82810360208401526130de81888a612ea0565b905082810360408401526130f3818688612ea0565b91505082606083015298975050505050505050565b5f60208284031215613118575f5ffd5b5051919050565b634e487b7160e01b5f52603160045260245ffdfec1f6fe24621ce81ec5827caf0253cadb74709b061630e6b55e82371705932000360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b626800a49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c21775a2646970667358221220adcac5656fbff7469096f3117c572315dc5ef26355b567e82e88299a7ed26afe64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xC5W_5`\xE0\x1C\x80c\x91\xD1HT\x11a\0\xF2W\x80c\xCA\x15\xC8s\x11a\0\x92W\x80c\xE6\xB3\xCAq\x11a\0bW\x80c\xE6\xB3\xCAq\x14a\x05MW\x80c\xF7\xCE\xF8\0\x14a\x05lW\x80c\xF8\x7F\x0Ea\x14a\x05\x8BW\x80c\xFC*\x88\xC3\x14a\x05\xAAW__\xFD[\x80c\xCA\x15\xC8s\x14a\x04\xDCW\x80c\xCD\x80,\x91\x14a\x04\xFBW\x80c\xD5Gt\x1F\x14a\x05\x1AW\x80c\xE6;]X\x14a\x059W__\xFD[\x80c\xAC\x96P\xD8\x11a\0\xCDW\x80c\xAC\x96P\xD8\x14a\x045W\x80c\xAD<\xB1\xCC\x14a\x04aW\x80c\xB0\x94r\x89\x14a\x04\x91W\x80c\xC5\x83C\xEF\x14a\x04\xB0W__\xFD[\x80c\x91\xD1HT\x14a\x03\xD7W\x80c\xA2\x17\xFD\xDF\x14a\x03\xF6W\x80c\xA3$j\xD3\x14a\x04\tW__\xFD[\x80cO\x1E\xF2\x86\x11a\x01hW\x80coB\x1E\xA9\x11a\x018W\x80coB\x1E\xA9\x14a\x03:W\x80cu\xB28\xFC\x14a\x03NW\x80c\x81\xD1,X\x14a\x03nW\x80c\x90\x10\xD0|\x14a\x03\xA0W__\xFD[\x80cO\x1E\xF2\x86\x14a\x02\xC9W\x80cR\xD1\x90-\x14a\x02\xDCW\x80cT\xFDMP\x14a\x02\xF0W\x80cW\x1Dp\x87\x14a\x03&W__\xFD[\x80c//\xF1]\x11a\x01\xA3W\x80c//\xF1]\x14a\x02KW\x80c6V\x8A\xBE\x14a\x02jW\x80cH\\\xC9U\x14a\x02\x89W\x80cK\x96\xE1f\x14a\x02\xA8W__\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xC9W\x80c\x16\xCC\x9A\x98\x14a\x01\xFDW\x80c$\x8A\x9C\xA3\x14a\x02\x1EW[__\xFD[4\x80\x15a\x01\xD4W__\xFD[Pa\x01\xE8a\x01\xE36`\x04a'\x16V[a\x05\xBFV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x08W__\xFD[Pa\x02\x1Ca\x02\x176`\x04a'QV[a\x05\xE9V[\0[4\x80\x15a\x02)W__\xFD[Pa\x02=a\x0286`\x04a'lV[a\x06HV[`@Q\x90\x81R` \x01a\x01\xF4V[4\x80\x15a\x02VW__\xFD[Pa\x02\x1Ca\x02e6`\x04a'\x83V[a\x06hV[4\x80\x15a\x02uW__\xFD[Pa\x02\x1Ca\x02\x846`\x04a'\x83V[a\x06\x8AV[4\x80\x15a\x02\x94W__\xFD[Pa\x02\x1Ca\x02\xA36`\x04a'\xB1V[a\x06\xC2V[4\x80\x15a\x02\xB3W__\xFD[Pa\x02\xBCa\x08\x82V[`@Qa\x01\xF4\x91\x90a'\xDDV[a\x02\x1Ca\x02\xD76`\x04a(\x89V[a\x08\x93V[4\x80\x15a\x02\xE7W__\xFD[Pa\x02=a\x08\xB2V[4\x80\x15a\x02\xFBW__\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rd0.0.1`\xD8\x1B` \x82\x01R[`@Qa\x01\xF4\x91\x90a)CV[4\x80\x15a\x031W__\xFD[Pa\x02\xBCa\x08\xCDV[4\x80\x15a\x03EW__\xFD[Pa\x02\xBCa\x08\xD9V[4\x80\x15a\x03YW__\xFD[Pa\x02=_Q` a1\x94_9_Q\x90_R\x81V[4\x80\x15a\x03yW__\xFD[Pa\x03\x8Da\x03\x886`\x04a'lV[a\x08\xE5V[`@Qa\x01\xF4\x97\x96\x95\x94\x93\x92\x91\x90a)UV[4\x80\x15a\x03\xABW__\xFD[Pa\x03\xBFa\x03\xBA6`\x04a)\xDAV[a\x0B\xCCV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xF4V[4\x80\x15a\x03\xE2W__\xFD[Pa\x01\xE8a\x03\xF16`\x04a'\x83V[a\x0B\xF9V[4\x80\x15a\x04\x01W__\xFD[Pa\x02=_\x81V[4\x80\x15a\x04\x14W__\xFD[Pa\x04(a\x04#6`\x04a'lV[a\x0C/V[`@Qa\x01\xF4\x91\x90a)\xFAV[4\x80\x15a\x04@W__\xFD[Pa\x04Ta\x04O6`\x04a*:V[a\x0C_V[`@Qa\x01\xF4\x91\x90a*\xA9V[4\x80\x15a\x04lW__\xFD[Pa\x03\x19`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x04\x9CW__\xFD[Pa\x01\xE8a\x04\xAB6`\x04a'lV[a\rDV[4\x80\x15a\x04\xBBW__\xFD[Pa\x04\xCFa\x04\xCA6`\x04a'lV[a\rPV[`@Qa\x01\xF4\x91\x90a+\x0CV[4\x80\x15a\x04\xE7W__\xFD[Pa\x02=a\x04\xF66`\x04a'lV[a\x10\xADV[4\x80\x15a\x05\x06W__\xFD[Pa\x01\xE8a\x05\x156`\x04a'lV[a\x10\xD1V[4\x80\x15a\x05%W__\xFD[Pa\x02\x1Ca\x0546`\x04a'\x83V[a\x10\xEEV[4\x80\x15a\x05DW__\xFD[Pa\x02=a\x11\nV[4\x80\x15a\x05XW__\xFD[P`\x03Ta\x03\xBF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05wW__\xFD[Pa\x02\x1Ca\x05\x866`\x04a,\x13V[a\x11\x15V[4\x80\x15a\x05\x96W__\xFD[Pa\x02=a\x05\xA56`\x04a,\x8AV[a\x18zV[4\x80\x15a\x05\xB5W__\xFD[Pa\x02=`\x01T\x81V[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x05\xE3WPa\x05\xE3\x82a\x1D\xEDV[\x92\x91PPV[a\x05\xFF_Q` a1\x94_9_Q\x90_Ra\x1E!V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7Fw$\xBC\xB4:\t\xAEe\x82\xAF\xFD\xEE/\n\xCE\x93\x1E&\xF2\xFF\xA8\xB5\xC34\xBA\xF0\xA3\x9E\x9D\xC04&\x90_\x90\xA2PV[_\x90\x81R_Q` a1t_9_Q\x90_R` R`@\x90 `\x01\x01T\x90V[a\x06q\x82a\x06HV[a\x06z\x81a\x1E!V[a\x06\x84\x83\x83a\x1E.V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x06\xB3W`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\xBD\x82\x82a\x1EpV[PPPV[_a\x06\xCBa\x1E\xA9V[\x80T\x90\x91P`\xFF`\x01`@\x1B\x82\x04\x16\x15\x90`\x01`\x01`@\x1B\x03\x16_\x81\x15\x80\x15a\x06\xF1WP\x82[\x90P_\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x07\x0CWP0;\x15[\x90P\x81\x15\x80\x15a\x07\x1AWP\x80\x15[\x15a\x078W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x07bW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x07ja\x1E\xD1V[a\x07ra\x1E\xD1V[a\x07\x89_Q` a1\x94_9_Q\x90_R\x88a\x1E.V[a\x07\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x11\xDC\x98[\x9D\x08\x1C\x9B\xDB\x19H\x19\x98Z[\x19Y`z\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x07\xD8_\x88a\x1E.V[a\x08\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x11\xDC\x98[\x9D\x08\x1C\x9B\xDB\x19H\x19\x98Z[\x19Y`z\x1B`D\x82\x01R`d\x01a\x07\xC5V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x88\x16\x17\x90U\x83\x15a\x08yW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[``a\x08\x8E`\x06a\x1E\xDBV[\x90P\x90V[a\x08\x9Ba\x1E\xE7V[a\x08\xA4\x82a\x1F\x8BV[a\x08\xAE\x82\x82a\x1F\xA1V[PPV[_a\x08\xBBa ]V[P_Q` a1T_9_Q\x90_R\x90V[``a\x08\x8E`\x04a\x1E\xDBV[``a\x08\x8E`\x08a\x1E\xDBV[`\x02` R_\x90\x81R`@\x90 \x80T\x81\x90a\x08\xFF\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t+\x90a-&V[\x80\x15a\tvW\x80`\x1F\x10a\tMWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\tvV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tYW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\t\x8B\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xB7\x90a-&V[\x80\x15a\n\x02W\x80`\x1F\x10a\t\xD9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x02V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xE5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta\n\x17\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\nC\x90a-&V[\x80\x15a\n\x8EW\x80`\x1F\x10a\neWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x8EV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\nqW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x03\x01\x80Ta\n\xA3\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xCF\x90a-&V[\x80\x15a\x0B\x1AW\x80`\x1F\x10a\n\xF1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\x1AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xFDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x04\x01\x80Ta\x0B/\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B[\x90a-&V[\x80\x15a\x0B\xA6W\x80`\x1F\x10a\x0B}Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xA6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x89W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP`\x05\x90\x93\x01T\x91\x92PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x90`\xFF`\x01`\xA0\x1B\x90\x91\x04\x16\x87V[_\x82\x81R_Q` a14_9_Q\x90_R` \x81\x90R`@\x82 a\x0B\xF1\x90\x84a \xA6V[\x94\x93PPPPV[_\x91\x82R_Q` a1t_9_Q\x90_R` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[_\x81\x81R_Q` a14_9_Q\x90_R` \x81\x90R`@\x90\x91 ``\x91\x90a\x0CX\x90a\x1E\xDBV[\x93\x92PPPV[`@\x80Q_\x81R` \x81\x01\x90\x91R``\x90\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\x88Wa\x0C\x88a(\x1FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xBBW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xA6W\x90P[P\x91P_[\x83\x81\x10\x15a\r<Wa\r\x170\x86\x86\x84\x81\x81\x10a\x0C\xDEWa\x0C\xDEa-\x85V[\x90P` \x02\x81\x01\x90a\x0C\xF0\x91\x90a-\x99V[\x85`@Q` \x01a\r\x03\x93\x92\x91\x90a-\xF2V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra \xB1V[\x83\x82\x81Q\x81\x10a\r)Wa\r)a-\x85V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0C\xC0V[PP\x92\x91PPV[_a\x05\xE3`\x08\x83a!#V[a\r\x99`@Q\x80`\xE0\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81RP\x90V[_\x82\x81R`\x02` R`@\x90\x81\x90 \x81Q`\xE0\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\r\xC0\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\xEC\x90a-&V[\x80\x15a\x0E7W\x80`\x1F\x10a\x0E\x0EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\x1AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x0EP\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E|\x90a-&V[\x80\x15a\x0E\xC7W\x80`\x1F\x10a\x0E\x9EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\xC7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\xAAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x0E\xE0\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\x0C\x90a-&V[\x80\x15a\x0FWW\x80`\x1F\x10a\x0F.Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0FWV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F:W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x0Fp\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\x9C\x90a-&V[\x80\x15a\x0F\xE7W\x80`\x1F\x10a\x0F\xBEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\xE7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F\xCAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x04\x82\x01\x80Ta\x10\0\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10,\x90a-&V[\x80\x15a\x10wW\x80`\x1F\x10a\x10NWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10wV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10ZW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x05\x91\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16` \x83\x01R`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15\x15`@\x90\x91\x01R\x92\x91PPV[_\x81\x81R_Q` a14_9_Q\x90_R` \x81\x90R`@\x82 a\x0CX\x90a!:V[_a\x10\xDD`\x06\x83a!#V[\x80a\x05\xE3WPa\x05\xE3`\x08\x83a!#V[a\x10\xF7\x82a\x06HV[a\x11\0\x81a\x1E!V[a\x06\x84\x83\x83a\x1EpV[_a\x08\x8E`\x06a!:V[a\x11\x1Da!CV[a\x11&\x85a\x10\xD1V[a\x11\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FNo pending request with specifie`D\x82\x01Rj\x19\x08\x1C\x99\\]Y\\\xDD\x12Y`\xAA\x1B`d\x82\x01R`\x84\x01a\x07\xC5V[_\x85\x81R`\x02` R`@\x80\x82 \x81Q`\xE0\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x11\xAC\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x11\xD8\x90a-&V[\x80\x15a\x12#W\x80`\x1F\x10a\x11\xFAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12#V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12\x06W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x12<\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12h\x90a-&V[\x80\x15a\x12\xB3W\x80`\x1F\x10a\x12\x8AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12\xB3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12\x96W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x12\xCC\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\xF8\x90a-&V[\x80\x15a\x13CW\x80`\x1F\x10a\x13\x1AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13CV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13&W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x13\\\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x88\x90a-&V[\x80\x15a\x13\xD3W\x80`\x1F\x10a\x13\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xD3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xB6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x04\x82\x01\x80Ta\x13\xEC\x90a-&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\x18\x90a-&V[\x80\x15a\x14cW\x80`\x1F\x10a\x14:Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14cV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x05\x91\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16` \x84\x01R`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15`@\x92\x83\x01R\x82Q`\x03T\x92Qc\x0Bv\x13\x9F`\xE3\x1B\x81R\x93\x94P\x92_\x92\x90\x91\x16\x90c[\xB0\x9C\xF8\x90a\x14\xC4\x90\x85\x90`\x04\x01a)CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xDFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x03\x91\x90a.\x11V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x15LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid scheme`\x90\x1B`D\x82\x01R`d\x01a\x07\xC5V[`@\x80\x84\x01Q\x90Qc\xEA\xE1\xE1[`\xE0\x1B\x81R\x82\x91_\x91`\x01`\x01`\xA0\x1B\x03\x84\x16\x91c\xEA\xE1\xE1[\x91a\x15\x80\x91\x90`\x04\x01a)CV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x9AW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x15\xC1\x91\x90\x81\x01\x90a.,V[\x90P\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF6\xE5H\xE9\x82\x89\x89\x86`\x01`\x01`\xA0\x1B\x03\x16c\xAC\xAE\x9F\xEE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x10W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x167\x91\x90\x81\x01\x90a.,V[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16V\x94\x93\x92\x91\x90a.\xC8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16qW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x95\x91\x90a/\x0CV[a\x16\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FSignature verification failed\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[_\x85`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c];\xE0\x01`\xE0\x1B\x8C\x8C\x8C\x8C\x8C`@Q`$\x01a\x17\x11\x95\x94\x93\x92\x91\x90a/+V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x17O\x91\x90a/cV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x17\x88W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x17\x8DV[``\x91P[PP_\x8C\x81R`\x02` R`@\x90 `\x05\x01\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U\x90Pa\x17\xBD`\x06\x8Ca!kV[P\x80a\x17\xFEWa\x17\xCE`\x08\x8Ca!vV[P`@Q\x8B\x90\x7F?\xED\x05\xC5\xD3\xA7\x9E\x0E/#\xFF\xD5\x94&\x7F\x0E)y\x9A\xD1\xB4\xB6\xA7ua8\xD5${\xAAJ{\x90_\x90\xA2a\x18dV[a\x18\x07\x8Ba\rDV[\x15a\x18\x19Wa\x18\x17`\x08\x8Ca!kV[P[a\x18$`\x04\x8Ca!vV[P\x8A\x7F\n\x1F[7\x82\xD8\xF6\x0454\x07\xC9\x80\xDB\xFEkUI\x9C[\x92\xCB\x8C9\xF0\x89\xC7?4\x84\x07\xE6\x8B\x8B\x8B\x8B`@Qa\x18[\x94\x93\x92\x91\x90a/nV[`@Q\x80\x91\x03\x90\xA2[PPPPPPa\x18s`\x01_UV[PPPPPV[_`\x01\x80_\x82\x82Ta\x18\x8C\x91\x90a/\x94V[\x90\x91UPP`\x03T`@Qc/\xC9\xFA3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c/\xC9\xFA3\x90a\x18\xC3\x90\x8A\x90\x8A\x90`\x04\x01a/\xA7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xDEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x02\x91\x90a/\x0CV[a\x19NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FSignature scheme not supported\0\0`D\x82\x01R`d\x01a\x07\xC5V[a\x19\x95`\x01a\x10\0\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x92\x94\x93\x92PPa!\x81\x90PV[a\x19\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FCiphertext failed length bounds `D\x82\x01Rdcheck`\xD8\x1B`d\x82\x01R`\x84\x01a\x07\xC5V[a\x1A6`\x01a\x10\0\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x92\x94\x93\x92PPa!\x81\x90PV[a\x1A\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FCondition failed length bounds c`D\x82\x01Rcheck`\xE0\x1B`d\x82\x01R`\x84\x01a\x07\xC5V[a\x1A\xCC\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\"\x11\x92PPPV[\x15a\x1B%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FCondition bytes cannot be all ze`D\x82\x01Rbros`\xE8\x1B`d\x82\x01R`\x84\x01a\x07\xC5V[`\x03T`@Qc\x0Bv\x13\x9F`\xE3\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c[\xB0\x9C\xF8\x90a\x1BW\x90\x8B\x90\x8B\x90`\x04\x01a/\xA7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BrW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x96\x91\x90a.\x11V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1B\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Finvalid signature scheme\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[`@Q\x80`\xE0\x01`@R\x80\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP`@\x80Q` `\x1F\x8A\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x88\x81R\x91\x81\x01\x91\x90\x89\x90\x89\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP`@\x80Q` `\x1F\x88\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x86\x81R\x91\x81\x01\x91\x90\x87\x90\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x93\x85RPP`@\x80Q` \x81\x81\x01\x83R\x84\x82R\x80\x86\x01\x91\x90\x91R\x81Q\x80\x82\x01\x83R\x84\x81R\x82\x86\x01R3``\x86\x01R`\x80\x90\x94\x01\x83\x90R`\x01T\x83R`\x02\x90\x93RP \x81Q\x81\x90a\x1C\xEE\x90\x82a/\xFEV[P` \x82\x01Q`\x01\x82\x01\x90a\x1D\x03\x90\x82a/\xFEV[P`@\x82\x01Q`\x02\x82\x01\x90a\x1D\x18\x90\x82a/\xFEV[P``\x82\x01Q`\x03\x82\x01\x90a\x1D-\x90\x82a/\xFEV[P`\x80\x82\x01Q`\x04\x82\x01\x90a\x1DB\x90\x82a/\xFEV[P`\xA0\x82\x01Q`\x05\x90\x91\x01\x80T`\xC0\x90\x93\x01Q\x15\x15`\x01`\xA0\x1B\x02`\x01`\x01`\xA8\x1B\x03\x19\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90U`\x01Ta\x1D\x8D\x90`\x06\x90a!vV[P3`\x01`\x01`\xA0\x1B\x03\x16`\x01T\x7F\x13;\x1A\xCF\xA4\x99 \xF8\x8B\xC8\xC2?\xAE\xB2\xBAC\xB7[=lE2\x90\xA8$\xF3\xC8^w3\xDF\x13\x8A\x8A\x88\x88\x8C\x8CB`@Qa\x1D\xD6\x97\x96\x95\x94\x93\x92\x91\x90a0\xB8V[`@Q\x80\x91\x03\x90\xA3PP`\x01T\x96\x95PPPPPPV[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x05\xE3WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x05\xE3V[a\x1E+\x813a\"\\V[PV[__Q` a14_9_Q\x90_R\x81a\x1EH\x85\x85a\"\x95V[\x90P\x80\x15a\x0B\xF1W_\x85\x81R` \x83\x90R`@\x90 a\x1Eg\x90\x85a#6V[P\x94\x93PPPPV[__Q` a14_9_Q\x90_R\x81a\x1E\x8A\x85\x85a#JV[\x90P\x80\x15a\x0B\xF1W_\x85\x81R` \x83\x90R`@\x90 a\x1Eg\x90\x85a#\xC3V[_\x80\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0a\x05\xE3V[a\x1E\xD9a#\xD7V[V[``_a\x0CX\x83a#\xFCV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x1FmWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x1Fa_Q` a1T_9_Q\x90_RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x1E\xD9W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E+_Q` a1\x94_9_Q\x90_Ra\x1E!V[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1F\xFBWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1F\xF8\x91\x81\x01\x90a1\x08V[`\x01[a #W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x07\xC5V[_Q` a1T_9_Q\x90_R\x81\x14a SW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xC5V[a\x06\xBD\x83\x83a$UV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1E\xD9W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0CX\x83\x83a$\xAAV[``__\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa \xCD\x91\x90a/cV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a!\x05W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a!\nV[``\x91P[P\x91P\x91Pa!\x1A\x85\x83\x83a$\xD0V[\x95\x94PPPPPV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x0CXV[_a\x05\xE3\x82T\x90V[`\x02_T\x03a!eW`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_UV[_a\x0CX\x83\x83a%,V[_a\x0CX\x83\x83a&\x06V[_\x81\x83\x11\x15a!\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FInvalid bounds: minLength cannot`D\x82\x01R\x7F be greater than maxLength\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC5V[\x83Q\x83\x81\x10\x80\x15\x90a!\x1AWP\x91\x90\x91\x11\x15\x93\x92PPPV[_\x80[\x82Q\x81\x10\x15a\"SW\x82\x81\x81Q\x81\x10a\"/Wa\"/a-\x85V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x15a\"KWP_\x92\x91PPV[`\x01\x01a\"\x14V[P`\x01\x92\x91PPV[a\"f\x82\x82a\x0B\xF9V[a\x08\xAEW`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x07\xC5V[__Q` a1t_9_Q\x90_Ra\"\xAE\x84\x84a\x0B\xF9V[a#-W_\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\"\xE33\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\x05\xE3V[_\x91PPa\x05\xE3V[_a\x0CX\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a&\x06V[__Q` a1t_9_Q\x90_Ra#c\x84\x84a\x0B\xF9V[\x15a#-W_\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\x05\xE3V[_a\x0CX\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a%,V[a#\xDFa&RV[a\x1E\xD9W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a$IW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a$5W[PPPPP\x90P\x91\x90PV[a$^\x82a&kV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a$\xA2Wa\x06\xBD\x82\x82a \xB1V[a\x08\xAEa&\xCEV[_\x82_\x01\x82\x81T\x81\x10a$\xBFWa$\xBFa-\x85V[\x90_R` _ \x01T\x90P\x92\x91PPV[``\x82a$\xE5Wa$\xE0\x82a&\xEDV[a\x0CXV[\x81Q\x15\x80\x15a$\xFCWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a%%W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x07\xC5V[P\x80a\x0CXV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a#-W_a%N`\x01\x83a-rV[\x85T\x90\x91P_\x90a%a\x90`\x01\x90a-rV[\x90P\x80\x82\x14a%\xC0W_\x86_\x01\x82\x81T\x81\x10a%\x7FWa%\x7Fa-\x85V[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a%\x9FWa%\x9Fa-\x85V[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a%\xD1Wa%\xD1a1\x1FV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x05\xE3V[_\x81\x81R`\x01\x83\x01` R`@\x81 Ta&KWP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x05\xE3V[P_a\x05\xE3V[_a&[a\x1E\xA9V[T`\x01`@\x1B\x90\x04`\xFF\x16\x91\x90PV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a&\xA0W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x07\xC5V[_Q` a1T_9_Q\x90_R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[4\x15a\x1E\xD9W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q\x15a&\xFDW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_` \x82\x84\x03\x12\x15a'&W__\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0CXW__\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1E+W__\xFD[_` \x82\x84\x03\x12\x15a'aW__\xFD[\x815a\x0CX\x81a'=V[_` \x82\x84\x03\x12\x15a'|W__\xFD[P5\x91\x90PV[__`@\x83\x85\x03\x12\x15a'\x94W__\xFD[\x825\x91P` \x83\x015a'\xA6\x81a'=V[\x80\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a'\xC2W__\xFD[\x825a'\xCD\x81a'=V[\x91P` \x83\x015a'\xA6\x81a'=V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a(\x14W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a'\xF6V[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a([Wa([a(\x1FV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a({Wa({a(\x1FV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[__`@\x83\x85\x03\x12\x15a(\x9AW__\xFD[\x825a(\xA5\x81a'=V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xBFW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a(\xCFW__\xFD[\x805a(\xE2a(\xDD\x82a(cV[a(3V[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a(\xF6W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x0CX` \x83\x01\x84a)\x15V[`\xE0\x81R_a)g`\xE0\x83\x01\x8Aa)\x15V[\x82\x81\x03` \x84\x01Ra)y\x81\x8Aa)\x15V[\x90P\x82\x81\x03`@\x84\x01Ra)\x8D\x81\x89a)\x15V[\x90P\x82\x81\x03``\x84\x01Ra)\xA1\x81\x88a)\x15V[\x90P\x82\x81\x03`\x80\x84\x01Ra)\xB5\x81\x87a)\x15V[`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16`\xA0\x84\x01RPP\x90\x15\x15`\xC0\x90\x91\x01R\x95\x94PPPPPV[__`@\x83\x85\x03\x12\x15a)\xEBW__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a(\x14W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a*\x13V[__` \x83\x85\x03\x12\x15a*KW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a*`W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a*pW__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a*\x85W__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a*\x99W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+\0W`?\x19\x87\x86\x03\x01\x84Ra*\xEB\x85\x83Qa)\x15V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a*\xCFV[P\x92\x96\x95PPPPPPV[` \x81R_\x82Q`\xE0` \x84\x01Ra+(a\x01\0\x84\x01\x82a)\x15V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra+E\x82\x82a)\x15V[\x91PP`@\x84\x01Q`\x1F\x19\x84\x83\x03\x01``\x85\x01Ra+c\x82\x82a)\x15V[\x91PP``\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\x80\x85\x01Ra+\x81\x82\x82a)\x15V[\x91PP`\x80\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\xA0\x85\x01Ra+\x9F\x82\x82a)\x15V[\x91PP`\x01\x80`\xA0\x1B\x03`\xA0\x85\x01Q\x16`\xC0\x84\x01R`\xC0\x84\x01Qa+\xC7`\xE0\x85\x01\x82\x15\x15\x90RV[P\x93\x92PPPV[__\x83`\x1F\x84\x01\x12a+\xDFW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a+\xF5W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a,\x0CW__\xFD[\x92P\x92\x90PV[_____``\x86\x88\x03\x12\x15a,'W__\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,CW__\xFD[a,O\x88\x82\x89\x01a+\xCFV[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,mW__\xFD[a,y\x88\x82\x89\x01a+\xCFV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[______``\x87\x89\x03\x12\x15a,\x9FW__\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15a,\xB4W__\xFD[a,\xC0\x89\x82\x8A\x01a+\xCFV[\x90\x97P\x95PP` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,\xDEW__\xFD[a,\xEA\x89\x82\x8A\x01a+\xCFV[\x90\x95P\x93PP`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-\x08W__\xFD[a-\x14\x89\x82\x8A\x01a+\xCFV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a-:W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a-XWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05\xE3Wa\x05\xE3a-^V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a-\xAEW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a-\xC7W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a,\x0CW__\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x82\x84\x827_\x83\x82\x01_\x81Ra.\x07\x81\x85a-\xDBV[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a.!W__\xFD[\x81Qa\x0CX\x81a'=V[_` \x82\x84\x03\x12\x15a.<W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a.QW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a.aW__\xFD[\x80Qa.oa(\xDD\x82a(cV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a.\x83W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[``\x81R_a.\xDA``\x83\x01\x87a)\x15V[\x82\x81\x03` \x84\x01Ra.\xED\x81\x86\x88a.\xA0V[\x90P\x82\x81\x03`@\x84\x01Ra/\x01\x81\x85a)\x15V[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a/\x1CW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x0CXW__\xFD[\x85\x81R``` \x82\x01R_a/D``\x83\x01\x86\x88a.\xA0V[\x82\x81\x03`@\x84\x01Ra/W\x81\x85\x87a.\xA0V[\x98\x97PPPPPPPPV[_a\x0CX\x82\x84a-\xDBV[`@\x81R_a/\x81`@\x83\x01\x86\x88a.\xA0V[\x82\x81\x03` \x84\x01Ra/\x01\x81\x85\x87a.\xA0V[\x80\x82\x01\x80\x82\x11\x15a\x05\xE3Wa\x05\xE3a-^V[` \x81R_a\x0B\xF1` \x83\x01\x84\x86a.\xA0V[`\x1F\x82\x11\x15a\x06\xBDW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a/\xDFWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x18sW_\x81U`\x01\x01a/\xEBV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x17Wa0\x17a(\x1FV[a0+\x81a0%\x84Ta-&V[\x84a/\xBAV[` `\x1F\x82\x11`\x01\x81\x14a0]W_\x83\x15a0FWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x18sV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a0\x8CW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a0lV[P\x84\x82\x10\x15a0\xA9W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80\x81R_a0\xCB`\x80\x83\x01\x89\x8Ba.\xA0V[\x82\x81\x03` \x84\x01Ra0\xDE\x81\x88\x8Aa.\xA0V[\x90P\x82\x81\x03`@\x84\x01Ra0\xF3\x81\x86\x88a.\xA0V[\x91PP\x82``\x83\x01R\x98\x97PPPPPPPPV[_` \x82\x84\x03\x12\x15a1\x18W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xC1\xF6\xFE$b\x1C\xE8\x1E\xC5\x82|\xAF\x02S\xCA\xDBtp\x9B\x06\x160\xE6\xB5^\x827\x17\x05\x93 \x006\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17u\xA2dipfsX\"\x12 \xAD\xCA\xC5eo\xBF\xF7F\x90\x96\xF3\x11|W#\x15\xDC^\xF2cU\xB5g\xE8.\x88)\x9A~\xD2j\xFEdsolcC\0\x08\x1C\x003",
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
    /**Event with signature `DecryptionReceiverCallbackFailed(uint256)` and selector `0x3fed05c5d3a79e0e2f23ffd594267f0e29799ad1b4b6a7756138d5247baa4a7b`.
```solidity
event DecryptionReceiverCallbackFailed(uint256 indexed requestId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DecryptionReceiverCallbackFailed {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for DecryptionReceiverCallbackFailed {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "DecryptionReceiverCallbackFailed(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                63u8, 237u8, 5u8, 197u8, 211u8, 167u8, 158u8, 14u8, 47u8, 35u8, 255u8,
                213u8, 148u8, 38u8, 127u8, 14u8, 41u8, 121u8, 154u8, 209u8, 180u8, 182u8,
                167u8, 117u8, 97u8, 56u8, 213u8, 36u8, 123u8, 170u8, 74u8, 123u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { requestId: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.requestId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DecryptionReceiverCallbackFailed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DecryptionReceiverCallbackFailed>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &DecryptionReceiverCallbackFailed,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DecryptionReceiverCallbackSuccess(uint256,bytes,bytes)` and selector `0x0a1f5b3782d8f604353407c980dbfe6b55499c5b92cb8c39f089c73f348407e6`.
```solidity
event DecryptionReceiverCallbackSuccess(uint256 indexed requestId, bytes decryptionKey, bytes signature);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DecryptionReceiverCallbackSuccess {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decryptionKey: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for DecryptionReceiverCallbackSuccess {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "DecryptionReceiverCallbackSuccess(uint256,bytes,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                10u8, 31u8, 91u8, 55u8, 130u8, 216u8, 246u8, 4u8, 53u8, 52u8, 7u8, 201u8,
                128u8, 219u8, 254u8, 107u8, 85u8, 73u8, 156u8, 91u8, 146u8, 203u8, 140u8,
                57u8, 240u8, 137u8, 199u8, 63u8, 52u8, 132u8, 7u8, 230u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: topics.1,
                    decryptionKey: data.0,
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.decryptionKey,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.requestId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData
        for DecryptionReceiverCallbackSuccess {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DecryptionReceiverCallbackSuccess>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &DecryptionReceiverCallbackSuccess,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DecryptionRequested(uint256,address,string,bytes,bytes,uint256)` and selector `0x133b1acfa49920f88bc8c23faeb2ba43b75b3d6c453290a824f3c85e7733df13`.
```solidity
event DecryptionRequested(uint256 indexed requestId, address indexed callback, string schemeID, bytes condition, bytes ciphertext, uint256 requestedAt);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DecryptionRequested {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub callback: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub schemeID: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub condition: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub ciphertext: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for DecryptionRequested {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DecryptionRequested(uint256,address,string,bytes,bytes,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                19u8, 59u8, 26u8, 207u8, 164u8, 153u8, 32u8, 248u8, 139u8, 200u8, 194u8,
                63u8, 174u8, 178u8, 186u8, 67u8, 183u8, 91u8, 61u8, 108u8, 69u8, 50u8,
                144u8, 168u8, 36u8, 243u8, 200u8, 94u8, 119u8, 51u8, 223u8, 19u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: topics.1,
                    callback: topics.2,
                    schemeID: data.0,
                    condition: data.1,
                    ciphertext: data.2,
                    requestedAt: data.3,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.schemeID,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.condition,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.ciphertext,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestedAt),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.requestId.clone(),
                    self.callback.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.callback,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DecryptionRequested {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DecryptionRequested> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DecryptionRequested) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `SignatureSchemeAddressProviderUpdated(address)` and selector `0x7724bcb43a09ae6582affdee2f0ace931e26f2ffa8b5c334baf0a39e9dc03426`.
```solidity
event SignatureSchemeAddressProviderUpdated(address indexed newSignatureSchemeAddressProvider);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SignatureSchemeAddressProviderUpdated {
        #[allow(missing_docs)]
        pub newSignatureSchemeAddressProvider: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for SignatureSchemeAddressProviderUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SignatureSchemeAddressProviderUpdated(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                119u8, 36u8, 188u8, 180u8, 58u8, 9u8, 174u8, 101u8, 130u8, 175u8, 253u8,
                238u8, 47u8, 10u8, 206u8, 147u8, 30u8, 38u8, 242u8, 255u8, 168u8, 181u8,
                195u8, 52u8, 186u8, 240u8, 163u8, 158u8, 157u8, 192u8, 52u8, 38u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    newSignatureSchemeAddressProvider: topics.1,
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
                    self.newSignatureSchemeAddressProvider.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newSignatureSchemeAddressProvider,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData
        for SignatureSchemeAddressProviderUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SignatureSchemeAddressProviderUpdated>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SignatureSchemeAddressProviderUpdated,
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
    /**Function with signature `fulfillDecryptionRequest(uint256,bytes,bytes)` and selector `0xf7cef800`.
```solidity
function fulfillDecryptionRequest(uint256 requestId, bytes memory decryptionKey, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fulfillDecryptionRequestCall {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decryptionKey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`fulfillDecryptionRequest(uint256,bytes,bytes)`](fulfillDecryptionRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fulfillDecryptionRequestReturn {}
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<fulfillDecryptionRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: fulfillDecryptionRequestCall) -> Self {
                    (value.requestId, value.decryptionKey, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fulfillDecryptionRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        requestId: tuple.0,
                        decryptionKey: tuple.1,
                        signature: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<fulfillDecryptionRequestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: fulfillDecryptionRequestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fulfillDecryptionRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl fulfillDecryptionRequestReturn {
            fn _tokenize(
                &self,
            ) -> <fulfillDecryptionRequestCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for fulfillDecryptionRequestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = fulfillDecryptionRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "fulfillDecryptionRequest(uint256,bytes,bytes)";
            const SELECTOR: [u8; 4] = [247u8, 206u8, 248u8, 0u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                fulfillDecryptionRequestReturn::_tokenize(ret)
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
    /**Function with signature `getAllErroredRequestIds()` and selector `0x6f421ea9`.
```solidity
function getAllErroredRequestIds() external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllErroredRequestIdsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getAllErroredRequestIds()`](getAllErroredRequestIdsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllErroredRequestIdsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<getAllErroredRequestIdsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllErroredRequestIdsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllErroredRequestIdsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<getAllErroredRequestIdsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllErroredRequestIdsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllErroredRequestIdsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllErroredRequestIdsCall {
            type Parameters<'a> = ();
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
            const SIGNATURE: &'static str = "getAllErroredRequestIds()";
            const SELECTOR: [u8; 4] = [111u8, 66u8, 30u8, 169u8];
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
                        let r: getAllErroredRequestIdsReturn = r.into();
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
                        let r: getAllErroredRequestIdsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getAllFulfilledRequestIds()` and selector `0x571d7087`.
```solidity
function getAllFulfilledRequestIds() external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllFulfilledRequestIdsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getAllFulfilledRequestIds()`](getAllFulfilledRequestIdsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllFulfilledRequestIdsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<getAllFulfilledRequestIdsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllFulfilledRequestIdsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllFulfilledRequestIdsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<getAllFulfilledRequestIdsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllFulfilledRequestIdsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllFulfilledRequestIdsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllFulfilledRequestIdsCall {
            type Parameters<'a> = ();
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
            const SIGNATURE: &'static str = "getAllFulfilledRequestIds()";
            const SELECTOR: [u8; 4] = [87u8, 29u8, 112u8, 135u8];
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
                        let r: getAllFulfilledRequestIdsReturn = r.into();
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
                        let r: getAllFulfilledRequestIdsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getAllUnfulfilledRequestIds()` and selector `0x4b96e166`.
```solidity
function getAllUnfulfilledRequestIds() external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllUnfulfilledRequestIdsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getAllUnfulfilledRequestIds()`](getAllUnfulfilledRequestIdsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllUnfulfilledRequestIdsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<getAllUnfulfilledRequestIdsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllUnfulfilledRequestIdsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllUnfulfilledRequestIdsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<getAllUnfulfilledRequestIdsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllUnfulfilledRequestIdsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllUnfulfilledRequestIdsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllUnfulfilledRequestIdsCall {
            type Parameters<'a> = ();
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
            const SIGNATURE: &'static str = "getAllUnfulfilledRequestIds()";
            const SELECTOR: [u8; 4] = [75u8, 150u8, 225u8, 102u8];
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
                        let r: getAllUnfulfilledRequestIdsReturn = r.into();
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
                        let r: getAllUnfulfilledRequestIdsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCountOfUnfulfilledRequestIds()` and selector `0xe63b5d58`.
```solidity
function getCountOfUnfulfilledRequestIds() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCountOfUnfulfilledRequestIdsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCountOfUnfulfilledRequestIds()`](getCountOfUnfulfilledRequestIdsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCountOfUnfulfilledRequestIdsReturn {
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
            impl ::core::convert::From<getCountOfUnfulfilledRequestIdsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCountOfUnfulfilledRequestIdsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCountOfUnfulfilledRequestIdsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<getCountOfUnfulfilledRequestIdsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCountOfUnfulfilledRequestIdsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCountOfUnfulfilledRequestIdsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCountOfUnfulfilledRequestIdsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCountOfUnfulfilledRequestIds()";
            const SELECTOR: [u8; 4] = [230u8, 59u8, 93u8, 88u8];
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
                        let r: getCountOfUnfulfilledRequestIdsReturn = r.into();
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
                        let r: getCountOfUnfulfilledRequestIdsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRequest(uint256)` and selector `0xc58343ef`.
```solidity
function getRequest(uint256 requestId) external view returns (TypesLib.DecryptionRequest memory);
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
        pub _0: <TypesLib::DecryptionRequest as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (TypesLib::DecryptionRequest,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <TypesLib::DecryptionRequest as alloy::sol_types::SolType>::RustType,
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
            type Return = <TypesLib::DecryptionRequest as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (TypesLib::DecryptionRequest,);
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
                    <TypesLib::DecryptionRequest as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `hasErrored(uint256)` and selector `0xb0947289`.
```solidity
function hasErrored(uint256 requestId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasErroredCall {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`hasErrored(uint256)`](hasErroredCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasErroredReturn {
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
            impl ::core::convert::From<hasErroredCall> for UnderlyingRustTuple<'_> {
                fn from(value: hasErroredCall) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasErroredCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<hasErroredReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hasErroredReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasErroredReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasErroredCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasErrored(uint256)";
            const SELECTOR: [u8; 4] = [176u8, 148u8, 114u8, 137u8];
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
                        let r: hasErroredReturn = r.into();
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
                        let r: hasErroredReturn = r.into();
                        r._0
                    })
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
function initialize(address owner, address _signatureSchemeAddressProvider) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _signatureSchemeAddressProvider: alloy::sol_types::private::Address,
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
                    (value.owner, value._signatureSchemeAddressProvider)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        _signatureSchemeAddressProvider: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
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
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._signatureSchemeAddressProvider,
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
    /**Function with signature `lastRequestId()` and selector `0xfc2a88c3`.
```solidity
function lastRequestId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastRequestIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lastRequestId()`](lastRequestIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastRequestIdReturn {
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
            impl ::core::convert::From<lastRequestIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: lastRequestIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastRequestIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<lastRequestIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lastRequestIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastRequestIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastRequestIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastRequestId()";
            const SELECTOR: [u8; 4] = [252u8, 42u8, 136u8, 195u8];
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
                        let r: lastRequestIdReturn = r.into();
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
                        let r: lastRequestIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `multicall(bytes[])` and selector `0xac9650d8`.
```solidity
function multicall(bytes[] memory data) external returns (bytes[] memory results);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multicallCall {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`multicall(bytes[])`](multicallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multicallReturn {
        #[allow(missing_docs)]
        pub results: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
            impl ::core::convert::From<multicallCall> for UnderlyingRustTuple<'_> {
                fn from(value: multicallCall) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multicallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
            impl ::core::convert::From<multicallReturn> for UnderlyingRustTuple<'_> {
                fn from(value: multicallReturn) -> Self {
                    (value.results,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multicallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { results: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for multicallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Bytes,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "multicall(bytes[])";
            const SELECTOR: [u8; 4] = [172u8, 150u8, 80u8, 216u8];
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
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.data),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: multicallReturn = r.into();
                        r.results
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
                        let r: multicallReturn = r.into();
                        r.results
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
    /**Function with signature `registerCiphertext(string,bytes,bytes)` and selector `0xf87f0e61`.
```solidity
function registerCiphertext(string memory schemeID, bytes memory ciphertext, bytes memory condition) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerCiphertextCall {
        #[allow(missing_docs)]
        pub schemeID: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub ciphertext: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub condition: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`registerCiphertext(string,bytes,bytes)`](registerCiphertextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerCiphertextReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
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
            impl ::core::convert::From<registerCiphertextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerCiphertextCall) -> Self {
                    (value.schemeID, value.ciphertext, value.condition)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerCiphertextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        schemeID: tuple.0,
                        ciphertext: tuple.1,
                        condition: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<registerCiphertextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerCiphertextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerCiphertextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerCiphertextCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerCiphertext(string,bytes,bytes)";
            const SELECTOR: [u8; 4] = [248u8, 127u8, 14u8, 97u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.schemeID,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.ciphertext,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.condition,
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
                        let r: registerCiphertextReturn = r.into();
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
                        let r: registerCiphertextReturn = r.into();
                        r._0
                    })
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
    /**Function with signature `requests(uint256)` and selector `0x81d12c58`.
```solidity
function requests(uint256) external view returns (string memory schemeID, bytes memory ciphertext, bytes memory condition, bytes memory decryptionKey, bytes memory signature, address callback, bool isFulfilled);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestsCall(pub alloy::sol_types::private::primitives::aliases::U256);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`requests(uint256)`](requestsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestsReturn {
        #[allow(missing_docs)]
        pub schemeID: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub ciphertext: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub condition: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub decryptionKey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub callback: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub isFulfilled: bool,
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
            impl ::core::convert::From<requestsCall> for UnderlyingRustTuple<'_> {
                fn from(value: requestsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for requestsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Address,
                bool,
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
            impl ::core::convert::From<requestsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: requestsReturn) -> Self {
                    (
                        value.schemeID,
                        value.ciphertext,
                        value.condition,
                        value.decryptionKey,
                        value.signature,
                        value.callback,
                        value.isFulfilled,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for requestsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        schemeID: tuple.0,
                        ciphertext: tuple.1,
                        condition: tuple.2,
                        decryptionKey: tuple.3,
                        signature: tuple.4,
                        callback: tuple.5,
                        isFulfilled: tuple.6,
                    }
                }
            }
        }
        impl requestsReturn {
            fn _tokenize(
                &self,
            ) -> <requestsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.schemeID,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.ciphertext,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.condition,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.decryptionKey,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.callback,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isFulfilled,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = requestsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "requests(uint256)";
            const SELECTOR: [u8; 4] = [129u8, 209u8, 44u8, 88u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                requestsReturn::_tokenize(ret)
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
    /**Function with signature `setSignatureSchemeAddressProvider(address)` and selector `0x16cc9a98`.
```solidity
function setSignatureSchemeAddressProvider(address newSignatureSchemeAddressProvider) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSignatureSchemeAddressProviderCall {
        #[allow(missing_docs)]
        pub newSignatureSchemeAddressProvider: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setSignatureSchemeAddressProvider(address)`](setSignatureSchemeAddressProviderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSignatureSchemeAddressProviderReturn {}
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
            impl ::core::convert::From<setSignatureSchemeAddressProviderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setSignatureSchemeAddressProviderCall) -> Self {
                    (value.newSignatureSchemeAddressProvider,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setSignatureSchemeAddressProviderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newSignatureSchemeAddressProvider: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<setSignatureSchemeAddressProviderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setSignatureSchemeAddressProviderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setSignatureSchemeAddressProviderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setSignatureSchemeAddressProviderReturn {
            fn _tokenize(
                &self,
            ) -> <setSignatureSchemeAddressProviderCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setSignatureSchemeAddressProviderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setSignatureSchemeAddressProviderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setSignatureSchemeAddressProvider(address)";
            const SELECTOR: [u8; 4] = [22u8, 204u8, 154u8, 152u8];
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
                        &self.newSignatureSchemeAddressProvider,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setSignatureSchemeAddressProviderReturn::_tokenize(ret)
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
    /**Function with signature `signatureSchemeAddressProvider()` and selector `0xe6b3ca71`.
```solidity
function signatureSchemeAddressProvider() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct signatureSchemeAddressProviderCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`signatureSchemeAddressProvider()`](signatureSchemeAddressProviderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct signatureSchemeAddressProviderReturn {
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
            impl ::core::convert::From<signatureSchemeAddressProviderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: signatureSchemeAddressProviderCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for signatureSchemeAddressProviderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<signatureSchemeAddressProviderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: signatureSchemeAddressProviderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for signatureSchemeAddressProviderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for signatureSchemeAddressProviderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "signatureSchemeAddressProvider()";
            const SELECTOR: [u8; 4] = [230u8, 179u8, 202u8, 113u8];
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
                        let r: signatureSchemeAddressProviderReturn = r.into();
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
                        let r: signatureSchemeAddressProviderReturn = r.into();
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
    ///Container for all the [`DecryptionSender`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum DecryptionSenderCalls {
        #[allow(missing_docs)]
        ADMIN_ROLE(ADMIN_ROLECall),
        #[allow(missing_docs)]
        DEFAULT_ADMIN_ROLE(DEFAULT_ADMIN_ROLECall),
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        fulfillDecryptionRequest(fulfillDecryptionRequestCall),
        #[allow(missing_docs)]
        getAllErroredRequestIds(getAllErroredRequestIdsCall),
        #[allow(missing_docs)]
        getAllFulfilledRequestIds(getAllFulfilledRequestIdsCall),
        #[allow(missing_docs)]
        getAllUnfulfilledRequestIds(getAllUnfulfilledRequestIdsCall),
        #[allow(missing_docs)]
        getCountOfUnfulfilledRequestIds(getCountOfUnfulfilledRequestIdsCall),
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
        grantRole(grantRoleCall),
        #[allow(missing_docs)]
        hasErrored(hasErroredCall),
        #[allow(missing_docs)]
        hasRole(hasRoleCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isInFlight(isInFlightCall),
        #[allow(missing_docs)]
        lastRequestId(lastRequestIdCall),
        #[allow(missing_docs)]
        multicall(multicallCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        registerCiphertext(registerCiphertextCall),
        #[allow(missing_docs)]
        renounceRole(renounceRoleCall),
        #[allow(missing_docs)]
        requests(requestsCall),
        #[allow(missing_docs)]
        revokeRole(revokeRoleCall),
        #[allow(missing_docs)]
        setSignatureSchemeAddressProvider(setSignatureSchemeAddressProviderCall),
        #[allow(missing_docs)]
        signatureSchemeAddressProvider(signatureSchemeAddressProviderCall),
        #[allow(missing_docs)]
        supportsInterface(supportsInterfaceCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    #[automatically_derived]
    impl DecryptionSenderCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 255u8, 201u8, 167u8],
            [22u8, 204u8, 154u8, 152u8],
            [36u8, 138u8, 156u8, 163u8],
            [47u8, 47u8, 241u8, 93u8],
            [54u8, 86u8, 138u8, 190u8],
            [72u8, 92u8, 201u8, 85u8],
            [75u8, 150u8, 225u8, 102u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [84u8, 253u8, 77u8, 80u8],
            [87u8, 29u8, 112u8, 135u8],
            [111u8, 66u8, 30u8, 169u8],
            [117u8, 178u8, 56u8, 252u8],
            [129u8, 209u8, 44u8, 88u8],
            [144u8, 16u8, 208u8, 124u8],
            [145u8, 209u8, 72u8, 84u8],
            [162u8, 23u8, 253u8, 223u8],
            [163u8, 36u8, 106u8, 211u8],
            [172u8, 150u8, 80u8, 216u8],
            [173u8, 60u8, 177u8, 204u8],
            [176u8, 148u8, 114u8, 137u8],
            [197u8, 131u8, 67u8, 239u8],
            [202u8, 21u8, 200u8, 115u8],
            [205u8, 128u8, 44u8, 145u8],
            [213u8, 71u8, 116u8, 31u8],
            [230u8, 59u8, 93u8, 88u8],
            [230u8, 179u8, 202u8, 113u8],
            [247u8, 206u8, 248u8, 0u8],
            [248u8, 127u8, 14u8, 97u8],
            [252u8, 42u8, 136u8, 195u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DecryptionSenderCalls {
        const NAME: &'static str = "DecryptionSenderCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 30usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ADMIN_ROLE(_) => {
                    <ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DEFAULT_ADMIN_ROLE(_) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::fulfillDecryptionRequest(_) => {
                    <fulfillDecryptionRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllErroredRequestIds(_) => {
                    <getAllErroredRequestIdsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllFulfilledRequestIds(_) => {
                    <getAllFulfilledRequestIdsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllUnfulfilledRequestIds(_) => {
                    <getAllUnfulfilledRequestIdsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCountOfUnfulfilledRequestIds(_) => {
                    <getCountOfUnfulfilledRequestIdsCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::grantRole(_) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasErrored(_) => {
                    <hasErroredCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasRole(_) => <hasRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isInFlight(_) => {
                    <isInFlightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastRequestId(_) => {
                    <lastRequestIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::multicall(_) => {
                    <multicallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerCiphertext(_) => {
                    <registerCiphertextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceRole(_) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::requests(_) => <requestsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::revokeRole(_) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setSignatureSchemeAddressProvider(_) => {
                    <setSignatureSchemeAddressProviderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::signatureSchemeAddressProvider(_) => {
                    <signatureSchemeAddressProviderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::upgradeToAndCall(_) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::version(_) => <versionCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<DecryptionSenderCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn setSignatureSchemeAddressProvider(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <setSignatureSchemeAddressProviderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                DecryptionSenderCalls::setSignatureSchemeAddressProvider,
                            )
                    }
                    setSignatureSchemeAddressProvider
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DecryptionSenderCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getAllUnfulfilledRequestIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getAllUnfulfilledRequestIdsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::getAllUnfulfilledRequestIds)
                    }
                    getAllUnfulfilledRequestIds
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DecryptionSenderCalls::version)
                    }
                    version
                },
                {
                    fn getAllFulfilledRequestIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getAllFulfilledRequestIdsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::getAllFulfilledRequestIds)
                    }
                    getAllFulfilledRequestIds
                },
                {
                    fn getAllErroredRequestIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getAllErroredRequestIdsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::getAllErroredRequestIds)
                    }
                    getAllErroredRequestIds
                },
                {
                    fn ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::ADMIN_ROLE)
                    }
                    ADMIN_ROLE
                },
                {
                    fn requests(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <requestsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DecryptionSenderCalls::requests)
                    }
                    requests
                },
                {
                    fn getRoleMember(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getRoleMemberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::getRoleMember)
                    }
                    getRoleMember
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DecryptionSenderCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn getRoleMembers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getRoleMembersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::getRoleMembers)
                    }
                    getRoleMembers
                },
                {
                    fn multicall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <multicallCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DecryptionSenderCalls::multicall)
                    }
                    multicall
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn hasErrored(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <hasErroredCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::hasErrored)
                    }
                    hasErrored
                },
                {
                    fn getRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::getRequest)
                    }
                    getRequest
                },
                {
                    fn getRoleMemberCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getRoleMemberCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::getRoleMemberCount)
                    }
                    getRoleMemberCount
                },
                {
                    fn isInFlight(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <isInFlightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::isInFlight)
                    }
                    isInFlight
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn getCountOfUnfulfilledRequestIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getCountOfUnfulfilledRequestIdsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::getCountOfUnfulfilledRequestIds)
                    }
                    getCountOfUnfulfilledRequestIds
                },
                {
                    fn signatureSchemeAddressProvider(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <signatureSchemeAddressProviderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::signatureSchemeAddressProvider)
                    }
                    signatureSchemeAddressProvider
                },
                {
                    fn fulfillDecryptionRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <fulfillDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::fulfillDecryptionRequest)
                    }
                    fulfillDecryptionRequest
                },
                {
                    fn registerCiphertext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <registerCiphertextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::registerCiphertext)
                    }
                    registerCiphertext
                },
                {
                    fn lastRequestId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <lastRequestIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderCalls::lastRequestId)
                    }
                    lastRequestId
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
            ) -> alloy_sol_types::Result<DecryptionSenderCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn setSignatureSchemeAddressProvider(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <setSignatureSchemeAddressProviderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                DecryptionSenderCalls::setSignatureSchemeAddressProvider,
                            )
                    }
                    setSignatureSchemeAddressProvider
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getAllUnfulfilledRequestIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getAllUnfulfilledRequestIdsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::getAllUnfulfilledRequestIds)
                    }
                    getAllUnfulfilledRequestIds
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::version)
                    }
                    version
                },
                {
                    fn getAllFulfilledRequestIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getAllFulfilledRequestIdsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::getAllFulfilledRequestIds)
                    }
                    getAllFulfilledRequestIds
                },
                {
                    fn getAllErroredRequestIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getAllErroredRequestIdsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::getAllErroredRequestIds)
                    }
                    getAllErroredRequestIds
                },
                {
                    fn ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::ADMIN_ROLE)
                    }
                    ADMIN_ROLE
                },
                {
                    fn requests(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <requestsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::requests)
                    }
                    requests
                },
                {
                    fn getRoleMember(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getRoleMemberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::getRoleMember)
                    }
                    getRoleMember
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn getRoleMembers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getRoleMembersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::getRoleMembers)
                    }
                    getRoleMembers
                },
                {
                    fn multicall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <multicallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::multicall)
                    }
                    multicall
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn hasErrored(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <hasErroredCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::hasErrored)
                    }
                    hasErrored
                },
                {
                    fn getRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getRequestCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::getRequest)
                    }
                    getRequest
                },
                {
                    fn getRoleMemberCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getRoleMemberCountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::getRoleMemberCount)
                    }
                    getRoleMemberCount
                },
                {
                    fn isInFlight(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <isInFlightCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::isInFlight)
                    }
                    isInFlight
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn getCountOfUnfulfilledRequestIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <getCountOfUnfulfilledRequestIdsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::getCountOfUnfulfilledRequestIds)
                    }
                    getCountOfUnfulfilledRequestIds
                },
                {
                    fn signatureSchemeAddressProvider(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <signatureSchemeAddressProviderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::signatureSchemeAddressProvider)
                    }
                    signatureSchemeAddressProvider
                },
                {
                    fn fulfillDecryptionRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <fulfillDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::fulfillDecryptionRequest)
                    }
                    fulfillDecryptionRequest
                },
                {
                    fn registerCiphertext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <registerCiphertextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::registerCiphertext)
                    }
                    registerCiphertext
                },
                {
                    fn lastRequestId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderCalls> {
                        <lastRequestIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderCalls::lastRequestId)
                    }
                    lastRequestId
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
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::fulfillDecryptionRequest(inner) => {
                    <fulfillDecryptionRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllErroredRequestIds(inner) => {
                    <getAllErroredRequestIdsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllFulfilledRequestIds(inner) => {
                    <getAllFulfilledRequestIdsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllUnfulfilledRequestIds(inner) => {
                    <getAllUnfulfilledRequestIdsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCountOfUnfulfilledRequestIds(inner) => {
                    <getCountOfUnfulfilledRequestIdsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hasErrored(inner) => {
                    <hasErroredCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::lastRequestId(inner) => {
                    <lastRequestIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::multicall(inner) => {
                    <multicallCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerCiphertext(inner) => {
                    <registerCiphertextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::requests(inner) => {
                    <requestsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setSignatureSchemeAddressProvider(inner) => {
                    <setSignatureSchemeAddressProviderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::signatureSchemeAddressProvider(inner) => {
                    <signatureSchemeAddressProviderCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::fulfillDecryptionRequest(inner) => {
                    <fulfillDecryptionRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllErroredRequestIds(inner) => {
                    <getAllErroredRequestIdsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllFulfilledRequestIds(inner) => {
                    <getAllFulfilledRequestIdsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllUnfulfilledRequestIds(inner) => {
                    <getAllUnfulfilledRequestIdsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCountOfUnfulfilledRequestIds(inner) => {
                    <getCountOfUnfulfilledRequestIdsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hasErrored(inner) => {
                    <hasErroredCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::lastRequestId(inner) => {
                    <lastRequestIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::multicall(inner) => {
                    <multicallCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::registerCiphertext(inner) => {
                    <registerCiphertextCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::requests(inner) => {
                    <requestsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setSignatureSchemeAddressProvider(inner) => {
                    <setSignatureSchemeAddressProviderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::signatureSchemeAddressProvider(inner) => {
                    <signatureSchemeAddressProviderCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`DecryptionSender`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum DecryptionSenderErrors {
        #[allow(missing_docs)]
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        #[allow(missing_docs)]
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        #[allow(missing_docs)]
        ERC1967NonPayable(ERC1967NonPayable),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
    }
    #[automatically_derived]
    impl DecryptionSenderErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [62u8, 229u8, 174u8, 181u8],
            [76u8, 156u8, 140u8, 227u8],
            [102u8, 151u8, 178u8, 50u8],
            [153u8, 150u8, 179u8, 21u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 230u8, 188u8, 248u8],
            [224u8, 124u8, 141u8, 186u8],
            [226u8, 81u8, 125u8, 63u8],
            [249u8, 46u8, 232u8, 169u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DecryptionSenderErrors {
        const NAME: &'static str = "DecryptionSenderErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 11usize;
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
                Self::ERC1967InvalidImplementation(_) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967NonPayable(_) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializing(_) => {
                    <NotInitializing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ReentrancyGuardReentrantCall(_) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<DecryptionSenderErrors>] = &[
                {
                    fn ReentrancyGuardReentrantCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderErrors::ReentrancyGuardReentrantCall)
                    }
                    ReentrancyGuardReentrantCall
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(DecryptionSenderErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                DecryptionSenderErrors::AccessControlUnauthorizedAccount,
                            )
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionSenderErrors::InvalidInitialization)
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
            ) -> alloy_sol_types::Result<DecryptionSenderErrors>] = &[
                {
                    fn ReentrancyGuardReentrantCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderErrors::ReentrancyGuardReentrantCall)
                    }
                    ReentrancyGuardReentrantCall
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                DecryptionSenderErrors::AccessControlUnauthorizedAccount,
                            )
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionSenderErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionSenderErrors::InvalidInitialization)
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
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotInitializing(inner) => {
                    <NotInitializing as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ReentrancyGuardReentrantCall(inner) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::ReentrancyGuardReentrantCall(inner) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_encode_raw(
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
    ///Container for all the [`DecryptionSender`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum DecryptionSenderEvents {
        #[allow(missing_docs)]
        DecryptionReceiverCallbackFailed(DecryptionReceiverCallbackFailed),
        #[allow(missing_docs)]
        DecryptionReceiverCallbackSuccess(DecryptionReceiverCallbackSuccess),
        #[allow(missing_docs)]
        DecryptionRequested(DecryptionRequested),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        RoleAdminChanged(RoleAdminChanged),
        #[allow(missing_docs)]
        RoleGranted(RoleGranted),
        #[allow(missing_docs)]
        RoleRevoked(RoleRevoked),
        #[allow(missing_docs)]
        SignatureSchemeAddressProviderUpdated(SignatureSchemeAddressProviderUpdated),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
    }
    #[automatically_derived]
    impl DecryptionSenderEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                10u8, 31u8, 91u8, 55u8, 130u8, 216u8, 246u8, 4u8, 53u8, 52u8, 7u8, 201u8,
                128u8, 219u8, 254u8, 107u8, 85u8, 73u8, 156u8, 91u8, 146u8, 203u8, 140u8,
                57u8, 240u8, 137u8, 199u8, 63u8, 52u8, 132u8, 7u8, 230u8,
            ],
            [
                19u8, 59u8, 26u8, 207u8, 164u8, 153u8, 32u8, 248u8, 139u8, 200u8, 194u8,
                63u8, 174u8, 178u8, 186u8, 67u8, 183u8, 91u8, 61u8, 108u8, 69u8, 50u8,
                144u8, 168u8, 36u8, 243u8, 200u8, 94u8, 119u8, 51u8, 223u8, 19u8,
            ],
            [
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ],
            [
                63u8, 237u8, 5u8, 197u8, 211u8, 167u8, 158u8, 14u8, 47u8, 35u8, 255u8,
                213u8, 148u8, 38u8, 127u8, 14u8, 41u8, 121u8, 154u8, 209u8, 180u8, 182u8,
                167u8, 117u8, 97u8, 56u8, 213u8, 36u8, 123u8, 170u8, 74u8, 123u8,
            ],
            [
                119u8, 36u8, 188u8, 180u8, 58u8, 9u8, 174u8, 101u8, 130u8, 175u8, 253u8,
                238u8, 47u8, 10u8, 206u8, 147u8, 30u8, 38u8, 242u8, 255u8, 168u8, 181u8,
                195u8, 52u8, 186u8, 240u8, 163u8, 158u8, 157u8, 192u8, 52u8, 38u8,
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
                199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8,
                19u8, 244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8,
                33u8, 238u8, 209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
            ],
            [
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for DecryptionSenderEvents {
        const NAME: &'static str = "DecryptionSenderEvents";
        const COUNT: usize = 9usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <DecryptionReceiverCallbackFailed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DecryptionReceiverCallbackFailed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DecryptionReceiverCallbackFailed)
                }
                Some(
                    <DecryptionReceiverCallbackSuccess as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DecryptionReceiverCallbackSuccess as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DecryptionReceiverCallbackSuccess)
                }
                Some(
                    <DecryptionRequested as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DecryptionRequested as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DecryptionRequested)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
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
                    <SignatureSchemeAddressProviderUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SignatureSchemeAddressProviderUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SignatureSchemeAddressProviderUpdated)
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
    impl alloy_sol_types::private::IntoLogData for DecryptionSenderEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DecryptionReceiverCallbackFailed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DecryptionReceiverCallbackSuccess(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DecryptionRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
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
                Self::SignatureSchemeAddressProviderUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DecryptionReceiverCallbackFailed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DecryptionReceiverCallbackSuccess(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DecryptionRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
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
                Self::SignatureSchemeAddressProviderUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`DecryptionSender`](self) contract instance.

See the [wrapper's documentation](`DecryptionSenderInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> DecryptionSenderInstance<P, N> {
        DecryptionSenderInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<DecryptionSenderInstance<P, N>>,
    > {
        DecryptionSenderInstance::<P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        DecryptionSenderInstance::<P, N>::deploy_builder(provider)
    }
    /**A [`DecryptionSender`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`DecryptionSender`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct DecryptionSenderInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for DecryptionSenderInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("DecryptionSenderInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > DecryptionSenderInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`DecryptionSender`](self) contract instance.

See the [wrapper's documentation](`DecryptionSenderInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<DecryptionSenderInstance<P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
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
    impl<P: ::core::clone::Clone, N> DecryptionSenderInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> DecryptionSenderInstance<P, N> {
            DecryptionSenderInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > DecryptionSenderInstance<P, N> {
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
        ///Creates a new call builder for the [`UPGRADE_INTERFACE_VERSION`] function.
        pub fn UPGRADE_INTERFACE_VERSION(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, UPGRADE_INTERFACE_VERSIONCall, N> {
            self.call_builder(&UPGRADE_INTERFACE_VERSIONCall)
        }
        ///Creates a new call builder for the [`fulfillDecryptionRequest`] function.
        pub fn fulfillDecryptionRequest(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
            decryptionKey: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, fulfillDecryptionRequestCall, N> {
            self.call_builder(
                &fulfillDecryptionRequestCall {
                    requestId,
                    decryptionKey,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`getAllErroredRequestIds`] function.
        pub fn getAllErroredRequestIds(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getAllErroredRequestIdsCall, N> {
            self.call_builder(&getAllErroredRequestIdsCall)
        }
        ///Creates a new call builder for the [`getAllFulfilledRequestIds`] function.
        pub fn getAllFulfilledRequestIds(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getAllFulfilledRequestIdsCall, N> {
            self.call_builder(&getAllFulfilledRequestIdsCall)
        }
        ///Creates a new call builder for the [`getAllUnfulfilledRequestIds`] function.
        pub fn getAllUnfulfilledRequestIds(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getAllUnfulfilledRequestIdsCall, N> {
            self.call_builder(&getAllUnfulfilledRequestIdsCall)
        }
        ///Creates a new call builder for the [`getCountOfUnfulfilledRequestIds`] function.
        pub fn getCountOfUnfulfilledRequestIds(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCountOfUnfulfilledRequestIdsCall, N> {
            self.call_builder(&getCountOfUnfulfilledRequestIdsCall)
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
        ///Creates a new call builder for the [`grantRole`] function.
        pub fn grantRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, grantRoleCall, N> {
            self.call_builder(&grantRoleCall { role, account })
        }
        ///Creates a new call builder for the [`hasErrored`] function.
        pub fn hasErrored(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, hasErroredCall, N> {
            self.call_builder(&hasErroredCall { requestId })
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
            owner: alloy::sol_types::private::Address,
            _signatureSchemeAddressProvider: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    owner,
                    _signatureSchemeAddressProvider,
                },
            )
        }
        ///Creates a new call builder for the [`isInFlight`] function.
        pub fn isInFlight(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isInFlightCall, N> {
            self.call_builder(&isInFlightCall { requestId })
        }
        ///Creates a new call builder for the [`lastRequestId`] function.
        pub fn lastRequestId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, lastRequestIdCall, N> {
            self.call_builder(&lastRequestIdCall)
        }
        ///Creates a new call builder for the [`multicall`] function.
        pub fn multicall(
            &self,
            data: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        ) -> alloy_contract::SolCallBuilder<&P, multicallCall, N> {
            self.call_builder(&multicallCall { data })
        }
        ///Creates a new call builder for the [`proxiableUUID`] function.
        pub fn proxiableUUID(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, proxiableUUIDCall, N> {
            self.call_builder(&proxiableUUIDCall)
        }
        ///Creates a new call builder for the [`registerCiphertext`] function.
        pub fn registerCiphertext(
            &self,
            schemeID: alloy::sol_types::private::String,
            ciphertext: alloy::sol_types::private::Bytes,
            condition: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, registerCiphertextCall, N> {
            self.call_builder(
                &registerCiphertextCall {
                    schemeID,
                    ciphertext,
                    condition,
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
        ///Creates a new call builder for the [`requests`] function.
        pub fn requests(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, requestsCall, N> {
            self.call_builder(&requestsCall(_0))
        }
        ///Creates a new call builder for the [`revokeRole`] function.
        pub fn revokeRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, revokeRoleCall, N> {
            self.call_builder(&revokeRoleCall { role, account })
        }
        ///Creates a new call builder for the [`setSignatureSchemeAddressProvider`] function.
        pub fn setSignatureSchemeAddressProvider(
            &self,
            newSignatureSchemeAddressProvider: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            setSignatureSchemeAddressProviderCall,
            N,
        > {
            self.call_builder(
                &setSignatureSchemeAddressProviderCall {
                    newSignatureSchemeAddressProvider,
                },
            )
        }
        ///Creates a new call builder for the [`signatureSchemeAddressProvider`] function.
        pub fn signatureSchemeAddressProvider(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, signatureSchemeAddressProviderCall, N> {
            self.call_builder(&signatureSchemeAddressProviderCall)
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
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > DecryptionSenderInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`DecryptionReceiverCallbackFailed`] event.
        pub fn DecryptionReceiverCallbackFailed_filter(
            &self,
        ) -> alloy_contract::Event<&P, DecryptionReceiverCallbackFailed, N> {
            self.event_filter::<DecryptionReceiverCallbackFailed>()
        }
        ///Creates a new event filter for the [`DecryptionReceiverCallbackSuccess`] event.
        pub fn DecryptionReceiverCallbackSuccess_filter(
            &self,
        ) -> alloy_contract::Event<&P, DecryptionReceiverCallbackSuccess, N> {
            self.event_filter::<DecryptionReceiverCallbackSuccess>()
        }
        ///Creates a new event filter for the [`DecryptionRequested`] event.
        pub fn DecryptionRequested_filter(
            &self,
        ) -> alloy_contract::Event<&P, DecryptionRequested, N> {
            self.event_filter::<DecryptionRequested>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
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
        ///Creates a new event filter for the [`SignatureSchemeAddressProviderUpdated`] event.
        pub fn SignatureSchemeAddressProviderUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, SignatureSchemeAddressProviderUpdated, N> {
            self.event_filter::<SignatureSchemeAddressProviderUpdated>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
