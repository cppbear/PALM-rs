// Answer 0

#[test]
fn test_dot_bytes() {
    let hir = dot(true);
    if let HirKind::Class(Class::Bytes(cls)) = hir.kind() {
        assert_eq!(cls.ranges().len(), 2);
        assert_eq!(cls.ranges()[0].start(), b'\0');
        assert_eq!(cls.ranges()[0].end(), b'\x09');
        assert_eq!(cls.ranges()[1].start(), b'\x0B');
        assert_eq!(cls.ranges()[1].end(), b'\xFF');
    } else {
        panic!("Expected Class::Bytes");
    }
}

#[test]
fn test_dot_unicode() {
    let hir = dot(false);
    if let HirKind::Class(Class::Unicode(cls)) = hir.kind() {
        assert_eq!(cls.ranges().len(), 2);
        assert_eq!(cls.ranges()[0].start(), '\0');
        assert_eq!(cls.ranges()[0].end(), '\x09');
        assert_eq!(cls.ranges()[1].start(), '\x0B');
        assert_eq!(cls.ranges()[1].end(), '\u{10FFFF}');
    } else {
        panic!("Expected Class::Unicode");
    }
}

