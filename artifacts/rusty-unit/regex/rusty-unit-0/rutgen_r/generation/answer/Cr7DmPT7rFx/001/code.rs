// Answer 0

#[test]
fn test_bump_eof_returns_false() {
    struct MockParser {
        eof: bool,
        position: Position,
        pattern: String,
        offset: usize,
    }

    impl MockParser {
        fn is_eof(&self) -> bool {
            self.eof
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn char(&self) -> char {
            '\0'
        }

        fn parser(&mut self) -> &mut Self {
            self
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }

        fn offset(&self) -> usize {
            self.offset
        }
        
        fn set_position(&mut self, pos: Position) {
            self.position = pos;
        }
    }

    #[derive(Debug)]
    struct Position {
        offset: usize,
        line: usize,
        column: usize,
    }

    let mut parser = MockParser {
        eof: true,
        position: Position { offset: 0, line: 1, column: 1 },
        pattern: String::from(""),
        offset: 0,
    };

    let result = parser.bump();
    assert_eq!(result, false);
}

