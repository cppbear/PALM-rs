// Answer 0

#[test]
fn test_deserialize_bytes_valid_case() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = Vec<u8>;

        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value> {
            Ok(vec![0xe5, 0x00, 0xe5])
        }

        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value> {
            Ok(vec![0xe5, 0x00, 0xe5])
        }
    }

    struct MockRead;
    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b' '))
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'"'))
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
        
        fn parse_str_raw<'s>(&mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(&[0xe5, 0x00, 0xe5]))
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: vec![],
        remaining_depth: 1,
    };

    let result: Result<Vec<u8>> = deserializer.deserialize_bytes(MockVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![0xe5, 0x00, 0xe5]);
}

#[test]
#[should_panic]
fn test_deserialize_bytes_invalid_case() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = Vec<u8>;

        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value> {
            Ok(vec![])
        }

        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value> {
            Ok(vec![])
        }
    }

    struct MockRead;
    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b' '))
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'['))
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
        
        fn parse_str_raw<'s>(&mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::from(ErrorCode::EofWhileParsingValue))
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: vec![],
        remaining_depth: 1,
    };

    let _result: Result<Vec<u8>> = deserializer.deserialize_bytes(MockVisitor);
}

