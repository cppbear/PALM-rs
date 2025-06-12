// Answer 0

#[test]
fn test_format64_positive_normal() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    unsafe {
        let f = 1.0f64; // IEEE: 0 01111111111 00000000000000000000000000000000000000000000000000000000000
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.0"); // Expects 1.0 format
    }
}

#[test]
fn test_format64_positive_large_exponent() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    unsafe {
        let f = 1.0e10; // IEEE: 0 10000111100 00000000000000000000000000000000000000000000000000000000000
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.0e10"); // Expects scientific notation
    }
}

#[test]
fn test_format64_negative() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    unsafe {
        let f = -1.0e10; // IEEE: 1 10000111100 00000000000000000000000000000000000000000000000000000000000
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "-1.0e10"); // Expects scientific notation with negative sign
    }
}

#[test]
fn test_format64_boundary_case() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    unsafe {
        let f = 1.0e-15; // IEEE: 0 10000101010 00000000000000000000000000000000000000000000000000000000000
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.000000000000001"); // Expects detailed decimal representation
    }
}

#[test]
fn test_format64_minimum_positive() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    unsafe {
        let f = 1.0e-324; // IEEE for minimal positive normal value.
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0000000000000000000000000000000001"); // Expects the minimum representation
    }
}

