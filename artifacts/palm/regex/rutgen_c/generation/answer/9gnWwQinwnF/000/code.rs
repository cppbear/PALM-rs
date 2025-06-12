// Answer 0

#[test]
fn test_write_vari32_positive() {
    let mut data = Vec::new();
    write_vari32(&mut data, 42);
    assert_eq!(data, vec![84]); // Result for write_varu32(42 << 1)
}

#[test]
fn test_write_vari32_negative() {
    let mut data = Vec::new();
    write_vari32(&mut data, -42);
    assert_eq!(data, vec![0b10010001]); // Result for write_varu32(!((-42 << 1)))
}

#[test]
fn test_write_vari32_zero() {
    let mut data = Vec::new();
    write_vari32(&mut data, 0);
    assert_eq!(data, vec![0]); // Result for write_varu32(0)
}

#[test]
fn test_write_vari32_large() {
    let mut data = Vec::new();
    write_vari32(&mut data, 127);
    assert_eq!(data, vec![254]); // Result for write_varu32(127 << 1)
}

#[test]
fn test_write_vari32_large_negative() {
    let mut data = Vec::new();
    write_vari32(&mut data, -128);
    assert_eq!(data, vec![0b10001110]); // Result for write_varu32(!(-128 << 1))
}

