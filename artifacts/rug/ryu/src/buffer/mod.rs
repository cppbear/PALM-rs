use crate::raw;
use core::mem::MaybeUninit;
use core::{slice, str};
#[cfg(feature = "no-panic")]
use no_panic::no_panic;

const NAN: &str = "NaN";
const INFINITY: &str = "inf";
const NEG_INFINITY: &str = "-inf";

/// Safe API for formatting floating point numbers to text.
///
/// ## Example
///
/// ```
/// let mut buffer = ryu::Buffer::new();
/// let printed = buffer.format_finite(1.234);
/// assert_eq!(printed, "1.234");
/// ```
pub struct Buffer {
    bytes: [MaybeUninit<u8>; 24],
}

impl Buffer {
    /// This is a cheap operation; you don't need to worry about reusing buffers
    /// for efficiency.
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn new() -> Self {
        let bytes = [MaybeUninit::<u8>::uninit(); 24];
        Buffer { bytes }
    }

    /// Print a floating point number into this buffer and return a reference to
    /// its string representation within the buffer.
    ///
    /// # Special cases
    ///
    /// This function formats NaN as the string "NaN", positive infinity as
    /// "inf", and negative infinity as "-inf" to match std::fmt.
    ///
    /// If your input is known to be finite, you may get better performance by
    /// calling the `format_finite` method instead of `format` to avoid the
    /// checks for special cases.
    #[cfg_attr(feature = "no-panic", inline)]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format<F: Float>(&mut self, f: F) -> &str {
        if f.is_nonfinite() {
            f.format_nonfinite()
        } else {
            self.format_finite(f)
        }
    }

    /// Print a floating point number into this buffer and return a reference to
    /// its string representation within the buffer.
    ///
    /// # Special cases
    ///
    /// This function **does not** check for NaN or infinity. If the input
    /// number is not a finite float, the printed representation will be some
    /// correctly formatted but unspecified numerical value.
    ///
    /// Please check [`is_finite`] yourself before calling this function, or
    /// check [`is_nan`] and [`is_infinite`] and handle those cases yourself.
    ///
    /// [`is_finite`]: f64::is_finite
    /// [`is_nan`]: f64::is_nan
    /// [`is_infinite`]: f64::is_infinite
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

impl Copy for Buffer {}

impl Clone for Buffer {
    #[inline]
    #[allow(clippy::non_canonical_clone_impl)] // false positive https://github.com/rust-lang/rust-clippy/issues/11072
    fn clone(&self) -> Self {
        Buffer::new()
    }
}

impl Default for Buffer {
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    fn default() -> Self {
        Buffer::new()
    }
}

/// A floating point number, f32 or f64, that can be written into a
/// [`ryu::Buffer`][Buffer].
///
/// This trait is sealed and cannot be implemented for types outside of the
/// `ryu` crate.
pub trait Float: Sealed {}
impl Float for f32 {}
impl Float for f64 {}

pub trait Sealed: Copy {
    fn is_nonfinite(self) -> bool;
    fn format_nonfinite(self) -> &'static str;
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize;
}

impl Sealed for f32 {
    #[inline]
    fn is_nonfinite(self) -> bool {
        const EXP_MASK: u32 = 0x7f800000;
        let bits = self.to_bits();
        bits & EXP_MASK == EXP_MASK
    }

    #[cold]
    #[cfg_attr(feature = "no-panic", inline)]
    fn format_nonfinite(self) -> &'static str {
        const MANTISSA_MASK: u32 = 0x007fffff;
        const SIGN_MASK: u32 = 0x80000000;
        let bits = self.to_bits();
        if bits & MANTISSA_MASK != 0 {
            NAN
        } else if bits & SIGN_MASK != 0 {
            NEG_INFINITY
        } else {
            INFINITY
        }
    }

    #[inline]
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
        raw::format32(self, result)
    }
}

impl Sealed for f64 {
    #[inline]
    fn is_nonfinite(self) -> bool {
        const EXP_MASK: u64 = 0x7ff0000000000000;
        let bits = self.to_bits();
        bits & EXP_MASK == EXP_MASK
    }

    #[cold]
    #[cfg_attr(feature = "no-panic", inline)]
    fn format_nonfinite(self) -> &'static str {
        const MANTISSA_MASK: u64 = 0x000fffffffffffff;
        const SIGN_MASK: u64 = 0x8000000000000000;
        let bits = self.to_bits();
        if bits & MANTISSA_MASK != 0 {
            NAN
        } else if bits & SIGN_MASK != 0 {
            NEG_INFINITY
        } else {
            INFINITY
        }
    }

    #[inline]
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
        raw::format64(self, result)
    }
}

#[cfg(test)]
mod tests_llm_16_4 {
    use super::*;

use crate::*;
    use crate::buffer::Sealed;

    #[test]
    fn test_is_nonfinite() {
        assert!(!f32::from_bits(0x3f800000).is_nonfinite()); // 1.0
        assert!(!f32::from_bits(0x7f800000).is_nonfinite()); // Infinity
        assert!(f32::from_bits(0x7fc00000).is_nonfinite()); // NaN
        assert!(f32::from_bits(0x80000000).is_nonfinite()); // -0.0
        assert!(!f32::from_bits(0x00000000).is_nonfinite()); // 0.0
    }
}

