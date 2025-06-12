// Answer 0

#[test]
fn test_write_varu32_single_byte() {
    let mut data = Vec::new();
    write_varu32(&mut data, 127);
    assert_eq!(data, vec![127]);
}

#[test]
fn test_write_varu32_two_bytes() {
    let mut data = Vec::new();
    write_varu32(&mut data, 128);
    assert_eq!(data, vec![0b1000_0000 | 1, 0]);
}

#[test]
fn test_write_varu32_three_bytes() {
    let mut data = Vec::new();
    write_varu32(&mut data, 16384);
    assert_eq!(data, vec![0b1000_0000 | 128, 64]);
}

#[test]
fn test_write_varu32_four_bytes() {
    let mut data = Vec::new();
    write_varu32(&mut data, 2097152);
    assert_eq!(data, vec![0b1000_0000 | 16, 32, 0]);
}

#[test]
fn test_write_varu32_edge_case() {
    let mut data = Vec::new();
    write_varu32(&mut data, 0);
    assert_eq!(data, vec![0]);
}

