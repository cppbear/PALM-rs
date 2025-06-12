// Answer 0

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

