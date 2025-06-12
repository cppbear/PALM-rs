// Answer 0

#[derive(Debug)]
struct InvalidUri;

#[derive(Debug)]
struct Uri {
    value: String,
}

impl TryFrom<&str> for Uri {
    type Error = InvalidUri;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.is_empty() {
            Err(InvalidUri)
        } else {
            Ok(Uri {
                value: s.to_string(),
            })
        }
    }
}

fn from_str(s: &str) -> Result<Uri, InvalidUri> {
    TryFrom::try_from(s)
}

#[test]
fn test_from_str_valid_uri() {
    let result = from_str("http://example.com");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, "http://example.com");
}

#[test]
fn test_from_str_empty_uri() {
    let result = from_str("");
    assert!(result.is_err());
}

#[test]
fn test_from_str_non_empty_uri() {
    let result = from_str("https://example.com/resource");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, "https://example.com/resource");
}

