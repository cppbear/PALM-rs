// Answer 0

#[test]
fn test_dot_with_bytes() {
    // Test the case when `bytes` is true.
    
    // Call the dot function with bytes set to true
    let hir = dot(true);

    // Check the HIR kind is a Class with Bytes
    match hir.kind() {
        HirKind::Class(Class::Bytes(cls)) => {
            // Check that cls has two entries in its interval
            let ranges = cls.ranges();
            assert_eq!(ranges.len(), 2);

            // Check that the ranges are as expected
            assert_eq!(ranges[0].start(), b'\0');
            assert_eq!(ranges[0].end(), b'\x09');
            assert_eq!(ranges[1].start(), b'\x0B');
            assert_eq!(ranges[1].end(), b'\xFF');
        },
        _ => panic!("Expected Class::Bytes variant"),
    }
}

#[test]
fn test_dot_with_unicode() {
    // Test the case when `bytes` is false.
    
    // Call the dot function with bytes set to false
    let hir = dot(false);

    // Check the HIR kind is a Class with Unicode
    match hir.kind() {
        HirKind::Class(Class::Unicode(cls)) => {
            // Check that cls has two entries in its interval
            let ranges = cls.ranges();
            assert_eq!(ranges.len(), 2);

            // Check that the ranges are as expected
            assert_eq!(ranges[0].start(), '\0');
            assert_eq!(ranges[0].end(), '\x09');
            assert_eq!(ranges[1].start(), '\x0B');
            assert_eq!(ranges[1].end(), '\u{10FFFF}');
        },
        _ => panic!("Expected Class::Unicode variant"),
    }
}

