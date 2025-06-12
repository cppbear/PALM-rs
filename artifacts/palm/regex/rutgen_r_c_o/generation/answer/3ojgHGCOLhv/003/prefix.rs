// Answer 0

#[test]
fn test_read_varu32_empty() {
    let data: &[u8] = &[];
    read_varu32(data);
}

#[test]
fn test_read_varu32_single_byte_high_bit() {
    let data: &[u8] = &[0b1000_0000];
    read_varu32(data);
}

#[test]
fn test_read_varu32_multiple_bytes_high_bit() {
    let data: &[u8] = &[0b1000_0000, 0b1000_0000, 0b1000_0000];
    read_varu32(data);
}

#[test]
fn test_read_varu32_multiple_bytes_no_high_bit() {
    let data: &[u8] = &[0b0111_1111, 0b0111_1111];
    read_varu32(data);
}

#[test]
fn test_read_varu32_single_byte_no_high_bit() {
    let data: &[u8] = &[0b0111_1111];
    read_varu32(data);
}

#[test]
fn test_read_varu32_single_zero_byte() {
    let data: &[u8] = &[0];
    read_varu32(data);
}

