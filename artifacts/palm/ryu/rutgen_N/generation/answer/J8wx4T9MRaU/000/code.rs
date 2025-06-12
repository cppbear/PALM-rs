// Answer 0

#[test]
fn test_write_mantissa_long_small_value() {
    let mut buffer = [0u8; 16];
    let result_ptr = buffer.as_mut_ptr().add(16);
    unsafe {
        write_mantissa_long(123456789, result_ptr);
    }
    assert_eq!(&buffer[8..], b"123456789\0");
}

#[test]
fn test_write_mantissa_long_large_value() {
    let mut buffer = [0u8; 16];
    let result_ptr = buffer.as_mut_ptr().add(16);
    unsafe {
        write_mantissa_long(9876543210123456789, result_ptr);
    }
    assert_eq!(&buffer[0..], b"9876543210123456789\0");
}

#[test]
fn test_write_mantissa_long_zero_value() {
    let mut buffer = [0u8; 16];
    let result_ptr = buffer.as_mut_ptr().add(16);
    unsafe {
        write_mantissa_long(0, result_ptr);
    }
    assert_eq!(&buffer[8..], b"0\0");
}

