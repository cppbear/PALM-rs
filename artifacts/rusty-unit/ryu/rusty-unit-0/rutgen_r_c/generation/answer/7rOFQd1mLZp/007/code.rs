// Answer 0

#[test]
fn test_write_exponent3_positive_single_digit() {
    // Allocate space for the result
    let mut result = [0u8; 4]; // Extra space for null terminator if needed
    let result_ptr = result.as_mut_ptr();

    // Test input where k is a single positive digit
    let k = 7; 
    unsafe {
        let length = write_exponent3(k, result_ptr);
        let output = std::str::from_utf8(&result[..length]).unwrap();
        assert_eq!(output, "7");
        assert_eq!(length, 1);
    }
}

#[test]
fn test_write_exponent3_positive_single_digit_boundary() {
    // Allocate space for the result
    let mut result = [0u8; 4]; // Extra space for null terminator if needed
    let result_ptr = result.as_mut_ptr();

    // Test input where k is a single positive digit
    let k = 0; 
    unsafe {
        let length = write_exponent3(k, result_ptr);
        let output = std::str::from_utf8(&result[..length]).unwrap();
        assert_eq!(output, "0");
        assert_eq!(length, 1);
    }
} 

#[test]
fn test_write_exponent3_positive_double_digit() {
    // Allocate space for the result
    let mut result = [0u8; 4]; // Extra space for null terminator if needed
    let result_ptr = result.as_mut_ptr();

    // Test input where k is a double-digit positive number
    let k = 23; 
    unsafe {
        let length = write_exponent3(k, result_ptr);
        let output = std::str::from_utf8(&result[..length]).unwrap();
        assert_eq!(output, "23");
        assert_eq!(length, 2);
    }
}

