use omnievent::proto_types::BlockSafety;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeoutConfig {
    #[serde(default = "default_block_safety")]
    pub block_safety: BlockSafety,
    #[serde(with = "humantime_serde", default = "default_request_timeout")]
    pub request_timeout: Duration,
    #[serde(with = "humantime_serde", default = "default_retry_duration")]
    pub retry_duration: Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        TimeoutConfig {
            block_safety: default_block_safety(),
            request_timeout: default_request_timeout(),
            retry_duration: default_retry_duration(),
        }
    }
}

const fn default_block_safety() -> BlockSafety {
    BlockSafety::Safe
}
const fn default_request_timeout() -> Duration {
    Duration::from_secs(30)
}

const fn default_retry_duration() -> Duration {
    Duration::from_secs(12)
}
