pub use stark_core as starkcore;
#[cfg(test)]
mod tests {
    use crate::starkcore::utils::constants::ZERO;
    use crate::starkcore::utils::constants::MASK_250;
    use crate::starkcore::utils::constants::MASK_251;
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
    #[test]
    fn check_stark_core_utils_constant_mask_251() {
        let a:U256  = *MASK_251;
        println!("{}",a);
        assert_eq!(a,U256::from_dec_str("3618502788666131106986593281521497120414687020801267626233049500247285301248").unwrap());
    }


}