#[cfg(test)]
mod tests_llm_16_5 {
    use super::*;

use crate::*;
    use crate::buffer::Sealed;

    #[test]
    fn test_write_to_ryu_buffer() {
        let mut buffer = [0u8; 32];
        let f: f32 = 1.0;

        let length = unsafe { f.write_to_ryu_buffer(buffer.as_mut_ptr()) };

        assert!(length > 0);
        assert_eq!(buffer[..length as usize].to_vec(), b"1.0".to_vec()); // Adjust the expected output as necessary
    }
}

#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;

use crate::*;
    use crate::buffer::Sealed; // Ensure the path is correct according to your module structure

    #[test]
    fn test_format_nonfinite_nan() {
        let nan_value = f64::NAN;
        let result = nan_value.format_nonfinite();
        assert_eq!(result, "NAN");
    }

    #[test]
    fn test_format_nonfinite_negative_infinity() {
        let neg_infinity_value = f64::NEG_INFINITY;
        let result = neg_infinity_value.format_nonfinite();
        assert_eq!(result, "NEG_INFINITY");
    }

    #[test]
    fn test_format_nonfinite_infinity() {
        let infinity_value = f64::INFINITY;
        let result = infinity_value.format_nonfinite();
        assert_eq!(result, "INFINITY");
    }

    #[test]
    fn test_format_nonfinite_finite() {
        let finite_value = 42.0;
        let result = finite_value.format_nonfinite();
        assert_eq!(result, "NAN");
    }
}

#[cfg(test)]
mod tests_llm_16_7 {
    use super::*;

use crate::*;
    use crate::buffer::Sealed;

    #[test]
    fn test_is_nonfinite() {
        assert!(f64::NAN.is_nonfinite());
        assert!(f64::INFINITY.is_nonfinite());
        assert!(f64::NEG_INFINITY.is_nonfinite());
        assert!(!0.0.is_nonfinite());
        assert!(!1.0.is_nonfinite());
        assert!(!f64::from_bits(0x7ff8000000000000).is_nonfinite()); // NaN
    }
}

#[cfg(test)]
mod tests_llm_16_9 {
    use super::*;

use crate::*;
    use crate::Buffer;
    use crate::Float;

    #[test]
    fn test_format_finite() {
        let mut buffer = Buffer::new();
        let printed = buffer.format(1.234);
        assert_eq!(printed, "1.234");
    }

    #[test]
    fn test_format_nan() {
        let mut buffer = Buffer::new();
        let printed = buffer.format(f64::NAN);
        assert_eq!(printed, "NaN");
    }

    #[test]
    fn test_format_infinity() {
        let mut buffer = Buffer::new();
        let printed = buffer.format(f64::INFINITY);
        assert_eq!(printed, "inf");
    }

    #[test]
    fn test_format_negative_infinity() {
        let mut buffer = Buffer::new();
        let printed = buffer.format(f64::NEG_INFINITY);
        assert_eq!(printed, "-inf");
    }
}

#[cfg(test)]
mod tests_llm_16_10 {
    use super::*;

use crate::*;
    use crate::buffer::Buffer;
    use crate::buffer::Float; // Assuming `Float` is implemented for f32 and f64 within the crate.

    #[test]
    fn test_format_finite_f32() {
        let mut buffer = Buffer::new();
        let printed = buffer.format_finite(1.234f32);
        assert_eq!(printed, "1.234");
    }

    #[test]
    fn test_format_finite_f64() {
        let mut buffer = Buffer::new();
        let printed = buffer.format_finite(1.23456789f64);
        assert_eq!(printed, "1.23456789");
    }

    #[test]
    fn test_format_finite_negative_number() {
        let mut buffer = Buffer::new();
        let printed = buffer.format_finite(-0.001f32);
        assert_eq!(printed, "-0.001");
    }

    #[test]
    fn test_format_finite_large_number() {
        let mut buffer = Buffer::new();
        let printed = buffer.format_finite(1234567890.123456f64);
        assert_eq!(printed, "1234567890.123456");
    }

    #[test]
    fn test_format_finite_small_number() {
        let mut buffer = Buffer::new();
        let printed = buffer.format_finite(0.000000000001f64);
        assert_eq!(printed, "1.000000000001e-12");
    }
}

#[cfg(test)]
mod tests_llm_16_11 {
    use crate::Buffer;

    #[test]
    fn test_new_buffer() {
        let buffer = Buffer::new();
        // Check if the buffer's byte array is initialized (although it contains uninitialized bytes)
        // We can check if the size of the array is correct
        assert_eq!(buffer.bytes.len(), 24);
    }

    #[test]
    fn test_clone_buffer() {
        let buffer = Buffer::new();
        let cloned_buffer = buffer.clone();
        assert_eq!(buffer.bytes.len(), cloned_buffer.bytes.len());
    }

    #[test]
    fn test_default_buffer() {
        let default_buffer = Buffer::default();
        assert_eq!(default_buffer.bytes.len(), 24);
    }
}
