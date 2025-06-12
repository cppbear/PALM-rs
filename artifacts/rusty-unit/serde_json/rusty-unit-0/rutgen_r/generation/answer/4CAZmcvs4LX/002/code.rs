// Answer 0

#[test]
fn test_serialize_element_with_serializable_value() {
    use serde::Serialize;
    use serde_json::Value;
    use serde_json::ser::Serializer;

    struct TestSerializer {
        vec: Vec<Value>,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer { vec: Vec::new() }
        }

        fn serialize_element<T>(&mut self, value: &T) -> Result<(), serde_json::error::Error>
        where
            T: ?Sized + Serialize,
        {
            self.vec.push(serde_json::to_value(value)?);
            Ok(())
        }
    }

    #[derive(Serialize)]
    struct SerializableStruct {
        field1: String,
        field2: i32,
    }

    let mut serializer = TestSerializer::new();
    let value = SerializableStruct {
        field1: "test".to_string(),
        field2: 42,
    };
    
    let result = serializer.serialize_element(&value);
    assert!(result.is_ok());
    assert_eq!(serializer.vec.len(), 1);
}

#[test]
fn test_serialize_element_with_empty_string() {
    use serde::Serialize;
    use serde_json::Value;
    use serde_json::ser::Serializer;

    struct TestSerializer {
        vec: Vec<Value>,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer { vec: Vec::new() }
        }

        fn serialize_element<T>(&mut self, value: &T) -> Result<(), serde_json::error::Error>
        where
            T: ?Sized + Serialize,
        {
            self.vec.push(serde_json::to_value(value)?);
            Ok(())
        }
    }

    #[derive(Serialize)]
    struct EmptyStringStruct {
        field: String,
    }

    let mut serializer = TestSerializer::new();
    let value = EmptyStringStruct {
        field: "".to_string(),
    };
    
    let result = serializer.serialize_element(&value);
    assert!(result.is_ok());
    assert_eq!(serializer.vec.len(), 1);
}

#[test]
fn test_serialize_element_with_numeric_value() {
    use serde::Serialize;
    use serde_json::Value;
    use serde_json::ser::Serializer;

    struct TestSerializer {
        vec: Vec<Value>,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer { vec: Vec::new() }
        }

        fn serialize_element<T>(&mut self, value: &T) -> Result<(), serde_json::error::Error>
        where
            T: ?Sized + Serialize,
        {
            self.vec.push(serde_json::to_value(value)?);
            Ok(())
        }
    }

    #[derive(Serialize)]
    struct NumericStruct {
        number: f64,
    }

    let mut serializer = TestSerializer::new();
    let value = NumericStruct {
        number: 3.14,
    };
    
    let result = serializer.serialize_element(&value);
    assert!(result.is_ok());
    assert_eq!(serializer.vec.len(), 1);
}

