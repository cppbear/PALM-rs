// Answer 0

#[test]
unsafe fn test_format64_negative_small_floats() {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let f = -2.2250738585072014e-308;
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format64_negative_mid_range_floats() {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let f = -1.0e-308;
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format64_negative_float_in_between() {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let f = -1.1125369292536007e-308;
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format64_negative_float_close_to_zero() {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let f = -1.0e-309;
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

