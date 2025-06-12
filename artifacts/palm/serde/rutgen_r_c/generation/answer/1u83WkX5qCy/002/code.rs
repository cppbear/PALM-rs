// Answer 0

#[test]
fn test_serialize_newtype_variant_error_on_serialize_map() {
    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<(), Error>;
        type SerializeTuple = Impossible<(), Error>;
        type SerializeTupleStruct = Impossible<(), Error>;
        type SerializeMap = MockSerializeMap;
        type SerializeStruct = Impossible<(), Error>;
        type SerializeStructVariant = Impossible<(), Error>;
        type SerializeTupleVariant = Impossible<(), Error>;

        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(MockSerializeMap { should_fail: true })
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
        
        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }
    }

    struct MockSerializeMap {
        should_fail: bool,
    }

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(Error)
        }

        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(Error)
        }

        fn serialize_entry<K, V>(&mut self, _: &K, _: &V) -> Result<(), Self::Error>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize,
        {
            Err(Error)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: MockSerializer,
    };

    let result = serializer.serialize_newtype_variant("TestType", 0, "inner_variant", &());
    assert!(result.is_err());
}

#[test]
fn test_serialize_newtype_variant_error_on_serialize_entry() {
    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<(), Error>;
        type SerializeTuple = Impossible<(), Error>;
        type SerializeTupleStruct = Impossible<(), Error>;
        type SerializeMap = MockSerializeMap;
        type SerializeStruct = Impossible<(), Error>;
        type SerializeStructVariant = Impossible<(), Error>;
        type SerializeTupleVariant = Impossible<(), Error>;

        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(MockSerializeMap { should_fail: true })
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
        
        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }
    }

    struct MockSerializeMap {
        should_fail: bool,
    }

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(Error)
        }

        fn serialize_entry<K, V>(&mut self, _: &K, _: &V) -> Result<(), Self::Error>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize,
        {
            Err(Error)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: MockSerializer,
    };

    let result = serializer.serialize_newtype_variant("TestType", 0, "inner_variant", &());
    assert!(result.is_err());
}

