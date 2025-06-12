// Answer 0

#[test]
fn test_write_mantissa_long_large_value() {
    use std::mem::MaybeUninit;

    let output: u64 = 1_000_000_000_000_000; // 1 quadrillion
    let mut buffer: [u8; 20] = [0; 20];
    let result = buffer.as_mut_ptr().add(19); // result pointer to the end of the buffer

    unsafe {
        write_mantissa_long(output, result);
    }

    assert_eq!(&buffer[10..20], b"1000000000000000000");
}

#[test]
fn test_write_mantissa_long_mid_value() {
    use std::mem::MaybeUninit;

    let output: u64 = 123_456_789_012; // Mid-range value
    let mut buffer: [u8; 20] = [0; 20];
    let result = buffer.as_mut_ptr().add(19); // result pointer to the end of the buffer

    unsafe {
        write_mantissa_long(output, result);
    }

    assert_eq!(&buffer[11..20], b"123456789012");
}

#[test]
fn test_write_mantissa_long_boundary_value() {
    use std::mem::MaybeUninit;

    let output: u64 = 4_294_967_296; // 2^32
    let mut buffer: [u8; 20] = [0; 20];
    let result = buffer.as_mut_ptr().add(19); // result pointer to the end of the buffer

    unsafe {
        write_mantissa_long(output, result);
    }

    assert_eq!(&buffer[11..20], b"4294967296");
}

#[test]
fn test_write_mantissa_long_max_value() {
    use std::mem::MaybeUninit;

    let output: u64 = u64::MAX; // Maximum value for u64
    let mut buffer: [u8; 20] = [0; 20];
    let result = buffer.as_mut_ptr().add(19); // result pointer to the end of the buffer

    unsafe {
        write_mantissa_long(output, result);
    }

    assert_eq!(&buffer[11..20], b"18446744073709551615");
}

