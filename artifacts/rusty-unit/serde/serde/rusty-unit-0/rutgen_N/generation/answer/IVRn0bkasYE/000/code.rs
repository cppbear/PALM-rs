// Answer 0

#[test]
fn test_format_u8_below_ten() {
    let mut output = [0; 3];
    let result = format_u8(5, &mut output);
    assert_eq!(result, 1);
    assert_eq!(output[0], b'5');
}

#[test]
fn test_format_u8_between_ten_and_ninety_nine() {
    let mut output = [0; 3];
    let result = format_u8(42, &mut output);
    assert_eq!(result, 2);
    assert_eq!(output[0], b'4');
    assert_eq!(output[1], b'2');
}

#[test]
fn test_format_u8_one_hundred() {
    let mut output = [0; 3];
    let result = format_u8(100, &mut output);
    assert_eq!(result, 3);
    assert_eq!(output[0], b'1');
    assert_eq!(output[1], b'0');
    assert_eq!(output[2], b'0');
}

#[test]
fn test_format_u8_ninety_nine() {
    let mut output = [0; 3];
    let result = format_u8(99, &mut output);
    assert_eq!(result, 2);
    assert_eq!(output[0], b'9');
    assert_eq!(output[1], b'9');
}

#[test]
fn test_format_u8_boundary_conditions() {
    let mut output = [0; 3];
    
    let result = format_u8(10, &mut output);
    assert_eq!(result, 2);
    assert_eq!(output[0], b'1');
    assert_eq!(output[1], b'0');

    let result = format_u8(98, &mut output);
    assert_eq!(result, 2);
    assert_eq!(output[0], b'9');
    assert_eq!(output[1], b'8');
}

