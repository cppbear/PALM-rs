// Answer 0

#[test]
fn test_serialize_field_insert_string_value() {
    struct TestStruct {
        field: String,
    }
    impl Serialize for TestStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            serializer.serialize_str(&self.field)
        }
    }

    let mut variant = SerializeStructVariant {
        name: String::from("test"),
        map: Map::new(),
    };
    let result = variant.serialize_field("key", &TestStruct { field: String::from("value") });
    assert!(result.is_ok());
    assert!(variant.map.contains_key("key"));
}

#[test]
fn test_serialize_field_insert_bool_value() {
    struct BoolStruct {
        is_true: bool,
    }
    impl Serialize for BoolStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            serializer.serialize_bool(self.is_true)
        }
    }

    let mut variant = SerializeStructVariant {
        name: String::from("test"),
        map: Map::new(),
    };
    let result = variant.serialize_field("key", &BoolStruct { is_true: true });
    assert!(result.is_ok());
    assert!(variant.map.contains_key("key"));
}

#[test]
fn test_serialize_field_insert_number_value() {
    struct NumberStruct {
        number: f64,
    }
    impl Serialize for NumberStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            serializer.serialize_f64(self.number)
        }
    }

    let mut variant = SerializeStructVariant {
        name: String::from("test"),
        map: Map::new(),
    };
    let result = variant.serialize_field("key", &NumberStruct { number: 3.14 });
    assert!(result.is_ok());
    assert!(variant.map.contains_key("key"));
}

#[test]
fn test_serialize_field_insert_null_value() {
    struct NullStruct;
    impl Serialize for NullStruct {
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            // Serialization results in a null value
            Ok(())
        }
    }

    let mut variant = SerializeStructVariant {
        name: String::from("test"),
        map: Map::new(),
    };
    let result = variant.serialize_field("key", &NullStruct);
    assert!(result.is_ok());
    assert!(variant.map.contains_key("key"));
}

