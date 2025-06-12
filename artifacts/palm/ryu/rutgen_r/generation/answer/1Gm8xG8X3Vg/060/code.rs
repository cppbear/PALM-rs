// Answer 0

#[test]
fn test_format64_positive_normal_value() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1.234f64;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.234");
    }
}

#[test]
fn test_format64_large_number() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 12345678901234.0f64;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "12345678901234");
    }
}

#[test]
fn test_format64_small_positive_value() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 0.001234f64;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.001234");
    }
}

#[test]
fn test_format64_large_negative_value() {
    use std::{mem::MaybeUninit, slice, str};

    let f = -9876543210.1234f64;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "-9876543210.1234");
    }
}

