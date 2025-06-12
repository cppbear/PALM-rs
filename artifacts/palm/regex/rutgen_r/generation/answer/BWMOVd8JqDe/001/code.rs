// Answer 0

#[derive(Debug)]
struct Parser {
    position: usize,
}

impl Parser {
    fn new(position: usize) -> Self {
        Parser { position }
    }

    fn pos(&self) -> usize {
        self.position
    }

    fn span(&self) -> Span {
        Span::splat(self.pos())
    }
}

#[derive(Debug, PartialEq)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    fn splat(pos: usize) -> Self {
        Span { start: pos, end: pos }
    }
}

#[test]
fn test_span_at_initial_position() {
    let parser = Parser::new(0);
    let span = parser.span();
    assert_eq!(span, Span { start: 0, end: 0 });
}

#[test]
fn test_span_at_position_one() {
    let parser = Parser::new(1);
    let span = parser.span();
    assert_eq!(span, Span { start: 1, end: 1 });
}

#[test]
fn test_span_at_large_position() {
    let parser = Parser::new(100);
    let span = parser.span();
    assert_eq!(span, Span { start: 100, end: 100 });
}

#[test]
fn test_span_at_max_position() {
    let parser = Parser::new(usize::MAX);
    let span = parser.span();
    assert_eq!(span, Span { start: usize::MAX, end: usize::MAX });
}

