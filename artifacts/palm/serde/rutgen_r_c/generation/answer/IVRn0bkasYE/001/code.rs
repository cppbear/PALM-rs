// Answer 0

#[test]
fn test_format_u8_case_above_99() {
    let mut output = [0u8; 3];
    let result = format_u8(100, &mut output);
    assert_eq!(result, 3);
    assert_eq!(&output, b"1\x02\x00");
}

#[test]
fn test_format_u8_case_between_10_and_99() {
    let mut output = [0u8; 2];
    let result = format_u8(42, &mut output);
    assert_eq!(result, 2);
    assert_eq!(&output, b"4\x02");
}

#[test]
fn test_format_u8_case_below_10() {
    let mut output = [0u8; 1];
    let result = format_u8(7, &mut output);
    assert_eq!(result, 1);
    assert_eq!(&output, b'0' + 7);
}

