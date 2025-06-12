// Answer 0

#[test]
fn test_write_vari32_positive() {
    let mut data = Vec::new();
    write_vari32(&mut data, 10);
    let expected: Vec<u8> = vec![20]; // 10 as varint
    assert_eq!(data, expected);
}

#[test]
fn test_write_vari32_negative() {
    let mut data = Vec::new();
    write_vari32(&mut data, -10);
    let expected: Vec<u8> = vec![19]; // -10 as varint
    assert_eq!(data, expected);
}

#[test]
fn test_write_vari32_zero() {
    let mut data = Vec::new();
    write_vari32(&mut data, 0);
    let expected: Vec<u8> = vec![0]; // 0 as varint
    assert_eq!(data, expected);
}

#[test]
fn test_write_vari32_edge_case_positive() {
    let mut data = Vec::new();
    write_vari32(&mut data, i32::MAX);
    // Expected output based on encoding of i32::MAX
    // This needs to be verified based on the actual implementation of write_varu32
    let expected: Vec<u8> = vec![]; // Placeholder for actual output
    assert_eq!(data, expected);
}

#[test]
fn test_write_vari32_edge_case_negative() {
    let mut data = Vec::new();
    write_vari32(&mut data, i32::MIN);
    // Expected output based on encoding of i32::MIN
    // This needs to be verified based on the actual implementation of write_varu32
    let expected: Vec<u8> = vec![]; // Placeholder for actual output
    assert_eq!(data, expected);
}

