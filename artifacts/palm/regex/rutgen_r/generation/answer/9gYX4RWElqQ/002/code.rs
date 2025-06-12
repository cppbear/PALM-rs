// Answer 0

#[test]
fn test_any_with_bytes_false() {
    let result = any(false);
    match result {
        Hir::Class(Class::Unicode(cls)) => {
            assert_eq!(cls.ranges.len(), 1);
            assert_eq!(cls.ranges[0], ClassUnicodeRange::new('\0', '\u{10FFFF}'));
        },
        _ => panic!("Expected Hir::Class with Class::Unicode"),
    }
}

#[test]
fn test_any_with_bytes_false_empty() {
    let result = any(false);
    match result {
        Hir::Class(Class::Unicode(cls)) => {
            assert!(cls.is_empty() == false); // Should not be empty
        },
        _ => panic!("Expected Hir::Class with Class::Unicode"),
    }
}

