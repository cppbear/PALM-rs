// Answer 0

#[test]
fn test_write_mantissa_small_values() {
    let mut buffer = [0u8; 10];
    let ptr = buffer.as_mut_ptr();
    unsafe {
        write_mantissa(5, ptr.add(10)); // Writing 5 at the end of buffer
    }
    assert_eq!(&buffer[6..10], b"5\0\0\0");
}

#[test]
fn test_write_mantissa_tens() {
    let mut buffer = [0u8; 10];
    let ptr = buffer.as_mut_ptr();
    unsafe {
        write_mantissa(42, ptr.add(10)); // Writing 42 at the end of buffer
    }
    assert_eq!(&buffer[6..10], b"42\0\0");
}

#[test]
fn test_write_mantissa_hundreds() {
    let mut buffer = [0u8; 10];
    let ptr = buffer.as_mut_ptr();
    unsafe {
        write_mantissa(356, ptr.add(10)); // Writing 356 at the end of buffer
    }
    assert_eq!(&buffer[6..10], b"356\0");
}

#[test]
fn test_write_mantissa_thousands() {
    let mut buffer = [0u8; 15];
    let ptr = buffer.as_mut_ptr();
    unsafe {
        write_mantissa(12345, ptr.add(15)); // Writing 12345 at the end of buffer
    }
    assert_eq!(&buffer[11..15], b"12345");
}

#[test]
fn test_write_mantissa_large_values() {
    let mut buffer = [0u8; 20];
    let ptr = buffer.as_mut_ptr();
    unsafe {
        write_mantissa(1234567890, ptr.add(20)); // Writing a larger number
    }
    assert_eq!(&buffer[16..20], b"1234567890");
}

