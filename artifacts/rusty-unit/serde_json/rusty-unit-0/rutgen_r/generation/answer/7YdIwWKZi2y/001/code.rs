// Answer 0

#[test]
fn test_deserialize_str_valid() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_string())
        }
    }

    let input = serde_json::Value::String("test".to_string());
    let result = serde_json::from_value(input).and_then(|v: serde_json::Value| {
        v.as_str()
            .map_or(Err(serde::de::Error::custom("Expected a string")), |s| {
                TestVisitor.visit_str(s)
            })
    });

    assert_eq!(result.unwrap(), "test".to_string());
}

#[test]
fn test_deserialize_str_empty() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_string())
        }
    }

    let input = serde_json::Value::String("".to_string());
    let result = serde_json::from_value(input).and_then(|v: serde_json::Value| {
        v.as_str()
            .map_or(Err(serde::de::Error::custom("Expected a string")), |s| {
                TestVisitor.visit_str(s)
            })
    });

    assert_eq!(result.unwrap(), "".to_string());
}

#[should_panic(expected = "Expected a string")]
#[test]
fn test_deserialize_str_invalid() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(serde::de::Error::custom("visit_str not called"))
        }
    }

    let input = serde_json::Value::Number(serde_json::Number::from(1));
    let _result = serde_json::from_value(input).and_then(|v: serde_json::Value| {
        v.as_str()
            .map_or(Err(serde::de::Error::custom("Expected a string")), |s| {
                TestVisitor.visit_str(s)
            })
    });
}

