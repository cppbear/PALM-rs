// Answer 0

#[test]
fn test_deserialize_float_invalid_type() {
    use std::marker::PhantomData;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_f32(self, _: f32) -> Result<Self::Value, ()> { Err(()) }
        fn visit_f64(self, _: f64) -> Result<Self::Value, ()> { Err(()) }
        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> { Err(()) }
        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> { Err(()) }
        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> { Err(()) }
        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i8(self, _: i8) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> { Err(()) }
        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> { Err(()) }
        fn visit_char(self, _: char) -> Result<Self::Value, ()> { Err(()) }
        fn visit_str(self, _: &str) -> Result<Self::Value, ()> { Err(()) }
        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value, ()> { Err(()) }
        fn visit_unit(self) -> Result<Self::Value, ()> { Err(()) }
        fn visit_none(self) -> Result<Self::Value, ()> { Err(()) }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, ()> where V: Visitor<'de> { Err(()) }
    }

    struct MockDeserializer<'de> {
        content: Content<'de>,
    }

    impl<'de, E> ContentDeserializer<'de, E> 
    where
        E: std::fmt::Debug,
    {
        fn new(content: Content<'de>) -> Self {
            ContentDeserializer {
                content,
                err: PhantomData,
            }
        }

        fn deserialize_float<V>(self, visitor: V) -> Result<V::Value, E>
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
                _ => Err((self.invalid_type(&visitor))),
            }
        }
    }

    let deserializer = MockDeserializer::new(Content::Bytes(vec![1, 2, 3]));
    let result = deserializer.deserialize_float(TestVisitor);
    assert!(result.is_err());
}

