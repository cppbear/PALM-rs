// Answer 0

#[test]
fn test_allow_invalid_utf8_enabled() {
    struct DummyHir {
        allow_invalid_utf8: bool,
    }

    struct ParserBuilder {
        hir: DummyHir,
    }

    impl ParserBuilder {
        fn new() -> Self {
            Self {
                hir: DummyHir {
                    allow_invalid_utf8: false,
                },
            }
        }

        pub fn allow_invalid_utf8(&mut self, yes: bool) -> &mut ParserBuilder {
            self.hir.allow_invalid_utf8 = yes;
            self
        }
    }

    let mut builder = ParserBuilder::new();
    let result = builder.allow_invalid_utf8(true);
    assert!(result.hir.allow_invalid_utf8);
}

#[test]
fn test_allow_invalid_utf8_disabled() {
    struct DummyHir {
        allow_invalid_utf8: bool,
    }

    struct ParserBuilder {
        hir: DummyHir,
    }

    impl ParserBuilder {
        fn new() -> Self {
            Self {
                hir: DummyHir {
                    allow_invalid_utf8: true,
                },
            }
        }

        pub fn allow_invalid_utf8(&mut self, yes: bool) -> &mut ParserBuilder {
            self.hir.allow_invalid_utf8 = yes;
            self
        }
    }

    let mut builder = ParserBuilder::new();
    let result = builder.allow_invalid_utf8(false);
    assert!(!result.hir.allow_invalid_utf8);
}

