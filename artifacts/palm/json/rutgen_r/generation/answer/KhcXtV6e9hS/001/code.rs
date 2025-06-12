// Answer 0

#[test]
fn test_deserialize_char_valid_string() {
    struct TestVisitor;
    
    impl serde::de::Visitor<'_> for TestVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a single character")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if value.len() == 1 {
                Ok(value.chars().next().unwrap())
            } else {
                Err(E::invalid_value(serde::de::Unexpected::Str(value), &self))
            }
        }
    }

    let input = "a";
    let result: Result<char, serde_json::Error> = serde_json::Deserializer::from_str(input)
        .deserialize_char(TestVisitor);
    assert_eq!(result, Ok('a'));
}

#[test]
fn test_deserialize_char_empty_string() {
    struct TestVisitor;
    
    impl serde::de::Visitor<'_> for TestVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a single character")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if value.len() == 1 {
                Ok(value.chars().next().unwrap())
            } else {
                Err(E::invalid_value(serde::de::Unexpected::Str(value), &self))
            }
        }
    }

    let input = "";
    let result: Result<char, serde_json::Error> = serde_json::Deserializer::from_str(input)
        .deserialize_char(TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_char_multiple_characters() {
    struct TestVisitor;
    
    impl serde::de::Visitor<'_> for TestVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a single character")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if value.len() == 1 {
                Ok(value.chars().next().unwrap())
            } else {
                Err(E::invalid_value(serde::de::Unexpected::Str(value), &self))
            }
        }
    }

    let input = "abc";
    let result: Result<char, serde_json::Error> = serde_json::Deserializer::from_str(input)
        .deserialize_char(TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_char_non_ascii() {
    struct TestVisitor;
    
    impl serde::de::Visitor<'_> for TestVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a single character")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if value.len() == 1 {
                Ok(value.chars().next().unwrap())
            } else {
                Err(E::invalid_value(serde::de::Unexpected::Str(value), &self))
            }
        }
    }

    let input = "ñ";
    let result: Result<char, serde_json::Error> = serde_json::Deserializer::from_str(input)
        .deserialize_char(TestVisitor);
    assert_eq!(result, Ok('ñ'));
}

