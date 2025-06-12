// Answer 0

#[test]
fn test_struct_variant_with_object() {
    use serde::de::Deserializer;
    use serde::de::MapAccess;
    use serde::de::Visitor;

    struct TestVisitor {
        visited: bool,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("any struct")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, Error>
        where
            M: MapAccess<'de>,
        {
            Ok(())
        }
    }

    let obj = Map {
        map: MapImpl::new(),
    };

    let variant_ref = VariantRefDeserializer {
        value: Some(&Value::Object(obj)),
    };

    let visitor = TestVisitor { visited: false };

    let result = variant_ref.struct_variant(&[], visitor);
    assert!(result.is_ok());
}

#[test]
fn test_struct_variant_with_non_object() {
    let variant_ref = VariantRefDeserializer {
        value: Some(&Value::Bool(true)),
    };

    let visitor = TestVisitor { visited: false };

    let result = variant_ref.struct_variant(&[], visitor);
    assert!(result.is_err());
}

#[test]
fn test_struct_variant_with_none() {
    let variant_ref = VariantRefDeserializer {
        value: None,
    };

    let visitor = TestVisitor { visited: false };

    let result = variant_ref.struct_variant(&[], visitor);
    assert!(result.is_err());
}

