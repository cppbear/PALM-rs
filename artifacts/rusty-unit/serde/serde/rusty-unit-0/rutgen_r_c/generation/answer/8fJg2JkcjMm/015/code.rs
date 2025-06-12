// Answer 0

#[test]
fn test_deserialize_any_i32_success() {
    use crate::de::Visitor;

    struct TestVisitor {
        value: Option<i32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(E::custom("Expected i32 but found unit"))
        }

        // Implement other trait methods as no-ops to satisfy the trait
        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found bool")) }
        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found u8")) }
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found u16")) }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found u32")) }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found u64")) }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found i8")) }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found i16")) }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found i64")) }
        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found f32")) }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found f64")) }
        fn visit_char<E>(self, _: char) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found char")) }
        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found str")) }
        fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found borrowed str")) }
        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found bytes")) }
        fn visit_borrowed_bytes<E>(self, _: &'de [u8]) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found borrowed bytes")) }
        fn visit_unit_struct<E>(self, _: &'static str) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found unit struct")) }
        fn visit_newtype_struct<E>(self, _: &'static str) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found newtype struct")) }
        fn visit_seq<E>(self) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found sequence")) }
        fn visit_map<E>(self) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found map")) }
        fn visit_option<E>(self) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found option")) }
        fn visit_none<E>(self) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found none")) }
        fn visit_some<V: Visitor<'de>>(self, _: V) -> Result<Self::Value, E> { Err(E::custom("Expected i32 but found some")) }
    }

    let content = Content::I32(42);
    let deserializer = ContentRefDeserializer::new(&content);
    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_any(visitor);
    assert_eq!(result.unwrap(), 42);
}

#[test]
#[should_panic]
fn test_deserialize_any_invalid_type() {
    use crate::de::Visitor;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
            unreachable!()
        }

        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> { unreachable!() }
        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> { unreachable!() }
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> { unreachable!() }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> { unreachable!() }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { unreachable!() }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> { unreachable!() }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> { unreachable!() }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { unreachable!() }
        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> { unreachable!() }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> { unreachable!() }
        fn visit_char<E>(self, _: char) -> Result<Self::Value, E> { unreachable!() }
        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> { unreachable!() }
        fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E> { unreachable!() }
        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> { unreachable!() }
        fn visit_borrowed_bytes<E>(self, _: &'de [u8]) -> Result<Self::Value, E> { unreachable!() }
        fn visit_unit_struct<E>(self, _: &'static str) -> Result<Self::Value, E> { unreachable!() }
        fn visit_newtype_struct<E>(self, _: &'static str) -> Result<Self::Value, E> { unreachable!() }
        fn visit_seq<E>(self) -> Result<Self::Value, E> { unreachable!() }
        fn visit_map<E>(self) -> Result<Self::Value, E> { unreachable!() }
        fn visit_option<E>(self) -> Result<Self::Value, E> { unreachable!() }
        fn visit_none<E>(self) -> Result<Self::Value, E> { unreachable!() }
        fn visit_some<V: Visitor<'de>>(self, _: V) -> Result<Self::Value, E> { unreachable!() }
    }

    let content = Content::String("not an i32".into());
    let deserializer = ContentRefDeserializer::new(&content);
    let visitor = TestVisitor;

    deserializer.deserialize_any(visitor).unwrap();
}

