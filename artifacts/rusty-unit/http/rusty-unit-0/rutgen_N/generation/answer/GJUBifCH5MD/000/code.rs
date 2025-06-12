// Answer 0

#[derive(Debug)]
struct Inner {
    data: String,
}

#[derive(Debug)]
struct HeaderName {
    inner: Inner,
}

impl HeaderName {
    pub fn new(data: String) -> Self {
        HeaderName {
            inner: Inner { data },
        }
    }

    pub(super) fn into_bytes(self) -> Bytes {
        self.inner.data.into()
    }
}

#[derive(Debug)]
struct Bytes(Vec<u8>);

impl From<String> for Bytes {
    fn from(s: String) -> Self {
        Bytes(s.into_bytes())
    }
}

#[test]
fn test_into_bytes() {
    let header_name = HeaderName::new("example".to_string());
    let bytes = header_name.into_bytes();
    assert_eq!(bytes.0, b"example".to_vec());
}

#[test]
fn test_into_bytes_empty() {
    let header_name = HeaderName::new("".to_string());
    let bytes = header_name.into_bytes();
    assert_eq!(bytes.0, b"".to_vec());
}

#[test]
fn test_into_bytes_special_characters() {
    let header_name = HeaderName::new("header-name!@#".to_string());
    let bytes = header_name.into_bytes();
    assert_eq!(bytes.0, b"header-name!@#".to_vec());
}

#[test]
fn test_into_bytes_whitespace() {
    let header_name = HeaderName::new("  ".to_string());
    let bytes = header_name.into_bytes();
    assert_eq!(bytes.0, b"  ".to_vec());
}

