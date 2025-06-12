// Answer 0

#[test]
fn test_write_mantissa_below_threshold() {
    let output: u32 = 9999;
    let mut buffer = [0u8; 10]; // Buffer for the result
    let result_ptr = buffer.as_mut_ptr().offset(10); // Start at the end of the buffer

    unsafe {
        write_mantissa(output, result_ptr);
    }
    
    assert_eq!(&buffer[8..10], b"99"); // Expected output when output == 9999
}

#[test]
fn test_write_mantissa_at_hundred() {
    let output: u32 = 100;
    let mut buffer = [0u8; 10]; // Buffer for the result
    let result_ptr = buffer.as_mut_ptr().offset(10); // Start at the end of the buffer

    unsafe {
        write_mantissa(output, result_ptr);
    }
    
    assert_eq!(&buffer[8..10], b"00"); // Expected output when output == 100
}

#[test]
fn test_write_mantissa_at_ten() {
    let output: u32 = 10;
    let mut buffer = [0u8; 10]; // Buffer for the result
    let result_ptr = buffer.as_mut_ptr().offset(10); // Start at the end of the buffer

    unsafe {
        write_mantissa(output, result_ptr);
    }
    
    assert_eq!(buffer[9], b'0' + 10); // Expected output when output == 10
}

