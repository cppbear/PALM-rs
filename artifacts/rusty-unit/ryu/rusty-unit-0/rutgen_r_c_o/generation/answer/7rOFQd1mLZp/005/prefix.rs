// Answer 0

#[test]
fn test_write_exponent3_k_100() {
    let mut result: [u8; 4] = [0; 4];
    let k: isize = 100;
    let result_ptr = result.as_mut_ptr();
    unsafe {
        write_exponent3(k, result_ptr);
    }
}

#[test]
fn test_write_exponent3_k_101() {
    let mut result: [u8; 4] = [0; 4];
    let k: isize = 101;
    let result_ptr = result.as_mut_ptr();
    unsafe {
        write_exponent3(k, result_ptr);
    }
}

#[test]
fn test_write_exponent3_k_999() {
    let mut result: [u8; 4] = [0; 4];
    let k: isize = 999;
    let result_ptr = result.as_mut_ptr();
    unsafe {
        write_exponent3(k, result_ptr);
    }
}

