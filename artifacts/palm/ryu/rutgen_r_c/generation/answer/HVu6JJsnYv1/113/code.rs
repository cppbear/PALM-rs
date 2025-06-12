// Answer 0

#[test]
fn test_format32_zero() {
    unsafe {
        let f: f32 = 0.0;
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
        assert_eq!(len, 3);
    }
}

#[test]
fn test_format32_negative_zero() {
    unsafe {
        let f: f32 = -0.0;
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0");
        assert_eq!(len, 4);
    }
}

#[test]
fn test_format32_small_positive() {
    unsafe {
        let f: f32 = 0.0001;
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0001");
        assert_eq!(len, 6);
    }
}

#[test]
fn test_format32_small_negative() {
    unsafe {
        let f: f32 = -0.0001;
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0001");
        assert_eq!(len, 7);
    }
}

#[test]
fn test_format32_exponent_notation() {
    unsafe {
        let f: f32 = 1e-45;
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.00000000000000000000");
        assert_eq!(len, 21);
    }
}

#[test]
fn test_format32_large_mantissa() {
    unsafe {
        let f: f32 = 123456.0;
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "123456");
        assert_eq!(len, 6);
    }
}

