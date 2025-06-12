// Answer 0

#[test]
fn test_read_vari32_zero() {
    let data = [0b00000000]; // This will produce (0, 1)
    let (n, i) = read_vari32(&data);
    assert_eq!(n, 0);
    assert_eq!(i, 1);
}

#[test]
fn test_read_vari32_positive() {
    let data = [0b00000010]; // This will produce (1, 1)
    let (n, i) = read_vari32(&data);
    assert_eq!(n, 1);
    assert_eq!(i, 1);
}

#[test]
fn test_read_vari32_large() {
    let data = [0b11111110]; // Encoding a large positive number
    let (n, i) = read_vari32(&data);
    assert_eq!(n, 63);
    assert_eq!(i, 1);
}

#[test]
fn test_read_vari32_multiple_bytes() {
    let data = [0b00000010, 0b00000000]; // Minimum two-byte input
    let (n, i) = read_vari32(&data);
    assert_eq!(n, 1);
    assert_eq!(i, 1); // `i` should still reflect the same byte count
}

