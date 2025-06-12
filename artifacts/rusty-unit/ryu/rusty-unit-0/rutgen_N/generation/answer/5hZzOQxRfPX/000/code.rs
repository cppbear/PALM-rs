// Answer 0

#[test]
fn test_write_exponent2_positive_single_digit() {
    let mut buffer = [0u8; 2];
    let result_ptr = buffer.as_mut_ptr();
    let k: isize = 5;
    
    unsafe {
        let length = write_exponent2(k, result_ptr);
        assert_eq!(length, 1);
        assert_eq!(buffer[0], b'5');
    }
}

#[test]
fn test_write_exponent2_positive_double_digit() {
    let mut buffer = [0u8; 3];
    let result_ptr = buffer.as_mut_ptr();
    let k: isize = 25;

    unsafe {
        let length = write_exponent2(k, result_ptr);
        assert_eq!(length, 2);
        assert_eq!(buffer[0], b'2');
        assert_eq!(buffer[1], b'5');
    }
}

#[test]
fn test_write_exponent2_negative_single_digit() {
    let mut buffer = [0u8; 3];
    let result_ptr = buffer.as_mut_ptr();
    let k: isize = -3;

    unsafe {
        let length = write_exponent2(k, result_ptr);
        assert_eq!(length, 2);
        assert_eq!(buffer[0], b'-');
        assert_eq!(buffer[1], b'3');
    }
}

#[test]
fn test_write_exponent2_negative_double_digit() {
    let mut buffer = [0u8; 4];
    let result_ptr = buffer.as_mut_ptr();
    let k: isize = -45;

    unsafe {
        let length = write_exponent2(k, result_ptr);
        assert_eq!(length, 3);
        assert_eq!(buffer[0], b'-');
        assert_eq!(buffer[1], b'4');
        assert_eq!(buffer[2], b'5');
    }
}

#[test]
#[should_panic]
fn test_write_exponent2_out_of_bounds() {
    let mut buffer = [0u8; 2];
    let result_ptr = buffer.as_mut_ptr();
    let k: isize = 100;

    unsafe {
        write_exponent2(k, result_ptr); // This should trigger a panic due to debug_assert.
    }
}

