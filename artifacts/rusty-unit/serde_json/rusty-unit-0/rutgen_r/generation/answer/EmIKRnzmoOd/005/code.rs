// Answer 0

#[test]
fn test_scan_integer128_valid_input() {
    struct MockScanner {
        input: Vec<u8>,
        pos: usize,
    }

    impl MockScanner {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                let char = self.input[self.pos];
                self.pos += 1;
                Ok(char)
            } else {
                Err(())
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Err(())
            }
        }

        fn eat_char(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn error(&self, _: ()) -> () {
            ()
        }

        fn peek_error(&self, _: ()) -> () {
            ()
        }
    }

    let input = vec![b'1', b'2', b'3']; // valid input for integer
    let mut scanner = MockScanner::new(input);
    let mut buf = String::new();
    let result = scanner.scan_integer128(&mut buf);
    
    assert_eq!(result, Ok(()));
    assert_eq!(buf, "123");
}

#[test]
fn test_scan_integer128_leading_zero() {
    struct MockScanner {
        input: Vec<u8>,
        pos: usize,
    }

    impl MockScanner {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                let char = self.input[self.pos];
                self.pos += 1;
                Ok(char)
            } else {
                Err(())
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Err(())
            }
        }

        fn eat_char(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn error(&self, _: ()) -> () {
            ()
        }

        fn peek_error(&self, _: ()) -> () {
            ()
        }
    }

    let input = vec![b'0', b'1']; // valid input but leading zero
    let mut scanner = MockScanner::new(input);
    let mut buf = String::new();
    let result = scanner.scan_integer128(&mut buf);
    
    assert_eq!(result, Err(()));
}

#[test]
fn test_scan_integer128_invalid_character() {
    struct MockScanner {
        input: Vec<u8>,
        pos: usize,
    }

    impl MockScanner {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                let char = self.input[self.pos];
                self.pos += 1;
                Ok(char)
            } else {
                Err(())
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Err(())
            }
        }

        fn eat_char(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn error(&self, _: ()) -> () {
            ()
        }

        fn peek_error(&self, _: ()) -> () {
            ()
        }
    }

    let input = vec![b'1', b'2', b'3', b'0']; // valid input but should panic due to leading zero after first digit
    let mut scanner = MockScanner::new(input);
    let mut buf = String::new();
    let result = scanner.scan_integer128(&mut buf);
    
    assert_eq!(result, Ok(()));
    assert_eq!(buf, "1230"); // leads to an error because of the invalid character
}

