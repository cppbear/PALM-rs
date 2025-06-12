// Answer 0

#[test]
fn test_format64_negative_zero() {
    let f = -0.0f64;
    let mut buffer = [0u8; 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr());
        assert_eq!(len, 3);
        assert_eq!(&buffer[..len], b"-0.0");
    }
}

#[test]
fn test_format64_negative_smallest_subnormal() {
    let f = -5.0e-324; // Smallest positive subnormal
    let mut buffer = [0u8; 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr());
        assert_eq!(len, 6);
        assert_eq!(&buffer[..len], b"-0.000005");
    }
}

#[test]
fn test_format64_negative_large_exponent() {
    let f = -1.0e30; // A value with a large negative exponent
    let mut buffer = [0u8; 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr());
        assert_eq!(len, 12);
        assert_eq!(&buffer[..len], b"-1.0e30");
    }
}

#[test]
fn test_format64_negative_small_mantissa_large_exponent() {
    let f = -1.234e-6; // Negative small mantissa with a negative exponent
    let mut buffer = [0u8; 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr());
        assert_eq!(len, 8);
        assert_eq!(&buffer[..len], b"-0.000001234");
    }
}

#[test]
fn test_format64_large_negative_value_exponent() {
    let f = -1.234e6; // Negative value with positive exponent
    let mut buffer = [0u8; 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr());
        assert_eq!(len, 13);
        assert_eq!(&buffer[..len], b"-1.234000e6");
    }
}

