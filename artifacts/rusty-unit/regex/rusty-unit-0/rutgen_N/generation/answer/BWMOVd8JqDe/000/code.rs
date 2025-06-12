// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    fn splat(pos: usize) -> Span {
        Span { start: pos, end: pos }
    }
}

struct Parser {
    position: usize,
}

impl Parser {
    fn new() -> Parser {
        Parser { position: 0 }
    }
    
    fn pos(&self) -> usize {
        self.position
    }

    fn advance(&mut self, steps: usize) {
        self.position += steps;
    }

    fn span(&self) -> Span {
        Span::splat(self.pos())
    }
}

#[test]
fn test_span_initial_position() {
    let parser = Parser::new();
    let span = parser.span();
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 0);
}

#[test]
fn test_span_after_advance() {
    let mut parser = Parser::new();
    parser.advance(5);
    let span = parser.span();
    assert_eq!(span.start, 5);
    assert_eq!(span.end, 5);
}

#[test]
fn test_multiple_advances() {
    let mut parser = Parser::new();
    parser.advance(3);
    parser.advance(2);
    let span = parser.span();
    assert_eq!(span.start, 5);
    assert_eq!(span.end, 5);
}

