// Answer 0

#[test]
fn test_allow_invalid_utf8_enable() {
    struct MockHir {
        allow_invalid_utf8: bool,
    }

    struct ParserBuilder {
        hir: MockHir,
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                hir: MockHir {
                    allow_invalid_utf8: false,
                },
            }
        }

        fn allow_invalid_utf8(&mut self, yes: bool) -> &mut ParserBuilder {
            self.hir.allow_invalid_utf8 = yes;
            self
        }
    }

    let mut builder = ParserBuilder::new();
    builder.allow_invalid_utf8(true);
    assert!(builder.hir.allow_invalid_utf8);
}

#[test]
fn test_allow_invalid_utf8_disable() {
    struct MockHir {
        allow_invalid_utf8: bool,
    }

    struct ParserBuilder {
        hir: MockHir,
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                hir: MockHir {
                    allow_invalid_utf8: true,
                },
            }
        }

        fn allow_invalid_utf8(&mut self, yes: bool) -> &mut ParserBuilder {
            self.hir.allow_invalid_utf8 = yes;
            self
        }
    }

    let mut builder = ParserBuilder::new();
    builder.allow_invalid_utf8(false);
    assert!(!builder.hir.allow_invalid_utf8);
}

