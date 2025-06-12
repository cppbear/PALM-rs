// Answer 0

#[test]
fn test_deserialize_any_bool() {
    struct TestVisitor {
        value: Option<bool>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        // Implement other Visitor trait methods as no-ops or return errors
        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_char<E>(self, _: char) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_string<E>(self, _: String) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_byte_buf<E>(self, _: Vec<u8>) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_borrowed_bytes<E>(self, _: &'de [u8]) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_unit<E>(self) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_none<E>(self) -> Result<Self::Value, E> { Err(E::custom("Expected bool")) }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, E> where V: DeserializeSeed<'de> { Err(E::custom("Expected bool")) }
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, E> where V: DeserializeSeed<'de> { Err(E::custom("Expected bool")) }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, E> where V: SeqAccess<'de> { Err(E::custom("Expected bool")) }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, E> where V: MapAccess<'de> { Err(E::custom("Expected bool")) }
    }

    let content_bool = Content::Bool(true);
    let deserializer = ContentDeserializer::new(content_bool);
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_any(visitor);

    assert_eq!(result, Ok(true));
}

#[test]
fn test_deserialize_any_u8() {
    struct TestVisitor {
        value: Option<u8>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = u8;

        fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E> {
            Ok(value)
        }

        // Implement other Visitor trait methods as no-ops or return errors
        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_char<E>(self, _: char) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_string<E>(self, _: String) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_byte_buf<E>(self, _: Vec<u8>) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_borrowed_bytes<E>(self, _: &'de [u8]) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_unit<E>(self) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_none<E>(self) -> Result<Self::Value, E> { Err(E::custom("Expected u8")) }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, E> where V: DeserializeSeed<'de> { Err(E::custom("Expected u8")) }
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, E> where V: DeserializeSeed<'de> { Err(E::custom("Expected u8")) }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, E> where V: SeqAccess<'de> { Err(E::custom("Expected u8")) }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, E> where V: MapAccess<'de> { Err(E::custom("Expected u8")) }
    }

    let content_u8 = Content::U8(10);
    let deserializer = ContentDeserializer::new(content_u8);
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_any(visitor);

    assert_eq!(result, Ok(10));
}

