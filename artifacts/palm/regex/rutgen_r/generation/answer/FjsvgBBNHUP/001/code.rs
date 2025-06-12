// Answer 0

#[test]
fn test_is_negated_ascii_negate() {
    struct WordBoundaryTest {
        boundary: WordBoundary,
    }

    impl WordBoundaryTest {
        fn is_negated(&self) -> bool {
            match self.boundary {
                WordBoundary::Unicode | WordBoundary::Ascii => false,
                WordBoundary::UnicodeNegate | WordBoundary::AsciiNegate => true,
            }
        }
    }

    enum WordBoundary {
        Unicode,
        Ascii,
        UnicodeNegate,
        AsciiNegate,
    }

    let test_case = WordBoundaryTest {
        boundary: WordBoundary::AsciiNegate,
    };

    assert!(test_case.is_negated());
}

