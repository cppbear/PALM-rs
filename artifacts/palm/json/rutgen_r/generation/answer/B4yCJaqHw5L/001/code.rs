// Answer 0

#[test]
fn test_deserialize_newtype_struct() {
    use serde::de::{self, Visitor};
    use serde_json::{Deserializer, Error};

    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a newtype struct")
        }

        fn visit_newtype_struct<E>(self, value: E) -> Result<Self::Value, E::Error>
        where
            E: de::Deserialize<'de>,
        {
            Ok(value.to_string())
        }
    }

    let json = r#""test_value""#;
    let deserializer = Deserializer::from_str(json);

    let visitor = TestVisitor { value: None };
    
    match deserializer.deserialize_newtype_struct("test_struct", visitor) {
        Ok(result) => assert_eq!(result, "test_value"),
        Err(_) => panic!("Deserialization failed"),
    }
}

#[test]
fn test_deserialize_newtype_struct_raw_value() {
    use serde::de::{self, Visitor};
    use serde_json::{Deserializer, Error};

    #[cfg(feature = "raw_value")]
    {
        struct TestVisitor {
            value: Option<String>,
        }

        impl<'de> Visitor<'de> for TestVisitor {
            type Value = String;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a raw value")
            }

            fn visit_map<E>(self, _: E) -> Result<Self::Value, E::Error>
            where
                E: de::Deserialize<'de>,
            {
                Ok("raw_value".to_string())
            }
        }

        let json = r#""some_raw_value""#;
        let deserializer = Deserializer::from_str(json);

        let visitor = TestVisitor { value: None };
        
        match deserializer.deserialize_newtype_struct(crate::raw::TOKEN, visitor) {
            Ok(result) => assert_eq!(result, "raw_value"),
            Err(_) => panic!("Deserialization of raw value failed"),
        }
    }
}

