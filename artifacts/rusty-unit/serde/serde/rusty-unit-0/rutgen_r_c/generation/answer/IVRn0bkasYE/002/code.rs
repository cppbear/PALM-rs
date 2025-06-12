// Answer 0

#[test]
fn test_format_u8_two_digit() {
    let mut output = [0u8; 3];
    let result = format_u8(10, &mut output);
    assert_eq!(result, 2);
    assert_eq!(&output[..2], b"10");
}

#[test]
fn test_format_u8_single_digit() {
    let mut output = [0u8; 3];
    let result = format_u8(5, &mut output);
    assert_eq!(result, 1);
    assert_eq!(&output[..1], b"5");
}

#[test]
fn test_format_u8_three_digit() {
    let mut output = [0u8; 3];
    let result = format_u8(100, &mut output);
    assert_eq!(result, 3);
    assert_eq!(&output[..3], b"100");
}

