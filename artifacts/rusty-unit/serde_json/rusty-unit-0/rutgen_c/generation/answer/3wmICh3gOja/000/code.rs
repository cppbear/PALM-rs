// Answer 0

#[test]
fn test_serialize_seq_end_with_empty_vec() {
    struct DummySerializer {
        vec: Vec<Value>,
    }

    impl serde::ser::SerializeSeq for DummySerializer {
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

    let serializer = DummySerializer { vec: Vec::new() };
    let result = serializer.end().unwrap();
    assert_eq!(result, Value::Array(vec![]));
}

#[test]
fn test_serialize_seq_end_with_some_elements() {
    struct DummySerializer {
        vec: Vec<Value>,
    }

    impl serde::ser::SerializeSeq for DummySerializer {
        type Ok = Value;
        type Error = Error;
        
        fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            // Simulating serialization of a string Value
            if let Some(string_value) = value.downcast_ref::<String>() {
                self.vec.push(Value::String(string_value.clone()));
            }
            Ok(())
        }
        
        fn end(self) -> Result<Value> {
            Ok(Value::Array(self.vec))
        }
    }

    let mut serializer = DummySerializer { vec: Vec::new() };
    let _ = serializer.serialize_element(&"first_value".to_string());
    let _ = serializer.serialize_element(&"second_value".to_string());
    let result = serializer.end().unwrap();
    assert_eq!(result, Value::Array(vec![
        Value::String("first_value".to_string()),
        Value::String("second_value".to_string()),
    ]));
}

