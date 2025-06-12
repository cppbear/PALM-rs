// Answer 0

#[test]
fn test_invalid_type_with_bool() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a dummy visitor")
        }
        fn visit_unit(self) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    let content = Content::Bool(true);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let result = deserializer.invalid_type(&DummyVisitor);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Bool(true), &DummyVisitor));
}

#[test]
fn test_invalid_type_with_u8() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a dummy visitor")
        }
        fn visit_unit(self) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    let content = Content::U8(10);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let result = deserializer.invalid_type(&DummyVisitor);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Unsigned(10), &DummyVisitor));
}

#[test]
fn test_invalid_type_with_str() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a dummy visitor")
        }
        fn visit_unit(self) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    let content = Content::Str("unexpected string");
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let result = deserializer.invalid_type(&DummyVisitor);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Str("unexpected string"), &DummyVisitor));
}

#[test]
fn test_invalid_type_with_unit() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a dummy visitor")
        }
        fn visit_unit(self) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    let content = Content::Unit;
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let result = deserializer.invalid_type(&DummyVisitor);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Unit, &DummyVisitor));
}

