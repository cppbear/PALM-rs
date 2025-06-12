// Answer 0

#[test]
fn test_deserialize_identifier_string() {
    struct MockVisitor {
        value: Result<String, ()>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = String;

        fn visit_string(self, value: String) -> Result<Self::Value, ()> {
            Ok(value)
        }

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> {
            Err(())
        }
    }

    enum Content {
        String(String),
        Str(&'static str),
        ByteBuf(Vec<u8>),
        Bytes(&'static [u8]),
        U8(u8),
        U64(u64),
        Invalid,
    }

    struct Deserializer {
        content: Content,
    }

    impl Deserializer {
        fn invalid_type<V>(&self, _: &V) -> () {
            panic!("Invalid type")
        }

        fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, ()>
        where
            V: Visitor<'_>,
        {
            match self.content {
                Content::String(v) => visitor.visit_string(v),
                Content::Str(v) => visitor.visit_borrowed_str(v),
                Content::ByteBuf(v) => visitor.visit_byte_buf(v),
                Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
                Content::U8(v) => visitor.visit_u8(v),
                Content::U64(v) => visitor.visit_u64(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    let deserializer = Deserializer {
        content: Content::String("test".to_string()),
    };
    let visitor = MockVisitor { value: Ok("test".to_string()) };

    let result = deserializer.deserialize_identifier(visitor);

    assert_eq!(result, Ok("test".to_string()));
}

#[test]
fn test_deserialize_identifier_u8() {
    struct MockVisitor {
        value: Result<u8, ()>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = u8;

        fn visit_string(self, _: String) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_u8(self, value: u8) -> Result<Self::Value, ()> {
            Ok(value)
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> {
            Err(())
        }
    }

    struct Deserializer {
        content: Content,
    }

    let deserializer = Deserializer {
        content: Content::U8(42),
    };
    let visitor = MockVisitor { value: Ok(42) };

    let result = deserializer.deserialize_identifier(visitor);

    assert_eq!(result, Ok(42));
}

