// Answer 0

#[test]
fn test_parse_flags_unexpected_eof() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            self.pos += 1;
            self.pos < self.input.len()
        }

        fn span(&self) -> usize {
            self.pos
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, span: usize, kind: ast::ErrorKind) -> Result<ast::Flags, ast::Error> {
            Err(ast::Error {
                span,
                kind,
            })
        }

        fn parse_flag(&self) -> Result<ast::Flag, ast::Error> {
            Ok(ast::Flag::default()) // Assuming an appropriate default exists
        }
    }

    impl TestParser {
        // Simulate adding an item that results in a duplication
        fn add_item(&mut self, _item: ast::FlagsItem) -> Option<usize> {
            Some(0)
        }
    }

    let mut parser = TestParser::new("-a"); // Starts with '-'
    assert_eq!(
        parser.parse_flags().unwrap_err(),
        parser.error(parser.span(), ast::ErrorKind::FlagUnexpectedEof)
    );
}

