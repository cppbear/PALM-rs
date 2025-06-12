// Answer 0

#[test]
fn test_is_negated_unicode_negate() {
    struct WordBoundary {
        kind: BoundaryKind,
    }

    enum BoundaryKind {
        Unicode,
        Ascii,
        UnicodeNegate,
        AsciiNegate,
    }

    impl WordBoundary {
        pub fn is_negated(&self) -> bool {
            match self.kind {
                BoundaryKind::Unicode | BoundaryKind::Ascii => false,
                BoundaryKind::UnicodeNegate | BoundaryKind::AsciiNegate => true,
            }
        }
    }

    let word_boundary = WordBoundary {
        kind: BoundaryKind::UnicodeNegate,
    };

    assert_eq!(word_boundary.is_negated(), true);
}

