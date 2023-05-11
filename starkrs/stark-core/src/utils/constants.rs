use ethers::types::U256;
use lazy_static::lazy_static;
pub const ZERO:u64 = 0;
pub const MASK:&str = "1809251394333065553493296640760748560207343510400633813116524750123642650623";

lazy_static! {
    pub static ref MASK_250: U256 = U256::from_dec_str(MASK).unwrap();
}