// Answer 0

#[test]
#[should_panic]
fn test_write_exponent3_panic_k_equals_1000() {
    use std::ptr;

    let mut buffer = [0u8; 5];
    let result_ptr = buffer.as_mut_ptr();

    unsafe {
        let length = write_exponent3(1000, result_ptr);
        // This should not be reached as it is expected to panic
        assert_eq!(length, 0);
    }
}

#[test]
fn test_write_exponent3_negative_k() {
    use std::ptr;

    let mut buffer = [0u8; 5];
    let result_ptr = buffer.as_mut_ptr();

    unsafe {
        let length = write_exponent3(-999, result_ptr);
        assert_eq!(length, 4);
        assert_eq!(&buffer[0..4], b"-999");
    }
}

#[test]
fn test_write_exponent3_small_positive_k() {
    use std::ptr;

    let mut buffer = [0u8; 5];
    let result_ptr = buffer.as_mut_ptr();

    unsafe {
        let length = write_exponent3(2, result_ptr);
        assert_eq!(length, 1);
        assert_eq!(&buffer[0..1], b"2");
    }
}

#[test]
fn test_write_exponent3_double_digit_k() {
    use std::ptr;

    let mut buffer = [0u8; 5];
    let result_ptr = buffer.as_mut_ptr();

    unsafe {
        let length = write_exponent3(23, result_ptr);
        assert_eq!(length, 2);
        assert_eq!(&buffer[0..2], b"23");
    }
}

#[test]
fn test_write_exponent3_three_digit_k() {
    use std::ptr;

    let mut buffer = [0u8; 5];
    let result_ptr = buffer.as_mut_ptr();

    unsafe {
        let length = write_exponent3(456, result_ptr);
        assert_eq!(length, 3);
        assert_eq!(&buffer[0..3], b"456");
    }
}

