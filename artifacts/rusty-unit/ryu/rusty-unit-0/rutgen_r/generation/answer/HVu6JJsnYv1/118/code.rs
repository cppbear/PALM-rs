// Answer 0

#[test]
fn test_format32_zero_positive() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 0.0f32;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format32_zero_negative() {
    use std::{mem::MaybeUninit, slice, str};

    let f = -0.0f32;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0");
    }
}

#[test]
#[should_panic]
fn test_format32_underflow() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1.0e-46f32; // this causes the exponent to fall below -45

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        // this test does not check the value but aims to cause a panic in the function
    }
}

