// Answer 0

#[test]
fn test_serialize_none() {
    struct MockSerializer {
        result: Result<(), String>,
    }

    impl serde::Serializer for MockSerializer {
        type Ok = ();
        type Error = String;

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Err("should not serialize bool".into()) }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Err("should not serialize u8".into()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Err("should not serialize u16".into()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Err("should not serialize u32".into()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Err("should not serialize u64".into()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Err("should not serialize i8".into()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Err("should not serialize i16".into()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Err("should not serialize i32".into()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Err("should not serialize i64".into()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Err("should not serialize f32".into()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Err("should not serialize f64".into()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Err("should not serialize char".into()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("should not serialize str".into()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Err("should not serialize bytes".into()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { 
            self.result = Ok(()); 
            Ok(())
        }
        fn serialize_some<V>(self, _: V) -> Result<Self::Ok, Self::Error> where V: Serialize { Err("should not serialize some".into()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err("should not serialize unit".into()) }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("should not serialize unit struct".into()) }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Err("should not serialize unit variant".into()) }
        fn serialize_newtype_struct<V>(self, _: &'static str, _: V) -> Result<Self::Ok, Self::Error> where V: Serialize { Err("should not serialize newtype struct".into()) }
        fn serialize_newtype_variant<V>(self, _: &'static str, _: u32, _: &'static str, _: V) -> Result<Self::Ok, Self::Error> where V: Serialize { Err("should not serialize newtype variant".into()) }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::Ok, Self::Error> { Err("should not serialize seq".into()) }
        fn serialize_tuple(self, _: usize) -> Result<Self::Ok, Self::Error> { Err("should not serialize tuple".into()) }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::Ok, Self::Error> { Err("should not serialize tuple struct".into()) }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::Ok, Self::Error> { Err("should not serialize tuple variant".into()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::Ok, Self::Error> { Err("should not serialize map".into()) }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::Ok, Self::Error> { Err("should not serialize struct".into()) }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::Ok, Self::Error> { Err("should not serialize struct variant".into()) }
    }

    enum Content {
        None,
    }

    impl Content {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match *self {
                Content::None => serializer.serialize_none(),
            }
        }
    }

    let content = Content::None;
    let mut serializer = MockSerializer { result: Err("not set".into()) };
    
    let result = content.serialize(serializer);
    assert!(result.is_ok());
}

