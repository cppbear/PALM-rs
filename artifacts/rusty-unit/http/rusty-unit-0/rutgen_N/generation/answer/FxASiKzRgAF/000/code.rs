// Answer 0

#[derive(Debug)]
struct HeaderName;

impl HeaderName {
    fn as_str(&self) -> &str {
        "Header-Example"
    }
}

#[derive(Debug)]
struct MyHeader {
    name: HeaderName,
}

impl MyHeader {
    fn as_str(&self) -> &str {
        self.name.as_str()
    }
}

#[test]
fn test_my_header_as_str() {
    let header_name = HeaderName;
    let my_header = MyHeader { name: header_name };
    
    assert_eq!(my_header.as_str(), "Header-Example");
}

