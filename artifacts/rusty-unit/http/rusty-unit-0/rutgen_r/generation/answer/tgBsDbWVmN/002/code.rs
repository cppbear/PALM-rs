// Answer 0

#[derive(Debug)]
struct StandardHeader {
    value: String,
}

impl StandardHeader {
    fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Debug)]
struct CustomHeader(String);

enum Repr {
    Standard(StandardHeader),
    Custom(CustomHeader),
}

struct Header {
    inner: Repr,
}

impl Header {
    pub fn as_str(&self) -> &str {
        match self.inner {
            Repr::Standard(ref v) => v.as_str(),
            Repr::Custom(ref v) => &v.0,
        }
    }
}

#[test]
fn test_as_str_with_standard_repr() {
    let header = Header {
        inner: Repr::Standard(StandardHeader {
            value: "content-type".to_string(),
        }),
    };
    assert_eq!(header.as_str(), "content-type");
}

#[test]
fn test_as_str_with_custom_repr() {
    let header = Header {
        inner: Repr::Custom(CustomHeader("accept-encoding".to_string())),
    };
    assert_eq!(header.as_str(), "accept-encoding");
}

#[test]
fn test_as_str_with_standard_repr_empty() {
    let header = Header {
        inner: Repr::Standard(StandardHeader {
            value: "".to_string(),
        }),
    };
    assert_eq!(header.as_str(), "");
}

