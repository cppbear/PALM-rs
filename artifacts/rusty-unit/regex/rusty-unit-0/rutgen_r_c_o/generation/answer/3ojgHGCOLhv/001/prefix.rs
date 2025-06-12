// Answer 0

#[test]
fn test_read_varu32_single_byte() {
    let data: &[u8] = &[0b0000_0001];
    read_varu32(data);
}

#[test]
fn test_read_varu32_multiple_bytes() {
    let data: &[u8] = &[0b0111_1111, 0b0000_0010];
    read_varu32(data);
}

#[test]
fn test_read_varu32_boundary_value() {
    let data: &[u8] = &[0b0111_1111];
    read_varu32(data);
}

#[test]
fn test_read_varu32_large_value() {
    let data: &[u8] = &[0b1000_0000, 0b0000_0001, 0b0000_0001];
    read_varu32(data);
}

#[test]
fn test_read_varu32_zero_value() {
    let data: &[u8] = &[0b0000_0000];
    read_varu32(data);
}

