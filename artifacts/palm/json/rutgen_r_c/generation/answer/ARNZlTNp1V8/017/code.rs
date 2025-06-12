// Answer 0

#[test]
fn test_deserialize_any_null() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            // Mock parsing logic for string
            let s = "test_str".as_bytes();
            scratch.extend_from_slice(s);
            Ok(Reference::Borrowed("test_str"))
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader {
        input: b"null".to_vec(),
        position: 0,
    };
    
    let mut deserializer = Deserializer {
        read: reader,
        scratch,
        remaining_depth: 1,
    };
    
    let result = deserializer.deserialize_any(TestVisitor {});
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_any_true() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            let s = "test_str".as_bytes();
            scratch.extend_from_slice(s);
            Ok(Reference::Borrowed("test_str"))
        }
    }

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            assert!(value);
            Ok(())
        }

        fn visit_borrowed_str(self, _value: &str) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_str(self, _value: &str) -> Result<Self::Value> {
            Ok(())
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader {
        input: b"true".to_vec(),
        position: 0,
    };
    
    let mut deserializer = Deserializer {
        read: reader,
        scratch,
        remaining_depth: 1,
    };
    
    let result = deserializer.deserialize_any(TestVisitor {});
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_any_sequence() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            let s = "sequence".as_bytes();
            scratch.extend_from_slice(s);
            Ok(Reference::Borrowed("sequence"))
        }
    }

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_seq(self, _seq: SeqAccess<'de, impl Read<'de>>) -> Result<Self::Value> {
            Ok(())
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader {
        input: b"[1, 2, 3]".to_vec(),
        position: 0,
    };
    
    let mut deserializer = Deserializer {
        read: reader,
        scratch,
        remaining_depth: 1,
    };
    
    let result = deserializer.deserialize_any(TestVisitor {});
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_any_object() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            let s = "key".as_bytes();
            scratch.extend_from_slice(s);
            Ok(Reference::Borrowed("key"))
        }
    }

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map(self, _map: MapAccess<'de, impl Read<'de>>) -> Result<Self::Value> {
            Ok(())
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader {
        input: b"{\"key\":\"value\"}".to_vec(),
        position: 0,
    };
    
    let mut deserializer = Deserializer {
        read: reader,
        scratch,
        remaining_depth: 1,
    };
    
    let result = deserializer.deserialize_any(TestVisitor {});
    assert!(result.is_ok());
}

