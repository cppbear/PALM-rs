// Answer 0

#[test]
fn test_deserialize_any_bool() {
    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        // Implement other visitor methods as no-op
        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> { unreachable!() }
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> { unreachable!() }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> { unreachable!() }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { unreachable!() }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> { unreachable!() }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> { unreachable!() }
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> { unreachable!() }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { unreachable!() }
        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> { unreachable!() }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> { unreachable!() }
        fn visit_char<E>(self, _: char) -> Result<Self::Value, E> { unreachable!() }
        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> { unreachable!() }
        fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E> { unreachable!() }
        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> { unreachable!() }
        fn visit_borrowed_bytes<E>(self, _: &'de [u8]) -> Result<Self::Value, E> { unreachable!() }
        fn visit_unit<E>(self) -> Result<Self::Value, E> { unreachable!() }
        fn visit_none<E>(self) -> Result<Self::Value, E> { unreachable!() }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, E> { unreachable!() }
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, E> { unreachable!() }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, E> { unreachable!() }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, E> { unreachable!() }
    }

    let content = Content::Bool(true);
    let deserializer = ContentRefDeserializer::new(&content);
    let result = deserializer.deserialize_any(BoolVisitor);
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_deserialize_any_u8() {
    struct U8Visitor;

    impl<'de> Visitor<'de> for U8Visitor {
        type Value = u8;

        fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E> {
            Ok(value)
        }

        // Implement other visitor methods as no-op
        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> { unreachable!() }
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> { unreachable!() }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> { unreachable!() }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { unreachable!() }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> { unreachable!() }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> { unreachable!() }
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> { unreachable!() }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { unreachable!() }
        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> { unreachable!() }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> { unreachable!() }
        fn visit_char<E>(self, _: char) -> Result<Self::Value, E> { unreachable!() }
        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> { unreachable!() }
        fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E> { unreachable!() }
        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> { unreachable!() }
        fn visit_borrowed_bytes<E>(self, _: &'de [u8]) -> Result<Self::Value, E> { unreachable!() }
        fn visit_unit<E>(self) -> Result<Self::Value, E> { unreachable!() }
        fn visit_none<E>(self) -> Result<Self::Value, E> { unreachable!() }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, E> { unreachable!() }
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, E> { unreachable!() }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, E> { unreachable!() }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, E> { unreachable!() }
    }

    let content = Content::U8(255);
    let deserializer = ContentRefDeserializer::new(&content);
    let result = deserializer.deserialize_any(U8Visitor);
    assert_eq!(result.unwrap(), 255);
}

