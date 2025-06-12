// Answer 0

#[test]
fn test_deserialize_enum_object() {
    struct TestRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u32 + 1 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u32 + 1 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestRead { data: b"{\"UnitVariant\"}".to_vec(), position: 0 },
        scratch: Vec::new(),
        remaining_depth: 8,
    };

    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = &'de str;

        fn visit_enum<V>(self, _access: V) -> Result<Self::Value>
        where
            V: de::EnumAccess<'de>,
        {
            Ok("UnitVariant")
        }
    }

    let result: Result<&str> = deserializer.deserialize_enum("TestEnum", &["UnitVariant"], TestVisitor);
    assert_eq!(result.unwrap(), "UnitVariant");
}

#[test]
fn test_deserialize_enum_invalid() {
    struct TestRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u32 + 1 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u32 + 1 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestRead { data: b"{\"UnexpectedValue\"".to_vec(), position: 0 },
        scratch: Vec::new(),
        remaining_depth: 8,
    };

    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = &'de str;

        fn visit_enum<V>(self, _access: V) -> Result<Self::Value>
        where
            V: de::EnumAccess<'de>,
        {
            Ok("UnitVariant")
        }
    }

    let result: Result<&str> = deserializer.deserialize_enum("TestEnum", &["UnitVariant"], TestVisitor);
    assert!(result.is_err());
}

