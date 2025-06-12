// Answer 0

#[test]
fn test_deserialize_any_borrowed_str() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = &'de str;

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            assert_eq!(value, "test");
            Ok(value)
        }

        fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E> {
            unimplemented!()
        }
    }

    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for MockRead {
        fn parse_str(&mut self, output: &mut Vec<u8>) -> Result<Reference, Error> {
            output.extend_from_slice(&self.data[self.index..self.index + 4]);
            self.index += 4;
            Ok(Reference::Borrowed(b"test"))
        }

        fn discard(&mut self) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: b"test".to_vec(), index: 0 },
        scratch: vec![],
        remaining_depth: 0,
    };

    let result = deserializer.deserialize_any(TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_any_copied_str() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_borrowed_str<E>(self, _value: &'de str) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            assert_eq!(value, "test");
            Ok(value.to_string())
        }
    }

    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for MockRead {
        fn parse_str(&mut self, output: &mut Vec<u8>) -> Result<Reference, Error> {
            output.extend_from_slice(b"test");
            Ok(Reference::Copied(b"test"))
        }

        fn discard(&mut self) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: b"test".to_vec(), index: 0 },
        scratch: vec![],
        remaining_depth: 0,
    };

    let result = deserializer.deserialize_any(TestVisitor);
    assert!(result.is_ok());
}

