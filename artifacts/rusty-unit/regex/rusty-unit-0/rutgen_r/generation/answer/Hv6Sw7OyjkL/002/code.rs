// Answer 0

#[test]
fn test_peek_not_eof() {
    struct Parser {
        pattern: String,
        offset: usize,
    }

    impl Parser {
        fn new(pattern: String) -> Self {
            Parser { pattern, offset: 0 }
        }
        
        fn is_eof(&self) -> bool {
            self.offset >= self.pattern.len()
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn char(&self) -> char {
            self.pattern[self.offset..].chars().next().unwrap()
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                return None;
            }
            self.pattern()[self.offset() + self.char().len_utf8()..].chars().next()
        }
    }

    let parser = Parser::new("abc".to_string());
    assert_eq!(parser.peek(), Some('b'));

    let parser2 = Parser::new("xyz".to_string());
    parser2.offset = 0;
    assert_eq!(parser2.peek(), Some('y'));

    let parser3 = Parser::new("a".to_string());
    parser3.offset = 0;
    assert_eq!(parser3.peek(), None); // edge case where offset points to the end of the string
}

