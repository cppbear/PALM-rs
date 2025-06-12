// Answer 0

#[test]
fn test_serialize_tuple_variant_end() {
    struct SimpleSerialize;

    impl Serialize for SimpleSerialize {
        // Implement required functionality for Serialize
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str("test")
        }
    }

    let variant = SerializeTupleVariant {
        name: String::from("key"),
        vec: vec![Value::Bool(true), Value::Number(Number::from(42))],
    };

    let result = variant.end().unwrap();

    if let Value::Object(map) = result {
        assert_eq!(map.len(), 1);
        assert!(map.contains_key("key"));
        if let Value::Array(values) = map.get("key").unwrap() {
            assert_eq!(values.len(), 2);
            assert_eq!(*values.get(0).unwrap(), Value::Bool(true));
            assert_eq!(*values.get(1).unwrap(), Value::Number(Number::from(42)));
        } else {
            panic!("Expected Value::Array");
        }
    } else {
        panic!("Expected Value::Object");
    }
}

