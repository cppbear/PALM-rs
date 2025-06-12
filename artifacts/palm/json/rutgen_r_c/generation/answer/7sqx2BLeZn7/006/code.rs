// Answer 0

#[test]
fn test_deserialize_unit_success() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct MockDeserializer {
        input: Vec<u8>,
        index: usize,
        scratch: Vec<u8>,
    }

    impl<'de> Read<'de> for MockDeserializer {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, 0)
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
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    impl<'de> Deserializer<MockDeserializer> {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'n'))
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            assert_eq!(ident, b"ull");
            Ok(())
        }

        fn peek_error(&self, _reason: ErrorCode) -> Error {
            unimplemented!()
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = MockDeserializer {
        input: b"   null".to_vec(),
        index: 0,
        scratch: Vec::new(),
    };
    
    let result: Result<()> = deserializer.deserialize_unit(MockVisitor);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_deserialize_unit_eof_error() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct MockDeserializer {
        input: Vec<u8>,
        index: usize,
        scratch: Vec<u8>,
    }

    impl<'de> Read<'de> for MockDeserializer {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, 0)
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
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    impl<'de> Deserializer<MockDeserializer> {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }

        fn peek_error(&self, _reason: ErrorCode) -> Error {
            unimplemented!()
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = MockDeserializer {
        input: b"   ".to_vec(),
        index: 0,
        scratch: Vec::new(),
    };

    let _result: Result<()> = deserializer.deserialize_unit(MockVisitor);
}

