// Answer 0

#[derive(Debug)]
enum Content {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

struct Deserializer<'de> {
    content: &'de Content,
}

impl<'de> Deserializer<'de> {
    fn invalid_type<V>(&self, _visitor: &V) -> &'static str {
        "invalid type"
    }

    fn deserialize_integer<V>(self, visitor: V) -> Result<V::Value, &'static str>
    where
        V: Visitor<'de>,
    {
        match *self.content {
            Content::U8(v) => visitor.visit_u8(v),
            Content::U16(v) => visitor.visit_u16(v),
            Content::U32(v) => visitor.visit_u32(v),
            Content::U64(v) => visitor.visit_u64(v),
            Content::I8(v) => visitor.visit_i8(v),
            Content::I16(v) => visitor.visit_i16(v),
            Content::I32(v) => visitor.visit_i32(v),
            Content::I64(v) => visitor.visit_i64(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

trait Visitor<'de> {
    type Value;

    fn visit_u8(self, value: u8) -> Result<Self::Value, &'static str>;
    fn visit_u16(self, value: u16) -> Result<Self::Value, &'static str>;
    fn visit_u32(self, value: u32) -> Result<Self::Value, &'static str>;
    fn visit_u64(self, value: u64) -> Result<Self::Value, &'static str>;
    fn visit_i8(self, value: i8) -> Result<Self::Value, &'static str>;
    fn visit_i16(self, value: i16) -> Result<Self::Value, &'static str>;
    fn visit_i32(self, value: i32) -> Result<Self::Value, &'static str>;
    fn visit_i64(self, value: i64) -> Result<Self::Value, &'static str>;
}

struct TestVisitor {
    value: Option<i8>,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = Option<i8>;

    fn visit_u8(self, value: u8) -> Result<Self::Value, &'static str> {
        Err("Unexpected type: u8")
    }

    fn visit_u16(self, value: u16) -> Result<Self::Value, &'static str> {
        Err("Unexpected type: u16")
    }

    fn visit_u32(self, value: u32) -> Result<Self::Value, &'static str> {
        Err("Unexpected type: u32")
    }

    fn visit_u64(self, value: u64) -> Result<Self::Value, &'static str> {
        Err("Unexpected type: u64")
    }

    fn visit_i8(self, value: i8) -> Result<Self::Value, &'static str> {
        self.value = Some(value);
        Ok(self.value)
    }

    fn visit_i16(self, value: i16) -> Result<Self::Value, &'static str> {
        Err("Unexpected type: i16")
    }

    fn visit_i32(self, value: i32) -> Result<Self::Value, &'static str> {
        Err("Unexpected type: i32")
    }

    fn visit_i64(self, value: i64) -> Result<Self::Value, &'static str> {
        Err("Unexpected type: i64")
    }
}

#[test]
fn test_deserialize_integer_i8() {
    let content = Content::I8(42);
    let deserializer = Deserializer { content: &content };
    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_integer(visitor);

    assert_eq!(result, Ok(Some(42)));
}

#[test]
fn test_deserialize_integer_invalid() {
    let content = Content::U8(255);
    let deserializer = Deserializer { content: &content };
    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_integer(visitor);

    assert_eq!(result, Err("Unexpected type: u8"));
}

