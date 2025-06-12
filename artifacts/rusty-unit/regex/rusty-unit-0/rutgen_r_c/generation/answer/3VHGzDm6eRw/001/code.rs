// Answer 0

#[test]
fn test_class_bytes_range_fmt_with_bounds() {
    let range = ClassBytesRange { start: 0x7F, end: 0x7F };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", range);
    assert!(result.is_ok());
    assert_eq!(output, "ClassBytesRange { start: '\u{7f}', end: '\u{7f}' }");
}

#[test]
fn test_class_bytes_range_fmt_with_start_below_bound() {
    let range = ClassBytesRange { start: 0x7E, end: 0x7F };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", range);
    assert!(result.is_ok());
    assert_eq!(output, "ClassBytesRange { start: '\u{7e}', end: '\u{7f}' }");
}

#[test]
fn test_class_bytes_range_fmt_with_end_below_bound() {
    let range = ClassBytesRange { start: 0x7F, end: 0x7E };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", range);
    assert!(result.is_ok());
    assert_eq!(output, "ClassBytesRange { start: '\u{7f}', end: '\u{7e}' }");
}

#[test]
fn test_class_bytes_range_fmt_with_both_below_bound() {
    let range = ClassBytesRange { start: 0x7E, end: 0x7E };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", range);
    assert!(result.is_ok());
    assert_eq!(output, "ClassBytesRange { start: '\u{7e}', end: '\u{7e}' }");
}

#[test]
fn test_class_bytes_range_fmt_with_start_above_bound() {
    let range = ClassBytesRange { start: 0x80, end: 0x7F };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", range);
    assert!(result.is_ok());
    assert_eq!(output, "ClassBytesRange { start: 128, end: '\u{7f}' }");
}

#[test]
fn test_class_bytes_range_fmt_with_end_above_bound() {
    let range = ClassBytesRange { start: 0x7F, end: 0x80 };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", range);
    assert!(result.is_ok());
    assert_eq!(output, "ClassBytesRange { start: '\u{7f}', end: 128 }");
}

#[test]
fn test_class_bytes_range_fmt_with_both_above_bound() {
    let range = ClassBytesRange { start: 0x80, end: 0x80 };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", range);
    assert!(result.is_ok());
    assert_eq!(output, "ClassBytesRange { start: 128, end: 128 }");
}

