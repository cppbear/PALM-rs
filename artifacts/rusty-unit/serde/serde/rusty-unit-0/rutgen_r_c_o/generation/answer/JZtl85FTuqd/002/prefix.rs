// Answer 0

#[test]
fn test_deserialize_newtype_struct_bool() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = ();
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, E>
        where
            V: Deserializer<'de>,
        {
            Ok(())
        }
    }

    let content = Content::Newtype(Box::new(Content::Bool(true)));
    let deserializer = ContentRefDeserializer::new(&content);
    deserializer.deserialize_newtype_struct("NewtypeBool", VisitorImpl);
}

#[test]
fn test_deserialize_newtype_struct_u8() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = ();
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, E>
        where
            V: Deserializer<'de>,
        {
            Ok(())
        }
    }

    let content = Content::Newtype(Box::new(Content::U8(0)));
    let deserializer = ContentRefDeserializer::new(&content);
    deserializer.deserialize_newtype_struct("NewtypeU8", VisitorImpl);
}

#[test]
fn test_deserialize_newtype_struct_string() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = ();
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, E>
        where
            V: Deserializer<'de>,
        {
            Ok(())
        }
    }

    let content = Content::Newtype(Box::new(Content::String("test".to_string())));
    let deserializer = ContentRefDeserializer::new(&content);
    deserializer.deserialize_newtype_struct("NewtypeString", VisitorImpl);
}

#[test]
fn test_deserialize_newtype_struct_char() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = ();
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, E>
        where
            V: Deserializer<'de>,
        {
            Ok(())
        }
    }

    let content = Content::Newtype(Box::new(Content::Char('a')));
    let deserializer = ContentRefDeserializer::new(&content);
    deserializer.deserialize_newtype_struct("NewtypeChar", VisitorImpl);
}

