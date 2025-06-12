// Answer 0

#[test]
fn test_deserialize_identifier_byte_buf() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_string(self, value: String) -> Result<Self::Value, serde::de::value::Error> {
            assert_eq!(value, "test string");
            Ok(value)
        }

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, serde::de::value::Error> {
            unreachable!("This method should not be called in this test");
        }

        fn visit_byte_buf(self, value: Vec<u8>) -> Result<Self::Value, serde::de::value::Error> {
            assert_eq!(value, b"test byte buf".to_vec());
            Ok(String::from_utf8(value).unwrap())
        }

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, serde::de::value::Error> {
            unreachable!("This method should not be called in this test");
        }

        fn visit_u8(self, value: u8) -> Result<Self::Value, serde::de::value::Error> {
            unreachable!("This method should not be called in this test");
        }

        fn visit_u64(self, value: u64) -> Result<Self::Value, serde::de::value::Error> {
            unreachable!("This method should not be called in this test");
        }

        fn visit_unit(self) -> Result<Self::Value, serde::de::value::Error> {
            unreachable!("This method should not be called in this test");
        }
    }

    let content = Content::ByteBuf(b"test byte buf".to_vec());
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    let result = deserializer.deserialize_identifier(TestVisitor).unwrap();
    assert_eq!(result, "test byte buf");
}

#[test]
fn test_deserialize_identifier_invalid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_string(self, value: String) -> Result<Self::Value, serde::de::value::Error> {
            unreachable!("This method should not be called in this test");
        }

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, serde::de::value::Error> {
            unreachable!("This method should not be called in this test");
        }

        fn visit_byte_buf(self, value: Vec<u8>) -> Result<Self::Value, serde::de::value::Error> {
            unreachable!("This method should not be called in this test");
        }

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, serde::de::value::Error> {
            unreachable!("This method should not be called in this test");
        }

        fn visit_u8(self, value: u8) -> Result<Self::Value, serde::de::value::Error> {
            unreachable!("This method should not be called in this test");
        }

        fn visit_u64(self, value: u64) -> Result<Self::Value, serde::de::value::Error> {
            unreachable!("This method should not be called in this test");
        }

        fn visit_unit(self) -> Result<Self::Value, serde::de::value::Error> {
            unreachable!("This method should not be called in this test");
        }
    }

    let content = Content::U8(255);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    let result = deserializer.deserialize_identifier(TestVisitor);
    assert!(result.is_err());
}

