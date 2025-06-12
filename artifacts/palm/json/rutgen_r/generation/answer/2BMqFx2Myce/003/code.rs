// Answer 0

#[test]
fn test_parse_whitespace_non_whitespace() {
    struct MockParser {
        index: usize,
        input: &'static [u8],
    }

    impl MockParser {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }
    }

    let mut parser = MockParser {
        index: 0,
        input: b"   hello",
    };

    let result = parse_whitespace(&mut parser);
    assert_eq!(result, Ok(Some(b'h')));
}

#[test]
fn test_parse_whitespace_empty() {
    struct MockParser {
        index: usize,
        input: &'static [u8],
    }

    impl MockParser {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }
    }

    let mut parser = MockParser {
        index: 0,
        input: b"",
    };

    let result = parse_whitespace(&mut parser);
    assert_eq!(result, Ok(None));
}

#[test]
fn test_parse_whitespace_only_whitespace() {
    struct MockParser {
        index: usize,
        input: &'static [u8],
    }

    impl MockParser {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }
    }

    let mut parser = MockParser {
        index: 0,
        input: b"   \n\t\r ",
    };

    let result = parse_whitespace(&mut parser);
    assert_eq!(result, Ok(None));
}

#[test]
fn test_parse_whitespace_error() {
    struct MockParser {
        index: usize,
        input: &'static [u8],
    }

    impl MockParser {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Err(())
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }
    }

    let mut parser = MockParser {
        index: 0,
        input: b"",
    };

    let result = parse_whitespace(&mut parser);
    assert_eq!(result, Ok(None));
}

