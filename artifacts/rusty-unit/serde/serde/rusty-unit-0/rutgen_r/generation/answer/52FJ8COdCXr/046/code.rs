// Answer 0

#[test]
fn test_serialize_bool() {
    struct TestSerializer {
        value: Option<bool>,
    }

    impl serde::Serializer for TestSerializer {
        type Ok = Option<bool>;
        type Error = std::convert::Infallible;

        fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
            self.value = Some(v);
            Ok(self.value)
        }

        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_some<T: ?Sized>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: serde::Serialize { unreachable!() }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_newtype_struct<T: ?Sized>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: serde::Serialize { unreachable!() }
        fn serialize_newtype_variant<T: ?Sized>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: serde::Serialize { unreachable!() }
        fn serialize_tuple(self, _: usize) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::Ok, Self::Error> { unreachable!() }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::Ok, Self::Error> { unreachable!() }
    }

    let serializer = TestSerializer { value: None };
    let content_true = Content::Bool(true);
    let content_false = Content::Bool(false);

    assert_eq!(content_true.serialize(serializer), Ok(Some(true)));
    assert_eq!(content_false.serialize(serializer), Ok(Some(false)));
}

