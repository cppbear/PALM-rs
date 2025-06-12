// Answer 0

#[test]
fn test_parse_octal_valid_octal() {
    struct MockParser {
        octal: bool,
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn new(octal: bool, input: &'static str) -> Self {
            Self { octal, input, pos: 0 }
        }
        
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            self.pos += self.char().len_utf8();
        }

        fn pos(&self) -> Position {
            Position {
                offset: self.pos,
                line: 1,
                column: self.pos + 1,
            }
        }

        fn pattern(&self) -> &'static str {
            self.input
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mock_parser = MockParser::new(true, "077");
    let parser_instance = ParserI { parser: mock_parser, pattern: "077" };

    let result = parser_instance.parse_octal();

    assert_eq!(result.kind, LiteralKind::Octal);
    assert_eq!(result.c, '℗'); // codepoint for 077 octal which is 63
}

#[test]
#[should_panic]
fn test_parse_octal_invalid_not_in_range() {
    struct MockParser {
        octal: bool,
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn new(octal: bool, input: &'static str) -> Self {
            Self { octal, input, pos: 0 }
        }
        
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            self.pos += self.char().len_utf8();
        }

        fn pos(&self) -> Position {
            Position {
                offset: self.pos,
                line: 1,
                column: self.pos + 1,
            }
        }

        fn pattern(&self) -> &'static str {
            self.input
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mock_parser = MockParser::new(true, "8"); // Invalid octal starts with '8'
    let parser_instance = ParserI { parser: mock_parser, pattern: "8" };

    parser_instance.parse_octal();
}

#[test]
fn test_parse_octal_valid_multiple_digits() {
    struct MockParser {
        octal: bool,
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn new(octal: bool, input: &'static str) -> Self {
            Self { octal, input, pos: 0 }
        }
        
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            self.pos += self.char().len_utf8();
        }

        fn pos(&self) -> Position {
            Position {
                offset: self.pos,
                line: 1,
                column: self.pos + 1,
            }
        }

        fn pattern(&self) -> &'static str {
            self.input
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mock_parser = MockParser::new(true, "0777"); // Three digits octal
    let parser_instance = ParserI { parser: mock_parser, pattern: "0777" };

    let result = parser_instance.parse_octal();

    assert_eq!(result.kind, LiteralKind::Octal);
    assert_eq!(result.c, ''); // codepoint for 0777 octal which is 511
}

