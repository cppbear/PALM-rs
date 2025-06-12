// Answer 0

#[test]
fn test_span_char_single_character() {
    struct TestInput {
        offset: usize,
        line: usize,
        column: usize,
        character: char,
    }

    impl TestInput {
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
                offset: self.offset(),
                line: self.line(),
                column: self.column(),
            }
        }
    }

    let input = TestInput {
        offset: 0,
        line: 1,
        column: 1,
        character: 'a',
    };

    let span = input.span_char();
    assert_eq!(span.start.offset, 0);
    assert_eq!(span.start.line, 1);
    assert_eq!(span.start.column, 1);
    assert_eq!(span.end.offset, 1);
    assert_eq!(span.end.line, 1);
    assert_eq!(span.end.column, 2);
}

#[test]
fn test_span_char_newline() {
    struct TestInput {
        offset: usize,
        line: usize,
        column: usize,
        character: char,
    }

    impl TestInput {
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
                offset: self.offset(),
                line: self.line(),
                column: self.column(),
            }
        }
    }

    let input = TestInput {
        offset: 1,
        line: 1,
        column: 1,
        character: '\n',
    };

    let span = input.span_char();
    assert_eq!(span.start.offset, 1);
    assert_eq!(span.start.line, 1);
    assert_eq!(span.start.column, 1);
    assert_eq!(span.end.offset, 2);
    assert_eq!(span.end.line, 2);
    assert_eq!(span.end.column, 1);
}

