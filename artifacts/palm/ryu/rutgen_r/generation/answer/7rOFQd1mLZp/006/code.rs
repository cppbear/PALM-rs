// Answer 0

#[test]
fn test_write_exponent3_k_is_10_positive() {
    let mut buffer = [0u8; 3];
    let result_ptr = buffer.as_mut_ptr();
    let k: isize = 10;

    unsafe {
        let result_length = write_exponent3(k, result_ptr);
        
        assert_eq!(result_length, 2);
        assert_eq!(&buffer[..result_length as usize], b"10");
    }
}

