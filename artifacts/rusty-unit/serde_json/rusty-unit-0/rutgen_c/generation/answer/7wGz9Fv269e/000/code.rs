// Answer 0

#[test]
fn test_serialize_element_with_bool() {
    struct DummySerializeSeq {
        vec: Vec<Value>,
    }

    impl serde::ser::SerializeSeq for DummySerializeSeq {
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

    let mut serialize_vec = SerializeVec { vec: Vec::new() };
    let value = &true;

    let result = serialize_vec.serialize_element(value);
    assert!(result.is_ok());
    if let Ok(Value::Array(arr)) = serialize_vec.serialize_element(value) {
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0], Value::Bool(true));
    }
}

#[test]
fn test_serialize_element_with_null() {
    struct DummySerializeSeq {
        vec: Vec<Value>,
    }

    impl serde::ser::SerializeSeq for DummySerializeSeq {
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

    let mut serialize_vec = SerializeVec { vec: Vec::new() };
    let value: &Option<()> = &None;

    let result = serialize_vec.serialize_element(value);
    assert!(result.is_ok());
    if let Ok(Value::Array(arr)) = serialize_vec.serialize_element(value) {
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0], Value::Null);
    }
}

#[test]
fn test_serialize_element_with_string() {
    struct DummySerializeSeq {
        vec: Vec<Value>,
    }

    impl serde::ser::SerializeSeq for DummySerializeSeq {
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

    let mut serialize_vec = SerializeVec { vec: Vec::new() };
    let value = &"Hello, world!";

    let result = serialize_vec.serialize_element(value);
    assert!(result.is_ok());
    if let Ok(Value::Array(arr)) = serialize_vec.serialize_element(value) {
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0], Value::String("Hello, world!".to_string()));
    }
}

