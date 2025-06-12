// Answer 0

#[test]
fn test_is_negated_unicode() {
    let word_boundary = WordBoundary::Unicode;
    assert_eq!(word_boundary.is_negated(), false);
}

#[test]
fn test_is_negated_ascii() {
    let word_boundary = WordBoundary::Ascii;
    assert_eq!(word_boundary.is_negated(), false);
}

#[test]
fn test_is_negated_unicode_negate() {
    let word_boundary = WordBoundary::UnicodeNegate;
    assert_eq!(word_boundary.is_negated(), true);
}

#[test]
fn test_is_negated_ascii_negate() {
    let word_boundary = WordBoundary::AsciiNegate;
    assert_eq!(word_boundary.is_negated(), true);
}

