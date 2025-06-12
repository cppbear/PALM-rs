// Answer 0

#[test]
fn test_fmt_with_ascii_start_and_end() {
    let range = ClassBytesRange { start: 65, end: 90 }; // 'A' to 'Z'
    let mut buf = String::new();
    let result = write!(&mut buf, "{:?}", range);
    assert!(result.is_ok());
    assert_eq!(buf, "ClassBytesRange { start: 'A', end: 'Z' }");
}

#[test]
fn test_fmt_with_non_ascii_start_and_end() {
    let range = ClassBytesRange { start: 200, end: 250 };
    let mut buf = String::new();
    let result = write!(&mut buf, "{:?}", range);
    assert!(result.is_ok());
    assert_eq!(buf, "ClassBytesRange { start: 200, end: 250 }");
}

#[test]
fn test_fmt_with_start_equal_to_end() {
    let range = ClassBytesRange { start: 100, end: 100 }; // 'd'
    let mut buf = String::new();
    let result = write!(&mut buf, "{:?}", range);
    assert!(result.is_ok());
    assert_eq!(buf, "ClassBytesRange { start: 'd', end: 'd' }");
}

#[test]
fn test_fmt_with_start_less_than_end() {
    let range = ClassBytesRange { start: 50, end: 100 }; // '2' to 'd'
    let mut buf = String::new();
    let result = write!(&mut buf, "{:?}", range);
    assert!(result.is_ok());
    assert_eq!(buf, "ClassBytesRange { start: '2', end: 'd' }");
}

#[test]
fn test_fmt_with_start_greater_than_end() {
    let range = ClassBytesRange { start: 150, end: 120 }; // range is reversed
    let mut buf = String::new();
    let result = write!(&mut buf, "{:?}", range);
    assert!(result.is_ok());
    assert_eq!(buf, "ClassBytesRange { start: 150, end: 120 }");
}

