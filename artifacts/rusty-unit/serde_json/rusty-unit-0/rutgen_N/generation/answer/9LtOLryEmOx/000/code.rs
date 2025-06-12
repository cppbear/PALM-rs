// Answer 0

#[test]
fn test_struct_variant_with_valid_visitor() {
    struct ValidVisitor {
        value: Option<String>,
    }

    impl<'de> de::Visitor<'de> for ValidVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct with fields")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    let fields = &["field"];
    let value: Result<String, _> = struct_variant("field_value", fields, ValidVisitor { value: None });
    assert!(value.is_ok());
    assert_eq!(value.unwrap(), "field_value");
}

#[test]
#[should_panic]
fn test_struct_variant_with_invalid_visitor() {
    struct InvalidVisitor;

    impl<'de> de::Visitor<'de> for InvalidVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct with fields")
        }

        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Err(E::custom("Invalid visitor can never succeed"))
        }
    }

    let fields = &["field"];
    let _: Result<(), _> = struct_variant("field_value", fields, InvalidVisitor);
}

