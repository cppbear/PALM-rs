// Answer 0

#[test]
fn test_write_mantissa_small_output() {
    let mut output: u32 = 5;
    let mut result = [0u8; 10];
    let result_ptr = result.as_mut_ptr();

    unsafe {
        write_mantissa(output, result_ptr.add(10));
    }

    assert_eq!(&result[8..10], b"05");
}

#[test]
fn test_write_mantissa_medium_output() {
    let mut output: u32 = 123;
    let mut result = [0u8; 10];
    let result_ptr = result.as_mut_ptr();

    unsafe {
        write_mantissa(output, result_ptr.add(10));
    }

    assert_eq!(&result[8..10], b"23");
}

#[test]
fn test_write_mantissa_large_output() {
    let mut output: u32 = 100_000;
    let mut result = [0u8; 12];
    let result_ptr = result.as_mut_ptr();

    unsafe {
        write_mantissa(output, result_ptr.add(12));
    }

    assert_eq!(&result[8..12], b"1000");
}

#[test]
fn test_write_mantissa_boundary_output() {
    let mut output: u32 = 10_000;
    let mut result = [0u8; 12];
    let result_ptr = result.as_mut_ptr();

    unsafe {
        write_mantissa(output, result_ptr.add(12));
    }

    assert_eq!(&result[8..12], b"100");
}

#[test]
fn test_write_mantissa_huge_output() {
    let mut output: u32 = 1_000_000;
    let mut result = [0u8; 14];
    let result_ptr = result.as_mut_ptr();

    unsafe {
        write_mantissa(output, result_ptr.add(14));
    }

    assert_eq!(&result[8..14], b"10000");
}

