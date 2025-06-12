// Answer 0

#[test]
fn test_write_exponent3_positive() {
    let mut buffer = [0u8; 4]; // Buffer size is enough for "999"
    let result_pointer = buffer.as_mut_ptr();
    let length = unsafe { write_exponent3(123, result_pointer) };
    assert_eq!(length, 3);
    assert_eq!(&buffer[0..3], b"123");
}

#[test]
fn test_write_exponent3_positive_with_sign() {
    let mut buffer = [0u8; 4]; // Buffer size is enough for "-999"
    let result_pointer = buffer.as_mut_ptr();
    let length = unsafe { write_exponent3(456, result_pointer) };
    assert_eq!(length, 3);
    assert_eq!(&buffer[0..3], b"456");
}

#[test]
fn test_write_exponent3_negative() {
    let mut buffer = [0u8; 5]; // Buffer size is enough for "-999"
    let result_pointer = buffer.as_mut_ptr();
    let length = unsafe { write_exponent3(-789, result_pointer) };
    assert_eq!(length, 4);
    assert_eq!(&buffer[0..4], b"-789");
}

#[test]
fn test_write_exponent3_boundary_zero() {
    let mut buffer = [0u8; 2]; // Buffer size is enough for "0"
    let result_pointer = buffer.as_mut_ptr();
    let length = unsafe { write_exponent3(0, result_pointer) };
    assert_eq!(length, 1);
    assert_eq!(&buffer[0..1], b"0");
}

#[test]
fn test_write_exponent3_boundary_one() {
    let mut buffer = [0u8; 2]; // Buffer size is enough for "1"
    let result_pointer = buffer.as_mut_ptr();
    let length = unsafe { write_exponent3(1, result_pointer) };
    assert_eq!(length, 1);
    assert_eq!(&buffer[0..1], b"1");
}

#[test]
fn test_write_exponent3_boundary_nine() {
    let mut buffer = [0u8; 2]; // Buffer size is enough for "9"
    let result_pointer = buffer.as_mut_ptr();
    let length = unsafe { write_exponent3(9, result_pointer) };
    assert_eq!(length, 1);
    assert_eq!(&buffer[0..1], b"9");
}

#[test]
fn test_write_exponent3_boundary_ten() {
    let mut buffer = [0u8; 3]; // Buffer size is enough for "10"
    let result_pointer = buffer.as_mut_ptr();
    let length = unsafe { write_exponent3(10, result_pointer) };
    assert_eq!(length, 2);
    assert_eq!(&buffer[0..2], b"10");
}

#[test]
fn test_write_exponent3_boundary_one_hundred() {
    let mut buffer = [0u8; 3]; // Buffer size is enough for "100"
    let result_pointer = buffer.as_mut_ptr();
    let length = unsafe { write_exponent3(100, result_pointer) };
    assert_eq!(length, 3);
    assert_eq!(&buffer[0..3], b"100");
}

#[test]
fn test_write_exponent3_boundary_one_hundred_ninety_nine() {
    let mut buffer = [0u8; 4]; // Buffer size is enough for "199"
    let result_pointer = buffer.as_mut_ptr();
    let length = unsafe { write_exponent3(199, result_pointer) };
    assert_eq!(length, 3);
    assert_eq!(&buffer[0..3], b"199");
}

#[test]
fn test_write_exponent3_boundary_max_value() {
    let mut buffer = [0u8; 4]; // Buffer size is enough for "999"
    let result_pointer = buffer.as_mut_ptr();
    let length = unsafe { write_exponent3(999, result_pointer) };
    assert_eq!(length, 3);
    assert_eq!(&buffer[0..3], b"999");
}

