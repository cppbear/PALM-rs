// Answer 0

#[test]
fn test_ignore_exponent_valid_plus() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(())
            }
        }
    }

    let mut parser = MockParser {
        input: b"e+123".to_vec(),
        index: 0,
    };
    
    let result = parser.ignore_exponent();
    assert_eq!(result, Ok(()));
}

#[test]
fn test_ignore_exponent_valid_minus() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(())
            }
        }
    }

    let mut parser = MockParser {
        input: b"e-456".to_vec(),
        index: 0,
    };
    
    let result = parser.ignore_exponent();
    assert_eq!(result, Ok(()));
}

#[test]
fn test_ignore_exponent_invalid_no_digit() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(())
            }
        }
    }

    let mut parser = MockParser {
        input: b"e+*".to_vec(),
        index: 0,
    };
    
    let result = parser.ignore_exponent();
    assert!(result.is_err());
}

#[test]
fn test_ignore_exponent_invalid_no_exponent() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(())
            }
        }
    }

    let mut parser = MockParser {
        input: b"e".to_vec(),
        index: 0,
    };
    
    let result = parser.ignore_exponent();
    assert!(result.is_err());
}

