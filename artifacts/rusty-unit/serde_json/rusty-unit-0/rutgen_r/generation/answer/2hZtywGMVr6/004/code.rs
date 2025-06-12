// Answer 0

#[test]
fn test_deserialize_map_success() {
    struct MyVisitor;

    impl<'de> serde::de::Visitor<'de> for MyVisitor {
        type Value = ();

        fn visit_map<V>(self, _map: V) -> Result<Self::Value>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    struct TestDeserializer {
        depth: usize,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
        }

        fn end_map(&mut self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new()
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn eat_char(&mut self) {}
    }

    let deserializer = TestDeserializer { depth: 0 };
    let result = deserializer.deserialize_map(MyVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_map_error_eof() {
    struct MyVisitor;

    impl<'de> serde::de::Visitor<'de> for MyVisitor {
        type Value = ();

        fn visit_map<V>(self, _map: V) -> Result<Self::Value>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    struct TestDeserializer;

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn end_map(&mut self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new()
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn eat_char(&mut self) {}
    }

    let deserializer = TestDeserializer {};
    let result = deserializer.deserialize_map(MyVisitor);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_deserialize_map_recursion_limit() {
    struct MyVisitor;

    impl<'de> serde::de::Visitor<'de> for MyVisitor {
        type Value = ();

        fn visit_map<V>(self, _map: V) -> Result<Self::Value>
        where
            V: serde::de::MapAccess<'de>,
        {
            panic!();
        }
    }

    struct TestDeserializer {
        depth: usize,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
        }

        fn end_map(&mut self) -> Result<()> {
            Err(Error::new())
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new()
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn eat_char(&mut self) {
            self.depth += 1;
        }
    }

    let mut deserializer = TestDeserializer { depth: 1 };
    let _ = deserializer.deserialize_map(MyVisitor);
}

