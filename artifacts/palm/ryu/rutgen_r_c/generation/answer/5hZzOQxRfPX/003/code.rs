// Answer 0

#[test]
#[should_panic]
fn test_write_exponent2_negative_k_within_bounds() {
    let mut buffer = [0u8; 3]; // Enough space for negative sign and two digits
    let result_ptr = buffer.as_mut_ptr();
    unsafe {
        let length = write_exponent2(-25, result_ptr);
        assert_eq!(length, 3);
        assert_eq!(&buffer[..length as usize], b"-25");
    }
}

#[test]
#[should_panic]
fn test_write_exponent2_positive_k_without_sign() {
    let mut buffer = [0u8; 2]; // Enough space for two digits
    let result_ptr = buffer.as_mut_ptr();
    unsafe {
        let length = write_exponent2(42, result_ptr);
        assert_eq!(length, 2);
        assert_eq!(&buffer[..length as usize], b"42");
    }
}

#[test]
#[should_panic]
fn test_write_exponent2_k_at_upper_bound() {
    let mut buffer = [0u8; 2]; // Only valid for two digits
    let result_ptr = buffer.as_mut_ptr();
    unsafe {
        let _ = write_exponent2(100, result_ptr);
    }
}

