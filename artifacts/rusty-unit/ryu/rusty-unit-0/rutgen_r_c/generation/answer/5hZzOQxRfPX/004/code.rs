// Answer 0

#[test]
fn test_write_exponent2_positive_double_digit() {
    let k: isize = 10; // satisfies the condition k >= 10 and k < 100
    let mut result: [u8; 3] = [0; 3]; // space for the result, including sign
    let result_ptr = result.as_mut_ptr();

    unsafe {
        let return_value = write_exponent2(k, result_ptr);
        assert_eq!(return_value, 2); // sign is false, so return_value should be 0 + 2
        assert_eq!(&result[..2], b"10"); // result should be "10"
    }
}

#[test]
fn test_write_exponent2_negative_double_digit() {
    let k: isize = -10; // satisfies the condition k < 100
    let mut result: [u8; 4] = [0; 4]; // space for the result, including sign
    let result_ptr = result.as_mut_ptr();

    unsafe {
        let return_value = write_exponent2(k, result_ptr);
        assert_eq!(return_value, 3); // sign is true, so return_value should be 1 + 2
        assert_eq!(&result[..3], b"-10"); // result should be "-10"
    }
}

