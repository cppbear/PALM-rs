// Answer 0

#[test]
fn test_next_value_seed() {
    struct TestDeserializer;
    
    impl<'de> Read<'de> for TestDeserializer {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
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

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::from_str("test"))
        }
        
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::from_bytes(&[b't', b'e', b's', b't']))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }
    
    struct DummyVisitor;

    impl<'de> de::Visitor<'de> for DummyVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }

    let mut deserializer = Deserializer { 
        read: TestDeserializer, 
        scratch: vec![], 
        remaining_depth: 10 
    };
    
    let seed = DummyVisitor;
    let result = deserializer.next_value_seed(seed);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test");
}

