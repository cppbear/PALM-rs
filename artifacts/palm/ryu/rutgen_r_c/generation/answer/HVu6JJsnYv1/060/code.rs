// Answer 0

#[test]
fn test_format32_positive_normal_number() {
    let f = 3.14f32;
    unsafe {
        let mut buffer = [core::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = core::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = core::str::from_utf8_unchecked(slice);
        assert_eq!(print, "3.14");
    }
}

#[test]
fn test_format32_large_exponent() {
    let f = 1.23e10f32;
    unsafe {
        let mut buffer = [core::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = core::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = core::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.23e10");
    }
}

#[test]
fn test_format32_decimal_with_exponent() {
    let f = 6.022f32;
    unsafe {
        let mut buffer = [core::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = core::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = core::str::from_utf8_unchecked(slice);
        assert_eq!(print, "6.022");
    }
}

