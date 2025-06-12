// Answer 0

#[test]
fn test_format64_positive_integer() {
    let f = 1234.0f64;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len: usize;

    unsafe {
        len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1234");
    }
}

#[test]
fn test_format64_negative_integer() {
    let f = -5678.0f64;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len: usize;

    unsafe {
        len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-5678");
    }
}

#[test]
fn test_format64_finite_fraction() {
    let f = 1.234f64;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len: usize;

    unsafe {
        len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.234");
    }
}

#[test]
fn test_format64_small_fraction() {
    let f = 0.001234f64;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len: usize;

    unsafe {
        len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.001234");
    }
}

#[test]
fn test_format64_large_exponent() {
    let f = 1e30f64;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len: usize;

    unsafe {
        len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1e30");
    }
}

#[test]
fn test_format64_zero() {
    let f = 0.0f64;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len: usize;

    unsafe {
        len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

