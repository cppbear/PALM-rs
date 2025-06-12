// Answer 0

#[test]
fn test_parse_set_class_empty_range() {
    struct TestParser {
        pattern: String,
        pos: Cell<Position>,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pattern: pattern.to_string(),
                pos: Cell::new(0),
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get()).unwrap_or('\0')
        }

        // Simulate the bump_space method
        fn bump_space(&self) {
            self.pos.set(self.pos.get() + 1);
        }

        fn is_eof(&self) -> bool {
            self.pos.get() >= self.pattern.len()
        }
        
        // Simulate `span` for the class
        fn span(&self) -> Span {
            Span {
                start: self.pos.get() as Position,
                end: self.pos.get() as Position,
            }
        }

        fn unclosed_class_error(&self) -> Result<ast::Class> {
            Err(ast::Error {
                kind: ast::ErrorKind::UnclosedClass,
                pattern: self.pattern.clone(),
                span: self.span(),
            })
        }

        // A mock implementation to adapt to methods in the original function.
        fn push_class_open(&self, union: ClassSetUnion) -> Result<ClassSetUnion> {
            Ok(union)
        }

        fn parse_set_class_range(&self) -> Result<ClassSetItem> {
            Ok(ClassSetItem::Empty(self.span()))
        }
        
        fn peek(&self) -> Option<char> {
            self.pattern.chars().nth(self.pos.get() + 1)
        }
        
        fn bump_if(&self, s: &str) -> bool {
            let len = s.len();
            if self.pattern[self.pos.get()..].starts_with(s) {
                self.pos.set(self.pos.get() + len);
                return true;
            }
            false
        }
        
        fn pop_class(&self, union: ClassSetUnion) -> Result<Either<ClassSetUnion, ast::Class>> {
            Ok(Either::Right(ast::Class::Bracketed(ClassBracketed {})))
        }
    }

    let parser = TestParser::new("[a-z]-[0-9]");
    let result = parser.parse_set_class();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_set_class_unclosed() {
    struct TestParser {
        pattern: String,
        pos: Cell<Position>,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pattern: pattern.to_string(),
                pos: Cell::new(0),
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get()).unwrap_or('\0')
        }

        fn bump_space(&self) {
            self.pos.set(self.pos.get() + 1);
        }

        fn is_eof(&self) -> bool {
            self.pos.get() >= self.pattern.len()
        }
        
        fn span(&self) -> Span {
            Span {
                start: self.pos.get() as Position,
                end: self.pos.get() as Position,
            }
        }

        fn unclosed_class_error(&self) -> Result<ast::Class> {
            Err(ast::Error {
                kind: ast::ErrorKind::UnclosedClass,
                pattern: self.pattern.clone(),
                span: self.span(),
            })
        }

        fn push_class_open(&self, union: ClassSetUnion) -> Result<ClassSetUnion> {
            Ok(union)
        }

        fn parse_set_class_range(&self) -> Result<ClassSetItem> {
            Ok(ClassSetItem::Empty(self.span()))
        }
        
        fn peek(&self) -> Option<char> {
            self.pattern.chars().nth(self.pos.get() + 1)
        }
        
        fn bump_if(&self, s: &str) -> bool {
            let len = s.len();
            if self.pattern[self.pos.get()..].starts_with(s) {
                self.pos.set(self.pos.get() + len);
                return true;
            }
            false
        }

        fn pop_class(&self, union: ClassSetUnion) -> Result<Either<ClassSetUnion, ast::Class>> {
            Ok(Either::Left(union))
        }
    }

    let parser = TestParser::new("[a-z");
    parser.parse_set_class().unwrap();
}

#[test]
fn test_parse_set_class_with_ascii_class() {
    struct TestParser {
        pattern: String,
        pos: Cell<Position>,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pattern: pattern.to_string(),
                pos: Cell::new(0),
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get()).unwrap_or('\0')
        }

        fn bump_space(&self) {
            self.pos.set(self.pos.get() + 1);
        }

        fn is_eof(&self) -> bool {
            self.pos.get() >= self.pattern.len()
        }
        
        fn span(&self) -> Span {
            Span {
                start: self.pos.get() as Position,
                end: self.pos.get() as Position,
            }
        }

        fn unclosed_class_error(&self) -> Result<ast::Class> {
            Err(ast::Error {
                kind: ast::ErrorKind::UnclosedClass,
                pattern: self.pattern.clone(),
                span: self.span(),
            })
        }

        fn push_class_open(&self, union: ClassSetUnion) -> Result<ClassSetUnion> {
            Ok(union)
        }

        fn parse_set_class_range(&self) -> Result<ClassSetItem> {
            Ok(ClassSetItem::Ascii(ClassAscii {
                span: self.span(),
                kind: ClassAsciiKind::Alnum,
                negated: false,
            }))
        }
        
        fn peek(&self) -> Option<char> {
            self.pattern.chars().nth(self.pos.get() + 1)
        }
        
        fn bump_if(&self, s: &str) -> bool {
            let len = s.len();
            if self.pattern[self.pos.get()..].starts_with(s) {
                self.pos.set(self.pos.get() + len);
                return true;
            }
            false
        }

        fn pop_class(&self, union: ClassSetUnion) -> Result<Either<ClassSetUnion, ast::Class>> {
            Ok(Either::Right(ast::Class::Bracketed(ClassBracketed {})))
        }
    }

    let parser = TestParser::new("[a-z][[:alpha:]]");
    let result = parser.parse_set_class();
    assert!(result.is_ok());
}

