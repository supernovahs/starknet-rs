use ethers::types::U256;
use lazy_static::lazy_static;
pub const ZERO:u64 = 0;
pub const MASK1:&str = "1809251394333065553493296640760748560207343510400633813116524750123642650623";
pub const MASK2:&str = "3618502788666131106986593281521497120414687020801267626233049500247285301248";
lazy_static! {
    pub static ref MASK_250: U256 = U256::from_dec_str(MASK1).unwrap();
    pub static ref MASK_251: U256 = U256::from_dec_str(MASK2).unwrap();
}