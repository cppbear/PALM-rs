// Answer 0

#[test]
fn test_deserialize_string() {
    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }
    }

    struct TestDeserializer;

    impl<'de> serde::de::Deserializer<'de> for TestDeserializer {
        type Error = serde::de::value::Error;

        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            visitor.visit_str("test string")
        }

        // Unused methods for this test:
        fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> { unimplemented!() }
        // ... other methods unimplemented for brevity
    }

    let deserializer = TestDeserializer;
    let visitor = TestVisitor { value: None };
    let result: Result<String, _> = deserializer.deserialize_string(visitor);
    assert_eq!(result.unwrap(), "test string");
}

#[test]
fn test_deserialize_string_fail() {
    struct FailVisitor;

    impl<'de> serde::de::Visitor<'de> for FailVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(E::custom("failed testing"))
        }
    }

    struct TestFailDeserializer;

    impl<'de> serde::de::Deserializer<'de> for TestFailDeserializer {
        type Error = serde::de::value::Error;

        fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            unimplemented!()
        }

        // Unused methods for this test:
        fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> { unimplemented!() }
    }

    let deserializer = TestFailDeserializer;
    let visitor = FailVisitor;
    let result: Result<String, _> = deserializer.deserialize_string(visitor);
    assert!(result.is_err());
}

