// Answer 0

#[test]
fn test_serialize_vec_end_empty() {
    struct TestSerializeVec {
        vec: Vec<Value>,
    }

    impl serde::ser::SerializeSeq for TestSerializeVec {
        type Ok = Value;
        type Error = Error;

        fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            // Intentionally left empty for this test.
            Ok(())
        }
        
        fn end(self) -> Result<Value> {
            self.vec = Vec::new();
            Ok(Value::Array(self.vec))
        }
    }

    let serializer = TestSerializeVec { vec: Vec::new() };
    let result = serializer.end().unwrap();
    assert_eq!(result, Value::Array(Vec::new()));
}

#[test]
fn test_serialize_vec_end_non_empty() {
    struct TestSerializeVec {
        vec: Vec<Value>,
    }

    impl serde::ser::SerializeSeq for TestSerializeVec {
        type Ok = Value;
        type Error = Error;

        fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            // Simulate adding a Value to the vector.
            self.vec.push(Value::String("test".to_string()));
            Ok(())
        }

        fn end(self) -> Result<Value> {
            Ok(Value::Array(self.vec))
        }
    }

    let mut serializer = TestSerializeVec { vec: Vec::new() };
    let _ = serializer.serialize_element(&"test_value").unwrap(); // Add an element.
    let result = serializer.end().unwrap();
    assert_eq!(result, Value::Array(vec![Value::String("test".to_string())]));
}

