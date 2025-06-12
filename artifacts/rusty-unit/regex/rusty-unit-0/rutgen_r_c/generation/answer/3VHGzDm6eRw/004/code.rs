// Answer 0

#[test]
fn test_fmt_start_and_end_above_127() {
    let class_bytes_range = ClassBytesRange { start: 128, end: 200 };
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    
    let result = class_bytes_range.fmt(formatter);
    
    assert!(result.is_ok());
    assert_eq!(output, "ClassBytesRange { start: 128, end: 200 }");
}

#[test]
fn test_fmt_start_above_127_and_end_below_127() {
    let class_bytes_range = ClassBytesRange { start: 128, end: 100 };
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    
    let result = class_bytes_range.fmt(formatter);
    
    assert!(result.is_ok());
    assert_eq!(output, "ClassBytesRange { start: 128, end: 100 }");
}

#[test]
fn test_fmt_start_below_127_and_end_above_127() {
    let class_bytes_range = ClassBytesRange { start: 100, end: 128 };
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    
    let result = class_bytes_range.fmt(formatter);
    
    assert!(result.is_ok());
    assert_eq!(output, "ClassBytesRange { start: 100, end: 128 }");
}

