// Answer 0

#[test]
fn test_deserialize_bool_true() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        // Other Visitor trait methods can be left unimplemented for this test
        // since they won't be called.
        forward_to_deserialize_any!();
    }

    let deserializer = MapKeyDeserializer {
        key: Cow::Borrowed("true"),
    };
    let result: Result<bool, Error> = deserializer.deserialize_bool(VisitorImpl);
    assert_eq!(result, Ok(true));
}

#[test]
fn test_deserialize_bool_false() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        // Other Visitor trait methods can be left unimplemented for this test
        forward_to_deserialize_any!();
    }

    let deserializer = MapKeyDeserializer {
        key: Cow::Borrowed("false"),
    };
    let result: Result<bool, Error> = deserializer.deserialize_bool(VisitorImpl);
    assert_eq!(result, Ok(false));
}

#[test]
fn test_deserialize_bool_invalid() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        // Other Visitor trait methods can be left unimplemented for this test
        forward_to_deserialize_any!();
    }

    let deserializer = MapKeyDeserializer {
        key: Cow::Borrowed("not_a_bool"),
    };
    let result: Result<bool, Error> = deserializer.deserialize_bool(VisitorImpl);
    assert!(result.is_err());
}

