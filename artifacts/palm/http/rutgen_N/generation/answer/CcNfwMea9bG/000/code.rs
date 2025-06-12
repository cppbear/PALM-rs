// Answer 0

#[test]
fn test_to_str_with_visible_ascii() {
    struct HeaderValue {
        data: &'static [u8],
    }

    impl HeaderValue {
        fn from_static(data: &'static str) -> Self {
            HeaderValue { data: data.as_bytes() }
        }

        fn as_ref(&self) -> &[u8] {
            self.data
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
        b.is_ascii() && b >= 32 && b <= 126
    }

    let val = HeaderValue::from_static("hello");
    assert_eq!(val.to_str().unwrap(), "hello");
}

#[test]
#[should_panic]
fn test_to_str_with_non_visible_ascii() {
    struct HeaderValue {
        data: &'static [u8],
    }

    impl HeaderValue {
        fn from_static(data: &'static str) -> Self {
            HeaderValue { data: data.as_bytes() }
        }

        fn as_ref(&self) -> &[u8] {
            self.data
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
        b.is_ascii() && b >= 32 && b <= 126
    }

    let val = HeaderValue::from_static("hello\x00");
    let _ = val.to_str().unwrap();
}

