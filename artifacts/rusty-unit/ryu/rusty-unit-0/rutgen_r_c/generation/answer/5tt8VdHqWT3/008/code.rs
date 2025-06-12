// Answer 0

#[test]
fn test_write_mantissa_zero() {
    let mut buffer: [u8; 20] = [0; 20];
    let result_ptr = buffer.as_mut_ptr().add(20); // Point to the end of the array

    unsafe {
        write_mantissa(0, result_ptr);
    }

    assert_eq!(&buffer[18..20], &[b'0', 0]);
}

#[test]
fn test_write_mantissa_one() {
    let mut buffer: [u8; 20] = [0; 20];
    let result_ptr = buffer.as_mut_ptr().add(20); // Point to the end of the array

    unsafe {
        write_mantissa(1, result_ptr);
    }

    assert_eq!(&buffer[19..20], &[b'1']);
}

#[test]
fn test_write_mantissa_nine() {
    let mut buffer: [u8; 20] = [0; 20];
    let result_ptr = buffer.as_mut_ptr().add(20); // Point to the end of the array

    unsafe {
        write_mantissa(9, result_ptr);
    }

    assert_eq!(&buffer[19..20], &[b'9']);
}

#[test]
fn test_write_mantissa_ten() {
    let mut buffer: [u8; 20] = [0; 20];
    let result_ptr = buffer.as_mut_ptr().add(20); // Point to the end of the array

    unsafe {
        write_mantissa(10, result_ptr);
    }

    assert_eq!(&buffer[18..20], &[b'1', b'0']);
}

#[test]
fn test_write_mantissa_eleven() {
    let mut buffer: [u8; 20] = [0; 20];
    let result_ptr = buffer.as_mut_ptr().add(20); // Point to the end of the array

    unsafe {
        write_mantissa(11, result_ptr);
    }

    assert_eq!(&buffer[18..20], &[b'1', b'1']);
}

