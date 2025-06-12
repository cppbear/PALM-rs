// Answer 0

#[test]
fn test_deserialize_borrowed() {
    use serde::de::{self, Visitor};
    use std::borrow::Cow;

    struct MyVisitor;

    impl<'de> Visitor<'de> for MyVisitor {
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

    struct MyDeserializer {
        value: Cow<'static, str>,
    }

    impl MyDeserializer {
        fn new(value: Cow<'static, str>) -> Self {
            MyDeserializer { value }
        }

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, V::Error>
        where
            V: Visitor<'de>,
        {
            match self.value {
                Cow::Borrowed(string) => visitor.visit_str(string),
                Cow::Owned(string) => visitor.visit_string(string),
            }
        }
    }

    let deserializer = MyDeserializer::new(Cow::Borrowed("test"));
    let result = deserializer.deserialize_any(MyVisitor).unwrap();
    assert_eq!(result, "test");
}

#[test]
fn test_deserialize_owned() {
    use serde::de::{self, Visitor};
    use std::borrow::Cow;

    struct MyVisitor;

    impl<'de> Visitor<'de> for MyVisitor {
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

    struct MyDeserializer {
        value: Cow<'static, str>,
    }

    impl MyDeserializer {
        fn new(value: Cow<'static, str>) -> Self {
            MyDeserializer { value }
        }

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, V::Error>
        where
            V: Visitor<'de>,
        {
            match self.value {
                Cow::Borrowed(string) => visitor.visit_str(string),
                Cow::Owned(string) => visitor.visit_string(string),
            }
        }
    }

    let deserializer = MyDeserializer::new(Cow::Owned("test".to_string()));
    let result = deserializer.deserialize_any(MyVisitor).unwrap();
    assert_eq!(result, "test");
}

