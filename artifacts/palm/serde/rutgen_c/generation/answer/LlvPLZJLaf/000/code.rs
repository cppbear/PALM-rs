// Answer 0

#[test]
fn test_serialize_map_none() {
    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = fmt::Error;
        type SerializeSeq = Impossible<(), fmt::Error>;
        type SerializeTuple = Impossible<(), fmt::Error>;
        type SerializeTupleStruct = Impossible<(), fmt::Error>;
        type SerializeTupleVariant = Impossible<(), fmt::Error>;
        type SerializeMap = Impossible<(), fmt::Error>;
        type SerializeStruct = Impossible<(), fmt::Error>;
        type SerializeStructVariant = Impossible<(), fmt::Error>;

        fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, fmt::Error> {
            Err(fmt::Error)
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_map(None);
    assert!(result.is_err());
}

#[test]
fn test_serialize_map_some() {
    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = fmt::Error;
        type SerializeSeq = Impossible<(), fmt::Error>;
        type SerializeTuple = Impossible<(), fmt::Error>;
        type SerializeTupleStruct = Impossible<(), fmt::Error>;
        type SerializeTupleVariant = Impossible<(), fmt::Error>;
        type SerializeMap = Impossible<(), fmt::Error>;
        type SerializeStruct = Impossible<(), fmt::Error>;
        type SerializeStructVariant = Impossible<(), fmt::Error>;

        fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, fmt::Error> {
            Err(fmt::Error)
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_map(Some(5));
    assert!(result.is_err());
}

