// Answer 0

#[test]
fn test_serialize_map_err() {
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

        fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
            Err(key_must_be_a_string())
        }

        // All other required methods must be implemented, but can return placeholders.
        fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<String> {
            Err(key_must_be_a_string())
        }

        fn serialize_newtype_struct<T>(self, _name: &'static str, _: &T) -> Result<String>
        where T: ?Sized + Serialize {
            Err(key_must_be_a_string())
        }

        fn serialize_bool(self, _value: bool) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_i8(self, _value: i8) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_i16(self, _value: i16) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_i32(self, _value: i32) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_i64(self, _value: i64) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_i128(self, _value: i128) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_u8(self, _value: u8) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_u16(self, _value: u16) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_u32(self, _value: u32) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_u64(self, _value: u64) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_u128(self, _value: u128) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_f32(self, _value: f32) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_f64(self, _value: f64) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_char(self, _value: char) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_str(self, _value: &str) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_bytes(self, _value: &[u8]) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_unit(self) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_unit_struct(self, _name: &'static str) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<String>
        where T: ?Sized + Serialize {
            Err(key_must_be_a_string())
        }
        fn serialize_none(self) -> Result<String> { Err(key_must_be_a_string()) }
        fn serialize_some<T>(self, _value: &T) -> Result<String>
        where T: ?Sized + Serialize {
            Err(key_must_be_a_string())
        }
        fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
            Err(key_must_be_a_string())
        }
        fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
            Err(key_must_be_a_string())
        }
        fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct> {
            Err(key_must_be_a_string())
        }
        fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant> {
            Err(key_must_be_a_string())
        }
        fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
            Err(key_must_be_a_string())
        }
        fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant> {
            Err(key_must_be_a_string())
        }

        fn collect_str<T>(self, _value: &T) -> Result<String>
        where T: ?Sized + Display {
            Err(key_must_be_a_string())
        }
    }

    let serializer = TestSerializer;
    let result: Result<TestSerializer::SerializeMap> = serializer.serialize_map(Some(1));
    assert!(result.is_err());
}

