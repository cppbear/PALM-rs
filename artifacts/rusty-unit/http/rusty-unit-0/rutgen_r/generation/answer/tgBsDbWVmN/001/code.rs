// Answer 0

#[derive(Debug)]
struct Custom(String);

#[derive(Debug)]
enum Repr {
    Standard(Custom),
    Custom(Custom),
}

struct Header {
    inner: Repr,
}

impl Header {
    pub fn as_str(&self) -> &str {
        match self.inner {
            Repr::Standard(ref v) => v.0.as_str(),
            Repr::Custom(ref v) => &v.0,
        }
    }
}

#[test]
fn test_as_str_custom_repr() {
    let custom_value = Custom("custom_header_value".to_string());
    let header = Header {
        inner: Repr::Custom(custom_value),
    };
    assert_eq!(header.as_str(), "custom_header_value");
}

#[test]
fn test_as_str_standard_repr() {
    let standard_value = Custom("standard_header_value".to_string());
    let header = Header {
        inner: Repr::Standard(standard_value),
    };
    assert_eq!(header.as_str(), "standard_header_value");
}

