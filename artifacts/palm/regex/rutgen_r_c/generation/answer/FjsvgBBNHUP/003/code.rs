// Answer 0

#[test]
fn test_word_boundary_unicode_negate() {
    let word_boundary = WordBoundary::UnicodeNegate;
    assert!(word_boundary.is_negated());
}

#[test]
fn test_word_boundary_ascii_negate() {
    let word_boundary = WordBoundary::AsciiNegate;
    assert!(word_boundary.is_negated());
}

#[test]
fn test_word_boundary_unicode() {
    let word_boundary = WordBoundary::Unicode;
    assert!(!word_boundary.is_negated());
}

#[test]
fn test_word_boundary_ascii() {
    let word_boundary = WordBoundary::Ascii;
    assert!(!word_boundary.is_negated());
}

