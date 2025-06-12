// Answer 0

#[test]
fn test_serialize_seq_end_with_empty_vec() {
    struct DummySerialize;
    
    impl Serialize for DummySerialize {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
        where S: serde::Serializer {
            // This is a placeholder implementation
            serializer.serialize_unit()
        }
    }

    let serialize_vec = SerializeVec { vec: vec![] };
    let result = serialize_vec.end();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Array(vec![]));
}

#[test]
fn test_serialize_seq_end_with_single_element() {
    struct DummySerialize(bool);

    impl Serialize for DummySerialize {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
        where S: serde::Serializer {
            serializer.serialize_bool(self.0)
        }
    }

    let mut serialize_vec = SerializeVec { vec: vec![] };
    let _ = serialize_vec.serialize_element(&DummySerialize(true)).unwrap();

    let result = serialize_vec.end();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Array(vec![Value::Bool(true)]));
}

#[test]
fn test_serialize_seq_end_with_multiple_elements() {
    struct DummySerialize(u32);

    impl Serialize for DummySerialize {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
        where S: serde::Serializer {
            serializer.serialize_u32(self.0)
        }
    }

    let mut serialize_vec = SerializeVec { vec: vec![] };
    let _ = serialize_vec.serialize_element(&DummySerialize(1)).unwrap();
    let _ = serialize_vec.serialize_element(&DummySerialize(2)).unwrap();
    let _ = serialize_vec.serialize_element(&DummySerialize(3)).unwrap();

    let result = serialize_vec.end();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Array(vec![Value::Number(Number::from(1)), Value::Number(Number::from(2)), Value::Number(Number::from(3))]));
}

