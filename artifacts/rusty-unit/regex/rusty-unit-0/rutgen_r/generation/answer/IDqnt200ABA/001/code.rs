// Answer 0

#[test]
fn test_case_insensitive_enable() {
    struct ParserBuilder {
        hir: Hir,
    }

    struct Hir {
        case_insensitive: bool,
    }

    impl Hir {
        fn new() -> Self {
            Hir { case_insensitive: false }
        }

        fn case_insensitive(&mut self, yes: bool) {
            self.case_insensitive = yes;
        }
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder { hir: Hir::new() }
        }

        pub fn case_insensitive(&mut self, yes: bool) -> &mut ParserBuilder {
            self.hir.case_insensitive(yes);
            self
        }
    }

    let mut parser_builder = ParserBuilder::new();
    let result = parser_builder.case_insensitive(true);
    assert_eq!(result.hir.case_insensitive, true);
}

#[test]
fn test_case_insensitive_disable() {
    struct ParserBuilder {
        hir: Hir,
    }

    struct Hir {
        case_insensitive: bool,
    }

    impl Hir {
        fn new() -> Self {
            Hir { case_insensitive: false }
        }

        fn case_insensitive(&mut self, yes: bool) {
            self.case_insensitive = yes;
        }
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder { hir: Hir::new() }
        }

        pub fn case_insensitive(&mut self, yes: bool) -> &mut ParserBuilder {
            self.hir.case_insensitive(yes);
            self
        }
    }

    let mut parser_builder = ParserBuilder::new();
    let result = parser_builder.case_insensitive(false);
    assert_eq!(result.hir.case_insensitive, false);
}

