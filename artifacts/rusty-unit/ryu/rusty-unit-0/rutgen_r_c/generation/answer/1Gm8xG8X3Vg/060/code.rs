// Answer 0

#[test]
fn test_format64_positive_normal_case() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let f: f64 = 3.141592653589793; // A positive normal float, not zero

    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "3.141592653589793"); // Check the expected output
    }
}

#[test]
fn test_format64_positive_large_exponent() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let f: f64 = 1.0e20; // A large positive number

    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1e20"); // Check the expected output
    }
}

#[test]
fn test_format64_positive_decimal() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let f: f64 = 12345.6789; // A positive float with both mantissa and exponent

    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "12345.6789"); // Check the expected output
    }
}

