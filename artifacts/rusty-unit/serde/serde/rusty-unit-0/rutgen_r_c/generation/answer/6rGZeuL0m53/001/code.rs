// Answer 0

#[test]
fn test_serialize_bool() {
    struct DummySerializer;

    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<(), Error>;
        type SerializeTuple = Impossible<(), Error>;
        type SerializeTupleStruct = Impossible<(), Error>;
        type SerializeTupleVariant = Impossible<(), Error>;
        type SerializeMap = Impossible<(), Error>;
        type SerializeStruct = Impossible<(), Error>;
        type SerializeStructVariant = Impossible<(), Error>;

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
            Err(Error {})
        }

        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
            Err(Error {})
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Err(Error {})
        }

        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(Error {})
        }

        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
            Err(Error {})
        }

        // Other methods would also need to be implemented, but are not necessary for this test
    }

    let serializer = DummySerializer;
    let result = serializer.serialize_bool(true);
    assert!(result.is_err());

    let expected_error = serializer.bad_type(Unsupported::Boolean);
    assert_eq!(result.err().unwrap(), expected_error);
}

