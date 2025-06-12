// Answer 0

#[test]
fn test_any_bytes() {
    let hir = regex_syntax::hir::any(true);
    match hir {
        regex_syntax::hir::Hir::Class(regex_syntax::hir::Class::Bytes(cls)) => {
            let ranges = cls.ranges();
            assert_eq!(ranges.len(), 1);
            assert_eq!(ranges[0].start(), b'\0');
            assert_eq!(ranges[0].end(), b'\xFF');
        },
        _ => panic!("Expected a Class::Bytes variant"),
    }
}

#[test]
fn test_any_unicode() {
    let hir = regex_syntax::hir::any(false);
    match hir {
        regex_syntax::hir::Hir::Class(regex_syntax::hir::Class::Unicode(cls)) => {
            let ranges = cls.ranges();
            assert_eq!(ranges.len(), 1);
            assert_eq!(ranges[0].start(), '\0');
            assert_eq!(ranges[0].end(), '\u{10FFFF}');
        },
        _ => panic!("Expected a Class::Unicode variant"),
    }
}

#[test]
fn test_any_bytes_empty() {
    let hir = regex_syntax::hir::any(true);
    let empty_cls = regex_syntax::hir::Class::Bytes(regex_syntax::hir::ClassBytes::empty());
    let expected_hir = regex_syntax::hir::Hir::class(empty_cls);
    assert_eq!(hir, expected_hir);
} 

#[test]
fn test_any_unicode_empty() {
    let hir = regex_syntax::hir::any(false);
    let empty_cls = regex_syntax::hir::Class::Unicode(regex_syntax::hir::ClassUnicode::empty());
    let expected_hir = regex_syntax::hir::Hir::class(empty_cls);
    assert_eq!(hir, expected_hir);
}

