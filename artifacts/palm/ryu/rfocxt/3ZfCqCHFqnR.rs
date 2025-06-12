use crate::raw;
use core::mem::MaybeUninit;
use core::{slice, str};
#[cfg(feature = "no-panic")]
use no_panic::no_panic;
const NAN: &str = "NaN";
const INFINITY: &str = "inf";
const NEG_INFINITY: &str = "-inf";
pub struct Buffer {
    bytes: [MaybeUninit<u8>; 24],
}
impl Buffer {
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn new() -> Self {
        let bytes = [MaybeUninit::<u8>::uninit(); 24];
        Buffer { bytes }
    }
    #[cfg_attr(feature = "no-panic", inline)]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format<F: Float>(&mut self, f: F) -> &str {}
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format_finite<F: Float>(&mut self, f: F) -> &str {}
}
