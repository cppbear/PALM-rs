// Answer 0

#[test]
fn test_any_with_bytes_true() {
    use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

    let result = regex_syntax::hir::any(true);
    
    match result {
        Hir::Class(Class::Bytes(cls)) => {
            let ranges = cls.ranges();
            assert_eq!(ranges.len(), 1);
            assert_eq!(ranges[0].start(), b'\0');
            assert_eq!(ranges[0].end(), b'\xFF');
        },
        _ => panic!("Expected a Class::Bytes variant."),
    }
}

#[test]
fn test_any_with_bytes_false() {
    use regex_syntax::hir::{Hir, Class, ClassUnicode, ClassUnicodeRange};

    let result = regex_syntax::hir::any(false);
    
    match result {
        Hir::Class(Class::Unicode(cls)) => {
            let ranges = cls.ranges();
            assert_eq!(ranges.len(), 1);
            assert_eq!(ranges[0].start(), '\0');
            assert_eq!(ranges[0].end(), '\u{10FFFF}');
        },
        _ => panic!("Expected a Class::Unicode variant."),
    }
}

