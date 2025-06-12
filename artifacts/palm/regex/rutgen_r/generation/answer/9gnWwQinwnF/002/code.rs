// Answer 0

#[test]
fn test_write_vari32_zero() {
    let mut data = Vec::new();
    write_vari32(&mut data, 0);
    assert_eq!(data, vec![0]); // Expected output for write_varu32(0)
}

#[test]
fn test_write_vari32_positive() {
    let mut data = Vec::new();
    write_vari32(&mut data, 1);
    assert_eq!(data, vec![2]); // Assuming write_varu32(2) is the expected result for n == 1

    let mut data = Vec::new();
    write_vari32(&mut data, 2);
    assert_eq!(data, vec![4]); // Assuming write_varu32(4) is the expected result for n == 2

    let mut data = Vec::new();
    write_vari32(&mut data, 127);
    assert_eq!(data, vec![254]); // Assuming write_varu32(254) is the expected result for n == 127
}

#[test]
fn test_write_vari32_large_positive() {
    let mut data = Vec::new();
    write_vari32(&mut data, 2147483647); // Maximum positive value for i32
    assert_eq!(data, vec![255, 255, 255, 255, 15]); // Assuming write_varu32 output for the max value
}

