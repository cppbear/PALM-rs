// Answer 0

#[test]
fn test_deserialize_any_with_string() {
    use std::marker::PhantomData;

    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();
        fn visit_bool(self, _: bool) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_f32(self, _: f32) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_char(self, _: char) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_str(self, _: &str) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_unit(self) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_none(self) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>> 
        where V: Visitor<'de, Value = Self::Value> { Ok(()) }
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>> 
        where V: Visitor<'de> { Ok(()) }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>> 
        where V: Visitor<'de> { Ok(()) }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>> 
        where V: Visitor<'de> { Ok(()) }
    }

    let content = Content::String(String::from("test"));
    let deserializer = ContentRefDeserializer::new(&content);
    deserializer.deserialize_any(VisitorImpl).unwrap();

    let content = Content::String(String::from("hello world"));
    let deserializer = ContentRefDeserializer::new(&content);
    deserializer.deserialize_any(VisitorImpl).unwrap();

    let content = Content::String(String::from("a"));
    let deserializer = ContentRefDeserializer::new(&content);
    deserializer.deserialize_any(VisitorImpl).unwrap();
    
    let content = Content::String(String::from("1"));
    let deserializer = ContentRefDeserializer::new(&content);
    deserializer.deserialize_any(VisitorImpl).unwrap();
}

#[test]
fn test_deserialize_any_with_bytes() {
    use std::marker::PhantomData;

    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = ();
        fn visit_bool(self, _: bool) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_f32(self, _: f32) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_char(self, _: char) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_str(self, _: &str) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_unit(self) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_none(self) -> Result<Self::Value, Box<dyn std::error::Error>> { Ok(()) }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>> 
        where V: Visitor<'de, Value = Self::Value> { Ok(()) }
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>> 
        where V: Visitor<'de> { Ok(()) }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>> 
        where V: Visitor<'de> { Ok(()) }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>> 
        where V: Visitor<'de> { Ok(()) }
    }

    let content = Content::Bytes(vec![b't', b'e', b's', b't']);
    let deserializer = ContentRefDeserializer::new(&content);
    deserializer.deserialize_any(VisitorImpl).unwrap();

    let content = Content::Bytes(vec![b'h', b'e', b'l', b'l', b'o']);
    let deserializer = ContentRefDeserializer::new(&content);
    deserializer.deserialize_any(VisitorImpl).unwrap();
}

