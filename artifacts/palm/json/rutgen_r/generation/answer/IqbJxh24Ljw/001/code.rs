// Answer 0

#[test]
fn test_deserialize_string_success() {
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
            Ok(v.to_owned())
        }
    }

    let input = r#""test string""#;
    let deserialized: Result<String, serde_json::Error> = serde_json::from_str(input).map(|v: String| TestVisitor.visit_str(&v));
    assert_eq!(deserialized.unwrap(), "test string");
}

#[test]
fn test_deserialize_string_empty() {
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
            Ok(v.to_owned())
        }
    }

    let input = r#""""#; 
    let deserialized: Result<String, serde_json::Error> = serde_json::from_str(input).map(|v: String| TestVisitor.visit_str(&v));
    assert_eq!(deserialized.unwrap(), "");
}

#[test]
#[should_panic(expected = "EOF while parsing a string")]
fn test_deserialize_string_invalid() {
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
            Ok(v.to_owned())
        }
    }

    let input = r#""test string"#; 
    let _: Result<String, serde_json::Error> = serde_json::from_str(input).map(|v| TestVisitor.visit_str(&v)).unwrap();
}

