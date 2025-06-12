// Answer 0

#[test]
fn test_write_mantissa_zero() {
    let mut output: u32 = 0;
    let mut result: [u8; 10] = [0; 10];
    let result_ptr = result.as_mut_ptr();
    unsafe {
        write_mantissa(output, result_ptr);
    }
    assert_eq!(&result[8..], b"0");
}

#[test]
fn test_write_mantissa_one() {
    let mut output: u32 = 1;
    let mut result: [u8; 10] = [0; 10];
    let result_ptr = result.as_mut_ptr();
    unsafe {
        write_mantissa(output, result_ptr);
    }
    assert_eq!(&result[9..], b"1");
}

#[test]
fn test_write_mantissa_two() {
    let mut output: u32 = 2;
    let mut result: [u8; 10] = [0; 10];
    let result_ptr = result.as_mut_ptr();
    unsafe {
        write_mantissa(output, result_ptr);
    }
    assert_eq!(&result[9..], b"2");
}

#[test]
fn test_write_mantissa_nine() {
    let mut output: u32 = 9;
    let mut result: [u8; 10] = [0; 10];
    let result_ptr = result.as_mut_ptr();
    unsafe {
        write_mantissa(output, result_ptr);
    }
    assert_eq!(&result[9..], b"9");
}

