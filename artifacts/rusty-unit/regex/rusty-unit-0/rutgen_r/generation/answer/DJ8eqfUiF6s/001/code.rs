// Answer 0

#[derive(Debug)]
struct Position {
    offset: usize,
    line: usize,
    column: usize,
}

impl Position {
    fn new(offset: usize, line: usize, column: usize) -> Self {
        Position { offset, line, column }
    }
}

struct Span {
    start: Position,
    end: Position,
}

impl Span {
    fn new(start: Position, end: Position) -> Self {
        Span { start, end }
    }
}

trait CharSource {
    fn offset(&self) -> usize;
    fn char(&self) -> char;
    fn line(&self) -> usize;
    fn column(&self) -> usize;
    fn pos(&self) -> Position;
}

struct TestSource {
    position: Position,
    character: char,
}

impl CharSource for TestSource {
    fn offset(&self) -> usize {
        self.position.offset
    }
    
    fn char(&self) -> char {
        self.character
    }
    
    fn line(&self) -> usize {
        self.position.line
    }
    
    fn column(&self) -> usize {
        self.position.column
    }
    
    fn pos(&self) -> Position {
        self.position.clone()
    }
}

#[test]
fn test_span_char_normal_character() {
    let position = Position::new(0, 1, 1);
    let source = TestSource {
        position,
        character: 'a', // Normal character
    };
    
    let expected_start = source.pos();
    let expected_end = Position::new(1, 1, 2); // Next character position
    
    let span = source.span_char();
    
    assert_eq!(span.start, expected_start);
    assert_eq!(span.end, expected_end);
}

#[test]
fn test_span_char_newline_character() {
    let position = Position::new(1, 1, 1);
    let source = TestSource {
        position,
        character: '\n', // Newline character
    };

    let expected_start = source.pos();
    let expected_end = Position::new(2, 2, 1); // Next line position
    
    let span = source.span_char();

    assert_eq!(span.start, expected_start);
    assert_eq!(span.end, expected_end);
}

#[test]
fn test_span_char_at_boundary() {
    let position = Position::new(usize::MAX - 1, 1, usize::MAX - 1);
    let source = TestSource {
        position,
        character: 'b', // Normal character, near boundary
    };

    let expected_start = source.pos();
    let expected_end = Position::new(usize::MAX, 1, usize::MAX); // Would panic when incrementing

    let span = source.span_char();
    
    assert_eq!(span.start, expected_start);
    // Check that the end position remains at the maximum for overflows
    assert_eq!(span.end, expected_end);
}

