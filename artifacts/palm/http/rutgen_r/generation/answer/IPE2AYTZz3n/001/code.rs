// Answer 0

#[derive(Debug)]
struct HeaderName {
    name: String,
}

impl HeaderName {
    fn as_str(&self) -> &str {
        &self.name
    }
}

#[test]
fn test_as_str_valid() {
    let header_name = HeaderName { name: String::from("Content-Type") };
    assert_eq!(header_name.as_str(), "Content-Type");
}

#[test]
fn test_as_str_empty() {
    let header_name = HeaderName { name: String::from("") };
    assert_eq!(header_name.as_str(), "");
}

#[test]
fn test_as_str_whitespace() {
    let header_name = HeaderName { name: String::from("   ") };
    assert_eq!(header_name.as_str(), "   ");
}

