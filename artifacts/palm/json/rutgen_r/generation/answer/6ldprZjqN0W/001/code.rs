// Answer 0

#[test]
fn test_serialize_unit_struct() {
    struct MockSerializer;

    impl MockSerializer {
        fn serialize_unit(&self) -> Result<Value> {
            Ok(Value::Unit)
        }

        fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
            self.serialize_unit()
        }
    }

    let serializer = MockSerializer;
    let result = serializer.serialize_unit_struct("TestUnitStruct").unwrap();
    assert_eq!(result, Value::Unit);
}

#[test]
fn test_serialize_unit_struct_empty_name() {
    struct MockSerializer;

    impl MockSerializer {
        fn serialize_unit(&self) -> Result<Value> {
            Ok(Value::Unit)
        }

        fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
            self.serialize_unit()
        }
    }
    
    let serializer = MockSerializer;
    let result = serializer.serialize_unit_struct("").unwrap();
    assert_eq!(result, Value::Unit);
}

