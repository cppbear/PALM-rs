// Answer 0

#[test]
fn test_write_vari32_negative_smallest() {
    let mut data = Vec::new();
    write_vari32(&mut data, -2147483648);
}

#[test]
fn test_write_vari32_negative_large() {
    let mut data = Vec::new();
    write_vari32(&mut data, -1);
}

#[test]
fn test_write_vari32_negative_mid() {
    let mut data = Vec::new();
    write_vari32(&mut data, -123456);
}

#[test]
fn test_write_vari32_negative_large_mid() {
    let mut data = Vec::new();
    write_vari32(&mut data, -1000000);
}

#[test]
fn test_write_vari32_negative_large_non_boundary() {
    let mut data = Vec::new();
    write_vari32(&mut data, -500000);
}

