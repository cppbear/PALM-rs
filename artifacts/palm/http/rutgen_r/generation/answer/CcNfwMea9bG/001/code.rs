// Answer 0

#[test]
fn test_to_str_with_visible_ascii() {
    struct HeaderValue {
        bytes: &'static [u8],
    }

    impl HeaderValue {
        fn from_static(s: &'static str) -> Self {
            HeaderValue {
                bytes: s.as_bytes(),
            }
        }

        fn as_ref(&self) -> &[u8] {
            self.bytes
        }

        fn to_str(&self) -> Result<&str, ()> {
            let bytes = self.as_ref();

            for &b in bytes {
                if !is_visible_ascii(b) {
                    return Err(());
                }
            }

            unsafe { Ok(std::str::from_utf8_unchecked(bytes)) }
        }
    }

    fn is_visible_ascii(b: u8) -> bool {
        (32..=126).contains(&b)
    }

    let val = HeaderValue::from_static("hello");
    assert_eq!(val.to_str().unwrap(), "hello");
}

#[test]
fn test_to_str_with_non_visible_ascii() {
    struct HeaderValue {
        bytes: &'static [u8],
    }

    impl HeaderValue {
        fn from_static(s: &'static str) -> Self {
            HeaderValue {
                bytes: s.as_bytes(),
            }
        }

        fn as_ref(&self) -> &[u8] {
            self.bytes
        }

        fn to_str(&self) -> Result<&str, ()> {
            let bytes = self.as_ref();

            for &b in bytes {
                if !is_visible_ascii(b) {
                    return Err(());
                }
            }

            unsafe { Ok(std::str::from_utf8_unchecked(bytes)) }
        }
    }

    fn is_visible_ascii(b: u8) -> bool {
        (32..=126).contains(&b)
    }

    let val = HeaderValue::from_static("hello\nworld");
    assert!(val.to_str().is_err());
}

#[test]
fn test_to_str_with_empty_string() {
    struct HeaderValue {
        bytes: &'static [u8],
    }

    impl HeaderValue {
        fn from_static(s: &'static str) -> Self {
            HeaderValue {
                bytes: s.as_bytes(),
            }
        }

        fn as_ref(&self) -> &[u8] {
            self.bytes
        }

        fn to_str(&self) -> Result<&str, ()> {
            let bytes = self.as_ref();

            for &b in bytes {
                if !is_visible_ascii(b) {
                    return Err(());
                }
            }

            unsafe { Ok(std::str::from_utf8_unchecked(bytes)) }
        }
    }

    fn is_visible_ascii(b: u8) -> bool {
        (32..=126).contains(&b)
    }

    let val = HeaderValue::from_static("");
    assert_eq!(val.to_str().unwrap(), "");
}

