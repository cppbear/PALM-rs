// Answer 0

#[test]
fn test_from_str_valid() {
    struct HeaderValue {
        value: String,
    }

    impl HeaderValue {
        fn from_str(src: &str) -> Result<Self, &'static str> {
            if src.chars().all(|c| c.is_ascii() && !c.is_control()) {
                Ok(HeaderValue { value: src.to_string() })
            } else {
                Err("Invalid header value")
            }
        }
    }

    let val = HeaderValue::from_str("hello").unwrap();
    assert_eq!(val.value, "hello");
}

#[test]
fn test_from_str_invalid() {
    struct HeaderValue {
        value: String,
    }

    impl HeaderValue {
        fn from_str(src: &str) -> Result<Self, &'static str> {
            if src.chars().all(|c| c.is_ascii() && !c.is_control()) {
                Ok(HeaderValue { value: src.to_string() })
            } else {
                Err("Invalid header value")
            }
        }
    }

    let val = HeaderValue::from_str("\n");
    assert!(val.is_err());
}

#[test]
fn test_from_str_empty() {
    struct HeaderValue {
        value: String,
    }

    impl HeaderValue {
        fn from_str(src: &str) -> Result<Self, &'static str> {
            if src.chars().all(|c| c.is_ascii() && !c.is_control()) {
                Ok(HeaderValue { value: src.to_string() })
            } else {
                Err("Invalid header value")
            }
        }
    }

    let val = HeaderValue::from_str("");
    assert_eq!(val.unwrap().value, "");
}

#[test]
fn test_from_str_invalid_control_char() {
    struct HeaderValue {
        value: String,
    }

    impl HeaderValue {
        fn from_str(src: &str) -> Result<Self, &'static str> {
            if src.chars().all(|c| c.is_ascii() && !c.is_control()) {
                Ok(HeaderValue { value: src.to_string() })
            } else {
                Err("Invalid header value")
            }
        }
    }

    let val = HeaderValue::from_str("\r");
    assert!(val.is_err());
}

