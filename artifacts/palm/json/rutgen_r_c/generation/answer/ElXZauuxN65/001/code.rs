// Answer 0

#[test]
fn test_serialize_unit_struct_panics() {
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

        fn serialize_unit_struct(self, _name: &'static str) -> Result<String> {
            Err(key_must_be_a_string())
        }

        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
        ) -> Result<String> {
            Ok(String::new())
        }

        fn serialize_bool(self, _value: bool) -> Result<String> { Ok(String::new()) }
        fn serialize_i8(self, _value: i8) -> Result<String> { Ok(String::new()) }
        fn serialize_i16(self, _value: i16) -> Result<String> { Ok(String::new()) }
        fn serialize_i32(self, _value: i32) -> Result<String> { Ok(String::new()) }
        fn serialize_i64(self, _value: i64) -> Result<String> { Ok(String::new()) }
        fn serialize_u8(self, _value: u8) -> Result<String> { Ok(String::new()) }
        fn serialize_u16(self, _value: u16) -> Result<String> { Ok(String::new()) }
        fn serialize_u32(self, _value: u32) -> Result<String> { Ok(String::new()) }
        fn serialize_u64(self, _value: u64) -> Result<String> { Ok(String::new()) }
        fn serialize_f32(self, _value: f32) -> Result<String> { Ok(String::new()) }
        fn serialize_f64(self, _value: f64) -> Result<String> { Ok(String::new()) }
        fn serialize_char(self, _value: char) -> Result<String> { Ok(String::new()) }
        fn serialize_str(self, _value: &str) -> Result<String> { Ok(String::new()) }
        fn serialize_bytes(self, _value: &[u8]) -> Result<String> { Ok(String::new()) }
        fn serialize_some<T>(self, _value: &T) -> Result<String> where T: ?Sized + Serialize { Ok(String::new()) }
        fn serialize_none(self) -> Result<String> { Ok(String::new()) }
        fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> { Ok(Impossible) }
        fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> { Ok(Impossible) }
        fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct> { Ok(Impossible) }
        fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> { Ok(Impossible) }
        fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> { Ok(Impossible) }
        fn collect_str<T>(self, _value: &T) -> Result<String> where T: ?Sized + Display { Ok(String::new()) }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_unit_struct("TestUnit");
    assert!(result.is_err());
}

