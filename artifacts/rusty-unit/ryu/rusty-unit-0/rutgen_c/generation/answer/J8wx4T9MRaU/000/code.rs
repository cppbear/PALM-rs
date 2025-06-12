// Answer 0

#[test]
fn test_write_mantissa_long_zero() {
    let output: u64 = 0;
    let mut result: [u8; 20] = [0; 20];
    let result_ptr = result.as_mut_ptr().offset(20);
    
    unsafe {
        write_mantissa_long(output, result_ptr);
    }

    assert_eq!(&result[12..20], b"0\0\0\0\0\0\0\0");
}

#[test]
fn test_write_mantissa_long_small_value() {
    let output: u64 = 12345;
    let mut result: [u8; 20] = [0; 20];
    let result_ptr = result.as_mut_ptr().offset(20);
    
    unsafe {
        write_mantissa_long(output, result_ptr);
    }

    assert_eq!(&result[16..20], b"12345\0");
}

#[test]
fn test_write_mantissa_long_large_value() {
    let output: u64 = 1234567890123456789;
    let mut result: [u8; 20] = [0; 20];
    let result_ptr = result.as_mut_ptr().offset(20);
    
    unsafe {
        write_mantissa_long(output, result_ptr);
    }

    assert_eq!(&result[8..20], b"12345678901234\0");
}

#[test]
fn test_write_mantissa_long_boundary_value() {
    let output: u64 = 100_000_000;
    let mut result: [u8; 20] = [0; 20];
    let result_ptr = result.as_mut_ptr().offset(20);
    
    unsafe {
        write_mantissa_long(output, result_ptr);
    }

    assert_eq!(&result[12..20], b"10000000\0");
}

#[test]
fn test_write_mantissa_long_max_value() {
    let output: u64 = u64::MAX;
    let mut result: [u8; 20] = [0; 20];
    let result_ptr = result.as_mut_ptr().offset(20);
    
    unsafe {
        write_mantissa_long(output, result_ptr);
    }

    assert_eq!(&result[0..20], b"18446744073709551615\0");
}

