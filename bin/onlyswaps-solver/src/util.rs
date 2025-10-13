use alloy::primitives::U256;

pub fn normalise_chain_id(chain_id: U256) -> u64 {
    chain_id.as_limbs()[0]
}

#[cfg(test)]
pub(crate) mod test {
    use crate::model::RequestId;
    use alloy::primitives::Address;
    use std::fs::File;
    use std::io::Read;

    pub fn generate_request_id() -> RequestId {
        let mut buf = [0u8; 32];
        File::open("/dev/urandom")
            .and_then(|mut f| f.read_exact(&mut buf))
            .expect("failed to read random bytes");
        buf.into()
    }

    pub fn generate_address() -> Address {
        let mut buf = [0u8; 20];
        File::open("/dev/urandom")
            .and_then(|mut f| f.read_exact(&mut buf))
            .expect("failed to read random bytes");
        buf.into()
    }
}
