// Answer 0

#[test]
fn test_format64_sign_false_ieee_exponent_zero_k_false() {
    let f: f64 = 0.0; // This will satisfy both sign == false and ieee_exponent == 0
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];

    unsafe {
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format64_smallest_positive_denormal() {
    let f: f64 = 5e-324; // This is the smallest positive denormal number
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];

    unsafe {
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "5e-324"); // Valid representation for denormal number
    }
}

#[test]
fn test_format64_large_exponent() {
    let f: f64 = 1e30; // Testing a large exponent
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];

    unsafe {
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1e30"); // Expected representation
    }
}

