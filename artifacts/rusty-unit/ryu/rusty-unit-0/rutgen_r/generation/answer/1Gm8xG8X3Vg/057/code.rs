// Answer 0

#[test]
fn test_format64_negative_zero() {
    let f: f64 = -0.0;

    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0");
    }
}

#[test]
fn test_format64_negative_smallest() {
    let f: f64 = -1e-324;

    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.000000000000000000000001"); // Adjust based on expected underflow representation
    }
}

#[test]
fn test_format64_negative_finite_large_exponent() {
    let f: f64 = -1e30;

    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-1e30");
    }
}

