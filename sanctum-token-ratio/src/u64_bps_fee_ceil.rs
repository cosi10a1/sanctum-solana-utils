use crate::{AmtsAfterFee, MathError, U64FeeCeil, BPS_DENOMINATOR};

/// A bps fee to charge where value <= 10_000
/// amt_after_fees = floor(amt * (10_000 - fee_num) / 10_000),
/// effectively maximizing fees charged
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct U64BpsFeeCeil(pub u16);

impl U64BpsFeeCeil {
    pub const fn as_u64_fee_ceil(&self) -> U64FeeCeil<u16, u16> {
        U64FeeCeil {
            fee_num: self.0,
            fee_denom: BPS_DENOMINATOR,
        }
    }

    /// Errors if value > 10_000 (fee > 100%)
    pub fn apply(&self, amt: u64) -> Result<AmtsAfterFee, MathError> {
        self.as_u64_fee_ceil().apply(amt)
    }

    pub fn is_valid(&self) -> bool {
        self.0 <= BPS_DENOMINATOR
    }

    /// Returns a possible amount that was fed into self.apply()
    ///
    /// Returns `amt_after_apply` if fee_num == 0 || fee_denom == 0
    ///
    /// Errors if fee_num > fee_denom (fee > 100%)
    pub fn pseudo_reverse(&self, amt_after_fee: u64) -> Result<u64, MathError> {
        self.as_u64_fee_ceil().pseudo_reverse(amt_after_fee)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        fn u64_fee_lte_one()
            (fee_bps in 0..=BPS_DENOMINATOR) -> U64BpsFeeCeil {
                U64BpsFeeCeil(fee_bps)
            }
    }

    proptest! {
        #[test]
        fn u64_fee_invariants(amt: u64, fee in u64_fee_lte_one()) {
            let AmtsAfterFee { amt_after_fee, fees_charged } = fee.apply(amt).unwrap();
            prop_assert!(amt_after_fee <= amt);
            prop_assert_eq!(amt, amt_after_fee + fees_charged);
        }
    }

    proptest! {
        #[test]
        fn u64_zero_fee(amt: u64) {
            let fee = U64BpsFeeCeil(0u16);
            let amts_after_fee = fee.apply(amt).unwrap();

            prop_assert_eq!(amts_after_fee.amt_after_fee, amt);
            prop_assert_eq!(amts_after_fee.fees_charged, 0);
        }
    }

    prop_compose! {
        fn u64_smaller_larger()
            (boundary in any::<u64>())
            (smaller in 0..=boundary, larger in boundary..=u64::MAX) -> (u64, u64) {
                (smaller, larger)
            }
    }

    proptest! {
        #[test]
        fn valid_invalid(bps: u16) {
            let fee = U64BpsFeeCeil(bps);
            if bps > BPS_DENOMINATOR {
                prop_assert!(!fee.is_valid())
            } else {
                prop_assert!(fee.is_valid())
            }
        }
    }
}
