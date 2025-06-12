// Answer 0

#[test]
fn test_bump_and_bump_space_eof() {
    struct MockParser {
        pos: Position,
        pattern: String,
        index: usize,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Position { offset: 0, line: 1, column: 1 },
                pattern: pattern.to_string(),
                index: 0,
            }
        }

        fn bump(&mut self) -> bool {
            if self.index < self.pattern.len() {
                self.pos.offset += 1;
                self.index += 1;
                if self.pattern.chars().nth(self.index - 1) == Some('\n') {
                    self.pos.line += 1;
                    self.pos.column = 1;
                } else {
                    self.pos.column += 1;
                }
                true
            } else {
                false
            }
        }

        fn ignore_whitespace(&self) -> bool {
            true // Always ignore whitespace in this mock
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.index).unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.index >= self.pattern.len()
        }

        fn bump_space(&mut self) {
            while self.ignore_whitespace() && !self.is_eof() {
                if self.char().is_whitespace() {
                    self.bump();
                } else {
                    break;
                }
            }
        }
    }

    struct ParserI<'s> {
        parser: MockParser,
        pattern: &'s str,
    }

    impl<'s> ParserI<'s> {
        fn new(parser: MockParser, pattern: &'s str) -> Self {
            Self {
                parser,
                pattern,
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if !self.parser.bump() {
                return false;
            }
            self.parser.bump_space();
            !self.parser.is_eof()
        }
    }

    let mock_parser = MockParser::new(""); // Empty pattern, should trigger EOF
    let mut parser = ParserI::new(mock_parser, "");
    assert_eq!(parser.bump_and_bump_space(), false);
}

