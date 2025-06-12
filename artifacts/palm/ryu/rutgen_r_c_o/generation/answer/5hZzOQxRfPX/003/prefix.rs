// Answer 0

#[test]
#[should_panic]
fn test_write_exponent2_k_equals_100() {
    let mut result: [u8; 4] = [0; 4];
    let result_ptr = result.as_mut_ptr();
    unsafe {
        write_exponent2(100, result_ptr);
    }
}

#[test]
fn test_write_exponent2_k_equals_negative_100() {
    let mut result: [u8; 4] = [0; 4];
    let result_ptr = result.as_mut_ptr();
    unsafe {
        let _ = write_exponent2(-100, result_ptr);
    }
}

