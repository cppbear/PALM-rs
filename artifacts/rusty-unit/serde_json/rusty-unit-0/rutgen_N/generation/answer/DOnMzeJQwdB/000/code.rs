// Answer 0

#[test]
fn test_deserialize_bool_success() {
    struct BoolVisitor;
    
    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        // Implementing the other required methods as no-op
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
        fn visit_unit<E>(self) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
        fn visit_seq<E>(self, _: Self::Value) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
        fn visit_map<E>(self, _: Self::Value) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
    }

    let value = Value::Bool(true);
    let result: Result<bool, Error> = value.deserialize_bool(BoolVisitor);
    assert_eq!(result, Ok(true));
}

#[test]
fn test_deserialize_bool_failure() {
    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        // Implementing the other required methods as no-op
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
        fn visit_unit<E>(self) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
        fn visit_seq<E>(self, _: Self::Value) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
        fn visit_map<E>(self, _: Self::Value) -> Result<Self::Value, E> { Err(E::custom("expected bool")) }
    }

    let value = Value::String("not_a_bool".to_string());
    let result: Result<bool, Error> = value.deserialize_bool(BoolVisitor);
    assert!(result.is_err());
}

