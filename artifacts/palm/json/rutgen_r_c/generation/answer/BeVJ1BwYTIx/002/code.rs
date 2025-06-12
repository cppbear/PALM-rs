// Answer 0

fn test_variant_seed_success() {
    struct MockVisitor {
        value: i32,
    }

    impl<'de> de::DeserializeSeed<'de> for MockVisitor {
        type Value = i32;

        fn deserialize<Deser>(&self, deserializer: &mut Deser) -> Result<Self::Value>
        where
            Deser: de::Deserializer<'de>,
        {
            Ok(self.value)
        }
    }

    struct MockRead;

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'a'))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'a'))
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, 0)
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: Vec::new(),
        remaining_depth: 10,
        #[cfg(feature = "float_roundtrip")]
        single_precision: true,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let access = UnitVariantAccess { de: &mut deserializer };
    let visitor = MockVisitor { value: 42 };
    
    let result = access.variant_seed(visitor).unwrap();
    
    assert_eq!(result, (42, access));
}

fn test_variant_seed_failure() {
    struct FailingVisitor;

    impl<'de> de::DeserializeSeed<'de> for FailingVisitor {
        type Value = i32;

        fn deserialize<Deser>(&self, _deserializer: &mut Deser) -> Result<Self::Value> 
        where
            Deser: de::Deserializer<'de>, 
        {
            Err(Error)
        }
    }

    struct MockRead;

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'a'))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'a'))
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, 0)
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: Vec::new(),
        remaining_depth: 10,
        #[cfg(feature = "float_roundtrip")]
        single_precision: true,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let access = UnitVariantAccess { de: &mut deserializer };
    let visitor = FailingVisitor;

    let result = access.variant_seed(visitor);
    
    assert!(result.is_err());
}

