// Answer 0

#[test]
fn test_write_varu32_zero() {
    let mut data = Vec::new();
    write_varu32(&mut data, 0);
}

#[test]
fn test_write_varu32_max_below_varint() {
    let mut data = Vec::new();
    write_varu32(&mut data, 127);
}

#[test]
fn test_write_varu32_small_values() {
    let mut data = Vec::new();
    write_varu32(&mut data, 1);
    write_varu32(&mut data, 63);
}

#[test]
fn test_write_varu32_boundary_value() {
    let mut data = Vec::new();
    write_varu32(&mut data, 128);
}

