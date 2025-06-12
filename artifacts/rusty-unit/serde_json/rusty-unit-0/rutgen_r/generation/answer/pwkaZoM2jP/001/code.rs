// Answer 0

#[test]
fn test_serialize_field_map() {
    use serde::Serialize;
    use serde_json::ser::{Serializer, SerializeStruct};
    use serde_json::{Result, Map};

    struct TestStruct {
        field_1: String,
        field_2: i32,
    }

    impl Serialize for TestStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut state = serializer.serialize_struct("TestStruct", 2)?;
            state.serialize_field("field_1", &self.field_1)?;
            state.serialize_field("field_2", &self.field_2)?;
            state.end()
        }
    }

    let mut compound_map = Map::new();
    let test_value = TestStruct {
        field_1: "test".to_string(),
        field_2: 42,
    };

    let result = compound_map.serialize_field("test_key", &test_value);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_field_unreachable_number() {
    #[cfg(feature = "arbitrary_precision")]
    fn unreachable_number() {
        // This is meant to panic if called when the feature is enabled
        let mut number_compound = Compound::Number { /* fields */ };
        let _ = number_compound.serialize_field("key", &42);
    }
}

#[test]
#[should_panic]
fn test_serialize_field_unreachable_raw_value() {
    #[cfg(feature = "raw_value")]
    fn unreachable_raw_value() {
        // This should panic if called when the feature is enabled
        let mut raw_value_compound = Compound::RawValue { /* fields */ };
        let _ = raw_value_compound.serialize_field("key", &"some_value");
    }
}

