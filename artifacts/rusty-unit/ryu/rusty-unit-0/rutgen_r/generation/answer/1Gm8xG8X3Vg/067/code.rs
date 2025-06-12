// Answer 0

#[test]
fn test_format64_simple_positive() {
    use std::{mem::MaybeUninit, slice, str};

    let f: f64 = 2.5; // This will not be NaN or infinity.
    
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "2.5");
    }
}

#[test]
fn test_format64_scientific() {
    use std::{mem::MaybeUninit, slice, str};

    let f: f64 = 1e30; // This tests a large number using scientific format.

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1e30");
    }
}

#[test]
fn test_format64_zero() {
    use std::{mem::MaybeUninit, slice, str};

    let f: f64 = 0.0; // Testing the handling of zero.

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format64_small_negative() {
    use std::{mem::MaybeUninit, slice, str};

    let f: f64 = -0.001234; // Negative number within the range.

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.001234");
    }
}

#[test]
fn test_format64_large_negative() {
    use std::{mem::MaybeUninit, slice, str};

    let f: f64 = -123456789.0; // A large negative number.

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "-123456789");
    }
}

