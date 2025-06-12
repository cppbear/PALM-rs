// Answer 0

#[test]
fn test_parse_set_class_with_intersection() {
    struct MockParser {
        chars: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            MockParser { chars, position: 0 }
        }

        fn char(&self) -> char {
            self.chars[self.position]
        }

        fn bump_space(&mut self) {
            // Simulate space bump (no-op in this case)
        }

        fn is_eof(&self) -> bool {
            self.position >= self.chars.len()
        }

        fn peek(&self) -> Option<char> {
            if self.position + 1 < self.chars.len() {
                Some(self.chars[self.position + 1])
            } else {
                None
            }
        }

        fn bump_if(&mut self, s: &str) -> bool {
            if self.position + s.len() - 1 < self.chars.len() && s.chars().enumerate().all(|(i, c)| c == self.chars[self.position + i]) {
                self.position += s.len();
                return true;
            }
            false
        }

        fn parse_set_class_range(&self) -> Result<ast::ClassSetItem> {
            // Implementation stub, adjust as necessary based on actual logic
            Ok(ast::ClassSetItem::Char(self.char()))
        }

        fn push_class_op(
            &self,
            _kind: ast::ClassSetBinaryOpKind,
            union: ast::ClassSetUnion,
        ) -> ast::ClassSetUnion {
            // Implementation stub, adjust as necessary based on actual logic
            union
        }

        fn push_class_open(&self, union: ast::ClassSetUnion) -> Result<ast::ClassSetUnion> {
            // Simulation of pushing a class open
            Ok(union)
        }

        fn pop_class(&self, union: ast::ClassSetUnion) -> Result<Either<ast::ClassSetUnion, ast::Class>> {
            // Simulation of popping a class, replace with actual logic
            Ok(Either::Right(ast::Class::Empty))
        }
        
        fn unclosed_class_error(&self) -> String {
            "Unclosed class error".to_string()
        }

        fn parser(&self) -> &ParserMock {
            &self.parser_mock
        }
    }

    let mut parser = MockParser::new(vec!['[', '&', '&', ']']);
    let result = parser.parse_set_class();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_set_class_unclosed_class() {
    struct MockParser {
        chars: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            MockParser { chars, position: 0 }
        }

        fn char(&self) -> char {
            self.chars[self.position]
        }

        fn bump_space(&mut self) {}

        fn is_eof(&self) -> bool {
            self.position >= self.chars.len()
        }

        fn peek(&self) -> Option<char> {
            if self.position + 1 < self.chars.len() {
                Some(self.chars[self.position + 1])
            } else {
                None
            }
        }

        fn parse_set_class_range(&self) -> Result<ast::ClassSetItem> {
            Ok(ast::ClassSetItem::Char(self.char()))
        }

        fn bump_if(&mut self, _: &str) -> bool { false }
        
        fn push_class_op(
            &self,
            _: ast::ClassSetBinaryOpKind,
            union: ast::ClassSetUnion,
        ) -> ast::ClassSetUnion {
            union
        }

        fn push_class_open(&self, union: ast::ClassSetUnion) -> Result<ast::ClassSetUnion> {
            Ok(union)
        }

        fn pop_class(&self, union: ast::ClassSetUnion) -> Result<Either<ast::ClassSetUnion, ast::Class>> {
            Err(self.unclosed_class_error()) 
        }

        fn unclosed_class_error(&self) -> String {
            "Unclosed class error".to_string()
        }
    }

    let mut parser = MockParser::new(vec!['[']);
    let _ = parser.parse_set_class(); 
}

