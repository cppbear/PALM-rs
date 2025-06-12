// Answer 0

#[test]
fn test_peek_not_at_eof() {
    struct Parser {
        pattern: String,
        offset: usize,
    }

    impl Parser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                offset: 0,
            }
        }

        fn is_eof(&self) -> bool {
            self.offset >= self.pattern.len()
        }

        fn pattern(&self) -> &String {
            &self.pattern
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn char(&self) -> char {
            self.pattern[self.offset..].chars().next().unwrap()
        }

        fn advance(&mut self) {
            if !self.is_eof() {
                self.offset += self.char().len_utf8();
            }
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                return None;
            }
            self.pattern()[self.offset + self.char().len_utf8()..].chars().next()
        }
    }

    let mut parser = Parser::new("abc");
    assert_eq!(parser.peek(), Some('a'));
    parser.advance();
    assert_eq!(parser.peek(), Some('b'));
    parser.advance();
    assert_eq!(parser.peek(), Some('c'));
}

#[test]
fn test_peek_at_eof() {
    struct Parser {
        pattern: String,
        offset: usize,
    }

    impl Parser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                offset: 3,
            }
        }

        fn is_eof(&self) -> bool {
            self.offset >= self.pattern.len()
        }

        fn pattern(&self) -> &String {
            &self.pattern
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn char(&self) -> char {
            self.pattern[self.offset..].chars().next().unwrap_or('\0')
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                return None;
            }
            self.pattern()[self.offset + self.char().len_utf8()..].chars().next()
        }
    }

    let parser = Parser::new("abc");
    assert_eq!(parser.peek(), None);
}

