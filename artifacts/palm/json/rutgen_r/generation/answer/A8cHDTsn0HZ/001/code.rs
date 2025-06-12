// Answer 0

#[test]
fn test_serialize_unit_returns_null() {
    struct UnitSerializer;

    impl UnitSerializer {
        fn serialize_unit(self) -> Result<Value> {
            Ok(Value::Null)
        }
    }

    let serializer = UnitSerializer;
    let result = serializer.serialize_unit();
    assert_eq!(result, Ok(Value::Null));
}

