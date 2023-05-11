pub use stark_core as core;
#[cfg(test)]
mod tests {
    use crate::core::utils::constants::ZERO;
    use crate::core::utils::constants::MASK_250;
    use ethers::prelude::*;
    #[test]
    fn check_stark_core_utils_constant_zero() {
        let a = ZERO;
        assert_eq!(ZERO,0);
    }
    #[test]
    fn check_stark_core_utils_constant_mask_250() {
        let a:U256  = *MASK_250;
        assert_eq!(a,U256::from_dec_str("180925139433306555349329664076074856020734351040063381311652475012364265062").unwrap());
    }
    
}
