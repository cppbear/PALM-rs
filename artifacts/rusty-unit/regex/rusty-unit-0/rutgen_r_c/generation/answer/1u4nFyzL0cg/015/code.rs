// Answer 0

#[test]
fn test_parse_set_class_basic() {
    struct TestParser {
        pattern: String,
        position: usize,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                position: 0,
            }
        }

        fn char(&self) -> char {
            self.pattern[self.position..self.position + 1].chars().next().unwrap()
        }

        fn bump_space(&mut self) {
            self.position += 1;
        }

        fn is_eof(&self) -> bool {
            self.position >= self.pattern.len()
        }

        fn peek(&self) -> Option<char> {
            if self.position + 1 < self.pattern.len() {
                Some(self.pattern[self.position + 1..self.position + 2].chars().next().unwrap())
            } else {
                None
            }
        }

        fn push_class_op(&self, _kind: ClassSetBinaryOpKind, union: ClassSetUnion) -> Result<ClassSetUnion> {
            Ok(union)
        }

        fn pop_class(&self, union: ClassSetUnion) -> Result<Either<ClassSetUnion, ast::Class>> {
            Ok(Either::Right(ast::Class::Bracketed(ClassBracketed {})))
        }

        fn parse_set_class_range(&self) -> Result<ClassSetItem> {
            Ok(ClassSetItem::Literal(ast::Literal {}))
        }

        fn unclosed_class_error(&self) -> Error {
            Error { 
                kind: ErrorKind::UnclosedClass, 
                pattern: self.pattern.clone(),
                span: Span { start: self.position as Position, end: self.position as Position },
            }
        }
    }

    let mut parser = TestParser::new("[a-z]");

    assert!(parser.char() == '[');
    let result = parser.parse_set_class();
    assert!(result.is_ok());
}

#[test]
fn test_parse_set_class_unclosed_class_error() {
    struct TestParser {
        pattern: String,
        position: usize,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                position: 0,
            }
        }

        fn char(&self) -> char {
            self.pattern[self.position..self.position + 1].chars().next().unwrap()
        }

        fn bump_space(&mut self) {
            self.position += 1;
        }

        fn is_eof(&self) -> bool {
            self.position >= self.pattern.len()
        }

        fn peek(&self) -> Option<char> {
            if self.position + 1 < self.pattern.len() {
                Some(self.pattern[self.position + 1..self.position + 2].chars().next().unwrap())
            } else {
                None
            }
        }

        fn push_class_op(&self, _kind: ClassSetBinaryOpKind, union: ClassSetUnion) -> Result<ClassSetUnion> {
            Ok(union)
        }

        fn pop_class(&self, union: ClassSetUnion) -> Result<Either<ClassSetUnion, ast::Class>> {
            Err(self.unclosed_class_error())
        }

        fn parse_set_class_range(&self) -> Result<ClassSetItem> {
            Ok(ClassSetItem::Literal(ast::Literal {}))
        }

        fn unclosed_class_error(&self) -> Error {
            Error { 
                kind: ErrorKind::UnclosedClass, 
                pattern: self.pattern.clone(),
                span: Span { start: self.position as Position, end: self.position as Position },
            }
        }
    }

    let mut parser = TestParser::new("[a-z");

    assert!(parser.char() == '[');
    let result = parser.parse_set_class();
    assert!(result.is_err());
}


