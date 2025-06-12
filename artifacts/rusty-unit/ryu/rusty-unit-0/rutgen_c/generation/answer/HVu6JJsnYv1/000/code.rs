// Answer 0

#[test]
fn test_format32_positive() {
    let f: f32 = 1.234;
    let mut buffer = [0u8; 16];
    
    unsafe {
        let len = format32(f, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "1.234");
    }
}

#[test]
fn test_format32_negative() {
    let f: f32 = -1.234;
    let mut buffer = [0u8; 16];

    unsafe {
        let len = format32(f, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "-1.234");
    }
}

#[test]
fn test_format32_zero() {
    let f: f32 = 0.0;
    let mut buffer = [0u8; 16];

    unsafe {
        let len = format32(f, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "0.0");
    }
}

#[test]
fn test_format32_small_number() {
    let f: f32 = 0.001234;
    let mut buffer = [0u8; 16];

    unsafe {
        let len = format32(f, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "0.001234");
    }
}

#[test]
fn test_format32_large_number() {
    let f: f32 = 123456789.0;
    let mut buffer = [0u8; 16];

    unsafe {
        let len = format32(f, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "123456789.0");
    }
}

#[test]
fn test_format32_exponential() {
    let f: f32 = 12340000000.0;
    let mut buffer = [0u8; 16];

    unsafe {
        let len = format32(f, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "12340000000.0");
    }
}

#[test]
fn test_format32_boundary_exponential() {
    let f: f32 = 1e30; // Should check for large exponent case
    let mut buffer = [0u8; 16];

    unsafe {
        let len = format32(f, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert!(result.starts_with("1.0e"));
    }
}

