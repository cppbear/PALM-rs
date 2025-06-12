// Answer 0

#[derive(Debug)]
enum Content {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

struct Deserializer<'de> {
    content: &'de Content,
}

impl<'de> Deserializer<'de> {
    fn invalid_type<V>(&self, visitor: &V) -> serde::de::Error
    where
        V: Visitor<'de>,
    {
        serde::de::Error::custom(format!("Invalid type for visitor {:?}", visitor))
    }

    fn deserialize_integer<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
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

struct TestVisitor {
    value: u8,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = u8;

    fn visit_u8(self, value: u8) -> Result<Self::Value, serde::de::Error> {
        Ok(value)
    }

    fn visit_u16(self, _value: u16) -> Result<Self::Value, serde::de::Error> {
        Err(serde::de::Error::custom("Expected u8"))
    }

    fn visit_u32(self, _value: u32) -> Result<Self::Value, serde::de::Error> {
        Err(serde::de::Error::custom("Expected u8"))
    }

    fn visit_u64(self, _value: u64) -> Result<Self::Value, serde::de::Error> {
        Err(serde::de::Error::custom("Expected u8"))
    }

    fn visit_i8(self, _value: i8) -> Result<Self::Value, serde::de::Error> {
        Err(serde::de::Error::custom("Expected u8"))
    }

    fn visit_i16(self, _value: i16) -> Result<Self::Value, serde::de::Error> {
        Err(serde::de::Error::custom("Expected u8"))
    }

    fn visit_i32(self, _value: i32) -> Result<Self::Value, serde::de::Error> {
        Err(serde::de::Error::custom("Expected u8"))
    }

    fn visit_i64(self, _value: i64) -> Result<Self::Value, serde::de::Error> {
        Err(serde::de::Error::custom("Expected u8"))
    }
}

#[test]
fn test_deserialize_integer_u8() {
    let content = Content::U8(42);
    let deserializer = Deserializer { content: &content };
    let visitor = TestVisitor { value: 0 };

    let result = deserializer.deserialize_integer(visitor);

    assert_eq!(result, Ok(42));
}

#[test]
fn test_deserialize_integer_invalid_type() {
    let content = Content::U16(42);
    let deserializer = Deserializer { content: &content };
    let visitor = TestVisitor { value: 0 };

    let result = deserializer.deserialize_integer(visitor);

    assert!(result.is_err());
}

