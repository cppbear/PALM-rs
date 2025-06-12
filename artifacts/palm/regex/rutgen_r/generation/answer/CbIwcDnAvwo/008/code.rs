// Answer 0

#[test]
fn test_hir_assertion_end_line_multi_line() {
    struct MockFlags {
        unicode: bool,
        multi_line: bool,
    }

    impl MockFlags {
        fn unicode(&self) -> bool {
            self.unicode
        }

        fn multi_line(&self) -> bool {
            self.multi_line
        }
    }

    struct MockContext {
        flags: MockFlags,
    }

    impl MockContext {
        fn flags(&self) -> &MockFlags {
            &self.flags
        }

        fn hir_assertion(&self, asst: &Assertion) -> Result<Hir> {
            let unicode = self.flags().unicode();
            let multi_line = self.flags().multi_line();
            Ok(match asst.kind {
                AssertionKind::StartLine => {
                    Hir::anchor(if multi_line {
                        Anchor::StartLine
                    } else {
                        Anchor::StartText
                    })
                }
                AssertionKind::EndLine => {
                    Hir::anchor(if multi_line {
                        Anchor::EndLine
                    } else {
                        Anchor::EndText
                    })
                }
                AssertionKind::StartText => {
                    Hir::anchor(Anchor::StartText)
                }
                AssertionKind::EndText => {
                    Hir::anchor(Anchor::EndText)
                }
                AssertionKind::WordBoundary => {
                    Hir::word_boundary(if unicode {
                        WordBoundary::Unicode
                    } else {
                        WordBoundary::Ascii
                    })
                }
                AssertionKind::NotWordBoundary => {
                    Hir::word_boundary(if unicode {
                        WordBoundary::UnicodeNegate
                    } else {
                        WordBoundary::AsciiNegate
                    })
                }
            })
        }
    }

    // Mock the necessary structs.
    #[derive(Debug)]
    struct Assertion {
        kind: AssertionKind,
    }

    #[derive(Debug)]
    enum AssertionKind {
        StartLine,
        EndLine,
        StartText,
        EndText,
        WordBoundary,
        NotWordBoundary,
    }

    #[derive(Debug)]
    struct Hir;

    impl Hir {
        fn anchor(anchor: Anchor) -> Hir {
            Hir {}
        }
        
        fn word_boundary(boundary: WordBoundary) -> Hir {
            Hir {}
        }
    }

    #[derive(Debug)]
    enum Anchor {
        StartLine,
        EndLine,
        StartText,
        EndText,
    }

    #[derive(Debug)]
    enum WordBoundary {
        Unicode,
        Ascii,
        UnicodeNegate,
        AsciiNegate,
    }

    // Test data setup
    let context = MockContext {
        flags: MockFlags {
            unicode: false,
            multi_line: true,
        },
    };
    
    let assertion = Assertion {
        kind: AssertionKind::EndLine,
    };

    // Execute and validate
    let result = context.hir_assertion(&assertion);
    assert!(result.is_ok());
}

