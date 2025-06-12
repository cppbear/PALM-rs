// Answer 0

#[test]
fn test_write_exponent3_negative() {
    let mut buffer: [u8; 5] = [0; 5];
    let result = unsafe { write_exponent3(-123, buffer.as_mut_ptr()) };
    assert_eq!(result, 4);
    assert_eq!(&buffer[0..4], b"-123");
}

#[test]
fn test_write_exponent3_positive_large() {
    let mut buffer: [u8; 5] = [0; 5];
    let result = unsafe { write_exponent3(456, buffer.as_mut_ptr()) };
    assert_eq!(result, 3);
    assert_eq!(&buffer[0..3], b"456");
}

#[test]
fn test_write_exponent3_positive_double_digit() {
    let mut buffer: [u8; 5] = [0; 5];
    let result = unsafe { write_exponent3(78, buffer.as_mut_ptr()) };
    assert_eq!(result, 2);
    assert_eq!(&buffer[0..2], b"78");
}

#[test]
fn test_write_exponent3_positive_single_digit() {
    let mut buffer: [u8; 5] = [0; 5];
    let result = unsafe { write_exponent3(5, buffer.as_mut_ptr()) };
    assert_eq!(result, 1);
    assert_eq!(&buffer[0..1], b"5");
}

#[test]
fn test_write_exponent3_zero() {
    let mut buffer: [u8; 5] = [0; 5];
    let result = unsafe { write_exponent3(0, buffer.as_mut_ptr()) };
    assert_eq!(result, 1);
    assert_eq!(&buffer[0..1], b"0");
}

