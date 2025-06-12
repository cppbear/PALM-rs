// Answer 0

#[test]
fn test_format64_positive() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let f = 1.234f64;
    
    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.234");
    }
}

#[test]
fn test_format64_negative() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let f = -2.718f64;

    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-2.718");
    }
}

#[test]
fn test_format64_zero() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let f = 0.0f64;

    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format64_small_number() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let f = 0.0001234f64;

    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0001234");
    }
}

#[test]
fn test_format64_large_exponent() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let f = 1.23e30f64;

    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.23e30");
    }
}

#[test]
#[should_panic]
fn test_format64_invalid_pointer() {
    let f = 1.234f64;
    unsafe {
        ryu::format64(f, std::ptr::null_mut());
    }
}

