// Answer 0

#[test]
fn test_contains_simple_case_mapping_with_equal_start_end() {
    let start_end_cases = vec![
        ('a', 'a'),  // Lowercase a
        ('A', 'A'),  // Uppercase A
        ('0', '0'),  // Digit 0
        ('1', '1'),  // Digit 1
        ('_', '_'),  // Underscore
        ('ñ', 'ñ'),  // Lowercase n with tilde
        ('ß', 'ß'),  // German sharp s
    ];

    for (start, end) in start_end_cases {
        assert_eq!(contains_simple_case_mapping(start, end), true);
    }
}

#[test]
fn test_contains_simple_case_mapping_with_contiguous_range() {
    assert!(contains_simple_case_mapping('a', 'c'));  // Range 'a' to 'c'
    assert!(contains_simple_case_mapping('A', 'C'));  // Range 'A' to 'C'
}

#[test]
fn test_contains_simple_case_mapping_with_large_unicode_range() {
    assert!(contains_simple_case_mapping('\u{0041}', '\u{0041}')); // 'A'
    assert!(contains_simple_case_mapping('\u{00E0}', '\u{00E0}')); // 'à'
    assert!(contains_simple_case_mapping('\u{03B1}', '\u{03B1}')); // Greek 'α'
}

#[should_panic(expected = "assertion failed")]
#[test]
fn test_contains_simple_case_mapping_with_reverse_range() {
    contains_simple_case_mapping('b', 'a');  // This should panic
}

#[test]
fn test_contains_simple_case_mapping_with_empty_case() {
    assert!(!contains_simple_case_mapping('!', '!')); // Not a case mapping
}

