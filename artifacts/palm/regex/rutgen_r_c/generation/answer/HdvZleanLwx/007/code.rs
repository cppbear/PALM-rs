// Answer 0

#[test]
#[should_panic]
fn test_parse_octal_non_numeric_start() {
    struct MockParser {
        octal: bool,
        offset: usize,
        line: usize,
        column: usize,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    struct TestParser<'s> {
        parser: MockParser,
        pattern: &'s str,
    }

    impl<'s> ParserI<'s, &'s TestParser<'s>> {
        fn new(parser: &'s TestParser<'s>, pattern: &'s str) -> Self {
            Self {
                parser,
                pattern,
            }
        }

        fn parser(&self) -> &MockParser {
            &self.parser
        }

        fn char(&self) -> char {
            // Simulate non-numeric starting character
            'a'
        }

        fn pos(&self) -> Position {
            Position {
                offset: self.parser.offset,
                line: self.parser.line,
                column: self.parser.column,
            }
        }

        fn bump(&mut self) {
            self.parser.offset += 1;
        }

        fn pattern(&self) -> &str {
            self.pattern
        }
    }

    let mock_parser = MockParser {
        octal: true,
        offset: 0,
        line: 1,
        column: 1,
    };
    let test_parser = TestParser {
        parser: mock_parser,
        pattern: "abc",
    };
    let parser_i = ParserI::new(&test_parser, test_parser.pattern);

    parser_i.parse_octal();
}

#[test]
#[should_panic]
fn test_parse_octal_out_of_range() {
    struct MockParser {
        octal: bool,
        offset: usize,
        line: usize,
        column: usize,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    struct TestParser<'s> {
        parser: MockParser,
        pattern: &'s str,
    }

    impl<'s> ParserI<'s, &'s TestParser<'s>> {
        fn new(parser: &'s TestParser<'s>, pattern: &'s str) -> Self {
            Self {
                parser,
                pattern,
            }
        }

        fn parser(&self) -> &MockParser {
            &self.parser
        }

        fn char(&self) -> char {
            '0'
        }

        fn pos(&self) -> Position {
            Position {
                offset: self.parser.offset,
                line: self.parser.line,
                column: self.parser.column,
            }
        }

        fn bump(&mut self) {
            // Simulate reaching beyond valid octal length
            self.parser.offset += 3; // Increment more than 3 to trigger panic
        }

        fn pattern(&self) -> &str {
            self.pattern
        }
    }

    let mock_parser = MockParser {
        octal: true,
        offset: 0,
        line: 1,
        column: 1,
    };
    let test_parser = TestParser {
        parser: mock_parser,
        pattern: "07777", // More than 3 digits
    };
    let parser_i = ParserI::new(&test_parser, test_parser.pattern);

    parser_i.parse_octal();
}

