// Answer 0

#[test]
fn test_serialize_u16_zero() {
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

        fn serialize_u64(self, value: u64) -> Result<Value> {
            Ok(Value::Number(value.into()))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_u16(0);
    assert_eq!(result.unwrap(), Value::Number(0.into()));
}

#[test]
fn test_serialize_u16_max() {
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

        fn serialize_u64(self, value: u64) -> Result<Value> {
            Ok(Value::Number(value.into()))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_u16(u16::MAX);
    assert_eq!(result.unwrap(), Value::Number(u16::MAX as u64.into()));
}

#[test]
fn test_serialize_u16_small() {
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

        fn serialize_u64(self, value: u64) -> Result<Value> {
            Ok(Value::Number(value.into()))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_u16(1);
    assert_eq!(result.unwrap(), Value::Number(1.into()));
}

#[test]
fn test_serialize_u16_invalid() {
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

        fn serialize_u64(self, value: u64) -> Result<Value> {
            Ok(Value::Number(value.into()))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_u16(2);
    assert_eq!(result.unwrap(), Value::Number(2.into()));
}

