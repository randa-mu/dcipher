//! Helper structures to manage retries.

use crate::fulfiller::Identifier;

#[derive(Clone, Debug)]
pub(super) struct RetryableRequest<R> {
    pub req: R,
    pub retry_strategy: RetryStrategyTypes,
}

impl<R> RetryableRequest<R> {
    pub(super) fn new(req: R, retry_strategy: RetryStrategy) -> RetryableRequest<R> {
        RetryableRequest {
            req,
            retry_strategy: retry_strategy.into(),
        }
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum RetryStrategy {
    Never,
    Times(usize),
}

#[derive(Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub(super) enum RetryStrategyTypes {
    Never(RetryStrategyNever),
    Times(RetryStrategyTimes),
}

pub(super) trait RequestRetryStrategy {
    fn should_retry_and_update(&mut self) -> bool;
}

impl From<RetryStrategy> for RetryStrategyTypes {
    fn from(value: RetryStrategy) -> Self {
        match value {
            RetryStrategy::Never => RetryStrategyTypes::Never(RetryStrategyNever),
            RetryStrategy::Times(retries) => RetryStrategyTypes::Times(RetryStrategyTimes(retries)),
        }
    }
}

impl RequestRetryStrategy for RetryStrategyTypes {
    fn should_retry_and_update(&mut self) -> bool {
        match self {
            RetryStrategyTypes::Never(rs) => rs.should_retry_and_update(),
            RetryStrategyTypes::Times(rs) => rs.should_retry_and_update(),
        }
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub(super) struct RetryStrategyNever;
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub(super) struct RetryStrategyTimes(usize);

impl RequestRetryStrategy for RetryStrategyNever {
    fn should_retry_and_update(&mut self) -> bool {
        false
    }
}

impl RequestRetryStrategy for RetryStrategyTimes {
    fn should_retry_and_update(&mut self) -> bool {
        if self.0 > 0 {
            self.0 -= 1;
            true
        } else {
            false
        }
    }
}

impl<R: Identifier> PartialOrd for RetryableRequest<R> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.req.id().cmp(other.req.id()))
    }
}

impl<R: Identifier> PartialEq for RetryableRequest<R> {
    fn eq(&self, other: &Self) -> bool {
        self.req.id().eq(other.req.id())
    }
}

impl<R: Identifier> Ord for RetryableRequest<R> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.req.id().cmp(other.req.id())
    }
}

impl<R: Identifier> Eq for RetryableRequest<R> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_strategy_none() {
        let mut never: RetryStrategyTypes = RetryStrategy::Never.into();
        assert!(!never.should_retry_and_update());
    }

    #[test]
    fn retry_strategy_times() {
        let mut retries_0: RetryStrategyTypes = RetryStrategy::Times(0).into();
        assert!(!retries_0.should_retry_and_update());

        let mut retries_1: RetryStrategyTypes = RetryStrategy::Times(1).into();
        assert!(retries_1.should_retry_and_update());
        assert!(!retries_1.should_retry_and_update());

        let mut retries_3: RetryStrategyTypes = RetryStrategy::Times(3).into();
        assert!(retries_3.should_retry_and_update());
        assert!(retries_3.should_retry_and_update());
        assert!(retries_3.should_retry_and_update());
        assert!(!retries_3.should_retry_and_update());
    }
}
