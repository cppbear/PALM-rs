// Answer 0

#[derive(Debug)]
struct DummyVisitor {
    value: Option<i64>,
}

impl<'de> Visitor<'de> for DummyVisitor {
    type Value = i64;

    fn visit_bool(self, _: bool) -> Result<Self::Value, E> {
        Err(E)
    }
    
    fn visit_u8(self, _: u8) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_u16(self, _: u16) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_u32(self, _: u32) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_u64(self, _: u64) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_i8(self, _: i8) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_i16(self, _: i16) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_i32(self, _: i32) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_i64(self, value: i64) -> Result<Self::Value, E> {
        Ok(value)
    }

    fn visit_f32(self, _: f32) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_f64(self, _: f64) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_char(self, _: char) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_str(self, _: &str) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_borrowed_str(self, _: &str) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_bytes(self, _: &[u8]) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_borrowed_bytes(self, _: &[u8]) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_unit(self) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_none(self) -> Result<Self::Value, E> {
        Err(E)
    }

    fn visit_some<V>(self, _: V) -> Result<Self::Value, E>
    where
        V: Deserializer<'de>,
    {
        Err(E)
    }

    fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, E>
    where
        V: Deserializer<'de>,
    {
        Err(E)
    }

    fn visit_seq<V>(self, _: V) -> Result<Self::Value, E>
    where
        V: SeqAccess<'de>,
    {
        Err(E)
    }

    fn visit_map<V>(self, _: V) -> Result<Self::Value, E>
    where
        V: MapAccess<'de>,
    {
        Err(E)
    }
}

#[test]
fn test_deserialize_i64() {
    struct DummyDeserializer {
        content: Content,
    }

    impl DummyDeserializer {
        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, E>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::I64(v) => visitor.visit_i64(v),
                _ => Err(E),
            }
        }
    }

    let deserializer = DummyDeserializer { 
        content: Content::I64(42),
    };
    let visitor = DummyVisitor { value: None };

    let result = deserializer.deserialize_any(visitor);
    assert_eq!(result, Ok(42));
}

