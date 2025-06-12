// Answer 0

#[test]
fn test_content_serializer_new() {
    use std::marker::PhantomData;

    struct TestError;
    impl serde::ser::Error for TestError {
        fn custom<T>(_: T) -> Self {
            TestError
        }
    }

    struct TestSerializer {
        error: PhantomData<TestError>,
    }

    impl serde::ser::Serializer for TestSerializer {
        type Ok = ();
        type Error = TestError;
        type SerializeSeq = TestSerializer;
        type SerializeTuple = TestSerializer;
        type SerializeTupleStruct = TestSerializer;
        type SerializeTupleVariant = TestSerializer;
        type SerializeMap = TestSerializer;
        type SerializeStruct = TestSerializer;
        type SerializeStructVariant = TestSerializer;

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + serde::ser::Serialize { Ok(()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Ok(()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + serde::ser::Serialize { Ok(()) }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + serde::ser::Serialize { Ok(()) }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { Ok(self) }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { Ok(self) }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { Ok(self) }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { Ok(self) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { Ok(self) }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { Ok(self) }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { Ok(self) }
        fn is_human_readable(&self) -> bool { false }
    }

    let serializer = TestSerializer {
        error: PhantomData,
    };

    let content_serializer = ContentSerializer::new();
    assert!(content_serializer.error == PhantomData::<TestError>);
}

