// Answer 0

#[test]
fn test_deserialize_bool_true() {
    struct TestVisitor {
        value: Option<bool>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        // Other visit methods can be ignored for this test
        forward_to_deserialize_any! { i64 u64 f64 str bytes seq map }
    }

    let deserializer = MapKeyDeserializer {
        key: Cow::Borrowed("true"),
    };
    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_bool(visitor).unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_deserialize_bool_false() {
    struct TestVisitor {
        value: Option<bool>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        forward_to_deserialize_any! { i64 u64 f64 str bytes seq map }
    }

    let deserializer = MapKeyDeserializer {
        key: Cow::Borrowed("false"),
    };
    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_bool(visitor).unwrap();
    assert_eq!(result, false);
}

#[test]
#[should_panic(expected = "invalid type")]
fn test_deserialize_bool_invalid() {
    struct TestVisitor {
        value: Option<bool>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E> {
            // This should not be called for invalid keys
            panic!("visit_bool should not be called for invalid values");
        }

        // Other visit methods can be ignored for this test
        forward_to_deserialize_any! { i64 u64 f64 str bytes seq map }
    }

    let deserializer = MapKeyDeserializer {
        key: Cow::Borrowed("not_a_bool"),
    };
    let visitor = TestVisitor { value: None };

    let _result = deserializer.deserialize_bool(visitor).unwrap_err();
}

