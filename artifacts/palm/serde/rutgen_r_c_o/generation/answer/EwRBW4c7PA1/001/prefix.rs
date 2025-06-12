// Answer 0

#[test]
fn test_serialize_unit_struct_delegate_error() {
    struct DummySerializer;

    impl SerializeMap for DummySerializer {
        type Ok = ();
        type Error = ();

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(())
        }

        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    struct TestSerializer {
        delegate: DummySerializer,
        tag: &'static str,
        variant_name: &'static str,
    }

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = ();
        type SerializeMap = DummySerializer;
        type SerializeStruct = DummySerializer;
        type SerializeSeq = Impossible<(), ()>;
        type SerializeTuple = Impossible<(), ()>;
        type SerializeTupleStruct = Impossible<(), ()>;
        type SerializeTupleVariant = Impossible<(), ()>;
        type SerializeStructVariant = Impossible<(), ()>;

        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(self.delegate)
        }

        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
            let mut map = tri!(self.delegate.serialize_map(Some(1)));
            tri!(map.serialize_entry(self.tag, self.variant_name));
            map.end()
        }
    }

    let serializer = TestSerializer {
        delegate: DummySerializer,
        tag: "tag",
        variant_name: "variant_name",
    };
    
    let _ = serializer.serialize_unit_struct("test_struct");
}

