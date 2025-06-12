// Answer 0

#[test]
fn test_deserialize_enum_valid_variant() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_enum<V>(
            self,
            _deserializer: V,
        ) -> Result<Self::Value, V::Error>
        where
            V: serde::de::EnumAccess<'de>,
        {
            Ok("valid_variant".to_string())
        }
    }

    struct FlattenedData(Vec<&'static str>);

    impl FlattenedData {
        fn new(data: Vec<&'static str>) -> Self {
            FlattenedData(data)
        }

        fn deserialize_enum<V>(
            self,
            name: &'static str,
            variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            for entry in self.0 {
                if variants.contains(&entry) {
                    return visitor.visit_enum(serde::de::EnumDeserializer::new(entry, None));
                }
            }
            Err(serde::de::Error::custom(format_args!(
                "no variant of enum {} found in flattened data",
                name
            )))
        }
    }

    let flattened_data = FlattenedData::new(vec!["valid_variant", "other_variant"]);
    let result = flattened_data.deserialize_enum("TestEnum", &["valid_variant", "other_variant"], TestVisitor);
    
    assert_eq!(result.unwrap(), "valid_variant".to_string());
}

#[test]
fn test_deserialize_enum_invalid_variant() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_enum<V>(
            self,
            _deserializer: V,
        ) -> Result<Self::Value, V::Error>
        where
            V: serde::de::EnumAccess<'de>,
        {
            Ok(())
        }
    }

    struct FlattenedData(Vec<&'static str>);

    impl FlattenedData {
        fn new(data: Vec<&'static str>) -> Self {
            FlattenedData(data)
        }

        fn deserialize_enum<V>(
            self,
            name: &'static str,
            variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            for entry in self.0 {
                if variants.contains(&entry) {
                    return visitor.visit_enum(serde::de::EnumDeserializer::new(entry, None));
                }
            }
            Err(serde::de::Error::custom(format_args!(
                "no variant of enum {} found in flattened data",
                name
            )))
        }
    }

    let flattened_data = FlattenedData::new(vec!["invalid_variant"]);
    let result = flattened_data.deserialize_enum("TestEnum", &["valid_variant", "other_variant"], TestVisitor);
    
    assert!(result.is_err());
}

