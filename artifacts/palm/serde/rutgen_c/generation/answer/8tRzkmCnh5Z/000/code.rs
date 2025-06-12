// Answer 0

#[test]
fn test_deserialize_f32_with_f32_content() {
    struct TestVisitor {
        value: Option<f32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<f32>;

        fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a floating point value")
        }
    }

    let content = Content::F32(3.14);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_f32(TestVisitor { value: None });
    assert_eq!(result.unwrap(), Some(3.14));
}

#[test]
fn test_deserialize_f32_with_invalid_content() {
    struct TestVisitor {
        value: Option<f32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<f32>;

        fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value))
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a floating point value")
        }
    }

    let content = Content::String(String::from("not a float"));
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_f32(TestVisitor { value: None });
    assert!(result.is_err());
}

