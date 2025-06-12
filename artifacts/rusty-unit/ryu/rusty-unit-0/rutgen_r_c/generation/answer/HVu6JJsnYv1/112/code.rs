// Answer 0

#[test]
fn test_format32_zero() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let f = 0.0f32;

    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format32_negative_zero() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let f = -0.0f32;

    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0");
    }
}

#[test]
fn test_format32_small_number() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let f = 1.234f32;

    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.234");
    }
}

#[test]
fn test_format32_large_exponent() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let f = 1e30f32; // This will trigger the exponent scenario

    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1e30");
    }
}

#[test]
fn test_format32_negative_large_exponent() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let f = -1e30f32; // This will trigger the negative exponent scenario

    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-1e30");
    }
}

#[test]
fn test_format32_small_negative_number() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let f = -0.001234f32;

    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.001234");
    }
}

