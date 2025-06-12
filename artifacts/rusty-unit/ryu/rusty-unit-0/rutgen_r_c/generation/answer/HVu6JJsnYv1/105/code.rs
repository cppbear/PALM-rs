// Answer 0

#[test]
fn test_format32_positive_zero() {
    let f: f32 = 0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = unsafe { ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = unsafe { std::str::from_utf8_unchecked(slice) };
    assert_eq!(print, "0.0");
}

#[test]
fn test_format32_negative_zero() {
    let f: f32 = -0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = unsafe { ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = unsafe { std::str::from_utf8_unchecked(slice) };
    assert_eq!(print, "-0.0");
}

#[test]
fn test_format32_large_negative_number() {
    let f: f32 = -100000000.0; // k == -45 condition
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = unsafe { ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = unsafe { std::str::from_utf8_unchecked(slice) };
    assert_eq!(print, "-1e8");
}

#[test]
fn test_format32_small_number() {
    let f: f32 = 0.0000001; // 0 < kk && kk <= 13 condition
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = unsafe { ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = unsafe { std::str::from_utf8_unchecked(slice) };
    assert_eq!(print, "1e-7");
}

