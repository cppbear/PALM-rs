// Answer 0

fn test_deserialize_seq_valid_input() {
    struct MockVisitor;

    impl serde::de::Visitor<'_> for MockVisitor {
        type Value = ();

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value>
        where
            V: serde::de::SeqAccess<'_>,
        {
            Ok(())
        }
    }

    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { unimplemented!() }

        fn peek_position(&self) -> Position { unimplemented!() }

        fn byte_offset(&self) -> usize { self.index }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: b"    [1, 2, 3]".to_vec(), index: 0 },
        scratch: vec![],
        remaining_depth: 1,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.deserialize_seq(MockVisitor);
    assert!(result.is_ok());
}

fn test_deserialize_seq_eof_error() {
    struct MockVisitor;

    impl serde::de::Visitor<'_> for MockVisitor {
        type Value = ();

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value>
        where
            V: serde::de::SeqAccess<'_>,
        {
            Ok(())
        }
    }

    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(Some(b'[')) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.index }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: vec![], index: 0 },
        scratch: vec![],
        remaining_depth: 1,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.deserialize_seq(MockVisitor);
    assert_eq!(result.unwrap_err().code, ErrorCode::EofWhileParsingValue);
}

fn test_deserialize_seq_recursion_limit_exceeded() {
    struct MockVisitor;

    impl serde::de::Visitor<'_> for MockVisitor {
        type Value = ();

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value>
        where
            V: serde::de::SeqAccess<'_>,
        {
            Ok(())
        }
    }

    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(Some(b'[')) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(Some(b'[')) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.index }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: vec![], index: 0 },
        scratch: vec![],
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.deserialize_seq(MockVisitor);
    assert_eq!(result.unwrap_err().code, ErrorCode::RecursionLimitExceeded);
}

