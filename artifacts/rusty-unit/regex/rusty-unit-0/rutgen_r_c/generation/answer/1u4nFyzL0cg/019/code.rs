// Answer 0

#[test]
fn test_parse_set_class_with_empty_class() {
    struct TestParser {
        pos: Cell<Position>,
        stack_class: RefCell<Vec<ClassState>>,
        input: String,
        current_index: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                pos: Cell::new(Position::new()),
                stack_class: RefCell::new(Vec::new()),
                input: input.to_string(),
                current_index: 0,
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.current_index).unwrap_or('\0')
        }

        fn bump_space(&mut self) {
            while self.current_index < self.input.len() && self.char().is_whitespace() {
                self.current_index += 1;
            }
        }

        fn is_eof(&self) -> bool {
            self.current_index >= self.input.len()
        }
        
        fn bump_if(&mut self, s: &str) -> bool {
            if self.input[self.current_index..].starts_with(s) {
                self.current_index += s.len();
                true
            } else {
                false
            }
        }

        fn peek(&self) -> Option<char> {
            self.input.chars().nth(self.current_index + 1)
        }

        fn push_class_open(&mut self, union: ClassSetUnion) -> Result<ClassSetUnion> {
            Err(ast::Error {
                kind: ast::ErrorKind::Invalid,
                pattern: self.input.clone(),
                span: Span::new(self.pos.get(), self.pos.get()),
            })
        }

        fn pop_class(&mut self) -> Result<Either<ClassSetUnion, ast::Class>> {
            // This method is a stub for testing purposes.
            Ok(Either::Right(ast::Class::Bracketed(ast::ClassBracketed {})))
        }
    }

    // Initialize a new TestParser at the opening '['
    let parser = TestParser::new("[");
    let result = parser.parse_set_class();

    assert!(result.is_err());
}

#[test]
fn test_parse_set_class_with_invalid_ranges() {
    struct TestParser {
        pos: Cell<Position>,
        stack_class: RefCell<Vec<ClassState>>,
        input: String,
        current_index: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                pos: Cell::new(Position::new()),
                stack_class: RefCell::new(Vec::new()),
                input: input.to_string(),
                current_index: 0,
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.current_index).unwrap_or('\0')
        }

        fn bump_space(&mut self) {
            while self.current_index < self.input.len() && self.char().is_whitespace() {
                self.current_index += 1;
            }
        }

        fn is_eof(&self) -> bool {
            self.current_index >= self.input.len()
        }
        
        fn bump_if(&mut self, s: &str) -> bool {
            if self.input[self.current_index..].starts_with(s) {
                self.current_index += s.len();
                true
            } else {
                false
            }
        }

        fn peek(&self) -> Option<char> {
            self.input.chars().nth(self.current_index + 1)
        }

        fn push_class_open(&mut self, union: ClassSetUnion) -> Result<ClassSetUnion> {
            Err(ast::Error {
                kind: ast::ErrorKind::Invalid,
                pattern: self.input.clone(),
                span: Span::new(self.pos.get(), self.pos.get()),
            })
        }

        fn pop_class(&mut self) -> Result<Either<ClassSetUnion, ast::Class>> {
            // This method is a stub for testing purposes.
            Ok(Either::Right(ast::Class::Bracketed(ast::ClassBracketed {})))
        }
    }

    // Initialize a new TestParser with an invalid range '-]'
    let parser = TestParser::new("[a--]");
    let result = parser.parse_set_class();

    assert!(result.is_err());
}

#[test]
fn test_parse_set_class_with_unclosed_class() {
    struct TestParser {
        pos: Cell<Position>,
        stack_class: RefCell<Vec<ClassState>>,
        input: String,
        current_index: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                pos: Cell::new(Position::new()),
                stack_class: RefCell::new(Vec::new()),
                input: input.to_string(),
                current_index: 0,
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.current_index).unwrap_or('\0')
        }

        fn bump_space(&mut self) {
            while self.current_index < self.input.len() && self.char().is_whitespace() {
                self.current_index += 1;
            }
        }

        fn is_eof(&self) -> bool {
            self.current_index >= self.input.len()
        }
        
        fn bump_if(&mut self, s: &str) -> bool {
            if self.input[self.current_index..].starts_with(s) {
                self.current_index += s.len();
                true
            } else {
                false
            }
        }

        fn peek(&self) -> Option<char> {
            self.input.chars().nth(self.current_index + 1)
        }

        fn push_class_open(&mut self, union: ClassSetUnion) -> Result<ClassSetUnion> {
            Err(ast::Error {
                kind: ast::ErrorKind::Invalid,
                pattern: self.input.clone(),
                span: Span::new(self.pos.get(), self.pos.get()),
            })
        }

        fn pop_class(&mut self) -> Result<Either<ClassSetUnion, ast::Class>> {
            // This method is a stub for testing purposes.
            Ok(Either::Right(ast::Class::Bracketed(ast::ClassBracketed {})))
        }
    }

    // Initialize a new TestParser positioned at '[' but not ending with ']'
    let parser = TestParser::new("[a-z");
    let result = parser.parse_set_class();

    assert!(result.is_err());
}

