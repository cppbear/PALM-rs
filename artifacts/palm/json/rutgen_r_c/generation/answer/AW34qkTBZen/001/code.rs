// Answer 0

#[test]
fn test_serialize_bool_true() {
    struct TestSerializer;

    impl serde::Serializer for TestSerializer {
        type Ok = String;
        type Error = Error;
        type SerializeSeq = Impossible<String, Error>;
        type SerializeTuple = Impossible<String, Error>;
        type SerializeTupleStruct = Impossible<String, Error>;
        type SerializeTupleVariant = Impossible<String, Error>;
        type SerializeMap = Impossible<String, Error>;
        type SerializeStruct = Impossible<String, Error>;
        type SerializeStructVariant = Impossible<String, Error>;

        fn serialize_bool(self, value: bool) -> Result<String> {
            Ok(if value { "true" } else { "false" }.to_owned())
        }

        // Other required methods would be defined here as no-op or not implemented.
    }

    let serializer = TestSerializer;

    let result = serializer.serialize_bool(true);
    assert_eq!(result, Ok("true".to_owned()));
}

#[test]
fn test_serialize_bool_false() {
    struct TestSerializer;

    impl serde::Serializer for TestSerializer {
        type Ok = String;
        type Error = Error;
        type SerializeSeq = Impossible<String, Error>;
        type SerializeTuple = Impossible<String, Error>;
        type SerializeTupleStruct = Impossible<String, Error>;
        type SerializeTupleVariant = Impossible<String, Error>;
        type SerializeMap = Impossible<String, Error>;
        type SerializeStruct = Impossible<String, Error>;
        type SerializeStructVariant = Impossible<String, Error>;

        fn serialize_bool(self, value: bool) -> Result<String> {
            Ok(if value { "true" } else { "false" }.to_owned())
        }

        // Other required methods would be defined here as no-op or not implemented.
    }

    let serializer = TestSerializer;

    let result = serializer.serialize_bool(false);
    assert_eq!(result, Ok("false".to_owned()));
}

