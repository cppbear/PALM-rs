// Answer 0

#[test]
fn test_format64_case_zero() {
    let f: f64 = 0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len = unsafe { ryu::format64(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let printed = unsafe { std::str::from_utf8_unchecked(slice) };
    assert_eq!(printed, "0.0");
}

#[test]
fn test_format64_case_positive_small() {
    let f: f64 = 1e-16; // This value will create an output with exponent 0 and mantissa as a small number.
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len = unsafe { ryu::format64(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let printed = unsafe { std::str::from_utf8_unchecked(slice) };
    assert_eq!(printed, "0.0000000000000001");
}

#[test]
fn test_format64_case_negative_small() {
    let f: f64 = -1e-16; // This value will create an output with exponent 0 and negative small mantissa.
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len = unsafe { ryu::format64(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let printed = unsafe { std::str::from_utf8_unchecked(slice) };
    assert_eq!(printed, "-0.0000000000000001");
}

#[test]
fn test_format64_case_exact_power_of_ten() {
    let f: f64 = 1e16; // This should return 10000000000000000.0
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len = unsafe { ryu::format64(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let printed = unsafe { std::str::from_utf8_unchecked(slice) };
    assert_eq!(printed, "10000000000000000.0");
}

