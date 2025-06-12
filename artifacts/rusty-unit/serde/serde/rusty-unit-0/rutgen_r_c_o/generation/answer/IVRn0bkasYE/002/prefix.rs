// Answer 0

#[test]
fn test_format_u8_lower_bound() {
    let mut out = [0u8; 3];
    let result = format_u8(10, &mut out);
}

#[test]
fn test_format_u8_exact_bound() {
    let mut out = [0u8; 3];
    let result = format_u8(10, &mut out);
}

#[test]
fn test_format_u8_above_lower_limit() {
    let mut out = [0u8; 3];
    let result = format_u8(12, &mut out);
}

#[test]
fn test_format_u8_above_lower_limit_maximum() {
    let mut out = [0u8; 3];
    let result = format_u8(99, &mut out);
}

