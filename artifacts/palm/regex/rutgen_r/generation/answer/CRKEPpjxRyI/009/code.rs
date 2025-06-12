// Answer 0

#[test]
fn test_parse_set_class_range_valid() {
    struct DummyParser {
        input: Vec<char>,
        position: usize,
    }

    impl DummyParser {
        fn parse_set_class_item(&mut self) -> Result<ast::ClassSetItem> {
            // Simulating valid parsing of a simple literal character
            let char = self.input[self.position];
            self.position += 1;
            Ok(ast::ClassSetItem::Literal(char))
        }

        fn bump_space(&mut self) {
            // Simulate bumping past space, if any
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.position]
        }

        fn peek_space(&self) -> Option<char> {
            if self.position + 1 < self.input.len() {
                Some(self.input[self.position + 1])
            } else {
                None
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            // Simulate a successful bump and space processing
            self.position += 1; // bump past the `-`
            true
        }

        fn unclosed_class_error(&self) -> ast::Error {
            ast::Error::new("Unclosed class").into()
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new(format!("Error: {:?}", kind)).into()
        }
    }

    let mut parser = DummyParser {
        input: vec!['a', '-', 'c'],
        position: 0,
    };

    let result = parser.parse_set_class_range();
    
    assert!(result.is_ok());

    let range = match result {
        Ok(ast::ClassSetItem::Range(range)) => range,
        _ => panic!("Expected a range"),
    };

    assert!(range.is_valid());
    assert_eq!(range.start, 'a'); 
    assert_eq!(range.end, 'c');
}

