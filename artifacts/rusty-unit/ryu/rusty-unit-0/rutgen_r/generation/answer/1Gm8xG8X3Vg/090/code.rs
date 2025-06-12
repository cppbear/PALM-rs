// Answer 0

#[test]
fn test_format64_zero() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let f: f64 = 0.0; // sign is false, ieee_exponent == 0 and ieee_mantissa == 0
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
        assert_eq!(len, 3); // index + kk + 2, with kk = 1 (0.0), thus len = 0 + 1 + 2
    }
}

#[test]
fn test_format64_small_magnitude() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let f: f64 = 0.0001234; // subnormal positive number
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0001234");
        assert_eq!(len, 10); // index + kk + 2, where kk = 7 and index = 1 (1 + 7 + 2 = 10)
    }
}

#[test]
fn test_format64_large_magnitude() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let f: f64 = 1.234e14; // regular number
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "123400000000000.0");
        assert_eq!(len, 17); // index + kk + 2, where kk = 16 (16 + 1 = 17, length past the decimal)
    }
}

#[test]
fn test_format64_large_exponent() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let f: f64 = 1.234e30; // Exceeds the boundary of decimal representation
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.234e30");
        assert_eq!(len, 10); // index + kk + 2 (for 'e30' part)
    }
}

