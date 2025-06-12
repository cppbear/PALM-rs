// Answer 0

#[test]
fn test_scan_integer128_with_valid_input() {
    struct MockDeserializer {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockDeserializer {
        fn new(data: Vec<u8>) -> Self {
            MockDeserializer { data, pos: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.pos < self.data.len() {
                let ch = self.data[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.data.len() {
                Ok(self.data[self.pos])
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Simulate error handling
        }
    }

    let mut deserializer = MockDeserializer::new(vec![b'1', b'2', b'3']);
    let mut buf = String::new();
    let result = scan_integer128(&mut deserializer, &mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "123");
}

#[test]
fn test_scan_integer128_with_leading_zero() {
    struct MockDeserializer {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockDeserializer {
        fn new(data: Vec<u8>) -> Self {
            MockDeserializer { data, pos: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.pos < self.data.len() {
                let ch = self.data[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.data.len() {
                Ok(self.data[self.pos])
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Simulate error handling
        }
    }

    let mut deserializer = MockDeserializer::new(vec![b'0', b'1']);
    let mut buf = String::new();
    let result = scan_integer128(&mut deserializer, &mut buf);
    assert!(result.is_err());
}

#[test]
fn test_scan_integer128_with_non_digit_after_zero() {
    struct MockDeserializer {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockDeserializer {
        fn new(data: Vec<u8>) -> Self {
            MockDeserializer { data, pos: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.pos < self.data.len() {
                let ch = self.data[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.data.len() {
                Ok(self.data[self.pos])
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Simulate error handling
        }
    }

    let mut deserializer = MockDeserializer::new(vec![b'0', b'a']);
    let mut buf = String::new();
    let result = scan_integer128(&mut deserializer, &mut buf);
    assert!(result.is_err());
}

