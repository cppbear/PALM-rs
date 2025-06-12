// Answer 0

#[test]
fn test_format_u8_single_digit() {
    let mut buffer = [0u8; 3];
    let result = format_u8(5, &mut buffer);
    assert_eq!(result, 1);
    assert_eq!(&buffer[..1], b"5");
}

#[test]
fn test_format_u8_double_digit() {
    let mut buffer = [0u8; 3];
    let result = format_u8(23, &mut buffer);
    assert_eq!(result, 2);
    assert_eq!(&buffer[..2], b"23");
}

#[test]
fn test_format_u8_triple_digit() {
    let mut buffer = [0u8; 3];
    let result = format_u8(100, &mut buffer);
    assert_eq!(result, 3);
    assert_eq!(&buffer[..3], b"100");
}

#[test]
fn test_format_u8_max_value() {
    let mut buffer = [0u8; 3];
    let result = format_u8(255, &mut buffer);
    assert_eq!(result, 3);
    assert_eq!(&buffer[..3], b"255");
}

#[test]
fn test_format_u8_zero() {
    let mut buffer = [0u8; 3];
    let result = format_u8(0, &mut buffer);
    assert_eq!(result, 1);
    assert_eq!(&buffer[..1], b"0");
}

#[test]
fn test_format_u8_edge_case_ten() {
    let mut buffer = [0u8; 3];
    let result = format_u8(10, &mut buffer);
    assert_eq!(result, 2);
    assert_eq!(&buffer[..2], b"10");
}

