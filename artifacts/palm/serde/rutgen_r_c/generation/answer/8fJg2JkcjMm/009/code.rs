// Answer 0

#[test]
fn test_deserialize_any_string() {
    use serde::de::{Visitor, Error};
    
    struct StringVisitor {
        visited: bool,
        result: Option<String>,
    }

    impl<'de> Visitor<'de> for StringVisitor {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(value.to_string())
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(value.to_string())
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(E::custom("Expected string but found unit"))
        }
    }

    let content = Content::Str("test".into());
    let deserializer = ContentRefDeserializer::new(&content);
    let visitor = StringVisitor { visited: false, result: None };

    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test".to_string());
}

#[test]
fn test_deserialize_any_bytes() {
    use serde::de::{Visitor, Error};

    struct BytesVisitor {
        visited: bool,
        result: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for BytesVisitor {
        type Value = Vec<u8>;

        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(value.to_vec())
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(value.to_vec())
        }
        
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(E::custom("Expected bytes, found unit"))
        }
    }

    let content = Content::Bytes(vec![1, 2, 3]);
    let deserializer = ContentRefDeserializer::new(&content);
    let visitor = BytesVisitor { visited: false, result: None };

    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_any_none() {
    use serde::de::{Visitor, Error};

    struct NoneVisitor {
        visited: bool,
    }

    impl<'de> Visitor<'de> for NoneVisitor {
        type Value = ();

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(())
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(E::custom("Expected none, found unit"))
        }
    }

    let content = Content::None;
    let deserializer = ContentRefDeserializer::new(&content);
    let visitor = NoneVisitor { visited: false };

    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_ok());
}

