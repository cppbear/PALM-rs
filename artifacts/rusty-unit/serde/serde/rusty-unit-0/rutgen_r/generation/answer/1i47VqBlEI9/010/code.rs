// Answer 0

#[derive(Debug)]
struct TestVisitor {
    value: Option<u16>,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = u16;

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E> {
        Ok(value)
    }

    fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
        Err(E::custom("visit_f32 not expected"))
    }

    fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
        Err(E::custom("visit_f64 not expected"))
    }

    fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
        Err(E::custom("visit_u8 not expected"))
    }

    fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
        Err(E::custom("visit_u32 not expected"))
    }

    fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
        Err(E::custom("visit_u64 not expected"))
    }

    fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
        Err(E::custom("visit_i8 not expected"))
    }

    fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
        Err(E::custom("visit_i16 not expected"))
    }

    fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
        Err(E::custom("visit_i32 not expected"))
    }

    fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
        Err(E::custom("visit_i64 not expected"))
    }
}

#[derive(Debug)]
enum Content {
    F32(f32),
    F64(f64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Invalid,
}

#[derive(Debug)]
struct TestDeserializer {
    content: Content,
}

impl TestDeserializer {
    fn invalid_type<E>(&self, visitor: &dyn Visitor<'_>) -> E {
        panic!("Invalid type for visitor: {:?}", visitor)
    }

    fn deserialize_float<V>(self, visitor: V) -> Result<V::Value, String>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::F32(v) => visitor.visit_f32(v),
            Content::F64(v) => visitor.visit_f64(v),
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

#[test]
fn test_deserialize_float_u16() {
    let deserializer = TestDeserializer {
        content: Content::U16(42),
    };
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_float(visitor);
    assert_eq!(result, Ok(42));
}

