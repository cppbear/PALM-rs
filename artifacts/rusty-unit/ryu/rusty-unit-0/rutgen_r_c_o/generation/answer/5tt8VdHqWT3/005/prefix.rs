// Answer 0

#[test]
fn test_write_mantissa_output_10() {
    let mut buffer = [0u8; 10];
    let output: u32 = 10;
    unsafe {
        write_mantissa(output, buffer.as_mut_ptr().add(10));
    }
}

#[test]
fn test_write_mantissa_output_50() {
    let mut buffer = [0u8; 10];
    let output: u32 = 50;
    unsafe {
        write_mantissa(output, buffer.as_mut_ptr().add(10));
    }
}

#[test]
fn test_write_mantissa_output_99() {
    let mut buffer = [0u8; 10];
    let output: u32 = 99;
    unsafe {
        write_mantissa(output, buffer.as_mut_ptr().add(10));
    }
}

#[test]
fn test_write_mantissa_output_100() {
    let mut buffer = [0u8; 10];
    let output: u32 = 100;
    unsafe {
        write_mantissa(output, buffer.as_mut_ptr().add(10));
    }
}

