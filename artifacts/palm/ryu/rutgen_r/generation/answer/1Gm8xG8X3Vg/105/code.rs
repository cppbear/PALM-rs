// Answer 0

#[test]
fn test_format64_negative_boundary() {
    let f = 0.0f64; // This will test ieee_exponent == 0 and ieee_mantissa == 0
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
        assert_eq!(len, 3);
    }
}

#[test]
fn test_format64_large_exponent() {
    let f = 1e16f64; // Testing a value with exponent that leads to kk being 16
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "10000000000000000.0");
        assert_eq!(len, 17);
    }
}

#[test]
fn test_format64_small_exponent() {
    let f = 1e-5f64; // Testing a number leading to 0.00001
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.00001");
        assert_eq!(len, 6);
    }
}

