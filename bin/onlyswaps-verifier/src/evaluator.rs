use crate::chain_state_resolver::ChainState;

#[derive(Clone)]
pub struct Evaluator;
impl Evaluator {
    pub fn evaluate(chain_state: ChainState) -> anyhow::Result<ChainState> {
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

        if swap_params.executed {
            anyhow::bail!("the swap has already been executed, solver cannot fulfil twice")
        }

        Ok(chain_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, FixedBytes, U160, U256};
    use generated::onlyswaps::i_router::IRouter::SwapRequestParametersWithHooks;
    use generated::onlyswaps::i_router::IRouter::getSwapRequestReceiptReturn;
    use std::str::FromStr;

    #[test]
    fn ok_when_everything_matches_exactly() {
        let params = base_params();
        let expected_out = params.amountOut;
        let dest = receipt_from(&params, expected_out);
        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let out_state = Evaluator::evaluate(chain_state).expect("should be ok");

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
        let expected_out = params.amountOut;
        let mut dest = receipt_from(&params, expected_out);
        dest.dstChainId = params.dstChainId + U256::from(1);

        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let _ =
            Evaluator::evaluate(chain_state).expect_err("should fail on wrong destination chain");
    }

    #[test]
    fn err_if_wrong_source_chain_in_receipt() {
        let params = base_params();
        let expected_out = params.amountOut;
        let mut dest = receipt_from(&params, expected_out);
        dest.srcChainId = params.srcChainId + U256::from(1);

        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let _ = Evaluator::evaluate(chain_state).expect_err("should fail on wrong source chain");
    }

    #[test]
    fn err_if_wrong_recipient_receipt() {
        let params = base_params();
        let expected_out = params.amountOut;
        let mut dest = receipt_from(&params, expected_out);
        dest.recipient = Address::from(U160::from(123142));

        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let _ = Evaluator::evaluate(chain_state).expect_err("should fail on wrong recipient");
    }

    #[test]
    fn err_if_token_mismatch() {
        let params = base_params();
        let expected_out = params.amountOut;
        let mut dest = receipt_from(&params, expected_out);
        dest.tokenOut = Address::from_str("cafebabecD7502C6b85ed2E11Fd5988AF76Cdd66").unwrap(); // different from params.token

        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let _ = Evaluator::evaluate(chain_state).expect_err("should fail on token mismatch");
    }

    #[test]
    fn err_if_amount_out_too_low() {
        let params = base_params();
        let expected_out = params.amountOut;
        let dest = receipt_from(&params, expected_out - U256::from(1));
        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let _ = Evaluator::evaluate(chain_state).expect_err("should fail on underpayment");
    }

    #[test]
    fn err_if_amount_out_too_high_overpay_not_allowed() {
        let params = base_params();
        let expected_out = params.amountOut;
        let dest = receipt_from(&params, expected_out + U256::from(1));
        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let _ = Evaluator::evaluate(chain_state).expect_err("should fail on overpayment");
    }

    #[test]
    fn err_if_swap_already_executed() {
        let mut params = base_params();
        params.executed = true;
        let dest = receipt_from(&params, params.amountOut);
        let chain_state = ChainState {
            transfer_receipt: dest.clone(),
            swap_params: params.clone(),
        };

        let _ = Evaluator::evaluate(chain_state).expect_err("should fail if already executed");
    }

    fn b32(byte: u8) -> FixedBytes<32> {
        FixedBytes::<32>::from([byte; 32])
    }

    fn base_params() -> SwapRequestParametersWithHooks {
        SwapRequestParametersWithHooks {
            dstChainId: U256::from(43114),
            srcChainId: U256::from(8453),
            tokenIn: Address::from_str("2226cAb3cD7502C6b85ed2E11Fd5988AF76Cdd66").unwrap(),
            tokenOut: Address::from_str("2226cAb3cD7502C6b85ed2E11Fd5988AF76Cdd67").unwrap(),
            amountIn: U256::from(1_000_000u64),
            amountOut: U256::from(1_000_000u64),
            verificationFee: U256::from(10_000u64),
            solverFee: U256::from(10_000u64),
            nonce: U256::from(1),
            sender: Address::from_str("6666cAb3cD7502C6b85ed2E11Fd5988AF76Cdd66").unwrap(),
            recipient: Address::from_str("2111cAb3cD7502C6b85ed2E11Fd5988AF76Cdd66").unwrap(),
            executed: false,
            requestedAt: U256::from(123456),
            preHooks: Vec::new(),
            postHooks: Vec::new(),
        }
    }

    fn receipt_from(
        params: &SwapRequestParametersWithHooks,
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
