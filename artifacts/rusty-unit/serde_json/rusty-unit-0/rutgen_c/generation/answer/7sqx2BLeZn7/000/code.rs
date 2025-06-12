// Answer 0

#[test]
fn test_deserialize_unit() {
    struct MockReader {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index >= self.input.len() {
                Ok(None)
            } else {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index >= self.input.len() {
                Ok(None)
            } else {
                Ok(Some(self.input[self.index]))
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.index as u32 + 1 }
        }

        fn peek_position(&self) -> Position {
            self.position() // same as current position
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let input = b"null".to_vec();
    let mut reader = MockReader { input, index: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_bool(self, _v: bool) -> Result<Self::Value> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }
    }

    let result = deserializer.deserialize_unit(Visitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_unit_empty_input() {
    struct MockReader {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index >= self.input.len() {
                Ok(None)
            } else {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index >= self.input.len() {
                Ok(None)
            } else {
                Ok(Some(self.input[self.index]))
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.index as u32 + 1 }
        }

        fn peek_position(&self) -> Position {
            self.position() // same as current position
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let input = b"".to_vec();
    let mut reader = MockReader { input, index: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_bool(self, _v: bool) -> Result<Self::Value> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }
    }

    let result = deserializer.deserialize_unit(Visitor);
    assert!(result.is_err());
}

