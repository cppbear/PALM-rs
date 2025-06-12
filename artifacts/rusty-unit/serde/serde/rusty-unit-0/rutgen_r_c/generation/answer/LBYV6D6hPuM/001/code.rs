// Answer 0

#[test]
fn test_serialize_unit_delegate_error() {
    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<(), Error>;
        type SerializeTuple = Impossible<(), Error>;
        type SerializeTupleStruct = Impossible<(), Error>;
        type SerializeMap = MockSerializeMap;
        type SerializeStruct = Impossible<(), Error>;
        type SerializeTupleVariant = Impossible<(), Error>;
        type SerializeStructVariant = Impossible<(), Error>;

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
            unimplemented!()
        }

        fn serialize_map(
            self,
            _: Option<usize>,
        ) -> Result<Self::SerializeMap, Self::Error> {
            Err(Error) // triggers the error condition
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            let mut map = tri!(self.serialize_map(Some(1)));
            tri!(map.serialize_entry("tag", "variant_name"));
            map.end()
        }

        // Other methods are unimplemented for brevity
        // ...
    }

    struct MockSerializeMap;

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            unimplemented!()
        }

        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            unimplemented!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            unimplemented!()
        }
    }

    let serializer = MockSerializer;
    let result = serializer.serialize_unit();
    assert!(result.is_err());
}

