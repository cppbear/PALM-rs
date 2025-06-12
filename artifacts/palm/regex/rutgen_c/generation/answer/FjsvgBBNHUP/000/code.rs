// Answer 0

#[test]
fn test_word_boundary_unicode_is_not_negated() {
    let boundary = WordBoundary::Unicode;
    assert_eq!(boundary.is_negated(), false);
}

#[test]
fn test_word_boundary_unicode_negate_is_negated() {
    let boundary = WordBoundary::UnicodeNegate;
    assert_eq!(boundary.is_negated(), true);
}

#[test]
fn test_word_boundary_ascii_is_not_negated() {
    let boundary = WordBoundary::Ascii;
    assert_eq!(boundary.is_negated(), false);
}

#[test]
fn test_word_boundary_ascii_negate_is_negated() {
    let boundary = WordBoundary::AsciiNegate;
    assert_eq!(boundary.is_negated(), true);
}

