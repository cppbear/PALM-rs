// Answer 0

#[test]
fn test_write_varu32_zero() {
    let mut data = Vec::new();
    write_varu32(&mut data, 0);
    assert_eq!(data, vec![0]);
}

#[test]
fn test_write_varu32_small_value() {
    let mut data = Vec::new();
    write_varu32(&mut data, 127);
    assert_eq!(data, vec![127]);
}

#[test]
fn test_write_varu32_boundary_value() {
    let mut data = Vec::new();
    write_varu32(&mut data, 128);
    assert_eq!(data, vec![0b1000_0000 | 0b0000_0000, 1]);
}

#[test]
fn test_write_varu32_large_value() {
    let mut data = Vec::new();
    write_varu32(&mut data, 16383);
    assert_eq!(data, vec![0b1000_0000 | 0b0011_1111, 63]);
}

#[test]
fn test_write_varu32_max_value() {
    let mut data = Vec::new();
    write_varu32(&mut data, 2_147_483_647); // Maximum u32 value
    assert_eq!(data, vec![
        0b1000_0000 | 0b1111_1111,  // 127
        0b1000_0000 | 0b1111_1111,  // 127
        0b1000_0000 | 0b1111_1111,  // 127
        0b1000_0000 | 0b1111_1111,  // 127
        0b1000_0000 | 0b1111_1111,  // 127
        0b1000_0000 | 0b1111_1111,  // 127
        0b1000_0000 | 0b1111_1111,  // 127
        0b1000_0000 | 0b0111_1111,  // 63
        255 // 255
    ]);
}

