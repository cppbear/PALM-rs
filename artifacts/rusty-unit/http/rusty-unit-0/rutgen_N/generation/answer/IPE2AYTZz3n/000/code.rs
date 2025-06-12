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

struct MyHeader {
    header_name: HeaderName,
}

impl MyHeader {
    fn as_str(&self) -> &str {
        HeaderName::as_str(&self.header_name)
    }
}

#[test]
fn test_my_header_as_str() {
    let header_name = HeaderName {
        name: String::from("Content-Type"),
    };
    let my_header = MyHeader { header_name };
    
    assert_eq!(my_header.as_str(), "Content-Type");
}

#[test]
fn test_my_header_as_str_empty() {
    let header_name = HeaderName {
        name: String::from(""),
    };
    let my_header = MyHeader { header_name };
    
    assert_eq!(my_header.as_str(), "");
}

