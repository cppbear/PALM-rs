// Answer 0

#[test]
fn test_serialize_field_map() {
    use serde::ser::{SerializeMap, Serializer};
    use serde_json::Value;
    use serde_json::map::Map;

    struct TestValue;

    impl Serialize for TestValue {
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str("test")
        }
    }

    let mut map = SerializeMap::Map { 
        map: Map::new(), 
        next_key: None 
    };

    let result = map.serialize_field("test_key", &TestValue);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_field_number() {
    #[cfg(feature = "arbitrary_precision")]
    {
        use serde::ser::{SerializeMap, Serializer};
        use serde_json::Value;
        use serde_json::map::Map;

        struct TestNumber;

        impl Serialize for TestNumber {
            fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_f64(10.0)
            }
        }

        let mut out_value = None;
        let mut number = SerializeMap::Number { out_value: Some(out_value) };

        let result = number.serialize_field(crate::number::TOKEN, &TestNumber);
        assert!(result.is_ok());
        assert!(number.out_value.is_some());
    }
}

#[test]
fn test_serialize_field_invalid_key() {
    #[cfg(feature = "arbitrary_precision")]
    {
        use serde::ser::{SerializeMap, Serializer};
        use serde_json::Value;
        use serde_json::map::Map;

        struct TestNumber;

        impl Serialize for TestNumber {
            fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_f64(10.0)
            }
        }

        let mut out_value = None;
        let mut number = SerializeMap::Number { out_value: Some(out_value) };

        let result = number.serialize_field("invalid_token", &TestNumber);
        assert!(result.is_err());
    }
}

