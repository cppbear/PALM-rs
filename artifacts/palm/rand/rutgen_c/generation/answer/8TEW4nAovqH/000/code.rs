// Answer 0

#[test]
fn test_to_le_bytes_u64() {
    let value: u64 = 0x1122334455667788;
    let expected: [u8; 8] = [0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11];
    let result = value.to_le_bytes();
    assert_eq!(result.as_ref(), &expected);
}

#[test]
fn test_to_le_bytes_u64_zero() {
    let value: u64 = 0;
    let expected: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let result = value.to_le_bytes();
    assert_eq!(result.as_ref(), &expected);
}

#[test]
fn test_to_le_bytes_u64_boundary() {
    let value: u64 = u64::MAX;
    let expected: [u8; 8] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let result = value.to_le_bytes();
    assert_eq!(result.as_ref(), &expected);
}

