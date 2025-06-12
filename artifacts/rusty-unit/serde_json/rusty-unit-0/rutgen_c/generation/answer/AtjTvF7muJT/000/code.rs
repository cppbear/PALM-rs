// Answer 0

#[test]
fn test_deserialize_string_success() {
    struct TestVisitor {
        value: String,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Ok(value.to_string())
        }
    }

    struct TestDeserializer {
        input: String,
    }

    impl<'de> Read<'de> for TestDeserializer {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.input.is_empty() {
                Ok(None)
            } else {
                let byte = self.input.remove(0);
                Ok(Some(byte as u8))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.input.is_empty() {
                Ok(None)
            } else {
                let byte = self.input.chars().next().unwrap();
                Ok(Some(byte as u8))
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::Borrowed(&self.input))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(self.input.as_bytes()))
        }

        fn ignore_str(&mut self) -> Result<()> {
            self.input.clear();
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::default())
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let deserializer = TestDeserializer { input: "\"hello world\"".to_string() };
    let result: Result<String> = deserializer.deserialize_string(TestVisitor { value: String::new() });
    assert_eq!(result.unwrap(), "hello world");
}

#[test]
#[should_panic]
fn test_deserialize_string_failure() {
    struct FailingVisitor;

    impl<'de> de::Visitor<'de> for FailingVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
            panic!("This visitor fails.");
        }
    }

    struct TestDeserializer {
        input: String,
    }

    impl<'de> Read<'de> for TestDeserializer {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::Borrowed(&self.input))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(self.input.as_bytes()))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::default())
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let deserializer = TestDeserializer { input: "\"should fail\"".to_string() };
    let _result: Result<String> = deserializer.deserialize_string(FailingVisitor);
}

