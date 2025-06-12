// Answer 0

#[test]
fn test_read_vari32_positive_integer() {
    let data = [0b00000001]; // Represents positive integer 0
    let (n, i) = read_vari32(&data);
    assert_eq!(n, 0);
    assert_eq!(i, 1);
}

#[test]
fn test_read_vari32_small_negative_integer() {
    let data = [0b00000010]; // Represents negative integer -1
    let (n, i) = read_vari32(&data);
    assert_eq!(n, -1);
    assert_eq!(i, 1);
}

#[test]
fn test_read_vari32_large_integer() {
    let data = [0b00000011, 0b00000001]; // Represents positive integer 2
    let (n, i) = read_vari32(&data);
    assert_eq!(n, 2);
    assert_eq!(i, 2);
}

#[test]
fn test_read_vari32_boundary_condition() {
    let data = [0b00000001, 0b10000000, 0b00000001]; // Represents integer 128
    let (n, i) = read_vari32(&data);
    assert_eq!(n, 128);
    assert_eq!(i, 3);
}

#[test]
fn test_read_vari32_large_negative_integer() {
    let data = [0b00000010, 0b10000000, 0b00000001]; // Represents negative integer -128
    let (n, i) = read_vari32(&data);
    assert_eq!(n, -128);
    assert_eq!(i, 3);
}

#[test]
fn test_read_vari32_multiple_bytes() {
    let data = [0b10000001, 0b00000010]; // Represents integer 129
    let (n, i) = read_vari32(&data);
    assert_eq!(n, 129);
    assert_eq!(i, 2);
}

#[test]
fn test_read_vari32_edge_case() {
    let data = [0b11111111, 0b00000001]; // Represents integer 127
    let (n, i) = read_vari32(&data);
    assert_eq!(n, 127);
    assert_eq!(i, 2);
}

