// Answer 0

#[test]
fn test_deserialize_bytes_with_valid_utf8() {
    struct MockReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'_> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                self.pos += 1;
                Ok(Some(self.data[self.pos - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.data.len() {
                self.pos += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.pos as usize + 1 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.pos as usize + 1 }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            Ok(Reference::Borrowed(&self.data))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let json_data = b"\"some bytes: \xe5\x00\xe5\"";
    let mut reader = MockReader { data: json_data.to_vec(), pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 10 };
    let result: Result<Vec<u8>> = deserializer.deserialize_bytes(MockVisitor);
    assert!(result.is_ok());
    let bytes = result.unwrap();
    assert_eq!(bytes, vec![0xe5, 0x00, 0xe5]);
}

struct MockVisitor;

impl de::Visitor<'_> for MockVisitor {
    type Value = Result<Vec<u8>>;

    fn visit_borrowed_bytes(self, _v: &[u8]) -> Self::Value {
        Ok(_v.to_vec())
    }

    fn visit_bytes(self, _v: &[u8]) -> Self::Value {
        Ok(_v.to_vec())
    }
}

#[test]
fn test_deserialize_bytes_with_invalid_utf8() {
    struct MockReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'_> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                self.pos += 1;
                Ok(Some(self.data[self.pos - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.data.len() {
                self.pos += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.pos as usize + 1 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.pos as usize + 1 }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::InvalidUnicodeCodePoint, 1, 1))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let json_data = b"\"invalid unicode: \\uD801\"";
    let mut reader = MockReader { data: json_data.to_vec(), pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 10 };
    let result: Result<Vec<u8>> = deserializer.deserialize_bytes(MockVisitor);
    assert!(result.is_err());
}

