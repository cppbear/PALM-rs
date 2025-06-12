// Answer 0

#[test]
fn test_deserialize_ignored_any_error_case_1() {
    struct MockReader {
        state: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.state < 3 {
                self.state += 1;
                Ok(Some(b'1'))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'1'))
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            self.state
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::new())
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::new())
        }

        fn ignore_str(&mut self) -> Result<()> {
            Err(Error::new())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::new())
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MockReader { state: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };
    let visitor = FakeVisitor;

    let result = deserializer.deserialize_ignored_any(visitor);
}

#[test]
fn test_deserialize_ignored_any_error_case_2() {
    struct MockReader {
        should_fail: bool,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'0'))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'0'))
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
            if self.should_fail {
                Err(Error::new())
            } else {
                Ok(Reference::new())
            }
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::new())
        }

        fn ignore_str(&mut self) -> Result<()> {
            Err(Error::new())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::new())
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MockReader { should_fail: true };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };
    let visitor = FakeVisitor;

    let result = deserializer.deserialize_ignored_any(visitor);
}

struct FakeVisitor;

impl<'de> de::Visitor<'de> for FakeVisitor {
    type Value = ();

    fn visit_unit(self) -> Result<Self::Value> {
        Ok(())
    }

    forward_to_deserialize_any! {
        bool, char, str, string, bytes, byte_buf, option, newtype_struct, seq, tuple,
        tuple_struct, map, struct, enum, identifier, ignored_any
    }
}

