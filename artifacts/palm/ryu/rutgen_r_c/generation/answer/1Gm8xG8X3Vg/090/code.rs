// Answer 0

#[test]
fn test_format64_zero() {
    let f: f64 = 0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let result = std::str::from_utf8_unchecked(slice);
        assert_eq!(result, "0.0");
        assert_eq!(len, 3);
    }
}

#[test]
fn test_format64_negative_zero() {
    let f: f64 = -0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let result = std::str::from_utf8_unchecked(slice);
        assert_eq!(result, "-0.0");
        assert_eq!(len, 4);
    }
}

#[test]
fn test_format64_with_k_equal_to_minus_324() {
    let f: f64 = -3.4e-324; // IEEE representation for k == -324
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let result = std::str::from_utf8_unchecked(slice);
        assert_eq!(result, "0.0000000000000000000000000000034");
        assert!(len <= 24);
    }
}

#[test]
fn test_format64_large_exponent() {
    let f: f64 = 1.234e15; // Should stay within limits and show as large number
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let result = std::str::from_utf8_unchecked(slice);
        assert_eq!(result, "1.234e15");
        assert!(len <= 24);
    }
}

#[test]
fn test_format64_small_exponent() {
    let f: f64 = 1.234e-5; // k < 0
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let result = std::str::from_utf8_unchecked(slice);
        assert_eq!(result, "0.00001234");
        assert!(len <= 24);
    }
}

