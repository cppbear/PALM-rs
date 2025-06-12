// Answer 0

#[test]
fn test_deserialize_any_map() {
    use crate::de::Visitor;

    struct TestVisitor {
        value: Option<(Content, Content)>,
        visited_count: u8,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_char<E>(self, _: char) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_borrowed_str<E>(self, _: &str) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_byte_buf<E>(self, _: Vec<u8>) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_borrowed_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_none<E>(self) -> Result<Self::Value, E> {
            Ok(())
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: DeserializeOwned,
        {
            self.visited_count += 1;
            Ok(())
        }

        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: DeserializeOwned,
        {
            Ok(())
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(())
        }

        fn visit_map<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            self.visited_count += 1;
            Ok(())
        }

        fn visit_enum<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: EnumAccess<'de>,
        {
            Ok(())
        }
    }

    let content = Content::Map(vec![
        (Content::String("key".to_string()), Content::U32(42)),
        (Content::String("another_key".to_string()), Content::Bool(true)),
    ]);
    
    let deserializer = ContentDeserializer::new(content);
    let mut visitor = TestVisitor { value: None, visited_count: 0 };

    let result = deserializer.deserialize_any(visitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ());
}

