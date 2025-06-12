// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use http::header::{HeaderValue, InvalidHeaderValue};

    #[test]
    fn test_from_bytes_valid() {
        let val = HeaderValue::from_bytes(b"hello\xfa").unwrap();
        assert_eq!(val.as_bytes(), b"hello\xfa");
    }

    #[test]
    fn test_from_bytes_invalid_control_character() {
        let val = HeaderValue::from_bytes(b"\n");
        assert!(val.is_err());
    }

    #[test]
    fn test_from_bytes_invalid_lower_bound() {
        let val = HeaderValue::from_bytes(b"\x1F");
        assert!(val.is_err());
    }

    #[test]
    fn test_from_bytes_invalid_upper_bound() {
        let val = HeaderValue::from_bytes(b"\x80");
        assert!(val.is_err());
    }

    #[test]
    fn test_from_bytes_empty() {
        let val = HeaderValue::from_bytes(b"");
        assert!(val.is_ok()); // Assuming empty byte string is valid
    }

    #[test]
    fn test_from_bytes_valid_boundary() {
        let val = HeaderValue::from_bytes(b"\x20"); // ASCII space
        assert!(val.is_ok());
        assert_eq!(val.unwrap().as_bytes(), b"\x20");

        let val = HeaderValue::from_bytes(b"\x7E"); // ASCII tilde
        assert!(val.is_ok());
        assert_eq!(val.unwrap().as_bytes(), b"\x7E");
    }
}

