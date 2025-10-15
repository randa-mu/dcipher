use alloy::primitives::U256;

pub(crate) fn to_float(n: U256, decimals: u8) -> f64 {
    let limbs = n.as_limbs();
    let mut val = 0f64;
    for (i, limb) in limbs.iter().enumerate() {
        val += (*limb as f64) * (2f64).powi((i * 64) as i32);
    }
    val / (10_u64.pow(decimals as u32) as f64)
}

#[cfg(test)]
mod test {
    use crate::maths::to_float;
    use alloy::primitives::U256;
    use speculoos::assert_that;
    use speculoos::numeric::OrderedAssertions;

    #[test]
    fn native_balance() {
        let balance = U256::from(1_000_000_000);
        let result = to_float(balance, 18);

        assert_that!(result).is_less_than(1_f64);
        assert_that!(result).is_greater_than(0_f64);
    }

    #[test]
    fn usdt_balance() {
        let balance = U256::from(1_000_000_000);
        let result = to_float(balance, 9);

        assert_that!(result).is_equal_to(1_f64);
    }

    #[test]
    fn usdt_fractional_balance() {
        let balance = U256::from(1_000_000_001);
        let result = to_float(balance, 9);

        assert_that!(result).is_greater_than(1_f64);
        assert_that!(result).is_less_than(2_f64);
    }
}
