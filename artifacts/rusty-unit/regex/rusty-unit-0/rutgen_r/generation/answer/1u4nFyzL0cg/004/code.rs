// Answer 0

#[test]
fn test_parse_set_class_unclosed_class_error() {
    struct MockParser {
        chars: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            Self { chars, pos: 0 }
        }

        fn char(&self) -> char {
            if self.pos < self.chars.len() {
                self.chars[self.pos]
            } else {
                '\0' // EOF
            }
        }

        fn bump_space(&mut self) {
            self.pos += 1;
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len()
        }

        fn peek(&self) -> Option<char> {
            if self.pos + 1 < self.chars.len() {
                Some(self.chars[self.pos + 1])
            } else {
                None
            }
        }

        fn bump_if(&mut self, s: &str) -> bool {
            if self.chars[self.pos..].iter().take(s.len()).collect::<String>() == s {
                self.pos += s.len();
                true
            } else {
                false
            }
        }

        fn unclosed_class_error(&self) -> String {
            "Unclosed character class".to_string()
        }

        fn parse_set_class_range(&self) -> Result<ast::ClassSetItem, String> {
            // Mock implementation returning a success
            Ok(ast::ClassSetItem::Char('a')) // Example range
        }

        fn push_class_op(&self, op: ast::ClassSetBinaryOpKind, union: ast::ClassSetUnion) -> Result<ast::ClassSetUnion, String> {
            // Mock implementation just returns the union
            Ok(union)
        }

        fn push_class_open(&self, union: ast::ClassSetUnion) -> Result<ast::ClassSetUnion, String> {
            // Mock implementation just returns the union
            Ok(union)
        }

        fn pop_class(&self, union: ast::ClassSetUnion) -> Result<Either<ast::ClassSetUnion, ast::Class>, String> {
            // Assume some logic that returns our union successfully
            Ok(Either::Right(ast::Class::new())) // Example success
        }
    }

    struct AstClass;
    impl AstClass {
        fn new() -> Self {
            AstClass
        }
    }

    struct AstClassSetUnion {
        span: usize,
        items: Vec<ast::ClassSetItem>,
    }

    impl AstClassSetUnion {
        fn new() -> Self {
            Self { span: 0, items: Vec::new() }
        }

        fn push(&mut self, item: ast::ClassSetItem) {
            self.items.push(item);
        }
    }

    struct AstClassSetItem;
    impl AstClassSetItem {
        fn Char(c: char) -> Self {
            AstClassSetItem
        }
    }

    struct Either<L, R> {
        left: L,
        right: R,
    }

    let mut parser = MockParser::new(vec!['[', '~', '~']);
    
    // Set position at the opening character class
    parser.bump_space(); // move to ' '~'
    parser.bump_space(); // move to ']' at the end but here it won't be allowed
    
    // Invoking the method under test
    let result = parser.parse_set_class();

    // Expected to return an error due to unclosed class
    assert_eq!(result, Err(parser.unclosed_class_error()));
}

