// Answer 0

#[test]
fn test_serialize_field_map_entry() {
    use serde::Serialize;
    use serde_json::ser::{Compound, SerializeMap};

    struct TestMap {
        inner: std::collections::HashMap<String, String>,
    }

    impl Serialize for TestMap {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut map = serializer.serialize_map(Some(self.inner.len()))?;
            for (key, value) in &self.inner {
                map.serialize_entry(key, value)?;
            }
            map.end()
        }
    }

    let mut map_data = TestMap { inner: std::collections::HashMap::new() };
    map_data.inner.insert("key1".to_string(), "value1".to_string());

    let mut compound = Compound::Map {
        ser: std::cell::RefCell::new(Vec::new()),
        state: std::cell::RefCell::new(()),
    };

    let result = compound.serialize_field("key1", &"value1");
    assert!(result.is_ok());
}

#[test]
fn test_serialize_field_number_case() {
    use serde::Serialize;
    use serde_json::ser::{Compound, NumberStrEmitter};

    struct TestNumber {
        value: i32,
    }

    impl Serialize for TestNumber {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_i32(self.value)
        }
    }

    let value = TestNumber { value: 10 };

    let mut compound = Compound::Number {
        ser: std::cell::RefCell::new(()),
        state: std::cell::RefCell::new(()),
    };

    #[cfg(feature = "arbitrary_precision")]
    {
        let result = compound.serialize_field(crate::number::TOKEN, &value);
        assert!(result.is_ok());
    }

    #[cfg(not(feature = "arbitrary_precision"))]
    {
        let result = compound.serialize_field(crate::number::TOKEN, &value);
        assert!(result.is_err());
    }
}

#[test]
fn test_serialize_field_raw_value_case() {
    use serde::Serialize;
    use serde_json::ser::{Compound, RawValueStrEmitter};

    struct RawValue {
        value: &'static str,
    }

    impl Serialize for RawValue {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.value)
        }
    }

    let raw_value = RawValue { value: "raw_value_data" };

    let mut compound = Compound::RawValue {
        ser: std::cell::RefCell::new(()),
        state: std::cell::RefCell::new(()),
    };

    #[cfg(feature = "raw_value")]
    {
        let result = compound.serialize_field(crate::raw::TOKEN, &raw_value);
        assert!(result.is_ok());
    }

    #[cfg(not(feature = "raw_value"))]
    {
        let result = compound.serialize_field(crate::raw::TOKEN, &raw_value);
        assert!(result.is_err());
    }
}

