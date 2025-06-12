// Answer 0

#[test]
fn test_deserialize_char_valid_input() {
    use serde::de::{self, Visitor};
    use serde_json::de::from_str;

    struct CharVisitor {
        value: Option<char>,
    }

    impl<'de> Visitor<'de> for CharVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a single character")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value.chars().next().ok_or_else(|| E::custom("expected a character"))
        }
    }

    let input = "\"a\"";
    let visitor = CharVisitor { value: None };
    let result: Result<char, _> = from_str(input).and_then(|s: String| visitor.visit_str(&s));

    assert_eq!(result.unwrap(), 'a');
}

#[test]
#[should_panic(expected = "expected a character")]
fn test_deserialize_char_empty_string() {
    use serde::de::{self, Visitor};
    use serde_json::de::from_str;

    struct CharVisitor {
        value: Option<char>,
    }

    impl<'de> Visitor<'de> for CharVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a single character")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value.chars().next().ok_or_else(|| E::custom("expected a character"))
        }
    }

    let input = "\"\""; // empty string
    let visitor = CharVisitor { value: None };
    let _: Result<char, _> = from_str(input).and_then(|s: String| visitor.visit_str(&s));
}

#[test]
fn test_deserialize_char_multiple_characters() {
    use serde::de::{self, Visitor};
    use serde_json::de::from_str;

    struct CharVisitor {
        value: Option<char>,
    }

    impl<'de> Visitor<'de> for CharVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a single character")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value.chars().next().ok_or_else(|| E::custom("expected a character"))
        }
    }

    let input = "\"abc\""; // multiple characters
    let visitor = CharVisitor { value: None };
    let result: Result<char, _> = from_str(input).and_then(|s: String| visitor.visit_str(&s));

    assert_eq!(result.unwrap(), 'a'); // Expecting to get the first character
}

