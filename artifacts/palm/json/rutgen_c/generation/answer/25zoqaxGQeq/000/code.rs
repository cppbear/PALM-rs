// Answer 0

#[test]
fn test_serialize_char() {
    struct TestSerializer;

    impl serde::Serializer for TestSerializer {
        type Ok = Value;
        type Error = Error;
        type SerializeSeq = SerializeVec;
        type SerializeTuple = SerializeVec;
        type SerializeTupleStruct = SerializeVec;
        type SerializeTupleVariant = SerializeTupleVariant;
        type SerializeMap = SerializeMap;
        type SerializeStruct = SerializeMap;
        type SerializeStructVariant = SerializeStructVariant;

        fn serialize_bool(self, _: bool) -> Result<Value> { unimplemented!() }
        fn serialize_i8(self, _: i8) -> Result<Value> { unimplemented!() }
        fn serialize_i16(self, _: i16) -> Result<Value> { unimplemented!() }
        fn serialize_i32(self, _: i32) -> Result<Value> { unimplemented!() }
        fn serialize_i64(self, _: i64) -> Result<Value> { unimplemented!() }
        fn serialize_i128(self, _: i128) -> Result<Value> { unimplemented!() }
        fn serialize_u8(self, _: u8) -> Result<Value> { unimplemented!() }
        fn serialize_u16(self, _: u16) -> Result<Value> { unimplemented!() }
        fn serialize_u32(self, _: u32) -> Result<Value> { unimplemented!() }
        fn serialize_u64(self, _: u64) -> Result<Value> { unimplemented!() }
        fn serialize_u128(self, _: u128) -> Result<Value> { unimplemented!() }
        fn serialize_f32(self, _: f32) -> Result<Value> { unimplemented!() }
        fn serialize_f64(self, _: f64) -> Result<Value> { unimplemented!() }
        fn serialize_char(self, value: char) -> Result<Value> {
            let mut s = String::new();
            s.push(value);
            Ok(Value::String(s))
        }
        fn serialize_str(self, _: &str) -> Result<Value> { unimplemented!() }
        fn serialize_bytes(self, _: &[u8]) -> Result<Value> { unimplemented!() }
        fn serialize_unit(self) -> Result<Value> { unimplemented!() }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Value> { unimplemented!() }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Value> { unimplemented!() }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Value>
            where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Value>
            where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_none(self) -> Result<Value> { unimplemented!() }
        fn serialize_some<T>(self, _: &T) -> Result<Value>
            where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
            Ok(SerializeVec { vec: Vec::with_capacity(len.unwrap_or(0)) })
        }
        fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
            self.serialize_seq(Some(len))
        }
        fn serialize_tuple_struct(self, _: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
            self.serialize_seq(Some(len))
        }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, len: usize) -> Result<Self::SerializeTupleVariant> {
            Ok(SerializeTupleVariant { name: String::new(), vec: Vec::with_capacity(len) })
        }
        fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
            Ok(SerializeMap::Map { map: Map::with_capacity(len.unwrap_or(0)), next_key: None })
        }
        fn serialize_struct(self, _: &'static str, len: usize) -> Result<Self::SerializeStruct> {
            self.serialize_map(Some(len))
        }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant> {
            Ok(SerializeStructVariant { name: String::new(), map: Map::new() })
        }
        fn collect_str<T>(self, _: &T) -> Result<Value>
            where T: ?Sized + Display { unimplemented!() }
    }

    let serializer = TestSerializer {};
    let result = serializer.serialize_char('a').unwrap();

    if let Value::String(s) = result {
        assert_eq!(s, String::from("a"));
    } else {
        panic!("Expected Value::String");
    }
}

