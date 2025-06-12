// Answer 0

#[test]
fn test_write_vari32_negative() {
    let mut data = Vec::new();
    let n: i32 = -1;
    write_vari32(&mut data, n);
    assert_eq!(data, vec![0x01]); // Expected output for n = -1
}

#[test]
fn test_write_vari32_negative_large() {
    let mut data = Vec::new();
    let n: i32 = -123456; // A larger negative value
    write_vari32(&mut data, n);
    assert_eq!(data, vec![0xD0, 0xE9, 0x04]); // Adjust based on expected output for n = -123456
}

#[test]
fn test_write_vari32_negative_boundary() {
    let mut data = Vec::new();
    let n: i32 = -2147483648; // Minimum i32 value, testing the lower boundary
    write_vari32(&mut data, n);
    assert_eq!(data, vec![0x00, 0x01, 0x00, 0x00, 0x00]); // Adjust based on expected output for n = -2147483648
}

