// Answer 0

#[test]
fn test_format32_positive_zero() {
    let f: f32 = 0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
        assert_eq!(len, 3);
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
        assert_eq!(len, 4);
    }
}

#[test]
fn test_format32_minimum_exponent() {
    let f: f32 = 1.0e-45; // ieee_exponent == 0
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.000000000000000000"); // should format to 0 with additional zeros for kk = 13
        assert_eq!(len, 16);
    }
}

#[test]
fn test_format32_normal_positive() {
    let f: f32 = 4.5; // Normal case
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "4.5");
        assert_eq!(len, 3);
    }
}

#[test]
fn test_format32_large_exponent() {
    let f: f32 = 1.34e+30; // tests large exponent
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.34e30");
        assert_eq!(len, 8);
    }
}

#[test]
fn test_format32_small_exponent() {
    let f: f32 = 1.2e-37; // tests small positive exponent
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.000000000000012"); // should correctly format for small values
        assert_eq!(len, 16);
    }
}

