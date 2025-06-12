// Answer 0

#[cfg(test)]
mod tests {
    use serde_json::de::Deserializer;
    use std::result;

    struct MyDeserializer;

    impl MyDeserializer {
        fn from_str(s: &str) -> result::Result<i64, serde_json::Error> {
            Deserializer::from_str(s)
                .parse_any_signed_number()
                .map(Into::into)
        }
    }

    #[test]
    fn test_from_str_valid_number() {
        let input = "42";
        let result = MyDeserializer::from_str(input);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_from_str_negative_number() {
        let input = "-10";
        let result = MyDeserializer::from_str(input);
        assert_eq!(result.unwrap(), -10);
    }

    #[test]
    fn test_from_str_invalid_number() {
        let input = "not_a_number";
        let result = MyDeserializer::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_empty() {
        let input = "";
        let result = MyDeserializer::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_boundary_case() {
        let input = "0";
        let result = MyDeserializer::from_str(input);
        assert_eq!(result.unwrap(), 0);
    }
}

