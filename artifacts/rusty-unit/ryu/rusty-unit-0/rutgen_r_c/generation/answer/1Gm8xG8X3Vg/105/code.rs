// Answer 0

#[test]
fn test_format64_positive_normal() {
    unsafe {
        let f: f64 = 1.234;
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let output = std::str::from_utf8_unchecked(slice);
        assert_eq!(output, "1.234");
    }
}

#[test]
fn test_format64_zero() {
    unsafe {
        let f: f64 = 0.0;
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let output = std::str::from_utf8_unchecked(slice);
        assert_eq!(output, "0.0");
    }
}

#[test]
fn test_format64_negative() {
    unsafe {
        let f: f64 = -1.234;
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let output = std::str::from_utf8_unchecked(slice);
        assert_eq!(output, "-1.234");
    }
}

#[test]
fn test_format64_large_exponent() {
    unsafe {
        let f: f64 = 1e15; // 10^15
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let output = std::str::from_utf8_unchecked(slice);
        assert_eq!(output, "1.0e15");
    }
}

#[test]
fn test_format64_small_number() {
    unsafe {
        let f: f64 = 1e-5; // 0.00001
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let output = std::str::from_utf8_unchecked(slice);
        assert_eq!(output, "0.00001");
    }
}

