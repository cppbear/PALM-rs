// Answer 0

#[test]
fn test_format32_negative_zero() {
    let f: f32 = -0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0");
    }
}

#[test]
fn test_format32_negative_subnormal() {
    let f: f32 = -1e-40;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0000000000000000000000000000000000000000000000000001");
    }
}

#[test]
fn test_format32_negative_small_magnitude() {
    let f: f32 = -1.0e-6;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.000001");
    }
}

