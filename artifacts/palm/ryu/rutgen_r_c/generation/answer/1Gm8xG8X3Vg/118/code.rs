// Answer 0

#[test]
fn test_format64_zero() {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let result = unsafe { format64(0.0, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { slice::from_raw_parts(buffer.as_ptr() as *const u8, result) };
    let output = unsafe { str::from_utf8_unchecked(slice) };
    assert_eq!(output, "0.0");
}

#[test]
fn test_format64_negative_small_number() {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let result = unsafe { format64(-0.0001, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { slice::from_raw_parts(buffer.as_ptr() as *const u8, result) };
    let output = unsafe { str::from_utf8_unchecked(slice) };
    assert_eq!(output, "-0.0001");
}

#[test]
fn test_format64_large_exponent() {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let result = unsafe { format64(1e30, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { slice::from_raw_parts(buffer.as_ptr() as *const u8, result) };
    let output = unsafe { str::from_utf8_unchecked(slice) };
    assert_eq!(output, "1e30");
}

#[test]
fn test_format64_large_number() {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let result = unsafe { format64(123456789.0, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { slice::from_raw_parts(buffer.as_ptr() as *const u8, result) };
    let output = unsafe { str::from_utf8_unchecked(slice) };
    assert_eq!(output, "123456789");
}

