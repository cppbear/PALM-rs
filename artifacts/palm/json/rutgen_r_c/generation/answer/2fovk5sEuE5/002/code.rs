// Answer 0

fn test_deserialize_any_ok() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'_> for MockRead {
        fn parse_str(&mut self, scratch: &mut Vec<u8>) -> Result<Reference<'_, '_, str>> {
            scratch.extend_from_slice(&self.data[self.pos..]);
            self.pos += self.data.len();
            Ok(Reference::Borrowed("test"))
        }
        fn discard(&mut self) {}
    }

    struct TestVisitor;

    impl de::Visitor<'_> for TestVisitor {
        type Value = String;
        
        fn visit_borrowed_str(self, v: &'static str) -> Result<Self::Value> {
            Ok(v.to_string())
        }

        fn visit_str(self, v: &str) -> Result<Self::Value> {
            Ok(v.to_string())
        }
    }

    let mut mock_read = MockRead { data: b"test".to_vec(), pos: 0 };
    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result: Result<String> = deserializer.deserialize_any(TestVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test");
}

fn test_deserialize_any_err() {
    struct MockRead {
        should_fail: bool,
    }

    impl Read<'_> for MockRead {
        fn parse_str(&mut self, _: &mut Vec<u8>) -> Result<Reference<'_, '_, str>> {
            if self.should_fail {
                Err(Error)
            } else {
                Ok(Reference::Borrowed("test"))
            }
        }
        fn discard(&mut self) {}
    }

    struct TestVisitor;

    impl de::Visitor<'_> for TestVisitor {
        type Value = String;

        fn visit_borrowed_str(self, _: &'static str) -> Result<Self::Value> {
            Ok("test".to_string())
        }

        fn visit_str(self, _: &str) -> Result<Self::Value> {
            Ok("test".to_string())
        }
    }

    let mut mock_read_fail = MockRead { should_fail: true };
    let mut deserializer_fail = Deserializer {
        read: mock_read_fail,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result: Result<String> = deserializer_fail.deserialize_any(TestVisitor);
    assert!(result.is_err());
}

