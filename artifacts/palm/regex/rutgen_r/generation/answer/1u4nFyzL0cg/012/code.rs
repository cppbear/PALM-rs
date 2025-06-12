// Answer 0

#[derive(Debug)]
struct MockParser {
    input: Vec<char>,
    index: usize,
}

impl MockParser {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            index: 0,
        }
    }

    fn char(&self) -> char {
        self.input[self.index]
    }

    fn bump_space(&mut self) {
        // For this mock, we simply bump to the next character.
        self.index += 1;
    }

    fn is_eof(&self) -> bool {
        self.index >= self.input.len()
    }

    fn peek(&self) -> Option<char> {
        if self.index + 1 < self.input.len() {
            Some(self.input[self.index + 1])
        } else {
            None
        }
    }

    fn bump_if(&mut self, s: &str) -> bool {
        if self.input[self.index..].starts_with(s.chars().collect::<Vec<_>>().as_slice()) {
            self.index += s.len();
            true
        } else {
            false
        }
    }

    fn unclosed_class_error(&self) -> String {
        "Unclosed class error".to_string()
    }

    fn parse_set_class_range(&self) -> Result<ast::ClassSetItem, String> {
        // Mock implementation, returning Ok for testing.
        Ok(ast::ClassSetItem::Range)
    }

    fn push_class_open(&self, union: ast::ClassSetUnion) -> Result<ast::ClassSetUnion, String> {
        // Mock implementation, returning the union.
        Ok(union)
    }

    fn pop_class(&self, union: ast::ClassSetUnion) -> Result<Either<ast::ClassSetUnion, ast::Class>, String> {
        // Mock implementation, returning the union.
        Ok(Either::Left(union))
    }

    fn push_class_op(&self, _op: ast::ClassSetBinaryOpKind, union: ast::ClassSetUnion) -> ast::ClassSetUnion {
        // Mock implementation, returning the union.
        union
    }
}

#[test]
fn test_parse_set_class_with_unclosed_class_error() {
    let mut parser = MockParser::new("[");
    match parser.parse_set_class() {
        Err(e) => assert_eq!(e, "Unclosed class error"),
        _ => panic!("Expected an error, but got Ok"),
    }
}

#[test]
fn test_parse_set_class_with_double_ampersand() {
    let mut parser = MockParser::new("[&&]");
    parser.bump_space(); // Simulate bumping space to find the character '&'
    assert!(parser.char() == '&');
    assert!(parser.peek() == Some('&'));
    assert!(parser.bump_if("&&"));

    let result = parser.parse_set_class();
    assert!(result.is_ok());
}

#[test]
fn test_parse_set_class_with_end_of_input() {
    let mut parser = MockParser::new("[]");
    parser.bump_space(); // Move past '['
    parser.bump_space(); // Move to ']'

    let result = parser.parse_set_class();
    assert!(result.is_ok());
}

