// Answer 0

#[test]
#[should_panic]
fn test_write_exponent3_sign_true_k_1000() {
    let k: isize = -1000;
    let mut result: [u8; 5] = [0; 5];
    unsafe {
        write_exponent3(k, result.as_mut_ptr());
    }
}

