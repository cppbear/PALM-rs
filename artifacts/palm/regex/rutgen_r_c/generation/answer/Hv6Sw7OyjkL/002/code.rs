// Answer 0

#[test]
fn test_peek_valid_case() {
    struct TestParser {
        pos: Cell<Position>,
        pattern: String,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: self.pos.clone(),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    impl<'s> ParserI<'s, TestParser> {
        fn new(parser: TestParser, pattern: &'s str) -> ParserI<'s, TestParser> {
            ParserI { parser, pattern }
        }

        fn offset(&self) -> usize {
            self.parser.pos.get().offset
        }
        
        fn is_eof(&self) -> bool {
            self.offset() >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern[self.offset()..].chars().next().unwrap_or('\0')
        }
        
        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                return None;
            }
            self.pattern[self.offset() + self.char().len_utf8()..].chars().next()
        }
    }

    let pattern = "abc";
    let parser = TestParser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }), // Not EOF
        pattern: pattern.to_string(),
    };

    let parser_i = ParserI::new(parser, pattern);
    assert_eq!(parser_i.peek(), Some('a'));

    // Move the offset
    parser_i.parser.pos.set(Position { offset: 1, line: 1, column: 2 });
    assert_eq!(parser_i.peek(), Some('b'));

    // Move the offset
    parser_i.parser.pos.set(Position { offset: 2, line: 1, column: 3 });
    assert_eq!(parser_i.peek(), Some('c'));

    // Move the offset to the end
    parser_i.parser.pos.set(Position { offset: 3, line: 1, column: 4 });
    assert!(parser_i.peek().is_none());
}

