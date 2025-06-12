// Answer 0

#[test]
fn test_parse_set_class_with_nested_class() {
    struct MockParser<'s> {
        pos: Cell<Position>,
        pattern: &'s str,
        stack_class: RefCell<Vec<()>>,
    }

    impl<'s> MockParser<'s> {
        fn new(pattern: &'s str) -> Self {
            Self {
                pos: Cell::new(0),
                pattern,
                stack_class: RefCell::new(vec![]),
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

        fn unclosed_class_error(&self) -> Error {
            Error {
                kind: ast::ErrorKind::InvalidCharacterClass,
                pattern: self.pattern.to_string(),
                span: Span {
                    start: self.pos.get() as Position,
                    end: self.pos.get() as Position,
                },
            }
        }

        fn push_class_open(&self, union: ClassSetUnion) -> Result<ClassSetUnion> {
            // Mock implementation of opening a class
            Ok(union)
        }

        fn pop_class(&self, union: ClassSetUnion) -> Result<Either<ClassSetUnion, ast::Class>> {
            // Mock implementation of popping a class
            Ok(Either::Right(ast::Class::Bracketed(ast::ClassBracketed {})))
        }

        fn peek(&self) -> Option<char> {
            if self.pos.get() + 1 < self.pattern.len() {
                Some(self.pattern.chars().nth(self.pos.get() + 1).unwrap_or('\0'))
            } else {
                None
            }
        }

        fn bump_if(&self, expected: &str) -> bool {
            // Mock implementation
            if self.pattern.get(self.pos.get()..).unwrap_or("") == expected {
                self.pos.set(self.pos.get() + expected.len());
                true
            } else {
                false
            }
        }

        fn parse_set_class(&self) -> Result<ast::Class> {
            assert_eq!(self.char(), '[');
            let mut union = ClassSetUnion {
                span: Span { start: self.pos.get(), end: self.pos.get() },
                items: vec![],
            };
            loop {
                self.bump_space();
                if self.is_eof() {
                    return Err(self.unclosed_class_error());
                }
                match self.char() {
                    '[' => {
                        if !self.stack_class.borrow().is_empty() {
                            continue; // Mock handling of ASCII class
                        }
                        union = self.push_class_open(union)?;
                    }
                    ']' => return Ok(ast::Class::Bracketed(ast::ClassBracketed {})),
                    '-' if self.peek() == Some('-') => {
                        assert!(self.bump_if("--"));
                        // Mock handling for different operations
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    let parser = MockParser::new("[a-z--]");
    let result = parser.parse_set_class();
    assert!(result.is_err());
}

#[test]
fn test_parse_set_class_unclosed_class() {
    struct MockParser<'s> {
        pos: Cell<Position>,
        pattern: &'s str,
    }

    impl<'s> MockParser<'s> {
        fn new(pattern: &'s str) -> Self {
            Self {
                pos: Cell::new(0),
                pattern,
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

        fn unclosed_class_error(&self) -> Error {
            Error {
                kind: ast::ErrorKind::InvalidCharacterClass,
                pattern: self.pattern.to_string(),
                span: Span {
                    start: self.pos.get() as Position,
                    end: self.pos.get() as Position,
                },
            }
        }

        fn parse_set_class(&self) -> Result<ast::Class> {
            assert_eq!(self.char(), '[');
            let mut union = ClassSetUnion {
                span: Span { start: self.pos.get(), end: self.pos.get() },
                items: vec![],
            };
            loop {
                self.bump_space();
                if self.is_eof() {
                    return Err(self.unclosed_class_error());
                }
                match self.char() {
                    '[' => {
                        // Handling of opening a nested class
                        union = ClassSetUnion { span: union.span, items: vec![] };
                    }
                    ']' => return Ok(ast::Class::Bracketed(ast::ClassBracketed {})),
                    _ => unreachable!(),
                }
            }
        }
    }

    let parser = MockParser::new("[[");
    let result = parser.parse_set_class();
    assert_eq!(result.unwrap_err().kind, ast::ErrorKind::InvalidCharacterClass);
}

