// Answer 0

#[test]
#[should_panic]
fn test_parse_octal_with_octal_disabled() {
    struct TestParser {
        octal: bool,
        position: Position,
        pattern: String,
    }
    
    impl TestParser {
        fn new(octal: bool, position: Position, pattern: &str) -> Self {
            TestParser {
                octal,
                position,
                pattern: pattern.to_string(),
            }
        }

        fn octal(&self) -> bool {
            self.octal
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.position.offset).unwrap()
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn bump(&mut self) {
            self.position.offset += 1;
        }
        
        fn pattern(&self) -> &str {
            &self.pattern
        }
    }
    
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let parser = TestParser::new(false, start_position, "012");
    let parser_i = ParserI { parser, pattern: "012" };
    
    parser_i.parse_octal();  // This should trigger a panic
}

#[test]
#[should_panic]
fn test_parse_octal_with_invalid_char() {
    struct TestParser {
        octal: bool,
        position: Position,
        pattern: String,
    }
    
    impl TestParser {
        fn new(octal: bool, position: Position, pattern: &str) -> Self {
            TestParser {
                octal,
                position,
                pattern: pattern.to_string(),
            }
        }

        fn octal(&self) -> bool {
            self.octal
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.position.offset).unwrap()
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn bump(&mut self) {
            self.position.offset += 1;
        }
        
        fn pattern(&self) -> &str {
            &self.pattern
        }
    }
    
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let parser = TestParser::new(true, start_position, "A12");
    let parser_i = ParserI { parser, pattern: "A12" };

    parser_i.parse_octal();  // This should trigger a panic
}

