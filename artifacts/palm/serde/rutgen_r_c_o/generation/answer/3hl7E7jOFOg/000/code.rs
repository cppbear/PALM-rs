// Answer 0

#[test]
fn test_deserialize_byte_buf_string() {
    struct TestVisitor;
    
    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;
        
        fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
            Ok(value)
        }
        
        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value.to_string())
        }
        
        fn visit_byte_buf<E>(self, _value: Vec<u8>) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Unexpected ByteBuf"))
        }
        
        fn visit_borrowed_bytes<E>(self, _value: &'de [u8]) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Unexpected Borrowed Bytes"))
        }
        
        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("Unexpected Sequence"))
        }
    }

    let content = Content::String("test".to_string());
    let result = deserialize_byte_buf(content, TestVisitor);
    assert_eq!(result.unwrap(), "test");
}

#[test]
fn test_deserialize_byte_buf_str() {
    struct TestVisitor;
    
    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_string<E>(self, _value: String) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Unexpected String"))
        }
        
        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value.to_string())
        }
        
        fn visit_byte_buf<E>(self, _value: Vec<u8>) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Unexpected ByteBuf"))
        }
        
        fn visit_borrowed_bytes<E>(self, _value: &'de [u8]) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Unexpected Borrowed Bytes"))
        }
        
        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("Unexpected Sequence"))
        }
    }

    let content = Content::Str("test");
    let result = deserialize_byte_buf(content, TestVisitor);
    assert_eq!(result.unwrap(), "test");
}

#[test]
fn test_deserialize_byte_buf_byte_buf() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_string<E>(self, _value: String) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Unexpected String"))
        }
        
        fn visit_borrowed_str<E>(self, _value: &'de str) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Unexpected Borrowed Str"))
        }
        
        fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E> {
            Ok(value)
        }
        
        fn visit_borrowed_bytes<E>(self, _value: &'de [u8]) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Unexpected Borrowed Bytes"))
        }
        
        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("Unexpected Sequence"))
        }
    }

    let content = Content::ByteBuf(vec![1, 2, 3]);
    let result = deserialize_byte_buf(content, TestVisitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_byte_buf_bytes() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_string<E>(self, _value: String) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Unexpected String"))
        }
        
        fn visit_borrowed_str<E>(self, _value: &'de str) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Unexpected Borrowed Str"))
        }

        fn visit_byte_buf<E>(self, _value: Vec<u8>) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Unexpected ByteBuf"))
        }
        
        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }

        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("Unexpected Sequence"))
        }
    }

    let content = Content::Bytes(&[1, 2, 3][..]);
    let result = deserialize_byte_buf(content, TestVisitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

