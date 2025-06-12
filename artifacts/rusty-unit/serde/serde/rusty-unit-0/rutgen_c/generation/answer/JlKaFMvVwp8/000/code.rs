// Answer 0

#[test]
fn test_serialize_u64() {
    struct MockError;

    struct MockMapSerializer;

    impl SerializeMap for MockMapSerializer {
        type Error = MockError;

        fn serialize_entry<K, V>(&mut self, _: &K, _: &V) -> Result<(), Self::Error> 
        where
            K: Serialize,
            V: Serialize,
        {
            Ok(())
        }
        
        fn serialize_key<K>(&mut self, _: &K) -> Result<(), Self::Error> 
        where
            K: Serialize,
        {
            Ok(())
        }
    }

    impl Serializer for FlatMapSerializer<'_, MockMapSerializer> {
        type Ok = ();
        type Error = MockError;
        type SerializeSeq = Impossible<Self::Ok, Self::Error>;
        type SerializeTuple = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeMap = FlatMapSerializeMap<'_, MockMapSerializer>;
        type SerializeStruct = FlatMapSerializeStruct<'_, MockMapSerializer>;
        type SerializeTupleVariant = FlatMapSerializeTupleVariantAsMapValue<'_, MockMapSerializer>;
        type SerializeStructVariant = FlatMapSerializeStructVariantAsMapValue<'_, MockMapSerializer>;

        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
            Err(Self::bad_type(Unsupported::Integer))
        }

        fn bad_type(_: Unsupported) -> Self::Error {
            MockError
        }
    }

    let mut serializer = FlatMapSerializer(&mut MockMapSerializer);
    let result = serializer.serialize_u64(42);
    assert!(result.is_err());
}

