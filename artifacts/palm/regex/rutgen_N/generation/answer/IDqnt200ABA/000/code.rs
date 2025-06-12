// Answer 0

#[derive(Default)]
struct ParserBuilder {
    hir: Hir,
}

#[derive(Default)]
struct Hir {
    case_insensitive: bool,
}

impl Hir {
    pub fn case_insensitive(&mut self, yes: bool) {
        self.case_insensitive = yes;
    }
}

impl ParserBuilder {
    pub fn case_insensitive(&mut self, yes: bool) -> &mut ParserBuilder {
        self.hir.case_insensitive(yes);
        self
    }
}

#[test]
fn test_case_insensitive_enable() {
    let mut parser_builder = ParserBuilder::default();
    parser_builder.case_insensitive(true);
    assert!(parser_builder.hir.case_insensitive);
}

#[test]
fn test_case_insensitive_disable() {
    let mut parser_builder = ParserBuilder::default();
    parser_builder.case_insensitive(false);
    assert!(!parser_builder.hir.case_insensitive);
}

#[test]
fn test_case_insensitive_default() {
    let parser_builder = ParserBuilder::default();
    assert!(!parser_builder.hir.case_insensitive);
}

