// Answer 0

#[test]
fn test_peek_end_of_value_ok_with_whitespace() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, position: 0 }
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }
        
        fn read(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
            } else {
                Ok(None)
            }
        }
        
        fn peek_position(&self) -> Position {
            Position {
                line: 1,
                column: (self.position + 1) as usize,
            }
        }
    }

    struct Deserializer<R> {
        read: R,
    }
    
    impl<R> Deserializer<R> {
        fn new(read: R) -> Self {
            Deserializer { read }
        }
    }

    struct StreamDeserializer<R> {
        de: Deserializer<R>,
    }
    
    impl<R> StreamDeserializer<R>
    where R: std::ops::Deref<Target = TestReader>, {
        fn peek_end_of_value(&mut self) -> Result<()> {
            match self.de.read.peek() {
                Ok(Some(b' ')) | Ok(Some(b'\n')) | Ok(Some(b'\t')) | Ok(Some(b'\r')) | 
                Ok(Some(b'"')) | Ok(Some(b'[')) | Ok(Some(b']')) | Ok(Some(b'{')) | 
                Ok(Some(b'}')) | Ok(Some(b',')) | Ok(Some(b':')) | Ok(None) => Ok(()),
                Ok(Some(_)) => {
                    let position = self.de.read.peek_position();
                    Err(Error::syntax(ErrorCode::TrailingCharacters, position.line, position.column))
                }
                Err(_) => panic!("Unexpected error"),
            }
        }
    }

    let reader = TestReader::new(vec![b' ']);
    let mut deserializer = StreamDeserializer { de: Deserializer::new(reader) };
    assert!(deserializer.peek_end_of_value().is_ok());
}

#[test]
fn test_peek_end_of_value_err_with_trailing_character() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, position: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn read(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek_position(&self) -> Position {
            Position {
                line: 1,
                column: (self.position + 1) as usize,
            }
        }
    }

    struct Deserializer<R> {
        read: R,
    }

    impl<R> Deserializer<R> {
        fn new(read: R) -> Self {
            Deserializer { read }
        }
    }

    struct StreamDeserializer<R> {
        de: Deserializer<R>,
    }

    impl<R> StreamDeserializer<R>
    where R: std::ops::Deref<Target = TestReader>, {
        fn peek_end_of_value(&mut self) -> Result<()> {
            match self.de.read.peek() {
                Ok(Some(b' ')) | Ok(Some(b'\n')) | Ok(Some(b'\t')) | Ok(Some(b'\r')) | 
                Ok(Some(b'"')) | Ok(Some(b'[')) | Ok(Some(b']')) | Ok(Some(b'{')) | 
                Ok(Some(b'}')) | Ok(Some(b',')) | Ok(Some(b':')) | Ok(None) => Ok(()),
                Ok(Some(_)) => {
                    let position = self.de.read.peek_position();
                    Err(Error::syntax(ErrorCode::TrailingCharacters, position.line, position.column))
                }
                Err(_) => panic!("Unexpected error"),
            }
        }
    }

    let reader = TestReader::new(vec![b'a']);
    let mut deserializer = StreamDeserializer { de: Deserializer::new(reader) };
    match deserializer.peek_end_of_value() {
        Err(Error { err: Box { .. } }) => {}
        _ => panic!("Expected an error for trailing character, but got OK"),
    }
}

#[test]
fn test_peek_end_of_value_ok_with_empty_input() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, position: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn peek_position(&self) -> Position {
            Position {
                line: 1,
                column: 1,
            }
        }
    }

    struct Deserializer<R> {
        read: R,
    }

    impl<R> Deserializer<R> {
        fn new(read: R) -> Self {
            Deserializer { read }
        }
    }

    struct StreamDeserializer<R> {
        de: Deserializer<R>,
    }

    impl<R> StreamDeserializer<R>
    where R: std::ops::Deref<Target = TestReader>, {
        fn peek_end_of_value(&mut self) -> Result<()> {
            match self.de.read.peek() {
                Ok(Some(b' ')) | Ok(Some(b'\n')) | Ok(Some(b'\t')) | Ok(Some(b'\r')) | 
                Ok(Some(b'"')) | Ok(Some(b'[')) | Ok(Some(b']')) | Ok(Some(b'{')) | 
                Ok(Some(b'}')) | Ok(Some(b',')) | Ok(Some(b':')) | Ok(None) => Ok(()),
                Ok(Some(_)) => {
                    let position = self.de.read.peek_position();
                    Err(Error::syntax(ErrorCode::TrailingCharacters, position.line, position.column))
                }
                Err(_) => panic!("Unexpected error"),
            }
        }
    }

    let reader = TestReader::new(vec![]);
    let mut deserializer = StreamDeserializer { de: Deserializer::new(reader) };
    assert!(deserializer.peek_end_of_value().is_ok());
}

