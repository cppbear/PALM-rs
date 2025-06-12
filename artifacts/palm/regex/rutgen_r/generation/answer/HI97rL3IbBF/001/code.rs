// Answer 0

#[test]
fn test_dot_matches_new_line_enable() {
    struct MockHir {
        dot_matches_new_line_enabled: bool,
    }

    impl MockHir {
        fn dot_matches_new_line(&mut self, yes: bool) {
            self.dot_matches_new_line_enabled = yes;
        }
    }

    struct ParserBuilder {
        hir: MockHir,
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                hir: MockHir {
                    dot_matches_new_line_enabled: false,
                },
            }
        }

        fn dot_matches_new_line(&mut self, yes: bool) -> &mut Self {
            self.hir.dot_matches_new_line(yes);
            self
        }
    }

    let mut parser = ParserBuilder::new();
    assert_eq!(parser.hir.dot_matches_new_line_enabled, false);
    parser.dot_matches_new_line(true);
    assert_eq!(parser.hir.dot_matches_new_line_enabled, true);
}

#[test]
fn test_dot_matches_new_line_disable() {
    struct MockHir {
        dot_matches_new_line_enabled: bool,
    }

    impl MockHir {
        fn dot_matches_new_line(&mut self, yes: bool) {
            self.dot_matches_new_line_enabled = yes;
        }
    }

    struct ParserBuilder {
        hir: MockHir,
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                hir: MockHir {
                    dot_matches_new_line_enabled: true,
                },
            }
        }

        fn dot_matches_new_line(&mut self, yes: bool) -> &mut Self {
            self.hir.dot_matches_new_line(yes);
            self
        }
    }

    let mut parser = ParserBuilder::new();
    assert_eq!(parser.hir.dot_matches_new_line_enabled, true);
    parser.dot_matches_new_line(false);
    assert_eq!(parser.hir.dot_matches_new_line_enabled, false);
}

#[test]
#[should_panic]
fn test_dot_matches_new_line_boundary() {
    struct MockHir {
        dot_matches_new_line_enabled: bool,
    }

    impl MockHir {
        fn dot_matches_new_line(&mut self, yes: bool) {
            if yes {
                panic!("Intentional panic for boundary test");
            }
        }
    }

    struct ParserBuilder {
        hir: MockHir,
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                hir: MockHir {
                    dot_matches_new_line_enabled: false,
                },
            }
        }

        fn dot_matches_new_line(&mut self, yes: bool) -> &mut Self {
            self.hir.dot_matches_new_line(yes);
            self
        }
    }

    let mut parser = ParserBuilder::new();
    parser.dot_matches_new_line(true);
}

