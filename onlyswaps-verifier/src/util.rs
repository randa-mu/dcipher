use alloy::primitives::U256;

pub fn normalise_chain_id(chain_id: U256) -> u64 {
    chain_id.as_limbs()[0]
}
