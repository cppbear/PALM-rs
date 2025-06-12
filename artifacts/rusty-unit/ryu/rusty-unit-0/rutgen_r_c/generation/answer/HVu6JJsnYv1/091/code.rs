// Answer 0

#[test]
fn test_format32_zero() {
    let f: f32 = 0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format32_negative_zero() {
    let f: f32 = -0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0");
    }
}

#[test]
fn test_format32_small_negative() {
    let f: f32 = -1e-45;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.000000000000000000000000000000000000000000000000000");
    }
}

#[test]
fn test_format32_small_positive() {
    let f: f32 = 1e-45;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.000000000000000000000000000000000000000000000000000");
    }
}

#[test]
fn test_format32_large_negative() {
    let f: f32 = -1e30;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-1e30");
    }
}

#[test]
fn test_format32_positive_exponent() {
    let f: f32 = 1e30;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1e30");
    }
}

#[test]
fn test_format32_e_float() {
    let f: f32 = 1.234e10;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.234e10");
    }
}

