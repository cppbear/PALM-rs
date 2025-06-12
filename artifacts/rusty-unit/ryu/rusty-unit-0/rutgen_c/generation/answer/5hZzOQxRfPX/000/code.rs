// Answer 0

#[test]
fn test_write_exponent2_positive_single_digit() {
    let mut buffer = [0u8; 3]; // Enough space for one digit and null terminator
    let result = unsafe { write_exponent2(5, buffer.as_mut_ptr()) };
    assert_eq!(result, 1);
    assert_eq!(buffer[..1], [b'5']);
}

#[test]
fn test_write_exponent2_positive_double_digits() {
    let mut buffer = [0u8; 3]; // Enough space for two digits and null terminator
    let result = unsafe { write_exponent2(12, buffer.as_mut_ptr()) };
    assert_eq!(result, 2);
    assert_eq!(&buffer[..2], b"12");
}

#[test]
fn test_write_exponent2_negative_single_digit() {
    let mut buffer = [0u8; 4]; // Enough space for '-' and one digit
    let result = unsafe { write_exponent2(-7, buffer.as_mut_ptr()) };
    assert_eq!(result, 2);
    assert_eq!(&buffer[..2], b"-7");
}

#[test]
fn test_write_exponent2_negative_double_digits() {
    let mut buffer = [0u8; 4]; // Enough space for '-' and two digits
    let result = unsafe { write_exponent2(-21, buffer.as_mut_ptr()) };
    assert_eq!(result, 3);
    assert_eq!(&buffer[..3], b"-21");
}

#[test]
#[should_panic]
fn test_write_exponent2_out_of_bounds() {
    let mut buffer = [0u8; 3]; // Buffer size for handling up to 99
    unsafe { write_exponent2(100, buffer.as_mut_ptr()) }; // This should panic due to debug_assert
}

