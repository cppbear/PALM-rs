// Answer 0

#[test]
fn test_to_str_with_invisible_ascii_characters() {
    struct HeaderValue {
        bytes: Vec<u8>,
    }

    impl HeaderValue {
        fn from_bytes(bytes: Vec<u8>) -> Self {
            HeaderValue { bytes }
        }
        
        fn as_ref(&self) -> &[u8] {
            &self.bytes
        }
        
        fn to_str(&self) -> Result<&str, ToStrError> {
            let bytes = self.as_ref();

            for &b in bytes {
                if !is_visible_ascii(b) {
                    return Err(ToStrError { _priv: () });
                }
            }

            unsafe { Ok(str::from_utf8_unchecked(bytes)) }
        }
    }

    #[derive(Debug)]
    struct ToStrError {
        _priv: (),
    }

    fn is_visible_ascii(b: u8) -> bool {
        b >= 32 && b <= 126
    }

    let val = HeaderValue::from_bytes(vec![0, 1, 2, 3, 4]); // All non-visible ASCII characters
    assert!(val.to_str().is_err());
}

#[test]
fn test_to_str_with_mixed_visible_and_invisible_ascii_characters() {
    struct HeaderValue {
        bytes: Vec<u8>,
    }

    impl HeaderValue {
        fn from_bytes(bytes: Vec<u8>) -> Self {
            HeaderValue { bytes }
        }
        
        fn as_ref(&self) -> &[u8] {
            &self.bytes
        }
        
        fn to_str(&self) -> Result<&str, ToStrError> {
            let bytes = self.as_ref();

            for &b in bytes {
                if !is_visible_ascii(b) {
                    return Err(ToStrError { _priv: () });
                }
            }

            unsafe { Ok(str::from_utf8_unchecked(bytes)) }
        }
    }

    #[derive(Debug)]
    struct ToStrError {
        _priv: (),
    }

    fn is_visible_ascii(b: u8) -> bool {
        b >= 32 && b <= 126
    }

    let val = HeaderValue::from_bytes(vec![65, 66, 67, 0, 69]); // Contains a non-visible ASCII character (0)
    assert!(val.to_str().is_err());
}

