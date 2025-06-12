// Answer 0

#[test]
fn test_write_mantissa_long_zero() {
    let mut result: [u8; 16] = [0; 16];
    unsafe {
        write_mantissa_long(0, result.as_mut_ptr().add(16));
    }
}

#[test]
fn test_write_mantissa_long_max() {
    let mut result: [u8; 16] = [0; 16];
    unsafe {
        write_mantissa_long(4_294_967_295, result.as_mut_ptr().add(16));
    }
}

#[test]
fn test_write_mantissa_long_smallest() {
    let mut result: [u8; 16] = [0; 16];
    unsafe {
        write_mantissa_long(1, result.as_mut_ptr().add(16));
    }
}

#[test]
fn test_write_mantissa_long_mid_value() {
    let mut result: [u8; 16] = [0; 16];
    unsafe {
        write_mantissa_long(2_147_483_647, result.as_mut_ptr().add(16));
    }
}

#[test]
fn test_write_mantissa_long_large_value() {
    let mut result: [u8; 16] = [0; 16];
    unsafe {
        write_mantissa_long(3_000_000_000, result.as_mut_ptr().add(16));
    }
}

#[test]
fn test_write_mantissa_long_edge() {
    let mut result: [u8; 16] = [0; 16];
    unsafe {
        write_mantissa_long(4_000_000_000, result.as_mut_ptr().add(16));
    }
}

