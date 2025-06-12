// Answer 0

#[test]
fn test_parse_flags_repeated_negation_error() {
    struct Parser {
        input: Vec<char>,
        pos: usize,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn span_char(&self) -> usize {
            self.pos
        }

        fn span(&self) -> (usize, usize) {
            (self.pos, self.pos + 1)
        }

        fn error(&self, span: usize, kind: ast::ErrorKind) -> Result<ast::Flags, ast::Error> {
            Err(ast::Error {
                span,
                kind,
            })
        }

        fn parse_flag(&self) -> Result<char, ast::Error> {
            // Simulate parsing a valid flag
            Ok(self.char())
        }

        fn pos(&self) -> usize {
            self.pos
        }
    }

    let mut parser = Parser::new("-a-b-c");
    let result = parser.parse_flags();
    assert!(result.is_err());
    if let Err(err) = result {
        if let ast::ErrorKind::FlagRepeatedNegation { original } = err.kind {
            assert_eq!(original, 2); // Assuming span of repeated negation is 2
        } else {
            panic!("Expected a FlagRepeatedNegation error");
        }
    } else {
        panic!("Expected an error, but got Ok value");
    }
}

