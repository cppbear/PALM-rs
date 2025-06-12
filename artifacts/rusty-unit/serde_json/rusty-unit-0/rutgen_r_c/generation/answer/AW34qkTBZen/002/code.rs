// Answer 0

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

        // Unimplemented methods would go here
        // ...
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_bool(false).unwrap();
    assert_eq!(result, "false");
}

