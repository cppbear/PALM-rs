// Answer 0

#[test]
fn test_bump_if_pattern_does_not_start_with_prefix() {
    struct MockParser {
        pattern: String,
        pos: Position,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            MockParser {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }
    }

    let parser = MockParser::new("abcdef");
    let parser_i = ParserI::new(&parser, "abcdef");
    
    let result = parser_i.bump_if("xyz");
    assert_eq!(result, false);
}

#[test]
fn test_bump_if_empty_pattern() {
    struct MockParser {
        pattern: String,
        pos: Position,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            MockParser {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }
    }

    let parser = MockParser::new("");
    let parser_i = ParserI::new(&parser, "");
    
    let result = parser_i.bump_if("xyz");
    assert_eq!(result, false);
}

#[test]
fn test_bump_if_prefix_longer_than_pattern() {
    struct MockParser {
        pattern: String,
        pos: Position,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            MockParser {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }
    }

    let parser = MockParser::new("ab");
    let parser_i = ParserI::new(&parser, "ab");
    
    let result = parser_i.bump_if("abcd");
    assert_eq!(result, false);
}

