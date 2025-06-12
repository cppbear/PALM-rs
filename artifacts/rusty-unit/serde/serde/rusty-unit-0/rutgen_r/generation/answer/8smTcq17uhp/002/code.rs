// Answer 0

#[test]
fn test_deserialize_any_borrowed() {
    use serde::de::{self, Visitor};
    use std::borrow::Cow;
    
    struct TestVisitor {
        value: String,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> 
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E> 
        where
            E: de::Error,
        {
            Ok(value)
        }
    }

    struct TestDeserializer {
        value: Cow<'static, str>,
    }

    impl TestDeserializer {
        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, &'static str>
        where
            V: Visitor<'de>,
        {
            match self.value {
                Cow::Borrowed(string) => visitor.visit_str(string),
                Cow::Owned(string) => visitor.visit_string(string),
            }
        }
    }

    // Test with Cow::Borrowed
    let deserializer = TestDeserializer {
        value: Cow::Borrowed("test string"),
    };
    let visitor = TestVisitor { value: String::new() };
    let result = deserializer.deserialize_any(visitor).unwrap();
    
    assert_eq!(result, "test string");
}

#[test]
fn test_deserialize_any_owned() {
    use serde::de::{self, Visitor};
    use std::borrow::Cow;

    struct TestVisitor {
        value: String,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> 
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E> 
        where
            E: de::Error,
        {
            Ok(value)
        }
    }

    struct TestDeserializer {
        value: Cow<'static, str>,
    }

    impl TestDeserializer {
        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, &'static str>
        where
            V: Visitor<'de>,
        {
            match self.value {
                Cow::Borrowed(string) => visitor.visit_str(string),
                Cow::Owned(string) => visitor.visit_string(string),
            }
        }
    }

    // Test with Cow::Owned
    let deserializer = TestDeserializer {
        value: Cow::Owned("owned test string".to_string()),
    };
    let visitor = TestVisitor { value: String::new() };
    let result = deserializer.deserialize_any(visitor).unwrap();

    assert_eq!(result, "owned test string");
}

