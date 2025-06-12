// Answer 0

#[test]
fn test_peek_eof() {
    struct Parser {
        input: String,
        offset: usize,
    }

    impl Parser {
        fn is_eof(&self) -> bool {
            self.offset >= self.input.len()
        }

        fn pattern(&self) -> &str {
            &self.input
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn char(&self) -> char {
            // This method is not used when is_eof is true
            ' ' // placeholder; it won't be called in this test case
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                return None;
            }
            self.pattern()[self.offset() + self.char().len_utf8()..].chars().next()
        }
    }

    let parser = Parser {
        input: String::from(""),
        offset: 0,
    };

    assert_eq!(parser.peek(), None);
}

