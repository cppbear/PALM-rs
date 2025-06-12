// Answer 0

#[test]
fn test_format64_zero() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let f = 0.0f64;
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
        assert_eq!(len, 3);
    }
}

#[test]
fn test_format64_negative_zero() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let f = -0.0f64;
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0");
        assert_eq!(len, 4);
    }
}

#[test]
fn test_format64_small_positive_value() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let f = 0.0001234f64;
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0001234");
        assert_eq!(len, 10);
    }
}

#[test]
fn test_format64_negative_small_value() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let f = -0.0001234f64;
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0001234");
        assert_eq!(len, 11);
    }
}

#[test]
fn test_format64_large_exponent_negative() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let f = 1e-324f64; // k == -324
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.000000000000000000000001");
        assert_eq!(len, 24);
    }
}

