// Answer 0

#[test]
fn test_deserialize_char_success() {
    struct MockVisitor {
        value: char,
    }

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a character")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            value.chars().next().ok_or_else(|| E::custom("expected a single character"))
        }
    }

    let result: Result<char, serde_json::Error> = serde_json::from_str("\"a\"").and_then(|s: String| {
        let visitor = MockVisitor { value: 'a' };
        visitor.visit_str(&s)
    });

    assert_eq!(result, Ok('a'));
}

#[test]
fn test_deserialize_char_empty_string() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a character")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            value.chars().next().ok_or_else(|| E::custom("expected a single character"))
        }
    }

    let result: Result<char, serde_json::Error> = serde_json::from_str("\"\"").and_then(|s: String| {
        let visitor = MockVisitor;
        visitor.visit_str(&s)
    });

    assert!(result.is_err());
}

