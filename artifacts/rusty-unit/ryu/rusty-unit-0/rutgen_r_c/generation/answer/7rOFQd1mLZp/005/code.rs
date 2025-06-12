// Answer 0

#[test]
fn test_write_exponent3_positive_k_100() {
    let mut result: [u8; 4] = [0; 4]; // Sufficient space for the output
    let ptr = result.as_mut_ptr();

    // Test input where k = 100
    unsafe {
        let return_value = write_exponent3(100, ptr);
        assert_eq!(return_value, 3); // Expected return value: sign(0) + 3

        // Validate the result buffer contains the correct ASCII representation of "100"
        assert_eq!(&result[..3], b"100"); // Expecting "100"
    }
}

#[test]
fn test_write_exponent3_positive_k_200() {
    let mut result: [u8; 4] = [0; 4]; // Sufficient space for the output
    let ptr = result.as_mut_ptr();

    // Test input where k = 200
    unsafe {
        let return_value = write_exponent3(200, ptr);
        assert_eq!(return_value, 3); // Expected return value: sign(0) + 3

        // Validate the result buffer contains the correct ASCII representation of "200"
        assert_eq!(&result[..3], b"200"); // Expecting "200"
    }
}

#[test]
fn test_write_exponent3_positive_k_999() {
    let mut result: [u8; 4] = [0; 4]; // Sufficient space for the output
    let ptr = result.as_mut_ptr();

    // Test input where k = 999
    unsafe {
        let return_value = write_exponent3(999, ptr);
        assert_eq!(return_value, 3); // Expected return value: sign(0) + 3

        // Validate the result buffer contains the correct ASCII representation of "999"
        assert_eq!(&result[..3], b"999"); // Expecting "999"
    }
}

