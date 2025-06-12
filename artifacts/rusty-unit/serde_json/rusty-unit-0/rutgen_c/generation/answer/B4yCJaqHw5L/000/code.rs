// Answer 0

#[test]
fn test_deserialize_newtype_struct() {
    use serde::de::{Deserializer, Visitor};
    use std::marker::PhantomData;

    struct TestVisitor {
        value: Option<Value>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn visit_newtype_struct<T>(self, value: T) -> Result<Self::Value, Error>
        where
            T: Deserialize<'de>,
        {
            Ok(Value::String(value.to_string()))
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("newtype struct")
        }
    }

    let value = Value::String("test".to_owned());
    let visitor = TestVisitor { value: None };
    
    assert_eq!(value.deserialize_newtype_struct("test_struct", visitor).unwrap(), Value::String("test".to_owned()));
}

#[test]
fn test_deserialize_newtype_struct_with_raw_value() {
    use serde::de::{Visitor};

    #[cfg(feature = "raw_value")]
    struct TestVisitor {
        value: Option<Value>,
    }

    #[cfg(feature = "raw_value")]
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn visit_newtype_struct<T>(self, value: T) -> Result<Self::Value, Error>
        where
            T: Deserialize<'de>,
        {
            Ok(Value::String(value.to_string()))
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("newtype struct")
        }
    }

    let value = Value::String("test".to_owned());
    
    #[cfg(feature = "raw_value")]
    {
        let visitor = TestVisitor { value: None };
        assert_eq!(value.deserialize_newtype_struct(crate::raw::TOKEN, visitor).unwrap(), Value::String("test".to_owned()));
    }
}

