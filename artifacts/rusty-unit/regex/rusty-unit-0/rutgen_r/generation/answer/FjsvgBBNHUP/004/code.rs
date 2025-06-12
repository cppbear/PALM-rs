// Answer 0

#[test]
fn test_is_negated_unicode() {
    struct WordBoundaryTest {
        boundary: WordBoundary,
    }

    enum WordBoundary {
        Unicode,
        Ascii,
        UnicodeNegate,
        AsciiNegate,
    }

    let word_boundary = WordBoundaryTest {
        boundary: WordBoundary::Unicode,
    };

    assert_eq!(word_boundary.is_negated(), false);
}

#[test]
fn test_is_negated_ascii() {
    struct WordBoundaryTest {
        boundary: WordBoundary,
    }

    enum WordBoundary {
        Unicode,
        Ascii,
        UnicodeNegate,
        AsciiNegate,
    }

    let word_boundary = WordBoundaryTest {
        boundary: WordBoundary::Ascii,
    };

    assert_eq!(word_boundary.is_negated(), false);
}

