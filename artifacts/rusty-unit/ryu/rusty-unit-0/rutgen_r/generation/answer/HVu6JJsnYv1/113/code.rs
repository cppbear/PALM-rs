// Answer 0

#[test]
fn test_format32_zero() {
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
fn test_format32_negative_zero() {
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
fn test_format32_small_negative() {
    use std::{mem::MaybeUninit, slice, str};

    let f = -1e-6f32;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.000001");
    }
}

#[test]
fn test_format32_large_number() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1e30f32;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1e30");
    }
}

#[test]
fn test_format32_small_positive() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1e-6f32;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.000001");
    }
}

