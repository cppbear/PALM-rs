// Answer 0

#[test]
fn test_deserialize_newtype_struct_valid() {
    use serde::Deserializer;
    use serde::de::Visitor;

    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<String>;

        fn visit_newtype_struct<V>(self, value: V) -> Result<Self::Value, Error>
        where
            V: Deserializer<'de>,
        {
            let value_string: String = String::deserialize(value)?;
            Ok(Some(value_string))
        }
    }

    let value = Value::String("test".to_string());
    let visitor = TestVisitor { value: None };
    let result: Result<Option<String>, Error> = value.deserialize_newtype_struct("test_name", visitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("test".to_string()));
}

#[test]
#[should_panic]
fn test_deserialize_newtype_struct_invalid_panic() {
    use serde::Deserializer;
    use serde::de::Visitor;

    struct PanicVisitor;

    impl<'de> Visitor<'de> for PanicVisitor {
        type Value = ();

        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: Deserializer<'de>,
        {
            panic!("visitor panic triggered");
        }
    }

    let value = Value::String("panic_test".to_string());
    let visitor = PanicVisitor;
    let _result: Result<(), Error> = value.deserialize_newtype_struct("panic_name", visitor);
}

#[test]
fn test_deserialize_newtype_struct_raw_value() {
    #[cfg(feature = "raw_value")]
    {
        use serde::Deserializer;
        use serde::de::Visitor;

        struct RawVisitor {
            value: Option<String>,
        }

        impl<'de> Visitor<'de> for RawVisitor {
            type Value = Option<String>;

            fn visit_map<V>(self, _: V) -> Result<Self::Value, Error>
            where
                V: Deserializer<'de>,
            {
                Ok(Some("raw_value_test".to_string()))
            }
        }

        let value = Value::String("valid".to_string());
        let visitor = RawVisitor { value: None };
        let result: Result<Option<String>, Error> = value.deserialize_newtype_struct(crate::raw::TOKEN, visitor);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("raw_value_test".to_string()));
    }
}

