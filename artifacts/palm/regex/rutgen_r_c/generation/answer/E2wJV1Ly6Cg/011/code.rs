// Answer 0

#[test]
fn test_peek_space_no_whitespace_ignore() {
    use ast::{ErrorKind, CaptureName};
    
    struct MockParser {
        pos: Position,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Position { offset: 0 }, // Initialize offset as needed
                ignore_whitespace: Cell::new(false),
                pattern: pattern.to_string(),
            }
        }

        fn offset(&self) -> usize {
            self.pos.offset
        }

        fn is_eof(&self) -> bool {
            self.offset() >= self.pattern.len()
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                None
            } else {
                self.pattern[self.offset()..].chars().next()
            }
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace.get()
        }
    }

    struct ParserIWrapper<'a> {
        parser: MockParser,
        pattern: &'a str,
    }

    impl<'a> ParserIWrapper<'a> {
        fn new(parser: MockParser, pattern: &'a str) -> Self {
            Self { parser, pattern }
        }

        fn peek_space(&self) -> Option<char> {
            if !self.parser.ignore_whitespace() {
                return self.parser.peek();
            }
            if self.parser.is_eof() {
                return None;
            }
            let mut start = self.parser.offset();
            for (i, c) in self.parser.pattern[start..].char_indices() {
                if !c.is_whitespace() {
                    start += i;
                    break;
                }
            }
            self.parser.pattern[start..].chars().next()
        }
    }

    let mock_parser = MockParser::new("a b c # comment goes here\n d e");
    let parser_wrapper = ParserIWrapper::new(mock_parser, "a b c # comment goes here\n d e");

    assert_eq!(parser_wrapper.peek_space(), Some('a'));
}

