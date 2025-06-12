use core::ptr;
#[cfg_attr(feature = "no-panic", inline)]
pub fn div5(x: u64) -> u64 {
    x / 5
}
