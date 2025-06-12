// Answer 0

#[test]
fn test_word_boundary_ascii() {
    let boundary = WordBoundary::Ascii;
    assert_eq!(boundary.is_negated(), false);
}

#[test]
fn test_word_boundary_unicode() {
    let boundary = WordBoundary::Unicode;
    assert_eq!(boundary.is_negated(), false);
}

#[test]
fn test_word_boundary_ascii_negate() {
    let boundary = WordBoundary::AsciiNegate;
    assert_eq!(boundary.is_negated(), true);
}

#[test]
fn test_word_boundary_unicode_negate() {
    let boundary = WordBoundary::UnicodeNegate;
    assert_eq!(boundary.is_negated(), true);
}

