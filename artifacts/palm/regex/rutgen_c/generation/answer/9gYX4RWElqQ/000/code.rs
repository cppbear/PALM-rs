// Answer 0

#[test]
fn test_any_with_bytes() {
    let result = any(true);
    if let Class::Bytes(ref cls) = result.kind().clone() {
        assert_eq!(cls.ranges().len(), 1);
        assert_eq!(cls.ranges()[0].start(), b'\0');
        assert_eq!(cls.ranges()[0].end(), b'\xFF');
    } else {
        panic!("Expected Class::Bytes");
    }
}

#[test]
fn test_any_with_unicode() {
    let result = any(false);
    if let Class::Unicode(ref cls) = result.kind().clone() {
        assert_eq!(cls.ranges().len(), 1);
        assert_eq!(cls.ranges()[0].start(), '\0');
        assert_eq!(cls.ranges()[0].end(), '\u{10FFFF}');
    } else {
        panic!("Expected Class::Unicode");
    }
}

