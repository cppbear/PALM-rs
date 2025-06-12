// Answer 0

#[derive(Debug, PartialEq)]
struct StatusCode;

impl StatusCode {
    fn from_bytes(t: &[u8]) -> Result<Self, String> {
        if t.is_empty() {
            Err("Empty byte array".to_string())
        } else {
            Ok(StatusCode)
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for StatusCode {
    type Error = String;

    fn try_from(t: &'a [u8]) -> Result<Self, Self::Error> {
        StatusCode::from_bytes(t)
    }
}

#[test]
fn test_try_from_valid_bytes() {
    let bytes: &[u8] = b"OK";
    assert_eq!(StatusCode::try_from(bytes).is_ok(), true);
}

#[test]
fn test_try_from_empty_bytes() {
    let bytes: &[u8] = b"";
    assert_eq!(StatusCode::try_from(bytes).is_err(), true);
}

#[test]
fn test_try_from_non_empty_bytes() {
    let bytes: &[u8] = b"200";
    assert_eq!(StatusCode::try_from(bytes).is_ok(), true);
}

