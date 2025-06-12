// Answer 0

#[test]
fn test_serialize_newtype_struct_with_string() {
    struct TestSerializer<'a>(&'a mut fmt::Formatter<'a>);

    impl<'a> Serializer for TestSerializer<'a> {
        type Ok = ();
        type Error = fmt::Error;
        type SerializeSeq = Impossible<(), fmt::Error>;
        type SerializeTuple = Impossible<(), fmt::Error>;
        type SerializeTupleStruct = Impossible<(), fmt::Error>;
        type SerializeTupleVariant = Impossible<(), fmt::Error>;
        type SerializeMap = Impossible<(), fmt::Error>;
        type SerializeStruct = Impossible<(), fmt::Error>;
        type SerializeStructVariant = Impossible<(), fmt::Error>;

        fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> fmt::Result {
            write!(self.0, "{}", variant)
        }

        fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> fmt::Result
        where
            T: ?Sized + Serialize,
        {
            Serialize::serialize(value, self)
        }

        fn serialize_str(self, value: &str) -> fmt::Result {
            write!(self.0, "{}", value)
        }

        fn serialize_unit(self) -> fmt::Result {
            write!(self.0, "unit")
        }
    }

    let mut output = String::new();
    let mut serializer = TestSerializer(&mut output);
    let result = serializer.serialize_newtype_struct("TestStruct", &"Hello");

    assert!(result.is_ok());
    assert_eq!(output, "Hello");
}

#[test]
fn test_serialize_newtype_struct_with_unit() {
    struct TestSerializer<'a>(&'a mut fmt::Formatter<'a>);

    impl<'a> Serializer for TestSerializer<'a> {
        type Ok = ();
        type Error = fmt::Error;
        type SerializeSeq = Impossible<(), fmt::Error>;
        type SerializeTuple = Impossible<(), fmt::Error>;
        type SerializeTupleStruct = Impossible<(), fmt::Error>;
        type SerializeTupleVariant = Impossible<(), fmt::Error>;
        type SerializeMap = Impossible<(), fmt::Error>;
        type SerializeStruct = Impossible<(), fmt::Error>;
        type SerializeStructVariant = Impossible<(), fmt::Error>;

        fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> fmt::Result
        where
            T: ?Sized + Serialize,
        {
            Serialize::serialize(value, self)
        }

        fn serialize_unit(self) -> fmt::Result {
            write!(self.0, "unit")
        }
    }

    let mut output = String::new();
    let mut serializer = TestSerializer(&mut output);
    let result = serializer.serialize_newtype_struct("TestUnit", &());

    assert!(result.is_ok());
    assert_eq!(output, "unit");
}

