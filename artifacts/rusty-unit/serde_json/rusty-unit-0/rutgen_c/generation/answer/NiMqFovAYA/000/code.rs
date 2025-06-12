// Answer 0

#[test]
fn test_serialize_field_bool() {
    struct TestSerialize;
    
    impl Serialize for TestSerialize {
        fn serialize<Serializer>(&self, serializer: Serializer) -> result::Result<Serializer::Ok, Serializer::Error>
        where
            Serializer: serde::ser::Serializer,
        {
            serializer.serialize_bool(true)
        }
    }

    let mut variant = SerializeTupleVariant {
        name: String::from("test"),
        vec: Vec::new(),
    };

    let result = variant.serialize_field(&TestSerialize);
    assert!(result.is_ok());
    assert_eq!(variant.vec.len(), 1);
    if let Value::Bool(value) = &variant.vec[0] {
        assert_eq!(*value, true);
    } else {
        panic!("Expected Value::Bool");
    }
}

#[test]
fn test_serialize_field_number() {
    struct TestSerialize {
        number: f64,
    }
    
    impl Serialize for TestSerialize {
        fn serialize<Serializer>(&self, serializer: Serializer) -> result::Result<Serializer::Ok, Serializer::Error>
        where
            Serializer: serde::ser::Serializer,
        {
            serializer.serialize_f64(self.number)
        }
    }

    let mut variant = SerializeTupleVariant {
        name: String::from("test"),
        vec: Vec::new(),
    };

    let test_value = TestSerialize { number: 42.0 };
    let result = variant.serialize_field(&test_value);
    assert!(result.is_ok());
    assert_eq!(variant.vec.len(), 1);
    if let Value::Number(_) = &variant.vec[0] {
        // Assume appropriate number serialization to Value is handled by to_value
    } else {
        panic!("Expected Value::Number");
    }
}

#[test]
fn test_serialize_field_string() {
    struct TestSerialize {
        text: &'static str,
    }
    
    impl Serialize for TestSerialize {
        fn serialize<Serializer>(&self, serializer: Serializer) -> result::Result<Serializer::Ok, Serializer::Error>
        where
            Serializer: serde::ser::Serializer,
        {
            serializer.serialize_str(self.text)
        }
    }

    let mut variant = SerializeTupleVariant {
        name: String::from("test"),
        vec: Vec::new(),
    };

    let test_value = TestSerialize { text: "hello" };
    let result = variant.serialize_field(&test_value);
    assert!(result.is_ok());
    assert_eq!(variant.vec.len(), 1);
    if let Value::String(ref value) = &variant.vec[0] {
        assert_eq!(value.as_str(), "hello");
    } else {
        panic!("Expected Value::String");
    }
}

