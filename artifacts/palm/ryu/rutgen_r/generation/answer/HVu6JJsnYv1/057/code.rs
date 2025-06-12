// Answer 0

#[test]
fn test_format32_with_negative_zero() {
    let f: f32 = -0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0"); // Expecting output for -0.0
    }
}

#[test]
fn test_format32_with_negative_small_value() {
    let f: f32 = -1e-45; // Negative small value, to explore safety around k
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];

    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.000000000000000000"); // Expecting output for very small negative value
    }
}

#[test]
fn test_format32_with_small_negative_normal_case() {
    let f: f32 = -0.01234; // Negative value for common case
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];

    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.01234"); // Expecting output for the negative decimal
    }
}

#[test]
fn test_format32_with_large_negative_exponent() {
    let f: f32 = -1e-6; // Negative value with large negative exponent
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];

    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.000001"); // Expecting output for -1e-6
    }
}

