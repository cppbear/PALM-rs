// Answer 0

#[test]
fn test_format32_positive_normal() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1.23f32; // ieee_exponent > 0 and ieee_mantissa > 0
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.23");
    }
}

#[test]
fn test_format32_positive_large() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 123456789.0f32; // ieee_exponent > 0 and ieee_mantissa > 0
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "123456789");
    }
}

#[test]
fn test_format32_positive_exponential() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1.0e30f32; // ieee_exponent > 0 and ieee_mantissa > 0
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1e30");
    }
}

