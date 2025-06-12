// Answer 0

#[test]
fn test_deserialize_bool_true() {
    struct MockVisitor {
        value: Option<bool>,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }

        // Add other required methods as no-op
        fn visit_i64(self, _: i64) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
        fn visit_u64(self, _: u64) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
        fn visit_f64(self, _: f64) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
        fn visit_str(self, _: &str) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
    }

    let mut deserializer = Deserializer::from_str("true");
    let result = deserializer.deserialize_bool(MockVisitor { value: None });
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_deserialize_bool_false() {
    struct MockVisitor {
        value: Option<bool>,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }

        // Add other required methods as no-op
        fn visit_i64(self, _: i64) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
        fn visit_u64(self, _: u64) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
        fn visit_f64(self, _: f64) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
        fn visit_str(self, _: &str) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
    }

    let mut deserializer = Deserializer::from_str("false");
    let result = deserializer.deserialize_bool(MockVisitor { value: None });
    assert_eq!(result.unwrap(), false);
}

#[test]
#[should_panic]
fn test_deserialize_bool_invalid() {
    struct MockVisitor {
        value: Option<bool>,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, _: bool) -> Result<Self::Value> {
            panic!("should not be called");
        }

        // Add other required methods as no-op
        fn visit_i64(self, _: i64) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
        fn visit_u64(self, _: u64) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
        fn visit_f64(self, _: f64) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
        fn visit_str(self, _: &str) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value> { Err(de::Error::custom("not a bool")) }
    }

    let mut deserializer = Deserializer::from_str("not_a_bool");
    let _ = deserializer.deserialize_bool(MockVisitor { value: None });
}

