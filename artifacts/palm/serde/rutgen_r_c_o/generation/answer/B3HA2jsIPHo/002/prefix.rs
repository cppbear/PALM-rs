// Answer 0

#[test]
fn test_struct_variant_with_empty_map() {
    struct TestVisitor;
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an empty map")
        }
        fn visit_map<V>(self, _map: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    let deserializer = VariantDeserializer {
        value: Some(Content::Map(Vec::new())),
        err: PhantomData::<()>,
    };
    deserializer.struct_variant(&[], TestVisitor);
}

#[test]
fn test_struct_variant_with_non_empty_map() {
    struct TestVisitor;
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a non-empty map")
        }
        fn visit_map<V>(self, _map: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    let deserializer = VariantDeserializer {
        value: Some(Content::Map(vec![
            (Content::String("key".to_string()), Content::String("value".to_string()))
        ])),
        err: PhantomData::<()>,
    };
    deserializer.struct_variant(&[], TestVisitor);
}

#[test]
fn test_struct_variant_with_seq() {
    struct TestVisitor;
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence")
        }
        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let deserializer = VariantDeserializer {
        value: Some(Content::Seq(vec![
            Content::U8(0),
            Content::U8(1)
        ])),
        err: PhantomData::<()>,
    };
    deserializer.struct_variant(&[], TestVisitor);
}

#[test]
fn test_struct_variant_with_none() {
    struct TestVisitor;
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a unit variant")
        }
        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(())
        }
    }

    let deserializer = VariantDeserializer {
        value: None,
        err: PhantomData::<()>,
    };
    let result = deserializer.struct_variant(&[], TestVisitor);
    assert!(result.is_err());
}

