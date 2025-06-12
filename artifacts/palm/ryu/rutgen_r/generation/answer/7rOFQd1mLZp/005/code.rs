// Answer 0

#[test]
fn test_write_exponent3_positive_three_digits() {
    let mut result: [u8; 4] = [0; 4]; // Buffer to hold the result
    let k: isize = 100; // k is set to 100, satisfying all constraints
    let result_ptr = result.as_mut_ptr();

    unsafe {
        let length = write_exponent3(k, result_ptr);
        assert_eq!(length, 3);
    }
    assert_eq!(&result[..3], b"100");
}

