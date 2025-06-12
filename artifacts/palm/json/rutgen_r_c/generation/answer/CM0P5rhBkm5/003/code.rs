// Answer 0

#[test]
fn test_deserialize_option_none() {
    struct MockVisitor {
        called: bool,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = Option<()>;

        fn visit_none(self) -> Result<Self::Value> {
            self.called = true;
            Ok(None)
        }

        fn visit_some<V>(self, _value: V) -> Result<Self::Value>
        where
            V: de::Deserialize<'de>,
        {
            Err(Error)
        }
    }

    struct MockDeserializer {
        whitespace_status: Result<Option<u8>>,
    }

    impl<'de> Read<'de> for MockDeserializer {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(Some(b'n')) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = MockDeserializer {
        whitespace_status: Ok(Some(b'n')),
    };

    let visitor = MockVisitor { called: false };
    let result = deserializer.deserialize_option(visitor);

    assert!(result.is_ok());
    assert!(visitor.called);
}

#[test]
#[should_panic]
fn test_deserialize_option_expected_some() {
    struct MockVisitor {
        called: bool,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = Option<()>;

        fn visit_none(self) -> Result<Self::Value> {
            Err(Error)
        }

        fn visit_some<V>(self, _value: V) -> Result<Self::Value>
        where
            V: de::Deserialize<'de>,
        {
            self.called = true;
            Ok(Some(()))
        }
    }

    struct MockDeserializer {
        whitespace_status: Result<Option<u8>>,
    }

    impl<'de> Read<'de> for MockDeserializer {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = MockDeserializer {
        whitespace_status: Ok(Some(b'1')),
    };

    let visitor = MockVisitor { called: false };
    deserializer.deserialize_option(visitor);
}

#[test]
fn test_deserialize_option_err() {
    struct MockVisitor {
        called: bool,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = Option<()>;

        fn visit_none(self) -> Result<Self::Value> {
            Err(Error)
        }

        fn visit_some<V>(self, _value: V) -> Result<Self::Value>
        where
            V: de::Deserialize<'de>,
        {
            self.called = true;
            Ok(Some(()))
        }
    }

    struct MockDeserializer {
        whitespace_status: Result<Option<u8>>,
    }

    impl<'de> Read<'de> for MockDeserializer {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Err(Error) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = MockDeserializer {
        whitespace_status: Err(Error),
    };

    let visitor = MockVisitor { called: false };
    let result = deserializer.deserialize_option(visitor);

    assert!(result.is_err());
}

