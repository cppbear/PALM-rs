// Answer 0

#[test]
fn test_format32_zero() {
    let f: f32 = 0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
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
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0");
    }
}

#[test]
fn test_format32_small_number() {
    let f: f32 = 1.234e-10;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0000000001234");
    }
}

#[test]
#[should_panic]
fn test_format32_non_finite() {
    let f: f32 = f32::INFINITY;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
    }
} 

#[test]
fn test_format32_large_exponent() {
    let f: f32 = 1.0e38;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1e38");
    }
}

