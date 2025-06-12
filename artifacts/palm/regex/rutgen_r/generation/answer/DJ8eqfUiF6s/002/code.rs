// Answer 0

#[derive(Debug)]
struct DummySource {
    offset: usize,
    line: usize,
    column: usize,
    character: char,
}

impl DummySource {
    fn offset(&self) -> usize {
        self.offset
    }

    fn line(&self) -> usize {
        self.line
    }

    fn column(&self) -> usize {
        self.column
    }

    fn char(&self) -> char {
        self.character
    }

    fn pos(&self) -> Position {
        Position {
            offset: self.offset,
            line: self.line,
            column: self.column,
        }
    }
}

#[derive(Debug)]
struct Position {
    offset: usize,
    line: usize,
    column: usize,
}

#[derive(Debug)]
struct Span {
    start: Position,
    end: Position,
}

impl Span {
    fn new(start: Position, end: Position) -> Span {
        Span { start, end }
    }
}

impl DummySource {
    fn span_char(&self) -> Span {
        let mut next = Position {
            offset: self.offset().checked_add(self.char().len_utf8()).unwrap(),
            line: self.line(),
            column: self.column().checked_add(1).unwrap(),
        };
        if self.char() == '\n' {
            next.line += 1;
            next.column = 1;
        }
        Span::new(self.pos(), next)
    }
}

#[test]
fn test_span_char_with_newline() {
    let source = DummySource {
        offset: 5,
        line: 1,
        column: 5,
        character: '\n',
    };
    let span = source.span_char();
    assert_eq!(span.start.offset, 5);
    assert_eq!(span.start.line, 1);
    assert_eq!(span.start.column, 5);
    assert_eq!(span.end.offset, 6); // 5 + len_utf8('\n')
    assert_eq!(span.end.line, 2);    // New line increment
    assert_eq!(span.end.column, 1);   // Column reset
}

#[test]
#[should_panic]
fn test_span_char_panic_on_offset() {
    let source = DummySource {
        offset: usize::MAX,
        line: 1,
        column: 5,
        character: 'a',
    };
    let _span = source.span_char(); // Should panic due to checked_add overflow
}

#[test]
#[should_panic]
fn test_span_char_panic_on_column() {
    let source = DummySource {
        offset: 5,
        line: 1,
        column: usize::MAX,
        character: 'a',
    };
    let _span = source.span_char(); // Should panic due to checked_add overflow
}

