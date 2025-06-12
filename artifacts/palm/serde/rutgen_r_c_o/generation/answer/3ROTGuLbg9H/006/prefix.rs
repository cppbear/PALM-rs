// Answer 0

#[test]
fn test_deserialize_integer_with_u64_zero() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = u64;
        fn visit_u8(self, _: u8) -> Result<u64, serde::de::value::Error> { Err(serde::de::value::Error) }
        fn visit_u16(self, _: u16) -> Result<u64, serde::de::value::Error> { Err(serde::de::value::Error) }
        fn visit_u32(self, _: u32) -> Result<u64, serde::de::value::Error> { Err(serde::de::value::Error) }
        fn visit_u64(self, value: u64) -> Result<u64, serde::de::value::Error> { Ok(value) }
        fn visit_i8(self, _: i8) -> Result<u64, serde::de::value::Error> { Err(serde::de::value::Error) }
        fn visit_i16(self, _: i16) -> Result<u64, serde::de::value::Error> { Err(serde::de::value::Error) }
        fn visit_i32(self, _: i32) -> Result<u64, serde::de::value::Error> { Err(serde::de::value::Error) }
        fn visit_i64(self, _: i64) -> Result<u64, serde::de::value::Error> { Err(serde::de::value::Error) }
    }
    
    let content = Content::U64(0);
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };
    let visitor = VisitorImpl;
    let _ = deserializer.deserialize_integer(visitor);
}

#[test]
fn test_deserialize_integer_with_u64_max() {
    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = u64;
        fn visit_u8(self, _: u8) -> Result<u64, serde::de::value::Error> { Err(serde::de::value::Error) }
        fn visit_u16(self, _: u16) -> Result<u64, serde::de::value::Error> { Err(serde::de::value::Error) }
        fn visit_u32(self, _: u32) -> Result<u64, serde::de::value::Error> { Err(serde::de::value::Error) }
        fn visit_u64(self, value: u64) -> Result<u64, serde::de::value::Error> { Ok(value) }
        fn visit_i8(self, _: i8) -> Result<u64, serde::de::value::Error> { Err(serde::de::value::Error) }
        fn visit_i16(self, _: i16) -> Result<u64, serde::de::value::Error> { Err(serde::de::value::Error) }
        fn visit_i32(self, _: i32) -> Result<u64, serde::de::value::Error> { Err(serde::de::value::Error) }
        fn visit_i64(self, _: i64) -> Result<u64, serde::de::value::Error> { Err(serde::de::value::Error) }
    }
    
    let content = Content::U64(18_446_744_073_709_551_615);
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };
    let visitor = VisitorImpl;
    let _ = deserializer.deserialize_integer(visitor);
}

