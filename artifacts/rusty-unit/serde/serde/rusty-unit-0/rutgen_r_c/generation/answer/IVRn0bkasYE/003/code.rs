// Answer 0

#[test]
fn test_format_u8_below_10() {
    let mut buffer = [0u8; 3];
    let output_length = format_u8(5, &mut buffer);
    assert_eq!(output_length, 1);
    assert_eq!(&buffer[..1], b"5");
}

#[test]
fn test_format_u8_equal_10() {
    let mut buffer = [0u8; 3];
    let output_length = format_u8(10, &mut buffer);
    assert_eq!(output_length, 2);
    assert_eq!(&buffer[..2], b"10");
}

#[test]
fn test_format_u8_below_100() {
    let mut buffer = [0u8; 3];
    let output_length = format_u8(99, &mut buffer);
    assert_eq!(output_length, 2);
    assert_eq!(&buffer[..2], b"99");
}

#[test]
fn test_format_u8_edge_case() {
    let mut buffer = [0u8; 3];
    let output_length = format_u8(0, &mut buffer);
    assert_eq!(output_length, 1);
    assert_eq!(&buffer[..1], b"0");
}

