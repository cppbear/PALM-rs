// Answer 0

#[test]
fn test_serialize_i128_positive() {
    struct TestSerializer;
    impl serde::Serializer for TestSerializer {
        type Ok = String;
        type Error = Error;
        type SerializeSeq = Impossible<String, Error>;
        type SerializeTuple = Impossible<String, Error>;
        type SerializeTupleStruct = Impossible<String, Error>;
        type SerializeTupleVariant = Impossible<String, Error>;
        type SerializeMap = Impossible<String, Error>;
        type SerializeStruct = Impossible<String, Error>;
        type SerializeStructVariant = Impossible<String, Error>;

        fn serialize_i128(self, value: i128) -> Result<String> {
            Ok(itoa::Buffer::new().format(value).to_owned())
        }

        // Other methods are left unimplemented
        fn serialize_bool(self, _: bool) -> Result<String> { unimplemented!() }
        fn serialize_i8(self, _: i8) -> Result<String> { unimplemented!() }
        fn serialize_i16(self, _: i16) -> Result<String> { unimplemented!() }
        fn serialize_i32(self, _: i32) -> Result<String> { unimplemented!() }
        fn serialize_i64(self, _: i64) -> Result<String> { unimplemented!() }
        fn serialize_u8(self, _: u8) -> Result<String> { unimplemented!() }
        fn serialize_u16(self, _: u16) -> Result<String> { unimplemented!() }
        fn serialize_u32(self, _: u32) -> Result<String> { unimplemented!() }
        fn serialize_u64(self, _: u64) -> Result<String> { unimplemented!() }
        fn serialize_f32(self, _: f32) -> Result<String> { unimplemented!() }
        fn serialize_f64(self, _: f64) -> Result<String> { unimplemented!() }
        fn serialize_char(self, _: char) -> Result<String> { unimplemented!() }
        fn serialize_str(self, _: &str) -> Result<String> { unimplemented!() }
        fn serialize_bytes(self, _: &[u8]) -> Result<String> { unimplemented!() }
        fn serialize_unit(self) -> Result<String> { unimplemented!() }
        fn serialize_unit_struct(self, _: &'static str) -> Result<String> { unimplemented!() }
        fn serialize_none(self) -> Result<String> { unimplemented!() }
        fn serialize_some<T>(self, _: &T) -> Result<String> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<String> { unimplemented!() }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<String> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<String> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq> { unimplemented!() }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> { unimplemented!() }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct> { unimplemented!() }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant> { unimplemented!() }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap> { unimplemented!() }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct> { unimplemented!() }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant> { unimplemented!() }
        fn collect_str<T>(self, _: &T) -> Result<String> where T: ?Sized + Display { unimplemented!() }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i128(123456789012345678901234567890i128).unwrap();
    assert_eq!(result, "123456789012345678901234567890");
}

#[test]
fn test_serialize_i128_negative() {
    struct TestSerializer;
    impl serde::Serializer for TestSerializer {
        type Ok = String;
        type Error = Error;
        type SerializeSeq = Impossible<String, Error>;
        type SerializeTuple = Impossible<String, Error>;
        type SerializeTupleStruct = Impossible<String, Error>;
        type SerializeTupleVariant = Impossible<String, Error>;
        type SerializeMap = Impossible<String, Error>;
        type SerializeStruct = Impossible<String, Error>;
        type SerializeStructVariant = Impossible<String, Error>;
        
        fn serialize_i128(self, value: i128) -> Result<String> {
            Ok(itoa::Buffer::new().format(value).to_owned())
        }

        // Other methods are left unimplemented
        fn serialize_bool(self, _: bool) -> Result<String> { unimplemented!() }
        fn serialize_i8(self, _: i8) -> Result<String> { unimplemented!() }
        fn serialize_i16(self, _: i16) -> Result<String> { unimplemented!() }
        fn serialize_i32(self, _: i32) -> Result<String> { unimplemented!() }
        fn serialize_i64(self, _: i64) -> Result<String> { unimplemented!() }
        fn serialize_u8(self, _: u8) -> Result<String> { unimplemented!() }
        fn serialize_u16(self, _: u16) -> Result<String> { unimplemented!() }
        fn serialize_u32(self, _: u32) -> Result<String> { unimplemented!() }
        fn serialize_u64(self, _: u64) -> Result<String> { unimplemented!() }
        fn serialize_f32(self, _: f32) -> Result<String> { unimplemented!() }
        fn serialize_f64(self, _: f64) -> Result<String> { unimplemented!() }
        fn serialize_char(self, _: char) -> Result<String> { unimplemented!() }
        fn serialize_str(self, _: &str) -> Result<String> { unimplemented!() }
        fn serialize_bytes(self, _: &[u8]) -> Result<String> { unimplemented!() }
        fn serialize_unit(self) -> Result<String> { unimplemented!() }
        fn serialize_unit_struct(self, _: &'static str) -> Result<String> { unimplemented!() }
        fn serialize_none(self) -> Result<String> { unimplemented!() }
        fn serialize_some<T>(self, _: &T) -> Result<String> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<String> { unimplemented!() }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<String> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<String> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq> { unimplemented!() }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> { unimplemented!() }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct> { unimplemented!() }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant> { unimplemented!() }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap> { unimplemented!() }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct> { unimplemented!() }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant> { unimplemented!() }
        fn collect_str<T>(self, _: &T) -> Result<String> where T: ?Sized + Display { unimplemented!() }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i128(-123456789012345678901234567890i128).unwrap();
    assert_eq!(result, "-123456789012345678901234567890");
}

