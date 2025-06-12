// Answer 0

#[derive(Debug)]
enum Content {
    Invalid,
}

struct Deserializer {
    content: Content,
}

impl Deserializer {
    fn invalid_type<V>(&self, _visitor: &V) -> String {
        "Invalid type".to_string()
    }

    fn deserialize_float<V>(self, visitor: V) -> Result<V::Value, String>
    where
        V: Visitor<'_>,
    {
        match self.content {
            Content::Invalid => Err(self.invalid_type(&visitor)),
            // Other matches omitted for brevity
        }
    }
}

trait Visitor<'de> {
    type Value;

    fn visit_f32(self, _: f32) -> Result<Self::Value, String>;
    fn visit_f64(self, _: f64) -> Result<Self::Value, String>;
    fn visit_u8(self, _: u8) -> Result<Self::Value, String>;
    fn visit_u16(self, _: u16) -> Result<Self::Value, String>;
    fn visit_u32(self, _: u32) -> Result<Self::Value, String>;
    fn visit_u64(self, _: u64) -> Result<Self::Value, String>;
    fn visit_i8(self, _: i8) -> Result<Self::Value, String>;
    fn visit_i16(self, _: i16) -> Result<Self::Value, String>;
    fn visit_i32(self, _: i32) -> Result<Self::Value, String>;
    fn visit_i64(self, _: i64) -> Result<Self::Value, String>;
}

struct MockVisitor;

impl<'de> Visitor<'de> for MockVisitor {
    type Value = ();

    fn visit_f32(self, _: f32) -> Result<Self::Value, String> { Err("not called".to_string()) }
    fn visit_f64(self, _: f64) -> Result<Self::Value, String> { Err("not called".to_string()) }
    fn visit_u8(self, _: u8) -> Result<Self::Value, String> { Err("not called".to_string()) }
    fn visit_u16(self, _: u16) -> Result<Self::Value, String> { Err("not called".to_string()) }
    fn visit_u32(self, _: u32) -> Result<Self::Value, String> { Err("not called".to_string()) }
    fn visit_u64(self, _: u64) -> Result<Self::Value, String> { Err("not called".to_string()) }
    fn visit_i8(self, _: i8) -> Result<Self::Value, String> { Err("not called".to_string()) }
    fn visit_i16(self, _: i16) -> Result<Self::Value, String> { Err("not called".to_string()) }
    fn visit_i32(self, _: i32) -> Result<Self::Value, String> { Err("not called".to_string()) }
    fn visit_i64(self, _: i64) -> Result<Self::Value, String> { Err("not called".to_string()) }
}

#[test]
fn test_deserialize_float_invalid() {
    let deserializer = Deserializer { content: Content::Invalid };
    let visitor = MockVisitor;

    let result = deserializer.deserialize_float(visitor);
    assert_eq!(result, Err("Invalid type".to_string()));
}

