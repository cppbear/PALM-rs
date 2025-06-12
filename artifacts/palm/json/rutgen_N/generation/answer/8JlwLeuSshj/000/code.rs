// Answer 0

#[test]
fn test_deserialize_char_success() {
    struct TestVisitor {
        value: Option<char>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a single character")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value.len() == 1 {
                Ok(value.chars().next().unwrap())
            } else {
                Err(E::custom("expected a single character"))
            }
        }
    }

    struct TestDeserializer;

    impl TestDeserializer {
        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            visitor.visit_str("x")
        }
    }

    let deserializer = TestDeserializer;
    let visitor = TestVisitor { value: None };
    let result: Result<char> = deserializer.deserialize_char(visitor);
    assert_eq!(result.unwrap(), 'x');
}

#[test]
#[should_panic(expected = "expected a single character")]
fn test_deserialize_char_failure() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a single character")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value.len() == 1 {
                Ok(value.chars().next().unwrap())
            } else {
                Err(E::custom("expected a single character"))
            }
        }
    }

    struct TestDeserializer;

    impl TestDeserializer {
        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            visitor.visit_str("longer string")
        }
    }

    let deserializer = TestDeserializer;
    let visitor = TestVisitor;
    let _ = deserializer.deserialize_char(visitor).unwrap();
}

