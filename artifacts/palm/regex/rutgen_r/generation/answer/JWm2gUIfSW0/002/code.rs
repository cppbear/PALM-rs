// Answer 0

#[test]
fn test_write_varu32_single_byte() {
    let mut data = Vec::new();
    write_varu32(&mut data, 0b0111_1111);
    assert_eq!(data, vec![0b0111_1111]);
}

#[test]
fn test_write_varu32_two_bytes() {
    let mut data = Vec::new();
    write_varu32(&mut data, 0b1000_0000);
    assert_eq!(data, vec![0b1000_0000 | 0b0000_0000, 0b0000_0000]);
}

#[test]
fn test_write_varu32_multiple_bytes() {
    let mut data = Vec::new();
    write_varu32(&mut data, 0b1000_0001);
    assert_eq!(data, vec![0b1000_0000 | 0b0000_0001, 0b0000_0000]);
}

#[test]
fn test_write_varu32_large_value() {
    let mut data = Vec::new();
    write_varu32(&mut data, 0xFFFFFFFF);
    assert_eq!(data, vec![0b1000_0000 | 0b1111_1111, 0b1000_0000 | 0b1111_1111, 0b1000_0000 | 0b1111_1111, 0b1111_1111]);
}

