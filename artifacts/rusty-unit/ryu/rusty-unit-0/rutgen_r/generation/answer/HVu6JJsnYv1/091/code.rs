// Answer 0

#[test]
fn test_format32_positive_zero() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let f = 0.0f32;
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format32_negative_zero() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let f = -0.0f32;
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0");
    }
}

#[test]
fn test_format32_small_positive() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let f = 1.0f32; // k is 0 and kk is 1
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.0");
    }
}

#[test]
fn test_format32_large_positive() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let f = 1000000000000.0f32; // Should produce "1e12"
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1e12");
    }
}

