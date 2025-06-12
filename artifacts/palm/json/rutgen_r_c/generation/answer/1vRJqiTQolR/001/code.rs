// Answer 0

fn test_tuple_variant_empty_array() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let deserializer = VariantRefDeserializer { value: Some(&Value::Array(vec![])) };
    let result = deserializer.tuple_variant(0, MockVisitor);
    assert!(result.is_ok());
}

fn test_tuple_variant_non_empty_array() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Vec<()>;

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(vec![])
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Ok(vec![()])
        }
    }

    let deserializer = VariantRefDeserializer { value: Some(&Value::Array(vec![Value::Null])) };
    let result = deserializer.tuple_variant(1, MockVisitor);
    assert!(result.is_ok());
}

fn test_tuple_variant_invalid_type() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let deserializer = VariantRefDeserializer { value: Some(&Value::Bool(true)) };
    let result = deserializer.tuple_variant(0, MockVisitor);
    assert!(result.is_err());
}

fn test_tuple_variant_none() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let deserializer = VariantRefDeserializer { value: None };
    let result = deserializer.tuple_variant(0, MockVisitor);
    assert!(result.is_err());
}

