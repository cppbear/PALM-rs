// Answer 0

#[test]
fn test_deserialize_ignored_any() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn visit_unit(self) -> Result<Self::Value, ()> {
            Ok(())
        }

        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> { Err(()) }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> { Err(()) }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> { Err(()) }
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> { Err(()) }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { Err(()) }
        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> { Err(()) }
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> { Err(()) }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> { Err(()) }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { Err(()) }
        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> { Err(()) }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> { Err(()) }
        fn visit_char<E>(self, _: char) -> Result<Self::Value, E> { Err(()) }
        fn visit_str<E>(self, _: &'de str) -> Result<Self::Value, E> { Err(()) }
        fn visit_string<E>(self, _: String) -> Result<Self::Value, E> { Err(()) }
        fn visit_bytes<E>(self, _: &'de [u8]) -> Result<Self::Value, E> { Err(()) }
        fn visit_byte_buf<E>(self, _: Vec<u8>) -> Result<Self::Value, E> { Err(()) }
        fn visit_option<V>(self, _: Option<V>) -> Result<Self::Value, ()> where V: Visitor<'de> { Err(()) }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, ()> where V: Visitor<'de> { Err(()) }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, ()> where V: Visitor<'de> { Err(()) }
    }

    struct DummyDeserializer<'de> {
        _marker: std::marker::PhantomData<&'de ()>,
    }

    impl<'de> Deserializer<'de> for DummyDeserializer<'de> {
        type Error = ();

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.deserialize_ignored_any(visitor)
        }

        // Implementing this method since it's required by the trait
        fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_unit()
        }
    }

    let deserializer = DummyDeserializer { _marker: std::marker::PhantomData };
    let visitor = TestVisitor;
    let result = deserializer.deserialize_ignored_any(visitor);
    assert_eq!(result, Ok(()));
}

