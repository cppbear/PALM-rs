// Answer 0

#[test]
fn test_deserialize_any_null() {
    struct Visitor;
    
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }

        // Required methods for Visitor trait must be implemented
        fn visit_bool(self, _v: bool) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_str(self, _v: &str) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_borrowed_str(self, _v: &'de str) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_seq(self, _seq: SeqAccess<'de>) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_map(self, _map: MapAccess<'de>) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
    }

    let result = deserialize_any(Visitor);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_deserialize_any_true() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = bool;

        fn visit_bool(self, v: bool) -> Result<Self::Value> {
            Ok(v)
        }

        // Required methods for Visitor trait must be implemented
        fn visit_unit(self) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_str(self, _v: &str) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_borrowed_str(self, _v: &'de str) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_seq(self, _seq: SeqAccess<'de>) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_map(self, _map: MapAccess<'de>) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
    }

    let result = deserialize_any(Visitor);
    assert_eq!(result, Ok(true));
}

#[test]
fn test_deserialize_any_false() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = bool;

        fn visit_bool(self, v: bool) -> Result<Self::Value> {
            Ok(v)
        }

        // Required methods for Visitor trait must be implemented
        fn visit_unit(self) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_str(self, _v: &str) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_borrowed_str(self, _v: &'de str) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_seq(self, _seq: SeqAccess<'de>) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_map(self, _map: MapAccess<'de>) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
    }

    let result = deserialize_any(Visitor);
    assert_eq!(result, Ok(false));
}

#[test]
fn test_deserialize_any_string() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = String;

        fn visit_str(self, v: &str) -> Result<Self::Value> {
            Ok(v.to_string())
        }

        fn visit_borrowed_str(self, v: &'de str) -> Result<Self::Value> {
            Ok(v.to_string())
        }

        // Required methods for Visitor trait must be implemented
        fn visit_unit(self) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_bool(self, _v: bool) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_seq(self, _seq: SeqAccess<'de>) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_map(self, _map: MapAccess<'de>) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
    }

    let result = deserialize_any(Visitor);
    assert_eq!(result, Ok("test".to_string()));
}

#[test]
fn test_deserialize_any_negative_number() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = i32;

        fn visit_i32(self, v: i32) -> Result<Self::Value> {
            Ok(v)
        }

        // Required methods for Visitor trait must be implemented
        fn visit_unit(self) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_bool(self, _v: bool) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_str(self, _v: &str) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_seq(self, _seq: SeqAccess<'de>) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_map(self, _map: MapAccess<'de>) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
    }

    let result = deserialize_any(Visitor);
    assert_eq!(result, Ok(-10));
}

#[test]
fn test_deserialize_any_array() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Vec<i32>;

        fn visit_seq(self, _seq: SeqAccess<'de>) -> Result<Self::Value> {
            Ok(vec![1, 2, 3])
        }

        // Required methods for Visitor trait must be implemented
        fn visit_unit(self) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_bool(self, _v: bool) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_str(self, _v: &str) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_map(self, _map: MapAccess<'de>) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
    }

    let result = deserialize_any(Visitor);
    assert_eq!(result, Ok(vec![1, 2, 3]));
}

#[test]
fn test_deserialize_any_object() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = std::collections::HashMap<String, String>;

        fn visit_map(self, _map: MapAccess<'de>) -> Result<Self::Value> {
            let mut map = std::collections::HashMap::new();
            map.insert("key".to_string(), "value".to_string());
            Ok(map)
        }

        // Required methods for Visitor trait must be implemented
        fn visit_unit(self) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_bool(self, _v: bool) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_str(self, _v: &str) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
        fn visit_seq(self, _seq: SeqAccess<'de>) -> Result<Self::Value> { Err(ErrorCode::ExpectedSomeValue.into()) }
    }

    let result = deserialize_any(Visitor);
    assert_eq!(result, Ok(std::collections::HashMap::from([("key".to_string(), "value".to_string())])));
}

