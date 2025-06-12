// Answer 0

#[test]
fn test_format32_sign_false_ieee_exponent_zero() {
    use std::{mem::MaybeUninit, ptr, slice, str};

    // Prepare the input value
    let f: f32 = 0.0; // sign is false, ieee_exponent == 0
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format32_boundary_k_negative() {
    use std::{mem::MaybeUninit, ptr, slice, str};

    // Prepare an input that makes k == -45
    let f: f32 = 1.401298464e-45; // Approaching the lower limit
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.401298464e-45");
    }
}

#[test]
fn test_format32_kk_with_bound() {
    use std::{mem::MaybeUninit, ptr, slice, str};

    // Prepare an input value that will give you kk == 13
    let f: f32 = 1234567890123.0; // 1.234567890123e+12
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1234567890123.0");
    }
}

