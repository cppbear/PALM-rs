// Answer 0

#[test]
fn test_to_str_with_visible_ascii() {
    struct HeaderValue {
        value: &'static [u8],
    }

    impl HeaderValue {
        fn from_static(value: &'static str) -> Self {
            HeaderValue {
                value: value.as_bytes(),
            }
        }

        fn as_ref(&self) -> &[u8] {
            self.value
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
        b >= 0x20 && b <= 0x7E
    }

    let val = HeaderValue::from_static("hello");
    assert_eq!(val.to_str().unwrap(), "hello");
}

#[test]
fn test_to_str_with_non_visible_ascii() {
    struct HeaderValue {
        value: &'static [u8],
    }

    impl HeaderValue {
        fn from_static(value: &'static str) -> Self {
            HeaderValue {
                value: value.as_bytes(),
            }
        }

        fn as_ref(&self) -> &[u8] {
            self.value
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
        b >= 0x20 && b <= 0x7E
    }

    let val = HeaderValue::from_static("hello\x00");
    assert!(val.to_str().is_err());
}

#[test]
fn test_to_str_with_empty_string() {
    struct HeaderValue {
        value: &'static [u8],
    }

    impl HeaderValue {
        fn from_static(value: &'static str) -> Self {
            HeaderValue {
                value: value.as_bytes(),
            }
        }

        fn as_ref(&self) -> &[u8] {
            self.value
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
        b >= 0x20 && b <= 0x7E
    }

    let val = HeaderValue::from_static("");
    assert_eq!(val.to_str().unwrap(), "");
}

#[test]
fn test_to_str_with_whitespace_only() {
    struct HeaderValue {
        value: &'static [u8],
    }

    impl HeaderValue {
        fn from_static(value: &'static str) -> Self {
            HeaderValue {
                value: value.as_bytes(),
            }
        }

        fn as_ref(&self) -> &[u8] {
            self.value
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
        b >= 0x20 && b <= 0x7E
    }

    let val = HeaderValue::from_static("     ");
    assert_eq!(val.to_str().unwrap(), "     ");
}

