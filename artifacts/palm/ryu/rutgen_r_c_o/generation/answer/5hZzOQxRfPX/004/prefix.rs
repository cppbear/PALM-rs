// Answer 0

#[test]
fn test_write_exponent2_positive_k_equals_10() {
    let mut result: [u8; 3] = [0; 3];
    let result_ptr = result.as_mut_ptr();
    unsafe {
        let length = write_exponent2(10, result_ptr);
    }
}

#[test]
fn test_write_exponent2_positive_k_greater_than_10() {
    let mut result: [u8; 3] = [0; 3];
    let result_ptr = result.as_mut_ptr();
    unsafe {
        let length = write_exponent2(11, result_ptr);
    }
}

#[test]
fn test_write_exponent2_negative_k_greater_than_10() {
    let mut result: [u8; 3] = [0; 3];
    let result_ptr = result.as_mut_ptr();
    unsafe {
        let length = write_exponent2(-11, result_ptr);
    }
}

#[test]
fn test_write_exponent2_positive_k_less_than_10() {
    let mut result: [u8; 3] = [0; 3];
    let result_ptr = result.as_mut_ptr();
    unsafe {
        let length = write_exponent2(9, result_ptr);
    }
}

