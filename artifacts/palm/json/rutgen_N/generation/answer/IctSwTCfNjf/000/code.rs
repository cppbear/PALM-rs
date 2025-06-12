// Answer 0

#[test]
fn test_serialize_seq_errors_when_called() {
    struct Serializer;

    impl serde::ser::Serializer for Serializer {
        type Ok = ();
        type Error = serde::ser::Error;
        type SerializeSeq = ();

        fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
            Err(serde::ser::Error::custom("key must be a string"))
        }

        // Required trait methods for Serializer, but unimplemented for this test
        fn serialize_bool(self, v: bool) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_i8(self, v: i8) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_i16(self, v: i16) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_i32(self, v: i32) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_i64(self, v: i64) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_u8(self, v: u8) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_u16(self, v: u16) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_u32(self, v: u32) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_u64(self, v: u64) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_f32(self, v: f32) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_f64(self, v: f64) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_char(self, v: char) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_str(self, v: &str) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_unit(self) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_newtype_struct<T: serde::ser::Serialize>(self, name: &'static str, value: &T) -> Result<Self::Ok> { unimplemented!() }
        fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
            Err(serde::ser::Error::custom("key must be a string"))
        }
        fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> { unimplemented!() }
        fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> { unimplemented!() }
        fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStruct> { unimplemented!() }
        fn collect_str<T: core::fmt::Write>(&mut self, writer: &mut T) -> Result<Self::Ok> { unimplemented!() }
    }

    let serializer = Serializer;
    let result = serializer.serialize_seq(None);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.to_string(), "key must be a string");
    }
}

