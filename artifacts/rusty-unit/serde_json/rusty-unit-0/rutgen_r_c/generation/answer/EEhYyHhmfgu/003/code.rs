// Answer 0

fn test_deserialize_bool_invalid_true() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(E::custom("should not visit bool"))
        }

        // Implement other required Visitor traits as no-op
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    let key = Cow::Borrowed("not_a_bool");
    let deserializer = MapKeyDeserializer { key };

    let result: Result<(), Error> = deserializer.deserialize_bool(MockVisitor);
    assert!(result.is_err());
}

fn test_deserialize_bool_invalid_false() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(E::custom("should not visit bool"))
        }

        // Implement other required Visitor traits as no-op
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    let key = Cow::Borrowed("also_not_a_bool");
    let deserializer = MapKeyDeserializer { key };

    let result: Result<(), Error> = deserializer.deserialize_bool(MockVisitor);
    assert!(result.is_err());
}

fn test_deserialize_bool_key_true() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_bool(self, value: bool) -> Result<Self::Value, serde::de::Error> {
            assert!(value);
            Ok(())
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    let key = Cow::Borrowed("true");
    let deserializer = MapKeyDeserializer { key };

    let result: Result<(), Error> = deserializer.deserialize_bool(MockVisitor);
    assert!(result.is_ok());
}

fn test_deserialize_bool_key_false() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_bool(self, value: bool) -> Result<Self::Value, serde::de::Error> {
            assert!(!value);
            Ok(())
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    let key = Cow::Borrowed("false");
    let deserializer = MapKeyDeserializer { key };

    let result: Result<(), Error> = deserializer.deserialize_bool(MockVisitor);
    assert!(result.is_ok());
}

