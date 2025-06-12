// Answer 0

#[test]
fn test_serialize_bool_true() {
    struct MockSerializer {
        result: Result<(), String>,
    }

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
            self.result
        }

        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_some<V: Serialize>(self, _: &V) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_unit_struct(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_unit_variant(self, _: &str, _: u32, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_newtype_struct<V: Serialize>(self, _: &str, _: &V) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_newtype_variant<V: Serialize>(self, _: &str, _: u32, _: &str, _: &V) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_tuple(self, _: usize) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_tuple_struct(self, _: &str, _: usize) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_tuple_variant(self, _: &str, _: u32, _: &str, _: usize) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_struct(self, _: &str, _: usize) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_struct_variant(self, _: &str, _: u32, _: &str, _: usize) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
    }

    let content = Content::Bool(true);
    let serializer = MockSerializer { result: Ok(()) };
    let result = content.serialize(serializer);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_bool_false() {
    struct MockSerializer {
        result: Result<(), String>,
    }

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
            self.result
        }

        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_some<V: Serialize>(self, _: &V) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_unit_struct(self, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_unit_variant(self, _: &str, _: u32, _: &str) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_newtype_struct<V: Serialize>(self, _: &str, _: &V) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_newtype_variant<V: Serialize>(self, _: &str, _: u32, _: &str, _: &V) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_tuple(self, _: usize) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_tuple_struct(self, _: &str, _: usize) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_tuple_variant(self, _: &str, _: u32, _: &str, _: usize) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_struct(self, _: &str, _: usize) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
        fn serialize_struct_variant(self, _: &str, _: u32, _: &str, _: usize) -> Result<Self::Ok, Self::Error> { Err("Not expected".to_string()) }
    }

    let content = Content::Bool(false);
    let serializer = MockSerializer { result: Ok(()) };
    let result = content.serialize(serializer);
    assert!(result.is_ok());
}

