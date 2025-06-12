// Answer 0

#[test]
fn test_deserialize_newtype_struct() {
    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a newtype struct")
        }

        fn visit_newtype_struct<E>(self, _: E) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(self.value.unwrap())
        }
    }

    let test_str = "\"test_value\"";
    let visitor = TestVisitor {
        value: Some(test_str.to_string()),
    };

    let result = deserialize_newtype_struct(test_str, visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test_value");
}

#[test]
#[should_panic]
fn test_deserialize_newtype_struct_empty_value() {
    struct PanickingVisitor;

    impl<'de> serde::de::Visitor<'de> for PanickingVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a newtype struct")
        }

        fn visit_newtype_struct<E>(self, _: E) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(E::custom("Visitor is empty"))
        }
    }

    let empty_str = "\"\"";
    let visitor = PanickingVisitor;

    let _ = deserialize_newtype_struct(empty_str, visitor).unwrap();
}

