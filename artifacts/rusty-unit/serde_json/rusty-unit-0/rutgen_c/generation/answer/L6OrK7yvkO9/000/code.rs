// Answer 0

#[test]
fn test_deserialize_bool_true() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Read<'_> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u32 }
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead::new(b"true".to_vec()),
        scratch: Vec::new(),
        remaining_depth: 10,
    };

    let visitor = TestVisitor {};
    let result = deserializer.deserialize_bool(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_deserialize_bool_false() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Read<'_> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u32 }
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead::new(b"false".to_vec()),
        scratch: Vec::new(),
        remaining_depth: 10,
    };

    let visitor = TestVisitor {};
    let result = deserializer.deserialize_bool(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[test]
fn test_deserialize_bool_invalid() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Read<'_> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u32 }
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead::new(b"invalid".to_vec()),
        scratch: Vec::new(),
        remaining_depth: 10,
    };

    let visitor = TestVisitor {};
    let result = deserializer.deserialize_bool(visitor);
    assert!(result.is_err());
}

struct TestVisitor {}

impl<'de> de::Visitor<'de> for TestVisitor {
    type Value = bool;

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(value)
    }

    // Implement other methods of Visitor if necessary for the test
}

