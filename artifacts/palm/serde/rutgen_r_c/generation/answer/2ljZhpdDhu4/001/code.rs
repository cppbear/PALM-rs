// Answer 0

#[test]
fn test_write_str_overflow_error() {
    let mut buffer: [u8; 5] = [0; 5];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 5, // Set offset to the full length of the buffer
    };

    let result = buf.write_str("hello"); // This exceeds the buffer capacity
    assert!(result.is_err());
}

#[test]
fn test_write_str_boundary_condition() {
    let mut buffer: [u8; 5] = [0; 5];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 0, // Set offset to 0
    };

    let result = buf.write_str("hello"); // This should succeed, filling the buffer
    assert!(result.is_ok());
    assert_eq!(&buffer, b"hello");
}

#[test]
fn test_write_str_partial_overflow_error() {
    let mut buffer: [u8; 5] = [0; 5];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 4, // Set offset close to the capacity
    };

    let result = buf.write_str("o"); // This should cause overflow
    assert!(result.is_err());
}

