// Answer 0

#[test]
fn test_parse_whitespace_non_whitespace() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.data.len() {
                self.index += 1;
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, &'static str> {
            loop {
                match self.peek()? {
                    Some(b' ' | b'\n' | b'\t' | b'\r') => {
                        self.eat_char();
                    }
                    other => {
                        return Ok(other);
                    }
                }
            }
        }
    }

    let mut reader = TestReader::new(vec![b' ', b'\t', b'\n', b'a']);
    assert_eq!(reader.parse_whitespace().unwrap(), Some(b'a'));
}

#[test]
fn test_parse_whitespace_only_whitespace() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.data.len() {
                self.index += 1;
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, &'static str> {
            loop {
                match self.peek()? {
                    Some(b' ' | b'\n' | b'\t' | b'\r') => {
                        self.eat_char();
                    }
                    other => {
                        return Ok(other);
                    }
                }
            }
        }
    }

    let mut reader = TestReader::new(vec![b' ', b'\t', b'\n']);
    assert_eq!(reader.parse_whitespace().unwrap(), None);
}

#[test]
fn test_parse_whitespace_eof() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.data.len() {
                self.index += 1;
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, &'static str> {
            loop {
                match self.peek()? {
                    Some(b' ' | b'\n' | b'\t' | b'\r') => {
                        self.eat_char();
                    }
                    other => {
                        return Ok(other);
                    }
                }
            }
        }
    }

    let mut reader = TestReader::new(vec![]);
    match reader.parse_whitespace() {
        Ok(result) => assert_eq!(result, None),
        Err(_) => panic!("Unexpected error"),
    }
}

#[test]
fn test_parse_whitespace_with_error() {
    struct TestReader {
        error: bool,
    }

    impl TestReader {
        fn new(error: bool) -> Self {
            Self { error }
        }

        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.error {
                Err("Peek error")
            } else {
                Ok(Some(b'a'))
            }
        }

        fn eat_char(&mut self) {}

        fn parse_whitespace(&mut self) -> Result<Option<u8>, &'static str> {
            loop {
                match self.peek()? {
                    Some(b' ' | b'\n' | b'\t' | b'\r') => {
                        self.eat_char();
                    }
                    other => {
                        return Ok(other);
                    }
                }
            }
        }
    }

    let mut reader = TestReader::new(true);
    match reader.parse_whitespace() {
        Ok(_) => panic!("Expected error, but got Ok"),
        Err(err) => assert_eq!(err, "Peek error"),
    }
}

