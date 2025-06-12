// Answer 0

#[test]
fn test_write_exponent2_positive_single_digit() {
    let mut result: [u8; 3] = [0; 3]; // Space for one digit plus null terminator
    let ptr = result.as_mut_ptr();

    let k: isize = 5; // k is a positive single digit
    let length = unsafe { write_exponent2(k, ptr) };

    assert_eq!(length, 1);
    assert_eq!(&result[..1], b"5");
}

#[test]
fn test_write_exponent2_negative_single_digit() {
    let mut result: [u8; 4] = [0; 4]; // Space for one digit, the sign, plus null terminator
    let ptr = result.as_mut_ptr();

    let k: isize = -3; // k is a negative single digit
    let length = unsafe { write_exponent2(k, ptr) };

    assert_eq!(length, 2);
    assert_eq!(&result[..2], b"-3");
}

