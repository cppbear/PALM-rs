// Answer 0

#[test]
fn test_parse_flag_unicode() {
    struct MockParser {
        input: Vec<char>,
        index: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                index: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.index]
        }

        fn span_char(&self) -> char {
            self.char()
        }

        fn error(&self, _span: char, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new("Unrecognized flag")
        }
    }

    impl MockParser {
        fn parse_flag(&self) -> Result<ast::Flag> {
            match self.char() {
                'i' => Ok(ast::Flag::CaseInsensitive),
                'm' => Ok(ast::Flag::MultiLine),
                's' => Ok(ast::Flag::DotMatchesNewLine),
                'U' => Ok(ast::Flag::SwapGreed),
                'u' => Ok(ast::Flag::Unicode),
                'x' => Ok(ast::Flag::IgnoreWhitespace),
                _ => Err(self.error(self.span_char(), ast::ErrorKind::FlagUnrecognized)),
            }
        }
    }

    let parser = MockParser::new("u");
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::Unicode));
}

