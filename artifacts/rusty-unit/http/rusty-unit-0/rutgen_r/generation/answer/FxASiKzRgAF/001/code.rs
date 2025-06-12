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
fn test_as_str() {
    let header1 = HeaderName {
        name: String::from("Content-Type"),
    };
    assert_eq!(header1.as_str(), "Content-Type");

    let header2 = HeaderName {
        name: String::from("Authorization"),
    };
    assert_eq!(header2.as_str(), "Authorization");

    let header3 = HeaderName {
        name: String::from("User-Agent"),
    };
    assert_eq!(header3.as_str(), "User-Agent");

    let header4 = HeaderName {
        name: String::from("Set-Cookie"),
    };
    assert_eq!(header4.as_str(), "Set-Cookie");
}

#[test]
fn test_as_str_empty() {
    let header = HeaderName {
        name: String::from(""),
    };
    assert_eq!(header.as_str(), "");
}

#[should_panic]
#[test]
fn test_as_str_uninitialized() {
    let header: HeaderName = HeaderName {
        name: String::from("Invalid-Header"),
    };
    let _panic_header = HeaderName {
        name: String::new(),  // This causes a panic when trying to use an uninitialized state
    };
    assert_eq!(header.as_str(), "Invalid-Header");
}

