// Answer 0

#[test]
fn test_serialize_vec_end() {
    struct DummySerialize {
        value: i32,
    }

    impl Serialize for DummySerialize {
        fn serialize<S>(&self, _serializer: S) -> result::Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            Ok(())
        }
    }

    let mut serialize_vec = SerializeVec { vec: Vec::new() };
    
    let element = DummySerialize { value: 42 };
    let _ = serialize_vec.serialize_element(&element).unwrap();
    
    let result = serialize_vec.end().unwrap();
    
    if let Value::Array(arr) = result {
        assert_eq!(arr.len(), 1);
    } else {
        panic!("Expected Value::Array, got a different variant");
    }
}

#[test]
fn test_serialize_vec_end_empty() {
    let serialize_vec = SerializeVec { vec: Vec::new() };
    
    let result = serialize_vec.end().unwrap();
    
    if let Value::Array(arr) = result {
        assert!(arr.is_empty());
    } else {
        panic!("Expected Value::Array, got a different variant");
    }
}

