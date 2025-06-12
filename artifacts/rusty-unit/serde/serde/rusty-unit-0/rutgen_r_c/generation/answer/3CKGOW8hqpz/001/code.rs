// Answer 0

fn test_serialize_unit_variant_err() {
    struct TestSerializer {
        called: bool,
    }

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<Self::Ok, Self::Error>;
        type SerializeTuple = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeMap = TestSerializeMap;
        type SerializeStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
        type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Err(Error)
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
        
        // Not used, but required by the trait
        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err(Error) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err(Error) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Err(Error) }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Err(Error) }
    }

    struct TestSerializeMap;

    impl SerializeMap for TestSerializeMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error> where T: ?Sized + Serialize { Err(Error) }
        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error> where T: ?Sized + Serialize { Err(Error) }
        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let serializer = TestSerializer { called: false };
    let result = serializer.serialize_unit_variant("TestEnum", 0, "TestVariant");
    assert!(result.is_err());
}

