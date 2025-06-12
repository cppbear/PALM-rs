// Answer 0

#[test]
fn test_deserialize_bool_false() {
    struct TestVisitor;
    
    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        // Other required methods can be empty or unimplemented
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    struct TestDeserializer {
        key: &'static str,
    }

    impl TestDeserializer {
        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            if self.key == "true" {
                visitor.visit_bool(true)
            } else if self.key == "false" {
                visitor.visit_bool(false)
            } else {
                Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Str(&self.key),
                    &visitor,
                ))
            }
        }
    }

    let deserializer = TestDeserializer { key: "false" };
    let visitor = TestVisitor;
    
    let result = deserializer.deserialize_bool(visitor).unwrap();
    assert_eq!(result, false);
}

#[test]
fn test_deserialize_bool_invalid() {
    struct TestVisitor;
    
    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    struct TestDeserializer {
        key: &'static str,
    }

    impl TestDeserializer {
        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            if self.key == "true" {
                visitor.visit_bool(true)
            } else if self.key == "false" {
                visitor.visit_bool(false)
            } else {
                Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Str(&self.key),
                    &visitor,
                ))
            }
        }
    }

    let deserializer = TestDeserializer { key: "not_a_bool" };
    let visitor = TestVisitor;
    
    let result = deserializer.deserialize_bool(visitor);
    assert!(result.is_err());
}

