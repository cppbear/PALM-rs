// Answer 0

#[test]
fn test_deserialize_byte_buf_string() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = &'de str;

        fn visit_string(self, value: String) -> Result<Self::Value, Box<dyn std::error::Error>> {
            assert_eq!(value, "test_string");
            Ok("test_string")
        }

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            assert_eq!(value, "test_borrowed_str");
            Ok(value)
        }

        fn visit_byte_buf(self, value: Vec<u8>) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("Unexpected call to visit_byte_buf");
        }

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("Unexpected call to visit_borrowed_bytes");
        }

        fn visit_seq<V>(self, _visitor: &mut V) -> Result<Self::Value, Box<dyn std::error::Error>> 
        where
            V: SeqAccess<'de> {
            panic!("Unexpected call to visit_seq");
        }

        fn visit_some<V>(self, _value: V) -> Result<Self::Value, Box<dyn std::error::Error>> 
        where
            V: Deserialize<'de> {
            panic!("Unexpected call to visit_some");
        }

        fn visit_unit(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("Unexpected call to visit_unit");
        }

        fn visit_none(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("Unexpected call to visit_none");
        }

        fn visit_enum<V>(self, _value: V) -> Result<Self::Value, Box<dyn std::error::Error>> 
        where
            V: EnumAccess<'de> {
            panic!("Unexpected call to visit_enum");
        }

        fn visit_identifier(self, _value: &str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("Unexpected call to visit_identifier");
        }
    }

    let content_string = Content::String("test_string".to_string());
    let content_str = Content::Str("test_borrowed_str");
    
    let deserializer_string = ContentDeserializer {
        content: content_string,
        err: std::marker::PhantomData,
    };

    let deserializer_str = ContentDeserializer {
        content: content_str,
        err: std::marker::PhantomData,
    };

    let visitor = VisitorImpl;

    let result_string = deserializer_string.deserialize_byte_buf(visitor);
    assert!(result_string.is_ok());

    let result_str = deserializer_str.deserialize_byte_buf(visitor);
    assert!(result_str.is_ok());
}

#[test]
fn test_deserialize_byte_buf_bytes() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = Vec<u8>;

        fn visit_string(self, _value: String) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("Unexpected call to visit_string");
        }

        fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("Unexpected call to visit_borrowed_str");
        }

        fn visit_byte_buf(self, value: Vec<u8>) -> Result<Self::Value, Box<dyn std::error::Error>> {
            assert_eq!(value, vec![1, 2, 3]);
            Ok(value)
        }

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("Unexpected call to visit_borrowed_bytes");
        }

        fn visit_seq<V>(self, _visitor: &mut V) -> Result<Self::Value, Box<dyn std::error::Error>> 
        where
            V: SeqAccess<'de> {
            panic!("Unexpected call to visit_seq");
        }

        fn visit_some<V>(self, _value: V) -> Result<Self::Value, Box<dyn std::error::Error>> 
        where
            V: Deserialize<'de> {
            panic!("Unexpected call to visit_some");
        }

        fn visit_unit(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("Unexpected call to visit_unit");
        }

        fn visit_none(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("Unexpected call to visit_none");
        }

        fn visit_enum<V>(self, _value: V) -> Result<Self::Value, Box<dyn std::error::Error>> 
        where
            V: EnumAccess<'de> {
            panic!("Unexpected call to visit_enum");
        }

        fn visit_identifier(self, _value: &str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("Unexpected call to visit_identifier");
        }
    }

    let content_bytes = Content::Bytes(&[1, 2, 3]);
    
    let deserializer_bytes = ContentDeserializer {
        content: content_bytes,
        err: std::marker::PhantomData,
    };

    let visitor = VisitorImpl;

    let result = deserializer_bytes.deserialize_byte_buf(visitor);
    assert!(result.is_ok());
}

