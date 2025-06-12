// Answer 0

#[test]
fn test_serialize_tuple_variant_end() {
    struct TestSerializeTupleVariant {
        name: String,
        vec: Vec<Value>,
    }

    impl SerializeTupleVariant for TestSerializeTupleVariant {
        type Ok = Value;
        type Error = Error;

        fn serialize_field<T>(&mut self, _: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Value> {
            let mut object = Map::new();
            object.insert(self.name, Value::Array(self.vec));
            Ok(Value::Object(object))
        }
    }

    let variant = TestSerializeTupleVariant {
        name: "my_variant".to_string(),
        vec: vec![Value::Bool(true), Value::Number(Number::from(3.14))],
    };

    let result = variant.end().unwrap();
    assert_eq!(
        result,
        Value::Object(Map::new().insert("my_variant".to_string(), Value::Array(vec![
            Value::Bool(true), 
            Value::Number(Number::from(3.14))
        ])))
    );
}

#[test]
fn test_serialize_tuple_variant_empty_vec() {
    struct TestSerializeTupleVariant {
        name: String,
        vec: Vec<Value>,
    }

    impl SerializeTupleVariant for TestSerializeTupleVariant {
        type Ok = Value;
        type Error = Error;

        fn serialize_field<T>(&mut self, _: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Value> {
            let mut object = Map::new();
            object.insert(self.name, Value::Array(self.vec));
            Ok(Value::Object(object))
        }
    }

    let variant = TestSerializeTupleVariant {
        name: "empty_variant".to_string(),
        vec: vec![],
    };

    let result = variant.end().unwrap();
    assert_eq!(
        result,
        Value::Object(Map::new().insert("empty_variant".to_string(), Value::Array(vec![])))
    );
}

#[test]
fn test_serialize_tuple_variant_special_chars() {
    struct TestSerializeTupleVariant {
        name: String,
        vec: Vec<Value>,
    }

    impl SerializeTupleVariant for TestSerializeTupleVariant {
        type Ok = Value;
        type Error = Error;

        fn serialize_field<T>(&mut self, _: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Value> {
            let mut object = Map::new();
            object.insert(self.name, Value::Array(self.vec));
            Ok(Value::Object(object))
        }
    }

    let variant = TestSerializeTupleVariant {
        name: "variant_with_special_&_chars".to_string(),
        vec: vec![Value::String("str_with_special_char".to_string())],
    };

    let result = variant.end().unwrap();
    assert_eq!(
        result,
        Value::Object(Map::new().insert("variant_with_special_&_chars".to_string(), Value::Array(vec![
            Value::String("str_with_special_char".to_string())
        ])))
    );
}

