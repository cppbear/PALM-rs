// Answer 0

#[test]
fn test_multi_line_enable() {
    struct ParserBuilder {
        hir: Hir,
    }

    struct Hir {
        multi_line_enabled: bool,
    }

    impl Hir {
        fn new() -> Self {
            Hir {
                multi_line_enabled: false,
            }
        }

        fn multi_line(&mut self, yes: bool) {
            self.multi_line_enabled = yes;
        }
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                hir: Hir::new(),
            }
        }

        fn multi_line(&mut self, yes: bool) -> &mut ParserBuilder {
            self.hir.multi_line(yes);
            self
        }
    }

    let mut parser = ParserBuilder::new();
    let returned_value = parser.multi_line(true);

    assert_eq!(returned_value as *const _ , &parser as *const _);
    assert!(parser.hir.multi_line_enabled);
}

#[test]
fn test_multi_line_disable() {
    struct ParserBuilder {
        hir: Hir,
    }

    struct Hir {
        multi_line_enabled: bool,
    }

    impl Hir {
        fn new() -> Self {
            Hir {
                multi_line_enabled: false,
            }
        }

        fn multi_line(&mut self, yes: bool) {
            self.multi_line_enabled = yes;
        }
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                hir: Hir::new(),
            }
        }

        fn multi_line(&mut self, yes: bool) -> &mut ParserBuilder {
            self.hir.multi_line(yes);
            self
        }
    }

    let mut parser = ParserBuilder::new();
    let returned_value = parser.multi_line(false);

    assert_eq!(returned_value as *const _ , &parser as *const _);
    assert!(!parser.hir.multi_line_enabled);
}

