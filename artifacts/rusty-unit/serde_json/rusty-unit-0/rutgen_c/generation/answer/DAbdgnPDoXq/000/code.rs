// Answer 0

#[test]
fn test_serialize_char() {
    struct MySerializer;

    impl serde::Serializer for MySerializer {
        type Ok = String;
        type Error = Error;
        type SerializeSeq = Impossible<String, Error>;
        type SerializeTuple = Impossible<String, Error>;
        type SerializeTupleStruct = Impossible<String, Error>;
        type SerializeTupleVariant = Impossible<String, Error>;
        type SerializeMap = Impossible<String, Error>;
        type SerializeStruct = Impossible<String, Error>;
        type SerializeStructVariant = Impossible<String, Error>;

        fn serialize_char(self, value: char) -> Result<String> {
            Ok({
                let mut s = String::new();
                s.push(value);
                s
            })
        }

        // Other methods are not used in this test context, thus left unimplemented.
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<String> {
            unimplemented!()
        }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<String>
        where
            T: ?Sized + Serialize,
        {
            unimplemented!()
        }
        fn serialize_bool(self, _: bool) -> Result<String> { unimplemented!() }
        fn serialize_i8(self, _: i8) -> Result<String> { unimplemented!() }
        fn serialize_i16(self, _: i16) -> Result<String> { unimplemented!() }
        fn serialize_i32(self, _: i32) -> Result<String> { unimplemented!() }
        fn serialize_i64(self, _: i64) -> Result<String> { unimplemented!() }
        fn serialize_i128(self, _: i128) -> Result<String> { unimplemented!() }
        fn serialize_u8(self, _: u8) -> Result<String> { unimplemented!() }
        fn serialize_u16(self, _: u16) -> Result<String> { unimplemented!() }
        fn serialize_u32(self, _: u32) -> Result<String> { unimplemented!() }
        fn serialize_u64(self, _: u64) -> Result<String> { unimplemented!() }
        fn serialize_u128(self, _: u128) -> Result<String> { unimplemented!() }
        fn serialize_f32(self, _: f32) -> Result<String> { unimplemented!() }
        fn serialize_f64(self, _: f64) -> Result<String> { unimplemented!() }
        fn serialize_str(self, _: &str) -> Result<String> { unimplemented!() }
        fn serialize_bytes(self, _: &[u8]) -> Result<String> { unimplemented!() }
        fn serialize_unit(self) -> Result<String> { unimplemented!() }
        fn serialize_unit_struct(self, _: &'static str) -> Result<String> { unimplemented!() }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<String>
        where
            T: ?Sized + Serialize,
        {
            unimplemented!()
        }
        fn serialize_none(self) -> Result<String> { unimplemented!() }
        fn serialize_some<T>(self, _: &T) -> Result<String>
        where
            T: ?Sized + Serialize,
        {
            unimplemented!()
        }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq> {
            unimplemented!()
        }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> {
            unimplemented!()
        }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct> {
            unimplemented!()
        }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant> {
            unimplemented!()
        }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap> {
            unimplemented!()
        }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct> {
            unimplemented!()
        }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant> {
            unimplemented!()
        }
    }
    
    let serializer = MySerializer;
    let result = serializer.serialize_char('a').unwrap();
    assert_eq!(result, "a");
}

#[test]
fn test_serialize_char_empty() {
    struct MySerializer;

    impl serde::Serializer for MySerializer {
        type Ok = String;
        type Error = Error;
        type SerializeSeq = Impossible<String, Error>;
        type SerializeTuple = Impossible<String, Error>;
        type SerializeTupleStruct = Impossible<String, Error>;
        type SerializeTupleVariant = Impossible<String, Error>;
        type SerializeMap = Impossible<String, Error>;
        type SerializeStruct = Impossible<String, Error>;
        type SerializeStructVariant = Impossible<String, Error>;

        fn serialize_char(self, value: char) -> Result<String> {
            Ok({
                let mut s = String::new();
                s.push(value);
                s
            })
        }

        // Unimplemented methods
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<String> { unimplemented!() }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<String> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_bool(self, _: bool) -> Result<String> { unimplemented!() }
        fn serialize_i8(self, _: i8) -> Result<String> { unimplemented!() }
        fn serialize_i16(self, _: i16) -> Result<String> { unimplemented!() }
        fn serialize_i32(self, _: i32) -> Result<String> { unimplemented!() }
        fn serialize_i64(self, _: i64) -> Result<String> { unimplemented!() }
        fn serialize_i128(self, _: i128) -> Result<String> { unimplemented!() }
        fn serialize_u8(self, _: u8) -> Result<String> { unimplemented!() }
        fn serialize_u16(self, _: u16) -> Result<String> { unimplemented!() }
        fn serialize_u32(self, _: u32) -> Result<String> { unimplemented!() }
        fn serialize_u64(self, _: u64) -> Result<String> { unimplemented!() }
        fn serialize_u128(self, _: u128) -> Result<String> { unimplemented!() }
        fn serialize_f32(self, _: f32) -> Result<String> { unimplemented!() }
        fn serialize_f64(self, _: f64) -> Result<String> { unimplemented!() }
        fn serialize_str(self, _: &str) -> Result<String> { unimplemented!() }
        fn serialize_bytes(self, _: &[u8]) -> Result<String> { unimplemented!() }
        fn serialize_unit(self) -> Result<String> { unimplemented!() }
        fn serialize_unit_struct(self, _: &'static str) -> Result<String> { unimplemented!() }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<String>
        where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_none(self) -> Result<String> { unimplemented!() }
        fn serialize_some<T>(self, _: &T) -> Result<String>
        where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq> { unimplemented!() }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> { unimplemented!() }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct> { unimplemented!() }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant> { unimplemented!() }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap> { unimplemented!() }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct> { unimplemented!() }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant> { unimplemented!() }
    }
    
    let serializer = MySerializer;
    let result = serializer.serialize_char(' ').unwrap();
    assert_eq!(result, " ");
}

