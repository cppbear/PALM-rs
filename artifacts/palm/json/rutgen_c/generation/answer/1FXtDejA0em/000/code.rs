// Answer 0

#[test]
fn test_serialize_i8() {
    struct TestSerializer;

    impl serde::Serializer for TestSerializer {
        type Ok = Value;
        type Error = Error;
        type SerializeSeq = SerializeVec;
        type SerializeTuple = SerializeVec;
        type SerializeTupleStruct = SerializeVec;
        type SerializeTupleVariant = SerializeTupleVariant;
        type SerializeMap = SerializeMap;
        type SerializeStruct = SerializeMap;
        type SerializeStructVariant = SerializeStructVariant;

        fn serialize_i64(self, value: i64) -> Result<Value> {
            Ok(Value::Number(value.into()))
        }

        #[inline]
        fn serialize_bool(self, _value: bool) -> Result<Value> { unimplemented!() }
        // Other methods can be unimplemented as they are not used.
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i8(8).unwrap();
    if let Value::Number(num) = result {
        assert_eq!(num, 8.into());
    } else {
        panic!("Expected a Value::Number");
    }
}

#[test]
fn test_serialize_i8_negative() {
    struct TestSerializer;

    impl serde::Serializer for TestSerializer {
        type Ok = Value;
        type Error = Error;
        type SerializeSeq = SerializeVec;
        type SerializeTuple = SerializeVec;
        type SerializeTupleStruct = SerializeVec;
        type SerializeTupleVariant = SerializeTupleVariant;
        type SerializeMap = SerializeMap;
        type SerializeStruct = SerializeMap;
        type SerializeStructVariant = SerializeStructVariant;

        fn serialize_i64(self, value: i64) -> Result<Value> {
            Ok(Value::Number(value.into()))
        }

        #[inline]
        fn serialize_bool(self, _value: bool) -> Result<Value> { unimplemented!() }
        // Other methods can be unimplemented as they are not used.
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i8(-10).unwrap();
    if let Value::Number(num) = result {
        assert_eq!(num, -10.into());
    } else {
        panic!("Expected a Value::Number");
    }
}

