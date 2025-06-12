// Answer 0

#[test]
fn test_parse_escape_forward_slash() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl TestRead {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, Error> {
            // In a real implementation, hex decoding would happen here.
            Ok(0)
        }

        fn discard(&mut self) {
            // Simulate discarding the character
        }

        fn peek_or_eof(&self) -> Option<u8> {
            if self.position < self.input.len() {
                Some(self.input[self.position])
            } else {
                None
            }
        }
    }

    impl Deref for TestRead {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.input
        }
    }

    let mut read = TestRead {
        input: vec![b'\\', b'/'],
        position: 1,
    };
    let mut scratch = Vec::new();
    let validate = false;

    let result = parse_escape(&mut read, validate, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'/']);
}

#[test]
fn test_parse_escape_backslash() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl TestRead {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, Error> {
            Ok(0)
        }

        fn discard(&mut self) {
        }

        fn peek_or_eof(&self) -> Option<u8> {
            if self.position < self.input.len() {
                Some(self.input[self.position])
            } else {
                None
            }
        }
    }

    impl Deref for TestRead {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.input
        }
    }

    let mut read = TestRead {
        input: vec![b'\\', b'\\'],
        position: 1,
    };
    let mut scratch = Vec::new();
    let validate = false;

    let result = parse_escape(&mut read, validate, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\\']);
}

#[test]
fn test_parse_escape_double_quote() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl TestRead {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, Error> {
            Ok(0)
        }

        fn discard(&mut self) {
        }

        fn peek_or_eof(&self) -> Option<u8> {
            if self.position < self.input.len() {
                Some(self.input[self.position])
            } else {
                None
            }
        }
    }

    impl Deref for TestRead {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.input
        }
    }

    let mut read = TestRead {
        input: vec![b'\\', b'"'],
        position: 1,
    };
    let mut scratch = Vec::new();
    let validate = false;

    let result = parse_escape(&mut read, validate, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'"']);
}

#[test]
fn test_parse_escape_form_feed() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl TestRead {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, Error> {
            Ok(0)
        }

        fn discard(&mut self) {
        }

        fn peek_or_eof(&self) -> Option<u8> {
            if self.position < self.input.len() {
                Some(self.input[self.position])
            } else {
                None
            }
        }
    }

    impl Deref for TestRead {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.input
        }
    }

    let mut read = TestRead {
        input: vec![b'\\', b'f'],
        position: 1,
    };
    let mut scratch = Vec::new();
    let validate = false;

    let result = parse_escape(&mut read, validate, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x0c']);
}

#[test]
fn test_parse_escape_new_line() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl TestRead {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, Error> {
            Ok(0)
        }

        fn discard(&mut self) {
        }

        fn peek_or_eof(&self) -> Option<u8> {
            if self.position < self.input.len() {
                Some(self.input[self.position])
            } else {
                None
            }
        }
    }

    impl Deref for TestRead {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.input
        }
    }

    let mut read = TestRead {
        input: vec![b'\\', b'n'],
        position: 1,
    };
    let mut scratch = Vec::new();
    let validate = false;

    let result = parse_escape(&mut read, validate, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\n']);
}

