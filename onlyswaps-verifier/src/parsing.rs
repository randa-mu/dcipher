use crate::eth::IRouter::SwapRequestParameters;
use alloy::primitives::{Address, FixedBytes, U256};

// this isn't exported from the bindings because it's not used in a func
// signature, boooo
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TransferReceipt {
    pub chain_id: U256,
    pub request_id: FixedBytes<32>,
    pub recipient: Address,
    pub src_chain_id: U256,
    pub token: Address,
    pub fulfilled: bool,
    pub solver: Address,
    pub amount_out: U256,
    pub fulfilled_at: U256,
}
pub fn reconcile_transfer_params(
    src_params: SwapRequestParameters,
    dest_receipt: TransferReceipt,
) -> anyhow::Result<SwapRequestParameters> {
    if dest_receipt.chain_id != src_params.dstChainId {
        anyhow::bail!("funds were sent on the wrong chain")
    }

    // this shouldn't be possible
    if dest_receipt.src_chain_id != src_params.srcChainId {
        anyhow::bail!("funds were sent from the wrong chain")
    }

    if dest_receipt.recipient != src_params.recipient {
        anyhow::bail!("funds were sent to the wrong recipient")
    }

    // right now this relies on the fact that tokens have the same address
    // on multiple chains which could be a bug
    if dest_receipt.token != src_params.tokenOut {
        anyhow::bail!("funds were sent from the wrong token")
    }

    // the contract manages the fee calculation, so we just used the `amountOut` to figure
    // out how much should be moved on the destination chain
    if dest_receipt.amount_out != src_params.amountOut {
        anyhow::bail!(
            "solver did not send the correct funds. expected {}, got {}",
            src_params.amountOut,
            dest_receipt.amount_out
        );
    }

    Ok(src_params)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, FixedBytes, U160, U256};
    use std::str::FromStr;

    #[test]
    fn ok_when_everything_matches_exactly() {
        let params = base_params();
        let expected_out = params.amountOut;
        let dest = receipt_from(&params, expected_out);

        let out = reconcile_transfer_params(params.clone(), dest).expect("should be ok");

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
        dest.chain_id = params.dstChainId + U256::from(1);

        let _ = reconcile_transfer_params(params, dest)
            .expect_err("should fail on wrong destination chain");
    }

    #[test]
    fn err_if_wrong_source_chain_in_receipt() {
        let params = base_params();
        let expected_out = params.amountOut;
        let mut dest = receipt_from(&params, expected_out);
        dest.src_chain_id = params.srcChainId + U256::from(1);

        let _ =
            reconcile_transfer_params(params, dest).expect_err("should fail on wrong source chain");
    }

    #[test]
    fn err_if_wrong_recipient_receipt() {
        let params = base_params();
        let expected_out = params.amountOut;
        let mut dest = receipt_from(&params, expected_out);
        dest.recipient = Address::from(U160::from(123142));

        let _ =
            reconcile_transfer_params(params, dest).expect_err("should fail on wrong recipient");
    }

    #[test]
    fn err_if_token_mismatch() {
        let params = base_params();
        let expected_out = params.amountOut;
        let mut dest = receipt_from(&params, expected_out);
        dest.token = Address::from_str("cafebabecD7502C6b85ed2E11Fd5988AF76Cdd66").unwrap(); // different from params.token

        let _ = reconcile_transfer_params(params, dest).expect_err("should fail on token mismatch");
    }

    #[test]
    fn err_if_amount_out_too_low() {
        let params = base_params();
        let expected_out = params.amountOut;
        let dest = receipt_from(&params, expected_out - U256::from(1));
        let _ = reconcile_transfer_params(params, dest).expect_err("should fail on underpayment");
    }

    #[test]
    fn err_if_amount_out_too_high_overpay_not_allowed() {
        let params = base_params();
        let expected_out = params.amountOut;
        let dest = receipt_from(&params, expected_out + U256::from(1));

        let _ = reconcile_transfer_params(params, dest).expect_err("should fail on overpayment");
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

    fn receipt_from(params: &SwapRequestParameters, amount_out: U256) -> TransferReceipt {
        TransferReceipt {
            chain_id: params.dstChainId,
            request_id: b32(0x11),
            token: params.tokenOut,
            src_chain_id: params.srcChainId,
            fulfilled: true,
            solver: Address::from_str("1111cAb3cD7502C6b85ed2E11Fd5988AF76Cdd66").unwrap(),
            recipient: Address::from_str("2111cAb3cD7502C6b85ed2E11Fd5988AF76Cdd66").unwrap(),
            amount_out,
            fulfilled_at: U256::from(123456u64),
        }
    }
}
