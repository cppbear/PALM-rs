// Answer 0

#[test]
fn test_parse_hex_digits_2() {
    struct Parser {
        pos: usize,
        input: Vec<char>,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Parser {
                pos: 0,
                input: input.chars().collect(),
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Result<ast::Literal> {
            Err(ast::Error)
        }
        
        fn scratch(&self) -> &mut Vec<char> {
            &mut Vec::new()
        }
    }

    let hex = ast::HexLiteralKind::new(2);
    let input = "0A";
    let mut parser = Parser::new(input);
    let result = parser.parse_hex_digits(hex);
    assert!(result.is_ok());
}

#[test]
fn test_parse_hex_digits_4() {
    struct Parser {
        pos: usize,
        input: Vec<char>,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Parser {
                pos: 0,
                input: input.chars().collect(),
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Result<ast::Literal> {
            Err(ast::Error)
        }

        fn scratch(&self) -> &mut Vec<char> {
            &mut Vec::new()
        }
    }

    let hex = ast::HexLiteralKind::new(4);
    let input = "1F4A";
    let mut parser = Parser::new(input);
    let result = parser.parse_hex_digits(hex);
    assert!(result.is_ok());
}

#[test]
fn test_parse_hex_digits_8() {
    struct Parser {
        pos: usize,
        input: Vec<char>,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Parser {
                pos: 0,
                input: input.chars().collect(),
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false;
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Result<ast::Literal> {
            Err(ast::Error)
        }

        fn scratch(&self) -> &mut Vec<char> {
            &mut Vec::new()
        }
    }

    let hex = ast::HexLiteralKind::new(8);
    let input = "0000007E";
    let mut parser = Parser::new(input);
    let result = parser.parse_hex_digits(hex);
    assert!(result.is_ok());
}

#[test]
fn test_parse_hex_digits_invalid() {
    struct Parser {
        pos: usize,
        input: Vec<char>,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Parser {
                pos: 0,
                input: input.chars().collect(),
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false;
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Result<ast::Literal> {
            Err(ast::Error)
        }

        fn scratch(&self) -> &mut Vec<char> {
            &mut Vec::new()
        }
    }

    let hex = ast::HexLiteralKind::new(2);
    let input = "0G"; // Invalid hex digit
    let mut parser = Parser::new(input);
    let result = parser.parse_hex_digits(hex);
    assert!(result.is_err());
}

