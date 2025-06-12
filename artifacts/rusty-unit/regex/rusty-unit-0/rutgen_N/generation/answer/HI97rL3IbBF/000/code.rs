// Answer 0

#[derive(Default)]
struct HIR {
    dot_matches_new_line: bool,
}

impl HIR {
    fn dot_matches_new_line(&mut self, yes: bool) {
        self.dot_matches_new_line = yes;
    }
}

struct ParserBuilder {
    hir: HIR,
}

impl Default for ParserBuilder {
    fn default() -> Self {
        Self {
            hir: HIR::default(),
        }
    }
}

impl ParserBuilder {
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut ParserBuilder {
        self.hir.dot_matches_new_line(yes);
        self
    }
}

#[test]
fn test_dot_matches_new_line_enable() {
    let mut parser_builder = ParserBuilder::default();
    parser_builder.dot_matches_new_line(true);
    assert!(parser_builder.hir.dot_matches_new_line);
}

#[test]
fn test_dot_matches_new_line_disable() {
    let mut parser_builder = ParserBuilder::default();
    parser_builder.dot_matches_new_line(false);
    assert!(!parser_builder.hir.dot_matches_new_line);
}

#[test]
fn test_dot_matches_new_line_toggle() {
    let mut parser_builder = ParserBuilder::default();

    parser_builder.dot_matches_new_line(true);
    assert!(parser_builder.hir.dot_matches_new_line);

    parser_builder.dot_matches_new_line(false);
    assert!(!parser_builder.hir.dot_matches_new_line);
}

