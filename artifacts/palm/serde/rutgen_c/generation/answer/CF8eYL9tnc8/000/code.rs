// Answer 0

#[test]
fn test_deserialize_enum_with_map() {
    struct TestVisitor {
        variant: Option<Content<'static>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = (Content<'de>, Option<Content<'de>>);

        fn visit_enum<V>(self, _visitor: V) -> Result<Self::Value, <ContentDeserializer<'de, E> as Deserializer<'de>>::Error>
        where
            V: Visitor<'de, Value = Self::Value>,
        {
            Ok((self.variant.unwrap(), None))
        }

        // Other required methods for the Visitor trait can be defined as needed
    }

    let content = Content::Map(vec![
        (Content::String("variant_name".to_string()), Content::U8(1)),
    ]);

    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let visitor = TestVisitor { variant: Some(Content::String("variant_name".to_string())) };
    let result: Result<(Content<'static>, Option<Content<'static>>), _> = deserializer.deserialize_enum("my_enum", &["variant_name"], visitor);

    assert!(result.is_ok());
}

#[test]
fn test_deserialize_enum_with_string() {
    struct TestVisitor {
        received: Option<Content<'static>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = (Content<'de>, Option<Content<'de>>);

        fn visit_enum<V>(self, _visitor: V) -> Result<Self::Value, <ContentDeserializer<'de, E> as Deserializer<'de>>::Error>
        where
            V: Visitor<'de, Value = Self::Value>,
        {
            Ok((self.received.unwrap(), None))
        }

        // Other required methods for the Visitor trait can be defined as needed
    }

    let content = Content::String("variant_name".to_string());

    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let visitor = TestVisitor { received: Some(Content::String("variant_name".to_string())) };
    
    let result: Result<(Content<'static>, Option<Content<'static>>), _> = deserializer.deserialize_enum("my_enum", &["variant_name"], visitor);

    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "invalid value")]
fn test_deserialize_enum_with_invalid_type() {
    struct InvalidVisitor;

    impl<'de> Visitor<'de> for InvalidVisitor {
        type Value = ();

        fn visit_enum<V>(self, _visitor: V) -> Result<Self::Value, <ContentDeserializer<'de, E> as Deserializer<'de>>::Error>
        where
            V: Visitor<'de, Value = Self::Value>,
        {
            Err(de::Error::invalid_value(de::Unexpected::Bool(true), &"string or map"))
        }
    }

    let content = Content::Bool(true);

    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let visitor = InvalidVisitor;
    let _ = deserializer.deserialize_enum("my_enum", &["variant_name"], visitor);
}

