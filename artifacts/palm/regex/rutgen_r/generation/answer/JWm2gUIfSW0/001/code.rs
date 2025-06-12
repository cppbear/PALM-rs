// Answer 0

#[test]
fn test_write_varu32_boundary_case_greater_than_or_equal() {
    let mut data = Vec::new();
    let n = 0b1000_0000; // boundary condition where n is exactly 128
    write_varu32(&mut data, n);
    assert_eq!(data, vec![0b1000_0000, 0]); // First byte has the continuation bit set, second byte is 0
}

#[test]
fn test_write_varu32_boundary_case_less_than() {
    let mut data = Vec::new();
    let n = 0b0111_1111; // boundary condition where n is 127 (less than 128)
    write_varu32(&mut data, n);
    assert_eq!(data, vec![127]); // Only one byte without continuation bit
}

