// Answer 0

#[test]
fn test_write_exponent3_positive_ten() {
    let mut result: [u8; 4] = [0; 4];
    let mut result_ptr = result.as_mut_ptr();
    let k: isize = 10;

    unsafe {
        let _ = write_exponent3(k, result_ptr);
    }
}

#[test]
fn test_write_exponent3_positive_between_ten_and_hundred() {
    let mut result: [u8; 4] = [0; 4];
    let mut result_ptr = result.as_mut_ptr();
    let k: isize = 50;

    unsafe {
        let _ = write_exponent3(k, result_ptr);
    }
}

