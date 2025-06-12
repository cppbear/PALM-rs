// Answer 0

#[test]
fn test_is_negated_ascii_negate() {
    let boundary = WordBoundary::AsciiNegate;
    boundary.is_negated();
}

#[test]
fn test_is_negated_unicode_negate() {
    let boundary = WordBoundary::UnicodeNegate;
    boundary.is_negated();
}

#[test]
fn test_is_negated_unicode() {
    let boundary = WordBoundary::Unicode;
    boundary.is_negated();
}

#[test]
fn test_is_negated_ascii() {
    let boundary = WordBoundary::Ascii;
    boundary.is_negated();
}

