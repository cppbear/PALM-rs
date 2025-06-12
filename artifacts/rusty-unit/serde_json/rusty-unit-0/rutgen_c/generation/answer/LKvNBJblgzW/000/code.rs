// Answer 0

#[test]
fn test_deserialize_ignored_any() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct MockReader;

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'n'))
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
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = MockReader;
    let mut deserializer = Deserializer {
        read: reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result: Result<()> = deserializer.deserialize_ignored_any(MockVisitor);
    assert!(result.is_ok());
}

