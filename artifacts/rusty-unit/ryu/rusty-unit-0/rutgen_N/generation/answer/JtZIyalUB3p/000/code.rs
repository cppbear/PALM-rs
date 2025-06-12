// Answer 0

#[test]
fn test_format_finite_negative() {
    struct Buffer {
        bytes: [u8; 32],
    }
    
    impl Buffer {
        fn new() -> Self {
            Buffer { bytes: [0; 32] }
        }
    }
    
    let mut buffer = Buffer::new();
    let result = buffer.format_finite(-1.23);
    assert_eq!(result, "-1.23");
}

#[test]
fn test_format_finite_zero() {
    struct Buffer {
        bytes: [u8; 32],
    }
    
    impl Buffer {
        fn new() -> Self {
            Buffer { bytes: [0; 32] }
        }
    }
    
    let mut buffer = Buffer::new();
    let result = buffer.format_finite(0.0);
    assert_eq!(result, "0");
}

#[test]
fn test_format_finite_large_number() {
    struct Buffer {
        bytes: [u8; 32],
    }
    
    impl Buffer {
        fn new() -> Self {
            Buffer { bytes: [0; 32] }
        }
    }
    
    let mut buffer = Buffer::new();
    let result = buffer.format_finite(123456789.123);
    assert_eq!(result, "123456789.123");
}

#[test]
fn test_format_finite_small_number() {
    struct Buffer {
        bytes: [u8; 32],
    }
    
    impl Buffer {
        fn new() -> Self {
            Buffer { bytes: [0; 32] }
        }
    }
    
    let mut buffer = Buffer::new();
    let result = buffer.format_finite(0.0000123);
    assert_eq!(result, "0.0000123");
}

#[test]
fn test_format_finite_boundary() {
    struct Buffer {
        bytes: [u8; 32],
    }
    
    impl Buffer {
        fn new() -> Self {
            Buffer { bytes: [0; 32] }
        }
    }
    
    let mut buffer = Buffer::new();
    let result = buffer.format_finite(1.2345678901234567);
    assert_eq!(result, "1.2345678901234567");
}

