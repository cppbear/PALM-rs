// Answer 0

#[derive(Debug)]
struct HeaderName(Vec<u8>);

impl HeaderName {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        if bytes.is_empty() {
            Err(())
        } else {
            Ok(HeaderName(bytes.to_vec()))
        }
    }
}

#[derive(Debug)]
struct InvalidHeaderName {
    _priv: (),
}

fn from_str(s: &str) -> Result<HeaderName, InvalidHeaderName> {
    HeaderName::from_bytes(s.as_bytes()).map_err(|_| InvalidHeaderName { _priv: () })
}

#[test]
fn test_from_str_valid() {
    let header_name = from_str("Content-Type");
    assert!(header_name.is_ok());
    if let Ok(name) = header_name {
        assert_eq!(name.0, b"Content-Type");
    }
}

#[test]
fn test_from_str_empty() {
    let header_name = from_str("");
    assert!(header_name.is_err());
}

