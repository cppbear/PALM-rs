// Answer 0

#[test]
fn test_write_varu32_zero() {
    let mut data = Vec::new();
    write_varu32(&mut data, 0);
    assert_eq!(data, vec![0]);
}

#[test]
fn test_write_varu32_one() {
    let mut data = Vec::new();
    write_varu32(&mut data, 1);
    assert_eq!(data, vec![1]);
}

#[test]
fn test_write_varu32_two() {
    let mut data = Vec::new();
    write_varu32(&mut data, 2);
    assert_eq!(data, vec![2]);
}

#[test]
fn test_write_varu32_seven() {
    let mut data = Vec::new();
    write_varu32(&mut data, 7);
    assert_eq!(data, vec![7]);
}

#[test]
fn test_write_varu32_eight() {
    let mut data = Vec::new();
    write_varu32(&mut data, 8);
    assert_eq!(data, vec![8]);
}

#[test]
fn test_write_varu32_max() {
    let mut data = Vec::new();
    let n: u32 = std::u32::MAX;
    write_varu32(&mut data, n);
    assert_eq!(data, vec![0b1000_0000 | 0b1111_1111, 0b1000_0000 | 0b1111_1111, 0b1000_0000 | 0b1111_1111, 0b1111_1111]);
}

