// Answer 0

#[test]
fn test_format64_zero_positive() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let f = 0.0f64; // sign = false, ieee_exponent == 0
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format64_zero_negative_exponent() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let f = 0.001234f64; // sign = false, ieee_exponent > 0 for this case, change as needed to test k and kk
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.001234");
    }
}

#[test]
fn test_format64_exponent_zero_edge_case() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let f = 1e-324f64; // gives zero exponent at the limit
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

