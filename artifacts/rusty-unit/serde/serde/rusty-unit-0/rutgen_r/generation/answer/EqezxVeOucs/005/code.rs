// Answer 0

#[test]
fn test_as_str_with_content_string() {
    struct Content {
        value: String,
    }

    impl Content {
        pub fn as_str(&self) -> Option<&str> {
            match self {
                Content { value: x } => Some(x),
                _ => None,
            }
        }
    }

    let content = Content {
        value: String::from("test string"),
    };

    assert_eq!(content.as_str(), Some("test string"));
}

#[test]
fn test_as_str_with_content_str() {
    struct Content<'a> {
        value: &'a str,
    }

    impl<'a> Content<'a> {
        pub fn as_str(&self) -> Option<&str> {
            match self {
                Content { value: x } => Some(x),
                _ => None,
            }
        }
    }

    let content = Content { value: "test str" };

    assert_eq!(content.as_str(), Some("test str"));
}

#[test]
fn test_as_str_with_content_bytes() {
    use std::str;

    struct Content {
        value: Vec<u8>,
    }

    impl Content {
        pub fn as_str(&self) -> Option<&str> {
            match &self.value[..] {
                x if !x.is_empty() => str::from_utf8(x).ok(),
                _ => None,
            }
        }
    }

    let content = Content { value: b"byte string".to_vec() };

    assert_eq!(content.as_str(), Some("byte string"));
}

#[test]
fn test_as_str_with_content_byte_buf() {
    use std::str;

    struct Content {
        value: Vec<u8>,
    }

    impl Content {
        pub fn as_str(&self) -> Option<&str> {
            match &self.value[..] {
                x if !x.is_empty() => str::from_utf8(x).ok(),
                _ => None,
            }
        }
    }

    let content = Content { value: b"byte buf".to_vec() };

    assert_eq!(content.as_str(), Some("byte buf"));
}

