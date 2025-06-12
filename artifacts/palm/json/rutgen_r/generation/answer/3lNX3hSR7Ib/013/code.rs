// Answer 0

fn test_peek_end_of_value_whitespace() -> Result<()> {
    struct TestDeserializer {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestDeserializer {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.pos as u32 + 1 }
        }
    }

    struct Position {
        line: u32,
        column: u32,
    }

    impl TestDeserializer {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }

        fn read(&mut self) -> &mut Self {
            self
        }
    }
    
    let mut deserializer = TestDeserializer::new(vec![b' ']);
    assert_eq!(peek_end_of_value(&mut deserializer), Ok(()));
}

fn test_peek_end_of_value_newline() -> Result<()> {
    struct TestDeserializer {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestDeserializer {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.pos as u32 + 1 }
        }
    }

    struct Position {
        line: u32,
        column: u32,
    }

    impl TestDeserializer {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }

        fn read(&mut self) -> &mut Self {
            self
        }
    }

    let mut deserializer = TestDeserializer::new(vec![b'\n']);
    assert_eq!(peek_end_of_value(&mut deserializer), Ok(()));
}

fn test_peek_end_of_value_comma() -> Result<()> {
    struct TestDeserializer {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestDeserializer {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.pos as u32 + 1 }
        }
    }

    struct Position {
        line: u32,
        column: u32,
    }

    impl TestDeserializer {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }

        fn read(&mut self) -> &mut Self {
            self
        }
    }

    let mut deserializer = TestDeserializer::new(vec![b',']);
    assert_eq!(peek_end_of_value(&mut deserializer), Ok(()));
}

fn test_peek_end_of_value_invalid() -> Result<()> {
    struct TestDeserializer {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestDeserializer {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.pos as u32 + 1 }
        }
    }

    struct Position {
        line: u32,
        column: u32,
    }

    impl TestDeserializer {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }

        fn read(&mut self) -> &mut Self {
            self
        }
    }

    let mut deserializer = TestDeserializer::new(vec![b'a']);
    let result = peek_end_of_value(&mut deserializer);
    assert!(result.is_err());
}

