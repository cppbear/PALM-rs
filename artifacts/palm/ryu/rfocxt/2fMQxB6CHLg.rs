use crate::raw;
use core::mem::MaybeUninit;
use core::{slice, str};
#[cfg(feature = "no-panic")]
use no_panic::no_panic;
const NAN: &str = "NaN";
const INFINITY: &str = "inf";
const NEG_INFINITY: &str = "-inf";
pub trait Sealed: Copy {
    fn is_nonfinite(self) -> bool;
    fn format_nonfinite(self) -> &'static str;
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize;
}
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
    pub fn format_finite<F: Float>(&mut self, f: F) -> &str {
        unsafe {
            let n = f.write_to_ryu_buffer(self.bytes.as_mut_ptr() as *mut u8);
            debug_assert!(n <= self.bytes.len());
            let slice = slice::from_raw_parts(self.bytes.as_ptr() as *const u8, n);
            str::from_utf8_unchecked(slice)
        }
    }
}
