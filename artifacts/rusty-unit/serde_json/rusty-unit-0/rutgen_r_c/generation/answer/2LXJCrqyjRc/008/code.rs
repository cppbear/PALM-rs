// Answer 0

#[test]
fn test_do_deserialize_i128_success() {
    struct MockVisitor {
        value: Option<i128>,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = i128;

        fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct MockRead;

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b' ')) // simulating whitespace
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            todo!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let visitor = MockVisitor { value: None };
    let result: Result<i128> = deserializer.do_deserialize_i128(visitor);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_do_deserialize_i128_eof_error() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = i128;

        fn visit_i128<E>(self, _value: i128) -> Result<Self::Value, E> {
            unimplemented!()
        }
    }

    struct MockRead;

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None) // simulating EOF
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            todo!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let visitor = MockVisitor;
    let _result: Result<i128> = deserializer.do_deserialize_i128(visitor);
}

#[test]
fn test_do_deserialize_i128_out_of_range_error() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = i128;

        fn visit_i128<E>(self, _value: i128) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Out of range"))
        }
    }

    struct MockRead;

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'0')) // valid number start
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            todo!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let visitor = MockVisitor;
    let result: Result<i128> = deserializer.do_deserialize_i128(visitor);
    assert!(result.is_err());
}

