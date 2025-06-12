// Answer 0

#[derive(Debug)]
struct StandardHeader {
    inner: Repr,
}

#[derive(Debug)]
enum Repr {
    Standard(String),
    Custom(CustomHeader),
}

#[derive(Debug)]
struct CustomHeader(String);

impl StandardHeader {
    pub fn as_str(&self) -> &str {
        match &self.inner {
            Repr::Standard(v) => v.as_str(),
            Repr::Custom(ref v) => &v.0,
        }
    }
}

#[test]
fn test_standard_header_as_str() {
    let header = StandardHeader {
        inner: Repr::Standard(String::from("content-type")),
    };
    assert_eq!(header.as_str(), "content-type");
}

#[test]
fn test_custom_header_as_str() {
    let header = StandardHeader {
        inner: Repr::Custom(CustomHeader(String::from("x-custom-header"))),
    };
    assert_eq!(header.as_str(), "x-custom-header");
}

#[test]
fn test_lower_case_conversion() {
    let header = StandardHeader {
        inner: Repr::Standard(String::from("Content-Type")),
    };
    assert_eq!(header.as_str().to_lowercase(), "content-type");
}

#[test]
fn test_custom_lower_case_conversion() {
    let header = StandardHeader {
        inner: Repr::Custom(CustomHeader(String::from("X-Custom-Header"))),
    };
    assert_eq!(header.as_str().to_lowercase(), "x-custom-header");
}

