// Answer 0

#[test]
fn test_deserialize_byte_buf_with_bytes() {
    struct TestVisitor {
        result: Option<Vec<u8>>,
    }
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_bytes(self, value: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Ok(value.to_vec())
        }

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Ok(value.to_vec())
        }

        fn visit_string(self, _: String) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("visit_string not expected"))
        }
        
        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("visit_borrowed_str not expected"))
        }

        fn visit_byte_buf(self, value: Vec<u8>) -> Result<Self::Value, serde::de::Error> {
            Ok(value)
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("visit_seq not expected"))
        }
        
        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("visit_unit not expected"))
        }

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("visit_none not expected"))
        }
    }

    let content = Content::Bytes(&[1, 2, 3, 4]);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let visitor = TestVisitor { result: None };
    let result = deserializer.deserialize_byte_buf(visitor).unwrap();
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_deserialize_byte_buf_with_byte_buf() {
    struct TestVisitor {
        result: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_string(self, _: String) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("visit_string not expected"))
        }

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("visit_borrowed_str not expected"))
        }

        fn visit_byte_buf(self, value: Vec<u8>) -> Result<Self::Value, serde::de::Error> {
            Ok(value)
        }
        
        fn visit_bytes(self, _: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("visit_bytes not expected"))
        }

        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("visit_borrowed_bytes not expected"))
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("visit_seq not expected"))
        }
        
        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("visit_unit not expected"))
        }

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("visit_none not expected"))
        }
    }

    let content = Content::ByteBuf(vec![5, 6, 7, 8]);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let visitor = TestVisitor { result: None };
    let result = deserializer.deserialize_byte_buf(visitor).unwrap();
    assert_eq!(result, vec![5, 6, 7, 8]);
}

