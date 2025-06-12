// Answer 0

#[test]
fn test_serialize_tuple_variant_valid() {
    struct TestSerializer;
    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = ();
        type SerializeSeq = Impossible<Self::Ok, Self::Error>;
        type SerializeTuple = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
        type SerializeMap = TestMap;
        type SerializeStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(TestMap)
        }

        fn is_human_readable(&self) -> bool {
            true
        }
    }

    struct TestMap;

    impl SerializeMap for TestMap {
        type Ok = ();
        type Error = ();

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
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize,
        {
            self.serialize_key(key)?;
            self.serialize_value(value)
        }
    }

    let serializer = TaggedSerializer {
        type_ident: "type",
        variant_ident: "variant",
        tag: "tag",
        variant_name: "name",
        delegate: TestSerializer,
    };

    let _ = serializer.serialize_tuple_variant("test", 0, "variant", 0);
}

#[test]
fn test_serialize_tuple_variant_err_case() {
    struct ErrorSerializer;
    impl Serializer for ErrorSerializer {
        type Ok = ();
        type Error = ();
        type SerializeSeq = Impossible<Self::Ok, Self::Error>;
        type SerializeTuple = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
        type SerializeMap = ErrMap;
        type SerializeStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(ErrMap)
        }

        fn is_human_readable(&self) -> bool {
            true
        }
    }

    struct ErrMap;

    impl SerializeMap for ErrMap {
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
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize,
        {
            self.serialize_key(key)?;
            self.serialize_value(value)
        }
    }

    let serializer = TaggedSerializer {
        type_ident: "type",
        variant_ident: "variant",
        tag: "tag",
        variant_name: "name",
        delegate: ErrorSerializer,
    };

    let result = serializer.serialize_tuple_variant("test", 0, "variant", 0);
    let _ = result; // should be an Err
}

