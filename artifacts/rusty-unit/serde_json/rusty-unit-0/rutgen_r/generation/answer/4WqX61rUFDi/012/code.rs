// Answer 0

#[test]
fn test_deserialize_seq_success() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value>
        where
            A: de::SeqAccess<'de>,
        {
            Ok(vec![1, 2, 3])
        }
    }

    struct TestDeserializer {
        data: &'static [u8],
        position: usize,
    }

    impl TestDeserializer {
        fn new(data: &'static [u8]) -> Self {
            Self { data, position: 0 }
        }
        
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn end_seq(&self) -> Result<()> {
            // Simulate a successful ending of the sequence
            Ok(())
        }

        fn peek_invalid_type<V>(&self, _: &V) -> serde_json::Error {
            serde_json::Error::custom("Invalid type")
        }

        fn peek_error(&self, _: ErrorCode) -> serde_json::Error {
            serde_json::Error::custom("Peek error")
        }

        fn fix_position(&self, err: serde_json::Error) -> serde_json::Error {
            err
        }
    }

    let mut deserializer = TestDeserializer::new(b"[1, 2, 3]");
    let result = deserializer.deserialize_seq(TestVisitor);
    assert_eq!(result, Ok(vec![1, 2, 3]));
}

#[test]
#[should_panic]
fn test_deserialize_seq_eof_error() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value>
        where
            A: de::SeqAccess<'de>,
        {
            Ok(vec![])
        }
    }

    struct TestDeserializer {
        data: &'static [u8],
        position: usize,
    }

    impl TestDeserializer {
        fn new(data: &'static [u8]) -> Self {
            Self { data, position: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn end_seq(&self) -> Result<()> {
            Err(serde_json::Error::custom("Sequence ended unexpectedly"))
        }

        fn peek_invalid_type<V>(&self, _: &V) -> serde_json::Error {
            serde_json::Error::custom("Invalid type")
        }

        fn peek_error(&self, _: ErrorCode) -> serde_json::Error {
            serde_json::Error::custom("Peek error")
        }

        fn fix_position(&self, err: serde_json::Error) -> serde_json::Error {
            err
        }
    }
    
    let mut deserializer = TestDeserializer::new(b"[");
    let _ = deserializer.deserialize_seq(TestVisitor);
}

