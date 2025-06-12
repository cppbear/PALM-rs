// Answer 0

fn test_deserialize_bool_true() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            assert_eq!(value, true);
            Ok(value)
        }
    }

    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                self.index += 1;
                Ok(Some(self.data[self.index - 1]))
            } else {
                Ok(None)
            }
        }
        
        fn discard(&mut self) {}
    }

    let mut read = MockRead { data: b"true".to_vec(), index: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };

    let result = deserializer.deserialize_bool(MockVisitor);
    assert!(result.is_ok());
}

fn test_deserialize_bool_false() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            assert_eq!(value, false);
            Ok(value)
        }
    }

    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                self.index += 1;
                Ok(Some(self.data[self.index - 1]))
            } else {
                Ok(None)
            }
        }
        
        fn discard(&mut self) {}
    }

    let mut read = MockRead { data: b"false".to_vec(), index: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };

    let result = deserializer.deserialize_bool(MockVisitor);
    assert!(result.is_ok());
}

fn test_deserialize_bool_invalid() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, _value: bool) -> Result<Self::Value> {
            panic!("This should not be called for invalid input");
        }
    }

    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                self.index += 1;
                Ok(Some(self.data[self.index - 1]))
            } else {
                Ok(None)
            }
        }
        
        fn discard(&mut self) {}
    }

    let mut read = MockRead { data: b"invalid".to_vec(), index: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };
    let result = deserializer.deserialize_bool(MockVisitor);
    assert!(result.is_err());
}

fn test_deserialize_bool_eof() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, _value: bool) -> Result<Self::Value> {
            panic!("This should not be called for EOF case");
        }
    }

    struct MockRead {
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None) // Simulate EOF
        }
        
        fn discard(&mut self) {}
    }

    let mut read = MockRead { index: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };
    let result = deserializer.deserialize_bool(MockVisitor);
    assert!(result.is_err());
}

