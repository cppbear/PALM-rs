// Answer 0

#[test]
fn test_as_str_byte_buf_valid_utf8() {
    struct Content {
        data: Vec<u8>,
    }

    impl Content {
        fn as_str(&self) -> Option<&str> {
            match &self.data {
                x if x.is_empty() => None,
                x => std::str::from_utf8(x).ok(),
            }
        }
    }

    let content = Content {
        data: vec![104, 101, 108, 108, 111], // "hello" in UTF-8
    };

    assert_eq!(content.as_str(), Some("hello"));
}

#[test]
fn test_as_str_byte_buf_invalid_utf8() {
    struct Content {
        data: Vec<u8>,
    }

    impl Content {
        fn as_str(&self) -> Option<&str> {
            match &self.data {
                x if x.is_empty() => None,
                x => std::str::from_utf8(x).ok(),
            }
        }
    }

    let content = Content {
        data: vec![255, 255, 255], // invalid UTF-8 sequence
    };

    assert_eq!(content.as_str(), None);
}

#[test]
fn test_as_str_byte_buf_empty() {
    struct Content {
        data: Vec<u8>,
    }

    impl Content {
        fn as_str(&self) -> Option<&str> {
            match &self.data {
                x if x.is_empty() => None,
                x => std::str::from_utf8(x).ok(),
            }
        }
    }

    let content = Content {
        data: vec![], // empty byte buffer
    };

    assert_eq!(content.as_str(), None);
}

