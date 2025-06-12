// Answer 0

#[test]
fn test_deserialize_any_with_i32() {
    use std::marker::PhantomData;
    use serde::de::{self, Visitor};

    struct TestVisitor {
        value: Option<i32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
            panic!("Expected i32 but received bool");
        }

        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            panic!("Expected i32 but received u8");
        }

        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            panic!("Expected i32 but received u16");
        }

        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            panic!("Expected i32 but received u32");
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            panic!("Expected i32 but received u64");
        }

        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            panic!("Expected i32 but received i8");
        }

        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
            panic!("Expected i32 but received i16");
        }

        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
            panic!("Expected i32 but received f32");
        }

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            panic!("Expected i32 but received f64");
        }

        fn visit_char<E>(self, _: char) -> Result<Self::Value, E> {
            panic!("Expected i32 but received char");
        }

        fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
            panic!("Expected i32 but received string");
        }

        fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E> {
            panic!("Expected i32 but received borrowed str");
        }

        fn visit_byte_buf<E>(self, _: Vec<u8>) -> Result<Self::Value, E> {
            panic!("Expected i32 but received byte buf");
        }

        fn visit_borrowed_bytes<E>(self, _: &'de [u8]) -> Result<Self::Value, E> {
            panic!("Expected i32 but received borrowed bytes");
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            panic!("Expected i32 but received unit");
        }

        fn visit_none<E>(self) -> Result<Self::Value, E> {
            panic!("Expected i32 but received none");
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, E>
        where
            V: de::Deserialize<'de>,
        {
            panic!("Expected i32 but received some");
        }

        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, E>
        where
            V: de::Deserialize<'de>,
        {
            panic!("Expected i32 but received newtype struct");
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, E>
        where
            V: de::SeqAccess<'de>,
        {
            panic!("Expected i32 but received sequence");
        }

        fn visit_map<V>(self, _: V) -> Result<Self::Value, E>
        where
            V: de::MapAccess<'de>,
        {
            panic!("Expected i32 but received map");
        }
    }

    let content = Content::I32(42);
    let deserializer = ContentDeserializer::<de::value::Error>::new(content);
    let visitor = TestVisitor { value: None };

    let result: Result<i32, _> = deserializer.deserialize_any(visitor);
    assert_eq!(result.unwrap(), 42);
}

