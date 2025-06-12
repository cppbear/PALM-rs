// Answer 0

#[test]
fn test_deserialize_map_with_object() {
    struct MockVisitor {
        result: Option<()>,
    }

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an object")
        }

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }
    }

    let object_value = Value::Object(Map { map: MapImpl::new() });
    let result = object_value.deserialize_map(MockVisitor { result: None });

    assert!(result.is_ok());
}

#[test]
fn test_deserialize_map_with_non_object() {
    struct MockVisitor {
        result: Option<()>,
    }

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an object")
        }

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }
    }

    let null_value = Value::Null;
    let result = null_value.deserialize_map(MockVisitor { result: None });

    assert!(result.is_err());
}

