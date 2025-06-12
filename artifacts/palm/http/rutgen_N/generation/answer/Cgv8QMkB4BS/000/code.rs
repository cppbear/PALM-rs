// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct UriScheme;

    impl TryFrom<&str> for UriScheme {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.is_empty() {
                Err("Invalid URI scheme: empty string")
            } else {
                Ok(UriScheme)
            }
        }
    }

    #[test]
    fn test_from_str_valid_scheme() {
        let result = UriScheme::try_from("http");
        assert!(result.is_ok());
    }

    #[test]
    fn test_from_str_invalid_scheme_empty() {
        let result = UriScheme::try_from("");
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid URI scheme: empty string"));
    }
}

