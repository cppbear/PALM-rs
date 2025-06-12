// Answer 0

#[test]
fn test_format64_zero() {
    let mut buffer = [0u8; 24];
    
    unsafe {
        let len = ryu::raw::format64(0.0, buffer.as_mut_ptr());
        let result = core::str::from_utf8_unchecked(&buffer[0..len]);
        assert_eq!(result, "0.0");
    }
}

#[test]
fn test_format64_negative_zero() {
    let mut buffer = [0u8; 24];
    
    unsafe {
        let len = ryu::raw::format64(-0.0, buffer.as_mut_ptr());
        let result = core::str::from_utf8_unchecked(&buffer[0..len]);
        assert_eq!(result, "-0.0");
    }
}

#[test]
fn test_format64_small_negative() {
    let mut buffer = [0u8; 24];
    
    unsafe {
        let len = ryu::raw::format64(-1e-324, buffer.as_mut_ptr());
        let result = core::str::from_utf8_unchecked(&buffer[0..len]);
        assert_eq!(result, "-0.000000000000000000000000000001");
    }
}

#[test]
fn test_format64_small_positive() {
    let mut buffer = [0u8; 24];
    
    unsafe {
        let len = ryu::raw::format64(1e-324, buffer.as_mut_ptr());
        let result = core::str::from_utf8_unchecked(&buffer[0..len]);
        assert_eq!(result, "0.000000000000000000000000000001");
    }
}

