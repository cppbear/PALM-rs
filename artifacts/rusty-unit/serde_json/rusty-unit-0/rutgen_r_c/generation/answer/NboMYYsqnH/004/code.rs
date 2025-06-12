// Answer 0

#[test]
fn test_do_deserialize_u128_valid() {
    struct MockVisitor {
        result: Option<u128>,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = u128;

        fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct MockRead;

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(Some(b' ')) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 1, column: 1 } }
        fn peek_position(&self) -> Position { Position { line: 1, column: 1 } }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: Vec::new(),
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let visitor = MockVisitor { result: None };
    let result = deserializer.do_deserialize_u128(visitor);
    assert!(result.is_ok() && result.unwrap() == 0);
}

#[test]
#[should_panic]
fn test_do_deserialize_u128_negative() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = u128;

        fn visit_u128<E>(self, _value: u128) -> Result<Self::Value, E> {
            unimplemented!()
        }
    }

    struct MockRead;

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(Some(b'-')) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 1, column: 1 } }
        fn peek_position(&self) -> Position { Position { line: 1, column: 1 } }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: Vec::new(),
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let visitor = MockVisitor;
    let _result = deserializer.do_deserialize_u128(visitor);
}

#[test]
fn test_do_deserialize_u128_eof() {
    struct MockVisitor {
        result: Option<u128>,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = u128;

        fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct MockRead;

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 1, column: 1 } }
        fn peek_position(&self) -> Position { Position { line: 1, column: 1 } }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: Vec::new(),
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let visitor = MockVisitor { result: None };
    let result = deserializer.do_deserialize_u128(visitor);
    assert!(result.is_err());
}

