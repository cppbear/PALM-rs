// Answer 0

#[test]
fn test_format64_zero() {
    unsafe {
        let f = 0.0f64;
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format64_negative_zero() {
    unsafe {
        let f = -0.0f64;
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0");
    }
}

#[test]
fn test_format64_small_negative() {
    unsafe {
        let f = -1e-324f64; // Very small negative number, edge case near denormalized numbers
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0000000000000000000000000001");
    }
}

#[test]
fn test_format64_small_positive() {
    unsafe {
        let f = 1e-324f64; // Very small positive number, edge case near denormalized numbers
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0000000000000000000000000001");
    }
}

#[test]
#[should_panic]
fn test_format64_overflow() {
    unsafe {
        let f = 1e300f64; // Value should cause overflow handling in output
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let _len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8); // Expecting this to panic or handle overflow
    }
}

