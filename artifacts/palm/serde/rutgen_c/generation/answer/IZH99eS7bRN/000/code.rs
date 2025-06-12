// Answer 0

#[test]
fn test_deserialize_enum_with_map() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = (&'de str, Option<&'de str>);
        
        fn visit_enum<V>(self, _visitor: EnumRefDeserializer<'de, ()>) -> Result<Self::Value, ()> {
            Ok(("test_variant", Some("test_value")))
        }

        fn visit_str(self, value: &'de str) -> Result<Self::Value, ()> {
            Ok((value, None))
        }
        
        fn visit_unit(self) -> Result<Self::Value, ()> {
            Ok(("", None))
        }
    }

    let content = Content::Map(vec![
        (Content::Str("test_variant".into()), Content::Str("test_value".into())),
    ]);

    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let result: Result<(&str, Option<&str>), ()> = deserializer.deserialize_enum("TestEnum", &["test_variant"], TestVisitor);
    
    assert_eq!(result.unwrap(), ("test_variant", Some("test_value")));
}

#[test]
fn test_deserialize_enum_with_string() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = &'de str;

        fn visit_enum<V>(self, _visitor: EnumRefDeserializer<'de, ()>) -> Result<Self::Value, ()> {
            Ok("test_enum")
        }
        
        fn visit_str(self, value: &'de str) -> Result<Self::Value, ()> {
            Ok(value)
        }
        
        fn visit_unit(self) -> Result<Self::Value, ()> {
            Ok("")
        }
    }

    let content = Content::String("test_enum".into());

    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let result: Result<&str, ()> = deserializer.deserialize_enum("TestEnum", &["test_variant"], TestVisitor);

    assert_eq!(result.unwrap(), "test_enum");
}

#[test]
#[should_panic(expected = "error with message about map with a single key")]
fn test_deserialize_enum_with_multiple_keys() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = (&'de str, Option<&'de str>);
        
        fn visit_enum<V>(self, _visitor: EnumRefDeserializer<'de, ()>) -> Result<Self::Value, ()> {
            Ok(("variant", Some("value")))
        }

        fn visit_str(self, value: &'de str) -> Result<Self::Value, ()> {
            Ok((value, None))
        }
        
        fn visit_unit(self) -> Result<Self::Value, ()> {
            Ok(("", None))
        }
    }

    let content = Content::Map(vec![
        (Content::Str("test_variant".into()), Content::Str("test_value".into())),
        (Content::Str("another_variant".into()), Content::Str("another_value".into())),
    ]);

    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let _result: Result<(&str, Option<&str>), ()> = deserializer.deserialize_enum("TestEnum", &["test_variant"], TestVisitor);
}

#[test]
#[should_panic(expected = "invalid type expected")]
fn test_deserialize_enum_with_invalid_type() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = (&'de str, Option<&'de str>);

        fn visit_unit(self) -> Result<Self::Value, ()> {
            Ok(("", None))
        }
    }

    let content = Content::U8(42);

    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let _result: Result<(&str, Option<&str>), ()> = deserializer.deserialize_enum("TestEnum", &["test_variant"], TestVisitor);
}

