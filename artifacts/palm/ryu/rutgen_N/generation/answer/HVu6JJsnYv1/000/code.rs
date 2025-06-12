// Answer 0

#[test]
fn test_format_positive_float() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1.234f32;
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.234");
    }
}

#[test]
fn test_format_zero_float() {
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
fn test_format_negative_float() {
    use std::{mem::MaybeUninit, slice, str};

    let f = -3.14159f32;
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "-3.14159");
    }
}

#[test]
fn test_format_large_float() {
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
fn test_format_small_float() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1e-30f32;
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.000000000000000000000001"); // Adjust based on actual output
    }
}

#[test]
fn test_format_negative_large_float() {
    use std::{mem::MaybeUninit, slice, str};

    let f = -1e30f32;
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "-1e30");
    }
}

