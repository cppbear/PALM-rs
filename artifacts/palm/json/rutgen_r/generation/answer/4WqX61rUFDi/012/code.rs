// Answer 0

#[test]
fn test_deserialize_seq_success() {
    struct TestVisitor {
        value: Vec<u8>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value>
        where
            A: de::SeqAccess<'de>,
        {
            Ok(self.value)
        }
    }

    struct MockDeserializer {
        depth: usize,
        input: Vec<u8>,
        pos: usize,
    }

    impl MockDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let ch = self.input[self.pos];
                self.pos += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new()
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn end_seq(&self) -> Result<()> {
            Ok(())
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn peek_invalid_type<V>(&self, _visitor: &V) -> Error {
            Error::new()
        }
    }

    let mut deserializer = MockDeserializer {
        depth: 0,
        input: b"[1, 2, 3]".to_vec(),
        pos: 0,
    };
    let visitor = TestVisitor {
        value: deserializer.input.clone(),
    };

    let result: Result<Vec<u8>> = deserializer.deserialize_seq(visitor);
    assert_eq!(result, Ok(deserializer.input));
}

#[test]
fn test_deserialize_seq_eof_error() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value>
        where
            A: de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    struct MockDeserializer {
        depth: usize,
        input: Vec<u8>,
        pos: usize,
    }

    impl MockDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let ch = self.input[self.pos];
                self.pos += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new()
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn end_seq(&self) -> Result<()> {
            Ok(())
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn peek_invalid_type<V>(&self, _visitor: &V) -> Error {
            Error::new()
        }
    }

    let mut deserializer = MockDeserializer {
        depth: 0,
        input: b"".to_vec(),
        pos: 0,
    };
    let visitor = TestVisitor;

    let result: Result<()> = deserializer.deserialize_seq(visitor);
    assert!(result.is_err());
}

