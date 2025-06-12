// Answer 0

#[test]
fn test_deserialize_u8_success() {
    struct TestVisitor {
        value: Option<u8>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = u8;

        fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }

        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_char<E>(self, _: char) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_string<E>(self, _: String) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_byte_buf<E>(self, _: Vec<u8>) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_none<E>(self) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_some<D>(self, _: D) -> Result<Self::Value, E> where D: Deserializer<'de>, E: de::Error { unimplemented!() }
        fn visit_unit<E>(self) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_newtype_struct<D>(self, _: D) -> Result<Self::Value, E> where D: Deserializer<'de>, E: de::Error { unimplemented!() }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, E> where V: SeqAccess<'de>, E: de::Error { unimplemented!() }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, E> where V: MapAccess<'de>, E: de::Error { unimplemented!() }
        fn visit_unit_struct<E>(self, _: &'static str) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_tuple<V>(self, _: V) -> Result<Self::Value, E> where V: SeqAccess<'de>, E: de::Error { unimplemented!() }
        fn visit_struct<V>(self, _: &'static str, _: V) -> Result<Self::Value, E> where V: MapAccess<'de>, E: de::Error { unimplemented!() }
    }

    let content = Content::U8(42);
    let deserializer = ContentDeserializer { content, err: PhantomData::<()>::default() };
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_u8(visitor).unwrap();
    assert_eq!(result, 42);
}

#[test]
fn test_deserialize_u8_invalid() {
    struct TestVisitor {
        value: Option<u8>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = u8;

        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_char<E>(self, _: char) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_string<E>(self, _: String) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_byte_buf<E>(self, _: Vec<u8>) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_none<E>(self) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_some<D>(self, _: D) -> Result<Self::Value, E> where D: Deserializer<'de>, E: de::Error { unimplemented!() }
        fn visit_unit<E>(self) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_newtype_struct<D>(self, _: D) -> Result<Self::Value, E> where D: Deserializer<'de>, E: de::Error { unimplemented!() }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, E> where V: SeqAccess<'de>, E: de::Error { unimplemented!() }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, E> where V: MapAccess<'de>, E: de::Error { unimplemented!() }
        fn visit_unit_struct<E>(self, _: &'static str) -> Result<Self::Value, E> where E: de::Error { unimplemented!() }
        fn visit_tuple<V>(self, _: V) -> Result<Self::Value, E> where V: SeqAccess<'de>, E: de::Error { unimplemented!() }
        fn visit_struct<V>(self, _: &'static str, _: V) -> Result<Self::Value, E> where V: MapAccess<'de>, E: de::Error { unimplemented!() }
    }

    let content = Content::Bool(true);
    let deserializer = ContentDeserializer { content, err: PhantomData::<()>::default() };
    let visitor = TestVisitor { value: None };
    assert!(deserializer.deserialize_u8(visitor).is_err());
}

