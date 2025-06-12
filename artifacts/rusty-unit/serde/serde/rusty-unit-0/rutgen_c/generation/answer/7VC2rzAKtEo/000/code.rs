// Answer 0

#[test]
fn test_deserialize_i32() {
    struct TestVisitor {
        value: Option<i32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> 
        where 
            E: de::Error 
        {
            Ok(value)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E> 
        where 
            E: de::Error 
        {
            Ok(0) // Handle `None` to return a default value
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i32 value")
        }
    }

    let content = Content::I32(42);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let result: Result<i32, _> = deserializer.deserialize_i32(TestVisitor { value: None });
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_deserialize_i32_invalid() {
    struct InvalidVisitor;

    impl<'de> Visitor<'de> for InvalidVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i32 value")
        }

        fn visit_i32<E>(self, _value: i32) -> Result<Self::Value, E> 
        where 
            E: de::Error 
        {
            Err(E::custom("expected i32"))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E> 
        where 
            E: de::Error 
        {
            Err(E::custom("unexpected None"))
        }
    }

    let content = Content::String("not an i32".to_string());
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let result: Result<i32, _> = deserializer.deserialize_i32(InvalidVisitor);
    assert!(result.is_err());
}

