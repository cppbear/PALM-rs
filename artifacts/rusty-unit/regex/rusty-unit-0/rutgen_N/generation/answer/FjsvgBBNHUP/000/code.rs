// Answer 0

#[derive(Debug)]
enum WordBoundary {
    Unicode,
    Ascii,
    UnicodeNegate,
    AsciiNegate,
}

impl WordBoundary {
    pub fn is_negated(&self) -> bool {
        match *self {
            WordBoundary::Unicode | WordBoundary::Ascii => false,
            WordBoundary::UnicodeNegate | WordBoundary::AsciiNegate => true,
        }
    }
}

#[test]
fn test_is_negated_unicode() {
    let boundary = WordBoundary::Unicode;
    assert_eq!(boundary.is_negated(), false);
}

#[test]
fn test_is_negated_ascii() {
    let boundary = WordBoundary::Ascii;
    assert_eq!(boundary.is_negated(), false);
}

#[test]
fn test_is_negated_unicode_negate() {
    let boundary = WordBoundary::UnicodeNegate;
    assert_eq!(boundary.is_negated(), true);
}

#[test]
fn test_is_negated_ascii_negate() {
    let boundary = WordBoundary::AsciiNegate;
    assert_eq!(boundary.is_negated(), true);
}

