// Answer 0

#[test]
fn test_deserialize_any_content_none() {
    struct VisitorMock {
        result: Option<Result<(), Box<dyn std::error::Error>>>,
    }

    impl<'de> Visitor<'de> for VisitorMock {
        type Value = ();
        
        fn visit_bool(self, _: bool) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_f32(self, _: f32) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_char(self, _: char) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_string(self, _: String) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_unit(self) -> Result<Self::Value, Self::Error> { Ok(()) }
        fn visit_none(self) -> Result<Self::Value, Self::Error> { Ok(()) }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, Self::Error> where V: Deserializer<'de> { Err("Unexpected call".into()) }
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, Self::Error> where V: Deserializer<'de> { Err("Unexpected call".into()) }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Self::Error> where V: SeqAccess<'de> { Err("Unexpected call".into()) }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, Self::Error> where V: MapAccess<'de> { Err("Unexpected call".into()) }
    }

    let content = Content::None;
    let deserializer = ContentDeserializer::new(content);
    let visitor = VisitorMock { result: None };

    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ());
}

#[test]
fn test_deserialize_any_content_some() {
    struct VisitorMock {
        result: Option<Result<(), Box<dyn std::error::Error>>>,
    }

    impl<'de> Visitor<'de> for VisitorMock {
        type Value = ();
        
        fn visit_bool(self, _: bool) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_f32(self, _: f32) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_char(self, _: char) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_string(self, _: String) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_unit(self) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_none(self) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, Self::Error> where V: Deserializer<'de> { Ok(()) }
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, Self::Error> where V: Deserializer<'de> { Ok(()) }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Self::Error> where V: SeqAccess<'de> { Err("Unexpected call".into()) }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, Self::Error> where V: MapAccess<'de> { Err("Unexpected call".into()) }
    }

    let content = Content::Some(Box::new(Content::Bool(true)));
    let deserializer = ContentDeserializer::new(content);
    let visitor = VisitorMock { result: None };

    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ());
}

#[test]
fn test_deserialize_any_content_unit() {
    struct VisitorMock {
        result: Option<Result<(), Box<dyn std::error::Error>>>,
    }

    impl<'de> Visitor<'de> for VisitorMock {
        type Value = ();
        
        fn visit_bool(self, _: bool) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_f32(self, _: f32) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_char(self, _: char) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_string(self, _: String) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_unit(self) -> Result<Self::Value, Self::Error> { Ok(()) }
        fn visit_none(self) -> Result<Self::Value, Self::Error> { Err("Unexpected call".into()) }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, Self::Error> where V: Deserializer<'de> { Err("Unexpected call".into()) }
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, Self::Error> where V: Deserializer<'de> { Err("Unexpected call".into()) }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Self::Error> where V: SeqAccess<'de> { Err("Unexpected call".into()) }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, Self::Error> where V: MapAccess<'de> { Err("Unexpected call".into()) }
    }

    let content = Content::Unit;
    let deserializer = ContentDeserializer::new(content);
    let visitor = VisitorMock { result: None };

    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ());
}

