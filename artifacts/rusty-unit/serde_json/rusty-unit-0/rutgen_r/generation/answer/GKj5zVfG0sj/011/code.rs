// Answer 0

#[test]
fn test_deserialize_struct_success_with_map() {
    struct MockVisitor {
        value: usize,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = usize;

        fn visit_map<M>(self, _: M) -> Result<usize>
        where
            M: de::MapAccess<'de>,
        {
            Ok(self.value)
        }
    }

    struct MockDeserializer {
        data: &'static [u8],
    }

    impl MockDeserializer {
        fn parse_whitespace(&self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
        }

        fn peek_invalid_type<V>(&self, _: &V) -> &'static str {
            "Invalid type"
        }

        fn eat_char(&mut self) {}

        fn end_map(&self) -> Result<()> {
            Ok(())
        }

        fn fix_position(&self, err: &'static str) -> &'static str {
            err
        }
    }

    let deserializer = MockDeserializer { data: b"{}" };
    let visitor = MockVisitor { value: 42 };

    let result = deserializer.deserialize_struct("MyStruct", &["field1"], visitor);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_deserialize_struct_fail_on_eof() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = usize;

        fn visit_map<M>(self, _: M) -> Result<usize> {
            Err("Error")
        }
    }

    struct MockDeserializer {
        data: &'static [u8],
        is_eof: bool,
    }

    impl MockDeserializer {
        fn parse_whitespace(&self) -> Result<Option<u8>> {
            if self.is_eof {
                Err("EOF")
            } else {
                Ok(Some(b'{'))
            }
        }

        fn end_map(&self) -> Result<()> {
            Err("End map error")
        }

        fn fix_position(&self, err: &'static str) -> &'static str {
            err
        }

        fn eat_char(&mut self) {}
    }

    let deserializer = MockDeserializer { data: b"{", is_eof: true };
    let visitor = MockVisitor;

    let result = deserializer.deserialize_struct("MyStruct", &["field1"], visitor);
    assert_eq!(result, Err("EOF"));
}

#[test]
fn test_deserialize_struct_failure_on_access() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = usize;

        fn visit_map<M>(self, _: M) -> Result<usize> {
            Err("Error while visiting")
        }
    }

    struct MockDeserializer {
        data: &'static [u8],
    }

    impl MockDeserializer {
        fn parse_whitespace(&self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
        }

        fn end_map(&self) -> Result<()> {
            Ok(())
        }

        fn fix_position(&self, err: &'static str) -> &'static str {
            err
        }

        fn eat_char(&mut self) {}
    }

    let deserializer = MockDeserializer { data: b"{ }" };
    let visitor = MockVisitor;

    let result = deserializer.deserialize_struct("MyStruct", &["field1"], visitor);
    assert_eq!(result, Err("Error while visiting"));
}

