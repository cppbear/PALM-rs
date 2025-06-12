// Answer 0

#[test]
unsafe fn test_write_mantissa_output_10000() {
    let mut buffer = [0u8; 10];
    let result_ptr = buffer.as_mut_ptr().add(10);
    write_mantissa(10_000, result_ptr);
}

#[test]
unsafe fn test_write_mantissa_output_below_10000() {
    let mut buffer = [0u8; 10];
    let result_ptr = buffer.as_mut_ptr().add(10);
    write_mantissa(9_999, result_ptr);
}

#[test]
unsafe fn test_write_mantissa_output_below_100() {
    let mut buffer = [0u8; 10];
    let result_ptr = buffer.as_mut_ptr().add(10);
    write_mantissa(99, result_ptr);
}

#[test]
unsafe fn test_write_mantissa_output_below_10() {
    let mut buffer = [0u8; 10];
    let result_ptr = buffer.as_mut_ptr().add(10);
    write_mantissa(9, result_ptr);
}

