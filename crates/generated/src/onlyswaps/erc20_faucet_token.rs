///Module containing a contract's types and functions.
/**

```solidity
library Checkpoints {
    struct Checkpoint208 { uint48 _key; uint208 _value; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Checkpoints {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Checkpoint208 { uint48 _key; uint208 _value; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Checkpoint208 {
        #[allow(missing_docs)]
        pub _key: alloy::sol_types::private::primitives::aliases::U48,
        #[allow(missing_docs)]
        pub _value: alloy::sol_types::private::primitives::aliases::U208,
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
            alloy::sol_types::sol_data::Uint<48>,
            alloy::sol_types::sol_data::Uint<208>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U48,
            alloy::sol_types::private::primitives::aliases::U208,
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
        impl ::core::convert::From<Checkpoint208> for UnderlyingRustTuple<'_> {
            fn from(value: Checkpoint208) -> Self {
                (value._key, value._value)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Checkpoint208 {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    _key: tuple.0,
                    _value: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Checkpoint208 {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Checkpoint208 {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self._key),
                    <alloy::sol_types::sol_data::Uint<
                        208,
                    > as alloy_sol_types::SolType>::tokenize(&self._value),
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
        impl alloy_sol_types::SolType for Checkpoint208 {
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
        impl alloy_sol_types::SolStruct for Checkpoint208 {
            const NAME: &'static str = "Checkpoint208";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Checkpoint208(uint48 _key,uint208 _value)",
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
                        48,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self._key)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        208,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self._value)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Checkpoint208 {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust._key)
                    + <alloy::sol_types::sol_data::Uint<
                        208,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust._value,
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
                    48,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust._key,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    208,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust._value,
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
    /**Creates a new wrapper around an on-chain [`Checkpoints`](self) contract instance.

See the [wrapper's documentation](`CheckpointsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> CheckpointsInstance<P, N> {
        CheckpointsInstance::<P, N>::new(address, __provider)
    }
    /**A [`Checkpoints`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Checkpoints`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct CheckpointsInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for CheckpointsInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("CheckpointsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > CheckpointsInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`Checkpoints`](self) contract instance.

See the [wrapper's documentation](`CheckpointsInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> CheckpointsInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> CheckpointsInstance<P, N> {
            CheckpointsInstance {
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
    > CheckpointsInstance<P, N> {
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
    > CheckpointsInstance<P, N> {
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
library Checkpoints {
    struct Checkpoint208 {
        uint48 _key;
        uint208 _value;
    }
}

interface ERC20FaucetToken {
    error CheckpointUnorderedInsertion();
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC20ExceededSafeSupply(uint256 increasedSupply, uint256 cap);
    error ERC20InsufficientAllowance(address spender, uint256 allowance, uint256 needed);
    error ERC20InsufficientBalance(address sender, uint256 balance, uint256 needed);
    error ERC20InvalidApprover(address approver);
    error ERC20InvalidReceiver(address receiver);
    error ERC20InvalidSender(address sender);
    error ERC20InvalidSpender(address spender);
    error ERC2612ExpiredSignature(uint256 deadline);
    error ERC2612InvalidSigner(address signer, address owner);
    error ERC5805FutureLookup(uint256 timepoint, uint48 clock);
    error ERC6372InconsistentClock();
    error InvalidAccountNonce(address account, uint256 currentNonce);
    error InvalidShortString();
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
    error SafeCastOverflowedUintDowncast(uint8 bits, uint256 value);
    error StringTooLong(string str);
    error VotesExpiredSignature(uint256 expiry);

    event Approval(address indexed owner, address indexed spender, uint256 value);
    event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);
    event DelegateVotesChanged(address indexed delegate, uint256 previousVotes, uint256 newVotes);
    event EIP712DomainChanged();
    event FaucetAmountSet(uint256 faucetAmount);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Transfer(address indexed from, address indexed to, uint256 value);

    constructor(string name, string symbol, uint8 decimals_, uint256 faucetAmount_, address _owner);

    function CLOCK_MODE() external view returns (string memory);
    function DOMAIN_SEPARATOR() external view returns (bytes32);
    function FAUCET_INTERVAL() external view returns (uint256);
    function allowance(address owner, address spender) external view returns (uint256);
    function approve(address spender, uint256 value) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
    function burn(address account, uint256 amount) external;
    function checkpoints(address account, uint32 pos) external view returns (Checkpoints.Checkpoint208 memory);
    function clock() external view returns (uint48);
    function decimals() external view returns (uint8);
    function delegate(address delegatee) external;
    function delegateBySig(address delegatee, uint256 nonce, uint256 expiry, uint8 v, bytes32 r, bytes32 s) external;
    function delegates(address account) external view returns (address);
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function faucetAmount() external view returns (uint256);
    function getPastTotalSupply(uint256 timepoint) external view returns (uint256);
    function getPastVotes(address account, uint256 timepoint) external view returns (uint256);
    function getVotes(address account) external view returns (uint256);
    function lastMint(address) external view returns (uint256);
    function mint() external;
    function name() external view returns (string memory);
    function nonces(address owner) external view returns (uint256);
    function numCheckpoints(address account) external view returns (uint32);
    function owner() external view returns (address);
    function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external;
    function renounceOwnership() external;
    function setFaucetAmount(uint256 faucetAmount_) external;
    function symbol() external view returns (string memory);
    function totalSupply() external view returns (uint256);
    function transfer(address to, uint256 value) external returns (bool);
    function transferFrom(address from, address to, uint256 value) external returns (bool);
    function transferOwnership(address newOwner) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "symbol",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "decimals_",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "faucetAmount_",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_owner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "CLOCK_MODE",
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
    "name": "DOMAIN_SEPARATOR",
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
    "name": "FAUCET_INTERVAL",
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
    "name": "allowance",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
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
    "name": "approve",
    "inputs": [
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
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
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "balanceOf",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
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
    "name": "burn",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "checkpoints",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "pos",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Checkpoints.Checkpoint208",
        "components": [
          {
            "name": "_key",
            "type": "uint48",
            "internalType": "uint48"
          },
          {
            "name": "_value",
            "type": "uint208",
            "internalType": "uint208"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "clock",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint48",
        "internalType": "uint48"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "decimals",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegate",
    "inputs": [
      {
        "name": "delegatee",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegateBySig",
    "inputs": [
      {
        "name": "delegatee",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "nonce",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "v",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "r",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegates",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
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
    "name": "eip712Domain",
    "inputs": [],
    "outputs": [
      {
        "name": "fields",
        "type": "bytes1",
        "internalType": "bytes1"
      },
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "version",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "verifyingContract",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "extensions",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "faucetAmount",
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
    "name": "getPastTotalSupply",
    "inputs": [
      {
        "name": "timepoint",
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
    "name": "getPastVotes",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "timepoint",
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
    "name": "getVotes",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
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
    "name": "lastMint",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
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
    "name": "mint",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "name",
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
    "name": "nonces",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
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
    "name": "numCheckpoints",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "permit",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "v",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "r",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setFaucetAmount",
    "inputs": [
      {
        "name": "faucetAmount_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "symbol",
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
    "name": "totalSupply",
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
    "name": "transfer",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
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
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferFrom",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
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
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
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
    "type": "event",
    "name": "Approval",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DelegateChanged",
    "inputs": [
      {
        "name": "delegator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "fromDelegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "toDelegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DelegateVotesChanged",
    "inputs": [
      {
        "name": "delegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "previousVotes",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newVotes",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EIP712DomainChanged",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "FaucetAmountSet",
    "inputs": [
      {
        "name": "faucetAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Transfer",
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
      },
      {
        "name": "value",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "CheckpointUnorderedInsertion",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureLength",
    "inputs": [
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureS",
    "inputs": [
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20ExceededSafeSupply",
    "inputs": [
      {
        "name": "increasedSupply",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "cap",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InsufficientAllowance",
    "inputs": [
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "allowance",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "needed",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InsufficientBalance",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "balance",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "needed",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InvalidApprover",
    "inputs": [
      {
        "name": "approver",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InvalidReceiver",
    "inputs": [
      {
        "name": "receiver",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InvalidSender",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InvalidSpender",
    "inputs": [
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC2612ExpiredSignature",
    "inputs": [
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC2612InvalidSigner",
    "inputs": [
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC5805FutureLookup",
    "inputs": [
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "clock",
        "type": "uint48",
        "internalType": "uint48"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC6372InconsistentClock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidAccountNonce",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "currentNonce",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidShortString",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OwnableInvalidOwner",
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
    "name": "OwnableUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "SafeCastOverflowedUintDowncast",
    "inputs": [
      {
        "name": "bits",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "StringTooLong",
    "inputs": [
      {
        "name": "str",
        "type": "string",
        "internalType": "string"
      }
    ]
  },
  {
    "type": "error",
    "name": "VotesExpiredSignature",
    "inputs": [
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
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
pub mod ERC20FaucetToken {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x61016080604052346104ac576126fe803803809161001d82856104b0565b833981019060a0818303126104ac5780516001600160401b0381116104ac57826100489183016104d3565b602082015190926001600160401b0382116104ac576100689183016104d3565b60408201519260ff841684036104ac5760608301516080909301516001600160a01b03811691908290036104ac576040928351916100a685846104b0565b60018352603160f81b6020840190815281519092906001600160401b0381116103bc57600354600181811c911680156104a2575b602082101461039e57601f811161043f575b50806020601f82116001146103db575f916103d0575b508160011b915f199060031b1c1916176003555b8051906001600160401b0382116103bc5760045490600182811c921680156103b2575b602083101461039e5781601f849311610330575b50602090601f83116001146102ca575f926102bf575b50508160011b915f199060031b1c1916176004555b61018181610528565b6101205261018e836106af565b6101405260208151910120918260e05251902080610100524660a05283519060208201927f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f84528583015260608201524660808201523060a082015260a081526101f960c0826104b0565b5190206080523060c05280156102ac57600b54915193816001600160a01b0384167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a36001600160a81b03199092161760a09190911b60ff60a01b1617600b55600c55611f1690816107e882396080518161133a015260a051816113f7015260c05181611304015260e05181611389015261010051816113af01526101205181610879015261014051816108a20152f35b631e4fbdf760e01b5f525f60045260245ffd5b015190505f80610163565b60045f9081528281209350601f198516905b8181106103185750908460019594939210610300575b505050811b01600455610178565b01515f1960f88460031b161c191690555f80806102f2565b929360206001819287860151815501950193016102dc565b60045f529091507f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b601f840160051c81019160208510610394575b90601f859493920160051c01905b818110610386575061014d565b5f8155849350600101610379565b909150819061036b565b634e487b7160e01b5f52602260045260245ffd5b91607f1691610139565b634e487b7160e01b5f52604160045260245ffd5b90508301515f610102565b60035f9081528181209250601f198416905b8181106104275750908360019493921061040f575b5050811b01600355610116565b8501515f1960f88460031b161c191690555f80610402565b9192602060018192868a0151815501940192016103ed565b60035f527fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b601f830160051c81019160208410610498575b601f0160051c01905b81811061048d57506100ec565b5f8155600101610480565b9091508190610477565b90607f16906100da565b5f80fd5b601f909101601f19168101906001600160401b038211908210176103bc57604052565b81601f820112156104ac578051906001600160401b0382116103bc5760405192610507601f8401601f1916602001856104b0565b828452602083830101116104ac57815f9260208093018386015e8301015290565b908151602081105f146105a2575090601f815111610562576020815191015160208210610553571790565b5f198260200360031b1b161790565b604460209160405192839163305a27a960e01b83528160048401528051918291826024860152018484015e5f828201840152601f01601f19168101030190fd5b6001600160401b0381116103bc57600554600181811c911680156106a5575b602082101461039e57601f8111610672575b50602092601f821160011461061157928192935f92610606575b50508160011b915f199060031b1c19161760055560ff90565b015190505f806105ed565b601f1982169360055f52805f20915f5b86811061065a5750836001959610610642575b505050811b0160055560ff90565b01515f1960f88460031b161c191690555f8080610634565b91926020600181928685015181550194019201610621565b60055f52601f60205f20910160051c810190601f830160051c015b81811061069a57506105d3565b5f815560010161068d565b90607f16906105c1565b908151602081105f146106da575090601f815111610562576020815191015160208210610553571790565b6001600160401b0381116103bc57600654600181811c911680156107dd575b602082101461039e57601f81116107aa575b50602092601f821160011461074957928192935f9261073e575b50508160011b915f199060031b1c19161760065560ff90565b015190505f80610725565b601f1982169360065f52805f20915f5b868110610792575083600195961061077a575b505050811b0160065560ff90565b01515f1960f88460031b161c191690555f808061076c565b91926020600181928685015181550194019201610759565b60065f52601f60205f20910160051c810190601f830160051c015b8181106107d2575061070b565b5f81556001016107c5565b90607f16906106f956fe60806040526004361015610011575f80fd5b5f3560e01c806306fdde0314610214578063095ea7b31461020f5780631249c58b1461020a57806318160ddd1461020557806323b872dd14610200578063313ce567146101fb5780633644e515146101f65780633a46b1a8146101f15780634bf5d7e9146101ec578063587cde1e146101e75780635c19a95c146101e25780636df62cef146101dd5780636fcfff45146101d857806370a08231146101d3578063715018a6146101ce5780637ecebe00146101c957806381d2fd9c146101c457806384b0196e146101bf5780638da5cb5b146101ba5780638e539e8c146101b557806391ddadf4146101b057806395d89b41146101ab5780639ab24eb0146101a65780639c281430146101a15780639dc29fac1461019c578063a9059cbb14610197578063adc20b1c14610192578063c3cda5201461018d578063d505accf14610188578063dd62ed3e14610183578063f1127ed81461017e5763f2fde38b14610179575f80fd5b610fe3565b610f31565b610ee6565b610ddb565b610ce3565b610c8b565b610c1e565b610b79565b610b5c565b610b15565b610a70565b610a45565b610986565b61095e565b610861565b610815565b6107dd565b610795565b610772565b610710565b6106f3565b6106d1565b610691565b610601565b610521565b6104ff565b6104dc565b610442565b610425565b61038c565b61035b565b610251565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b90602061024e928181520190610219565b90565b3461032b575f36600319011261032b576040515f60035461027181611056565b808452906001811690811561030757506001146102a9575b6102a58361029981850382611143565b6040519182918261023d565b0390f35b60035f9081527fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b939250905b8082106102ed57509091508101602001610299610289565b9192600181602092548385880101520191019092916102d5565b60ff191660208086019190915291151560051b840190910191506102999050610289565b5f80fd5b600435906001600160a01b038216820361032b57565b602435906001600160a01b038216820361032b57565b3461032b57604036600319011261032b5761038161037761032f565b6024359033611578565b602060405160018152f35b3461032b575f36600319011261032b57335f52600d60205260405f20546201518081018091116104205742106103db57335f908152600d602052604090204290556103d9600c54336111cb565b005b60405162461bcd60e51b815260206004820152601e60248201527f4661756365743a205761697420323468206265747765656e206d696e747300006044820152606490fd5b611166565b3461032b575f36600319011261032b576020600254604051908152f35b3461032b57606036600319011261032b5761045b61032f565b610463610345565b6044359060018060a01b0383165f5260016020526104843360405f20610c76565b54925f1984106104a5575b6104999350611278565b60405160018152602090f35b8284106104c1576104bc8361049995033383611603565b61048f565b8284637dc7a0d960e11b5f523360045260245260445260645ffd5b3461032b575f36600319011261032b57602060ff600b5460a01c16604051908152f35b3461032b575f36600319011261032b576020610519611301565b604051908152f35b3461032b57604036600319011261032b5761053a61032f565b6001600160a01b03165f90815260096020526040902061055b60243561141d565b8154905f8291600584116105ae575b6105759350846117e3565b908161059357505060205f5b6040516001600160d01b039091168152f35b61059e602092611456565b905f52815f20015460301c610581565b91926105b981611686565b81039081116104205761057593855f5265ffffffffffff8260205f2001541665ffffffffffff8516105f146105ef57509161056a565b9291506105fb9061117a565b9061056a565b3461032b575f36600319011261032b5761061a43611639565b65ffffffffffff8061062b43611639565b16911603610682576102a5604051610644604082611143565b601d81527f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c740000006020820152604051918291602083526020830190610219565b6301bfc1c560e61b5f5260045ffd5b3461032b57602036600319011261032b576001600160a01b036106b261032f565b165f526008602052602060018060a01b0360405f205416604051908152f35b3461032b57602036600319011261032b576103d96106ed61032f565b33611464565b3461032b575f36600319011261032b576020604051620151808152f35b3461032b57602036600319011261032b576001600160a01b0361073161032f565b165f52600960205260405f205463ffffffff811161075b5760405163ffffffff9091168152602090f35b6306dfcc6560e41b5f52602060045260245260445ffd5b3461032b57602036600319011261032b57602061051961079061032f565b611199565b3461032b575f36600319011261032b576107ad6114d0565b600b80546001600160a01b031981169091555f906001600160a01b03165f516020611eb65f395f51905f528280a3005b3461032b57602036600319011261032b576001600160a01b036107fe61032f565b165f526007602052602060405f2054604051908152f35b3461032b57602036600319011261032b577f6b485e2740573a57af1fe9acd3f9a33ea4b26b8cd88c6d2daa967036d1b1c38d60206004356108546114d0565b80600c55604051908152a1005b3461032b575f36600319011261032b5761090561089d7f0000000000000000000000000000000000000000000000000000000000000000611925565b6108c67f0000000000000000000000000000000000000000000000000000000000000000611985565b60206040516108d58282611143565b5f81528161091381830194601f198301368737604051978897600f60f81b895260e0858a015260e0890190610219565b908782036040890152610219565b914660608701523060808701525f60a087015285830360c087015251918281520192915f5b82811061094757505050500390f35b835185528695509381019392810192600101610938565b3461032b575f36600319011261032b57600b546040516001600160a01b039091168152602090f35b3461032b57602036600319011261032b576109a260043561141d565b600a54905f8291600584116109f1575b6109be9350600a6117e3565b806109cf57506040515f8152602090f35b6109ec6109dd602092611456565b600a5f52825f20015460301c90565b610581565b91926109fc81611686565b8103908111610420576109be93600a5f5265ffffffffffff8260205f2001541665ffffffffffff8516105f14610a335750916109b2565b929150610a3f9061117a565b906109b2565b3461032b575f36600319011261032b576020610a6043611639565b65ffffffffffff60405191168152f35b3461032b575f36600319011261032b576040515f600454610a9081611056565b80845290600181169081156103075750600114610ab7576102a58361029981850382611143565b60045f9081527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b939250905b808210610afb57509091508101602001610299610289565b919260018160209254838588010152019101909291610ae3565b3461032b57602036600319011261032b576001600160a01b03610b3661032f565b165f526009602052602060018060d01b03610b5360405f206114f7565b16604051908152f35b3461032b575f36600319011261032b576020600c54604051908152f35b3461032b57604036600319011261032b57610b9261032f565b6024356001600160a01b038216918215610c0b57610baf81610c44565b54828110610bf4576103d993835f9203610bc884610c44565b55610bd68460025403600255565b6040518481525f516020611ed65f395f51905f5290602090a3611c09565b63391434e360e21b5f52610c089291611261565b5ffd5b634b637e8f60e11b5f525f60045260245ffd5b3461032b57604036600319011261032b57610381610c3a61032f565b6024359033611278565b6001600160a01b03165f90815260208190526040902090565b6001600160a01b03165f90815260096020526040902090565b9060018060a01b03165f5260205260405f2090565b3461032b57602036600319011261032b576001600160a01b03610cac61032f565b165f52600d602052602060405f2054604051908152f35b6064359060ff8216820361032b57565b6084359060ff8216820361032b57565b3461032b5760c036600319011261032b57610cfc61032f565b60243590604435610d0b610cc3565b6084359060a43592804211610dc95791610d909391610d82610d879460405160208101917fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf835260018060a01b038a1660408301528a6060830152608082015260808152610d7a60a082611143565b519020611521565b6119bc565b90929192611a5d565b610d9981611556565b809303610daa576103d99250611464565b90506301d4b62360e61b5f5260018060a01b031660045260245260445ffd5b632341d78760e11b5f5260045260245ffd5b3461032b5760e036600319011261032b57610df461032f565b610dfc610345565b6044359060643592610e0c610cd3565b60a43560c43590864211610ed357610e9892610e93610e2a86611556565b9860405160208101917f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9835260018060a01b0389169b8c604084015260018060a01b038b1660608401528b608084015260a083015260c082015260c08152610d7a60e082611143565b611547565b936001600160a01b03851603610eb2576103d99350611578565b6325c0072360e11b5f526001600160a01b038085166004521660245260445ffd5b8663313c898160e11b5f5260045260245ffd5b3461032b57604036600319011261032b576020610f28610f0461032f565b610f0c610345565b6001600160a01b039091165f9081526001845260409020610c76565b54604051908152f35b3461032b57604036600319011261032b57610f4a61032f565b6024359063ffffffff8216820361032b576102a591610f9991610f6b6111b3565b50610f746111b3565b506001600160a01b03165f908152600960205260409020610f936111b3565b50611ad9565b5060405190610fa782611123565b5465ffffffffffff811680835260309190911c60209283019081526040805192835290516001600160d01b031692820192909252918291820190565b3461032b57602036600319011261032b57610ffc61032f565b6110046114d0565b6001600160a01b0316801561104357600b80546001600160a01b0319811683179091556001600160a01b03165f516020611eb65f395f51905f525f80a3005b631e4fbdf760e01b5f525f60045260245ffd5b90600182811c92168015611084575b602083101461107057565b634e487b7160e01b5f52602260045260245ffd5b91607f1691611065565b5f929181549161109d83611056565b80835292600181169081156110f257506001146110b957505050565b5f9081526020812093945091925b8383106110d8575060209250010190565b6001816020929493945483858701015201910191906110c7565b915050602093945060ff929192191683830152151560051b010190565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761113e57604052565b61110f565b601f909101601f19168101906001600160401b0382119082101761113e57604052565b634e487b7160e01b5f52601160045260245ffd5b906001820180921161042057565b60405190611197604083611143565b565b6001600160a01b03165f9081526020819052604090205490565b604051906111c082611123565b5f6020838281520152565b91906001600160a01b038316801561124e57600254828101809111610420576002556111f684610c44565b8054830190556040518281525f905f516020611ed65f395f51905f5290602090a36002546001600160d01b0393908481116112375750611197929350611c7b565b8490630e58ae9360e11b5f5260045260245260445ffd5b63ec442f0560e01b5f525f60045260245ffd5b6001600160a01b0316600452602452604452606490565b6001600160a01b038116939291908415610c0b576001600160a01b038216801561124e576112a582610c44565b54958487106112ec57846111979697036112be84610c44565b556112c884610c44565b8054860190556040518581525f516020611ed65f395f51905f5290602090a3611ce1565b63391434e360e21b5f52610c08858885611261565b307f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614806113f4575b1561135c577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f82527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a081526113ee60c082611143565b51902090565b507f00000000000000000000000000000000000000000000000000000000000000004614611333565b65ffffffffffff61142d43611639565b1680821015611440575061024e90611639565b90637669fc0f60e11b5f5260045260245260445ffd5b5f1981019190821161042057565b6001600160a01b038181165f81815260086020526040812080548685166001600160a01b031982168117909255611197969416946114ca9390928691907f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9080a4611199565b91611847565b600b546001600160a01b031633036114e457565b63118cdaa760e01b5f523360045260245ffd5b805490816115055750505f90565b815f19810111610420575f525f199060205f2001015460301c90565b60429061152c611301565b906040519161190160f01b8352600283015260228201522090565b9161024e9391610d87936119bc565b6001600160a01b03165f90815260076020526040902080546001810190915590565b6001600160a01b03169081156115f0576001600160a01b0381169283156115dd57806115d07f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92593855f52600160205260405f20610c76565b55604051908152602090a3565b634a1406b160e11b5f525f60045260245ffd5b63e602df0560e01b5f525f60045260245ffd5b6001600160a01b03169081156115f0576001600160a01b038116156115dd57611636915f52600160205260405f20610c76565b55565b65ffffffffffff81116116515765ffffffffffff1690565b6306dfcc6560e41b5f52603060045260245260445ffd5b8115611672570490565b634e487b7160e01b5f52601260045260245ffd5b600181111561024e57806001600160801b8210156117a6575b61174c61174261173861172e61172461171a6117096117539760046117589a600160401b811015611799575b600160201b81101561178c575b6201000081101561177f575b610100811015611772575b6010811015611765575b101561175d575b60030260011c90565b611713818b611668565b0160011c90565b611713818a611668565b6117138189611668565b6117138188611668565b6117138187611668565b6117138186611668565b8093611668565b821190565b900390565b60011b611700565b60041c9160021b916116f9565b60081c9160041b916116ef565b60101c9160081b916116e4565b60201c9160101b916116d8565b60401c9160201b916116cb565b505061175861175361174c61174261173861172e61172461171a6117096117cd8a60801c90565b9850600160401b975061169f9650505050505050565b91905b8382106117f35750505090565b9091928083169080841860011c820180921161042057845f5265ffffffffffff8260205f2001541665ffffffffffff8416105f146118355750925b91906117e6565b9392506118419061117a565b9161182e565b6001600160a01b0380831693929190811690818514158061191c575b61186f575b5050505050565b816118d1575b505082611884575b8080611868565b5f516020611ef65f395f51905f52916118a86118a26118ae93610c5d565b91611b02565b90611b6b565b604080516001600160d01b039384168152919092166020820152a25f808061187d565b6118fa6118eb5f516020611ef65f395f51905f5292610c5d565b6118f486611b02565b90611b33565b604080516001600160d01b039384168152919092166020820152a25f80611875565b50831515611863565b60ff811461196b5760ff811690601f821161195c5760405191611949604084611143565b6020808452838101919036833783525290565b632cd44ac360e21b5f5260045ffd5b5060405161024e8161197e81600561108e565b0382611143565b60ff81146119a95760ff811690601f821161195c5760405191611949604084611143565b5060405161024e8161197e81600661108e565b91906fa2a8918ca85bafe22016d0b997e4df60600160ff1b038411611a34579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15611a29575f516001600160a01b03811615611a1f57905f905f90565b505f906001905f90565b6040513d5f823e3d90fd5b5050505f9160039190565b60041115611a4957565b634e487b7160e01b5f52602160045260245ffd5b611a6681611a3f565b80611a6f575050565b611a7881611a3f565b60018103611a8f5763f645eedf60e01b5f5260045ffd5b611a9881611a3f565b60028103611ab3575063fce698f760e01b5f5260045260245ffd5b80611abf600392611a3f565b14611ac75750565b6335e2f38360e21b5f5260045260245ffd5b8054821015611aee575f5260205f2001905f90565b634e487b7160e01b5f52603260045260245ffd5b6001600160d01b038111611b1c576001600160d01b031690565b6306dfcc6560e41b5f5260d060045260245260445ffd5b90611b3d43611639565b90611b47836114f7565b6001600160d01b03918216908216039190821161042057611b6792611dd3565b9091565b90611b7543611639565b90611b7f836114f7565b6001600160d01b03918216908216019190821161042057611b6792611dd3565b611ba843611639565b90611bb3600a6114f7565b6001600160d01b039182169082160190811161042057611b6791600a611dd3565b611bdd43611639565b90611be8600a6114f7565b6001600160d01b039182169082160390811161042057611b6791600a611dd3565b61119791906001600160a01b03168015611c63575b611c2f611c2a83611b02565b611bd4565b50505f908152600860205260408120549080525f516020611e965f395f51905f52546001600160a01b039081169116611847565b611c74611c6f83611b02565b611b9f565b5050611c1e565b9061119791611c8c611c6f83611b02565b50506001600160a01b03168015611cce575b60086020525f516020611e965f395f51905f52545f9182526040909120546001600160a01b039081169116611847565b611cda611c2a83611b02565b5050611c9e565b61119792916001600160a01b03909116908115611d44575b6001600160a01b0316908115611d31575b5f90815260086020526040808220549282529020546001600160a01b039081169116611847565b611d3d611c2a84611b02565b5050611d0a565b611d50611c6f84611b02565b5050611cf9565b9065ffffffffffff82549181199060301b169116179055565b8054600160401b81101561113e57611d8d91600182018155611ad9565b611dc0578165ffffffffffff61119793511665ffffffffffff19835416178255602060018060d01b039101511690611d57565b634e487b7160e01b5f525f60045260245ffd5b80549293928015611e6b57611dea611df591611456565b825f5260205f200190565b8054603081901c9365ffffffffffff91821692918116808411611e5c57879303611e285750611e249250611d57565b9190565b915050611e2491611e48611e3a611188565b65ffffffffffff9093168352565b6001600160d01b0386166020830152611d70565b632520601d60e01b5f5260045ffd5b5090611e9091611e7c611e3a611188565b6001600160d01b0385166020830152611d70565b5f919056fe5eff886ea0ce6ca488a3d6e336d6c0f75f46d19b42c06ce5ee98e42c96d256c78be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01`\x80`@R4a\x04\xACWa&\xFE\x808\x03\x80\x91a\0\x1D\x82\x85a\x04\xB0V[\x839\x81\x01\x90`\xA0\x81\x83\x03\x12a\x04\xACW\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x04\xACW\x82a\0H\x91\x83\x01a\x04\xD3V[` \x82\x01Q\x90\x92`\x01`\x01`@\x1B\x03\x82\x11a\x04\xACWa\0h\x91\x83\x01a\x04\xD3V[`@\x82\x01Q\x92`\xFF\x84\x16\x84\x03a\x04\xACW``\x83\x01Q`\x80\x90\x93\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x90\x82\x90\x03a\x04\xACW`@\x92\x83Q\x91a\0\xA6\x85\x84a\x04\xB0V[`\x01\x83R`1`\xF8\x1B` \x84\x01\x90\x81R\x81Q\x90\x92\x90`\x01`\x01`@\x1B\x03\x81\x11a\x03\xBCW`\x03T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x04\xA2W[` \x82\x10\x14a\x03\x9EW`\x1F\x81\x11a\x04?W[P\x80` `\x1F\x82\x11`\x01\x14a\x03\xDBW_\x91a\x03\xD0W[P\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x03U[\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xBCW`\x04T\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x03\xB2W[` \x83\x10\x14a\x03\x9EW\x81`\x1F\x84\x93\x11a\x030W[P` \x90`\x1F\x83\x11`\x01\x14a\x02\xCAW_\x92a\x02\xBFW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x04U[a\x01\x81\x81a\x05(V[a\x01 Ra\x01\x8E\x83a\x06\xAFV[a\x01@R` \x81Q\x91\x01 \x91\x82`\xE0RQ\x90 \x80a\x01\0RF`\xA0R\x83Q\x90` \x82\x01\x92\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x84R\x85\x83\x01R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x01\xF9`\xC0\x82a\x04\xB0V[Q\x90 `\x80R0`\xC0R\x80\x15a\x02\xACW`\x0BT\x91Q\x93\x81`\x01`\x01`\xA0\x1B\x03\x84\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3`\x01`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x17`\xA0\x91\x90\x91\x1B`\xFF`\xA0\x1B\x16\x17`\x0BU`\x0CUa\x1F\x16\x90\x81a\x07\xE8\x829`\x80Q\x81a\x13:\x01R`\xA0Q\x81a\x13\xF7\x01R`\xC0Q\x81a\x13\x04\x01R`\xE0Q\x81a\x13\x89\x01Ra\x01\0Q\x81a\x13\xAF\x01Ra\x01 Q\x81a\x08y\x01Ra\x01@Q\x81a\x08\xA2\x01R\xF3[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[\x01Q\x90P_\x80a\x01cV[`\x04_\x90\x81R\x82\x81 \x93P`\x1F\x19\x85\x16\x90[\x81\x81\x10a\x03\x18WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x03\0W[PPP\x81\x1B\x01`\x04Ua\x01xV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x02\xF2V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x02\xDCV[`\x04_R\x90\x91P\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x03\x94W[\x90`\x1F\x85\x94\x93\x92\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x03\x86WPa\x01MV[_\x81U\x84\x93P`\x01\x01a\x03yV[\x90\x91P\x81\x90a\x03kV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x019V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90P\x83\x01Q_a\x01\x02V[`\x03_\x90\x81R\x81\x81 \x92P`\x1F\x19\x84\x16\x90[\x81\x81\x10a\x04'WP\x90\x83`\x01\x94\x93\x92\x10a\x04\x0FW[PP\x81\x1B\x01`\x03Ua\x01\x16V[\x85\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80a\x04\x02V[\x91\x92` `\x01\x81\x92\x86\x8A\x01Q\x81U\x01\x94\x01\x92\x01a\x03\xEDV[`\x03_R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[`\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x04\x98W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x04\x8DWPa\0\xECV[_\x81U`\x01\x01a\x04\x80V[\x90\x91P\x81\x90a\x04wV[\x90`\x7F\x16\x90a\0\xDAV[_\x80\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x03\xBCW`@RV[\x81`\x1F\x82\x01\x12\x15a\x04\xACW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xBCW`@Q\x92a\x05\x07`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\x04\xB0V[\x82\x84R` \x83\x83\x01\x01\x11a\x04\xACW\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x90V[\x90\x81Q` \x81\x10_\x14a\x05\xA2WP\x90`\x1F\x81Q\x11a\x05bW` \x81Q\x91\x01Q` \x82\x10a\x05SW\x17\x90V[_\x19\x82` \x03`\x03\x1B\x1B\x16\x17\x90V[`D` \x91`@Q\x92\x83\x91c0Z'\xA9`\xE0\x1B\x83R\x81`\x04\x84\x01R\x80Q\x91\x82\x91\x82`$\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xFD[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xBCW`\x05T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x06\xA5W[` \x82\x10\x14a\x03\x9EW`\x1F\x81\x11a\x06rW[P` \x92`\x1F\x82\x11`\x01\x14a\x06\x11W\x92\x81\x92\x93_\x92a\x06\x06W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x05U`\xFF\x90V[\x01Q\x90P_\x80a\x05\xEDV[`\x1F\x19\x82\x16\x93`\x05_R\x80_ \x91_[\x86\x81\x10a\x06ZWP\x83`\x01\x95\x96\x10a\x06BW[PPP\x81\x1B\x01`\x05U`\xFF\x90V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x064V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x06!V[`\x05_R`\x1F` _ \x91\x01`\x05\x1C\x81\x01\x90`\x1F\x83\x01`\x05\x1C\x01[\x81\x81\x10a\x06\x9AWPa\x05\xD3V[_\x81U`\x01\x01a\x06\x8DV[\x90`\x7F\x16\x90a\x05\xC1V[\x90\x81Q` \x81\x10_\x14a\x06\xDAWP\x90`\x1F\x81Q\x11a\x05bW` \x81Q\x91\x01Q` \x82\x10a\x05SW\x17\x90V[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xBCW`\x06T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x07\xDDW[` \x82\x10\x14a\x03\x9EW`\x1F\x81\x11a\x07\xAAW[P` \x92`\x1F\x82\x11`\x01\x14a\x07IW\x92\x81\x92\x93_\x92a\x07>W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x06U`\xFF\x90V[\x01Q\x90P_\x80a\x07%V[`\x1F\x19\x82\x16\x93`\x06_R\x80_ \x91_[\x86\x81\x10a\x07\x92WP\x83`\x01\x95\x96\x10a\x07zW[PPP\x81\x1B\x01`\x06U`\xFF\x90V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x07lV[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x07YV[`\x06_R`\x1F` _ \x91\x01`\x05\x1C\x81\x01\x90`\x1F\x83\x01`\x05\x1C\x01[\x81\x81\x10a\x07\xD2WPa\x07\x0BV[_\x81U`\x01\x01a\x07\xC5V[\x90`\x7F\x16\x90a\x06\xF9V\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x06\xFD\xDE\x03\x14a\x02\x14W\x80c\t^\xA7\xB3\x14a\x02\x0FW\x80c\x12I\xC5\x8B\x14a\x02\nW\x80c\x18\x16\r\xDD\x14a\x02\x05W\x80c#\xB8r\xDD\x14a\x02\0W\x80c1<\xE5g\x14a\x01\xFBW\x80c6D\xE5\x15\x14a\x01\xF6W\x80c:F\xB1\xA8\x14a\x01\xF1W\x80cK\xF5\xD7\xE9\x14a\x01\xECW\x80cX|\xDE\x1E\x14a\x01\xE7W\x80c\\\x19\xA9\\\x14a\x01\xE2W\x80cm\xF6,\xEF\x14a\x01\xDDW\x80co\xCF\xFFE\x14a\x01\xD8W\x80cp\xA0\x821\x14a\x01\xD3W\x80cqP\x18\xA6\x14a\x01\xCEW\x80c~\xCE\xBE\0\x14a\x01\xC9W\x80c\x81\xD2\xFD\x9C\x14a\x01\xC4W\x80c\x84\xB0\x19n\x14a\x01\xBFW\x80c\x8D\xA5\xCB[\x14a\x01\xBAW\x80c\x8ES\x9E\x8C\x14a\x01\xB5W\x80c\x91\xDD\xAD\xF4\x14a\x01\xB0W\x80c\x95\xD8\x9BA\x14a\x01\xABW\x80c\x9A\xB2N\xB0\x14a\x01\xA6W\x80c\x9C(\x140\x14a\x01\xA1W\x80c\x9D\xC2\x9F\xAC\x14a\x01\x9CW\x80c\xA9\x05\x9C\xBB\x14a\x01\x97W\x80c\xAD\xC2\x0B\x1C\x14a\x01\x92W\x80c\xC3\xCD\xA5 \x14a\x01\x8DW\x80c\xD5\x05\xAC\xCF\x14a\x01\x88W\x80c\xDDb\xED>\x14a\x01\x83W\x80c\xF1\x12~\xD8\x14a\x01~Wc\xF2\xFD\xE3\x8B\x14a\x01yW_\x80\xFD[a\x0F\xE3V[a\x0F1V[a\x0E\xE6V[a\r\xDBV[a\x0C\xE3V[a\x0C\x8BV[a\x0C\x1EV[a\x0ByV[a\x0B\\V[a\x0B\x15V[a\npV[a\nEV[a\t\x86V[a\t^V[a\x08aV[a\x08\x15V[a\x07\xDDV[a\x07\x95V[a\x07rV[a\x07\x10V[a\x06\xF3V[a\x06\xD1V[a\x06\x91V[a\x06\x01V[a\x05!V[a\x04\xFFV[a\x04\xDCV[a\x04BV[a\x04%V[a\x03\x8CV[a\x03[V[a\x02QV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x02N\x92\x81\x81R\x01\x90a\x02\x19V[\x90V[4a\x03+W_6`\x03\x19\x01\x12a\x03+W`@Q_`\x03Ta\x02q\x81a\x10VV[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x03\x07WP`\x01\x14a\x02\xA9W[a\x02\xA5\x83a\x02\x99\x81\x85\x03\x82a\x11CV[`@Q\x91\x82\x91\x82a\x02=V[\x03\x90\xF3[`\x03_\x90\x81R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x93\x92P\x90[\x80\x82\x10a\x02\xEDWP\x90\x91P\x81\x01` \x01a\x02\x99a\x02\x89V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x02\xD5V[`\xFF\x19\x16` \x80\x86\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91Pa\x02\x99\x90Pa\x02\x89V[_\x80\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03+WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03+WV[4a\x03+W`@6`\x03\x19\x01\x12a\x03+Wa\x03\x81a\x03wa\x03/V[`$5\x903a\x15xV[` `@Q`\x01\x81R\xF3[4a\x03+W_6`\x03\x19\x01\x12a\x03+W3_R`\r` R`@_ Tb\x01Q\x80\x81\x01\x80\x91\x11a\x04 WB\x10a\x03\xDBW3_\x90\x81R`\r` R`@\x90 B\x90Ua\x03\xD9`\x0CT3a\x11\xCBV[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FFaucet: Wait 24h between mints\0\0`D\x82\x01R`d\x90\xFD[a\x11fV[4a\x03+W_6`\x03\x19\x01\x12a\x03+W` `\x02T`@Q\x90\x81R\xF3[4a\x03+W``6`\x03\x19\x01\x12a\x03+Wa\x04[a\x03/V[a\x04ca\x03EV[`D5\x90`\x01\x80`\xA0\x1B\x03\x83\x16_R`\x01` Ra\x04\x843`@_ a\x0CvV[T\x92_\x19\x84\x10a\x04\xA5W[a\x04\x99\x93Pa\x12xV[`@Q`\x01\x81R` \x90\xF3[\x82\x84\x10a\x04\xC1Wa\x04\xBC\x83a\x04\x99\x95\x033\x83a\x16\x03V[a\x04\x8FV[\x82\x84c}\xC7\xA0\xD9`\xE1\x1B_R3`\x04R`$R`DR`d_\xFD[4a\x03+W_6`\x03\x19\x01\x12a\x03+W` `\xFF`\x0BT`\xA0\x1C\x16`@Q\x90\x81R\xF3[4a\x03+W_6`\x03\x19\x01\x12a\x03+W` a\x05\x19a\x13\x01V[`@Q\x90\x81R\xF3[4a\x03+W`@6`\x03\x19\x01\x12a\x03+Wa\x05:a\x03/V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\t` R`@\x90 a\x05[`$5a\x14\x1DV[\x81T\x90_\x82\x91`\x05\x84\x11a\x05\xAEW[a\x05u\x93P\x84a\x17\xE3V[\x90\x81a\x05\x93WPP` _[`@Q`\x01`\x01`\xD0\x1B\x03\x90\x91\x16\x81R\xF3[a\x05\x9E` \x92a\x14VV[\x90_R\x81_ \x01T`0\x1Ca\x05\x81V[\x91\x92a\x05\xB9\x81a\x16\x86V[\x81\x03\x90\x81\x11a\x04 Wa\x05u\x93\x85_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\x05\xEFWP\x91a\x05jV[\x92\x91Pa\x05\xFB\x90a\x11zV[\x90a\x05jV[4a\x03+W_6`\x03\x19\x01\x12a\x03+Wa\x06\x1ACa\x169V[e\xFF\xFF\xFF\xFF\xFF\xFF\x80a\x06+Ca\x169V[\x16\x91\x16\x03a\x06\x82Wa\x02\xA5`@Qa\x06D`@\x82a\x11CV[`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02\x19V[c\x01\xBF\xC1\xC5`\xE6\x1B_R`\x04_\xFD[4a\x03+W` 6`\x03\x19\x01\x12a\x03+W`\x01`\x01`\xA0\x1B\x03a\x06\xB2a\x03/V[\x16_R`\x08` R` `\x01\x80`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x03+W` 6`\x03\x19\x01\x12a\x03+Wa\x03\xD9a\x06\xEDa\x03/V[3a\x14dV[4a\x03+W_6`\x03\x19\x01\x12a\x03+W` `@Qb\x01Q\x80\x81R\xF3[4a\x03+W` 6`\x03\x19\x01\x12a\x03+W`\x01`\x01`\xA0\x1B\x03a\x071a\x03/V[\x16_R`\t` R`@_ Tc\xFF\xFF\xFF\xFF\x81\x11a\x07[W`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x90\xF3[c\x06\xDF\xCCe`\xE4\x1B_R` `\x04R`$R`D_\xFD[4a\x03+W` 6`\x03\x19\x01\x12a\x03+W` a\x05\x19a\x07\x90a\x03/V[a\x11\x99V[4a\x03+W_6`\x03\x19\x01\x12a\x03+Wa\x07\xADa\x14\xD0V[`\x0B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16_Q` a\x1E\xB6_9_Q\x90_R\x82\x80\xA3\0[4a\x03+W` 6`\x03\x19\x01\x12a\x03+W`\x01`\x01`\xA0\x1B\x03a\x07\xFEa\x03/V[\x16_R`\x07` R` `@_ T`@Q\x90\x81R\xF3[4a\x03+W` 6`\x03\x19\x01\x12a\x03+W\x7FkH^'@W:W\xAF\x1F\xE9\xAC\xD3\xF9\xA3>\xA4\xB2k\x8C\xD8\x8Cm-\xAA\x96p6\xD1\xB1\xC3\x8D` `\x045a\x08Ta\x14\xD0V[\x80`\x0CU`@Q\x90\x81R\xA1\0[4a\x03+W_6`\x03\x19\x01\x12a\x03+Wa\t\x05a\x08\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19%V[a\x08\xC6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\x85V[` `@Qa\x08\xD5\x82\x82a\x11CV[_\x81R\x81a\t\x13\x81\x83\x01\x94`\x1F\x19\x83\x016\x877`@Q\x97\x88\x97`\x0F`\xF8\x1B\x89R`\xE0\x85\x8A\x01R`\xE0\x89\x01\x90a\x02\x19V[\x90\x87\x82\x03`@\x89\x01Ra\x02\x19V[\x91F``\x87\x01R0`\x80\x87\x01R_`\xA0\x87\x01R\x85\x83\x03`\xC0\x87\x01RQ\x91\x82\x81R\x01\x92\x91_[\x82\x81\x10a\tGWPPPP\x03\x90\xF3[\x83Q\x85R\x86\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\t8V[4a\x03+W_6`\x03\x19\x01\x12a\x03+W`\x0BT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03+W` 6`\x03\x19\x01\x12a\x03+Wa\t\xA2`\x045a\x14\x1DV[`\nT\x90_\x82\x91`\x05\x84\x11a\t\xF1W[a\t\xBE\x93P`\na\x17\xE3V[\x80a\t\xCFWP`@Q_\x81R` \x90\xF3[a\t\xECa\t\xDD` \x92a\x14VV[`\n_R\x82_ \x01T`0\x1C\x90V[a\x05\x81V[\x91\x92a\t\xFC\x81a\x16\x86V[\x81\x03\x90\x81\x11a\x04 Wa\t\xBE\x93`\n_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\n3WP\x91a\t\xB2V[\x92\x91Pa\n?\x90a\x11zV[\x90a\t\xB2V[4a\x03+W_6`\x03\x19\x01\x12a\x03+W` a\n`Ca\x169V[e\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x03+W_6`\x03\x19\x01\x12a\x03+W`@Q_`\x04Ta\n\x90\x81a\x10VV[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x03\x07WP`\x01\x14a\n\xB7Wa\x02\xA5\x83a\x02\x99\x81\x85\x03\x82a\x11CV[`\x04_\x90\x81R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x93\x92P\x90[\x80\x82\x10a\n\xFBWP\x90\x91P\x81\x01` \x01a\x02\x99a\x02\x89V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\n\xE3V[4a\x03+W` 6`\x03\x19\x01\x12a\x03+W`\x01`\x01`\xA0\x1B\x03a\x0B6a\x03/V[\x16_R`\t` R` `\x01\x80`\xD0\x1B\x03a\x0BS`@_ a\x14\xF7V[\x16`@Q\x90\x81R\xF3[4a\x03+W_6`\x03\x19\x01\x12a\x03+W` `\x0CT`@Q\x90\x81R\xF3[4a\x03+W`@6`\x03\x19\x01\x12a\x03+Wa\x0B\x92a\x03/V[`$5`\x01`\x01`\xA0\x1B\x03\x82\x16\x91\x82\x15a\x0C\x0BWa\x0B\xAF\x81a\x0CDV[T\x82\x81\x10a\x0B\xF4Wa\x03\xD9\x93\x83_\x92\x03a\x0B\xC8\x84a\x0CDV[Ua\x0B\xD6\x84`\x02T\x03`\x02UV[`@Q\x84\x81R_Q` a\x1E\xD6_9_Q\x90_R\x90` \x90\xA3a\x1C\tV[c9\x144\xE3`\xE2\x1B_Ra\x0C\x08\x92\x91a\x12aV[_\xFD[cKc~\x8F`\xE1\x1B_R_`\x04R`$_\xFD[4a\x03+W`@6`\x03\x19\x01\x12a\x03+Wa\x03\x81a\x0C:a\x03/V[`$5\x903a\x12xV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R` \x81\x90R`@\x90 \x90V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\t` R`@\x90 \x90V[\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[4a\x03+W` 6`\x03\x19\x01\x12a\x03+W`\x01`\x01`\xA0\x1B\x03a\x0C\xACa\x03/V[\x16_R`\r` R` `@_ T`@Q\x90\x81R\xF3[`d5\x90`\xFF\x82\x16\x82\x03a\x03+WV[`\x845\x90`\xFF\x82\x16\x82\x03a\x03+WV[4a\x03+W`\xC06`\x03\x19\x01\x12a\x03+Wa\x0C\xFCa\x03/V[`$5\x90`D5a\r\x0Ba\x0C\xC3V[`\x845\x90`\xA45\x92\x80B\x11a\r\xC9W\x91a\r\x90\x93\x91a\r\x82a\r\x87\x94`@Q` \x81\x01\x91\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x83R`\x01\x80`\xA0\x1B\x03\x8A\x16`@\x83\x01R\x8A``\x83\x01R`\x80\x82\x01R`\x80\x81Ra\rz`\xA0\x82a\x11CV[Q\x90 a\x15!V[a\x19\xBCV[\x90\x92\x91\x92a\x1A]V[a\r\x99\x81a\x15VV[\x80\x93\x03a\r\xAAWa\x03\xD9\x92Pa\x14dV[\x90Pc\x01\xD4\xB6#`\xE6\x1B_R`\x01\x80`\xA0\x1B\x03\x16`\x04R`$R`D_\xFD[c#A\xD7\x87`\xE1\x1B_R`\x04R`$_\xFD[4a\x03+W`\xE06`\x03\x19\x01\x12a\x03+Wa\r\xF4a\x03/V[a\r\xFCa\x03EV[`D5\x90`d5\x92a\x0E\x0Ca\x0C\xD3V[`\xA45`\xC45\x90\x86B\x11a\x0E\xD3Wa\x0E\x98\x92a\x0E\x93a\x0E*\x86a\x15VV[\x98`@Q` \x81\x01\x91\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R`\x01\x80`\xA0\x1B\x03\x89\x16\x9B\x8C`@\x84\x01R`\x01\x80`\xA0\x1B\x03\x8B\x16``\x84\x01R\x8B`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xC0\x81Ra\rz`\xE0\x82a\x11CV[a\x15GV[\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x03a\x0E\xB2Wa\x03\xD9\x93Pa\x15xV[c%\xC0\x07#`\xE1\x1B_R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x04R\x16`$R`D_\xFD[\x86c1<\x89\x81`\xE1\x1B_R`\x04R`$_\xFD[4a\x03+W`@6`\x03\x19\x01\x12a\x03+W` a\x0F(a\x0F\x04a\x03/V[a\x0F\x0Ca\x03EV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16_\x90\x81R`\x01\x84R`@\x90 a\x0CvV[T`@Q\x90\x81R\xF3[4a\x03+W`@6`\x03\x19\x01\x12a\x03+Wa\x0FJa\x03/V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03+Wa\x02\xA5\x91a\x0F\x99\x91a\x0Fka\x11\xB3V[Pa\x0Fta\x11\xB3V[P`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\t` R`@\x90 a\x0F\x93a\x11\xB3V[Pa\x1A\xD9V[P`@Q\x90a\x0F\xA7\x82a\x11#V[Te\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x83R`0\x91\x90\x91\x1C` \x92\x83\x01\x90\x81R`@\x80Q\x92\x83R\x90Q`\x01`\x01`\xD0\x1B\x03\x16\x92\x82\x01\x92\x90\x92R\x91\x82\x91\x82\x01\x90V[4a\x03+W` 6`\x03\x19\x01\x12a\x03+Wa\x0F\xFCa\x03/V[a\x10\x04a\x14\xD0V[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x10CW`\x0B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x16_Q` a\x1E\xB6_9_Q\x90_R_\x80\xA3\0[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x10\x84W[` \x83\x10\x14a\x10pWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x10eV[_\x92\x91\x81T\x91a\x10\x9D\x83a\x10VV[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a\x10\xF2WP`\x01\x14a\x10\xB9WPPPV[_\x90\x81R` \x81 \x93\x94P\x91\x92[\x83\x83\x10a\x10\xD8WP` \x92P\x01\x01\x90V[`\x01\x81` \x92\x94\x93\x94T\x83\x85\x87\x01\x01R\x01\x91\x01\x91\x90a\x10\xC7V[\x91PP` \x93\x94P`\xFF\x92\x91\x92\x19\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x11>W`@RV[a\x11\x0FV[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x11>W`@RV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x04 WV[`@Q\x90a\x11\x97`@\x83a\x11CV[V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90a\x11\xC0\x82a\x11#V[_` \x83\x82\x81R\x01RV[\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x80\x15a\x12NW`\x02T\x82\x81\x01\x80\x91\x11a\x04 W`\x02Ua\x11\xF6\x84a\x0CDV[\x80T\x83\x01\x90U`@Q\x82\x81R_\x90_Q` a\x1E\xD6_9_Q\x90_R\x90` \x90\xA3`\x02T`\x01`\x01`\xD0\x1B\x03\x93\x90\x84\x81\x11a\x127WPa\x11\x97\x92\x93Pa\x1C{V[\x84\x90c\x0EX\xAE\x93`\xE1\x1B_R`\x04R`$R`D_\xFD[c\xECD/\x05`\xE0\x1B_R_`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\x04R`$R`DR`d\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x93\x92\x91\x90\x84\x15a\x0C\x0BW`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x15a\x12NWa\x12\xA5\x82a\x0CDV[T\x95\x84\x87\x10a\x12\xECW\x84a\x11\x97\x96\x97\x03a\x12\xBE\x84a\x0CDV[Ua\x12\xC8\x84a\x0CDV[\x80T\x86\x01\x90U`@Q\x85\x81R_Q` a\x1E\xD6_9_Q\x90_R\x90` \x90\xA3a\x1C\xE1V[c9\x144\xE3`\xE2\x1B_Ra\x0C\x08\x85\x88\x85a\x12aV[0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x13\xF4W[\x15a\x13\\W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q` \x81\x01\x90\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x13\xEE`\xC0\x82a\x11CV[Q\x90 \x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\x133V[e\xFF\xFF\xFF\xFF\xFF\xFFa\x14-Ca\x169V[\x16\x80\x82\x10\x15a\x14@WPa\x02N\x90a\x169V[\x90cvi\xFC\x0F`\xE1\x1B_R`\x04R`$R`D_\xFD[_\x19\x81\x01\x91\x90\x82\x11a\x04 WV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\x08` R`@\x81 \x80T\x86\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92Ua\x11\x97\x96\x94\x16\x94a\x14\xCA\x93\x90\x92\x86\x91\x90\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x90\x80\xA4a\x11\x99V[\x91a\x18GV[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x14\xE4WV[c\x11\x8C\xDA\xA7`\xE0\x1B_R3`\x04R`$_\xFD[\x80T\x90\x81a\x15\x05WPP_\x90V[\x81_\x19\x81\x01\x11a\x04 W_R_\x19\x90` _ \x01\x01T`0\x1C\x90V[`B\x90a\x15,a\x13\x01V[\x90`@Q\x91a\x19\x01`\xF0\x1B\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[\x91a\x02N\x93\x91a\r\x87\x93a\x19\xBCV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x07` R`@\x90 \x80T`\x01\x81\x01\x90\x91U\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x15\xF0W`\x01`\x01`\xA0\x1B\x03\x81\x16\x92\x83\x15a\x15\xDDW\x80a\x15\xD0\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93\x85_R`\x01` R`@_ a\x0CvV[U`@Q\x90\x81R` \x90\xA3V[cJ\x14\x06\xB1`\xE1\x1B_R_`\x04R`$_\xFD[c\xE6\x02\xDF\x05`\xE0\x1B_R_`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x15\xF0W`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x15\xDDWa\x166\x91_R`\x01` R`@_ a\x0CvV[UV[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x16QWe\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[c\x06\xDF\xCCe`\xE4\x1B_R`0`\x04R`$R`D_\xFD[\x81\x15a\x16rW\x04\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[`\x01\x81\x11\x15a\x02NW\x80`\x01`\x01`\x80\x1B\x82\x10\x15a\x17\xA6W[a\x17La\x17Ba\x178a\x17.a\x17$a\x17\x1Aa\x17\ta\x17S\x97`\x04a\x17X\x9A`\x01`@\x1B\x81\x10\x15a\x17\x99W[`\x01` \x1B\x81\x10\x15a\x17\x8CW[b\x01\0\0\x81\x10\x15a\x17\x7FW[a\x01\0\x81\x10\x15a\x17rW[`\x10\x81\x10\x15a\x17eW[\x10\x15a\x17]W[`\x03\x02`\x01\x1C\x90V[a\x17\x13\x81\x8Ba\x16hV[\x01`\x01\x1C\x90V[a\x17\x13\x81\x8Aa\x16hV[a\x17\x13\x81\x89a\x16hV[a\x17\x13\x81\x88a\x16hV[a\x17\x13\x81\x87a\x16hV[a\x17\x13\x81\x86a\x16hV[\x80\x93a\x16hV[\x82\x11\x90V[\x90\x03\x90V[`\x01\x1Ba\x17\0V[`\x04\x1C\x91`\x02\x1B\x91a\x16\xF9V[`\x08\x1C\x91`\x04\x1B\x91a\x16\xEFV[`\x10\x1C\x91`\x08\x1B\x91a\x16\xE4V[` \x1C\x91`\x10\x1B\x91a\x16\xD8V[`@\x1C\x91` \x1B\x91a\x16\xCBV[PPa\x17Xa\x17Sa\x17La\x17Ba\x178a\x17.a\x17$a\x17\x1Aa\x17\ta\x17\xCD\x8A`\x80\x1C\x90V[\x98P`\x01`@\x1B\x97Pa\x16\x9F\x96PPPPPPPV[\x91\x90[\x83\x82\x10a\x17\xF3WPPP\x90V[\x90\x91\x92\x80\x83\x16\x90\x80\x84\x18`\x01\x1C\x82\x01\x80\x92\x11a\x04 W\x84_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x10_\x14a\x185WP\x92[\x91\x90a\x17\xE6V[\x93\x92Pa\x18A\x90a\x11zV[\x91a\x18.V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x93\x92\x91\x90\x81\x16\x90\x81\x85\x14\x15\x80a\x19\x1CW[a\x18oW[PPPPPV[\x81a\x18\xD1W[PP\x82a\x18\x84W[\x80\x80a\x18hV[_Q` a\x1E\xF6_9_Q\x90_R\x91a\x18\xA8a\x18\xA2a\x18\xAE\x93a\x0C]V[\x91a\x1B\x02V[\x90a\x1BkV[`@\x80Q`\x01`\x01`\xD0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R\xA2_\x80\x80a\x18}V[a\x18\xFAa\x18\xEB_Q` a\x1E\xF6_9_Q\x90_R\x92a\x0C]V[a\x18\xF4\x86a\x1B\x02V[\x90a\x1B3V[`@\x80Q`\x01`\x01`\xD0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R\xA2_\x80a\x18uV[P\x83\x15\x15a\x18cV[`\xFF\x81\x14a\x19kW`\xFF\x81\x16\x90`\x1F\x82\x11a\x19\\W`@Q\x91a\x19I`@\x84a\x11CV[` \x80\x84R\x83\x81\x01\x91\x906\x837\x83RR\x90V[c,\xD4J\xC3`\xE2\x1B_R`\x04_\xFD[P`@Qa\x02N\x81a\x19~\x81`\x05a\x10\x8EV[\x03\x82a\x11CV[`\xFF\x81\x14a\x19\xA9W`\xFF\x81\x16\x90`\x1F\x82\x11a\x19\\W`@Q\x91a\x19I`@\x84a\x11CV[P`@Qa\x02N\x81a\x19~\x81`\x06a\x10\x8EV[\x91\x90o\xA2\xA8\x91\x8C\xA8[\xAF\xE2 \x16\xD0\xB9\x97\xE4\xDF``\x01`\xFF\x1B\x03\x84\x11a\x1A4W\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x1A)W_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x1A\x1FW\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[`@Q=_\x82>=\x90\xFD[PPP_\x91`\x03\x91\x90V[`\x04\x11\x15a\x1AIWV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[a\x1Af\x81a\x1A?V[\x80a\x1AoWPPV[a\x1Ax\x81a\x1A?V[`\x01\x81\x03a\x1A\x8FWc\xF6E\xEE\xDF`\xE0\x1B_R`\x04_\xFD[a\x1A\x98\x81a\x1A?V[`\x02\x81\x03a\x1A\xB3WPc\xFC\xE6\x98\xF7`\xE0\x1B_R`\x04R`$_\xFD[\x80a\x1A\xBF`\x03\x92a\x1A?V[\x14a\x1A\xC7WPV[c5\xE2\xF3\x83`\xE2\x1B_R`\x04R`$_\xFD[\x80T\x82\x10\x15a\x1A\xEEW_R` _ \x01\x90_\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\x01`\x01`\xD0\x1B\x03\x81\x11a\x1B\x1CW`\x01`\x01`\xD0\x1B\x03\x16\x90V[c\x06\xDF\xCCe`\xE4\x1B_R`\xD0`\x04R`$R`D_\xFD[\x90a\x1B=Ca\x169V[\x90a\x1BG\x83a\x14\xF7V[`\x01`\x01`\xD0\x1B\x03\x91\x82\x16\x90\x82\x16\x03\x91\x90\x82\x11a\x04 Wa\x1Bg\x92a\x1D\xD3V[\x90\x91V[\x90a\x1BuCa\x169V[\x90a\x1B\x7F\x83a\x14\xF7V[`\x01`\x01`\xD0\x1B\x03\x91\x82\x16\x90\x82\x16\x01\x91\x90\x82\x11a\x04 Wa\x1Bg\x92a\x1D\xD3V[a\x1B\xA8Ca\x169V[\x90a\x1B\xB3`\na\x14\xF7V[`\x01`\x01`\xD0\x1B\x03\x91\x82\x16\x90\x82\x16\x01\x90\x81\x11a\x04 Wa\x1Bg\x91`\na\x1D\xD3V[a\x1B\xDDCa\x169V[\x90a\x1B\xE8`\na\x14\xF7V[`\x01`\x01`\xD0\x1B\x03\x91\x82\x16\x90\x82\x16\x03\x90\x81\x11a\x04 Wa\x1Bg\x91`\na\x1D\xD3V[a\x11\x97\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x1CcW[a\x1C/a\x1C*\x83a\x1B\x02V[a\x1B\xD4V[PP_\x90\x81R`\x08` R`@\x81 T\x90\x80R_Q` a\x1E\x96_9_Q\x90_RT`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16a\x18GV[a\x1Cta\x1Co\x83a\x1B\x02V[a\x1B\x9FV[PPa\x1C\x1EV[\x90a\x11\x97\x91a\x1C\x8Ca\x1Co\x83a\x1B\x02V[PP`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x1C\xCEW[`\x08` R_Q` a\x1E\x96_9_Q\x90_RT_\x91\x82R`@\x90\x91 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16a\x18GV[a\x1C\xDAa\x1C*\x83a\x1B\x02V[PPa\x1C\x9EV[a\x11\x97\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x81\x15a\x1DDW[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x1D1W[_\x90\x81R`\x08` R`@\x80\x82 T\x92\x82R\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16a\x18GV[a\x1D=a\x1C*\x84a\x1B\x02V[PPa\x1D\nV[a\x1DPa\x1Co\x84a\x1B\x02V[PPa\x1C\xF9V[\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90`0\x1B\x16\x91\x16\x17\x90UV[\x80T`\x01`@\x1B\x81\x10\x15a\x11>Wa\x1D\x8D\x91`\x01\x82\x01\x81Ua\x1A\xD9V[a\x1D\xC0W\x81e\xFF\xFF\xFF\xFF\xFF\xFFa\x11\x97\x93Q\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83T\x16\x17\x82U` `\x01\x80`\xD0\x1B\x03\x91\x01Q\x16\x90a\x1DWV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x80T\x92\x93\x92\x80\x15a\x1EkWa\x1D\xEAa\x1D\xF5\x91a\x14VV[\x82_R` _ \x01\x90V[\x80T`0\x81\x90\x1C\x93e\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92\x91\x81\x16\x80\x84\x11a\x1E\\W\x87\x93\x03a\x1E(WPa\x1E$\x92Pa\x1DWV[\x91\x90V[\x91PPa\x1E$\x91a\x1EHa\x1E:a\x11\x88V[e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[`\x01`\x01`\xD0\x1B\x03\x86\x16` \x83\x01Ra\x1DpV[c% `\x1D`\xE0\x1B_R`\x04_\xFD[P\x90a\x1E\x90\x91a\x1E|a\x1E:a\x11\x88V[`\x01`\x01`\xD0\x1B\x03\x85\x16` \x83\x01Ra\x1DpV[_\x91\x90V\xFE^\xFF\x88n\xA0\xCEl\xA4\x88\xA3\xD6\xE36\xD6\xC0\xF7_F\xD1\x9BB\xC0l\xE5\xEE\x98\xE4,\x96\xD2V\xC7\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c806306fdde0314610214578063095ea7b31461020f5780631249c58b1461020a57806318160ddd1461020557806323b872dd14610200578063313ce567146101fb5780633644e515146101f65780633a46b1a8146101f15780634bf5d7e9146101ec578063587cde1e146101e75780635c19a95c146101e25780636df62cef146101dd5780636fcfff45146101d857806370a08231146101d3578063715018a6146101ce5780637ecebe00146101c957806381d2fd9c146101c457806384b0196e146101bf5780638da5cb5b146101ba5780638e539e8c146101b557806391ddadf4146101b057806395d89b41146101ab5780639ab24eb0146101a65780639c281430146101a15780639dc29fac1461019c578063a9059cbb14610197578063adc20b1c14610192578063c3cda5201461018d578063d505accf14610188578063dd62ed3e14610183578063f1127ed81461017e5763f2fde38b14610179575f80fd5b610fe3565b610f31565b610ee6565b610ddb565b610ce3565b610c8b565b610c1e565b610b79565b610b5c565b610b15565b610a70565b610a45565b610986565b61095e565b610861565b610815565b6107dd565b610795565b610772565b610710565b6106f3565b6106d1565b610691565b610601565b610521565b6104ff565b6104dc565b610442565b610425565b61038c565b61035b565b610251565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b90602061024e928181520190610219565b90565b3461032b575f36600319011261032b576040515f60035461027181611056565b808452906001811690811561030757506001146102a9575b6102a58361029981850382611143565b6040519182918261023d565b0390f35b60035f9081527fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b939250905b8082106102ed57509091508101602001610299610289565b9192600181602092548385880101520191019092916102d5565b60ff191660208086019190915291151560051b840190910191506102999050610289565b5f80fd5b600435906001600160a01b038216820361032b57565b602435906001600160a01b038216820361032b57565b3461032b57604036600319011261032b5761038161037761032f565b6024359033611578565b602060405160018152f35b3461032b575f36600319011261032b57335f52600d60205260405f20546201518081018091116104205742106103db57335f908152600d602052604090204290556103d9600c54336111cb565b005b60405162461bcd60e51b815260206004820152601e60248201527f4661756365743a205761697420323468206265747765656e206d696e747300006044820152606490fd5b611166565b3461032b575f36600319011261032b576020600254604051908152f35b3461032b57606036600319011261032b5761045b61032f565b610463610345565b6044359060018060a01b0383165f5260016020526104843360405f20610c76565b54925f1984106104a5575b6104999350611278565b60405160018152602090f35b8284106104c1576104bc8361049995033383611603565b61048f565b8284637dc7a0d960e11b5f523360045260245260445260645ffd5b3461032b575f36600319011261032b57602060ff600b5460a01c16604051908152f35b3461032b575f36600319011261032b576020610519611301565b604051908152f35b3461032b57604036600319011261032b5761053a61032f565b6001600160a01b03165f90815260096020526040902061055b60243561141d565b8154905f8291600584116105ae575b6105759350846117e3565b908161059357505060205f5b6040516001600160d01b039091168152f35b61059e602092611456565b905f52815f20015460301c610581565b91926105b981611686565b81039081116104205761057593855f5265ffffffffffff8260205f2001541665ffffffffffff8516105f146105ef57509161056a565b9291506105fb9061117a565b9061056a565b3461032b575f36600319011261032b5761061a43611639565b65ffffffffffff8061062b43611639565b16911603610682576102a5604051610644604082611143565b601d81527f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c740000006020820152604051918291602083526020830190610219565b6301bfc1c560e61b5f5260045ffd5b3461032b57602036600319011261032b576001600160a01b036106b261032f565b165f526008602052602060018060a01b0360405f205416604051908152f35b3461032b57602036600319011261032b576103d96106ed61032f565b33611464565b3461032b575f36600319011261032b576020604051620151808152f35b3461032b57602036600319011261032b576001600160a01b0361073161032f565b165f52600960205260405f205463ffffffff811161075b5760405163ffffffff9091168152602090f35b6306dfcc6560e41b5f52602060045260245260445ffd5b3461032b57602036600319011261032b57602061051961079061032f565b611199565b3461032b575f36600319011261032b576107ad6114d0565b600b80546001600160a01b031981169091555f906001600160a01b03165f516020611eb65f395f51905f528280a3005b3461032b57602036600319011261032b576001600160a01b036107fe61032f565b165f526007602052602060405f2054604051908152f35b3461032b57602036600319011261032b577f6b485e2740573a57af1fe9acd3f9a33ea4b26b8cd88c6d2daa967036d1b1c38d60206004356108546114d0565b80600c55604051908152a1005b3461032b575f36600319011261032b5761090561089d7f0000000000000000000000000000000000000000000000000000000000000000611925565b6108c67f0000000000000000000000000000000000000000000000000000000000000000611985565b60206040516108d58282611143565b5f81528161091381830194601f198301368737604051978897600f60f81b895260e0858a015260e0890190610219565b908782036040890152610219565b914660608701523060808701525f60a087015285830360c087015251918281520192915f5b82811061094757505050500390f35b835185528695509381019392810192600101610938565b3461032b575f36600319011261032b57600b546040516001600160a01b039091168152602090f35b3461032b57602036600319011261032b576109a260043561141d565b600a54905f8291600584116109f1575b6109be9350600a6117e3565b806109cf57506040515f8152602090f35b6109ec6109dd602092611456565b600a5f52825f20015460301c90565b610581565b91926109fc81611686565b8103908111610420576109be93600a5f5265ffffffffffff8260205f2001541665ffffffffffff8516105f14610a335750916109b2565b929150610a3f9061117a565b906109b2565b3461032b575f36600319011261032b576020610a6043611639565b65ffffffffffff60405191168152f35b3461032b575f36600319011261032b576040515f600454610a9081611056565b80845290600181169081156103075750600114610ab7576102a58361029981850382611143565b60045f9081527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b939250905b808210610afb57509091508101602001610299610289565b919260018160209254838588010152019101909291610ae3565b3461032b57602036600319011261032b576001600160a01b03610b3661032f565b165f526009602052602060018060d01b03610b5360405f206114f7565b16604051908152f35b3461032b575f36600319011261032b576020600c54604051908152f35b3461032b57604036600319011261032b57610b9261032f565b6024356001600160a01b038216918215610c0b57610baf81610c44565b54828110610bf4576103d993835f9203610bc884610c44565b55610bd68460025403600255565b6040518481525f516020611ed65f395f51905f5290602090a3611c09565b63391434e360e21b5f52610c089291611261565b5ffd5b634b637e8f60e11b5f525f60045260245ffd5b3461032b57604036600319011261032b57610381610c3a61032f565b6024359033611278565b6001600160a01b03165f90815260208190526040902090565b6001600160a01b03165f90815260096020526040902090565b9060018060a01b03165f5260205260405f2090565b3461032b57602036600319011261032b576001600160a01b03610cac61032f565b165f52600d602052602060405f2054604051908152f35b6064359060ff8216820361032b57565b6084359060ff8216820361032b57565b3461032b5760c036600319011261032b57610cfc61032f565b60243590604435610d0b610cc3565b6084359060a43592804211610dc95791610d909391610d82610d879460405160208101917fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf835260018060a01b038a1660408301528a6060830152608082015260808152610d7a60a082611143565b519020611521565b6119bc565b90929192611a5d565b610d9981611556565b809303610daa576103d99250611464565b90506301d4b62360e61b5f5260018060a01b031660045260245260445ffd5b632341d78760e11b5f5260045260245ffd5b3461032b5760e036600319011261032b57610df461032f565b610dfc610345565b6044359060643592610e0c610cd3565b60a43560c43590864211610ed357610e9892610e93610e2a86611556565b9860405160208101917f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9835260018060a01b0389169b8c604084015260018060a01b038b1660608401528b608084015260a083015260c082015260c08152610d7a60e082611143565b611547565b936001600160a01b03851603610eb2576103d99350611578565b6325c0072360e11b5f526001600160a01b038085166004521660245260445ffd5b8663313c898160e11b5f5260045260245ffd5b3461032b57604036600319011261032b576020610f28610f0461032f565b610f0c610345565b6001600160a01b039091165f9081526001845260409020610c76565b54604051908152f35b3461032b57604036600319011261032b57610f4a61032f565b6024359063ffffffff8216820361032b576102a591610f9991610f6b6111b3565b50610f746111b3565b506001600160a01b03165f908152600960205260409020610f936111b3565b50611ad9565b5060405190610fa782611123565b5465ffffffffffff811680835260309190911c60209283019081526040805192835290516001600160d01b031692820192909252918291820190565b3461032b57602036600319011261032b57610ffc61032f565b6110046114d0565b6001600160a01b0316801561104357600b80546001600160a01b0319811683179091556001600160a01b03165f516020611eb65f395f51905f525f80a3005b631e4fbdf760e01b5f525f60045260245ffd5b90600182811c92168015611084575b602083101461107057565b634e487b7160e01b5f52602260045260245ffd5b91607f1691611065565b5f929181549161109d83611056565b80835292600181169081156110f257506001146110b957505050565b5f9081526020812093945091925b8383106110d8575060209250010190565b6001816020929493945483858701015201910191906110c7565b915050602093945060ff929192191683830152151560051b010190565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761113e57604052565b61110f565b601f909101601f19168101906001600160401b0382119082101761113e57604052565b634e487b7160e01b5f52601160045260245ffd5b906001820180921161042057565b60405190611197604083611143565b565b6001600160a01b03165f9081526020819052604090205490565b604051906111c082611123565b5f6020838281520152565b91906001600160a01b038316801561124e57600254828101809111610420576002556111f684610c44565b8054830190556040518281525f905f516020611ed65f395f51905f5290602090a36002546001600160d01b0393908481116112375750611197929350611c7b565b8490630e58ae9360e11b5f5260045260245260445ffd5b63ec442f0560e01b5f525f60045260245ffd5b6001600160a01b0316600452602452604452606490565b6001600160a01b038116939291908415610c0b576001600160a01b038216801561124e576112a582610c44565b54958487106112ec57846111979697036112be84610c44565b556112c884610c44565b8054860190556040518581525f516020611ed65f395f51905f5290602090a3611ce1565b63391434e360e21b5f52610c08858885611261565b307f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614806113f4575b1561135c577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f82527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a081526113ee60c082611143565b51902090565b507f00000000000000000000000000000000000000000000000000000000000000004614611333565b65ffffffffffff61142d43611639565b1680821015611440575061024e90611639565b90637669fc0f60e11b5f5260045260245260445ffd5b5f1981019190821161042057565b6001600160a01b038181165f81815260086020526040812080548685166001600160a01b031982168117909255611197969416946114ca9390928691907f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9080a4611199565b91611847565b600b546001600160a01b031633036114e457565b63118cdaa760e01b5f523360045260245ffd5b805490816115055750505f90565b815f19810111610420575f525f199060205f2001015460301c90565b60429061152c611301565b906040519161190160f01b8352600283015260228201522090565b9161024e9391610d87936119bc565b6001600160a01b03165f90815260076020526040902080546001810190915590565b6001600160a01b03169081156115f0576001600160a01b0381169283156115dd57806115d07f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92593855f52600160205260405f20610c76565b55604051908152602090a3565b634a1406b160e11b5f525f60045260245ffd5b63e602df0560e01b5f525f60045260245ffd5b6001600160a01b03169081156115f0576001600160a01b038116156115dd57611636915f52600160205260405f20610c76565b55565b65ffffffffffff81116116515765ffffffffffff1690565b6306dfcc6560e41b5f52603060045260245260445ffd5b8115611672570490565b634e487b7160e01b5f52601260045260245ffd5b600181111561024e57806001600160801b8210156117a6575b61174c61174261173861172e61172461171a6117096117539760046117589a600160401b811015611799575b600160201b81101561178c575b6201000081101561177f575b610100811015611772575b6010811015611765575b101561175d575b60030260011c90565b611713818b611668565b0160011c90565b611713818a611668565b6117138189611668565b6117138188611668565b6117138187611668565b6117138186611668565b8093611668565b821190565b900390565b60011b611700565b60041c9160021b916116f9565b60081c9160041b916116ef565b60101c9160081b916116e4565b60201c9160101b916116d8565b60401c9160201b916116cb565b505061175861175361174c61174261173861172e61172461171a6117096117cd8a60801c90565b9850600160401b975061169f9650505050505050565b91905b8382106117f35750505090565b9091928083169080841860011c820180921161042057845f5265ffffffffffff8260205f2001541665ffffffffffff8416105f146118355750925b91906117e6565b9392506118419061117a565b9161182e565b6001600160a01b0380831693929190811690818514158061191c575b61186f575b5050505050565b816118d1575b505082611884575b8080611868565b5f516020611ef65f395f51905f52916118a86118a26118ae93610c5d565b91611b02565b90611b6b565b604080516001600160d01b039384168152919092166020820152a25f808061187d565b6118fa6118eb5f516020611ef65f395f51905f5292610c5d565b6118f486611b02565b90611b33565b604080516001600160d01b039384168152919092166020820152a25f80611875565b50831515611863565b60ff811461196b5760ff811690601f821161195c5760405191611949604084611143565b6020808452838101919036833783525290565b632cd44ac360e21b5f5260045ffd5b5060405161024e8161197e81600561108e565b0382611143565b60ff81146119a95760ff811690601f821161195c5760405191611949604084611143565b5060405161024e8161197e81600661108e565b91906fa2a8918ca85bafe22016d0b997e4df60600160ff1b038411611a34579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15611a29575f516001600160a01b03811615611a1f57905f905f90565b505f906001905f90565b6040513d5f823e3d90fd5b5050505f9160039190565b60041115611a4957565b634e487b7160e01b5f52602160045260245ffd5b611a6681611a3f565b80611a6f575050565b611a7881611a3f565b60018103611a8f5763f645eedf60e01b5f5260045ffd5b611a9881611a3f565b60028103611ab3575063fce698f760e01b5f5260045260245ffd5b80611abf600392611a3f565b14611ac75750565b6335e2f38360e21b5f5260045260245ffd5b8054821015611aee575f5260205f2001905f90565b634e487b7160e01b5f52603260045260245ffd5b6001600160d01b038111611b1c576001600160d01b031690565b6306dfcc6560e41b5f5260d060045260245260445ffd5b90611b3d43611639565b90611b47836114f7565b6001600160d01b03918216908216039190821161042057611b6792611dd3565b9091565b90611b7543611639565b90611b7f836114f7565b6001600160d01b03918216908216019190821161042057611b6792611dd3565b611ba843611639565b90611bb3600a6114f7565b6001600160d01b039182169082160190811161042057611b6791600a611dd3565b611bdd43611639565b90611be8600a6114f7565b6001600160d01b039182169082160390811161042057611b6791600a611dd3565b61119791906001600160a01b03168015611c63575b611c2f611c2a83611b02565b611bd4565b50505f908152600860205260408120549080525f516020611e965f395f51905f52546001600160a01b039081169116611847565b611c74611c6f83611b02565b611b9f565b5050611c1e565b9061119791611c8c611c6f83611b02565b50506001600160a01b03168015611cce575b60086020525f516020611e965f395f51905f52545f9182526040909120546001600160a01b039081169116611847565b611cda611c2a83611b02565b5050611c9e565b61119792916001600160a01b03909116908115611d44575b6001600160a01b0316908115611d31575b5f90815260086020526040808220549282529020546001600160a01b039081169116611847565b611d3d611c2a84611b02565b5050611d0a565b611d50611c6f84611b02565b5050611cf9565b9065ffffffffffff82549181199060301b169116179055565b8054600160401b81101561113e57611d8d91600182018155611ad9565b611dc0578165ffffffffffff61119793511665ffffffffffff19835416178255602060018060d01b039101511690611d57565b634e487b7160e01b5f525f60045260245ffd5b80549293928015611e6b57611dea611df591611456565b825f5260205f200190565b8054603081901c9365ffffffffffff91821692918116808411611e5c57879303611e285750611e249250611d57565b9190565b915050611e2491611e48611e3a611188565b65ffffffffffff9093168352565b6001600160d01b0386166020830152611d70565b632520601d60e01b5f5260045ffd5b5090611e9091611e7c611e3a611188565b6001600160d01b0385166020830152611d70565b5f919056fe5eff886ea0ce6ca488a3d6e336d6c0f75f46d19b42c06ce5ee98e42c96d256c78be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x06\xFD\xDE\x03\x14a\x02\x14W\x80c\t^\xA7\xB3\x14a\x02\x0FW\x80c\x12I\xC5\x8B\x14a\x02\nW\x80c\x18\x16\r\xDD\x14a\x02\x05W\x80c#\xB8r\xDD\x14a\x02\0W\x80c1<\xE5g\x14a\x01\xFBW\x80c6D\xE5\x15\x14a\x01\xF6W\x80c:F\xB1\xA8\x14a\x01\xF1W\x80cK\xF5\xD7\xE9\x14a\x01\xECW\x80cX|\xDE\x1E\x14a\x01\xE7W\x80c\\\x19\xA9\\\x14a\x01\xE2W\x80cm\xF6,\xEF\x14a\x01\xDDW\x80co\xCF\xFFE\x14a\x01\xD8W\x80cp\xA0\x821\x14a\x01\xD3W\x80cqP\x18\xA6\x14a\x01\xCEW\x80c~\xCE\xBE\0\x14a\x01\xC9W\x80c\x81\xD2\xFD\x9C\x14a\x01\xC4W\x80c\x84\xB0\x19n\x14a\x01\xBFW\x80c\x8D\xA5\xCB[\x14a\x01\xBAW\x80c\x8ES\x9E\x8C\x14a\x01\xB5W\x80c\x91\xDD\xAD\xF4\x14a\x01\xB0W\x80c\x95\xD8\x9BA\x14a\x01\xABW\x80c\x9A\xB2N\xB0\x14a\x01\xA6W\x80c\x9C(\x140\x14a\x01\xA1W\x80c\x9D\xC2\x9F\xAC\x14a\x01\x9CW\x80c\xA9\x05\x9C\xBB\x14a\x01\x97W\x80c\xAD\xC2\x0B\x1C\x14a\x01\x92W\x80c\xC3\xCD\xA5 \x14a\x01\x8DW\x80c\xD5\x05\xAC\xCF\x14a\x01\x88W\x80c\xDDb\xED>\x14a\x01\x83W\x80c\xF1\x12~\xD8\x14a\x01~Wc\xF2\xFD\xE3\x8B\x14a\x01yW_\x80\xFD[a\x0F\xE3V[a\x0F1V[a\x0E\xE6V[a\r\xDBV[a\x0C\xE3V[a\x0C\x8BV[a\x0C\x1EV[a\x0ByV[a\x0B\\V[a\x0B\x15V[a\npV[a\nEV[a\t\x86V[a\t^V[a\x08aV[a\x08\x15V[a\x07\xDDV[a\x07\x95V[a\x07rV[a\x07\x10V[a\x06\xF3V[a\x06\xD1V[a\x06\x91V[a\x06\x01V[a\x05!V[a\x04\xFFV[a\x04\xDCV[a\x04BV[a\x04%V[a\x03\x8CV[a\x03[V[a\x02QV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x02N\x92\x81\x81R\x01\x90a\x02\x19V[\x90V[4a\x03+W_6`\x03\x19\x01\x12a\x03+W`@Q_`\x03Ta\x02q\x81a\x10VV[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x03\x07WP`\x01\x14a\x02\xA9W[a\x02\xA5\x83a\x02\x99\x81\x85\x03\x82a\x11CV[`@Q\x91\x82\x91\x82a\x02=V[\x03\x90\xF3[`\x03_\x90\x81R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x93\x92P\x90[\x80\x82\x10a\x02\xEDWP\x90\x91P\x81\x01` \x01a\x02\x99a\x02\x89V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x02\xD5V[`\xFF\x19\x16` \x80\x86\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91Pa\x02\x99\x90Pa\x02\x89V[_\x80\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03+WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03+WV[4a\x03+W`@6`\x03\x19\x01\x12a\x03+Wa\x03\x81a\x03wa\x03/V[`$5\x903a\x15xV[` `@Q`\x01\x81R\xF3[4a\x03+W_6`\x03\x19\x01\x12a\x03+W3_R`\r` R`@_ Tb\x01Q\x80\x81\x01\x80\x91\x11a\x04 WB\x10a\x03\xDBW3_\x90\x81R`\r` R`@\x90 B\x90Ua\x03\xD9`\x0CT3a\x11\xCBV[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FFaucet: Wait 24h between mints\0\0`D\x82\x01R`d\x90\xFD[a\x11fV[4a\x03+W_6`\x03\x19\x01\x12a\x03+W` `\x02T`@Q\x90\x81R\xF3[4a\x03+W``6`\x03\x19\x01\x12a\x03+Wa\x04[a\x03/V[a\x04ca\x03EV[`D5\x90`\x01\x80`\xA0\x1B\x03\x83\x16_R`\x01` Ra\x04\x843`@_ a\x0CvV[T\x92_\x19\x84\x10a\x04\xA5W[a\x04\x99\x93Pa\x12xV[`@Q`\x01\x81R` \x90\xF3[\x82\x84\x10a\x04\xC1Wa\x04\xBC\x83a\x04\x99\x95\x033\x83a\x16\x03V[a\x04\x8FV[\x82\x84c}\xC7\xA0\xD9`\xE1\x1B_R3`\x04R`$R`DR`d_\xFD[4a\x03+W_6`\x03\x19\x01\x12a\x03+W` `\xFF`\x0BT`\xA0\x1C\x16`@Q\x90\x81R\xF3[4a\x03+W_6`\x03\x19\x01\x12a\x03+W` a\x05\x19a\x13\x01V[`@Q\x90\x81R\xF3[4a\x03+W`@6`\x03\x19\x01\x12a\x03+Wa\x05:a\x03/V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\t` R`@\x90 a\x05[`$5a\x14\x1DV[\x81T\x90_\x82\x91`\x05\x84\x11a\x05\xAEW[a\x05u\x93P\x84a\x17\xE3V[\x90\x81a\x05\x93WPP` _[`@Q`\x01`\x01`\xD0\x1B\x03\x90\x91\x16\x81R\xF3[a\x05\x9E` \x92a\x14VV[\x90_R\x81_ \x01T`0\x1Ca\x05\x81V[\x91\x92a\x05\xB9\x81a\x16\x86V[\x81\x03\x90\x81\x11a\x04 Wa\x05u\x93\x85_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\x05\xEFWP\x91a\x05jV[\x92\x91Pa\x05\xFB\x90a\x11zV[\x90a\x05jV[4a\x03+W_6`\x03\x19\x01\x12a\x03+Wa\x06\x1ACa\x169V[e\xFF\xFF\xFF\xFF\xFF\xFF\x80a\x06+Ca\x169V[\x16\x91\x16\x03a\x06\x82Wa\x02\xA5`@Qa\x06D`@\x82a\x11CV[`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02\x19V[c\x01\xBF\xC1\xC5`\xE6\x1B_R`\x04_\xFD[4a\x03+W` 6`\x03\x19\x01\x12a\x03+W`\x01`\x01`\xA0\x1B\x03a\x06\xB2a\x03/V[\x16_R`\x08` R` `\x01\x80`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x03+W` 6`\x03\x19\x01\x12a\x03+Wa\x03\xD9a\x06\xEDa\x03/V[3a\x14dV[4a\x03+W_6`\x03\x19\x01\x12a\x03+W` `@Qb\x01Q\x80\x81R\xF3[4a\x03+W` 6`\x03\x19\x01\x12a\x03+W`\x01`\x01`\xA0\x1B\x03a\x071a\x03/V[\x16_R`\t` R`@_ Tc\xFF\xFF\xFF\xFF\x81\x11a\x07[W`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x90\xF3[c\x06\xDF\xCCe`\xE4\x1B_R` `\x04R`$R`D_\xFD[4a\x03+W` 6`\x03\x19\x01\x12a\x03+W` a\x05\x19a\x07\x90a\x03/V[a\x11\x99V[4a\x03+W_6`\x03\x19\x01\x12a\x03+Wa\x07\xADa\x14\xD0V[`\x0B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16_Q` a\x1E\xB6_9_Q\x90_R\x82\x80\xA3\0[4a\x03+W` 6`\x03\x19\x01\x12a\x03+W`\x01`\x01`\xA0\x1B\x03a\x07\xFEa\x03/V[\x16_R`\x07` R` `@_ T`@Q\x90\x81R\xF3[4a\x03+W` 6`\x03\x19\x01\x12a\x03+W\x7FkH^'@W:W\xAF\x1F\xE9\xAC\xD3\xF9\xA3>\xA4\xB2k\x8C\xD8\x8Cm-\xAA\x96p6\xD1\xB1\xC3\x8D` `\x045a\x08Ta\x14\xD0V[\x80`\x0CU`@Q\x90\x81R\xA1\0[4a\x03+W_6`\x03\x19\x01\x12a\x03+Wa\t\x05a\x08\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19%V[a\x08\xC6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\x85V[` `@Qa\x08\xD5\x82\x82a\x11CV[_\x81R\x81a\t\x13\x81\x83\x01\x94`\x1F\x19\x83\x016\x877`@Q\x97\x88\x97`\x0F`\xF8\x1B\x89R`\xE0\x85\x8A\x01R`\xE0\x89\x01\x90a\x02\x19V[\x90\x87\x82\x03`@\x89\x01Ra\x02\x19V[\x91F``\x87\x01R0`\x80\x87\x01R_`\xA0\x87\x01R\x85\x83\x03`\xC0\x87\x01RQ\x91\x82\x81R\x01\x92\x91_[\x82\x81\x10a\tGWPPPP\x03\x90\xF3[\x83Q\x85R\x86\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\t8V[4a\x03+W_6`\x03\x19\x01\x12a\x03+W`\x0BT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03+W` 6`\x03\x19\x01\x12a\x03+Wa\t\xA2`\x045a\x14\x1DV[`\nT\x90_\x82\x91`\x05\x84\x11a\t\xF1W[a\t\xBE\x93P`\na\x17\xE3V[\x80a\t\xCFWP`@Q_\x81R` \x90\xF3[a\t\xECa\t\xDD` \x92a\x14VV[`\n_R\x82_ \x01T`0\x1C\x90V[a\x05\x81V[\x91\x92a\t\xFC\x81a\x16\x86V[\x81\x03\x90\x81\x11a\x04 Wa\t\xBE\x93`\n_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\n3WP\x91a\t\xB2V[\x92\x91Pa\n?\x90a\x11zV[\x90a\t\xB2V[4a\x03+W_6`\x03\x19\x01\x12a\x03+W` a\n`Ca\x169V[e\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x03+W_6`\x03\x19\x01\x12a\x03+W`@Q_`\x04Ta\n\x90\x81a\x10VV[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x03\x07WP`\x01\x14a\n\xB7Wa\x02\xA5\x83a\x02\x99\x81\x85\x03\x82a\x11CV[`\x04_\x90\x81R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x93\x92P\x90[\x80\x82\x10a\n\xFBWP\x90\x91P\x81\x01` \x01a\x02\x99a\x02\x89V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\n\xE3V[4a\x03+W` 6`\x03\x19\x01\x12a\x03+W`\x01`\x01`\xA0\x1B\x03a\x0B6a\x03/V[\x16_R`\t` R` `\x01\x80`\xD0\x1B\x03a\x0BS`@_ a\x14\xF7V[\x16`@Q\x90\x81R\xF3[4a\x03+W_6`\x03\x19\x01\x12a\x03+W` `\x0CT`@Q\x90\x81R\xF3[4a\x03+W`@6`\x03\x19\x01\x12a\x03+Wa\x0B\x92a\x03/V[`$5`\x01`\x01`\xA0\x1B\x03\x82\x16\x91\x82\x15a\x0C\x0BWa\x0B\xAF\x81a\x0CDV[T\x82\x81\x10a\x0B\xF4Wa\x03\xD9\x93\x83_\x92\x03a\x0B\xC8\x84a\x0CDV[Ua\x0B\xD6\x84`\x02T\x03`\x02UV[`@Q\x84\x81R_Q` a\x1E\xD6_9_Q\x90_R\x90` \x90\xA3a\x1C\tV[c9\x144\xE3`\xE2\x1B_Ra\x0C\x08\x92\x91a\x12aV[_\xFD[cKc~\x8F`\xE1\x1B_R_`\x04R`$_\xFD[4a\x03+W`@6`\x03\x19\x01\x12a\x03+Wa\x03\x81a\x0C:a\x03/V[`$5\x903a\x12xV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R` \x81\x90R`@\x90 \x90V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\t` R`@\x90 \x90V[\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[4a\x03+W` 6`\x03\x19\x01\x12a\x03+W`\x01`\x01`\xA0\x1B\x03a\x0C\xACa\x03/V[\x16_R`\r` R` `@_ T`@Q\x90\x81R\xF3[`d5\x90`\xFF\x82\x16\x82\x03a\x03+WV[`\x845\x90`\xFF\x82\x16\x82\x03a\x03+WV[4a\x03+W`\xC06`\x03\x19\x01\x12a\x03+Wa\x0C\xFCa\x03/V[`$5\x90`D5a\r\x0Ba\x0C\xC3V[`\x845\x90`\xA45\x92\x80B\x11a\r\xC9W\x91a\r\x90\x93\x91a\r\x82a\r\x87\x94`@Q` \x81\x01\x91\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x83R`\x01\x80`\xA0\x1B\x03\x8A\x16`@\x83\x01R\x8A``\x83\x01R`\x80\x82\x01R`\x80\x81Ra\rz`\xA0\x82a\x11CV[Q\x90 a\x15!V[a\x19\xBCV[\x90\x92\x91\x92a\x1A]V[a\r\x99\x81a\x15VV[\x80\x93\x03a\r\xAAWa\x03\xD9\x92Pa\x14dV[\x90Pc\x01\xD4\xB6#`\xE6\x1B_R`\x01\x80`\xA0\x1B\x03\x16`\x04R`$R`D_\xFD[c#A\xD7\x87`\xE1\x1B_R`\x04R`$_\xFD[4a\x03+W`\xE06`\x03\x19\x01\x12a\x03+Wa\r\xF4a\x03/V[a\r\xFCa\x03EV[`D5\x90`d5\x92a\x0E\x0Ca\x0C\xD3V[`\xA45`\xC45\x90\x86B\x11a\x0E\xD3Wa\x0E\x98\x92a\x0E\x93a\x0E*\x86a\x15VV[\x98`@Q` \x81\x01\x91\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R`\x01\x80`\xA0\x1B\x03\x89\x16\x9B\x8C`@\x84\x01R`\x01\x80`\xA0\x1B\x03\x8B\x16``\x84\x01R\x8B`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xC0\x81Ra\rz`\xE0\x82a\x11CV[a\x15GV[\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x03a\x0E\xB2Wa\x03\xD9\x93Pa\x15xV[c%\xC0\x07#`\xE1\x1B_R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x04R\x16`$R`D_\xFD[\x86c1<\x89\x81`\xE1\x1B_R`\x04R`$_\xFD[4a\x03+W`@6`\x03\x19\x01\x12a\x03+W` a\x0F(a\x0F\x04a\x03/V[a\x0F\x0Ca\x03EV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16_\x90\x81R`\x01\x84R`@\x90 a\x0CvV[T`@Q\x90\x81R\xF3[4a\x03+W`@6`\x03\x19\x01\x12a\x03+Wa\x0FJa\x03/V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03+Wa\x02\xA5\x91a\x0F\x99\x91a\x0Fka\x11\xB3V[Pa\x0Fta\x11\xB3V[P`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\t` R`@\x90 a\x0F\x93a\x11\xB3V[Pa\x1A\xD9V[P`@Q\x90a\x0F\xA7\x82a\x11#V[Te\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x83R`0\x91\x90\x91\x1C` \x92\x83\x01\x90\x81R`@\x80Q\x92\x83R\x90Q`\x01`\x01`\xD0\x1B\x03\x16\x92\x82\x01\x92\x90\x92R\x91\x82\x91\x82\x01\x90V[4a\x03+W` 6`\x03\x19\x01\x12a\x03+Wa\x0F\xFCa\x03/V[a\x10\x04a\x14\xD0V[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x10CW`\x0B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x16_Q` a\x1E\xB6_9_Q\x90_R_\x80\xA3\0[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x10\x84W[` \x83\x10\x14a\x10pWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x10eV[_\x92\x91\x81T\x91a\x10\x9D\x83a\x10VV[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a\x10\xF2WP`\x01\x14a\x10\xB9WPPPV[_\x90\x81R` \x81 \x93\x94P\x91\x92[\x83\x83\x10a\x10\xD8WP` \x92P\x01\x01\x90V[`\x01\x81` \x92\x94\x93\x94T\x83\x85\x87\x01\x01R\x01\x91\x01\x91\x90a\x10\xC7V[\x91PP` \x93\x94P`\xFF\x92\x91\x92\x19\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x11>W`@RV[a\x11\x0FV[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x11>W`@RV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x04 WV[`@Q\x90a\x11\x97`@\x83a\x11CV[V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90a\x11\xC0\x82a\x11#V[_` \x83\x82\x81R\x01RV[\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x80\x15a\x12NW`\x02T\x82\x81\x01\x80\x91\x11a\x04 W`\x02Ua\x11\xF6\x84a\x0CDV[\x80T\x83\x01\x90U`@Q\x82\x81R_\x90_Q` a\x1E\xD6_9_Q\x90_R\x90` \x90\xA3`\x02T`\x01`\x01`\xD0\x1B\x03\x93\x90\x84\x81\x11a\x127WPa\x11\x97\x92\x93Pa\x1C{V[\x84\x90c\x0EX\xAE\x93`\xE1\x1B_R`\x04R`$R`D_\xFD[c\xECD/\x05`\xE0\x1B_R_`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\x04R`$R`DR`d\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x93\x92\x91\x90\x84\x15a\x0C\x0BW`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x15a\x12NWa\x12\xA5\x82a\x0CDV[T\x95\x84\x87\x10a\x12\xECW\x84a\x11\x97\x96\x97\x03a\x12\xBE\x84a\x0CDV[Ua\x12\xC8\x84a\x0CDV[\x80T\x86\x01\x90U`@Q\x85\x81R_Q` a\x1E\xD6_9_Q\x90_R\x90` \x90\xA3a\x1C\xE1V[c9\x144\xE3`\xE2\x1B_Ra\x0C\x08\x85\x88\x85a\x12aV[0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x13\xF4W[\x15a\x13\\W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q` \x81\x01\x90\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x13\xEE`\xC0\x82a\x11CV[Q\x90 \x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\x133V[e\xFF\xFF\xFF\xFF\xFF\xFFa\x14-Ca\x169V[\x16\x80\x82\x10\x15a\x14@WPa\x02N\x90a\x169V[\x90cvi\xFC\x0F`\xE1\x1B_R`\x04R`$R`D_\xFD[_\x19\x81\x01\x91\x90\x82\x11a\x04 WV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\x08` R`@\x81 \x80T\x86\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92Ua\x11\x97\x96\x94\x16\x94a\x14\xCA\x93\x90\x92\x86\x91\x90\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x90\x80\xA4a\x11\x99V[\x91a\x18GV[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x14\xE4WV[c\x11\x8C\xDA\xA7`\xE0\x1B_R3`\x04R`$_\xFD[\x80T\x90\x81a\x15\x05WPP_\x90V[\x81_\x19\x81\x01\x11a\x04 W_R_\x19\x90` _ \x01\x01T`0\x1C\x90V[`B\x90a\x15,a\x13\x01V[\x90`@Q\x91a\x19\x01`\xF0\x1B\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[\x91a\x02N\x93\x91a\r\x87\x93a\x19\xBCV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x07` R`@\x90 \x80T`\x01\x81\x01\x90\x91U\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x15\xF0W`\x01`\x01`\xA0\x1B\x03\x81\x16\x92\x83\x15a\x15\xDDW\x80a\x15\xD0\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93\x85_R`\x01` R`@_ a\x0CvV[U`@Q\x90\x81R` \x90\xA3V[cJ\x14\x06\xB1`\xE1\x1B_R_`\x04R`$_\xFD[c\xE6\x02\xDF\x05`\xE0\x1B_R_`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x15\xF0W`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x15\xDDWa\x166\x91_R`\x01` R`@_ a\x0CvV[UV[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x16QWe\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[c\x06\xDF\xCCe`\xE4\x1B_R`0`\x04R`$R`D_\xFD[\x81\x15a\x16rW\x04\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[`\x01\x81\x11\x15a\x02NW\x80`\x01`\x01`\x80\x1B\x82\x10\x15a\x17\xA6W[a\x17La\x17Ba\x178a\x17.a\x17$a\x17\x1Aa\x17\ta\x17S\x97`\x04a\x17X\x9A`\x01`@\x1B\x81\x10\x15a\x17\x99W[`\x01` \x1B\x81\x10\x15a\x17\x8CW[b\x01\0\0\x81\x10\x15a\x17\x7FW[a\x01\0\x81\x10\x15a\x17rW[`\x10\x81\x10\x15a\x17eW[\x10\x15a\x17]W[`\x03\x02`\x01\x1C\x90V[a\x17\x13\x81\x8Ba\x16hV[\x01`\x01\x1C\x90V[a\x17\x13\x81\x8Aa\x16hV[a\x17\x13\x81\x89a\x16hV[a\x17\x13\x81\x88a\x16hV[a\x17\x13\x81\x87a\x16hV[a\x17\x13\x81\x86a\x16hV[\x80\x93a\x16hV[\x82\x11\x90V[\x90\x03\x90V[`\x01\x1Ba\x17\0V[`\x04\x1C\x91`\x02\x1B\x91a\x16\xF9V[`\x08\x1C\x91`\x04\x1B\x91a\x16\xEFV[`\x10\x1C\x91`\x08\x1B\x91a\x16\xE4V[` \x1C\x91`\x10\x1B\x91a\x16\xD8V[`@\x1C\x91` \x1B\x91a\x16\xCBV[PPa\x17Xa\x17Sa\x17La\x17Ba\x178a\x17.a\x17$a\x17\x1Aa\x17\ta\x17\xCD\x8A`\x80\x1C\x90V[\x98P`\x01`@\x1B\x97Pa\x16\x9F\x96PPPPPPPV[\x91\x90[\x83\x82\x10a\x17\xF3WPPP\x90V[\x90\x91\x92\x80\x83\x16\x90\x80\x84\x18`\x01\x1C\x82\x01\x80\x92\x11a\x04 W\x84_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x10_\x14a\x185WP\x92[\x91\x90a\x17\xE6V[\x93\x92Pa\x18A\x90a\x11zV[\x91a\x18.V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x93\x92\x91\x90\x81\x16\x90\x81\x85\x14\x15\x80a\x19\x1CW[a\x18oW[PPPPPV[\x81a\x18\xD1W[PP\x82a\x18\x84W[\x80\x80a\x18hV[_Q` a\x1E\xF6_9_Q\x90_R\x91a\x18\xA8a\x18\xA2a\x18\xAE\x93a\x0C]V[\x91a\x1B\x02V[\x90a\x1BkV[`@\x80Q`\x01`\x01`\xD0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R\xA2_\x80\x80a\x18}V[a\x18\xFAa\x18\xEB_Q` a\x1E\xF6_9_Q\x90_R\x92a\x0C]V[a\x18\xF4\x86a\x1B\x02V[\x90a\x1B3V[`@\x80Q`\x01`\x01`\xD0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R\xA2_\x80a\x18uV[P\x83\x15\x15a\x18cV[`\xFF\x81\x14a\x19kW`\xFF\x81\x16\x90`\x1F\x82\x11a\x19\\W`@Q\x91a\x19I`@\x84a\x11CV[` \x80\x84R\x83\x81\x01\x91\x906\x837\x83RR\x90V[c,\xD4J\xC3`\xE2\x1B_R`\x04_\xFD[P`@Qa\x02N\x81a\x19~\x81`\x05a\x10\x8EV[\x03\x82a\x11CV[`\xFF\x81\x14a\x19\xA9W`\xFF\x81\x16\x90`\x1F\x82\x11a\x19\\W`@Q\x91a\x19I`@\x84a\x11CV[P`@Qa\x02N\x81a\x19~\x81`\x06a\x10\x8EV[\x91\x90o\xA2\xA8\x91\x8C\xA8[\xAF\xE2 \x16\xD0\xB9\x97\xE4\xDF``\x01`\xFF\x1B\x03\x84\x11a\x1A4W\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x1A)W_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x1A\x1FW\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[`@Q=_\x82>=\x90\xFD[PPP_\x91`\x03\x91\x90V[`\x04\x11\x15a\x1AIWV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[a\x1Af\x81a\x1A?V[\x80a\x1AoWPPV[a\x1Ax\x81a\x1A?V[`\x01\x81\x03a\x1A\x8FWc\xF6E\xEE\xDF`\xE0\x1B_R`\x04_\xFD[a\x1A\x98\x81a\x1A?V[`\x02\x81\x03a\x1A\xB3WPc\xFC\xE6\x98\xF7`\xE0\x1B_R`\x04R`$_\xFD[\x80a\x1A\xBF`\x03\x92a\x1A?V[\x14a\x1A\xC7WPV[c5\xE2\xF3\x83`\xE2\x1B_R`\x04R`$_\xFD[\x80T\x82\x10\x15a\x1A\xEEW_R` _ \x01\x90_\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\x01`\x01`\xD0\x1B\x03\x81\x11a\x1B\x1CW`\x01`\x01`\xD0\x1B\x03\x16\x90V[c\x06\xDF\xCCe`\xE4\x1B_R`\xD0`\x04R`$R`D_\xFD[\x90a\x1B=Ca\x169V[\x90a\x1BG\x83a\x14\xF7V[`\x01`\x01`\xD0\x1B\x03\x91\x82\x16\x90\x82\x16\x03\x91\x90\x82\x11a\x04 Wa\x1Bg\x92a\x1D\xD3V[\x90\x91V[\x90a\x1BuCa\x169V[\x90a\x1B\x7F\x83a\x14\xF7V[`\x01`\x01`\xD0\x1B\x03\x91\x82\x16\x90\x82\x16\x01\x91\x90\x82\x11a\x04 Wa\x1Bg\x92a\x1D\xD3V[a\x1B\xA8Ca\x169V[\x90a\x1B\xB3`\na\x14\xF7V[`\x01`\x01`\xD0\x1B\x03\x91\x82\x16\x90\x82\x16\x01\x90\x81\x11a\x04 Wa\x1Bg\x91`\na\x1D\xD3V[a\x1B\xDDCa\x169V[\x90a\x1B\xE8`\na\x14\xF7V[`\x01`\x01`\xD0\x1B\x03\x91\x82\x16\x90\x82\x16\x03\x90\x81\x11a\x04 Wa\x1Bg\x91`\na\x1D\xD3V[a\x11\x97\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x1CcW[a\x1C/a\x1C*\x83a\x1B\x02V[a\x1B\xD4V[PP_\x90\x81R`\x08` R`@\x81 T\x90\x80R_Q` a\x1E\x96_9_Q\x90_RT`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16a\x18GV[a\x1Cta\x1Co\x83a\x1B\x02V[a\x1B\x9FV[PPa\x1C\x1EV[\x90a\x11\x97\x91a\x1C\x8Ca\x1Co\x83a\x1B\x02V[PP`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x1C\xCEW[`\x08` R_Q` a\x1E\x96_9_Q\x90_RT_\x91\x82R`@\x90\x91 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16a\x18GV[a\x1C\xDAa\x1C*\x83a\x1B\x02V[PPa\x1C\x9EV[a\x11\x97\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x81\x15a\x1DDW[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x1D1W[_\x90\x81R`\x08` R`@\x80\x82 T\x92\x82R\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16a\x18GV[a\x1D=a\x1C*\x84a\x1B\x02V[PPa\x1D\nV[a\x1DPa\x1Co\x84a\x1B\x02V[PPa\x1C\xF9V[\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90`0\x1B\x16\x91\x16\x17\x90UV[\x80T`\x01`@\x1B\x81\x10\x15a\x11>Wa\x1D\x8D\x91`\x01\x82\x01\x81Ua\x1A\xD9V[a\x1D\xC0W\x81e\xFF\xFF\xFF\xFF\xFF\xFFa\x11\x97\x93Q\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83T\x16\x17\x82U` `\x01\x80`\xD0\x1B\x03\x91\x01Q\x16\x90a\x1DWV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x80T\x92\x93\x92\x80\x15a\x1EkWa\x1D\xEAa\x1D\xF5\x91a\x14VV[\x82_R` _ \x01\x90V[\x80T`0\x81\x90\x1C\x93e\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92\x91\x81\x16\x80\x84\x11a\x1E\\W\x87\x93\x03a\x1E(WPa\x1E$\x92Pa\x1DWV[\x91\x90V[\x91PPa\x1E$\x91a\x1EHa\x1E:a\x11\x88V[e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[`\x01`\x01`\xD0\x1B\x03\x86\x16` \x83\x01Ra\x1DpV[c% `\x1D`\xE0\x1B_R`\x04_\xFD[P\x90a\x1E\x90\x91a\x1E|a\x1E:a\x11\x88V[`\x01`\x01`\xD0\x1B\x03\x85\x16` \x83\x01Ra\x1DpV[_\x91\x90V\xFE^\xFF\x88n\xA0\xCEl\xA4\x88\xA3\xD6\xE36\xD6\xC0\xF7_F\xD1\x9BB\xC0l\xE5\xEE\x98\xE4,\x96\xD2V\xC7\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `CheckpointUnorderedInsertion()` and selector `0x2520601d`.
```solidity
error CheckpointUnorderedInsertion();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckpointUnorderedInsertion;
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
        impl ::core::convert::From<CheckpointUnorderedInsertion>
        for UnderlyingRustTuple<'_> {
            fn from(value: CheckpointUnorderedInsertion) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CheckpointUnorderedInsertion {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CheckpointUnorderedInsertion {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CheckpointUnorderedInsertion()";
            const SELECTOR: [u8; 4] = [37u8, 32u8, 96u8, 29u8];
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
    /**Custom error with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`.
```solidity
error ECDSAInvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignature;
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
        impl ::core::convert::From<ECDSAInvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignature()";
            const SELECTOR: [u8; 4] = [246u8, 69u8, 238u8, 223u8];
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
    /**Custom error with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`.
```solidity
error ECDSAInvalidSignatureLength(uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureLength {
        #[allow(missing_docs)]
        pub length: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<ECDSAInvalidSignatureLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureLength) -> Self {
                (value.length,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ECDSAInvalidSignatureLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { length: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureLength(uint256)";
            const SELECTOR: [u8; 4] = [252u8, 230u8, 152u8, 247u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
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
    /**Custom error with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`.
```solidity
error ECDSAInvalidSignatureS(bytes32 s);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureS {
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<ECDSAInvalidSignatureS> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureS) -> Self {
                (value.s,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignatureS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { s: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureS(bytes32)";
            const SELECTOR: [u8; 4] = [215u8, 139u8, 206u8, 12u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
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
    /**Custom error with signature `ERC20ExceededSafeSupply(uint256,uint256)` and selector `0x1cb15d26`.
```solidity
error ERC20ExceededSafeSupply(uint256 increasedSupply, uint256 cap);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20ExceededSafeSupply {
        #[allow(missing_docs)]
        pub increasedSupply: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub cap: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<ERC20ExceededSafeSupply> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20ExceededSafeSupply) -> Self {
                (value.increasedSupply, value.cap)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20ExceededSafeSupply {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    increasedSupply: tuple.0,
                    cap: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20ExceededSafeSupply {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20ExceededSafeSupply(uint256,uint256)";
            const SELECTOR: [u8; 4] = [28u8, 177u8, 93u8, 38u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.increasedSupply),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.cap),
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
    /**Custom error with signature `ERC20InsufficientAllowance(address,uint256,uint256)` and selector `0xfb8f41b2`.
```solidity
error ERC20InsufficientAllowance(address spender, uint256 allowance, uint256 needed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InsufficientAllowance {
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub allowance: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub needed: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<ERC20InsufficientAllowance>
        for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InsufficientAllowance) -> Self {
                (value.spender, value.allowance, value.needed)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ERC20InsufficientAllowance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    spender: tuple.0,
                    allowance: tuple.1,
                    needed: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InsufficientAllowance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InsufficientAllowance(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [251u8, 143u8, 65u8, 178u8];
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
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.allowance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.needed),
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
    /**Custom error with signature `ERC20InsufficientBalance(address,uint256,uint256)` and selector `0xe450d38c`.
```solidity
error ERC20InsufficientBalance(address sender, uint256 balance, uint256 needed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InsufficientBalance {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub balance: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub needed: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<ERC20InsufficientBalance>
        for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InsufficientBalance) -> Self {
                (value.sender, value.balance, value.needed)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ERC20InsufficientBalance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    sender: tuple.0,
                    balance: tuple.1,
                    needed: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InsufficientBalance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InsufficientBalance(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [228u8, 80u8, 211u8, 140u8];
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
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.balance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.needed),
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
    /**Custom error with signature `ERC20InvalidApprover(address)` and selector `0xe602df05`.
```solidity
error ERC20InvalidApprover(address approver);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InvalidApprover {
        #[allow(missing_docs)]
        pub approver: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<ERC20InvalidApprover> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InvalidApprover) -> Self {
                (value.approver,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20InvalidApprover {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { approver: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InvalidApprover {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InvalidApprover(address)";
            const SELECTOR: [u8; 4] = [230u8, 2u8, 223u8, 5u8];
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
                        &self.approver,
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
    /**Custom error with signature `ERC20InvalidReceiver(address)` and selector `0xec442f05`.
```solidity
error ERC20InvalidReceiver(address receiver);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InvalidReceiver {
        #[allow(missing_docs)]
        pub receiver: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<ERC20InvalidReceiver> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InvalidReceiver) -> Self {
                (value.receiver,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20InvalidReceiver {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { receiver: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InvalidReceiver {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InvalidReceiver(address)";
            const SELECTOR: [u8; 4] = [236u8, 68u8, 47u8, 5u8];
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
                        &self.receiver,
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
    /**Custom error with signature `ERC20InvalidSender(address)` and selector `0x96c6fd1e`.
```solidity
error ERC20InvalidSender(address sender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InvalidSender {
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
        impl ::core::convert::From<ERC20InvalidSender> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InvalidSender) -> Self {
                (value.sender,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20InvalidSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { sender: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InvalidSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InvalidSender(address)";
            const SELECTOR: [u8; 4] = [150u8, 198u8, 253u8, 30u8];
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
                        &self.sender,
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
    /**Custom error with signature `ERC20InvalidSpender(address)` and selector `0x94280d62`.
```solidity
error ERC20InvalidSpender(address spender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InvalidSpender {
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<ERC20InvalidSpender> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InvalidSpender) -> Self {
                (value.spender,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20InvalidSpender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { spender: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InvalidSpender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InvalidSpender(address)";
            const SELECTOR: [u8; 4] = [148u8, 40u8, 13u8, 98u8];
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
                        &self.spender,
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
    /**Custom error with signature `ERC2612ExpiredSignature(uint256)` and selector `0x62791302`.
```solidity
error ERC2612ExpiredSignature(uint256 deadline);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC2612ExpiredSignature {
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
        impl ::core::convert::From<ERC2612ExpiredSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ERC2612ExpiredSignature) -> Self {
                (value.deadline,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC2612ExpiredSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { deadline: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC2612ExpiredSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC2612ExpiredSignature(uint256)";
            const SELECTOR: [u8; 4] = [98u8, 121u8, 19u8, 2u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
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
    /**Custom error with signature `ERC2612InvalidSigner(address,address)` and selector `0x4b800e46`.
```solidity
error ERC2612InvalidSigner(address signer, address owner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC2612InvalidSigner {
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<ERC2612InvalidSigner> for UnderlyingRustTuple<'_> {
            fn from(value: ERC2612InvalidSigner) -> Self {
                (value.signer, value.owner)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC2612InvalidSigner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signer: tuple.0,
                    owner: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC2612InvalidSigner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC2612InvalidSigner(address,address)";
            const SELECTOR: [u8; 4] = [75u8, 128u8, 14u8, 70u8];
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
                        &self.signer,
                    ),
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
    /**Custom error with signature `ERC5805FutureLookup(uint256,uint48)` and selector `0xecd3f81e`.
```solidity
error ERC5805FutureLookup(uint256 timepoint, uint48 clock);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC5805FutureLookup {
        #[allow(missing_docs)]
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub clock: alloy::sol_types::private::primitives::aliases::U48,
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
            alloy::sol_types::sol_data::Uint<48>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U48,
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
        impl ::core::convert::From<ERC5805FutureLookup> for UnderlyingRustTuple<'_> {
            fn from(value: ERC5805FutureLookup) -> Self {
                (value.timepoint, value.clock)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC5805FutureLookup {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    timepoint: tuple.0,
                    clock: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC5805FutureLookup {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC5805FutureLookup(uint256,uint48)";
            const SELECTOR: [u8; 4] = [236u8, 211u8, 248u8, 30u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.timepoint),
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self.clock),
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
    /**Custom error with signature `ERC6372InconsistentClock()` and selector `0x6ff07140`.
```solidity
error ERC6372InconsistentClock();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC6372InconsistentClock;
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
        impl ::core::convert::From<ERC6372InconsistentClock>
        for UnderlyingRustTuple<'_> {
            fn from(value: ERC6372InconsistentClock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ERC6372InconsistentClock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC6372InconsistentClock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC6372InconsistentClock()";
            const SELECTOR: [u8; 4] = [111u8, 240u8, 113u8, 64u8];
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
    /**Custom error with signature `InvalidAccountNonce(address,uint256)` and selector `0x752d88c0`.
```solidity
error InvalidAccountNonce(address account, uint256 currentNonce);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAccountNonce {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub currentNonce: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InvalidAccountNonce> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAccountNonce) -> Self {
                (value.account, value.currentNonce)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAccountNonce {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    account: tuple.0,
                    currentNonce: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAccountNonce {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAccountNonce(address,uint256)";
            const SELECTOR: [u8; 4] = [117u8, 45u8, 136u8, 192u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentNonce),
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
    /**Custom error with signature `InvalidShortString()` and selector `0xb3512b0c`.
```solidity
error InvalidShortString();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidShortString;
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
        impl ::core::convert::From<InvalidShortString> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidShortString) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidShortString {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidShortString {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidShortString()";
            const SELECTOR: [u8; 4] = [179u8, 81u8, 43u8, 12u8];
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
    /**Custom error with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`.
```solidity
error OwnableInvalidOwner(address owner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableInvalidOwner {
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
        impl ::core::convert::From<OwnableInvalidOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OwnableInvalidOwner) -> Self {
                (value.owner,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OwnableInvalidOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { owner: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableInvalidOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableInvalidOwner(address)";
            const SELECTOR: [u8; 4] = [30u8, 79u8, 189u8, 247u8];
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
    /**Custom error with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`.
```solidity
error OwnableUnauthorizedAccount(address account);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableUnauthorizedAccount {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<OwnableUnauthorizedAccount>
        for UnderlyingRustTuple<'_> {
            fn from(value: OwnableUnauthorizedAccount) -> Self {
                (value.account,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OwnableUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableUnauthorizedAccount(address)";
            const SELECTOR: [u8; 4] = [17u8, 140u8, 218u8, 167u8];
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
    /**Custom error with signature `SafeCastOverflowedUintDowncast(uint8,uint256)` and selector `0x6dfcc650`.
```solidity
error SafeCastOverflowedUintDowncast(uint8 bits, uint256 value);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SafeCastOverflowedUintDowncast {
        #[allow(missing_docs)]
        pub bits: u8,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u8,
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
        impl ::core::convert::From<SafeCastOverflowedUintDowncast>
        for UnderlyingRustTuple<'_> {
            fn from(value: SafeCastOverflowedUintDowncast) -> Self {
                (value.bits, value.value)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SafeCastOverflowedUintDowncast {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    bits: tuple.0,
                    value: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SafeCastOverflowedUintDowncast {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SafeCastOverflowedUintDowncast(uint8,uint256)";
            const SELECTOR: [u8; 4] = [109u8, 252u8, 198u8, 80u8];
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.bits),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
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
    /**Custom error with signature `StringTooLong(string)` and selector `0x305a27a9`.
```solidity
error StringTooLong(string str);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StringTooLong {
        #[allow(missing_docs)]
        pub str: alloy::sol_types::private::String,
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
        impl ::core::convert::From<StringTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: StringTooLong) -> Self {
                (value.str,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StringTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { str: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StringTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StringTooLong(string)";
            const SELECTOR: [u8; 4] = [48u8, 90u8, 39u8, 169u8];
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
                        &self.str,
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
    /**Custom error with signature `VotesExpiredSignature(uint256)` and selector `0x4683af0e`.
```solidity
error VotesExpiredSignature(uint256 expiry);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VotesExpiredSignature {
        #[allow(missing_docs)]
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<VotesExpiredSignature> for UnderlyingRustTuple<'_> {
            fn from(value: VotesExpiredSignature) -> Self {
                (value.expiry,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for VotesExpiredSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { expiry: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for VotesExpiredSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "VotesExpiredSignature(uint256)";
            const SELECTOR: [u8; 4] = [70u8, 131u8, 175u8, 14u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
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
    /**Event with signature `Approval(address,address,uint256)` and selector `0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925`.
```solidity
event Approval(address indexed owner, address indexed spender, uint256 value);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Approval {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Approval {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Approval(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                140u8, 91u8, 225u8, 229u8, 235u8, 236u8, 125u8, 91u8, 209u8, 79u8, 113u8,
                66u8, 125u8, 30u8, 132u8, 243u8, 221u8, 3u8, 20u8, 192u8, 247u8, 178u8,
                41u8, 30u8, 91u8, 32u8, 10u8, 200u8, 199u8, 195u8, 185u8, 37u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    owner: topics.1,
                    spender: topics.2,
                    value: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.owner.clone(), self.spender.clone())
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
                    &self.owner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.spender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Approval {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Approval> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Approval) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DelegateChanged(address,address,address)` and selector `0x3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f`.
```solidity
event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DelegateChanged {
        #[allow(missing_docs)]
        pub delegator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub fromDelegate: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub toDelegate: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for DelegateChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DelegateChanged(address,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                49u8, 52u8, 232u8, 162u8, 230u8, 217u8, 126u8, 146u8, 154u8, 126u8, 84u8,
                1u8, 30u8, 165u8, 72u8, 93u8, 125u8, 25u8, 109u8, 213u8, 240u8, 186u8,
                77u8, 78u8, 249u8, 88u8, 3u8, 232u8, 227u8, 252u8, 37u8, 127u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    delegator: topics.1,
                    fromDelegate: topics.2,
                    toDelegate: topics.3,
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
                    self.delegator.clone(),
                    self.fromDelegate.clone(),
                    self.toDelegate.clone(),
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
                    &self.delegator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.fromDelegate,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.toDelegate,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DelegateChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DelegateChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DelegateChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DelegateVotesChanged(address,uint256,uint256)` and selector `0xdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724`.
```solidity
event DelegateVotesChanged(address indexed delegate, uint256 previousVotes, uint256 newVotes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DelegateVotesChanged {
        #[allow(missing_docs)]
        pub delegate: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub previousVotes: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newVotes: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for DelegateVotesChanged {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DelegateVotesChanged(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                222u8, 194u8, 186u8, 205u8, 210u8, 240u8, 91u8, 89u8, 222u8, 52u8, 218u8,
                155u8, 82u8, 61u8, 255u8, 139u8, 228u8, 46u8, 94u8, 56u8, 232u8, 24u8,
                200u8, 47u8, 219u8, 11u8, 174u8, 119u8, 67u8, 135u8, 167u8, 36u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    delegate: topics.1,
                    previousVotes: data.0,
                    newVotes: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.previousVotes),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newVotes),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.delegate.clone())
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
                    &self.delegate,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DelegateVotesChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DelegateVotesChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DelegateVotesChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EIP712DomainChanged()` and selector `0x0a6387c9ea3628b88a633bb4f3b151770f70085117a15f9bf3787cda53f13d31`.
```solidity
event EIP712DomainChanged();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EIP712DomainChanged;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EIP712DomainChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EIP712DomainChanged()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                10u8, 99u8, 135u8, 201u8, 234u8, 54u8, 40u8, 184u8, 138u8, 99u8, 59u8,
                180u8, 243u8, 177u8, 81u8, 119u8, 15u8, 112u8, 8u8, 81u8, 23u8, 161u8,
                95u8, 155u8, 243u8, 120u8, 124u8, 218u8, 83u8, 241u8, 61u8, 49u8,
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
        impl alloy_sol_types::private::IntoLogData for EIP712DomainChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EIP712DomainChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EIP712DomainChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `FaucetAmountSet(uint256)` and selector `0x6b485e2740573a57af1fe9acd3f9a33ea4b26b8cd88c6d2daa967036d1b1c38d`.
```solidity
event FaucetAmountSet(uint256 faucetAmount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct FaucetAmountSet {
        #[allow(missing_docs)]
        pub faucetAmount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for FaucetAmountSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "FaucetAmountSet(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                107u8, 72u8, 94u8, 39u8, 64u8, 87u8, 58u8, 87u8, 175u8, 31u8, 233u8,
                172u8, 211u8, 249u8, 163u8, 62u8, 164u8, 178u8, 107u8, 140u8, 216u8,
                140u8, 109u8, 45u8, 170u8, 150u8, 112u8, 54u8, 209u8, 177u8, 195u8, 141u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { faucetAmount: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.faucetAmount),
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
        impl alloy_sol_types::private::IntoLogData for FaucetAmountSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&FaucetAmountSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &FaucetAmountSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
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
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
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
                    previousOwner: topics.1,
                    newOwner: topics.2,
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
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
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
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
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
    /**Event with signature `Transfer(address,address,uint256)` and selector `0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef`.
```solidity
event Transfer(address indexed from, address indexed to, uint256 value);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Transfer {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Transfer {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Transfer(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                221u8, 242u8, 82u8, 173u8, 27u8, 226u8, 200u8, 155u8, 105u8, 194u8,
                176u8, 104u8, 252u8, 55u8, 141u8, 170u8, 149u8, 43u8, 167u8, 241u8, 99u8,
                196u8, 161u8, 22u8, 40u8, 245u8, 90u8, 77u8, 245u8, 35u8, 179u8, 239u8,
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
                    value: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
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
        impl alloy_sol_types::private::IntoLogData for Transfer {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Transfer> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Transfer) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(string name, string symbol, uint8 decimals_, uint256 faucetAmount_, address _owner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub symbol: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub decimals_: u8,
        #[allow(missing_docs)]
        pub faucetAmount_: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _owner: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
                u8,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value.name,
                        value.symbol,
                        value.decimals_,
                        value.faucetAmount_,
                        value._owner,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        name: tuple.0,
                        symbol: tuple.1,
                        decimals_: tuple.2,
                        faucetAmount_: tuple.3,
                        _owner: tuple.4,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.symbol,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals_),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.faucetAmount_),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._owner,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`.
```solidity
function CLOCK_MODE() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOCK_MODECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`CLOCK_MODE()`](CLOCK_MODECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOCK_MODEReturn {
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
            impl ::core::convert::From<CLOCK_MODECall> for UnderlyingRustTuple<'_> {
                fn from(value: CLOCK_MODECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOCK_MODECall {
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
            impl ::core::convert::From<CLOCK_MODEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: CLOCK_MODEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOCK_MODEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CLOCK_MODECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CLOCK_MODE()";
            const SELECTOR: [u8; 4] = [75u8, 245u8, 215u8, 233u8];
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
                        let r: CLOCK_MODEReturn = r.into();
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
                        let r: CLOCK_MODEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`.
```solidity
function DOMAIN_SEPARATOR() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_SEPARATORCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`DOMAIN_SEPARATOR()`](DOMAIN_SEPARATORCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_SEPARATORReturn {
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
            impl ::core::convert::From<DOMAIN_SEPARATORCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DOMAIN_SEPARATORCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DOMAIN_SEPARATORCall {
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
            impl ::core::convert::From<DOMAIN_SEPARATORReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DOMAIN_SEPARATORReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DOMAIN_SEPARATORReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DOMAIN_SEPARATORCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DOMAIN_SEPARATOR()";
            const SELECTOR: [u8; 4] = [54u8, 68u8, 229u8, 21u8];
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
                        let r: DOMAIN_SEPARATORReturn = r.into();
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
                        let r: DOMAIN_SEPARATORReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `FAUCET_INTERVAL()` and selector `0x6df62cef`.
```solidity
function FAUCET_INTERVAL() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FAUCET_INTERVALCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`FAUCET_INTERVAL()`](FAUCET_INTERVALCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FAUCET_INTERVALReturn {
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
            impl ::core::convert::From<FAUCET_INTERVALCall> for UnderlyingRustTuple<'_> {
                fn from(value: FAUCET_INTERVALCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for FAUCET_INTERVALCall {
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
            impl ::core::convert::From<FAUCET_INTERVALReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: FAUCET_INTERVALReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for FAUCET_INTERVALReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for FAUCET_INTERVALCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FAUCET_INTERVAL()";
            const SELECTOR: [u8; 4] = [109u8, 246u8, 44u8, 239u8];
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
                        let r: FAUCET_INTERVALReturn = r.into();
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
                        let r: FAUCET_INTERVALReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `allowance(address,address)` and selector `0xdd62ed3e`.
```solidity
function allowance(address owner, address spender) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allowanceCall {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`allowance(address,address)`](allowanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allowanceReturn {
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
            impl ::core::convert::From<allowanceCall> for UnderlyingRustTuple<'_> {
                fn from(value: allowanceCall) -> Self {
                    (value.owner, value.spender)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        spender: tuple.1,
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
            impl ::core::convert::From<allowanceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allowanceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allowanceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allowance(address,address)";
            const SELECTOR: [u8; 4] = [221u8, 98u8, 237u8, 62u8];
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
                        &self.spender,
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
                        let r: allowanceReturn = r.into();
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
                        let r: allowanceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `approve(address,uint256)` and selector `0x095ea7b3`.
```solidity
function approve(address spender, uint256 value) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveCall {
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`approve(address,uint256)`](approveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveReturn {
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
            impl ::core::convert::From<approveCall> for UnderlyingRustTuple<'_> {
                fn from(value: approveCall) -> Self {
                    (value.spender, value.value)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        spender: tuple.0,
                        value: tuple.1,
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
            impl ::core::convert::From<approveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: approveReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for approveCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "approve(address,uint256)";
            const SELECTOR: [u8; 4] = [9u8, 94u8, 167u8, 179u8];
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
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
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
                        let r: approveReturn = r.into();
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
                        let r: approveReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `balanceOf(address)` and selector `0x70a08231`.
```solidity
function balanceOf(address account) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`balanceOf(address)`](balanceOfCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfReturn {
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
            impl ::core::convert::From<balanceOfCall> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
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
            impl ::core::convert::From<balanceOfReturn> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for balanceOfCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "balanceOf(address)";
            const SELECTOR: [u8; 4] = [112u8, 160u8, 130u8, 49u8];
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
                        let r: balanceOfReturn = r.into();
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
                        let r: balanceOfReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `burn(address,uint256)` and selector `0x9dc29fac`.
```solidity
function burn(address account, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct burnCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`burn(address,uint256)`](burnCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct burnReturn {}
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
            impl ::core::convert::From<burnCall> for UnderlyingRustTuple<'_> {
                fn from(value: burnCall) -> Self {
                    (value.account, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        amount: tuple.1,
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
            impl ::core::convert::From<burnReturn> for UnderlyingRustTuple<'_> {
                fn from(value: burnReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl burnReturn {
            fn _tokenize(
                &self,
            ) -> <burnCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for burnCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = burnReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "burn(address,uint256)";
            const SELECTOR: [u8; 4] = [157u8, 194u8, 159u8, 172u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                burnReturn::_tokenize(ret)
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
    /**Function with signature `checkpoints(address,uint32)` and selector `0xf1127ed8`.
```solidity
function checkpoints(address account, uint32 pos) external view returns (Checkpoints.Checkpoint208 memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointsCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub pos: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`checkpoints(address,uint32)`](checkpointsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointsReturn {
        #[allow(missing_docs)]
        pub _0: <Checkpoints::Checkpoint208 as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkpointsCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkpointsCall) -> Self {
                    (value.account, value.pos)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkpointsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        pos: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Checkpoints::Checkpoint208,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Checkpoints::Checkpoint208 as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<checkpointsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkpointsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkpointsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkpointsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Checkpoints::Checkpoint208 as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Checkpoints::Checkpoint208,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkpoints(address,uint32)";
            const SELECTOR: [u8; 4] = [241u8, 18u8, 126u8, 216u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.pos),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <Checkpoints::Checkpoint208 as alloy_sol_types::SolType>::tokenize(
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
                        let r: checkpointsReturn = r.into();
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
                        let r: checkpointsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `clock()` and selector `0x91ddadf4`.
```solidity
function clock() external view returns (uint48);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`clock()`](clockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U48,
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
            impl ::core::convert::From<clockCall> for UnderlyingRustTuple<'_> {
                fn from(value: clockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<48>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U48,
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
            impl ::core::convert::From<clockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: clockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U48;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<48>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clock()";
            const SELECTOR: [u8; 4] = [145u8, 221u8, 173u8, 244u8];
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
                        48,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: clockReturn = r.into();
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
                        let r: clockReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `decimals()` and selector `0x313ce567`.
```solidity
function decimals() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decimalsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`decimals()`](decimalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decimalsReturn {
        #[allow(missing_docs)]
        pub _0: u8,
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
            impl ::core::convert::From<decimalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: decimalsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for decimalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<decimalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: decimalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for decimalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for decimalsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u8;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "decimals()";
            const SELECTOR: [u8; 4] = [49u8, 60u8, 229u8, 103u8];
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: decimalsReturn = r.into();
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
                        let r: decimalsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `delegate(address)` and selector `0x5c19a95c`.
```solidity
function delegate(address delegatee) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateCall {
        #[allow(missing_docs)]
        pub delegatee: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegate(address)`](delegateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateReturn {}
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
            impl ::core::convert::From<delegateCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateCall) -> Self {
                    (value.delegatee,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { delegatee: tuple.0 }
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
            impl ::core::convert::From<delegateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl delegateReturn {
            fn _tokenize(
                &self,
            ) -> <delegateCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegate(address)";
            const SELECTOR: [u8; 4] = [92u8, 25u8, 169u8, 92u8];
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
                        &self.delegatee,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                delegateReturn::_tokenize(ret)
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
    /**Function with signature `delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xc3cda520`.
```solidity
function delegateBySig(address delegatee, uint256 nonce, uint256 expiry, uint8 v, bytes32 r, bytes32 s) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateBySigCall {
        #[allow(missing_docs)]
        pub delegatee: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub v: u8,
        #[allow(missing_docs)]
        pub r: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)`](delegateBySigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateBySigReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<delegateBySigCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateBySigCall) -> Self {
                    (
                        value.delegatee,
                        value.nonce,
                        value.expiry,
                        value.v,
                        value.r,
                        value.s,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateBySigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        delegatee: tuple.0,
                        nonce: tuple.1,
                        expiry: tuple.2,
                        v: tuple.3,
                        r: tuple.4,
                        s: tuple.5,
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
            impl ::core::convert::From<delegateBySigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateBySigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateBySigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl delegateBySigReturn {
            fn _tokenize(
                &self,
            ) -> <delegateBySigCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateBySigCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateBySigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [195u8, 205u8, 165u8, 32u8];
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
                        &self.delegatee,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.v),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.r),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                delegateBySigReturn::_tokenize(ret)
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
    /**Function with signature `delegates(address)` and selector `0x587cde1e`.
```solidity
function delegates(address account) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatesCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`delegates(address)`](delegatesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatesReturn {
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
            impl ::core::convert::From<delegatesCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegatesCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
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
            impl ::core::convert::From<delegatesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegatesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegatesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegates(address)";
            const SELECTOR: [u8; 4] = [88u8, 124u8, 222u8, 30u8];
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
                        let r: delegatesReturn = r.into();
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
                        let r: delegatesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `eip712Domain()` and selector `0x84b0196e`.
```solidity
function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`eip712Domain()`](eip712DomainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainReturn {
        #[allow(missing_docs)]
        pub fields: alloy::sol_types::private::FixedBytes<1>,
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub version: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub verifyingContract: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub extensions: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<eip712DomainCall> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<1>,
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<eip712DomainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainReturn) -> Self {
                    (
                        value.fields,
                        value.name,
                        value.version,
                        value.chainId,
                        value.verifyingContract,
                        value.salt,
                        value.extensions,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        fields: tuple.0,
                        name: tuple.1,
                        version: tuple.2,
                        chainId: tuple.3,
                        verifyingContract: tuple.4,
                        salt: tuple.5,
                        extensions: tuple.6,
                    }
                }
            }
        }
        impl eip712DomainReturn {
            fn _tokenize(
                &self,
            ) -> <eip712DomainCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        1,
                    > as alloy_sol_types::SolType>::tokenize(&self.fields),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.version,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.verifyingContract,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.extensions),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eip712DomainCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eip712DomainReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eip712Domain()";
            const SELECTOR: [u8; 4] = [132u8, 176u8, 25u8, 110u8];
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
                eip712DomainReturn::_tokenize(ret)
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
    /**Function with signature `faucetAmount()` and selector `0x9c281430`.
```solidity
function faucetAmount() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct faucetAmountCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`faucetAmount()`](faucetAmountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct faucetAmountReturn {
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
            impl ::core::convert::From<faucetAmountCall> for UnderlyingRustTuple<'_> {
                fn from(value: faucetAmountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for faucetAmountCall {
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
            impl ::core::convert::From<faucetAmountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: faucetAmountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for faucetAmountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for faucetAmountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "faucetAmount()";
            const SELECTOR: [u8; 4] = [156u8, 40u8, 20u8, 48u8];
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
                        let r: faucetAmountReturn = r.into();
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
                        let r: faucetAmountReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getPastTotalSupply(uint256)` and selector `0x8e539e8c`.
```solidity
function getPastTotalSupply(uint256 timepoint) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastTotalSupplyCall {
        #[allow(missing_docs)]
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getPastTotalSupply(uint256)`](getPastTotalSupplyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastTotalSupplyReturn {
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
            impl ::core::convert::From<getPastTotalSupplyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPastTotalSupplyCall) -> Self {
                    (value.timepoint,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPastTotalSupplyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { timepoint: tuple.0 }
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
            impl ::core::convert::From<getPastTotalSupplyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPastTotalSupplyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPastTotalSupplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPastTotalSupplyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPastTotalSupply(uint256)";
            const SELECTOR: [u8; 4] = [142u8, 83u8, 158u8, 140u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.timepoint),
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
                        let r: getPastTotalSupplyReturn = r.into();
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
                        let r: getPastTotalSupplyReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getPastVotes(address,uint256)` and selector `0x3a46b1a8`.
```solidity
function getPastVotes(address account, uint256 timepoint) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastVotesCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getPastVotes(address,uint256)`](getPastVotesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastVotesReturn {
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
            impl ::core::convert::From<getPastVotesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPastVotesCall) -> Self {
                    (value.account, value.timepoint)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPastVotesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        timepoint: tuple.1,
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
            impl ::core::convert::From<getPastVotesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPastVotesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPastVotesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPastVotesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
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
            const SIGNATURE: &'static str = "getPastVotes(address,uint256)";
            const SELECTOR: [u8; 4] = [58u8, 70u8, 177u8, 168u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timepoint),
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
                        let r: getPastVotesReturn = r.into();
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
                        let r: getPastVotesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getVotes(address)` and selector `0x9ab24eb0`.
```solidity
function getVotes(address account) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotesCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getVotes(address)`](getVotesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotesReturn {
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
            impl ::core::convert::From<getVotesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getVotesCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVotesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
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
            impl ::core::convert::From<getVotesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getVotesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVotesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getVotesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getVotes(address)";
            const SELECTOR: [u8; 4] = [154u8, 178u8, 78u8, 176u8];
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
                        let r: getVotesReturn = r.into();
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
                        let r: getVotesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `lastMint(address)` and selector `0xadc20b1c`.
```solidity
function lastMint(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastMintCall(pub alloy::sol_types::private::Address);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lastMint(address)`](lastMintCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastMintReturn {
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
            impl ::core::convert::From<lastMintCall> for UnderlyingRustTuple<'_> {
                fn from(value: lastMintCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastMintCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<lastMintReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lastMintReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastMintReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastMintCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastMint(address)";
            const SELECTOR: [u8; 4] = [173u8, 194u8, 11u8, 28u8];
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
                        &self.0,
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
                        let r: lastMintReturn = r.into();
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
                        let r: lastMintReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `mint()` and selector `0x1249c58b`.
```solidity
function mint() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mintCall;
    ///Container type for the return parameters of the [`mint()`](mintCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mintReturn {}
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
            impl ::core::convert::From<mintCall> for UnderlyingRustTuple<'_> {
                fn from(value: mintCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintCall {
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
            impl ::core::convert::From<mintReturn> for UnderlyingRustTuple<'_> {
                fn from(value: mintReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl mintReturn {
            fn _tokenize(
                &self,
            ) -> <mintCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mintCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mintReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mint()";
            const SELECTOR: [u8; 4] = [18u8, 73u8, 197u8, 139u8];
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
                mintReturn::_tokenize(ret)
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
    /**Function with signature `name()` and selector `0x06fdde03`.
```solidity
function name() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nameCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`name()`](nameCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nameReturn {
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
            impl ::core::convert::From<nameCall> for UnderlyingRustTuple<'_> {
                fn from(value: nameCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameCall {
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
            impl ::core::convert::From<nameReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nameReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nameCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "name()";
            const SELECTOR: [u8; 4] = [6u8, 253u8, 222u8, 3u8];
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
                        let r: nameReturn = r.into();
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
                        let r: nameReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `nonces(address)` and selector `0x7ecebe00`.
```solidity
function nonces(address owner) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct noncesCall {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`nonces(address)`](noncesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct noncesReturn {
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
            impl ::core::convert::From<noncesCall> for UnderlyingRustTuple<'_> {
                fn from(value: noncesCall) -> Self {
                    (value.owner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for noncesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { owner: tuple.0 }
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
            impl ::core::convert::From<noncesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: noncesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for noncesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for noncesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nonces(address)";
            const SELECTOR: [u8; 4] = [126u8, 206u8, 190u8, 0u8];
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
                        let r: noncesReturn = r.into();
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
                        let r: noncesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `numCheckpoints(address)` and selector `0x6fcfff45`.
```solidity
function numCheckpoints(address account) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numCheckpointsCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`numCheckpoints(address)`](numCheckpointsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numCheckpointsReturn {
        #[allow(missing_docs)]
        pub _0: u32,
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
            impl ::core::convert::From<numCheckpointsCall> for UnderlyingRustTuple<'_> {
                fn from(value: numCheckpointsCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for numCheckpointsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<numCheckpointsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: numCheckpointsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for numCheckpointsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for numCheckpointsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "numCheckpoints(address)";
            const SELECTOR: [u8; 4] = [111u8, 207u8, 255u8, 69u8];
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
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
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
                        let r: numCheckpointsReturn = r.into();
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
                        let r: numCheckpointsReturn = r.into();
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
    /**Function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`.
```solidity
function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permitCall {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub v: u8,
        #[allow(missing_docs)]
        pub r: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`permit(address,address,uint256,uint256,uint8,bytes32,bytes32)`](permitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permitReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<permitCall> for UnderlyingRustTuple<'_> {
                fn from(value: permitCall) -> Self {
                    (
                        value.owner,
                        value.spender,
                        value.value,
                        value.deadline,
                        value.v,
                        value.r,
                        value.s,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        spender: tuple.1,
                        value: tuple.2,
                        deadline: tuple.3,
                        v: tuple.4,
                        r: tuple.5,
                        s: tuple.6,
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
            impl ::core::convert::From<permitReturn> for UnderlyingRustTuple<'_> {
                fn from(value: permitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl permitReturn {
            fn _tokenize(
                &self,
            ) -> <permitCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for permitCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = permitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [213u8, 5u8, 172u8, 207u8];
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
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.v),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.r),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                permitReturn::_tokenize(ret)
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
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall;
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
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
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
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
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <renounceOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
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
                renounceOwnershipReturn::_tokenize(ret)
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
    /**Function with signature `setFaucetAmount(uint256)` and selector `0x81d2fd9c`.
```solidity
function setFaucetAmount(uint256 faucetAmount_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setFaucetAmountCall {
        #[allow(missing_docs)]
        pub faucetAmount_: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setFaucetAmount(uint256)`](setFaucetAmountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setFaucetAmountReturn {}
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
            impl ::core::convert::From<setFaucetAmountCall> for UnderlyingRustTuple<'_> {
                fn from(value: setFaucetAmountCall) -> Self {
                    (value.faucetAmount_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setFaucetAmountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { faucetAmount_: tuple.0 }
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
            impl ::core::convert::From<setFaucetAmountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setFaucetAmountReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setFaucetAmountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setFaucetAmountReturn {
            fn _tokenize(
                &self,
            ) -> <setFaucetAmountCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setFaucetAmountCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setFaucetAmountReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setFaucetAmount(uint256)";
            const SELECTOR: [u8; 4] = [129u8, 210u8, 253u8, 156u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.faucetAmount_),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setFaucetAmountReturn::_tokenize(ret)
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
    /**Function with signature `symbol()` and selector `0x95d89b41`.
```solidity
function symbol() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct symbolCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`symbol()`](symbolCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct symbolReturn {
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
            impl ::core::convert::From<symbolCall> for UnderlyingRustTuple<'_> {
                fn from(value: symbolCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for symbolCall {
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
            impl ::core::convert::From<symbolReturn> for UnderlyingRustTuple<'_> {
                fn from(value: symbolReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for symbolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for symbolCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "symbol()";
            const SELECTOR: [u8; 4] = [149u8, 216u8, 155u8, 65u8];
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
                        let r: symbolReturn = r.into();
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
                        let r: symbolReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `totalSupply()` and selector `0x18160ddd`.
```solidity
function totalSupply() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalSupplyCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`totalSupply()`](totalSupplyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalSupplyReturn {
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
            impl ::core::convert::From<totalSupplyCall> for UnderlyingRustTuple<'_> {
                fn from(value: totalSupplyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalSupplyCall {
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
            impl ::core::convert::From<totalSupplyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: totalSupplyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalSupplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalSupplyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "totalSupply()";
            const SELECTOR: [u8; 4] = [24u8, 22u8, 13u8, 221u8];
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
                        let r: totalSupplyReturn = r.into();
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
                        let r: totalSupplyReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `transfer(address,uint256)` and selector `0xa9059cbb`.
```solidity
function transfer(address to, uint256 value) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferCall {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`transfer(address,uint256)`](transferCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferReturn {
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
            impl ::core::convert::From<transferCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferCall) -> Self {
                    (value.to, value.value)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        value: tuple.1,
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
            impl ::core::convert::From<transferReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transfer(address,uint256)";
            const SELECTOR: [u8; 4] = [169u8, 5u8, 156u8, 187u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
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
                        let r: transferReturn = r.into();
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
                        let r: transferReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`.
```solidity
function transferFrom(address from, address to, uint256 value) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferFromCall {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`transferFrom(address,address,uint256)`](transferFromCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferFromReturn {
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<transferFromCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferFromCall) -> Self {
                    (value.from, value.to, value.value)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferFromCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        to: tuple.1,
                        value: tuple.2,
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
            impl ::core::convert::From<transferFromReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferFromReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferFromReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferFromCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferFrom(address,address,uint256)";
            const SELECTOR: [u8; 4] = [35u8, 184u8, 114u8, 221u8];
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
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
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
                        let r: transferFromReturn = r.into();
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
                        let r: transferFromReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
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
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
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
                        &self.newOwner,
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
    ///Container for all the [`ERC20FaucetToken`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum ERC20FaucetTokenCalls {
        #[allow(missing_docs)]
        CLOCK_MODE(CLOCK_MODECall),
        #[allow(missing_docs)]
        DOMAIN_SEPARATOR(DOMAIN_SEPARATORCall),
        #[allow(missing_docs)]
        FAUCET_INTERVAL(FAUCET_INTERVALCall),
        #[allow(missing_docs)]
        allowance(allowanceCall),
        #[allow(missing_docs)]
        approve(approveCall),
        #[allow(missing_docs)]
        balanceOf(balanceOfCall),
        #[allow(missing_docs)]
        burn(burnCall),
        #[allow(missing_docs)]
        checkpoints(checkpointsCall),
        #[allow(missing_docs)]
        clock(clockCall),
        #[allow(missing_docs)]
        decimals(decimalsCall),
        #[allow(missing_docs)]
        delegate(delegateCall),
        #[allow(missing_docs)]
        delegateBySig(delegateBySigCall),
        #[allow(missing_docs)]
        delegates(delegatesCall),
        #[allow(missing_docs)]
        eip712Domain(eip712DomainCall),
        #[allow(missing_docs)]
        faucetAmount(faucetAmountCall),
        #[allow(missing_docs)]
        getPastTotalSupply(getPastTotalSupplyCall),
        #[allow(missing_docs)]
        getPastVotes(getPastVotesCall),
        #[allow(missing_docs)]
        getVotes(getVotesCall),
        #[allow(missing_docs)]
        lastMint(lastMintCall),
        #[allow(missing_docs)]
        mint(mintCall),
        #[allow(missing_docs)]
        name(nameCall),
        #[allow(missing_docs)]
        nonces(noncesCall),
        #[allow(missing_docs)]
        numCheckpoints(numCheckpointsCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        permit(permitCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        setFaucetAmount(setFaucetAmountCall),
        #[allow(missing_docs)]
        symbol(symbolCall),
        #[allow(missing_docs)]
        totalSupply(totalSupplyCall),
        #[allow(missing_docs)]
        transfer(transferCall),
        #[allow(missing_docs)]
        transferFrom(transferFromCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
    }
    impl ERC20FaucetTokenCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 253u8, 222u8, 3u8],
            [9u8, 94u8, 167u8, 179u8],
            [18u8, 73u8, 197u8, 139u8],
            [24u8, 22u8, 13u8, 221u8],
            [35u8, 184u8, 114u8, 221u8],
            [49u8, 60u8, 229u8, 103u8],
            [54u8, 68u8, 229u8, 21u8],
            [58u8, 70u8, 177u8, 168u8],
            [75u8, 245u8, 215u8, 233u8],
            [88u8, 124u8, 222u8, 30u8],
            [92u8, 25u8, 169u8, 92u8],
            [109u8, 246u8, 44u8, 239u8],
            [111u8, 207u8, 255u8, 69u8],
            [112u8, 160u8, 130u8, 49u8],
            [113u8, 80u8, 24u8, 166u8],
            [126u8, 206u8, 190u8, 0u8],
            [129u8, 210u8, 253u8, 156u8],
            [132u8, 176u8, 25u8, 110u8],
            [141u8, 165u8, 203u8, 91u8],
            [142u8, 83u8, 158u8, 140u8],
            [145u8, 221u8, 173u8, 244u8],
            [149u8, 216u8, 155u8, 65u8],
            [154u8, 178u8, 78u8, 176u8],
            [156u8, 40u8, 20u8, 48u8],
            [157u8, 194u8, 159u8, 172u8],
            [169u8, 5u8, 156u8, 187u8],
            [173u8, 194u8, 11u8, 28u8],
            [195u8, 205u8, 165u8, 32u8],
            [213u8, 5u8, 172u8, 207u8],
            [221u8, 98u8, 237u8, 62u8],
            [241u8, 18u8, 126u8, 216u8],
            [242u8, 253u8, 227u8, 139u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(name),
            ::core::stringify!(approve),
            ::core::stringify!(mint),
            ::core::stringify!(totalSupply),
            ::core::stringify!(transferFrom),
            ::core::stringify!(decimals),
            ::core::stringify!(DOMAIN_SEPARATOR),
            ::core::stringify!(getPastVotes),
            ::core::stringify!(CLOCK_MODE),
            ::core::stringify!(delegates),
            ::core::stringify!(delegate),
            ::core::stringify!(FAUCET_INTERVAL),
            ::core::stringify!(numCheckpoints),
            ::core::stringify!(balanceOf),
            ::core::stringify!(renounceOwnership),
            ::core::stringify!(nonces),
            ::core::stringify!(setFaucetAmount),
            ::core::stringify!(eip712Domain),
            ::core::stringify!(owner),
            ::core::stringify!(getPastTotalSupply),
            ::core::stringify!(clock),
            ::core::stringify!(symbol),
            ::core::stringify!(getVotes),
            ::core::stringify!(faucetAmount),
            ::core::stringify!(burn),
            ::core::stringify!(transfer),
            ::core::stringify!(lastMint),
            ::core::stringify!(delegateBySig),
            ::core::stringify!(permit),
            ::core::stringify!(allowance),
            ::core::stringify!(checkpoints),
            ::core::stringify!(transferOwnership),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <nameCall as alloy_sol_types::SolCall>::SIGNATURE,
            <approveCall as alloy_sol_types::SolCall>::SIGNATURE,
            <mintCall as alloy_sol_types::SolCall>::SIGNATURE,
            <totalSupplyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferFromCall as alloy_sol_types::SolCall>::SIGNATURE,
            <decimalsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getPastVotesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <CLOCK_MODECall as alloy_sol_types::SolCall>::SIGNATURE,
            <delegatesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <delegateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <FAUCET_INTERVALCall as alloy_sol_types::SolCall>::SIGNATURE,
            <numCheckpointsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <balanceOfCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <noncesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setFaucetAmountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <eip712DomainCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getPastTotalSupplyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <clockCall as alloy_sol_types::SolCall>::SIGNATURE,
            <symbolCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getVotesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <faucetAmountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <burnCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lastMintCall as alloy_sol_types::SolCall>::SIGNATURE,
            <delegateBySigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <permitCall as alloy_sol_types::SolCall>::SIGNATURE,
            <allowanceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <checkpointsCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for ERC20FaucetTokenCalls {
        const NAME: &'static str = "ERC20FaucetTokenCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 32usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CLOCK_MODE(_) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DOMAIN_SEPARATOR(_) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::FAUCET_INTERVAL(_) => {
                    <FAUCET_INTERVALCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allowance(_) => {
                    <allowanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::approve(_) => <approveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::balanceOf(_) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::burn(_) => <burnCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::checkpoints(_) => {
                    <checkpointsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::clock(_) => <clockCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::decimals(_) => <decimalsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegate(_) => <delegateCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegateBySig(_) => {
                    <delegateBySigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegates(_) => {
                    <delegatesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::faucetAmount(_) => {
                    <faucetAmountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPastTotalSupply(_) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPastVotes(_) => {
                    <getPastVotesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVotes(_) => <getVotesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::lastMint(_) => <lastMintCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::mint(_) => <mintCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::name(_) => <nameCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::nonces(_) => <noncesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::numCheckpoints(_) => {
                    <numCheckpointsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::permit(_) => <permitCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setFaucetAmount(_) => {
                    <setFaucetAmountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::symbol(_) => <symbolCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::totalSupply(_) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transfer(_) => <transferCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::transferFrom(_) => {
                    <transferFromCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls>] = &[
                {
                    fn name(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <nameCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::name)
                    }
                    name
                },
                {
                    fn approve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <approveCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::approve)
                    }
                    approve
                },
                {
                    fn mint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <mintCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::mint)
                    }
                    mint
                },
                {
                    fn totalSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <totalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::totalSupply)
                    }
                    totalSupply
                },
                {
                    fn transferFrom(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <transferFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::transferFrom)
                    }
                    transferFrom
                },
                {
                    fn decimals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <decimalsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::decimals)
                    }
                    decimals
                },
                {
                    fn DOMAIN_SEPARATOR(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::DOMAIN_SEPARATOR)
                    }
                    DOMAIN_SEPARATOR
                },
                {
                    fn getPastVotes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <getPastVotesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::getPastVotes)
                    }
                    getPastVotes
                },
                {
                    fn CLOCK_MODE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::CLOCK_MODE)
                    }
                    CLOCK_MODE
                },
                {
                    fn delegates(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <delegatesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::delegates)
                    }
                    delegates
                },
                {
                    fn delegate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <delegateCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::delegate)
                    }
                    delegate
                },
                {
                    fn FAUCET_INTERVAL(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <FAUCET_INTERVALCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::FAUCET_INTERVAL)
                    }
                    FAUCET_INTERVAL
                },
                {
                    fn numCheckpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <numCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::numCheckpoints)
                    }
                    numCheckpoints
                },
                {
                    fn balanceOf(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::balanceOf)
                    }
                    balanceOf
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn nonces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <noncesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::nonces)
                    }
                    nonces
                },
                {
                    fn setFaucetAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <setFaucetAmountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::setFaucetAmount)
                    }
                    setFaucetAmount
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::owner)
                    }
                    owner
                },
                {
                    fn getPastTotalSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::getPastTotalSupply)
                    }
                    getPastTotalSupply
                },
                {
                    fn clock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <clockCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::clock)
                    }
                    clock
                },
                {
                    fn symbol(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <symbolCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::symbol)
                    }
                    symbol
                },
                {
                    fn getVotes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <getVotesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::getVotes)
                    }
                    getVotes
                },
                {
                    fn faucetAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <faucetAmountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::faucetAmount)
                    }
                    faucetAmount
                },
                {
                    fn burn(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <burnCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::burn)
                    }
                    burn
                },
                {
                    fn transfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <transferCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::transfer)
                    }
                    transfer
                },
                {
                    fn lastMint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <lastMintCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::lastMint)
                    }
                    lastMint
                },
                {
                    fn delegateBySig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <delegateBySigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::delegateBySig)
                    }
                    delegateBySig
                },
                {
                    fn permit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <permitCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::permit)
                    }
                    permit
                },
                {
                    fn allowance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <allowanceCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ERC20FaucetTokenCalls::allowance)
                    }
                    allowance
                },
                {
                    fn checkpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <checkpointsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::checkpoints)
                    }
                    checkpoints
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::transferOwnership)
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
            ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls>] = &[
                {
                    fn name(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <nameCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::name)
                    }
                    name
                },
                {
                    fn approve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <approveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::approve)
                    }
                    approve
                },
                {
                    fn mint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <mintCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::mint)
                    }
                    mint
                },
                {
                    fn totalSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <totalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::totalSupply)
                    }
                    totalSupply
                },
                {
                    fn transferFrom(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <transferFromCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::transferFrom)
                    }
                    transferFrom
                },
                {
                    fn decimals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <decimalsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::decimals)
                    }
                    decimals
                },
                {
                    fn DOMAIN_SEPARATOR(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::DOMAIN_SEPARATOR)
                    }
                    DOMAIN_SEPARATOR
                },
                {
                    fn getPastVotes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <getPastVotesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::getPastVotes)
                    }
                    getPastVotes
                },
                {
                    fn CLOCK_MODE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::CLOCK_MODE)
                    }
                    CLOCK_MODE
                },
                {
                    fn delegates(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <delegatesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::delegates)
                    }
                    delegates
                },
                {
                    fn delegate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <delegateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::delegate)
                    }
                    delegate
                },
                {
                    fn FAUCET_INTERVAL(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <FAUCET_INTERVALCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::FAUCET_INTERVAL)
                    }
                    FAUCET_INTERVAL
                },
                {
                    fn numCheckpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <numCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::numCheckpoints)
                    }
                    numCheckpoints
                },
                {
                    fn balanceOf(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::balanceOf)
                    }
                    balanceOf
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn nonces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <noncesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::nonces)
                    }
                    nonces
                },
                {
                    fn setFaucetAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <setFaucetAmountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::setFaucetAmount)
                    }
                    setFaucetAmount
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::owner)
                    }
                    owner
                },
                {
                    fn getPastTotalSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::getPastTotalSupply)
                    }
                    getPastTotalSupply
                },
                {
                    fn clock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <clockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::clock)
                    }
                    clock
                },
                {
                    fn symbol(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <symbolCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::symbol)
                    }
                    symbol
                },
                {
                    fn getVotes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <getVotesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::getVotes)
                    }
                    getVotes
                },
                {
                    fn faucetAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <faucetAmountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::faucetAmount)
                    }
                    faucetAmount
                },
                {
                    fn burn(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <burnCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::burn)
                    }
                    burn
                },
                {
                    fn transfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <transferCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::transfer)
                    }
                    transfer
                },
                {
                    fn lastMint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <lastMintCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::lastMint)
                    }
                    lastMint
                },
                {
                    fn delegateBySig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <delegateBySigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::delegateBySig)
                    }
                    delegateBySig
                },
                {
                    fn permit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <permitCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::permit)
                    }
                    permit
                },
                {
                    fn allowance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <allowanceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::allowance)
                    }
                    allowance
                },
                {
                    fn checkpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <checkpointsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::checkpoints)
                    }
                    checkpoints
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenCalls::transferOwnership)
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
                Self::CLOCK_MODE(inner) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::DOMAIN_SEPARATOR(inner) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FAUCET_INTERVAL(inner) => {
                    <FAUCET_INTERVALCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allowance(inner) => {
                    <allowanceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::approve(inner) => {
                    <approveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::burn(inner) => {
                    <burnCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::checkpoints(inner) => {
                    <checkpointsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::clock(inner) => {
                    <clockCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::decimals(inner) => {
                    <decimalsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delegate(inner) => {
                    <delegateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delegateBySig(inner) => {
                    <delegateBySigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegates(inner) => {
                    <delegatesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::faucetAmount(inner) => {
                    <faucetAmountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPastTotalSupply(inner) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPastVotes(inner) => {
                    <getPastVotesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getVotes(inner) => {
                    <getVotesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::lastMint(inner) => {
                    <lastMintCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::mint(inner) => {
                    <mintCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::name(inner) => {
                    <nameCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nonces(inner) => {
                    <noncesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::numCheckpoints(inner) => {
                    <numCheckpointsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::permit(inner) => {
                    <permitCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setFaucetAmount(inner) => {
                    <setFaucetAmountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::symbol(inner) => {
                    <symbolCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::totalSupply(inner) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transferFrom(inner) => {
                    <transferFromCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::CLOCK_MODE(inner) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DOMAIN_SEPARATOR(inner) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FAUCET_INTERVAL(inner) => {
                    <FAUCET_INTERVALCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allowance(inner) => {
                    <allowanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::approve(inner) => {
                    <approveCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::burn(inner) => {
                    <burnCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::checkpoints(inner) => {
                    <checkpointsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::clock(inner) => {
                    <clockCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::decimals(inner) => {
                    <decimalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegate(inner) => {
                    <delegateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegateBySig(inner) => {
                    <delegateBySigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegates(inner) => {
                    <delegatesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::faucetAmount(inner) => {
                    <faucetAmountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPastTotalSupply(inner) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPastVotes(inner) => {
                    <getPastVotesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getVotes(inner) => {
                    <getVotesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastMint(inner) => {
                    <lastMintCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mint(inner) => {
                    <mintCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::name(inner) => {
                    <nameCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::nonces(inner) => {
                    <noncesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::numCheckpoints(inner) => {
                    <numCheckpointsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::permit(inner) => {
                    <permitCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setFaucetAmount(inner) => {
                    <setFaucetAmountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::symbol(inner) => {
                    <symbolCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::totalSupply(inner) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferFrom(inner) => {
                    <transferFromCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`ERC20FaucetToken`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ERC20FaucetTokenErrors {
        #[allow(missing_docs)]
        CheckpointUnorderedInsertion(CheckpointUnorderedInsertion),
        #[allow(missing_docs)]
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        #[allow(missing_docs)]
        ERC20ExceededSafeSupply(ERC20ExceededSafeSupply),
        #[allow(missing_docs)]
        ERC20InsufficientAllowance(ERC20InsufficientAllowance),
        #[allow(missing_docs)]
        ERC20InsufficientBalance(ERC20InsufficientBalance),
        #[allow(missing_docs)]
        ERC20InvalidApprover(ERC20InvalidApprover),
        #[allow(missing_docs)]
        ERC20InvalidReceiver(ERC20InvalidReceiver),
        #[allow(missing_docs)]
        ERC20InvalidSender(ERC20InvalidSender),
        #[allow(missing_docs)]
        ERC20InvalidSpender(ERC20InvalidSpender),
        #[allow(missing_docs)]
        ERC2612ExpiredSignature(ERC2612ExpiredSignature),
        #[allow(missing_docs)]
        ERC2612InvalidSigner(ERC2612InvalidSigner),
        #[allow(missing_docs)]
        ERC5805FutureLookup(ERC5805FutureLookup),
        #[allow(missing_docs)]
        ERC6372InconsistentClock(ERC6372InconsistentClock),
        #[allow(missing_docs)]
        InvalidAccountNonce(InvalidAccountNonce),
        #[allow(missing_docs)]
        InvalidShortString(InvalidShortString),
        #[allow(missing_docs)]
        OwnableInvalidOwner(OwnableInvalidOwner),
        #[allow(missing_docs)]
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        #[allow(missing_docs)]
        SafeCastOverflowedUintDowncast(SafeCastOverflowedUintDowncast),
        #[allow(missing_docs)]
        StringTooLong(StringTooLong),
        #[allow(missing_docs)]
        VotesExpiredSignature(VotesExpiredSignature),
    }
    impl ERC20FaucetTokenErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [17u8, 140u8, 218u8, 167u8],
            [28u8, 177u8, 93u8, 38u8],
            [30u8, 79u8, 189u8, 247u8],
            [37u8, 32u8, 96u8, 29u8],
            [48u8, 90u8, 39u8, 169u8],
            [70u8, 131u8, 175u8, 14u8],
            [75u8, 128u8, 14u8, 70u8],
            [98u8, 121u8, 19u8, 2u8],
            [109u8, 252u8, 198u8, 80u8],
            [111u8, 240u8, 113u8, 64u8],
            [117u8, 45u8, 136u8, 192u8],
            [148u8, 40u8, 13u8, 98u8],
            [150u8, 198u8, 253u8, 30u8],
            [179u8, 81u8, 43u8, 12u8],
            [215u8, 139u8, 206u8, 12u8],
            [228u8, 80u8, 211u8, 140u8],
            [230u8, 2u8, 223u8, 5u8],
            [236u8, 68u8, 47u8, 5u8],
            [236u8, 211u8, 248u8, 30u8],
            [246u8, 69u8, 238u8, 223u8],
            [251u8, 143u8, 65u8, 178u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(OwnableUnauthorizedAccount),
            ::core::stringify!(ERC20ExceededSafeSupply),
            ::core::stringify!(OwnableInvalidOwner),
            ::core::stringify!(CheckpointUnorderedInsertion),
            ::core::stringify!(StringTooLong),
            ::core::stringify!(VotesExpiredSignature),
            ::core::stringify!(ERC2612InvalidSigner),
            ::core::stringify!(ERC2612ExpiredSignature),
            ::core::stringify!(SafeCastOverflowedUintDowncast),
            ::core::stringify!(ERC6372InconsistentClock),
            ::core::stringify!(InvalidAccountNonce),
            ::core::stringify!(ERC20InvalidSpender),
            ::core::stringify!(ERC20InvalidSender),
            ::core::stringify!(InvalidShortString),
            ::core::stringify!(ECDSAInvalidSignatureS),
            ::core::stringify!(ERC20InsufficientBalance),
            ::core::stringify!(ERC20InvalidApprover),
            ::core::stringify!(ERC20InvalidReceiver),
            ::core::stringify!(ERC5805FutureLookup),
            ::core::stringify!(ECDSAInvalidSignature),
            ::core::stringify!(ERC20InsufficientAllowance),
            ::core::stringify!(ECDSAInvalidSignatureLength),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SIGNATURE,
            <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableInvalidOwner as alloy_sol_types::SolError>::SIGNATURE,
            <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::SIGNATURE,
            <StringTooLong as alloy_sol_types::SolError>::SIGNATURE,
            <VotesExpiredSignature as alloy_sol_types::SolError>::SIGNATURE,
            <ERC2612InvalidSigner as alloy_sol_types::SolError>::SIGNATURE,
            <ERC2612ExpiredSignature as alloy_sol_types::SolError>::SIGNATURE,
            <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::SIGNATURE,
            <ERC6372InconsistentClock as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidAccountNonce as alloy_sol_types::SolError>::SIGNATURE,
            <ERC20InvalidSpender as alloy_sol_types::SolError>::SIGNATURE,
            <ERC20InvalidSender as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidShortString as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
            <ERC20InsufficientBalance as alloy_sol_types::SolError>::SIGNATURE,
            <ERC20InvalidApprover as alloy_sol_types::SolError>::SIGNATURE,
            <ERC20InvalidReceiver as alloy_sol_types::SolError>::SIGNATURE,
            <ERC5805FutureLookup as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignature as alloy_sol_types::SolError>::SIGNATURE,
            <ERC20InsufficientAllowance as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for ERC20FaucetTokenErrors {
        const NAME: &'static str = "ERC20FaucetTokenErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 22usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CheckpointUnorderedInsertion(_) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignature(_) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureLength(_) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureS(_) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20ExceededSafeSupply(_) => {
                    <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InsufficientAllowance(_) => {
                    <ERC20InsufficientAllowance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InsufficientBalance(_) => {
                    <ERC20InsufficientBalance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InvalidApprover(_) => {
                    <ERC20InvalidApprover as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InvalidReceiver(_) => {
                    <ERC20InvalidReceiver as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InvalidSender(_) => {
                    <ERC20InvalidSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InvalidSpender(_) => {
                    <ERC20InvalidSpender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC2612ExpiredSignature(_) => {
                    <ERC2612ExpiredSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC2612InvalidSigner(_) => {
                    <ERC2612InvalidSigner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC5805FutureLookup(_) => {
                    <ERC5805FutureLookup as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC6372InconsistentClock(_) => {
                    <ERC6372InconsistentClock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidAccountNonce(_) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableInvalidOwner(_) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableUnauthorizedAccount(_) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SafeCastOverflowedUintDowncast(_) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StringTooLong(_) => {
                    <StringTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::VotesExpiredSignature(_) => {
                    <VotesExpiredSignature as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors>] = &[
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn ERC20ExceededSafeSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC20ExceededSafeSupply)
                    }
                    ERC20ExceededSafeSupply
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn CheckpointUnorderedInsertion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::CheckpointUnorderedInsertion)
                    }
                    CheckpointUnorderedInsertion
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn VotesExpiredSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <VotesExpiredSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::VotesExpiredSignature)
                    }
                    VotesExpiredSignature
                },
                {
                    fn ERC2612InvalidSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC2612InvalidSigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC2612InvalidSigner)
                    }
                    ERC2612InvalidSigner
                },
                {
                    fn ERC2612ExpiredSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC2612ExpiredSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC2612ExpiredSignature)
                    }
                    ERC2612ExpiredSignature
                },
                {
                    fn SafeCastOverflowedUintDowncast(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::SafeCastOverflowedUintDowncast)
                    }
                    SafeCastOverflowedUintDowncast
                },
                {
                    fn ERC6372InconsistentClock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC6372InconsistentClock)
                    }
                    ERC6372InconsistentClock
                },
                {
                    fn InvalidAccountNonce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <InvalidAccountNonce as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::InvalidAccountNonce)
                    }
                    InvalidAccountNonce
                },
                {
                    fn ERC20InvalidSpender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC20InvalidSpender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC20InvalidSpender)
                    }
                    ERC20InvalidSpender
                },
                {
                    fn ERC20InvalidSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC20InvalidSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC20InvalidSender)
                    }
                    ERC20InvalidSender
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn ERC20InsufficientBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC20InsufficientBalance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC20InsufficientBalance)
                    }
                    ERC20InsufficientBalance
                },
                {
                    fn ERC20InvalidApprover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC20InvalidApprover as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC20InvalidApprover)
                    }
                    ERC20InvalidApprover
                },
                {
                    fn ERC20InvalidReceiver(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC20InvalidReceiver as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC20InvalidReceiver)
                    }
                    ERC20InvalidReceiver
                },
                {
                    fn ERC5805FutureLookup(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC5805FutureLookup)
                    }
                    ERC5805FutureLookup
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn ERC20InsufficientAllowance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC20InsufficientAllowance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC20InsufficientAllowance)
                    }
                    ERC20InsufficientAllowance
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
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
            ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors>] = &[
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn ERC20ExceededSafeSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC20ExceededSafeSupply)
                    }
                    ERC20ExceededSafeSupply
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn CheckpointUnorderedInsertion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::CheckpointUnorderedInsertion)
                    }
                    CheckpointUnorderedInsertion
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn VotesExpiredSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <VotesExpiredSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::VotesExpiredSignature)
                    }
                    VotesExpiredSignature
                },
                {
                    fn ERC2612InvalidSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC2612InvalidSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC2612InvalidSigner)
                    }
                    ERC2612InvalidSigner
                },
                {
                    fn ERC2612ExpiredSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC2612ExpiredSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC2612ExpiredSignature)
                    }
                    ERC2612ExpiredSignature
                },
                {
                    fn SafeCastOverflowedUintDowncast(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::SafeCastOverflowedUintDowncast)
                    }
                    SafeCastOverflowedUintDowncast
                },
                {
                    fn ERC6372InconsistentClock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC6372InconsistentClock)
                    }
                    ERC6372InconsistentClock
                },
                {
                    fn InvalidAccountNonce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <InvalidAccountNonce as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::InvalidAccountNonce)
                    }
                    InvalidAccountNonce
                },
                {
                    fn ERC20InvalidSpender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC20InvalidSpender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC20InvalidSpender)
                    }
                    ERC20InvalidSpender
                },
                {
                    fn ERC20InvalidSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC20InvalidSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC20InvalidSender)
                    }
                    ERC20InvalidSender
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn ERC20InsufficientBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC20InsufficientBalance as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC20InsufficientBalance)
                    }
                    ERC20InsufficientBalance
                },
                {
                    fn ERC20InvalidApprover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC20InvalidApprover as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC20InvalidApprover)
                    }
                    ERC20InvalidApprover
                },
                {
                    fn ERC20InvalidReceiver(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC20InvalidReceiver as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC20InvalidReceiver)
                    }
                    ERC20InvalidReceiver
                },
                {
                    fn ERC5805FutureLookup(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC5805FutureLookup)
                    }
                    ERC5805FutureLookup
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn ERC20InsufficientAllowance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ERC20InsufficientAllowance as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ERC20InsufficientAllowance)
                    }
                    ERC20InsufficientAllowance
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ERC20FaucetTokenErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ERC20FaucetTokenErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
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
                Self::CheckpointUnorderedInsertion(inner) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20ExceededSafeSupply(inner) => {
                    <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InsufficientAllowance(inner) => {
                    <ERC20InsufficientAllowance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InsufficientBalance(inner) => {
                    <ERC20InsufficientBalance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InvalidApprover(inner) => {
                    <ERC20InvalidApprover as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InvalidReceiver(inner) => {
                    <ERC20InvalidReceiver as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InvalidSender(inner) => {
                    <ERC20InvalidSender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InvalidSpender(inner) => {
                    <ERC20InvalidSpender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC2612ExpiredSignature(inner) => {
                    <ERC2612ExpiredSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC2612InvalidSigner(inner) => {
                    <ERC2612InvalidSigner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC5805FutureLookup(inner) => {
                    <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC6372InconsistentClock(inner) => {
                    <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidAccountNonce(inner) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SafeCastOverflowedUintDowncast(inner) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::VotesExpiredSignature(inner) => {
                    <VotesExpiredSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::CheckpointUnorderedInsertion(inner) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20ExceededSafeSupply(inner) => {
                    <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InsufficientAllowance(inner) => {
                    <ERC20InsufficientAllowance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InsufficientBalance(inner) => {
                    <ERC20InsufficientBalance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InvalidApprover(inner) => {
                    <ERC20InvalidApprover as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InvalidReceiver(inner) => {
                    <ERC20InvalidReceiver as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InvalidSender(inner) => {
                    <ERC20InvalidSender as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InvalidSpender(inner) => {
                    <ERC20InvalidSpender as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC2612ExpiredSignature(inner) => {
                    <ERC2612ExpiredSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC2612InvalidSigner(inner) => {
                    <ERC2612InvalidSigner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC5805FutureLookup(inner) => {
                    <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC6372InconsistentClock(inner) => {
                    <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidAccountNonce(inner) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SafeCastOverflowedUintDowncast(inner) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::VotesExpiredSignature(inner) => {
                    <VotesExpiredSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ERC20FaucetToken`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ERC20FaucetTokenEvents {
        #[allow(missing_docs)]
        Approval(Approval),
        #[allow(missing_docs)]
        DelegateChanged(DelegateChanged),
        #[allow(missing_docs)]
        DelegateVotesChanged(DelegateVotesChanged),
        #[allow(missing_docs)]
        EIP712DomainChanged(EIP712DomainChanged),
        #[allow(missing_docs)]
        FaucetAmountSet(FaucetAmountSet),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Transfer(Transfer),
    }
    impl ERC20FaucetTokenEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                10u8, 99u8, 135u8, 201u8, 234u8, 54u8, 40u8, 184u8, 138u8, 99u8, 59u8,
                180u8, 243u8, 177u8, 81u8, 119u8, 15u8, 112u8, 8u8, 81u8, 23u8, 161u8,
                95u8, 155u8, 243u8, 120u8, 124u8, 218u8, 83u8, 241u8, 61u8, 49u8,
            ],
            [
                49u8, 52u8, 232u8, 162u8, 230u8, 217u8, 126u8, 146u8, 154u8, 126u8, 84u8,
                1u8, 30u8, 165u8, 72u8, 93u8, 125u8, 25u8, 109u8, 213u8, 240u8, 186u8,
                77u8, 78u8, 249u8, 88u8, 3u8, 232u8, 227u8, 252u8, 37u8, 127u8,
            ],
            [
                107u8, 72u8, 94u8, 39u8, 64u8, 87u8, 58u8, 87u8, 175u8, 31u8, 233u8,
                172u8, 211u8, 249u8, 163u8, 62u8, 164u8, 178u8, 107u8, 140u8, 216u8,
                140u8, 109u8, 45u8, 170u8, 150u8, 112u8, 54u8, 209u8, 177u8, 195u8, 141u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                140u8, 91u8, 225u8, 229u8, 235u8, 236u8, 125u8, 91u8, 209u8, 79u8, 113u8,
                66u8, 125u8, 30u8, 132u8, 243u8, 221u8, 3u8, 20u8, 192u8, 247u8, 178u8,
                41u8, 30u8, 91u8, 32u8, 10u8, 200u8, 199u8, 195u8, 185u8, 37u8,
            ],
            [
                221u8, 242u8, 82u8, 173u8, 27u8, 226u8, 200u8, 155u8, 105u8, 194u8,
                176u8, 104u8, 252u8, 55u8, 141u8, 170u8, 149u8, 43u8, 167u8, 241u8, 99u8,
                196u8, 161u8, 22u8, 40u8, 245u8, 90u8, 77u8, 245u8, 35u8, 179u8, 239u8,
            ],
            [
                222u8, 194u8, 186u8, 205u8, 210u8, 240u8, 91u8, 89u8, 222u8, 52u8, 218u8,
                155u8, 82u8, 61u8, 255u8, 139u8, 228u8, 46u8, 94u8, 56u8, 232u8, 24u8,
                200u8, 47u8, 219u8, 11u8, 174u8, 119u8, 67u8, 135u8, 167u8, 36u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(EIP712DomainChanged),
            ::core::stringify!(DelegateChanged),
            ::core::stringify!(FaucetAmountSet),
            ::core::stringify!(OwnershipTransferred),
            ::core::stringify!(Approval),
            ::core::stringify!(Transfer),
            ::core::stringify!(DelegateVotesChanged),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE,
            <DelegateChanged as alloy_sol_types::SolEvent>::SIGNATURE,
            <FaucetAmountSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <Approval as alloy_sol_types::SolEvent>::SIGNATURE,
            <Transfer as alloy_sol_types::SolEvent>::SIGNATURE,
            <DelegateVotesChanged as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for ERC20FaucetTokenEvents {
        const NAME: &'static str = "ERC20FaucetTokenEvents";
        const COUNT: usize = 7usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Approval as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Approval as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Approval)
                }
                Some(<DelegateChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DelegateChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DelegateChanged)
                }
                Some(
                    <DelegateVotesChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DelegateVotesChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DelegateVotesChanged)
                }
                Some(
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EIP712DomainChanged)
                }
                Some(<FaucetAmountSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <FaucetAmountSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::FaucetAmountSet)
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
                Some(<Transfer as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Transfer as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Transfer)
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
    impl alloy_sol_types::private::IntoLogData for ERC20FaucetTokenEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DelegateChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DelegateVotesChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::FaucetAmountSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DelegateChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DelegateVotesChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::FaucetAmountSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ERC20FaucetToken`](self) contract instance.

See the [wrapper's documentation](`ERC20FaucetTokenInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> ERC20FaucetTokenInstance<P, N> {
        ERC20FaucetTokenInstance::<P, N>::new(address, __provider)
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
        name: alloy::sol_types::private::String,
        symbol: alloy::sol_types::private::String,
        decimals_: u8,
        faucetAmount_: alloy::sol_types::private::primitives::aliases::U256,
        _owner: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<ERC20FaucetTokenInstance<P, N>>,
    > {
        ERC20FaucetTokenInstance::<
            P,
            N,
        >::deploy(__provider, name, symbol, decimals_, faucetAmount_, _owner)
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
        name: alloy::sol_types::private::String,
        symbol: alloy::sol_types::private::String,
        decimals_: u8,
        faucetAmount_: alloy::sol_types::private::primitives::aliases::U256,
        _owner: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        ERC20FaucetTokenInstance::<
            P,
            N,
        >::deploy_builder(__provider, name, symbol, decimals_, faucetAmount_, _owner)
    }
    /**A [`ERC20FaucetToken`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ERC20FaucetToken`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ERC20FaucetTokenInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ERC20FaucetTokenInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ERC20FaucetTokenInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ERC20FaucetTokenInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ERC20FaucetToken`](self) contract instance.

See the [wrapper's documentation](`ERC20FaucetTokenInstance`) for more details.*/
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
            name: alloy::sol_types::private::String,
            symbol: alloy::sol_types::private::String,
            decimals_: u8,
            faucetAmount_: alloy::sol_types::private::primitives::aliases::U256,
            _owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<ERC20FaucetTokenInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                name,
                symbol,
                decimals_,
                faucetAmount_,
                _owner,
            );
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
            name: alloy::sol_types::private::String,
            symbol: alloy::sol_types::private::String,
            decimals_: u8,
            faucetAmount_: alloy::sol_types::private::primitives::aliases::U256,
            _owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            name,
                            symbol,
                            decimals_,
                            faucetAmount_,
                            _owner,
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
    impl<P: ::core::clone::Clone, N> ERC20FaucetTokenInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ERC20FaucetTokenInstance<P, N> {
            ERC20FaucetTokenInstance {
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
    > ERC20FaucetTokenInstance<P, N> {
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
        ///Creates a new call builder for the [`CLOCK_MODE`] function.
        pub fn CLOCK_MODE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, CLOCK_MODECall, N> {
            self.call_builder(&CLOCK_MODECall)
        }
        ///Creates a new call builder for the [`DOMAIN_SEPARATOR`] function.
        pub fn DOMAIN_SEPARATOR(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, DOMAIN_SEPARATORCall, N> {
            self.call_builder(&DOMAIN_SEPARATORCall)
        }
        ///Creates a new call builder for the [`FAUCET_INTERVAL`] function.
        pub fn FAUCET_INTERVAL(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, FAUCET_INTERVALCall, N> {
            self.call_builder(&FAUCET_INTERVALCall)
        }
        ///Creates a new call builder for the [`allowance`] function.
        pub fn allowance(
            &self,
            owner: alloy::sol_types::private::Address,
            spender: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, allowanceCall, N> {
            self.call_builder(&allowanceCall { owner, spender })
        }
        ///Creates a new call builder for the [`approve`] function.
        pub fn approve(
            &self,
            spender: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, approveCall, N> {
            self.call_builder(&approveCall { spender, value })
        }
        ///Creates a new call builder for the [`balanceOf`] function.
        pub fn balanceOf(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, balanceOfCall, N> {
            self.call_builder(&balanceOfCall { account })
        }
        ///Creates a new call builder for the [`burn`] function.
        pub fn burn(
            &self,
            account: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, burnCall, N> {
            self.call_builder(&burnCall { account, amount })
        }
        ///Creates a new call builder for the [`checkpoints`] function.
        pub fn checkpoints(
            &self,
            account: alloy::sol_types::private::Address,
            pos: u32,
        ) -> alloy_contract::SolCallBuilder<&P, checkpointsCall, N> {
            self.call_builder(&checkpointsCall { account, pos })
        }
        ///Creates a new call builder for the [`clock`] function.
        pub fn clock(&self) -> alloy_contract::SolCallBuilder<&P, clockCall, N> {
            self.call_builder(&clockCall)
        }
        ///Creates a new call builder for the [`decimals`] function.
        pub fn decimals(&self) -> alloy_contract::SolCallBuilder<&P, decimalsCall, N> {
            self.call_builder(&decimalsCall)
        }
        ///Creates a new call builder for the [`delegate`] function.
        pub fn delegate(
            &self,
            delegatee: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, delegateCall, N> {
            self.call_builder(&delegateCall { delegatee })
        }
        ///Creates a new call builder for the [`delegateBySig`] function.
        pub fn delegateBySig(
            &self,
            delegatee: alloy::sol_types::private::Address,
            nonce: alloy::sol_types::private::primitives::aliases::U256,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
            v: u8,
            r: alloy::sol_types::private::FixedBytes<32>,
            s: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, delegateBySigCall, N> {
            self.call_builder(
                &delegateBySigCall {
                    delegatee,
                    nonce,
                    expiry,
                    v,
                    r,
                    s,
                },
            )
        }
        ///Creates a new call builder for the [`delegates`] function.
        pub fn delegates(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, delegatesCall, N> {
            self.call_builder(&delegatesCall { account })
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall)
        }
        ///Creates a new call builder for the [`faucetAmount`] function.
        pub fn faucetAmount(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, faucetAmountCall, N> {
            self.call_builder(&faucetAmountCall)
        }
        ///Creates a new call builder for the [`getPastTotalSupply`] function.
        pub fn getPastTotalSupply(
            &self,
            timepoint: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getPastTotalSupplyCall, N> {
            self.call_builder(
                &getPastTotalSupplyCall {
                    timepoint,
                },
            )
        }
        ///Creates a new call builder for the [`getPastVotes`] function.
        pub fn getPastVotes(
            &self,
            account: alloy::sol_types::private::Address,
            timepoint: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getPastVotesCall, N> {
            self.call_builder(
                &getPastVotesCall {
                    account,
                    timepoint,
                },
            )
        }
        ///Creates a new call builder for the [`getVotes`] function.
        pub fn getVotes(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, getVotesCall, N> {
            self.call_builder(&getVotesCall { account })
        }
        ///Creates a new call builder for the [`lastMint`] function.
        pub fn lastMint(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, lastMintCall, N> {
            self.call_builder(&lastMintCall(_0))
        }
        ///Creates a new call builder for the [`mint`] function.
        pub fn mint(&self) -> alloy_contract::SolCallBuilder<&P, mintCall, N> {
            self.call_builder(&mintCall)
        }
        ///Creates a new call builder for the [`name`] function.
        pub fn name(&self) -> alloy_contract::SolCallBuilder<&P, nameCall, N> {
            self.call_builder(&nameCall)
        }
        ///Creates a new call builder for the [`nonces`] function.
        pub fn nonces(
            &self,
            owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, noncesCall, N> {
            self.call_builder(&noncesCall { owner })
        }
        ///Creates a new call builder for the [`numCheckpoints`] function.
        pub fn numCheckpoints(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, numCheckpointsCall, N> {
            self.call_builder(&numCheckpointsCall { account })
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
        }
        ///Creates a new call builder for the [`permit`] function.
        pub fn permit(
            &self,
            owner: alloy::sol_types::private::Address,
            spender: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
            v: u8,
            r: alloy::sol_types::private::FixedBytes<32>,
            s: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, permitCall, N> {
            self.call_builder(
                &permitCall {
                    owner,
                    spender,
                    value,
                    deadline,
                    v,
                    r,
                    s,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall)
        }
        ///Creates a new call builder for the [`setFaucetAmount`] function.
        pub fn setFaucetAmount(
            &self,
            faucetAmount_: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, setFaucetAmountCall, N> {
            self.call_builder(
                &setFaucetAmountCall {
                    faucetAmount_,
                },
            )
        }
        ///Creates a new call builder for the [`symbol`] function.
        pub fn symbol(&self) -> alloy_contract::SolCallBuilder<&P, symbolCall, N> {
            self.call_builder(&symbolCall)
        }
        ///Creates a new call builder for the [`totalSupply`] function.
        pub fn totalSupply(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, totalSupplyCall, N> {
            self.call_builder(&totalSupplyCall)
        }
        ///Creates a new call builder for the [`transfer`] function.
        pub fn transfer(
            &self,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, transferCall, N> {
            self.call_builder(&transferCall { to, value })
        }
        ///Creates a new call builder for the [`transferFrom`] function.
        pub fn transferFrom(
            &self,
            from: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, transferFromCall, N> {
            self.call_builder(
                &transferFromCall {
                    from,
                    to,
                    value,
                },
            )
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ERC20FaucetTokenInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Approval`] event.
        pub fn Approval_filter(&self) -> alloy_contract::Event<&P, Approval, N> {
            self.event_filter::<Approval>()
        }
        ///Creates a new event filter for the [`DelegateChanged`] event.
        pub fn DelegateChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, DelegateChanged, N> {
            self.event_filter::<DelegateChanged>()
        }
        ///Creates a new event filter for the [`DelegateVotesChanged`] event.
        pub fn DelegateVotesChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, DelegateVotesChanged, N> {
            self.event_filter::<DelegateVotesChanged>()
        }
        ///Creates a new event filter for the [`EIP712DomainChanged`] event.
        pub fn EIP712DomainChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, EIP712DomainChanged, N> {
            self.event_filter::<EIP712DomainChanged>()
        }
        ///Creates a new event filter for the [`FaucetAmountSet`] event.
        pub fn FaucetAmountSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, FaucetAmountSet, N> {
            self.event_filter::<FaucetAmountSet>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`Transfer`] event.
        pub fn Transfer_filter(&self) -> alloy_contract::Event<&P, Transfer, N> {
            self.event_filter::<Transfer>()
        }
    }
}
