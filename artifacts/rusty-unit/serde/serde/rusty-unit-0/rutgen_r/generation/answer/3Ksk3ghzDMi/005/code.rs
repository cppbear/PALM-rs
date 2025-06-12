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

trait Visitor<'de> {
    type Value;
    fn visit_u8(self, value: u8) -> Result<Self::Value, ()>;
    fn visit_u16(self, value: u16) -> Result<Self::Value, ()>;
    fn visit_u32(self, value: u32) -> Result<Self::Value, ()>;
    fn visit_u64(self, value: u64) -> Result<Self::Value, ()>;
    fn visit_i8(self, value: i8) -> Result<Self::Value, ()>;
    fn visit_i16(self, value: i16) -> Result<Self::Value, ()>;
    fn visit_i32(self, value: i32) -> Result<Self::Value, ()>;
    fn visit_i64(self, value: i64) -> Result<Self::Value, ()>;
}

struct Deserializer {
    content: Content,
}

impl Deserializer {
    fn invalid_type<V>(&self, _visitor: &V) -> () {
        panic!("Invalid type");
    }

    fn deserialize_integer<V>(self, visitor: V) -> Result<V::Value, ()>
    where
        V: Visitor<'_>,
    {
        match self.content {
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
    value: i8,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = i8;
    fn visit_u8(self, _: u8) -> Result<Self::Value, ()> { unreachable!() }
    fn visit_u16(self, _: u16) -> Result<Self::Value, ()> { unreachable!() }
    fn visit_u32(self, _: u32) -> Result<Self::Value, ()> { unreachable!() }
    fn visit_u64(self, _: u64) -> Result<Self::Value, ()> { unreachable!() }
    fn visit_i8(self, value: i8) -> Result<Self::Value, ()> { Ok(value) }
    fn visit_i16(self, _: i16) -> Result<Self::Value, ()> { unreachable!() }
    fn visit_i32(self, _: i32) -> Result<Self::Value, ()> { unreachable!() }
    fn visit_i64(self, _: i64) -> Result<Self::Value, ()> { unreachable!() }
}

#[test]
fn test_deserialize_integer_i8() {
    let deserializer = Deserializer { content: Content::I8(42) };
    let visitor = TestVisitor { value: 0 };
    let result = deserializer.deserialize_integer(visitor);
    assert_eq!(result, Ok(42));
}

#[test]
#[should_panic]
fn test_deserialize_integer_invalid_type() {
    let deserializer = Deserializer { content: Content::U8(1) };
    let visitor = TestVisitor { value: 0 };
    let _ = deserializer.deserialize_integer(visitor);
}

