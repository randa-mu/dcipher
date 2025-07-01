//! The event handler receives decoded events, stores them in a database,
//! and forwards the events to a broadcast stream.

pub mod db;
pub(crate) mod listener;

#[cfg(test)]
mod tests {
    pub(crate) mod test_contracts {
        use crate::event_manager::tests::test_contracts::EventEmitter::EventEmitterInstance;
        use crate::proto_types::BlockSafety;
        use crate::types::{EventStreamId, ParsedEventField, RegisteredEvent};
        use alloy::dyn_abi::DynSolType;
        use alloy::network::Network;
        use alloy::providers::Provider;

        alloy::sol! {
            #[sol(rpc, bytecode = "6080604052348015600e575f5ffd5b506101ec8061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610029575f3560e01c80632536f1271461002d575b5f5ffd5b610047600480360381019061004291906100ef565b610049565b005b7f500918a1acf84fe22df8e73c039449df2f37619cf220d2a4d382cddec5e088e1828260405161007a929190610194565b60405180910390a15050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f8401126100af576100ae61008e565b5b8235905067ffffffffffffffff8111156100cc576100cb610092565b5b6020830191508360018202830111156100e8576100e7610096565b5b9250929050565b5f5f6020838503121561010557610104610086565b5b5f83013567ffffffffffffffff8111156101225761012161008a565b5b61012e8582860161009a565b92509250509250929050565b5f82825260208201905092915050565b828183375f83830152505050565b5f601f19601f8301169050919050565b5f610173838561013a565b935061018083858461014a565b61018983610158565b840190509392505050565b5f6020820190508181035f8301526101ad818486610168565b9050939250505056fea264697066735822122009bdd348f95f2e120079efff8b79e472ad4d569739061e9d398a4d57a765dd5d64736f6c634300081e0033")]
            contract EventEmitter {
                event StringEmitted(string value);
                function emitString(string calldata _value) external {
                    emit StringEmitted(_value);
                }
            }
        }

        pub(crate) async fn deploy_event_emitter<P, N>(provider: P) -> EventEmitterInstance<P, N>
        where
            P: Provider<N>,
            N: Network,
        {
            EventEmitterInstance::deploy(provider).await.unwrap()
        }

        pub(crate) async fn get_string_registered_event<P, N>(
            instance: &EventEmitterInstance<P, N>,
        ) -> RegisteredEvent
        where
            P: Provider<N>,
            N: Network,
        {
            RegisteredEvent::try_new(
                EventStreamId::new(b"EventEmitterInstance::StringEmitted"),
                instance.provider().get_chain_id().await.unwrap(),
                *instance.address(),
                "StringEmitted".to_owned(),
                vec![ParsedEventField::new(DynSolType::String, false)],
                BlockSafety::Latest,
            )
            .unwrap()
        }
    }
}
