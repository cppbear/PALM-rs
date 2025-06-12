// Answer 0

#[test]
fn test_dot_unicode() {
    let hir = dot(false);
    if let HirKind::Class(Class::Unicode(class_unicode)) = hir.kind() {
        let ranges = class_unicode.ranges();
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].start(), '\0');
        assert_eq!(ranges[0].end(), '\u{0009}');
        assert_eq!(ranges[1].start(), '\u{000B}');
        assert_eq!(ranges[1].end(), '\u{10FFFF}');
    } else {
        panic!("Expected HirKind::Class(Class::Unicode), found {:?}", hir.kind());
    }
}

#[test]
fn test_dot_bytes() {
    let hir = dot(true);
    if let HirKind::Class(Class::Bytes(class_bytes)) = hir.kind() {
        let ranges = class_bytes.ranges();
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].start(), b'\0');
        assert_eq!(ranges[0].end(), b'\x09');
        assert_eq!(ranges[1].start(), b'\x0B');
        assert_eq!(ranges[1].end(), b'\xFF');
    } else {
        panic!("Expected HirKind::Class(Class::Bytes), found {:?}", hir.kind());
    }
}

