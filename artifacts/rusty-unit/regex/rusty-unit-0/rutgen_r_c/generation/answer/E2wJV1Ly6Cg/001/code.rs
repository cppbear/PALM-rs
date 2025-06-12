// Answer 0

#[test]
fn test_peek_space_ignore_whitespace_true_eof_true() {
    // Define a simple parser state with ignore_whitespace set to true
    struct MockParser {
        ignore_whitespace: Cell<bool>,
        pos: Cell<Position>,
        pattern: String,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                ignore_whitespace: Cell::new(true),
                pos: Cell::new(Position { offset: pattern.len() }),
                pattern: pattern.to_string(),
            }
        }
    }

    let mock_parser = MockParser::new(" ");
    let parser_i = ParserI {
        parser: &mock_parser,
        pattern: &mock_parser.pattern,
    };

    // expectation: since we are at EOF and ignore_whitespace is true, 
    // peek_space should return None
    let result = parser_i.peek_space();
    assert_eq!(result, None);
}

