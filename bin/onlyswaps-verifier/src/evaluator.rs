use crate::chain_state_resolver::ChainState;
use alloy::primitives::U256;
use anyhow::anyhow;
use chrono::{DateTime, TimeDelta, Utc};
use config::network::NetworkConfig;
use std::collections::HashMap;
use std::ops::Add;

type ChainId = U256;
#[derive(Clone)]
pub struct Evaluator {
    finality_durations: HashMap<ChainId, TimeDelta>,
}
impl Evaluator {
    pub fn new(finalisation_durations: &[NetworkConfig]) -> anyhow::Result<Self> {
        let mut finality_durations = HashMap::new();

        for config in finalisation_durations {
            finality_durations.insert(
                U256::from(config.chain_id),
                TimeDelta::from_std(config.finality_duration_secs)?,
            );
        }

        Ok(Self { finality_durations })
    }
    pub fn evaluate(&self, chain_state: ChainState) -> anyhow::Result<ChainState> {
        let ChainState {
            swap_params,
            transfer_receipt,
        } = &chain_state;
        if transfer_receipt.dstChainId != swap_params.dstChainId {
            anyhow::bail!("funds were sent on the wrong chain")
        }

        // this shouldn't be possible
        if transfer_receipt.srcChainId != swap_params.srcChainId {
            anyhow::bail!("funds were sent from the wrong chain")
        }

        if transfer_receipt.recipient != swap_params.recipient {
            anyhow::bail!("funds were sent to the wrong recipient")
        }

        if transfer_receipt.tokenOut != swap_params.tokenOut {
            anyhow::bail!("funds were sent from the wrong token")
        }

        // the contract manages the fee calculation, so we just used the `amountOut` to figure
        // out how much should be moved on the destination chain
        if transfer_receipt.amountOut != swap_params.amountOut {
            anyhow::bail!(
                "solver did not send the correct funds. expected {}, got {}",
                swap_params.amountOut,
                transfer_receipt.amountOut
            );
        }

        // if we haven't crossed the required timestamp the fulfilment tx is
        // expected to be finalised at, blow up. Reorgs could affect this, so
        // operators are advised to be conservative when setting this in mainnet
        // so as not to lose funds.
        let finality_duration = self
            .finality_durations
            .get(&transfer_receipt.dstChainId)
            .expect("cannot get a chainID that doesn't exist");
        let fulfilled_time = DateTime::from_timestamp(transfer_receipt.fulfilledAt.try_into()?, 0)
            .ok_or(anyhow!("invalid fulfilled time"))?;

        let earliest_finalisation_time = fulfilled_time.add(*finality_duration);

        if Utc::now() < earliest_finalisation_time {
            anyhow::bail!("fulfillment hasn't yet finalised")
        }

        Ok(chain_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, FixedBytes, U160, U256};
    use chrono::Duration;
    use generated::onlyswaps::router::IRouter::SwapRequestParameters;
    use generated::onlyswaps::router::Router::getSwapRequestReceiptReturn;
    use std::str::FromStr;

    #[test]
    fn ok_when_everything_matches_exactly() {
        let params = base_params();
        let evaluator = Evaluator {
            finality_durations: HashMap::from([(params.dstChainId, Duration::seconds(1))]),
        };
        let expected_out = params.amountOut;
        let dest = receipt_from(&params, expected_out);
        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let out_state = evaluator.evaluate(chain_state).expect("should be ok");

        let out = out_state.swap_params;

        assert_eq!(out.dstChainId, params.dstChainId);
        assert_eq!(out.srcChainId, params.srcChainId);
        assert_eq!(out.tokenIn, params.tokenIn);
        assert_eq!(out.tokenOut, params.tokenOut);
        assert_eq!(out.amountOut, params.amountOut);
        assert_eq!(out.verificationFee, params.verificationFee);
        assert_eq!(out.executed, params.executed);
        assert_eq!(out.nonce, params.nonce);
        assert_eq!(out.solverFee, params.solverFee);
        assert_eq!(out.sender, params.sender);
        assert_eq!(out.recipient, params.recipient);
    }

    #[test]
    fn err_if_wrong_destination_chain() {
        let params = base_params();
        let evaluator = Evaluator {
            finality_durations: HashMap::from([(params.dstChainId, Duration::seconds(1))]),
        };
        let expected_out = params.amountOut;
        let mut dest = receipt_from(&params, expected_out);
        dest.dstChainId = params.dstChainId + U256::from(1);

        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let _ = evaluator
            .evaluate(chain_state)
            .expect_err("should fail on wrong destination chain");
    }

    #[test]
    fn err_if_wrong_source_chain_in_receipt() {
        let params = base_params();
        let evaluator = Evaluator {
            finality_durations: HashMap::from([(params.dstChainId, Duration::seconds(1))]),
        };
        let expected_out = params.amountOut;
        let mut dest = receipt_from(&params, expected_out);
        dest.srcChainId = params.srcChainId + U256::from(1);

        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let _ = evaluator
            .evaluate(chain_state)
            .expect_err("should fail on wrong source chain");
    }

    #[test]
    fn err_if_wrong_recipient_receipt() {
        let params = base_params();
        let evaluator = Evaluator {
            finality_durations: HashMap::from([(params.dstChainId, Duration::seconds(1))]),
        };
        let expected_out = params.amountOut;
        let mut dest = receipt_from(&params, expected_out);
        dest.recipient = Address::from(U160::from(123142));

        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let _ = evaluator
            .evaluate(chain_state)
            .expect_err("should fail on wrong recipient");
    }

    #[test]
    fn err_if_token_mismatch() {
        let params = base_params();
        let evaluator = Evaluator {
            finality_durations: HashMap::from([(params.dstChainId, Duration::seconds(1))]),
        };
        let expected_out = params.amountOut;
        let mut dest = receipt_from(&params, expected_out);
        dest.tokenOut = Address::from_str("cafebabecD7502C6b85ed2E11Fd5988AF76Cdd66").unwrap(); // different from params.token

        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let _ = evaluator
            .evaluate(chain_state)
            .expect_err("should fail on token mismatch");
    }

    #[test]
    fn err_if_amount_out_too_low() {
        let params = base_params();
        let evaluator = Evaluator {
            finality_durations: HashMap::from([(params.dstChainId, Duration::seconds(1))]),
        };
        let expected_out = params.amountOut;
        let dest = receipt_from(&params, expected_out - U256::from(1));
        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let _ = evaluator
            .evaluate(chain_state)
            .expect_err("should fail on underpayment");
    }

    #[test]
    fn err_if_amount_out_too_high_overpay_not_allowed() {
        let params = base_params();
        let evaluator = Evaluator {
            finality_durations: HashMap::from([(params.dstChainId, Duration::seconds(1))]),
        };
        let expected_out = params.amountOut;
        let dest = receipt_from(&params, expected_out + U256::from(1));
        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let _ = evaluator
            .evaluate(chain_state)
            .expect_err("should fail on overpayment");
    }

    #[test]
    fn err_if_not_yet_finalised() {
        let params = base_params();
        let evaluator = Evaluator {
            finality_durations: HashMap::from([(params.dstChainId, Duration::seconds(1000))]),
        };
        let expected_out = params.amountOut;
        let mut dest = receipt_from(&params, expected_out);
        dest.fulfilledAt = U256::from(Utc::now().timestamp() as u64).add(U256::from(1000000));

        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let _ = evaluator
            .evaluate(chain_state)
            .expect_err("not finalised should fail");
    }

    fn b32(byte: u8) -> FixedBytes<32> {
        FixedBytes::<32>::from([byte; 32])
    }

    fn base_params() -> SwapRequestParameters {
        SwapRequestParameters {
            dstChainId: U256::from(43114),
            srcChainId: U256::from(8453),
            tokenIn: Address::from_str("2226cAb3cD7502C6b85ed2E11Fd5988AF76Cdd66").unwrap(),
            tokenOut: Address::from_str("2226cAb3cD7502C6b85ed2E11Fd5988AF76Cdd67").unwrap(),
            amountOut: U256::from(1_000_000u64),
            verificationFee: U256::from(10_000u64),
            solverFee: U256::from(10_000u64),
            nonce: U256::from(1),
            sender: Address::from_str("6666cAb3cD7502C6b85ed2E11Fd5988AF76Cdd66").unwrap(),
            recipient: Address::from_str("2111cAb3cD7502C6b85ed2E11Fd5988AF76Cdd66").unwrap(),
            executed: true,
            requestedAt: U256::from(123456),
        }
    }

    fn receipt_from(
        params: &SwapRequestParameters,
        amount_out: U256,
    ) -> getSwapRequestReceiptReturn {
        getSwapRequestReceiptReturn {
            dstChainId: params.dstChainId,
            requestId: b32(0x11),
            tokenIn: params.tokenIn,
            tokenOut: params.tokenOut,
            srcChainId: params.srcChainId,
            fulfilled: true,
            solver: Address::from_str("1111cAb3cD7502C6b85ed2E11Fd5988AF76Cdd66").unwrap(),
            recipient: Address::from_str("2111cAb3cD7502C6b85ed2E11Fd5988AF76Cdd66").unwrap(),
            amountOut: amount_out,
            fulfilledAt: U256::from(123456u64),
        }
    }
}
