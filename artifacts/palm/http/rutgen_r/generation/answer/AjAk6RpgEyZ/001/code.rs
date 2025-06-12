// Answer 0

#[derive(Debug)]
struct Authority {
    data: String,
}

impl Authority {
    pub fn new(data: String) -> Self {
        Authority { data }
    }

    pub fn as_str(&self) -> &str {
        &self.data[..]
    }
}

#[test]
fn test_as_str_non_empty() {
    let authority = Authority::new("example.com".to_string());
    assert_eq!(authority.as_str(), "example.com");
}

#[test]
fn test_as_str_single_character() {
    let authority = Authority::new("a".to_string());
    assert_eq!(authority.as_str(), "a");
}

#[test]
fn test_as_str_whitespace() {
    let authority = Authority::new("    ".to_string());
    assert_eq!(authority.as_str(), "    ");
}

#[test]
#[should_panic]
fn test_as_str_empty() {
    let authority = Authority::new("".to_string());
    authority.as_str();
}

