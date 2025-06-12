// Answer 0

#[test]
fn test_any_unicode() {
    // When bytes is false, it should create a Unicode class that includes the full range of Unicode characters.
    let result = any(false);
    
    // The resulting kind should be of type Class::Unicode
    match result.kind() {
        HirKind::Class(Class::Unicode(cls)) => {
            // Check that the class contains the full Unicode range from '\0' to '\u{10FFFF}'
            let range = cls.ranges();
            assert_eq!(range.len(), 1);
            assert_eq!(range[0].start(), '\0');
            assert_eq!(range[0].end(), '\u{10FFFF}');
        },
        _ => panic!("Expected a Unicode class"),
    }
}

