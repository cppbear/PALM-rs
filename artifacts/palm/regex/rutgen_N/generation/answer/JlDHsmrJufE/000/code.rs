// Answer 0

#[test]
fn test_swap_greed_enable() {
    struct DummyHir {
        swap_greed_enabled: bool,
    }

    impl DummyHir {
        fn swap_greed(&mut self, yes: bool) {
            self.swap_greed_enabled = yes;
        }
    }

    struct ParserBuilder {
        hir: DummyHir,
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                hir: DummyHir {
                    swap_greed_enabled: false,
                },
            }
        }

        fn swap_greed(&mut self, yes: bool) -> &mut Self {
            self.hir.swap_greed(yes);
            self
        }
    }

    let mut builder = ParserBuilder::new();
    builder.swap_greed(true);
    assert_eq!(builder.hir.swap_greed_enabled, true);
}

#[test]
fn test_swap_greed_disable() {
    struct DummyHir {
        swap_greed_enabled: bool,
    }

    impl DummyHir {
        fn swap_greed(&mut self, yes: bool) {
            self.swap_greed_enabled = yes;
        }
    }

    struct ParserBuilder {
        hir: DummyHir,
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                hir: DummyHir {
                    swap_greed_enabled: true,
                },
            }
        }

        fn swap_greed(&mut self, yes: bool) -> &mut Self {
            self.hir.swap_greed(yes);
            self
        }
    }

    let mut builder = ParserBuilder::new();
    builder.swap_greed(false);
    assert_eq!(builder.hir.swap_greed_enabled, false);
}

