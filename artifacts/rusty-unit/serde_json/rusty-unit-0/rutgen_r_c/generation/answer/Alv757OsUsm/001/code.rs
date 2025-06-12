// Answer 0

#[test]
fn test_serialize_seq_end_no_elements() {
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
            Ok(())
        }

        fn end(self) -> Result<Value> {
            Ok(Value::Array(self.vec))
        }
    }

    let serializer = TestSerializeVec { vec: Vec::new() };
    let result = serializer.end().unwrap();
    assert_eq!(result, Value::Array(Vec::new()));
}

#[test]
fn test_serialize_seq_end_single_element() {
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
            self.vec.push(to_value(value)?);
            Ok(())
        }

        fn end(self) -> Result<Value> {
            Ok(Value::Array(self.vec))
        }
    }

    let mut serializer = TestSerializeVec { vec: Vec::new() };
    let single_value = Value::Bool(true);
    serializer.serialize_element(&single_value).unwrap();
    let result = serializer.end().unwrap();
    assert_eq!(result, Value::Array(vec![single_value]));
}

#[test]
fn test_serialize_seq_end_multiple_elements() {
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
            self.vec.push(to_value(value)?);
            Ok(())
        }

        fn end(self) -> Result<Value> {
            Ok(Value::Array(self.vec))
        }
    }

    let mut serializer = TestSerializeVec { vec: Vec::new() };
    let values = vec![Value::Bool(true), Value::Number(Number::from(42)), Value::String("test".to_string())];

    for value in &values {
        serializer.serialize_element(value).unwrap();
    }

    let result = serializer.end().unwrap();
    assert_eq!(result, Value::Array(values));
}

