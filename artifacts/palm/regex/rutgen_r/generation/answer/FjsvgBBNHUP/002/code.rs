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
fn test_is_negated_ascii() {
    let boundary = WordBoundary::Ascii;
    assert_eq!(boundary.is_negated(), false);
}

#[test]
fn test_is_negated_unicode() {
    let boundary = WordBoundary::Unicode;
    assert_eq!(boundary.is_negated(), false);
}

