// Answer 0

#[derive(Default)]
struct Parser {
    pos: Position,
}

#[derive(Default)]
struct Position {
    offset: usize,
}

struct RegexSyntax {
    inner: Parser,
}

impl RegexSyntax {
    fn parser(&self) -> &Parser {
        &self.inner
    }
    
    fn new() -> Self {
        RegexSyntax { inner: Parser::default() }
    }
}

#[test]
fn test_offset_initial() {
    let regex_syntax = RegexSyntax::new();
    assert_eq!(regex_syntax.offset(), 0);
}

#[test]
fn test_offset_after_increment() {
    let mut regex_syntax = RegexSyntax::new();
    regex_syntax.inner.pos.offset = 42;
    assert_eq!(regex_syntax.offset(), 42);
}

#[derive(Debug)]
struct BoundCondition {
    offset: usize,
}

#[test]
#[should_panic]
fn test_offset_panics_out_of_bounds() {
    let mut regex_syntax = RegexSyntax::new();
    regex_syntax.inner.pos.offset = usize::MAX;
    assert_eq!(regex_syntax.offset(), usize::MAX);
}

