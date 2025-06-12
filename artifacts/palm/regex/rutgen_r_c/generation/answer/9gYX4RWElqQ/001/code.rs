// Answer 0

#[test]
fn test_any_with_bytes_true() {
    let result = any(true);
    if let Class::Bytes(class_bytes) = result.kind() {
        let ranges = class_bytes.ranges();
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].start(), b'\0');
        assert_eq!(ranges[0].end(), b'\xFF');
    } else {
        panic!("Expected Class::Bytes");
    }
}

#[test]
fn test_any_with_bytes_false() {
    let result = any(false);
    if let Class::Unicode(class_unicode) = result.kind() {
        let ranges = class_unicode.ranges();
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].start(), '\0');
        assert_eq!(ranges[0].end(), '\u{10FFFF}');
    } else {
        panic!("Expected Class::Unicode");
    }
}

