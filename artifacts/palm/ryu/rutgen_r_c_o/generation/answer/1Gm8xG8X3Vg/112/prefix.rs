// Answer 0

#[test]
fn test_format64_zero() {
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let f = 0.0f64;
        let len = format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
    }
}

#[test]
fn test_format64_negative_zero() {
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let f = -0.0f64;
        let len = format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
    }
}

#[test]
fn test_format64_negative_small_value() {
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let f = -1e-324;
        let len = format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
    }
}

#[test]
fn test_format64_small_value() {
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let f = 1e-324;
        let len = format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
    }
}

#[test]
fn test_format64_negative_small_mantissa() {
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let f = -5e-324;
        let len = format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
    }
}

#[test]
fn test_format64_positive_small_mantissa() {
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let f = 5e-324;
        let len = format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
    }
}

