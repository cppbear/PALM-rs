// Answer 0

fn test_next_element_seed_ok() {
    struct MockSeed;

    impl<'de> de::DeserializeSeed<'de> for MockSeed {
        type Value = i32;
        
        fn deserialize<T>(self, _: &mut T) -> Result<Self::Value>
        where
            T: de::Deserializer<'de>,
        {
            Ok(42)
        }
    }

    struct MockReader;

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'['))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b']'))
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { 0 }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mock_reader = &mut MockReader;
    let mut seq_access = SeqAccess { de: mock_reader, first: true };
    let result = seq_access.next_element_seed(MockSeed);
    
    assert_eq!(result, Ok(Some(42)));
}

fn test_next_element_seed_no_next() {
    struct MockSeed;

    impl<'de> de::DeserializeSeed<'de> for MockSeed {
        type Value = i32;

        fn deserialize<T>(self, _: &mut T) -> Result<Self::Value>
        where
            T: de::Deserializer<'de>,
        {
            Ok(42)
        }
    }

    struct MockReader;

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b']'))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b']'))
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { 0 }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mock_reader = &mut MockReader;
    let mut seq_access = SeqAccess { de: mock_reader, first: true };
    let result = seq_access.next_element_seed(MockSeed);
    
    assert_eq!(result, Ok(None));
}

fn test_next_element_seed_err() {
    struct MockSeed;

    impl<'de> de::DeserializeSeed<'de> for MockSeed {
        type Value = i32;

        fn deserialize<T>(self, _: &mut T) -> Result<Self::Value>
        where
            T: de::Deserializer<'de>,
        {
            Err(Error::new())
        }
    }

    struct MockReader;

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'['))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b',')) // adjust to trigger error
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { 0 }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mock_reader = &mut MockReader;
    let mut seq_access = SeqAccess { de: mock_reader, first: true };
    let result = seq_access.next_element_seed(MockSeed);
    
    assert!(result.is_err());
}

