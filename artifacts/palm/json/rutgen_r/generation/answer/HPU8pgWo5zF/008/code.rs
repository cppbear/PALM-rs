// Answer 0

#[test]
fn test_serialize_bool_true() {
    use serde_json::Value;
    use serde::ser::Serializer;
    use serde::ser::Serializer as SerdeSerializer;

    struct TestSerializer;

    impl SerdeSerializer for TestSerializer {
        type Ok = ();
        type Error = ();
        
        fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
            assert!(v == true);
            Ok(())
        }

        // Implement other required methods as no-op
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_entry(self, _: &str, _: &Value) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_number(self, _: &serde_json::Number) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_array(self, _: usize) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let value_true = Value::Bool(true);
    let result = value_true.serialize(TestSerializer);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_bool_false() {
    use serde_json::Value;
    use serde::ser::Serializer;
    use serde::ser::Serializer as SerdeSerializer;

    struct TestSerializer;

    impl SerdeSerializer for TestSerializer {
        type Ok = ();
        type Error = ();

        fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
            assert!(v == false);
            Ok(())
        }

        // Implement other required methods as no-op
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_entry(self, _: &str, _: &Value) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_number(self, _: &serde_json::Number) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_array(self, _: usize) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let value_false = Value::Bool(false);
    let result = value_false.serialize(TestSerializer);
    assert!(result.is_ok());
}

