// Answer 0

fn test_deserialize_bool_true() {
    struct Visitor;
    impl de::Visitor<'static> for Visitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        
        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }
    }

    let mut read = MockRead { input: b"true".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };
    let map_key = MapKey { de: &mut deserializer };
    let result = map_key.deserialize_bool(Visitor);
    assert_eq!(result, Ok(true));
}

fn test_deserialize_bool_false() {
    struct Visitor;
    impl de::Visitor<'static> for Visitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        
        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }
    }

    let mut read = MockRead { input: b"false".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };
    let map_key = MapKey { de: &mut deserializer };
    let result = map_key.deserialize_bool(Visitor);
    assert_eq!(result, Ok(false));
}

fn test_deserialize_bool_invalid() {
    struct Visitor;
    impl de::Visitor<'static> for Visitor {
        type Value = bool;

        fn visit_bool(self, _value: bool) -> Result<Self::Value> {
            unreachable!()
        }
    }

    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }
    }
    
    let mut read = MockRead { input: b"notabool".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };
    let map_key = MapKey { de: &mut deserializer };
    let result = map_key.deserialize_bool(Visitor);
    assert!(result.is_err());
}

fn test_deserialize_bool_eof() {
    struct Visitor;
    impl de::Visitor<'static> for Visitor {
        type Value = bool;

        fn visit_bool(self, _value: bool) -> Result<Self::Value> {
            unreachable!()
        }
    }

    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            Err(ErrorCode::Io(io::Error::new(io::ErrorKind::Other, "Error")))
        }

        fn discard(&mut self) {}

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }
    }

    let mut read = MockRead { input: Vec::new(), position: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };
    let map_key = MapKey { de: &mut deserializer };
    let result = map_key.deserialize_bool(Visitor);
    assert!(result.is_err());
}

fn test_deserialize_bool_parse_ident_err() {
    struct Visitor;
    impl de::Visitor<'static> for Visitor {
        type Value = bool;

        fn visit_bool(self, _value: bool) -> Result<Self::Value> {
            unreachable!()
        }
    }

    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b't'))
        }

        fn discard(&mut self) {}

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }
    }

    let mut read = MockRead { input: Vec::new(), position: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };
    let map_key = MapKey { de: &mut deserializer };
    
    // Simulating error during parse_ident
    let result = map_key.deserialize_bool(Visitor);
    assert!(result.is_err());
}

